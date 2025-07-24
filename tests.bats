#!/usr/bin/env bats

# Build transpiler once before all tests
setup_file() {
    go build -o go2rust .
}

# Clean up after each test
teardown() {
    find tests -name "*.rs" -delete
    find tests -name "temp_*" -type d -exec rm -rf {} + 2>/dev/null || true
}

# Helper to compile and run Rust code
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
        output=$(cd "$temp_dir" && cargo run --quiet 2>&1 < "$input_file")
    else
        output=$(cd "$temp_dir" && cargo run --quiet 2>&1)
    fi

    rm -rf "$temp_dir"
    echo "$output"
}

# Generic test runner
run_transpilation_test() {
    local go_file="$1"
    local test_name=$(basename "$go_file" .go)
    local rust_file="${go_file%.go}.rs"
    local input_dir="tests/$test_name"

    # Transpile
    ./go2rust "$go_file" > "$rust_file" || return 1

    # Check for input directory
    if [ -d "$input_dir" ]; then
        # Test with each input file
        for input_file in "$input_dir"/*; do
            [ -f "$input_file" ] || continue

            local input_name=$(basename "$input_file")

            # Run Go version
            go_output=$(go run "$go_file" < "$input_file" 2>&1)

            # Run Rust version
            rust_output=$(compile_and_run_rust "$rust_file" "$input_file")

            # Compare
            [ "$go_output" = "$rust_output" ] || {
                echo "Failed on input: $input_name"
                echo "Go output:   '$go_output'"
                echo "Rust output: '$rust_output'"
                return 1
            }
        done
    else
        # No input files, just run without stdin
        go_output=$(go run "$go_file" 2>&1)
        rust_output=$(compile_and_run_rust "$rust_file" "")

        [ "$go_output" = "$rust_output" ] || {
            echo "Go output:   '$go_output'"
            echo "Rust output: '$rust_output'"
            return 1
        }
    fi
}

# Directory-based test runner for multi-file tests
run_directory_test() {
    local test_dir="$1"
    local test_name=$(basename "$test_dir")
    
    # Check if it's a directory with lib.go and test.go
    if [ ! -f "$test_dir/main.go" ]; then
        skip "Not a directory test"
    fi
    
    # Run Go version
    go_output=$(cd "$test_dir" && go run . 2>&1)
    
    # Transpile all .go files
    for go_file in "$test_dir"/*.go; do
        [ -f "$go_file" ] || continue
        base_name=$(basename "$go_file" .go)
        ./go2rust "$go_file" > "$test_dir/$base_name.rs" || return 1
    done
    
    # Create a Rust project structure
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
    
    # Create Cargo.toml
    cat > "$temp_dir/Cargo.toml" << CARGO_EOF
[package]
name = "test_program"
version = "0.1.0"
edition = "2021"
CARGO_EOF
    
    # Run Rust version
    rust_output=$(cd "$temp_dir" && cargo run --quiet 2>&1)
    
    # Clean up
    rm -rf "$temp_dir"
    
    # Compare outputs
    [ "$go_output" = "$rust_output" ] || {
        echo "Go output:   '$go_output'"
        echo "Rust output: '$rust_output'"
        return 1
    }
}


# BEGIN GENERATED TESTS - DO NOT EDIT
@test "fmt_println" {
    run_transpilation_test "tests/fmt_println//main.go"
}

@test "hello_world" {
    run_transpilation_test "tests/hello_world//main.go"
}

@test "simple_functions" {
    run_directory_test "tests/simple_functions/"
}
# END GENERATED TESTS
