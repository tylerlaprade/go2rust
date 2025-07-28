#!/usr/bin/env bats

setup_file() {
    # Clean up any generated files from previous runs
    find tests -name "*.rs" -type f -delete 2>/dev/null || true
    find tests -name "Cargo.toml" -type f -delete 2>/dev/null || true
    find tests -name "Cargo.lock" -type f -delete 2>/dev/null || true
    
    go build -o go2rust ./go
}

teardown() {
    # Clean up Rust build artifacts after each test
    find tests -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
    # Clean up Go binaries created by XFAIL validation in test.sh
    rm -f tests/XFAIL/*/methods tests/XFAIL/*/pointers_basic 2>/dev/null || true
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
@test "builtin_functions" {
    run_test "tests/builtin_functions/"
}

@test "fmt_println" {
    run_test "tests/fmt_println/"
}

@test "hello_world" {
    run_test "tests/hello_world/"
}

@test "library_example" {
    run_test "tests/library_example/"
}

@test "mixed_output" {
    run_test "tests/mixed_output/"
}

@test "simple_functions" {
    run_test "tests/simple_functions/"
}

@test "stdlib_imports" {
    run_test "tests/stdlib_imports/"
}

@test "stdlib_strings" {
    run_test "tests/stdlib_strings/"
}

@test "variable_declarations" {
    run_test "tests/variable_declarations/"
}

@test "XFAIL: methods" {
    run_xfail_test "tests/XFAIL/methods/"
}

@test "XFAIL: pointers_basic" {
    run_xfail_test "tests/XFAIL/pointers_basic/"
}
# END GENERATED TESTS
