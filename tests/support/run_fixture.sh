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

# Some argv-boundary fixtures need bytes that Rust's UTF-8 std::env::args API
# rejects. Encode one raw argument as HEX so Go and generated Rust receive the
# same argv bytes. NUL is rejected because Unix argv cannot contain it.
raw_argv_hex=$(fixture_config_value "$test_dir" "raw_argv_hex")
if [[ -n "$raw_argv_hex" ]]; then
    if [[ "$raw_argv_hex" =~ [^0-9A-Fa-f] ]] ||
       (( ${#raw_argv_hex} % 2 != 0 )) ||
       [[ "${raw_argv_hex,,}" == *00* ]]; then
        echo "Invalid raw_argv_hex fixture config: $raw_argv_hex"
        exit 1
    fi
    export GO2RUST_TEST_RAW_ARG=""
    while [[ -n "$raw_argv_hex" ]]; do
        printf -v raw_argv_byte '%b' "\\x${raw_argv_hex:0:2}"
        GO2RUST_TEST_RAW_ARG+="$raw_argv_byte"
        raw_argv_hex=${raw_argv_hex:2}
    done
fi

# Some filesystem-boundary fixtures need a working-directory basename that is
# not valid UTF-8. Encode the suffix as HEX; the runner creates a private
# directory named "cwd-<suffix>" and launches both executables from it. NUL
# and slash are rejected because neither can occur inside a Unix path segment.
raw_cwd_hex=$(fixture_config_value "$test_dir" "raw_cwd_hex")
if [[ -n "$raw_cwd_hex" ]]; then
    raw_cwd_spec=$raw_cwd_hex
    if [[ "$raw_cwd_hex" =~ [^0-9A-Fa-f] ]] ||
       (( ${#raw_cwd_hex} % 2 != 0 )); then
        echo "Invalid raw_cwd_hex fixture config: $raw_cwd_hex"
        exit 1
    fi
    raw_cwd_suffix=""
    while [[ -n "$raw_cwd_hex" ]]; do
        raw_cwd_pair=${raw_cwd_hex:0:2}
        if [[ "${raw_cwd_pair,,}" == "00" || "${raw_cwd_pair,,}" == "2f" ]]; then
            echo "Invalid raw_cwd_hex fixture config: $raw_cwd_spec"
            exit 1
        fi
        printf -v raw_cwd_byte '%b' "\\x$raw_cwd_pair"
        raw_cwd_suffix+="$raw_cwd_byte"
        raw_cwd_hex=${raw_cwd_hex:2}
    done
    export GO2RUST_TEST_RAW_CWD="$test_tmp_root/cwd-$raw_cwd_suffix"
    mkdir -- "$GO2RUST_TEST_RAW_CWD"
fi

# A read-only filesystem fixture can request one raw-byte file name inside the
# private working directory. The same entry remains present for the Go and
# generated Rust executions, so directory enumeration observes identical
# external state.
raw_cwd_entry_hex=$(fixture_config_value "$test_dir" "raw_cwd_entry_hex")
if [[ -n "$raw_cwd_entry_hex" ]]; then
    raw_cwd_entry_spec=$raw_cwd_entry_hex
    if [[ -z "${GO2RUST_TEST_RAW_CWD:-}" ]] ||
       [[ "$raw_cwd_entry_hex" =~ [^0-9A-Fa-f] ]] ||
       (( ${#raw_cwd_entry_hex} % 2 != 0 )); then
        echo "Invalid raw_cwd_entry_hex fixture config: $raw_cwd_entry_spec"
        exit 1
    fi
    raw_cwd_entry=""
    while [[ -n "$raw_cwd_entry_hex" ]]; do
        raw_cwd_entry_pair=${raw_cwd_entry_hex:0:2}
        if [[ "${raw_cwd_entry_pair,,}" == "00" || "${raw_cwd_entry_pair,,}" == "2f" ]]; then
            echo "Invalid raw_cwd_entry_hex fixture config: $raw_cwd_entry_spec"
            exit 1
        fi
        printf -v raw_cwd_entry_byte '%b' "\\x$raw_cwd_entry_pair"
        raw_cwd_entry+="$raw_cwd_entry_byte"
        raw_cwd_entry_hex=${raw_cwd_entry_hex:2}
    done
    touch -- "$GO2RUST_TEST_RAW_CWD/$raw_cwd_entry"
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
go_run=(go run .)
if [[ -n "${GO2RUST_TEST_RAW_CWD:-}" ]]; then
    note_fixture_phase "go build"
    if ! go_build_output=$(cd "$test_dir" && go build -o "$test_tmp_root/go-fixture" . 2>&1); then
        echo "Go compilation failed:"
        echo "$go_build_output"
        exit 1
    fi
    go_run=("$test_tmp_root/go-fixture")
fi
if [[ -n "${GO2RUST_TEST_RAW_ARG+x}" ]]; then
    go_run+=("$GO2RUST_TEST_RAW_ARG")
fi
go_run_dir=${GO2RUST_TEST_RAW_CWD:-$test_dir}
if [[ -n "${GO2RUST_TEST_RAW_ENV_NAME:-}" ]]; then
    go_output=$(cd "$go_run_dir" && env "$GO2RUST_TEST_RAW_ENV_NAME=$GO2RUST_TEST_RAW_ENV_VALUE" "${go_run[@]}" 2>&1)
    go_exit_code=$?
else
    go_output=$(cd "$go_run_dir" && "${go_run[@]}" 2>&1)
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
