#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: ./self_transpile_check.sh [--cargo-check] [--behavior-suite] [--package <crate>]...

Transpile go2rust's own Go package in a temporary workspace.

Options:
  --cargo-check       Run cargo check after transpilation.
  --behavior-suite    Build the generated Rust transpiler and run the fixture
                      suite against that binary in a copied test workspace.
  --package <crate>   With --cargo-check, check one crate. May be repeated.
  -h, --help          Show this help.

Environment:
  KEEP_SELF_TRANSPILE=1   Preserve the temporary workspace for inspection.
  GOCACHE=<path>          Override the temporary Go build cache.
  CARGO_HOME=<path>       Override the temporary Cargo registry/cache home.
  CARGO_TARGET_DIR=<path> Override the temporary Cargo target directory.
  GO2RUST_CARGO_OFFLINE=auto|1|0
                      Use Cargo --offline. With auto, enable it only when
                      CARGO_HOME already has a registry index (default: auto).
  GOFLAGS=<flags>         Override Go build/load flags (default: -tags=purego).
  GO2RUST_BEHAVIOR_JOBS=N Number of behavior-suite shards (default: auto via
                      test.sh memory detection).
  GO2RUST_BEHAVIOR_TIMEOUT=TIME Per-test behavior timeout (default: 30s).
  GO2RUST_BEHAVIOR_TESTS="name [name...]"
                      Restrict --behavior-suite to specific fixture names.
  GO2RUST_SELF_CLEAN_AGE_MINUTES=N
                      Age threshold for startup cleanup of stale go2rust temp
                      artifacts (default: 60).
  GO2RUST_SELF_CLEAN_STALE=0
                      Disable startup cleanup of stale self-transpile workspaces
                      that were created with an owner pid marker.
  GO2RUST_SELF_MIN_AVAILABLE_MEM_MB
                      Minimum available memory before self-transpile work
                      (default: 1024; 0 disables).
  GO2RUST_SELF_CARGO_MIN_AVAILABLE_MEM_MB
                      Minimum available memory before self-transpile Cargo
                      validation (default: 2048; 0 disables).
  GO2RUST_SELF_SKIP_PRESSURE_GUARD=1
                      Bypass the available-memory guard.
  GO2RUST_SOURCE_STDLIB_PACKAGES=PATTERNS
                      Stdlib packages to transpile from GOROOT source
                      instead of semantic stubs (default: go/..., internal/...,
                      and the self-host stdlib dependencies that must not route
                      through semantic stubs).
EOF
}

cargo_check=false
behavior_suite=false
packages=()

while [ "$#" -gt 0 ]; do
    case "$1" in
        --cargo-check)
            cargo_check=true
            shift
            ;;
        --behavior-suite)
            behavior_suite=true
            shift
            ;;
        --package|-p)
            if [ "$#" -lt 2 ]; then
                echo "error: --package requires a crate name" >&2
                exit 2
            fi
            packages+=("$2")
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown argument: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
tmp_root="${TMPDIR:-/private/tmp}"

cleanup_stale_self_workspaces() {
    [ "${GO2RUST_SELF_CLEAN_STALE:-1}" = "0" ] && return
    "$repo_root/cleanup.sh" --age-minutes "${GO2RUST_SELF_CLEAN_AGE_MINUTES:-60}" --keep-repo-artifacts >/dev/null
}

copy_behavior_suite_tests() {
    local suite="$1"
    if command -v rsync >/dev/null 2>&1; then
        mkdir -p "$suite/tests"
        rsync -a \
            --exclude='*.rs' \
            --exclude='Cargo.toml' \
            --exclude='Cargo.lock' \
            "$repo_root/tests/" "$suite/tests/"
        return
    fi

    cp -R "$repo_root/tests" "$suite/tests"
    find "$suite/tests" -name "*.rs" -type f -delete
    find "$suite/tests" -name "Cargo.toml" -type f -delete
    find "$suite/tests" -name "Cargo.lock" -type f -delete
}

detect_available_memory_bytes() {
    if [ -r /proc/meminfo ]; then
        awk '/MemAvailable/ { printf "%.0f\n", $2 * 1024 }' /proc/meminfo 2>/dev/null
        return
    fi

    if command -v vm_stat >/dev/null 2>&1; then
        vm_stat 2>/dev/null | awk '
            /page size of/ {
                page_size = $8
                gsub(/[^0-9]/, "", page_size)
            }
            /Pages free:/ {
                free_pages = $3
                gsub(/[^0-9]/, "", free_pages)
            }
            /Pages speculative:/ {
                speculative_pages = $3
                gsub(/[^0-9]/, "", speculative_pages)
            }
            END {
                if (page_size > 0) {
                    printf "%.0f\n", (free_pages + speculative_pages) * page_size
                }
            }
        '
        return
    fi

    if command -v memory_pressure >/dev/null 2>&1; then
        memory_pressure 2>/dev/null | awk '
            /^The system has / {
                total = $4
            }
            /System-wide memory free percentage:/ {
                pct = $5
                gsub(/%/, "", pct)
            }
            END {
                if (total ~ /^[0-9]+$/ && pct ~ /^[0-9]+$/) {
                    printf "%.0f\n", total * pct / 100
                }
            }
        '
        return
    fi
}

enforce_available_memory_floor() {
    local min_var="${1:-GO2RUST_SELF_MIN_AVAILABLE_MEM_MB}"
    local default_min_mb="${2:-1024}"
    local work_label="${3:-self-transpile work}"

    case "${GO2RUST_SELF_SKIP_PRESSURE_GUARD:-0}" in
        1|true|TRUE|yes|YES)
            return
            ;;
    esac

    local min_mb="${!min_var:-$default_min_mb}"
    case "$min_mb" in
        ''|*[!0-9]*)
            echo "error: $min_var must be a non-negative integer" >&2
            exit 2
            ;;
    esac
    [ "$min_mb" -eq 0 ] && return

    local available_bytes
    available_bytes=$(detect_available_memory_bytes)
    case "$available_bytes" in
        ''|*[!0-9]*)
            return
            ;;
    esac

    local min_bytes=$(( min_mb * 1024 * 1024 ))
    if [ "$available_bytes" -lt "$min_bytes" ]; then
        local available_mb=$(( available_bytes / 1024 / 1024 ))
        echo "Error: available memory is ${available_mb} MiB, below ${min_var}=${min_mb} MiB." >&2
        echo "Refusing to start $work_label while the machine is under memory pressure." >&2
        echo "Run ./cleanup.sh --pressure --quick to inspect current pressure, or set GO2RUST_SELF_SKIP_PRESSURE_GUARD=1 to force the run." >&2
        exit 1
    fi
}

cleanup_stale_self_workspaces
if [ "$cargo_check" = true ] || [ "$behavior_suite" = true ]; then
    enforce_available_memory_floor GO2RUST_SELF_CARGO_MIN_AVAILABLE_MEM_MB 2048 "self-transpile Cargo validation"
else
    enforce_available_memory_floor GO2RUST_SELF_MIN_AVAILABLE_MEM_MB 1024 "self-transpile work"
fi
work=$(mktemp -d "$tmp_root/go2rust-self.XXXXXX")
keep=${KEEP_SELF_TRANSPILE:-0}
echo "$$" > "$work/self_transpile_check.pid"

cleanup() {
    status=$?
    if [ "$keep" = "1" ]; then
        echo "Preserved self-transpile workspace: $work" >&2
    else
        rm -rf "$work"
    fi
    exit "$status"
}
trap cleanup EXIT

echo "Self-transpile workspace: $work" >&2

cp -R "$repo_root/go" "$work/go"
cp "$repo_root/go.mod" "$work/go.mod"
cp "$repo_root/go.sum" "$work/go.sum"

export GOCACHE="${GOCACHE:-$work/go-build-cache}"
export CARGO_HOME="${CARGO_HOME:-$work/cargo-home}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-0}"
export CARGO_PROFILE_DEV_INCREMENTAL="${CARGO_PROFILE_DEV_INCREMENTAL:-false}"
export RUSTFLAGS="${RUSTFLAGS:--Awarnings -C debuginfo=0}"
export GOFLAGS="${GOFLAGS:--tags=purego}"
export GO2RUST_SOURCE_STDLIB_PACKAGES="${GO2RUST_SOURCE_STDLIB_PACKAGES:-go/...,internal/...,cmp,slices,strconv,reflect,math/big,math/bits,math,strings,regexp,regexp/syntax,path/filepath,path,sort,container/heap,sync,sync/atomic,text/scanner,unicode,unicode/utf8,hash/maphash,crypto/rand,crypto/internal/boring,crypto/internal/boring/sig,crypto/internal/fips140,crypto/internal/fips140deps/godebug,crypto/internal/sysrand}"
cargo_offline_args=()
case "${GO2RUST_CARGO_OFFLINE:-auto}" in
    1|true|yes)
        cargo_offline_args=(--offline)
        ;;
    0|false|no)
        ;;
    auto|"")
        if compgen -G "$CARGO_HOME/registry/index/*" >/dev/null; then
            cargo_offline_args=(--offline)
        fi
        ;;
    *)
        echo "error: GO2RUST_CARGO_OFFLINE must be auto, 1, or 0" >&2
        exit 2
        ;;
esac
go build -o "$work/go2rust" "$repo_root/go"

(
    cd "$work"
    ./go2rust go
)

if [ "$cargo_check" = true ]; then
    export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$work/cargo-target}"
    if [ "${#packages[@]}" -eq 0 ]; then
        (
            cd "$work/go"
            cargo "${cargo_offline_args[@]}" check --workspace --message-format=short
        )
    else
        for package in "${packages[@]}"; do
            (
                cd "$work/go"
                cargo "${cargo_offline_args[@]}" check -p "$package" --message-format=short
            )
        done
    fi
fi

if [ "$behavior_suite" = true ]; then
    export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$work/cargo-target}"
    (
        cd "$work/go"
        cargo "${cargo_offline_args[@]}" build -p go --bin go
    )

    suite="$work/behavior-suite"
    mkdir -p "$suite"
    cp "$repo_root/test.sh" "$suite/test.sh"
    cp "$repo_root/cleanup.sh" "$suite/cleanup.sh"
    chmod +x "$suite/test.sh" "$suite/cleanup.sh"
    cp "$repo_root/tests.bats" "$suite/tests.bats"
    cp "$repo_root/go.mod" "$suite/go.mod"
    cp "$repo_root/go.sum" "$suite/go.sum"
    copy_behavior_suite_tests "$suite"

    (
        cd "$suite"
        behavior_tests=()
        if [ -n "${GO2RUST_BEHAVIOR_TESTS:-}" ]; then
            # shellcheck disable=SC2206
            behavior_tests=(${GO2RUST_BEHAVIOR_TESTS})
        fi
        behavior_args=(-t "${GO2RUST_BEHAVIOR_TIMEOUT:-30s}")
        if [ -n "${GO2RUST_BEHAVIOR_JOBS:-}" ]; then
            behavior_args=(-n "$GO2RUST_BEHAVIOR_JOBS" "${behavior_args[@]}")
        fi
        GO2RUST_TEST_BINARY="$CARGO_TARGET_DIR/debug/go" \
            GOCACHE="${GOCACHE:-$work/go-build-cache}" \
            ./test.sh "${behavior_args[@]}" "${behavior_tests[@]}"
    )
fi

echo "Self-transpile check passed" >&2
