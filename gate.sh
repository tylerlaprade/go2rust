#!/usr/bin/env bash
# gate.sh — serialize ONE heavy command (go build / go test / cargo) across ALL
# worktree agents + the lane, via a single shared mkdir-lock + memory floor.
# Worktree agents draft code in parallel (cheap, remote); when they need to
# COMPILE, they funnel through this gate so only one build runs at a time on the
# 8GB box. Usage:  /abs/path/gate.sh <cmd> [args...]   (run from the cwd you want)
set -euo pipefail
LANE_DIR="${GO2RUST_LANE_DIR:-/tmp/go2rust-lane}"
GLOCK="$LANE_DIR/lane.lock.d"            # SAME lock the probe uses -> global serialization
FLOOR_MB="${GO2RUST_LANE_FLOOR_MB:-200}"
mkdir -p "$LANE_DIR"
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
avail_mb() {
  local available_bytes
  available_bytes=$("$REPO/pressure_guard.sh" --available-bytes)
  case "$available_bytes" in
    ''|*[!0-9]*) echo 999999 ;;
    *) echo $((available_bytes / 1048576)) ;;
  esac
}
waited=0
while ! mkdir "$GLOCK" 2>/dev/null; do
  h=$(cat "$GLOCK/pid" 2>/dev/null||true)
  if [[ -n "$h" ]] && ! kill -0 "$h" 2>/dev/null; then rm -rf "$GLOCK"; continue; fi
  [[ $((waited % 30)) -eq 0 ]] && echo "gate: another build holds the lock; waiting..." >&2
  sleep 3; waited=$((waited+3))
done
echo "$$" > "$GLOCK/pid"
trap 'rm -rf "$GLOCK"' EXIT
# wait for memory headroom before starting the heavy command
available_mb=$(avail_mb)
while [[ "$available_mb" -lt "$FLOOR_MB" ]]; do echo "gate: avail ${available_mb}MB < ${FLOOR_MB}MB, waiting..." >&2; sleep 5; available_mb=$(avail_mb); done
echo "gate: running [$*] avail=${available_mb}MB" >&2
export GOCACHE="${GOCACHE:-$LANE_DIR/go-build-cache}" GOFLAGS="${GOFLAGS:--tags=purego}"
"$@"
