#!/bin/bash

# Generate test cases and update the GENERATED TESTS section in tests.bats

# Create temporary file for new test cases
temp_file=$(mktemp)

# Generate test cases for single .go files
for go_file in tests/*.go; do
    [ -f "$go_file" ] || continue
    test_name=$(basename "$go_file" .go)
    
    # Skip if it's part of a directory test
    dir_name="${go_file%.go}"
    if [ -d "$dir_name" ] && [ -f "$dir_name/lib.go" ]; then
        continue
    fi
    
    cat >> "$temp_file" << EOF
@test "$test_name" {
    run_transpilation_test "$go_file"
}

EOF
done

# Generate test cases for directories with lib.go and test.go
for dir in tests/*/; do
    [ -d "$dir" ] || continue
    if [ -f "$dir/lib.go" ] && [ -f "$dir/test.go" ]; then
        test_name=$(basename "$dir")
        
        cat >> "$temp_file" << EOF
@test "$test_name" {
    run_directory_test "$dir"
}

EOF
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