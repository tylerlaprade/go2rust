#!/usr/bin/env bats

setup_file() {
    find tests -name "*.rs" -type f -delete 2>/dev/null || true
    find tests -name "Cargo.toml" -type f -delete 2>/dev/null || true
    find tests -name "Cargo.lock" -type f -delete 2>/dev/null || true
    
    go build -o go2rust ./go
}

teardown_file() {
    # find tests -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
    # find tests -name "debug" -type d -exec rm -rf {} + 2>/dev/null || true
    # Only have to clean up binaries in XFAIL since we generate them to confirm compilation works
    find tests/XFAIL -mindepth 2 -maxdepth 2 -type f -perm +111 -delete 2>/dev/null || true
}

# Helper to run a command and prefix stdout/stderr
run_with_prefix() {
    local stdout_file
    stdout_file=$(mktemp)
    local stderr_file
    stderr_file=$(mktemp)
    
    # Run command, capturing stdout and stderr separately
    "$@" >"$stdout_file" 2>"$stderr_file"
    local exit_code=$?
    
    # Interleave the outputs with prefixes, preserving order as much as possible
    while IFS= read -r line; do
        echo "[stdout] $line"
    done < "$stdout_file"
    
    while IFS= read -r line; do
        echo "[stderr] $line"
    done < "$stderr_file"
    
    rm -f "$stdout_file" "$stderr_file"
    
    # Return the original exit code
    return $exit_code
}

compile_and_run_rust() {
    local rust_file="$1"
    local input_file="$2"
    local temp_dir
    temp_dir=$(mktemp -d)

    mkdir -p "$temp_dir/src"
    cp "$rust_file" "$temp_dir/src/main.rs"

    cat > "$temp_dir/Cargo.toml" << CARGO_EOF
[package]
name = "test_program"
version = "0.1.0"
edition = "2021"
CARGO_EOF

    if [ -n "$input_file" ]; then
        (cd "$temp_dir" && run_with_prefix cargo run --quiet < "$input_file")
    else
        (cd "$temp_dir" && run_with_prefix cargo run --quiet)
    fi

    rm -rf "$temp_dir"
}


run_test() {
    local test_dir="$1"
    local timeout="${TEST_TIMEOUT:-60s}"

    # Run the entire test with timeout
    if ! timeout "$timeout" bash -c '
        test_dir="$1"
        
        # Run Go version
        go_output=$(cd "$test_dir" && go run . 2>&1)
        go_exit_code=$?
        
        if [ $go_exit_code -ne 0 ]; then
            echo "Go compilation/execution failed:"
            echo "$go_output"
            exit 1
        fi

        # Transpile to Rust
        if ! ./go2rust "$test_dir" >/dev/null 2>&1; then
            echo "Transpilation failed"
            exit 1
        fi
        
        # Run Rust version
        rust_output=$(cd "$test_dir" && RUSTFLAGS="-A warnings" cargo run --quiet 2>&1)
        rust_exit_code=$?
        
        if [ $rust_exit_code -ne 0 ]; then
            echo ""
            echo "Go output:"
            echo "$go_output"
            echo ""
            echo "Rust compilation/execution failed:"
            echo "$rust_output"
            exit 1
        fi
        
        # Compare outputs
        if [ "$go_output" != "$rust_output" ]; then
            echo ""
            echo "Output mismatch:"
            echo "Go output:"
            echo "$go_output"
            echo ""
            echo "Rust output:"
            echo "$rust_output"
            exit 1
        fi
        
        exit 0
    ' _ "$test_dir"; then
        if [ $? -eq 124 ]; then
            echo "Test timed out after $timeout"
        fi
        return 1
    fi
    
    return 0
}

run_xfail_test() {
    local test_dir="$1"
    local test_name
    test_name=$(basename "$test_dir")
    
    if run_test "$test_dir"; then
        echo "🎉 Promoting XFAIL test '$test_name' - it now passes!"
        mv "$test_dir" "tests/"
    fi
    return 0
}


# BEGIN GENERATED TESTS - DO NOT EDIT
@test "aliasing_mutation" {
    run_test "tests/aliasing_mutation"
}

@test "arrays_basic" {
    run_test "tests/arrays_basic"
}

@test "builtin_functions" {
    run_test "tests/builtin_functions"
}

@test "fmt_println" {
    run_test "tests/fmt_println"
}

@test "for_loops" {
    run_test "tests/for_loops"
}

@test "functions_multiple_return" {
    run_test "tests/functions_multiple_return"
}

@test "hello_world" {
    run_test "tests/hello_world"
}

@test "if_else_basic" {
    run_test "tests/if_else_basic"
}

@test "late_address_of" {
    run_test "tests/late_address_of"
}

@test "library_example" {
    run_test "tests/library_example"
}

@test "pointers_simple" {
    run_test "tests/pointers_simple"
}

@test "simple_functions" {
    run_test "tests/simple_functions"
}

@test "variable_declarations" {
    run_test "tests/variable_declarations"
}

@test "wrap_everything" {
    run_test "tests/wrap_everything"
}

@test "wrap_simple" {
    run_test "tests/wrap_simple"
}

@test "XFAIL: advanced_control_flow" {
    run_xfail_test "tests/XFAIL/advanced_control_flow"
}

@test "XFAIL: atomic_operations" {
    run_xfail_test "tests/XFAIL/atomic_operations"
}

@test "XFAIL: base64_encoding" {
    run_xfail_test "tests/XFAIL/base64_encoding"
}

@test "XFAIL: blank_identifier" {
    run_xfail_test "tests/XFAIL/blank_identifier"
}

@test "XFAIL: channel_buffering" {
    run_xfail_test "tests/XFAIL/channel_buffering"
}

@test "XFAIL: channel_directions" {
    run_xfail_test "tests/XFAIL/channel_directions"
}

@test "XFAIL: channel_sync" {
    run_xfail_test "tests/XFAIL/channel_sync"
}

@test "XFAIL: channels_basic" {
    run_xfail_test "tests/XFAIL/channels_basic"
}

@test "XFAIL: channels_simple" {
    run_xfail_test "tests/XFAIL/channels_simple"
}

@test "XFAIL: closing_channels" {
    run_xfail_test "tests/XFAIL/closing_channels"
}

@test "XFAIL: closures_basic" {
    run_xfail_test "tests/XFAIL/closures_basic"
}

@test "XFAIL: complex_expressions" {
    run_xfail_test "tests/XFAIL/complex_expressions"
}

@test "XFAIL: concurrency_patterns" {
    run_xfail_test "tests/XFAIL/concurrency_patterns"
}

@test "XFAIL: constants_basic" {
    run_xfail_test "tests/XFAIL/constants_basic"
}

@test "XFAIL: context_usage" {
    run_xfail_test "tests/XFAIL/context_usage"
}

@test "XFAIL: crypto_hash" {
    run_xfail_test "tests/XFAIL/crypto_hash"
}

@test "XFAIL: defer_statements" {
    run_xfail_test "tests/XFAIL/defer_statements"
}

@test "XFAIL: embedded_structs" {
    run_xfail_test "tests/XFAIL/embedded_structs"
}

@test "XFAIL: enums_iota" {
    run_xfail_test "tests/XFAIL/enums_iota"
}

@test "XFAIL: error_handling" {
    run_xfail_test "tests/XFAIL/error_handling"
}

@test "XFAIL: errors_custom" {
    run_xfail_test "tests/XFAIL/errors_custom"
}

@test "XFAIL: file_io" {
    run_xfail_test "tests/XFAIL/file_io"
}

@test "XFAIL: file_operations" {
    run_xfail_test "tests/XFAIL/file_operations"
}

@test "XFAIL: flag_parsing" {
    run_xfail_test "tests/XFAIL/flag_parsing"
}

@test "XFAIL: function_types" {
    run_xfail_test "tests/XFAIL/function_types"
}

@test "XFAIL: generics_basic" {
    run_xfail_test "tests/XFAIL/generics_basic"
}

@test "XFAIL: goroutines_basic" {
    run_xfail_test "tests/XFAIL/goroutines_basic"
}

@test "XFAIL: goroutines_simple" {
    run_xfail_test "tests/XFAIL/goroutines_simple"
}

@test "XFAIL: http_client" {
    run_xfail_test "tests/XFAIL/http_client"
}

@test "XFAIL: init_functions" {
    run_xfail_test "tests/XFAIL/init_functions"
}

@test "XFAIL: interface_basic" {
    run_xfail_test "tests/XFAIL/interface_basic"
}

@test "XFAIL: interfaces_basic" {
    run_xfail_test "tests/XFAIL/interfaces_basic"
}

@test "XFAIL: interfaces_simple" {
    run_xfail_test "tests/XFAIL/interfaces_simple"
}

@test "XFAIL: iota_enums" {
    run_xfail_test "tests/XFAIL/iota_enums"
}

@test "XFAIL: json_marshal" {
    run_xfail_test "tests/XFAIL/json_marshal"
}

@test "XFAIL: maps_basic" {
    run_xfail_test "tests/XFAIL/maps_basic"
}

@test "XFAIL: maps_operations" {
    run_xfail_test "tests/XFAIL/maps_operations"
}

@test "XFAIL: methods" {
    run_xfail_test "tests/XFAIL/methods"
}

@test "XFAIL: methods_basic" {
    run_xfail_test "tests/XFAIL/methods_basic"
}

@test "XFAIL: mixed_output" {
    run_xfail_test "tests/XFAIL/mixed_output"
}

@test "XFAIL: multiple_returns" {
    run_xfail_test "tests/XFAIL/multiple_returns"
}

@test "XFAIL: mutex_counter" {
    run_xfail_test "tests/XFAIL/mutex_counter"
}

@test "XFAIL: nested_structures" {
    run_xfail_test "tests/XFAIL/nested_structures"
}

@test "XFAIL: nil_basic" {
    run_xfail_test "tests/XFAIL/nil_basic"
}

@test "XFAIL: non_blocking_channels" {
    run_xfail_test "tests/XFAIL/non_blocking_channels"
}

@test "XFAIL: os_args" {
    run_xfail_test "tests/XFAIL/os_args"
}

@test "XFAIL: panic_recover" {
    run_xfail_test "tests/XFAIL/panic_recover"
}

@test "XFAIL: pointers_basic" {
    run_xfail_test "tests/XFAIL/pointers_basic"
}

@test "XFAIL: pointers_dereference" {
    run_xfail_test "tests/XFAIL/pointers_dereference"
}

@test "XFAIL: random_numbers" {
    run_xfail_test "tests/XFAIL/random_numbers"
}

@test "XFAIL: range_iteration" {
    run_xfail_test "tests/XFAIL/range_iteration"
}

@test "XFAIL: range_loops" {
    run_xfail_test "tests/XFAIL/range_loops"
}

@test "XFAIL: range_over_channels" {
    run_xfail_test "tests/XFAIL/range_over_channels"
}

@test "XFAIL: rate_limiting" {
    run_xfail_test "tests/XFAIL/rate_limiting"
}

@test "XFAIL: recursion_basic" {
    run_xfail_test "tests/XFAIL/recursion_basic"
}

@test "XFAIL: recursion_factorial" {
    run_xfail_test "tests/XFAIL/recursion_factorial"
}

@test "XFAIL: regex_basic" {
    run_xfail_test "tests/XFAIL/regex_basic"
}

@test "XFAIL: select_basic" {
    run_xfail_test "tests/XFAIL/select_basic"
}

@test "XFAIL: select_statements" {
    run_xfail_test "tests/XFAIL/select_statements"
}

@test "XFAIL: shared_mutation" {
    run_xfail_test "tests/XFAIL/shared_mutation"
}

@test "XFAIL: slices_append" {
    run_xfail_test "tests/XFAIL/slices_append"
}

@test "XFAIL: slices_basic" {
    run_xfail_test "tests/XFAIL/slices_basic"
}

@test "XFAIL: sort_slice" {
    run_xfail_test "tests/XFAIL/sort_slice"
}

@test "XFAIL: stateful_goroutines" {
    run_xfail_test "tests/XFAIL/stateful_goroutines"
}

@test "XFAIL: stdlib_imports" {
    run_xfail_test "tests/XFAIL/stdlib_imports"
}

@test "XFAIL: stdlib_strings" {
    run_xfail_test "tests/XFAIL/stdlib_strings"
}

@test "XFAIL: strconv_parse" {
    run_xfail_test "tests/XFAIL/strconv_parse"
}

@test "XFAIL: string_builder" {
    run_xfail_test "tests/XFAIL/string_builder"
}

@test "XFAIL: string_interpolation" {
    run_xfail_test "tests/XFAIL/string_interpolation"
}

@test "XFAIL: strings_runes" {
    run_xfail_test "tests/XFAIL/strings_runes"
}

@test "XFAIL: struct_embedding" {
    run_xfail_test "tests/XFAIL/struct_embedding"
}

@test "XFAIL: struct_methods" {
    run_xfail_test "tests/XFAIL/struct_methods"
}

@test "XFAIL: structs_basic" {
    run_xfail_test "tests/XFAIL/structs_basic"
}

@test "XFAIL: switch_basic" {
    run_xfail_test "tests/XFAIL/switch_basic"
}

@test "XFAIL: switch_statements" {
    run_xfail_test "tests/XFAIL/switch_statements"
}

@test "XFAIL: tickers_basic" {
    run_xfail_test "tests/XFAIL/tickers_basic"
}

@test "XFAIL: time_operations" {
    run_xfail_test "tests/XFAIL/time_operations"
}

@test "XFAIL: timeouts_basic" {
    run_xfail_test "tests/XFAIL/timeouts_basic"
}

@test "XFAIL: timers_basic" {
    run_xfail_test "tests/XFAIL/timers_basic"
}

@test "XFAIL: type_assertion_simple" {
    run_xfail_test "tests/XFAIL/type_assertion_simple"
}

@test "XFAIL: type_assertions" {
    run_xfail_test "tests/XFAIL/type_assertions"
}

@test "XFAIL: type_conversions" {
    run_xfail_test "tests/XFAIL/type_conversions"
}

@test "XFAIL: type_embedding" {
    run_xfail_test "tests/XFAIL/type_embedding"
}

@test "XFAIL: url_parsing" {
    run_xfail_test "tests/XFAIL/url_parsing"
}

@test "XFAIL: values_basic" {
    run_xfail_test "tests/XFAIL/values_basic"
}

@test "XFAIL: variadic_functions" {
    run_xfail_test "tests/XFAIL/variadic_functions"
}

@test "XFAIL: waitgroup_sync" {
    run_xfail_test "tests/XFAIL/waitgroup_sync"
}

@test "XFAIL: worker_pools" {
    run_xfail_test "tests/XFAIL/worker_pools"
}
# END GENERATED TESTS
