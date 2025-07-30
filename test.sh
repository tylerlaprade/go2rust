#!/bin/bash

# Generate test cases and update the GENERATED TESTS section in tests.bats

# Create temporary file for new test cases
# Note: We use a temp file here because passing multi-line content to awk
# via variables or command substitution can corrupt newlines. A temp file
# is the most reliable way to preserve formatting.
temp_file=$(mktemp)

# Validate XFAIL tests compile
for xfail_dir in $(find tests/XFAIL -maxdepth 1 -type d ! -name XFAIL | sort); do
    if [ -f "$xfail_dir/main.go" ]; then
        # Check if Go code compiles first
        if ! (cd "$xfail_dir" && go build . >/dev/null 2>&1); then
            echo "ERROR: XFAIL test $(basename "$xfail_dir") does not compile"
            exit 1
        fi
    fi
done

# Generate test cases for directories containing main.go
for dir in $(find tests -maxdepth 1 -type d ! -name tests ! -name XFAIL | sort); do
    if [ -f "$dir/main.go" ]; then
        test_name=$(basename "$dir")

        cat >> "$temp_file" << EOF
@test "$test_name" {
    run_test "$dir"
}

EOF
    fi
done

# Generate XFAIL test cases (run and expect to fail)
for xfail_dir in $(find tests/XFAIL -maxdepth 1 -type d ! -name XFAIL | sort); do
    if [ -f "$xfail_dir/main.go" ]; then
        test_name=$(basename "$xfail_dir")

        cat >> "$temp_file" << EOF
@test "XFAIL: $test_name" {
    run_xfail_test "$xfail_dir"
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

# Check if GNU parallel is installed for parallel execution
if command -v parallel >/dev/null 2>&1; then
    # Detect CPU cores but leave some headroom for Rust's memory usage
    CORES=$(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 4)
    # Use 75% of cores (minimum 2) to avoid memory pressure from Rust compilation
    JOBS=$(( CORES * 3 / 4 ))
    [ $JOBS -lt 2 ] && JOBS=2
    echo "Running tests in parallel with $JOBS jobs (detected $CORES cores)..."
    bats -j "$JOBS" tests.bats | tee test_output.tmp
else
    echo "Running tests sequentially (install GNU parallel for faster execution)..."
    bats tests.bats | tee test_output.tmp
fi

# Extract and display test summary with colors
echo ""
echo -e "\033[1mTest Summary:\033[0m"
echo "============="

# Count passing and failing tests (excluding XFAIL)
PASSING=$(grep "^ok " test_output.tmp | grep -v "XFAIL" | wc -l | tr -d ' ')
FAILING=$(grep "^not ok " test_output.tmp | grep -v "XFAIL" | wc -l | tr -d ' ')
TOTAL=$((PASSING + FAILING))

# Display with colors
if [ $FAILING -eq 0 ]; then
    echo -e "\033[32mPassing: $PASSING/$TOTAL ✓\033[0m"
else
    echo -e "\033[32mPassing: $PASSING/$TOTAL\033[0m"
    echo -e "\033[31mFailing: $FAILING/$TOTAL\033[0m"
fi

if [ $FAILING -gt 0 ]; then
    echo ""
    echo -e "\033[31mFailed tests:\033[0m"
    grep "^not ok " test_output.tmp | grep -v "XFAIL" | sed 's/^not ok [0-9]* /  - /'
fi

rm -f test_output.tmp