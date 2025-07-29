#!/usr/bin/env bats

setup_file() {
    find tests -name "*.rs" -type f -delete 2>/dev/null || true
    find tests -name "Cargo.toml" -type f -delete 2>/dev/null || true
    find tests -name "Cargo.lock" -type f -delete 2>/dev/null || true
    
    go build -o go2rust ./go
}

teardown_file() {
    find tests -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
    find tests -name "debug" -type d -exec rm -rf {} + 2>/dev/null || true
    # Only have to clean up binaries in XFAIL since we generate them to confirm compilation works
    find tests/XFAIL -mindepth 2 -maxdepth 2 -type f -perm +111 -delete 2>/dev/null || true
}

# Helper to run a command and prefix stdout/stderr
run_with_prefix() {
    local stdout_file=$(mktemp)
    local stderr_file=$(mktemp)
    
    # Run command, capturing stdout and stderr separately
    "$@" >"$stdout_file" 2>"$stderr_file"
    
    # Interleave the outputs with prefixes, preserving order as much as possible
    while IFS= read -r line; do
        echo "[stdout] $line"
    done < "$stdout_file"
    
    while IFS= read -r line; do
        echo "[stderr] $line"
    done < "$stderr_file"
    
    rm -f "$stdout_file" "$stderr_file"
}

compile_and_run_rust() {
    local rust_file="$1"
    local input_file="$2"
    local temp_dir=$(mktemp -d)

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

    go_output=$(cd "$test_dir" && run_with_prefix go run .)

    ./go2rust "$test_dir" || return 1
    rust_output=$(cd "$test_dir" && RUSTFLAGS="-A warnings" run_with_prefix cargo run --quiet)
    
    [ "$go_output" = "$rust_output" ] || {
        echo ""
        echo "Go output:"
        echo "$go_output"
        echo ""
        echo "Rust output:"
        echo "$rust_output"
        return 1
    }
}

run_xfail_test() {
    local test_dir="$1"
    local test_name=$(basename "$test_dir")
    
    if run_test "$test_dir"; then
        echo "🎉 Promoting XFAIL test '$test_name' - it now passes!"
        mv "$test_dir" "tests/"
    fi
    return 0
}


# BEGIN GENERATED TESTS - DO NOT EDIT
@test "arrays_basic" {
    run_test "tests/arrays_basic"
}

@test "builtin_functions" {
    run_test "tests/builtin_functions"
}

@test "fmt_println" {
    run_test "tests/fmt_println"
}

@test "hello_world" {
    run_test "tests/hello_world"
}

@test "library_example" {
    run_test "tests/library_example"
}

@test "simple_functions" {
    run_test "tests/simple_functions"
}

@test "variable_declarations" {
    run_test "tests/variable_declarations"
}

@test "XFAIL: advanced_control_flow" {
    run_xfail_test "tests/XFAIL/advanced_control_flow"
}

@test "XFAIL: aliasing_mutation" {
    run_xfail_test "tests/XFAIL/aliasing_mutation"
}

@test "XFAIL: blank_identifier" {
    run_xfail_test "tests/XFAIL/blank_identifier"
}

@test "XFAIL: channels_basic" {
    run_xfail_test "tests/XFAIL/channels_basic"
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

@test "XFAIL: defer_statements" {
    run_xfail_test "tests/XFAIL/defer_statements"
}

@test "XFAIL: embedded_structs" {
    run_xfail_test "tests/XFAIL/embedded_structs"
}

@test "XFAIL: error_handling" {
    run_xfail_test "tests/XFAIL/error_handling"
}

@test "XFAIL: file_operations" {
    run_xfail_test "tests/XFAIL/file_operations"
}

@test "XFAIL: function_types" {
    run_xfail_test "tests/XFAIL/function_types"
}

@test "XFAIL: goroutines_basic" {
    run_xfail_test "tests/XFAIL/goroutines_basic"
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

@test "XFAIL: late_address_of" {
    run_xfail_test "tests/XFAIL/late_address_of"
}

@test "XFAIL: maps_basic" {
    run_xfail_test "tests/XFAIL/maps_basic"
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

@test "XFAIL: nested_structures" {
    run_xfail_test "tests/XFAIL/nested_structures"
}

@test "XFAIL: nil_basic" {
    run_xfail_test "tests/XFAIL/nil_basic"
}

@test "XFAIL: panic_recover" {
    run_xfail_test "tests/XFAIL/panic_recover"
}

@test "XFAIL: pointers_basic" {
    run_xfail_test "tests/XFAIL/pointers_basic"
}

@test "XFAIL: pointers_simple" {
    run_xfail_test "tests/XFAIL/pointers_simple"
}

@test "XFAIL: range_loops" {
    run_xfail_test "tests/XFAIL/range_loops"
}

@test "XFAIL: recursion_basic" {
    run_xfail_test "tests/XFAIL/recursion_basic"
}

@test "XFAIL: select_statements" {
    run_xfail_test "tests/XFAIL/select_statements"
}

@test "XFAIL: shared_mutation" {
    run_xfail_test "tests/XFAIL/shared_mutation"
}

@test "XFAIL: slices_basic" {
    run_xfail_test "tests/XFAIL/slices_basic"
}

@test "XFAIL: stdlib_imports" {
    run_xfail_test "tests/XFAIL/stdlib_imports"
}

@test "XFAIL: stdlib_strings" {
    run_xfail_test "tests/XFAIL/stdlib_strings"
}

@test "XFAIL: structs_basic" {
    run_xfail_test "tests/XFAIL/structs_basic"
}

@test "XFAIL: switch_statements" {
    run_xfail_test "tests/XFAIL/switch_statements"
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

@test "XFAIL: variadic_functions" {
    run_xfail_test "tests/XFAIL/variadic_functions"
}

@test "XFAIL: wrap_everything" {
    run_xfail_test "tests/XFAIL/wrap_everything"
}
# END GENERATED TESTS
