#!/usr/bin/env bats

setup_file() {
    # find tests -name "*.rs" -type f -delete 2>/dev/null || true
    # find tests -name "Cargo.toml" -type f -delete 2>/dev/null || true
    # find tests -name "Cargo.lock" -type f -delete 2>/dev/null || true
    
    go build -o go2rust ./go
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

# Simple comparison function
compare_outputs() {
    local go_output="$1"
    local rust_output="$2"
    
    # Simple string comparison
    if [ "$go_output" = "$rust_output" ]; then
        return 0
    else
        return 1
    fi
}

# Helper function that handles transpilation, Rust compilation, and output comparison
# Takes test_dir and go_output as parameters
run_transpile_and_compare() {
    local test_dir="$1"
    local go_output="$2"
    
    # Transpile to Rust
    transpile_output=$(./go2rust "$test_dir" 2>&1)
    if [ ! $? ]; then
        echo "Transpilation failed:"
        echo "$transpile_output" | sed "s/^/  /"
        return 1
    fi
    
    # Run Rust version
    rust_output=$(cd "$test_dir" && RUSTFLAGS="-A warnings" cargo run --quiet 2>&1)
    rust_exit_code=$?
    
    if [ $rust_exit_code -ne 0 ]; then
        echo ""
        echo "Rust compilation/execution failed:"
        echo "$rust_output" | sed "s/^/  /"
        return 1
    fi
    
    # Compare outputs with smart map comparison
    if ! compare_outputs "$go_output" "$rust_output"; then
        echo ""
        echo "Output mismatch:"
        echo "Go output:"
        echo "$go_output"
        echo ""
        echo "Rust output:"
        echo "$rust_output"
        return 1
    fi
    
    return 0
}


run_test() {
    local test_dir="$1"
    local timeout="${TEST_TIMEOUT:-60s}"

    # Export the helper functions so they're available in the subshell
    export -f run_transpile_and_compare
    export -f compare_outputs

    # Run the entire test with timeout
    # shellcheck disable=SC2016
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
        
        # Check if expected output exists and compare
        expected_file="$test_dir/expected_output.txt"
        if [ -f "$expected_file" ]; then
            expected_output=$(cat "$expected_file")
            if [ "$go_output" != "$expected_output" ]; then
                echo ""
                echo "ERROR: Go output doesn'"'"'t match expected (non-deterministic?):"
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
        run_transpile_and_compare "$test_dir" "$go_output"    ' _ "$test_dir"; then
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
    local timeout="${TEST_TIMEOUT:-60s}"
    
    # Export the helper functions so they're available in the subshell
    export -f run_transpile_and_compare
    export -f compare_outputs
    
    # Run the entire test with timeout
    # shellcheck disable=SC2016
    if ! timeout "$timeout" bash -c '
        test_dir="$1"
        test_name="$2"
        
        # Build Go version
        go_build_output=$(cd "$test_dir" && go build -o "$test_name" . 2>&1)
        if [ $? -ne 0 ]; then
            echo "ERROR: XFAIL test '"'"'$test_name'"'"' does not compile:"
            echo "$go_build_output"
            exit 2  # Special exit code for compilation failure
        fi
        
        # Run Go binary
        go_output=$(cd "$test_dir" && ./"$test_name" 2>&1)
        go_exit_code=$?
        
        # Clean up Go binary
        rm -f "$test_dir/$test_name"
        
        if [ $go_exit_code -ne 0 ]; then
            echo "Go execution failed:"
            echo "$go_output"
            exit 1
        fi
        
        # Check if expected output exists and compare
        expected_file="$test_dir/expected_output.txt"
        if [ -f "$expected_file" ]; then
            expected_output=$(cat "$expected_file")
            if [ "$go_output" != "$expected_output" ]; then
                echo ""
                echo "ERROR: Go output doesn'"'"'t match expected (non-deterministic?):"
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
    ' _ "$test_dir" "$test_name"; then
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo "Test timed out after $timeout"
        elif [ $exit_code -eq 2 ]; then
            # Compilation failure - this is a real error for XFAIL tests
            return 1
        fi
        # Other failures are expected for XFAIL
        # But if we're running specific tests (not all tests), fail so we see the output
        if [ "$SHOW_XFAIL_ERRORS" = "true" ]; then
            return 1
        fi
        return 0
    else
        # Test passed - promote it!
        echo "🎉 Promoting XFAIL test '$test_name' - it now passes!"
        mv "$test_dir" "tests/"
        return 0
    fi
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

@test "compound_assignments" {
    run_test "tests/compound_assignments"
}

@test "const_basic" {
    run_test "tests/const_basic"
}

@test "constants_basic" {
    run_test "tests/constants_basic"
}

@test "error_simple" {
    run_test "tests/error_simple"
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

@test "maps_basic" {
    run_test "tests/maps_basic"
}

@test "maps_operations" {
    run_test "tests/maps_operations"
}

@test "methods_basic" {
    run_test "tests/methods_basic"
}

@test "nil_basic" {
    run_test "tests/nil_basic"
}

@test "pointers_simple" {
    run_test "tests/pointers_simple"
}

@test "simple_functions" {
    run_test "tests/simple_functions"
}

@test "simple_multiple_returns" {
    run_test "tests/simple_multiple_returns"
}

@test "simple_range" {
    run_test "tests/simple_range"
}

@test "slices_basic" {
    run_test "tests/slices_basic"
}

@test "strconv_parse" {
    run_test "tests/strconv_parse"
}

@test "switch_basic" {
    run_test "tests/switch_basic"
}

@test "type_assertion_simple" {
    run_test "tests/type_assertion_simple"
}

@test "values_basic" {
    run_test "tests/values_basic"
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

@test "XFAIL: blank_imports_side_effects" {
    run_xfail_test "tests/XFAIL/blank_imports_side_effects"
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

@test "XFAIL: context_usage" {
    run_xfail_test "tests/XFAIL/context_usage"
}

@test "XFAIL: cross_file_func_vars" {
    run_xfail_test "tests/XFAIL/cross_file_func_vars"
}

@test "XFAIL: cross_file_maps" {
    run_xfail_test "tests/XFAIL/cross_file_maps"
}

@test "XFAIL: cross_file_methods" {
    run_xfail_test "tests/XFAIL/cross_file_methods"
}

@test "XFAIL: cross_file_types" {
    run_xfail_test "tests/XFAIL/cross_file_types"
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

@test "XFAIL: fallthrough_switch" {
    run_xfail_test "tests/XFAIL/fallthrough_switch"
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

@test "XFAIL: function_literals_closures" {
    run_xfail_test "tests/XFAIL/function_literals_closures"
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

@test "XFAIL: goto_labels" {
    run_xfail_test "tests/XFAIL/goto_labels"
}

@test "XFAIL: http_client" {
    run_xfail_test "tests/XFAIL/http_client"
}

@test "XFAIL: init_functions" {
    run_xfail_test "tests/XFAIL/init_functions"
}

@test "XFAIL: init_order_complex" {
    run_xfail_test "tests/XFAIL/init_order_complex"
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

@test "XFAIL: iota_complex" {
    run_xfail_test "tests/XFAIL/iota_complex"
}

@test "XFAIL: iota_enums" {
    run_xfail_test "tests/XFAIL/iota_enums"
}

@test "XFAIL: json_marshal" {
    run_xfail_test "tests/XFAIL/json_marshal"
}

@test "XFAIL: labeled_statements" {
    run_xfail_test "tests/XFAIL/labeled_statements"
}

@test "XFAIL: methods" {
    run_xfail_test "tests/XFAIL/methods"
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

@test "XFAIL: named_types_methods" {
    run_xfail_test "tests/XFAIL/named_types_methods"
}

@test "XFAIL: nested_structures" {
    run_xfail_test "tests/XFAIL/nested_structures"
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

@test "XFAIL: simple_embedding" {
    run_xfail_test "tests/XFAIL/simple_embedding"
}

@test "XFAIL: slice_operations_advanced" {
    run_xfail_test "tests/XFAIL/slice_operations_advanced"
}

@test "XFAIL: slices_append" {
    run_xfail_test "tests/XFAIL/slices_append"
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

@test "XFAIL: struct_tags_reflection" {
    run_xfail_test "tests/XFAIL/struct_tags_reflection"
}

@test "XFAIL: structs_basic" {
    run_xfail_test "tests/XFAIL/structs_basic"
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

@test "XFAIL: type_assertions" {
    run_xfail_test "tests/XFAIL/type_assertions"
}

@test "XFAIL: type_conversions" {
    run_xfail_test "tests/XFAIL/type_conversions"
}

@test "XFAIL: type_embedding" {
    run_xfail_test "tests/XFAIL/type_embedding"
}

@test "XFAIL: unsafe_pointer_ops" {
    run_xfail_test "tests/XFAIL/unsafe_pointer_ops"
}

@test "XFAIL: url_parsing" {
    run_xfail_test "tests/XFAIL/url_parsing"
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
