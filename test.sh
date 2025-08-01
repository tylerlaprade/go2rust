#!/bin/bash

# Cleanup function to remove temporary files
cleanup() {
    rm -f test_output.tmp
}

# Set trap to cleanup on exit or interrupt
trap cleanup EXIT INT TERM

# Generate test cases and update the GENERATED TESTS section in tests.bats

# Create temporary file for new test cases
# Note: We use a temp file here because passing multi-line content to awk
# via variables or command substitution can corrupt newlines. A temp file
# is the most reliable way to preserve formatting.
temp_file=$(mktemp)

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

# Parse command line arguments
VERBOSE=false
JOBS=""
TIMEOUT="60s"
HELP=false
TEST_NAMES=()

i=1
skip_next=false
for arg in "$@"; do
    if [ "$skip_next" = true ]; then
        skip_next=false
        i=$((i+1))
        continue
    fi
    
    case $arg in
        -h|--help)
            HELP=true
            ;;
        -v|--verbose)
            VERBOSE=true
            ;;
        -n|--jobs)
            # Next argument should be the number
            eval "JOBS=\${$((i+1))}"
            skip_next=true
            ;;
        -n*)
            # -n4 format
            JOBS="${arg#-n}"
            ;;
        --jobs=*)
            # --jobs=4 format
            JOBS="${arg#--jobs=}"
            ;;
        -t|--timeout)
            # Next argument should be the timeout
            eval "TIMEOUT=\${$((i+1))}"
            skip_next=true
            ;;
        -t*)
            # -t30s format
            TIMEOUT="${arg#-t}"
            ;;
        --timeout=*)
            # --timeout=30s format
            TIMEOUT="${arg#--timeout=}"
            ;;
        -*)
            echo "Unknown option: $arg"
            exit 1
            ;;
        *)
            # Not a flag, assume it's a test name
            TEST_NAMES+=("$arg")
            ;;
    esac
    i=$((i+1))
done

# Show help if requested
if [ "$HELP" = true ]; then
    echo ""
    echo "Usage: $0 [options] [test_names...]"
    echo ""
    echo "Options:"
    echo "  -v, --verbose      Show XFAIL tests in output"
    echo "  -n, --jobs N       Number of parallel jobs (default: auto-detect)"
    echo "  -t, --timeout TIME Timeout per test (default: 60s)"
    echo "  -h, --help         Show this help message"
    echo ""
    echo "Arguments:"
    echo "  test_names         Specific tests to run (default: all tests)"
    echo ""
    echo "Examples:"
    echo "  $0                  # Run all tests"
    echo "  $0 hello_world      # Run specific test"
    echo "  $0 -v methods_basic # Run specific test with verbose output"
    echo "  $0 -n 1 foo bar     # Run multiple tests sequentially"
    exit 0
fi

# Run tests
if ! command -v bats >/dev/null 2>&1; then
    echo "Error: bats is not installed"
    echo "Install with: brew install bats-core"
    exit 1
fi

# Check if we're running specific XFAIL tests
SHOW_XFAIL_ERRORS=false
if [ ${#TEST_NAMES[@]} -gt 0 ]; then
    # Check if any of the requested tests are XFAIL
    for test_name in "${TEST_NAMES[@]}"; do
        if [ -d "tests/XFAIL/$test_name" ]; then
            SHOW_XFAIL_ERRORS=true
            break
        fi
    done
fi

# Export for use in tests.bats
export SHOW_XFAIL_ERRORS

# Set default job count if not specified
if [ -z "$JOBS" ]; then
    # Detect CPU cores but leave some headroom for Rust's memory usage
    CORES=$(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 4)
    # Use 75% of cores (minimum 2) to avoid memory pressure from Rust compilation
    JOBS=$(( CORES * 3 / 4 ))
    [ $JOBS -lt 2 ] && JOBS=2
fi

# Function to colorize test output
colorize_output() {
    local in_failure=false
    while IFS= read -r line; do
        if [[ "$line" =~ ^"not ok " ]]; then
            # Start of failing test - red X
            local test_name
            test_name=$(echo "$line" | sed 's/^not ok [0-9]* //')
            echo -e "\033[31m✗ $test_name\033[0m"
            in_failure=true
        elif [[ "$line" =~ ^"ok " ]] && [[ "$line" =~ " XFAIL:" ]]; then
            # XFAIL tests - yellow ⚠ (only show if verbose)
            if [ "$VERBOSE" = true ]; then
                local test_name
                test_name=$(echo "$line" | sed 's/^ok [0-9]* //')
                echo -e "\033[33m⚠ $test_name\033[0m"
            fi
            in_failure=false
        elif [[ "$line" =~ ^"ok " ]]; then
            # Passing test - green checkmark
            local test_name
            test_name=$(echo "$line" | sed 's/^ok [0-9]* //')
            echo -e "\033[32m✓\033[0m $test_name"
            in_failure=false
        elif [[ "$line" =~ ^"#" ]] && [ "$in_failure" = true ]; then
            # Error details from failed test - red
            echo -e "\033[31m$line\033[0m"
        elif [[ "$line" =~ ^[0-9]+\.\.[0-9]+ ]]; then
            # Test count header - show it
            echo "$line"
        else
            # Other output - only show if verbose or if it's an error
            if [ "$VERBOSE" = true ] || [ "$in_failure" = true ]; then
                echo "$line"
            fi
        fi
    done
}

# Export timeout for bats to use
export TEST_TIMEOUT="$TIMEOUT"

# Build filter pattern if specific tests requested
FILTER_PATTERN=""
if [ ${#TEST_NAMES[@]} -gt 0 ]; then
    # Build a regex pattern that matches any of the test names
    FILTER_PATTERN="("
    for i in "${!TEST_NAMES[@]}"; do
        if [ $i -gt 0 ]; then
            FILTER_PATTERN+="|"
        fi
        # Escape special regex characters and handle both regular and XFAIL tests
        escaped_name=$(echo "${TEST_NAMES[$i]}" | sed 's/[[\.*^$()+?{|]/\\&/g')
        FILTER_PATTERN+="^${escaped_name}$|^XFAIL: ${escaped_name}$"
    done
    FILTER_PATTERN+=")"
    echo "Running tests matching: ${TEST_NAMES[*]}"
fi

# Choose execution mode based on job count
if [ "$JOBS" -eq 1 ]; then
    # Sequential mode - real-time output
    echo "Running tests sequentially (timeout: $TIMEOUT per test)..."
    if [ -n "$FILTER_PATTERN" ]; then
        bats --tap --filter "$FILTER_PATTERN" tests.bats | colorize_output
    else
        bats --tap tests.bats | colorize_output
    fi
else
    # Parallel mode - buffered output
    echo "Running tests in parallel with $JOBS jobs (timeout: $TIMEOUT per test)..."
    if [ -n "$FILTER_PATTERN" ]; then
        bats -j "$JOBS" --filter "$FILTER_PATTERN" tests.bats | tee test_output.tmp | colorize_output
    else
        bats -j "$JOBS" tests.bats | tee test_output.tmp | colorize_output
    fi
fi

# Extract and display test summary with colors (only for parallel mode)
if [ "$JOBS" -gt 1 ] && [ -f test_output.tmp ]; then
    echo ""
    echo -e "\033[1mTest Summary:\033[0m"
    echo "============="

    # Count all test types
    PASSING=$(grep "^ok " test_output.tmp | grep -v "XFAIL" | wc -l | tr -d ' ')
    FAILING=$(grep "^not ok " test_output.tmp | grep -v "XFAIL" | wc -l | tr -d ' ')
    XFAIL_TOTAL=$(grep "XFAIL" test_output.tmp | wc -l | tr -d ' ')
    TOTAL=$((PASSING + FAILING + XFAIL_TOTAL))

    # Display with colors and symbols
    echo -e "\033[32m✓ Passing: $PASSING/$TOTAL\033[0m"
    if [ "$FAILING" -gt 0 ]; then
        echo -e "\033[31m✗ Failing: $FAILING/$TOTAL\033[0m"
    fi
    echo -e "\033[33m⚠ XFAIL: $XFAIL_TOTAL/$TOTAL\033[0m"

    if [ "$FAILING" -gt 0 ]; then
        echo ""
        echo -e "\033[31mFailed tests:\033[0m"
        grep "^not ok " test_output.tmp | grep -v "XFAIL" | while IFS= read -r line; do
            echo -e "\033[31m  - $(echo "$line" | sed 's/^not ok [0-9]* //')\033[0m"
        done
    fi

fi