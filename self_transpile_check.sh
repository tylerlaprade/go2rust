#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: ./self_transpile_check.sh [--cargo-check] [--package <crate>]...

Transpile go2rust's own Go package in a temporary workspace.

Options:
  --cargo-check       Run cargo check after transpilation.
  --package <crate>   With --cargo-check, check one crate. May be repeated.
  -h, --help          Show this help.

Environment:
  KEEP_SELF_TRANSPILE=1   Preserve the temporary workspace for inspection.
  GOCACHE=<path>          Override the temporary Go build cache.
  CARGO_TARGET_DIR=<path> Override the temporary Cargo target directory.
EOF
}

cargo_check=false
packages=()

while [ "$#" -gt 0 ]; do
    case "$1" in
        --cargo-check)
            cargo_check=true
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
work=$(mktemp -d "${TMPDIR:-/private/tmp}/go2rust-self.XXXXXX")
keep=${KEEP_SELF_TRANSPILE:-0}

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

echo "Self-transpile check passed" >&2
