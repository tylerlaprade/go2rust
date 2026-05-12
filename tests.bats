#!/usr/bin/env bats

setup_file() {
    # find tests -name "*.rs" -type f -delete 2>/dev/null || true
    # find tests -name "Cargo.toml" -type f -delete 2>/dev/null || true
    # find tests -name "Cargo.lock" -type f -delete 2>/dev/null || true

    if [ "${GO2RUST_TEST_BINARY_READY:-}" != "1" ]; then
        go build -o go2rust ./go
    fi
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
    local exit_code=0

    mkdir -p "$temp_dir/src"
    cp "$rust_file" "$temp_dir/src/main.rs"

    cat > "$temp_dir/Cargo.toml" << CARGO_EOF
[package]
name = "test_program"
version = "0.1.0"
edition = "2021"
CARGO_EOF

    if [ -n "$input_file" ]; then
        if (cd "$temp_dir" && run_with_prefix cargo run --quiet < "$input_file"); then
            exit_code=0
        else
            exit_code=$?
        fi
    else
        if (cd "$temp_dir" && run_with_prefix cargo run --quiet); then
            exit_code=0
        else
            exit_code=$?
        fi
    fi

    rm -rf "$temp_dir"
    return $exit_code
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
    
    # Check for test-specific configuration
    local external_mode=""
    if [ -f "$test_dir/.go2rust.toml" ]; then
        # Simple parsing - just look for external_packages line
        external_mode=$(grep "^external_packages" "$test_dir/.go2rust.toml" | cut -d'"' -f2)
    fi
    
    # Build transpile command with appropriate flags
    local transpile_cmd="./go2rust"
    if [ -n "$external_mode" ]; then
        transpile_cmd="$transpile_cmd --external-packages=$external_mode"
    fi
    transpile_cmd="$transpile_cmd \"$test_dir\""
    
    # Transpile to Rust
    transpile_output=$(eval $transpile_cmd 2>&1)
    if [ $? -ne 0 ]; then
        echo "Transpilation failed:"
        echo "$transpile_output" | sed "s/^/  /"
        return 1
    fi
    
    # Run Rust version with faster compilation settings
    # -A warnings: Allow all warnings (don't spend time on lints)
    # -C opt-level=0: No optimizations (fastest compilation)
    # -C debuginfo=0: No debug symbols (smaller binary, faster linking)
    local cargo_target_dir
    local remove_cargo_target=false
    if [ -n "${GO2RUST_TEST_TMP:-}" ]; then
        cargo_target_dir="$GO2RUST_TEST_TMP/cargo-target"
        mkdir -p "$cargo_target_dir"
    else
        cargo_target_dir=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-cargo-target.XXXXXX")
        remove_cargo_target=true
    fi

    if rust_output=$(cd "$test_dir" && CARGO_TARGET_DIR="$cargo_target_dir" RUSTFLAGS="-A warnings -C opt-level=0 -C debuginfo=0" cargo run --quiet 2>&1); then
        rust_exit_code=0
    else
        rust_exit_code=$?
    fi

    if [ "$remove_cargo_target" = true ]; then
        rm -rf "$cargo_target_dir"
    fi
    
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
        test_tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test.XXXXXX")
        trap '"'"'rm -rf "$test_tmp_root"'"'"' EXIT
        export GO2RUST_TEST_TMP="$test_tmp_root"

        if [ -f "$test_dir/go.mod" ]; then
            mod_download_output=$(cd "$test_dir" && go mod download 2>&1)
            mod_download_exit_code=$?
            if [ $mod_download_exit_code -ne 0 ]; then
                echo "Go module download failed:"
                echo "$mod_download_output"
                exit 1
            fi
        fi
        
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
        test_tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test.XXXXXX")
        trap '"'"'rm -rf "$test_tmp_root"'"'"' EXIT
        export GO2RUST_TEST_TMP="$test_tmp_root"
        
        # Build Go version
        go_build_output=$(cd "$test_dir" && go build -o "$test_name" . 2>&1)
        if [ $? -ne 0 ]; then
            echo "ERROR: XFAIL test '"'"'$test_name'"'"' does not compile:"
            echo "$go_build_output"
            exit 2
        fi
        
        # Run Go binary
        go_output=$(cd "$test_dir" && ./"$test_name" 2>&1)
        go_exit_code=$?
        
        # Clean up Go binary
        rm -f "$test_dir/$test_name"
        
        if [ $go_exit_code -ne 0 ]; then
            echo "Go execution failed:"
            echo "$go_output"
            exit 2
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
                exit 2
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
            # Compilation failure or other problem in the Go code itself- this is a real error for XFAIL tests
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
@test "address_of_fields_and_elements" {
    run_test "tests/address_of_fields_and_elements"
}

@test "address_of_pointer_argument" {
    run_test "tests/address_of_pointer_argument"
}

@test "address_of_stdlib_struct_literal" {
    run_test "tests/address_of_stdlib_struct_literal"
}

@test "advanced_control_flow" {
    run_test "tests/advanced_control_flow"
}

@test "aliasing_mutation" {
    run_test "tests/aliasing_mutation"
}

@test "anonymous_structs_basic" {
    run_test "tests/anonymous_structs_basic"
}

@test "anonymous_structs_functions" {
    run_test "tests/anonymous_structs_functions"
}

@test "anonymous_structs_nested" {
    run_test "tests/anonymous_structs_nested"
}

@test "anonymous_structs_positional" {
    run_test "tests/anonymous_structs_positional"
}

@test "any_field_handle_reuse" {
    run_test "tests/any_field_handle_reuse"
}

@test "any_interface_static_assertion" {
    run_test "tests/any_interface_static_assertion"
}

@test "any_named_return_zero" {
    run_test "tests/any_named_return_zero"
}

@test "any_selector_return" {
    run_test "tests/any_selector_return"
}

@test "any_struct_empty_literal" {
    run_test "tests/any_struct_empty_literal"
}

@test "any_struct_field_from_any" {
    run_test "tests/any_struct_field_from_any"
}

@test "any_type_conversion" {
    run_test "tests/any_type_conversion"
}

@test "append_function_slice_field" {
    run_test "tests/append_function_slice_field"
}

@test "append_selector_slice_expansion" {
    run_test "tests/append_selector_slice_expansion"
}

@test "append_slice_element" {
    run_test "tests/append_slice_element"
}

@test "append_slice_expansion" {
    run_test "tests/append_slice_expansion"
}

@test "append_string_to_bytes" {
    run_test "tests/append_string_to_bytes"
}

@test "arrays_basic" {
    run_test "tests/arrays_basic"
}

@test "atomic_operations" {
    run_test "tests/atomic_operations"
}

@test "base64_encoding" {
    run_test "tests/base64_encoding"
}

@test "blank_function_name" {
    run_test "tests/blank_function_name"
}

@test "blank_identifier" {
    run_test "tests/blank_identifier"
}

@test "blank_identifier_simple" {
    run_test "tests/blank_identifier_simple"
}

@test "blank_imports_side_effects" {
    run_test "tests/blank_imports_side_effects"
}

@test "blank_multi_assignment_discard" {
    run_test "tests/blank_multi_assignment_discard"
}

@test "blank_named_result_defer" {
    run_test "tests/blank_named_result_defer"
}

@test "blank_named_return" {
    run_test "tests/blank_named_return"
}

@test "builtin_functions" {
    run_test "tests/builtin_functions"
}

@test "builtin_min_max" {
    run_test "tests/builtin_min_max"
}

@test "byte_char_comparisons" {
    run_test "tests/byte_char_comparisons"
}

@test "byte_const_field_assignment" {
    run_test "tests/byte_const_field_assignment"
}

@test "byte_slice_from_global_array_field" {
    run_test "tests/byte_slice_from_global_array_field"
}

@test "bytes_buffer_zero_value" {
    run_test "tests/bytes_buffer_zero_value"
}

@test "cap_struct_slice_field" {
    run_test "tests/cap_struct_slice_field"
}

@test "channel_buffering" {
    run_test "tests/channel_buffering"
}

@test "channel_error_values" {
    run_test "tests/channel_error_values"
}

@test "channel_struct_fields" {
    run_test "tests/channel_struct_fields"
}

@test "channel_sync" {
    run_test "tests/channel_sync"
}

@test "channels_basic" {
    run_test "tests/channels_basic"
}

@test "channels_simple" {
    run_test "tests/channels_simple"
}

@test "closure_append_pointer_capture" {
    run_test "tests/closure_append_pointer_capture"
}

@test "closure_map_assignment_capture" {
    run_test "tests/closure_map_assignment_capture"
}

@test "closure_receiver_func_param_fnmut" {
    run_test "tests/closure_receiver_func_param_fnmut"
}

@test "closure_slice_assignment_capture" {
    run_test "tests/closure_slice_assignment_capture"
}

@test "closure_struct_literal_field_keys" {
    run_test "tests/closure_struct_literal_field_keys"
}

@test "closures_basic" {
    run_test "tests/closures_basic"
}

@test "comment_complex" {
    run_test "tests/comment_complex"
}

@test "comment_preservation" {
    run_test "tests/comment_preservation"
}

@test "complex_expressions" {
    run_test "tests/complex_expressions"
}

@test "compound_assign_len_int" {
    run_test "tests/compound_assign_len_int"
}

@test "compound_assign_wrapped_call" {
    run_test "tests/compound_assign_wrapped_call"
}

@test "compound_assignments" {
    run_test "tests/compound_assignments"
}

@test "concrete_error_call_argument" {
    run_test "tests/concrete_error_call_argument"
}

@test "concurrent_address_composite_method" {
    run_test "tests/concurrent_address_composite_method"
}

@test "concurrent_byte_index_or_assign" {
    run_test "tests/concurrent_byte_index_or_assign"
}

@test "concurrent_for_method_len" {
    run_test "tests/concurrent_for_method_len"
}

@test "concurrent_interface_field_global" {
    run_test "tests/concurrent_interface_field_global"
}

@test "concurrent_len_comparison" {
    run_test "tests/concurrent_len_comparison"
}

@test "concurrent_make_pointer_map_keys" {
    run_test "tests/concurrent_make_pointer_map_keys"
}

@test "concurrent_make_pointer_map_values" {
    run_test "tests/concurrent_make_pointer_map_values"
}

@test "concurrent_map_field_alias" {
    run_test "tests/concurrent_map_field_alias"
}

@test "concurrent_named_type_comparisons" {
    run_test "tests/concurrent_named_type_comparisons"
}

@test "concurrent_package_pointer_global" {
    run_test "tests/concurrent_package_pointer_global"
}

@test "concurrent_pointer_map_keys" {
    run_test "tests/concurrent_pointer_map_keys"
}

@test "concurrent_pointer_range_map_append" {
    run_test "tests/concurrent_pointer_range_map_append"
}

@test "concurrent_selector_map_lookup" {
    run_test "tests/concurrent_selector_map_lookup"
}

@test "concurrent_stdlib_map_slice_comma_ok" {
    run_test "tests/concurrent_stdlib_map_slice_comma_ok"
}

@test "concurrent_stdlib_selector_string_compare" {
    run_test "tests/concurrent_stdlib_selector_string_compare"
}

@test "const_basic" {
    run_test "tests/const_basic"
}

@test "const_integer_call_argument" {
    run_test "tests/const_integer_call_argument"
}

@test "constants_basic" {
    run_test "tests/constants_basic"
}

@test "context_err_error_return" {
    run_test "tests/context_err_error_return"
}

@test "context_struct_display" {
    run_test "tests/context_struct_display"
}

@test "context_usage" {
    run_test "tests/context_usage"
}

@test "copy_from_string" {
    run_test "tests/copy_from_string"
}

@test "cross_file_func_vars" {
    run_test "tests/cross_file_func_vars"
}

@test "cross_file_function_alias_param" {
    run_test "tests/cross_file_function_alias_param"
}

@test "cross_file_interface_typed_const_argument" {
    run_test "tests/cross_file_interface_typed_const_argument"
}

@test "cross_file_lower_const_compound" {
    run_test "tests/cross_file_lower_const_compound"
}

@test "cross_file_maps" {
    run_test "tests/cross_file_maps"
}

@test "cross_file_methods" {
    run_test "tests/cross_file_methods"
}

@test "cross_file_named_integer_call_conversion" {
    run_test "tests/cross_file_named_integer_call_conversion"
}

@test "cross_file_named_integer_const_arguments" {
    run_test "tests/cross_file_named_integer_const_arguments"
}

@test "cross_file_named_uint_const_peers" {
    run_test "tests/cross_file_named_uint_const_peers"
}

@test "cross_file_types" {
    run_test "tests/cross_file_types"
}

@test "cross_file_variadic_any_call" {
    run_test "tests/cross_file_variadic_any_call"
}

@test "crypto_hash" {
    run_test "tests/crypto_hash"
}

@test "current_receiver_selector_pointer_argument" {
    run_test "tests/current_receiver_selector_pointer_argument"
}

@test "current_receiver_selector_struct_field" {
    run_test "tests/current_receiver_selector_struct_field"
}

@test "declared_slice_elem_pointer" {
    run_test "tests/declared_slice_elem_pointer"
}

@test "defer_closure_return_scope" {
    run_test "tests/defer_closure_return_scope"
}

@test "defer_field_capture_rename" {
    run_test "tests/defer_field_capture_rename"
}

@test "defer_method_named_return_switch" {
    run_test "tests/defer_method_named_return_switch"
}

@test "defer_named_error_return" {
    run_test "tests/defer_named_error_return"
}

@test "defer_named_return_map_capture" {
    run_test "tests/defer_named_return_map_capture"
}

@test "defer_named_return_switch_default_first" {
    run_test "tests/defer_named_return_switch_default_first"
}

@test "defer_named_return_switch_terminates" {
    run_test "tests/defer_named_return_switch_terminates"
}

@test "defer_named_returns" {
    run_test "tests/defer_named_returns"
}

@test "defer_nested_pointer_field_capture" {
    run_test "tests/defer_nested_pointer_field_capture"
}

@test "defer_select_case" {
    run_test "tests/defer_select_case"
}

@test "defer_statements" {
    run_test "tests/defer_statements"
}

@test "dynamic_format_strings" {
    run_test "tests/dynamic_format_strings"
}

@test "elided_nested_composites" {
    run_test "tests/elided_nested_composites"
}

@test "else_if_init" {
    run_test "tests/else_if_init"
}

@test "embedded_error_return" {
    run_test "tests/embedded_error_return"
}

@test "embedded_external_method_promotion" {
    run_test "tests/embedded_external_method_promotion"
}

@test "embedded_method_promotion" {
    run_test "tests/embedded_method_promotion"
}

@test "embedded_structs" {
    run_test "tests/embedded_structs"
}

@test "enums_iota" {
    run_test "tests/enums_iota"
}

@test "error_argument_clone" {
    run_test "tests/error_argument_clone"
}

@test "error_field_return" {
    run_test "tests/error_field_return"
}

@test "error_handling" {
    run_test "tests/error_handling"
}

@test "error_map_values" {
    run_test "tests/error_map_values"
}

@test "error_method_on_error_field" {
    run_test "tests/error_method_on_error_field"
}

@test "error_method_on_error_interface" {
    run_test "tests/error_method_on_error_interface"
}

@test "error_named_string_assignment" {
    run_test "tests/error_named_string_assignment"
}

@test "error_sentinel_comparison" {
    run_test "tests/error_sentinel_comparison"
}

@test "error_simple" {
    run_test "tests/error_simple"
}

@test "error_slice_range" {
    run_test "tests/error_slice_range"
}

@test "error_slice_tuple_assignment" {
    run_test "tests/error_slice_tuple_assignment"
}

@test "errors_custom" {
    run_test "tests/errors_custom"
}

@test "errors_new_string_from_byte_field" {
    run_test "tests/errors_new_string_from_byte_field"
}

@test "exec_exit_error_assertion" {
    run_test "tests/exec_exit_error_assertion"
}

@test "external_named_integer_conversion" {
    run_test "tests/external_named_integer_conversion"
}

@test "external_packages" {
    run_test "tests/external_packages"
}

@test "external_simple" {
    run_test "tests/external_simple"
}

@test "external_stdlib_variadic" {
    run_test "tests/external_stdlib_variadic"
}

@test "external_stub_closure_capture" {
    run_test "tests/external_stub_closure_capture"
}

@test "fallthrough_switch" {
    run_test "tests/fallthrough_switch"
}

@test "file_io" {
    run_test "tests/file_io"
}

@test "flag_parsing" {
    run_test "tests/flag_parsing"
}

@test "fmt_errorf_interface_field" {
    run_test "tests/fmt_errorf_interface_field"
}

@test "fmt_errorf_type_name" {
    run_test "tests/fmt_errorf_type_name"
}

@test "fmt_pointer_slice_stringer" {
    run_test "tests/fmt_pointer_slice_stringer"
}

@test "fmt_println" {
    run_test "tests/fmt_println"
}

@test "fmt_sprintf_type_name_call" {
    run_test "tests/fmt_sprintf_type_name_call"
}

@test "for_loops" {
    run_test "tests/for_loops"
}

@test "full_slice_expressions" {
    run_test "tests/full_slice_expressions"
}

@test "func_literal_keyword_params" {
    run_test "tests/func_literal_keyword_params"
}

@test "function_field_calls" {
    run_test "tests/function_field_calls"
}

@test "function_literal_named_result" {
    run_test "tests/function_literal_named_result"
}

@test "function_literals_closures" {
    run_test "tests/function_literals_closures"
}

@test "function_return_assignment" {
    run_test "tests/function_return_assignment"
}

@test "function_type_alias_struct_field" {
    run_test "tests/function_type_alias_struct_field"
}

@test "function_type_assertion_call" {
    run_test "tests/function_type_assertion_call"
}

@test "function_type_pointer_return" {
    run_test "tests/function_type_pointer_return"
}

@test "functions_multiple_return" {
    run_test "tests/functions_multiple_return"
}

@test "generic_function_type_alias" {
    run_test "tests/generic_function_type_alias"
}

@test "global_fixed_array" {
    run_test "tests/global_fixed_array"
}

@test "global_inferred_array" {
    run_test "tests/global_inferred_array"
}

@test "goroutine_function_param_capture" {
    run_test "tests/goroutine_function_param_capture"
}

@test "goroutines_basic" {
    run_test "tests/goroutines_basic"
}

@test "goroutines_simple" {
    run_test "tests/goroutines_simple"
}

@test "goto_labels" {
    run_test "tests/goto_labels"
}

@test "hello_world" {
    run_test "tests/hello_world"
}

@test "if_else_basic" {
    run_test "tests/if_else_basic"
}

@test "import_aliases" {
    run_test "tests/import_aliases"
}

@test "imported_embedded_method_promotion" {
    run_test "tests/imported_embedded_method_promotion"
}

@test "infinite_loop_tuple_return" {
    run_test "tests/infinite_loop_tuple_return"
}

@test "init_functions" {
    run_test "tests/init_functions"
}

@test "init_order_complex" {
    run_test "tests/init_order_complex"
}

@test "interface_basic" {
    run_test "tests/interface_basic"
}

@test "interfaces_basic" {
    run_test "tests/interfaces_basic"
}

@test "interfaces_simple" {
    run_test "tests/interfaces_simple"
}

@test "iota_complex" {
    run_test "tests/iota_complex"
}

@test "iota_enums" {
    run_test "tests/iota_enums"
}

@test "json_marshal" {
    run_test "tests/json_marshal"
}

@test "json_marshal_driver_request_shape" {
    run_test "tests/json_marshal_driver_request_shape"
}

@test "json_marshal_map_field" {
    run_test "tests/json_marshal_map_field"
}

@test "keyed_array_literal" {
    run_test "tests/keyed_array_literal"
}

@test "labeled_statements" {
    run_test "tests/labeled_statements"
}

@test "late_address_of" {
    run_test "tests/late_address_of"
}

@test "len_call_argument" {
    run_test "tests/len_call_argument"
}

@test "len_short_decl" {
    run_test "tests/len_short_decl"
}

@test "len_string_literal_slice_bounds" {
    run_test "tests/len_string_literal_slice_bounds"
}

@test "len_typed_int_comparison" {
    run_test "tests/len_typed_int_comparison"
}

@test "library_example" {
    run_test "tests/library_example"
}

@test "local_concrete_stdlib_interface_return" {
    run_test "tests/local_concrete_stdlib_interface_return"
}

@test "local_const_shadows_param" {
    run_test "tests/local_const_shadows_param"
}

@test "local_const_string_slice" {
    run_test "tests/local_const_string_slice"
}

@test "local_interface_concrete_return" {
    run_test "tests/local_interface_concrete_return"
}

@test "local_interface_equality_contains" {
    run_test "tests/local_interface_equality_contains"
}

@test "local_interface_field_zero" {
    run_test "tests/local_interface_field_zero"
}

@test "local_interface_function_type_assertion" {
    run_test "tests/local_interface_function_type_assertion"
}

@test "local_interface_method_param" {
    run_test "tests/local_interface_method_param"
}

@test "local_interface_pointer_receiver" {
    run_test "tests/local_interface_pointer_receiver"
}

@test "local_large_fixed_array_zero" {
    run_test "tests/local_large_fixed_array_zero"
}

@test "local_var_multi_names" {
    run_test "tests/local_var_multi_names"
}

@test "make_capacity_wrapped_call" {
    run_test "tests/make_capacity_wrapped_call"
}

@test "make_default_slice_type_annotation" {
    run_test "tests/make_default_slice_type_annotation"
}

@test "make_error_slice_var" {
    run_test "tests/make_error_slice_var"
}

@test "make_map_with_capacity" {
    run_test "tests/make_map_with_capacity"
}

@test "make_slice_zero_len_capacity" {
    run_test "tests/make_slice_zero_len_capacity"
}

@test "map_comma_ok_bool_default" {
    run_test "tests/map_comma_ok_bool_default"
}

@test "map_len_value_assignment" {
    run_test "tests/map_len_value_assignment"
}

@test "map_lookup_wrapped_call_key" {
    run_test "tests/map_lookup_wrapped_call_key"
}

@test "map_named_integer_values" {
    run_test "tests/map_named_integer_values"
}

@test "map_pointer_key_comma_ok" {
    run_test "tests/map_pointer_key_comma_ok"
}

@test "map_pointer_selector_value_assignment" {
    run_test "tests/map_pointer_selector_value_assignment"
}

@test "map_pointer_value_literal_assignment" {
    run_test "tests/map_pointer_value_literal_assignment"
}

@test "map_range_key_comma_ok" {
    run_test "tests/map_range_key_comma_ok"
}

@test "map_range_key_value_reuse" {
    run_test "tests/map_range_key_value_reuse"
}

@test "map_range_string_value_compare" {
    run_test "tests/map_range_string_value_compare"
}

@test "map_range_value_delete_key" {
    run_test "tests/map_range_value_delete_key"
}

@test "map_slice_append_assignment" {
    run_test "tests/map_slice_append_assignment"
}

@test "map_slice_range_value_assignment" {
    run_test "tests/map_slice_range_value_assignment"
}

@test "map_wrapped_key_append_reuse" {
    run_test "tests/map_wrapped_key_append_reuse"
}

@test "maps_basic" {
    run_test "tests/maps_basic"
}

@test "maps_operations" {
    run_test "tests/maps_operations"
}

@test "method_address_local_pointer_arg" {
    run_test "tests/method_address_local_pointer_arg"
}

@test "method_any_argument" {
    run_test "tests/method_any_argument"
}

@test "method_call_struct_field_pointer" {
    run_test "tests/method_call_struct_field_pointer"
}

@test "method_case_collision" {
    run_test "tests/method_case_collision"
}

@test "method_closure_receiver_capture" {
    run_test "tests/method_closure_receiver_capture"
}

@test "method_func_literal_argument" {
    run_test "tests/method_func_literal_argument"
}

@test "method_nil_pointer_argument" {
    run_test "tests/method_nil_pointer_argument"
}

@test "method_on_function_type" {
    run_test "tests/method_on_function_type"
}

@test "method_receiver_calls" {
    run_test "tests/method_receiver_calls"
}

@test "method_receiver_goroutine_capture" {
    run_test "tests/method_receiver_goroutine_capture"
}

@test "methods" {
    run_test "tests/methods"
}

@test "methods_basic" {
    run_test "tests/methods_basic"
}

@test "mixed_output" {
    run_test "tests/mixed_output"
}

@test "multiple_returns" {
    run_test "tests/multiple_returns"
}

@test "mutex_counter" {
    run_test "tests/mutex_counter"
}

@test "mutex_direct_unlock" {
    run_test "tests/mutex_direct_unlock"
}

@test "mutex_guard_method_call" {
    run_test "tests/mutex_guard_method_call"
}

@test "named_func_return_defer" {
    run_test "tests/named_func_return_defer"
}

@test "named_integer_bitwise_xor_assign" {
    run_test "tests/named_integer_bitwise_xor_assign"
}

@test "named_integer_const_arguments" {
    run_test "tests/named_integer_const_arguments"
}

@test "named_integer_const_array_literal" {
    run_test "tests/named_integer_const_array_literal"
}

@test "named_integer_const_compound_assign" {
    run_test "tests/named_integer_const_compound_assign"
}

@test "named_integer_conversion" {
    run_test "tests/named_integer_conversion"
}

@test "named_integer_index" {
    run_test "tests/named_integer_index"
}

@test "named_integer_negative_index" {
    run_test "tests/named_integer_negative_index"
}

@test "named_pointer_defer_return" {
    run_test "tests/named_pointer_defer_return"
}

@test "named_slice_field_method_argument" {
    run_test "tests/named_slice_field_method_argument"
}

@test "named_slice_methods" {
    run_test "tests/named_slice_methods"
}

@test "named_slice_parameters" {
    run_test "tests/named_slice_parameters"
}

@test "named_slice_pointer_append" {
    run_test "tests/named_slice_pointer_append"
}

@test "named_string_conversions" {
    run_test "tests/named_string_conversions"
}

@test "named_string_receiver_conversion" {
    run_test "tests/named_string_receiver_conversion"
}

@test "named_type_comparisons" {
    run_test "tests/named_type_comparisons"
}

@test "named_types_methods" {
    run_test "tests/named_types_methods"
}

@test "nested_closure_type_assert_capture" {
    run_test "tests/nested_closure_type_assert_capture"
}

@test "nested_embedding" {
    run_test "tests/nested_embedding"
}

@test "nested_slice_array_append" {
    run_test "tests/nested_slice_array_append"
}

@test "nested_structures" {
    run_test "tests/nested_structures"
}

@test "nil_basic" {
    run_test "tests/nil_basic"
}

@test "non_blocking_channels" {
    run_test "tests/non_blocking_channels"
}

@test "numeric_conversion_literals" {
    run_test "tests/numeric_conversion_literals"
}

@test "os_args" {
    run_test "tests/os_args"
}

@test "os_file_direct_close" {
    run_test "tests/os_file_direct_close"
}

@test "os_file_readcloser_assignment" {
    run_test "tests/os_file_readcloser_assignment"
}

@test "os_stat_fileinfo_isdir" {
    run_test "tests/os_stat_fileinfo_isdir"
}

@test "package_const_lowercase_condition" {
    run_test "tests/package_const_lowercase_condition"
}

@test "package_context_shared_helper" {
    run_test "tests/package_context_shared_helper"
}

@test "package_function_array_literal_argument" {
    run_test "tests/package_function_array_literal_argument"
}

@test "package_function_nil_slice_argument" {
    run_test "tests/package_function_nil_slice_argument"
}

@test "package_function_selector_value_assignment" {
    run_test "tests/package_function_selector_value_assignment"
}

@test "package_function_type_conversion" {
    run_test "tests/package_function_type_conversion"
}

@test "package_global_anonymous_struct_array" {
    run_test "tests/package_global_anonymous_struct_array"
}

@test "package_global_channel_make" {
    run_test "tests/package_global_channel_make"
}

@test "package_global_error" {
    run_test "tests/package_global_error"
}

@test "package_global_map_incremental" {
    run_test "tests/package_global_map_incremental"
}

@test "package_global_named_slice" {
    run_test "tests/package_global_named_slice"
}

@test "package_global_shadowing" {
    run_test "tests/package_global_shadowing"
}

@test "package_interface_concrete_equality" {
    run_test "tests/package_interface_concrete_equality"
}

@test "package_named_const_argument" {
    run_test "tests/package_named_const_argument"
}

@test "package_named_string_conversion" {
    run_test "tests/package_named_string_conversion"
}

@test "package_selector_pointer_argument" {
    run_test "tests/package_selector_pointer_argument"
}

@test "package_selector_zero_values" {
    run_test "tests/package_selector_zero_values"
}

@test "package_stdlib_interface_argument" {
    run_test "tests/package_stdlib_interface_argument"
}

@test "package_variadic_import" {
    run_test "tests/package_variadic_import"
}

@test "panic_fmt_errorf_dynamic" {
    run_test "tests/panic_fmt_errorf_dynamic"
}

@test "parallel_slice_field_assign" {
    run_test "tests/parallel_slice_field_assign"
}

@test "parameter_reassignment" {
    run_test "tests/parameter_reassignment"
}

@test "parser_mode_const_expression" {
    run_test "tests/parser_mode_const_expression"
}

@test "pointer_composite_assignment" {
    run_test "tests/pointer_composite_assignment"
}

@test "pointer_field_handle_assignment" {
    run_test "tests/pointer_field_handle_assignment"
}

@test "pointer_receiver_deref_assignment" {
    run_test "tests/pointer_receiver_deref_assignment"
}

@test "pointer_receiver_nil_compare" {
    run_test "tests/pointer_receiver_nil_compare"
}

@test "pointer_slice_element_handle_assignment" {
    run_test "tests/pointer_slice_element_handle_assignment"
}

@test "pointer_string_calls_helper" {
    run_test "tests/pointer_string_calls_helper"
}

@test "pointers_basic" {
    run_test "tests/pointers_basic"
}

@test "pointers_dereference" {
    run_test "tests/pointers_dereference"
}

@test "pointers_simple" {
    run_test "tests/pointers_simple"
}

@test "promoted_pointer_index_field_assign" {
    run_test "tests/promoted_pointer_index_field_assign"
}

@test "promoted_pointer_map_field_range" {
    run_test "tests/promoted_pointer_map_field_range"
}

@test "random_numbers" {
    run_test "tests/random_numbers"
}

@test "range_index_array_literal" {
    run_test "tests/range_index_array_literal"
}

@test "range_index_assignment" {
    run_test "tests/range_index_assignment"
}

@test "range_index_call_argument" {
    run_test "tests/range_index_call_argument"
}

@test "range_index_conversion_map_key" {
    run_test "tests/range_index_conversion_map_key"
}

@test "range_index_map_int_value" {
    run_test "tests/range_index_map_int_value"
}

@test "range_index_slice_assignment" {
    run_test "tests/range_index_slice_assignment"
}

@test "range_iteration" {
    run_test "tests/range_iteration"
}

@test "range_loops" {
    run_test "tests/range_loops"
}

@test "range_map_slice_value" {
    run_test "tests/range_map_slice_value"
}

@test "range_nested_map_lookup_keys" {
    run_test "tests/range_nested_map_lookup_keys"
}

@test "range_over_integer" {
    run_test "tests/range_over_integer"
}

@test "range_over_slice_field" {
    run_test "tests/range_over_slice_field"
}

@test "range_pointer_fields" {
    run_test "tests/range_pointer_fields"
}

@test "range_pointer_value_reassign" {
    run_test "tests/range_pointer_value_reassign"
}

@test "range_return_struct_value" {
    run_test "tests/range_return_struct_value"
}

@test "range_selector_string_values" {
    run_test "tests/range_selector_string_values"
}

@test "range_stdlib_interface_map_value" {
    run_test "tests/range_stdlib_interface_map_value"
}

@test "range_stdlib_interface_slice_call" {
    run_test "tests/range_stdlib_interface_slice_call"
}

@test "range_string_closure_capture" {
    run_test "tests/range_string_closure_capture"
}

@test "range_string_key_slice_literal_reuse" {
    run_test "tests/range_string_key_slice_literal_reuse"
}

@test "range_string_method_argument" {
    run_test "tests/range_string_method_argument"
}

@test "range_string_value_reassign" {
    run_test "tests/range_string_value_reassign"
}

@test "range_wrapped_call_and_array_pointer" {
    run_test "tests/range_wrapped_call_and_array_pointer"
}

@test "rate_limiting" {
    run_test "tests/rate_limiting"
}

@test "raw_string_literals" {
    run_test "tests/raw_string_literals"
}

@test "receiver_pointer_field_literal" {
    run_test "tests/receiver_pointer_field_literal"
}

@test "receiver_self_argument_temps" {
    run_test "tests/receiver_self_argument_temps"
}

@test "recover_named_type_assertion" {
    run_test "tests/recover_named_type_assertion"
}

@test "recursion_basic" {
    run_test "tests/recursion_basic"
}

@test "recursion_factorial" {
    run_test "tests/recursion_factorial"
}

@test "recursive_closure_assignment" {
    run_test "tests/recursive_closure_assignment"
}

@test "reflect_string_header_pointer" {
    run_test "tests/reflect_string_header_pointer"
}

@test "reflect_typeof_non_struct" {
    run_test "tests/reflect_typeof_non_struct"
}

@test "regex_basic" {
    run_test "tests/regex_basic"
}

@test "regexp_methods" {
    run_test "tests/regexp_methods"
}

@test "return_indexed_pointer" {
    run_test "tests/return_indexed_pointer"
}

@test "return_pointer_selector_field" {
    run_test "tests/return_pointer_selector_field"
}

@test "return_selector_values" {
    run_test "tests/return_selector_values"
}

@test "return_slice_literal" {
    run_test "tests/return_slice_literal"
}

@test "return_string_const" {
    run_test "tests/return_string_const"
}

@test "rust_keyword_identifiers" {
    run_test "tests/rust_keyword_identifiers"
}

@test "select_basic" {
    run_test "tests/select_basic"
}

@test "select_statements" {
    run_test "tests/select_statements"
}

@test "selector_map_field_short_decl_range" {
    run_test "tests/selector_map_field_short_decl_range"
}

@test "selector_slice_call_argument" {
    run_test "tests/selector_slice_call_argument"
}

@test "selector_slice_short_decl_capture" {
    run_test "tests/selector_slice_short_decl_capture"
}

@test "selector_string_clone" {
    run_test "tests/selector_string_clone"
}

@test "selector_string_struct_literal_field" {
    run_test "tests/selector_string_struct_literal_field"
}

@test "selector_struct_field_copy" {
    run_test "tests/selector_struct_field_copy"
}

@test "shared_mutation" {
    run_test "tests/shared_mutation"
}

@test "shared_stdlib_stubs_external" {
    run_test "tests/shared_stdlib_stubs_external"
}

@test "simple_embedding" {
    run_test "tests/simple_embedding"
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

@test "slice_expr_call_argument" {
    run_test "tests/slice_expr_call_argument"
}

@test "slice_expr_method_argument" {
    run_test "tests/slice_expr_method_argument"
}

@test "slice_operations_advanced" {
    run_test "tests/slice_operations_advanced"
}

@test "slice_parallel_swap" {
    run_test "tests/slice_parallel_swap"
}

@test "slice_pointer_variable_literal" {
    run_test "tests/slice_pointer_variable_literal"
}

@test "slices_append" {
    run_test "tests/slices_append"
}

@test "slices_basic" {
    run_test "tests/slices_basic"
}

@test "slices_clip_append" {
    run_test "tests/slices_clip_append"
}

@test "slices_clone_pointer_slice" {
    run_test "tests/slices_clone_pointer_slice"
}

@test "slices_contains" {
    run_test "tests/slices_contains"
}

@test "slices_sort_func" {
    run_test "tests/slices_sort_func"
}

@test "sort_slice" {
    run_test "tests/sort_slice"
}

@test "stdlib_concrete_error_variable" {
    run_test "tests/stdlib_concrete_error_variable"
}

@test "stdlib_function_field_stub" {
    run_test "tests/stdlib_function_field_stub"
}

@test "stdlib_function_type_alias" {
    run_test "tests/stdlib_function_type_alias"
}

@test "stdlib_imports" {
    run_test "tests/stdlib_imports"
}

@test "stdlib_indexed_pointer_method" {
    run_test "tests/stdlib_indexed_pointer_method"
}

@test "stdlib_interface_call_argument" {
    run_test "tests/stdlib_interface_call_argument"
}

@test "stdlib_interface_field_assignment" {
    run_test "tests/stdlib_interface_field_assignment"
}

@test "stdlib_interface_handle_arguments" {
    run_test "tests/stdlib_interface_handle_arguments"
}

@test "stdlib_interface_ident_argument" {
    run_test "tests/stdlib_interface_ident_argument"
}

@test "stdlib_interface_map_key" {
    run_test "tests/stdlib_interface_map_key"
}

@test "stdlib_interface_map_value_assignment" {
    run_test "tests/stdlib_interface_map_value_assignment"
}

@test "stdlib_interface_named_return_defer" {
    run_test "tests/stdlib_interface_named_return_defer"
}

@test "stdlib_interface_range_assertion" {
    run_test "tests/stdlib_interface_range_assertion"
}

@test "stdlib_interface_return" {
    run_test "tests/stdlib_interface_return"
}

@test "stdlib_interface_return_variable" {
    run_test "tests/stdlib_interface_return_variable"
}

@test "stdlib_interface_selector_assertion" {
    run_test "tests/stdlib_interface_selector_assertion"
}

@test "stdlib_interface_slice_nil_compare" {
    run_test "tests/stdlib_interface_slice_nil_compare"
}

@test "stdlib_interface_struct_literal_concrete" {
    run_test "tests/stdlib_interface_struct_literal_concrete"
}

@test "stdlib_method_stubs" {
    run_test "tests/stdlib_method_stubs"
}

@test "stdlib_named_slice_type_switch" {
    run_test "tests/stdlib_named_slice_type_switch"
}

@test "stdlib_package_func_stubs" {
    run_test "tests/stdlib_package_func_stubs"
}

@test "stdlib_package_pointer_var" {
    run_test "tests/stdlib_package_pointer_var"
}

@test "stdlib_package_pointer_var_method" {
    run_test "tests/stdlib_package_pointer_var_method"
}

@test "stdlib_package_var_comparison" {
    run_test "tests/stdlib_package_var_comparison"
}

@test "stdlib_pointer_field_stub" {
    run_test "tests/stdlib_pointer_field_stub"
}

@test "stdlib_pointer_map_slice_values" {
    run_test "tests/stdlib_pointer_map_slice_values"
}

@test "stdlib_strings" {
    run_test "tests/stdlib_strings"
}

@test "stdlib_struct_field_map" {
    run_test "tests/stdlib_struct_field_map"
}

@test "stdlib_stub_value_comparison" {
    run_test "tests/stdlib_stub_value_comparison"
}

@test "stdlib_type_signatures" {
    run_test "tests/stdlib_type_signatures"
}

@test "stdlib_typed_const_index" {
    run_test "tests/stdlib_typed_const_index"
}

@test "strconv_itoa_wrapped_call" {
    run_test "tests/strconv_itoa_wrapped_call"
}

@test "strconv_parse" {
    run_test "tests/strconv_parse"
}

@test "string_append_sprintf_selector_slice" {
    run_test "tests/string_append_sprintf_selector_slice"
}

@test "string_builder" {
    run_test "tests/string_builder"
}

@test "string_builder_const" {
    run_test "tests/string_builder_const"
}

@test "string_builder_pointer" {
    run_test "tests/string_builder_pointer"
}

@test "string_interpolation" {
    run_test "tests/string_interpolation"
}

@test "string_map_parameter_keys" {
    run_test "tests/string_map_parameter_keys"
}

@test "strings_case_nested" {
    run_test "tests/strings_case_nested"
}

@test "strings_has_prefix_const_arg" {
    run_test "tests/strings_has_prefix_const_arg"
}

@test "strings_repeat_selector_count" {
    run_test "tests/strings_repeat_selector_count"
}

@test "strings_runes" {
    run_test "tests/strings_runes"
}

@test "strings_trim_space_index_expr" {
    run_test "tests/strings_trim_space_index_expr"
}

@test "strings_trim_space_slice_expr" {
    run_test "tests/strings_trim_space_slice_expr"
}

@test "struct_compare_literal" {
    run_test "tests/struct_compare_literal"
}

@test "struct_const_fields" {
    run_test "tests/struct_const_fields"
}

@test "struct_embedding" {
    run_test "tests/struct_embedding"
}

@test "struct_function_slice_field" {
    run_test "tests/struct_function_slice_field"
}

@test "struct_map_trait_value_display" {
    run_test "tests/struct_map_trait_value_display"
}

@test "struct_methods" {
    run_test "tests/struct_methods"
}

@test "struct_named_integer_map_key" {
    run_test "tests/struct_named_integer_map_key"
}

@test "struct_pointer_slice_format" {
    run_test "tests/struct_pointer_slice_format"
}

@test "struct_tags_reflection" {
    run_test "tests/struct_tags_reflection"
}

@test "structs_basic" {
    run_test "tests/structs_basic"
}

@test "structs_positional" {
    run_test "tests/structs_positional"
}

@test "switch_basic" {
    run_test "tests/switch_basic"
}

@test "switch_break_statements" {
    run_test "tests/switch_break_statements"
}

@test "switch_init_statement" {
    run_test "tests/switch_init_statement"
}

@test "switch_nil_case" {
    run_test "tests/switch_nil_case"
}

@test "switch_statements" {
    run_test "tests/switch_statements"
}

@test "switch_strings" {
    run_test "tests/switch_strings"
}

@test "sync_once" {
    run_test "tests/sync_once"
}

@test "sync_once_receiver_init" {
    run_test "tests/sync_once_receiver_init"
}

@test "tickers_basic" {
    run_test "tests/tickers_basic"
}

@test "time_duration_field_assignment" {
    run_test "tests/time_duration_field_assignment"
}

@test "time_operations" {
    run_test "tests/time_operations"
}

@test "timeouts_basic" {
    run_test "tests/timeouts_basic"
}

@test "timers_basic" {
    run_test "tests/timers_basic"
}

@test "tuple_return_reassignment" {
    run_test "tests/tuple_return_reassignment"
}

@test "type_assertion_method_receiver" {
    run_test "tests/type_assertion_method_receiver"
}

@test "type_assertion_pointer_argument" {
    run_test "tests/type_assertion_pointer_argument"
}

@test "type_assertion_simple" {
    run_test "tests/type_assertion_simple"
}

@test "type_embedding" {
    run_test "tests/type_embedding"
}

@test "type_switch_for_post" {
    run_test "tests/type_switch_for_post"
}

@test "type_switch_local_interface" {
    run_test "tests/type_switch_local_interface"
}

@test "type_switch_no_binding" {
    run_test "tests/type_switch_no_binding"
}

@test "type_switch_shadow_capture" {
    run_test "tests/type_switch_shadow_capture"
}

@test "type_switch_stdlib_selectors" {
    run_test "tests/type_switch_stdlib_selectors"
}

@test "type_switch_temporary_subject" {
    run_test "tests/type_switch_temporary_subject"
}

@test "unary_minus_unsigned" {
    run_test "tests/unary_minus_unsigned"
}

@test "unary_plus_bitwise_not" {
    run_test "tests/unary_plus_bitwise_not"
}

@test "unsafe_pointer_deref_assignment" {
    run_test "tests/unsafe_pointer_deref_assignment"
}

@test "unsafe_pointer_named_uintptr" {
    run_test "tests/unsafe_pointer_named_uintptr"
}

@test "unsafe_pointer_uintptr" {
    run_test "tests/unsafe_pointer_uintptr"
}

@test "unsafe_sizeof_alignof" {
    run_test "tests/unsafe_sizeof_alignof"
}

@test "unsafe_sizeof_comparison" {
    run_test "tests/unsafe_sizeof_comparison"
}

@test "url_parsing" {
    run_test "tests/url_parsing"
}

@test "values_basic" {
    run_test "tests/values_basic"
}

@test "var_len_make" {
    run_test "tests/var_len_make"
}

@test "variable_declarations" {
    run_test "tests/variable_declarations"
}

@test "variadic_address_selector_argument" {
    run_test "tests/variadic_address_selector_argument"
}

@test "variadic_any_arguments" {
    run_test "tests/variadic_any_arguments"
}

@test "variadic_function_value_empty" {
    run_test "tests/variadic_function_value_empty"
}

@test "variadic_functions" {
    run_test "tests/variadic_functions"
}

@test "variadic_method_any" {
    run_test "tests/variadic_method_any"
}

@test "variadic_mixed_expansion" {
    run_test "tests/variadic_mixed_expansion"
}

@test "waitgroup_sync" {
    run_test "tests/waitgroup_sync"
}

@test "worker_pools" {
    run_test "tests/worker_pools"
}

@test "wrap_everything" {
    run_test "tests/wrap_everything"
}

@test "wrap_simple" {
    run_test "tests/wrap_simple"
}

@test "wrapped_bool_field_conditions" {
    run_test "tests/wrapped_bool_field_conditions"
}

@test "wrapped_bool_not" {
    run_test "tests/wrapped_bool_not"
}

@test "wrapped_call_argument" {
    run_test "tests/wrapped_call_argument"
}

@test "XFAIL: concurrency_patterns" {
    run_xfail_test "tests/XFAIL/concurrency_patterns"
}

@test "XFAIL: file_operations" {
    run_xfail_test "tests/XFAIL/file_operations"
}

@test "XFAIL: function_types" {
    run_xfail_test "tests/XFAIL/function_types"
}

@test "XFAIL: generics_basic" {
    run_xfail_test "tests/XFAIL/generics_basic"
}

@test "XFAIL: http_client" {
    run_xfail_test "tests/XFAIL/http_client"
}

@test "XFAIL: panic_recover" {
    run_xfail_test "tests/XFAIL/panic_recover"
}

@test "XFAIL: stateful_goroutines" {
    run_xfail_test "tests/XFAIL/stateful_goroutines"
}

@test "XFAIL: type_assertions" {
    run_xfail_test "tests/XFAIL/type_assertions"
}

@test "XFAIL: type_conversions" {
    run_xfail_test "tests/XFAIL/type_conversions"
}

@test "XFAIL: unsafe_pointer_ops" {
    run_xfail_test "tests/XFAIL/unsafe_pointer_ops"
}
# END GENERATED TESTS
