#!/bin/bash

# Generate test cases and update the GENERATED TESTS section in tests.bats

# Create temporary file for new test cases
# Note: We use a temp file here because passing multi-line content to awk
# via variables or command substitution can corrupt newlines. A temp file
# is the most reliable way to preserve formatting.
temp_file=$(mktemp)

# Generate test cases for directories containing main.go
for dir in tests/*/; do
    [ -d "$dir" ] || continue
    if [ -f "$dir/main.go" ]; then
        test_name=$(basename "$dir")

        cat >> "$temp_file" << EOF
@test "$test_name" {
    run_test "$dir"
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