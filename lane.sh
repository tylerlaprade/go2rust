#!/usr/bin/env bash
# lane.sh — serialized, memory-guarded, INCREMENTAL self-host verification lane.
#
# Why this exists: the 8GB box can run exactly ONE cargo build at a time, and the
# repo's pressure_guard counts only free+speculative pages (it under-reports real
# available memory on macOS, refusing valid runs / killing builds at 2s). This lane:
#   - serializes ALL heavy work via flock (only one build machine-wide at a time)
#   - uses an accurate MemAvailable-style metric (free+inactive+speculative+purgeable)
#   - keeps a PERSISTENT workspace + GOCACHE + CARGO_HOME + CARGO_TARGET_DIR so
#     cargo/go work is incremental, not cold every run
#   - scopes cargo check to ONE crate (default go_types) and emits an error histogram
#
# Usage:
#   ./lane.sh baseline [--crate go_types] [--packages "<src-stdlib-pattern>"]
#   ./lane.sh check    [--crate go_types]          # re-check after a transpiler edit
#   ./lane.sh transpile-only                        # just rebuild+transpile, no cargo
#   ./lane.sh avail                                 # print accurate available memory (MB)
set -euo pipefail

REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
LANE_DIR="${GO2RUST_LANE_DIR:-/tmp/go2rust-lane}"
WORK="$LANE_DIR/ws"
LOCK="$LANE_DIR/lane.lock"
LOG="$LANE_DIR/last.cargo.json"
FLOOR_MB="${GO2RUST_LANE_FLOOR_MB:-250}"   # kill build if accurate-avail drops below this
# go/types self-host stack (the keystone). Full self-host uses the bigger default set.
STACK="go/types,go/ast,go/token,go/scanner,go/parser,go/constant,go/internal/typeparams,go/internal/typesinternal,internal/types/errors"

mkdir -p "$LANE_DIR"

avail_mb() {
  vm_stat 2>/dev/null | awk '
    /page size of/ { for (i=1;i<=NF;i++) if ($i ~ /^[0-9]+$/) ps=$i }
    /Pages free/        { gsub(/\./,"",$3); f=$3 }
    /Pages inactive/    { gsub(/\./,"",$3); ia=$3 }
    /Pages speculative/ { gsub(/\./,"",$3); s=$3 }
    /Pages purgeable/   { gsub(/\./,"",$3); p=$3 }
    END { if (ps>0) printf "%.0f\n", (f+ia+s+p)*ps/1048576; else print 999999 }'
}

cmd="${1:-}"; shift || true
CRATE="go_types"
PACKAGES="$STACK"
PROBE_DIR=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --crate)    CRATE="$2"; shift 2;;
    --packages) PACKAGES="$2"; shift 2;;
    --*) echo "lane.sh: unknown arg $1" >&2; exit 2;;
    *) PROBE_DIR="$1"; shift;;          # positional: probe fixture dir
  esac
done

if [ "$cmd" = "avail" ]; then avail_mb; exit 0; fi

# --- memory watchdog: kill the whole process tree if accurate-avail dips below floor
terminate_tree() { local pid="$1" c; while IFS= read -r c; do [ -n "$c" ] && terminate_tree "$c"; done < <(ps -o pid= -P "$pid" 2>/dev/null||true); kill -TERM "$pid" 2>/dev/null||true; }
guarded() {
  local label="$1"; shift
  local a; a=$(avail_mb)
  if [ "$a" -lt "$FLOOR_MB" ]; then echo "lane: REFUSING $label — avail ${a}MB < floor ${FLOOR_MB}MB" >&2; return 137; fi
  "$@" & local cpid=$!
  ( while kill -0 "$cpid" 2>/dev/null; do
      local av; av=$(avail_mb)
      if [ "$av" -lt "$FLOOR_MB" ]; then echo "lane: KILLING $label — avail ${av}MB < floor ${FLOOR_MB}MB" >&2; terminate_tree "$cpid"; sleep 2; kill -KILL "$cpid" 2>/dev/null||true; break; fi
      sleep 3
    done ) & local wpid=$!
  local st=0; wait "$cpid" || st=$?
  kill "$wpid" 2>/dev/null||true; wait "$wpid" 2>/dev/null||true
  return $st
}

histogram() {
  # parse cargo --message-format=json error stream in $LOG
  python3 - "$LOG" <<'PY'
import json,sys,collections
codes=collections.Counter(); files=collections.Counter(); total=0
for line in open(sys.argv[1],encoding='utf-8',errors='replace'):
    line=line.strip()
    if not line.startswith('{'): continue
    try: m=json.loads(line)
    except: continue
    if m.get('reason')!='compiler-message': continue
    msg=m.get('message',{})
    if msg.get('level')!='error': continue
    total+=1
    code=(msg.get('code') or {}).get('code') or 'NONE'
    codes[code]+=1
    for sp in msg.get('spans',[]):
        if sp.get('is_primary'):
            fn=sp.get('file_name','?'); files[fn.split('/')[-1] if fn else '?']+=1
            break
print(f"TOTAL_ERRORS={total}")
print("BY_CODE:");  [print(f"  {c:8} {n}") for c,n in codes.most_common()]
print("BY_FILE(top15):"); [print(f"  {n:4} {f}") for f,n in files.most_common(15)]
PY
}

run() {
  echo "lane: avail=$(avail_mb)MB floor=${FLOOR_MB}MB  work=$WORK target=$LANE_DIR/cargo-target" >&2
  export GOCACHE="$LANE_DIR/go-build-cache"
  export CARGO_HOME="$LANE_DIR/cargo-home"
  export CARGO_TARGET_DIR="$LANE_DIR/cargo-target"
  export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
  export RUSTFLAGS="${RUSTFLAGS:--Awarnings -C debuginfo=0}"
  export GOFLAGS="${GOFLAGS:--tags=purego}"
  export GO2RUST_SOURCE_STDLIB_PACKAGES="$PACKAGES"

  mkdir -p "$WORK"
  cp "$REPO/go.mod" "$REPO/go.sum" "$WORK/" 2>/dev/null || true

  echo "lane: building go2rust (from repo, incremental GOCACHE) ..." >&2
  guarded "go build" go build -o "$WORK/go2rust" "$REPO/go" || return $?

  # --- PROBE mode: transpile ONLY a tiny self-contained fixture (+ the stdlib it
  #     reaches), NOT all of go2rust+x/tools. Reachability bounds the generated
  #     go_types to what the probe touches -> cheap partial check.
  if [ "$cmd" = "probe" ]; then
    [ -n "$PROBE_DIR" ] && [ -d "$PROBE_DIR" ] || { echo "lane: probe needs an existing fixture dir" >&2; return 2; }
    local pkgs; pkgs=$(grep '^source_stdlib_packages' "$PROBE_DIR/.go2rust.toml" 2>/dev/null | cut -d'"' -f2)
    local pdir="$WORK/probe"
    rm -rf "$pdir"; cp -R "$PROBE_DIR" "$pdir"
    echo "lane: transpiling probe $(basename "$PROBE_DIR") (source-stdlib: ${pkgs:-none}) ..." >&2
    ( cd "$pdir" && guarded "transpile" "$WORK/go2rust" ${pkgs:+--source-stdlib-packages="$pkgs"} . ) || return $?
    echo "lane: cargo check -p $CRATE (partial) ..." >&2
    ( cd "$pdir" && guarded "cargo" bash -c "cargo check $([ \"$CRATE\" = all ] && echo --workspace || echo \"-p $CRATE\") --keep-going --message-format=json > '$LOG' 2>/dev/null; true" )
    histogram; return 0
  fi

  if [ "$cmd" = "transpile-only" ] || [ "$cmd" = "baseline" ] || [ "$cmd" = "check" ]; then
    echo "lane: transpiling self-host stack ..." >&2
    rm -rf "$WORK/go"
    cp -R "$REPO/go" "$WORK/go"
    ( cd "$WORK" && guarded "transpile" ./go2rust go ) || return $?
  fi
  [ "$cmd" = "transpile-only" ] && { echo "lane: transpile OK"; return 0; }

  echo "lane: cargo check -p $CRATE ..." >&2
  ( cd "$WORK/go" && guarded "cargo" bash -c "cargo check $([ \"$CRATE\" = all ] && echo --workspace || echo \"-p $CRATE\") --keep-going --message-format=json > '$LOG' 2>/dev/null; true" )
  histogram
}

# portable lock (macOS has no flock): atomic mkdir + stale-PID reclaim
LOCKDIR="$LANE_DIR/lane.lock.d"
while ! mkdir "$LOCKDIR" 2>/dev/null; do
  if [ -f "$LOCKDIR/pid" ]; then
    holder=$(cat "$LOCKDIR/pid" 2>/dev/null || true)
    if [ -n "$holder" ] && ! kill -0 "$holder" 2>/dev/null; then
      echo "lane: reclaiming stale lock from dead pid $holder" >&2; rm -rf "$LOCKDIR"; continue
    fi
  fi
  echo "lane: another build holds the lock; waiting..." >&2; sleep 3
done
echo "$$" > "$LOCKDIR/pid"
trap 'rm -rf "$LOCKDIR"' EXIT
run
