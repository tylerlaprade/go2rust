#!/usr/bin/env bash

test_dir="$1"
phase_file="$2"
phase_detail_file="$3"
export GO2RUST_TEST_PHASE_FILE="$phase_file"
export GO2RUST_TEST_PHASE_DETAIL_FILE="$phase_detail_file"
note_fixture_phase "allocating temp workspace"
test_tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test.XXXXXX")
echo "$$" > "$test_tmp_root/go2rust-test.pid"
trap 'rm -rf "$test_tmp_root"' EXIT
export GO2RUST_TEST_TMP="$test_tmp_root"

if [[ -f "$test_dir/go.mod" ]]; then
    note_fixture_phase "go mod download"
    if ! mod_download_output=$(cd "$test_dir" && go mod download 2>&1); then
        echo "Go module download failed:"
        echo "$mod_download_output"
        exit 1
    fi
fi

# Run Go version
note_fixture_phase "go run"
if ! go_output=$(cd "$test_dir" && go run . 2>&1); then
    echo "Go compilation/execution failed:"
    echo "$go_output"
    exit 1
fi

# Check if expected output exists and compare
expected_file="$test_dir/expected_output.txt"
if [[ -f "$expected_file" ]]; then
    expected_output=$(cat "$expected_file")
    if [[ "$go_output" != "$expected_output" ]]; then
        echo ""
        echo "ERROR: Go output doesn't match expected (non-deterministic?):"
        echo ""
        echo "Expected output:"
        echo "$expected_output"
        echo ""
        echo "Actual Go output:"
        echo "$go_output"
        echo ""
        echo "This likely means the Go test produces non-deterministic output."
        echo "Please update the test to ensure deterministic output (e.g., sort map keys before iteration)."
        exit 1
    fi
else
    # Save the Go output as expected for future runs
    echo "$go_output" > "$expected_file"
fi

# Use the shared helper for transpilation and comparison
run_transpile_and_compare "$test_dir" "$go_output"
