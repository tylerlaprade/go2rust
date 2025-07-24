#!/bin/bash

# Generate test cases and update the GENERATED TESTS section in tests.bats

# Create temporary file for new test cases
temp_file=$(mktemp)

# Generate test cases for directories containing main.go
for dir in tests/*/; do
    [ -d "$dir" ] || continue
    if [ -f "$dir/main.go" ]; then
        test_name=$(basename "$dir")
        
        # Check if it's a multi-file test (has other .go files besides main.go)
        go_file_count=$(find "$dir" -name "*.go" -type f | wc -l)
        
        if [ "$go_file_count" -gt 1 ]; then
            # Multi-file test
            cat >> "$temp_file" << EOF
@test "$test_name" {
    run_directory_test "$dir"
}

EOF
        else
            # Single-file test
            cat >> "$temp_file" << EOF
@test "$test_name" {
    run_transpilation_test "$dir/main.go"
}

EOF
        fi
    fi
done

# Remove trailing newline
sed -i.bak '$ { /^$/ d; }' "$temp_file" && rm "$temp_file.bak"

# Replace content between markers
awk '
    /^# BEGIN GENERATED TESTS/ {
        print
        while ((getline line < "'"$temp_file"'") > 0) {
            print line
        }
        close("'"$temp_file"'")
        skip = 1
        next
    }
    /^# END GENERATED TESTS/ {
        skip = 0
    }
    !skip
' tests.bats > tests.bats.new

mv tests.bats.new tests.bats
rm "$temp_file"

echo "Updated tests.bats with $(grep -c '^@test' tests.bats) tests"

# Run tests
if ! command -v bats >/dev/null 2>&1; then
    echo "Error: bats is not installed"
    echo "Install with: brew install bats-core"
    exit 1
fi

echo "Running tests..."
bats tests.bats