#!/usr/bin/env bats

setup_file() {
    go build -o go2rust ./go
}

teardown() {
    find tests -name "temp_*" -type d -exec rm -rf {} + 2>/dev/null || true
    find tests -name "main" -type f -delete 2>/dev/null || true
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

    # Transpile all Go files in the directory
    for go_file in "$test_dir"/*.go; do
        [ -f "$go_file" ] || continue
        base_name=$(basename "$go_file" .go)
        rust_file="$test_dir/$base_name.rs"
        
        ./go2rust "$go_file" > "$rust_file" || return 1
    done

    # Set up Rust project
    local temp_dir=$(mktemp -d)
    mkdir -p "$temp_dir/src"

    # For now, concatenate all .rs files with main.rs last
    # First, add all non-main.rs files
    for rs_file in "$test_dir"/*.rs; do
        [ -f "$rs_file" ] || continue
        if [ "$(basename "$rs_file")" != "main.rs" ]; then
            cat "$rs_file" >> "$temp_dir/src/main.rs"
            echo "" >> "$temp_dir/src/main.rs"  # Add newline between files
        fi
    done
    # Then add main.rs
    if [ -f "$test_dir/main.rs" ]; then
        cat "$test_dir/main.rs" >> "$temp_dir/src/main.rs"
    fi
    
    cat > "$temp_dir/Cargo.toml" << CARGO_EOF
[package]
name = "test_program"
version = "0.1.0"
edition = "2021"
CARGO_EOF
    
    rust_output=$(cd "$temp_dir" && run_with_prefix cargo run --quiet)
    
    rm -rf "$temp_dir"
    
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
@test "fmt_println" {
    run_test "tests/fmt_println/"
}

@test "hello_world" {
    run_test "tests/hello_world/"
}

@test "mixed_output" {
    run_test "tests/mixed_output/"
}

@test "simple_functions" {
    run_test "tests/simple_functions/"
}

@test "stdlib_strings" {
    run_test "tests/stdlib_strings/"
}

@test "XFAIL: builtin_functions" {
    run_xfail_test "tests/XFAIL/builtin_functions/"
}

@test "XFAIL: methods" {
    run_xfail_test "tests/XFAIL/methods/"
}

@test "XFAIL: pointers_basic" {
    run_xfail_test "tests/XFAIL/pointers_basic/"
}

@test "XFAIL: stdlib_imports" {
    run_xfail_test "tests/XFAIL/stdlib_imports/"
}

@test "XFAIL: variable_declarations" {
    run_xfail_test "tests/XFAIL/variable_declarations/"
}
# END GENERATED TESTS
