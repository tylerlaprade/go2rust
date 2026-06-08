#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: ./go_test.sh [go test args...]

Run go test ./go with a disposable Go build cache by default.

Environment:
  GOCACHE=<path>                    Use an explicit Go build cache.
  KEEP_GO2RUST_GO_TEST_CACHE=1      Preserve the temp cache created by this script.
  GO2RUST_GO_TEST_CLEAN_AGE_MINUTES Age threshold for startup cleanup (default: 60).
  GO2RUST_GO_TEST_CLEAN_STALE=0     Disable startup cleanup of stale go2rust temps.
  GO2RUST_GO_TEST_MIN_AVAILABLE_MEM_MB Minimum available memory before go test (default: 512; 0 disables).
  GO2RUST_GO_TEST_SKIP_PRESSURE_GUARD=1 Bypass the available-memory guard.

Examples:
  ./go_test.sh -run TestExternalPackageStubConstantsPreserveGoTypesValues
  ./go_test.sh -run TestFoo -count=1
EOF
}

if [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ]; then
    usage
    exit 0
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
GO_TEST_GOCACHE_DIR=""

cleanup() {
    status=$?
    if [ -n "$GO_TEST_GOCACHE_DIR" ]; then
        if [ "${KEEP_GO2RUST_GO_TEST_CACHE:-0}" = "1" ]; then
            echo "Preserved Go test cache: $GO_TEST_GOCACHE_DIR" >&2
        else
            rm -rf "$GO_TEST_GOCACHE_DIR"
        fi
    fi
    exit "$status"
}
trap cleanup EXIT

if [ "${GO2RUST_GO_TEST_CLEAN_STALE:-1}" != "0" ]; then
    "$repo_root/cleanup.sh" --age-minutes "${GO2RUST_GO_TEST_CLEAN_AGE_MINUTES:-60}" --keep-repo-artifacts >/dev/null
fi

detect_available_memory_bytes() {
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
}

enforce_available_memory_floor() {
    case "${GO2RUST_GO_TEST_SKIP_PRESSURE_GUARD:-0}" in
        1|true|TRUE|yes|YES)
            return
            ;;
    esac

    local min_mb="${GO2RUST_GO_TEST_MIN_AVAILABLE_MEM_MB:-512}"
    case "$min_mb" in
        ''|*[!0-9]*)
            echo "error: GO2RUST_GO_TEST_MIN_AVAILABLE_MEM_MB must be a non-negative integer" >&2
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
        echo "Error: available memory is ${available_mb} MiB, below GO2RUST_GO_TEST_MIN_AVAILABLE_MEM_MB=${min_mb} MiB." >&2
        echo "Refusing to start go test while the machine is under memory pressure." >&2
        echo "Run ./cleanup.sh --pressure --quick to inspect current pressure, or set GO2RUST_GO_TEST_SKIP_PRESSURE_GUARD=1 to force the run." >&2
        exit 1
    fi
}

if [ -z "${GOCACHE:-}" ]; then
    GO_TEST_GOCACHE_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-go-cache.XXXXXX")
    echo "$$" > "$GO_TEST_GOCACHE_DIR/go2rust-test.pid"
    export GOCACHE="$GO_TEST_GOCACHE_DIR"
fi

cd "$repo_root"
enforce_available_memory_floor
go test ./go "$@"
