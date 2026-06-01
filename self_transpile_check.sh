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
  GOFLAGS=<flags>         Override Go build/load flags (default: -tags=purego).
  GO2RUST_BEHAVIOR_JOBS=N Number of behavior-suite shards (default: 3).
  GO2RUST_BEHAVIOR_TIMEOUT=TIME Per-test behavior timeout (default: 30s).
  GO2RUST_BEHAVIOR_TESTS="name [name...]"
                      Restrict --behavior-suite to specific fixture names.
  GO2RUST_SELF_CLEAN_STALE=0
                      Disable startup cleanup of stale self-transpile workspaces
                      that were created with an owner pid marker.
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

    find "$tmp_root" -maxdepth 1 -type d -name 'go2rust-self.*' -print 2>/dev/null | while IFS= read -r dir; do
        pid_file="$dir/self_transpile_check.pid"
        [ -f "$pid_file" ] || continue

        pid=$(cat "$pid_file" 2>/dev/null || true)
        if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
            continue
        fi

        rm -rf "$dir"
    done
}

cleanup_stale_self_workspaces
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
export GO2RUST_SOURCE_STDLIB_PACKAGES="${GO2RUST_SOURCE_STDLIB_PACKAGES:-go/...,internal/...,cmp,slices,reflect,math/big,math/bits,math,strings,regexp,regexp/syntax,path/filepath,text/scanner,unicode,unicode/utf8,hash/maphash,crypto/rand,crypto/internal/boring,crypto/internal/fips140}"
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
            cargo check --workspace --message-format=short
        )
    else
        for package in "${packages[@]}"; do
            (
                cd "$work/go"
                cargo check -p "$package" --message-format=short
            )
        done
    fi
fi

if [ "$behavior_suite" = true ]; then
    export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$work/cargo-target}"
    (
        cd "$work/go"
        cargo build -p go --bin go
    )

    suite="$work/behavior-suite"
    mkdir -p "$suite"
    cp "$repo_root/test.sh" "$suite/test.sh"
    cp "$repo_root/tests.bats" "$suite/tests.bats"
    cp "$repo_root/go.mod" "$suite/go.mod"
    cp "$repo_root/go.sum" "$suite/go.sum"
    cp -R "$repo_root/tests" "$suite/tests"
    find "$suite/tests" -name "*.rs" -type f -delete
    find "$suite/tests" -name "Cargo.toml" -type f -delete
    find "$suite/tests" -name "Cargo.lock" -type f -delete

    (
        cd "$suite"
        behavior_tests=()
        if [ -n "${GO2RUST_BEHAVIOR_TESTS:-}" ]; then
            # shellcheck disable=SC2206
            behavior_tests=(${GO2RUST_BEHAVIOR_TESTS})
        fi
        GO2RUST_TEST_BINARY="$CARGO_TARGET_DIR/debug/go" \
            GOCACHE="${GOCACHE:-$work/go-build-cache}" \
            ./test.sh -n "${GO2RUST_BEHAVIOR_JOBS:-3}" -t "${GO2RUST_BEHAVIOR_TIMEOUT:-30s}" "${behavior_tests[@]}"
    )
fi

echo "Self-transpile check passed" >&2
