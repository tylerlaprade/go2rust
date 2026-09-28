#!/usr/bin/env bash

test_dir="$1"
test_name="$2"
phase_file="$3"
phase_detail_file="$4"
export GO2RUST_TEST_PHASE_FILE="$phase_file"
export GO2RUST_TEST_PHASE_DETAIL_FILE="$phase_detail_file"
note_fixture_phase "allocating temp workspace"
test_tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test.XXXXXX")
echo "$$" > "$test_tmp_root/go2rust-test.pid"
trap 'rm -rf "$test_tmp_root"' EXIT
export GO2RUST_TEST_TMP="$test_tmp_root"

# Build Go version
note_fixture_phase "go build"
if ! go_build_output=$(cd "$test_dir" && go build -o "$test_name" . 2>&1); then
    echo "ERROR: XFAIL test '$test_name' does not compile:"
    echo "$go_build_output"
    exit 2
fi

# Run Go binary
note_fixture_phase "go run"
go_output=$(cd "$test_dir" && ./"$test_name" 2>&1)
go_exit_code=$?

# Clean up Go binary
rm -f "$test_dir/$test_name"

if [[ "$go_exit_code" -ne 0 ]]; then
    echo "Go execution failed:"
    echo "$go_output"
    exit 2
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
        exit 2
    fi
else
    # Save the Go output as expected for future runs
    echo "$go_output" > "$expected_file"
fi

# Use the shared helper for transpilation and comparison
run_transpile_and_compare "$test_dir" "$go_output"
