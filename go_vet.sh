#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: ./go_vet.sh [go vet args...]

Run go vet ./go with a disposable Go build cache by default.

Environment:
  GOCACHE=<path>                   Use an explicit Go build cache.
  KEEP_GO2RUST_GO_VET_CACHE=1      Preserve the temp cache created by this script.
  GO2RUST_GO_VET_CLEAN_AGE_MINUTES Age threshold for startup cleanup (default: 60).
  GO2RUST_GO_VET_CLEAN_STALE=0     Disable startup cleanup of stale go2rust temps.
  GO2RUST_GO_VET_MIN_AVAILABLE_MEM_MB Minimum available memory before go vet (default: 512; 0 disables).
  GO2RUST_GO_VET_SKIP_PRESSURE_GUARD=1 Bypass the available-memory guard.

Examples:
  ./go_vet.sh
  ./go_vet.sh -tags=purego
EOF
}

if [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ]; then
    usage
    exit 0
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
GO_VET_GOCACHE_DIR=""

cleanup() {
    status=$?
    if [ -n "$GO_VET_GOCACHE_DIR" ]; then
        if [ "${KEEP_GO2RUST_GO_VET_CACHE:-0}" = "1" ]; then
            echo "Preserved Go vet cache: $GO_VET_GOCACHE_DIR" >&2
        else
            rm -rf "$GO_VET_GOCACHE_DIR"
        fi
    fi
    exit "$status"
}
trap cleanup EXIT

if [ "${GO2RUST_GO_VET_CLEAN_STALE:-1}" != "0" ]; then
    "$repo_root/cleanup.sh" --age-minutes "${GO2RUST_GO_VET_CLEAN_AGE_MINUTES:-60}" --keep-repo-artifacts >/dev/null
fi

enforce_available_memory_floor() {
    "$repo_root/pressure_guard.sh" \
        --min-env GO2RUST_GO_VET_MIN_AVAILABLE_MEM_MB \
        --default-min-mb 512 \
        --skip-env GO2RUST_GO_VET_SKIP_PRESSURE_GUARD \
        --label "go vet" \
        --hint "Run ./cleanup.sh --pressure --quick to inspect current pressure, or set GO2RUST_GO_VET_SKIP_PRESSURE_GUARD=1 to force the run." || exit $?
}

cd "$repo_root"
enforce_available_memory_floor

if [ -z "${GOCACHE:-}" ]; then
    GO_VET_GOCACHE_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-go-vet-cache.XXXXXX")
    echo "$$" > "$GO_VET_GOCACHE_DIR/go2rust-vet.pid"
    export GOCACHE="$GO_VET_GOCACHE_DIR"
fi

go vet ./go "$@"
