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

# Some OS-boundary fixtures need bytes that Go can observe but Rust's UTF-8
# std::env::var API rejects. Encode one raw environment entry as NAME=HEX in
# .go2rust.toml so the identical byte value reaches both executables.
raw_env_spec=$(fixture_config_value "$test_dir" "raw_env_hex")
if [[ -n "$raw_env_spec" ]]; then
    raw_env_name=${raw_env_spec%%=*}
    raw_env_hex=${raw_env_spec#*=}
    if [[ ! "$raw_env_name" =~ ^[A-Za-z_][A-Za-z0-9_]*$ ]] ||
       [[ "$raw_env_hex" == "$raw_env_spec" ]] ||
       [[ "$raw_env_hex" =~ [^0-9A-Fa-f] ]] ||
       (( ${#raw_env_hex} % 2 != 0 )) ||
       [[ "${raw_env_hex,,}" == *00* ]]; then
        echo "Invalid raw_env_hex fixture config: $raw_env_spec"
        exit 1
    fi
    export GO2RUST_TEST_RAW_ENV_NAME="$raw_env_name"
    export GO2RUST_TEST_RAW_ENV_VALUE
    GO2RUST_TEST_RAW_ENV_VALUE=""
    while [[ -n "$raw_env_hex" ]]; do
        printf -v raw_env_byte '%b' "\\x${raw_env_hex:0:2}"
        GO2RUST_TEST_RAW_ENV_VALUE+="$raw_env_byte"
        raw_env_hex=${raw_env_hex:2}
    done
fi

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
if [[ -n "${GO2RUST_TEST_RAW_ENV_NAME:-}" ]]; then
    go_output=$(cd "$test_dir" && env "$GO2RUST_TEST_RAW_ENV_NAME=$GO2RUST_TEST_RAW_ENV_VALUE" go run . 2>&1)
    go_exit_code=$?
else
    go_output=$(cd "$test_dir" && go run . 2>&1)
    go_exit_code=$?
fi
if [[ "$go_exit_code" -ne 0 ]]; then
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
