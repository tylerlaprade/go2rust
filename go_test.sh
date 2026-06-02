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

if [ -z "${GOCACHE:-}" ]; then
    GO_TEST_GOCACHE_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-go-cache.XXXXXX")
    echo "$$" > "$GO_TEST_GOCACHE_DIR/go2rust-test.pid"
    export GOCACHE="$GO_TEST_GOCACHE_DIR"
fi

cd "$repo_root"
go test ./go "$@"
