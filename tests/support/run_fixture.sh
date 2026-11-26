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

# LookPath order fixtures need the Go and Rust programs to observe the same
# controlled PATH prefix. The first command is executable in both directories;
# the fallback command is non-executable in the first and executable in the
# second.
controlled_path_lookup=$(fixture_config_value "$test_dir" "controlled_path_lookup")
if [[ "$controlled_path_lookup" = "true" ]]; then
    first_path_dir="$test_tmp_root/path-first"
    second_path_dir="$test_tmp_root/path-second"
    mkdir -p "$first_path_dir" "$second_path_dir"
    for command_path in \
        "$first_path_dir/go2rust-path-first" \
        "$second_path_dir/go2rust-path-first" \
        "$first_path_dir/go2rust-path-fallback" \
        "$second_path_dir/go2rust-path-fallback"; do
        printf '#!/bin/sh\nexit 0\n' > "$command_path"
    done
    chmod 0755 \
        "$first_path_dir/go2rust-path-first" \
        "$second_path_dir/go2rust-path-first" \
        "$second_path_dir/go2rust-path-fallback"
    chmod 0644 "$first_path_dir/go2rust-path-fallback"
    export GO2RUST_TEST_PATH_FIRST="$first_path_dir"
    export GO2RUST_TEST_PATH_SECOND="$second_path_dir"
    export GO2RUST_TEST_RUNTIME_PATH="$first_path_dir:$second_path_dir:$PATH"
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
stdin_file=$(fixture_stdin "$test_dir")
# A fixture whose program ends with a status other than 0, such as a panic
# that nothing recovers (status 2), names it as exit_status. go run reports
# every failure as status 1, so the program is built and run directly. Both
# programs must end with that status, and their output is compared up to
# Go's goroutine trace, which the translation cannot reproduce.
expected_exit_status=$(fixture_config_value "$test_dir" "exit_status")
export GO2RUST_TEST_EXIT_STATUS="${expected_exit_status:-0}"
if [[ -n "$expected_exit_status" ]]; then
    note_fixture_phase "go build"
    go_binary="$test_tmp_root/go-fixture"
    if ! go_build_output=$(cd "$test_dir" && go build -o "$go_binary" . 2>&1); then
        echo "Go compilation failed:"
        echo "$go_build_output"
        exit 1
    fi
    note_fixture_phase "go run"
    go_exit_status=0
    go_output=$(cd "$test_dir" && PATH="${GO2RUST_TEST_RUNTIME_PATH:-$PATH}" "$go_binary" 2>&1 < "$stdin_file") || go_exit_status=$?
    if [[ "$go_exit_status" -ne "$expected_exit_status" ]]; then
        echo "Go program exited with status $go_exit_status, not $expected_exit_status:"
        echo "$go_output"
        exit 1
    fi
    go_output=$(strip_go_panic_trace "$go_output")
else
    note_fixture_phase "go run"
    if ! go_output=$(cd "$test_dir" && PATH="${GO2RUST_TEST_RUNTIME_PATH:-$PATH}" go run . 2>&1 < "$stdin_file"); then
        echo "Go compilation/execution failed:"
        echo "$go_output"
        exit 1
    fi
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
