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
    
    cat > "$temp_dir/Cargo.toml" << EOF
[package]
name = "test_program"
version = "0.1.0"
edition = "2021"
EOF
    
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

# Test each .go file in tests/
@test "hello_world" {
    run_transpilation_test "tests/hello_world.go"
}

# Add more tests here as .go files are added:
# @test "variables" {
#     run_transpilation_test "tests/variables.go"
# }
# @test "echo_program" {
#     run_transpilation_test "tests/echo_program.go"
# }