#!/bin/bash

# Centralized cleanup so the lock and the shard dir share one EXIT trap.
LOCK_DIR=""
SHARD_DIR=""
TEMP_FILE=""
BUILT_TEST_BINARY_DIR=""
TEST_GOCACHE_DIR=""
_test_sh_cleanup() {
    [ -n "$LOCK_DIR" ] && rm -rf "$LOCK_DIR"
    [ -n "$SHARD_DIR" ] && rm -rf "$SHARD_DIR"
    [ -n "$TEMP_FILE" ] && rm -f "$TEMP_FILE" "$TEMP_FILE.bak"
    [ -n "$BUILT_TEST_BINARY_DIR" ] && rm -rf "$BUILT_TEST_BINARY_DIR"
    [ -n "$TEST_GOCACHE_DIR" ] && rm -rf "$TEST_GOCACHE_DIR"
}
trap _test_sh_cleanup EXIT

# Single-instance lock. Concurrent ./test.sh runs would corrupt the in-place
# tests.bats rewrite below and the startup sweep would clobber active per-test
# workspaces. mkdir is atomic. The lock dir name avoids the 'go2rust-test.*'
# prefix so the sweep glob below can't match it.
LOCK_DIR="${TMPDIR:-/tmp}/go2rust-test-sh.lock"
if ! mkdir "$LOCK_DIR" 2>/dev/null; then
    holder=""
    [ -r "$LOCK_DIR/pid" ] && holder=$(cat "$LOCK_DIR/pid" 2>/dev/null)
    if [ -n "$holder" ] && kill -0 "$holder" 2>/dev/null; then
        echo "Error: another ./test.sh is running (pid $holder). Wait for it to finish or kill it." >&2
        LOCK_DIR=""
        exit 1
    fi
    # Stale lock from a prior run that didn't release (SIGKILL/OOM).
    rm -rf "$LOCK_DIR"
    if ! mkdir "$LOCK_DIR" 2>/dev/null; then
        echo "Error: failed to acquire $LOCK_DIR" >&2
        LOCK_DIR=""
        exit 1
    fi
fi
echo $$ > "$LOCK_DIR/pid"

pid_is_active() {
    local pid_file="$1"
    [ -f "$pid_file" ] || return 1

    local pid
    pid=$(cat "$pid_file" 2>/dev/null || true)
    [ -n "$pid" ] || return 1
    kill -0 "$pid" 2>/dev/null
}

maybe_remove_stale_temp_dir() {
    local dir="$1"

    if pid_is_active "$dir/go2rust-test.pid" || pid_is_active "$dir/self_transpile_check.pid" || pid_is_active "$dir/pid"; then
        return
    fi

    rm -rf "$dir"
}

cleanup_stale_test_artifacts() {
    [ "${GO2RUST_TEST_CLEAN_STALE:-1}" = "0" ] && return

    local tmp_roots=()
    add_stale_tmp_root() {
        local root="$1"
        [ -n "$root" ] || return
        case "$root" in
            */) root="${root%/}" ;;
        esac
        [ -n "$root" ] || return
        tmp_roots+=("$root")
    }

    add_stale_tmp_root "${TMPDIR:-}"
    add_stale_tmp_root "/tmp"
    add_stale_tmp_root "/private/tmp"

    local seen_roots=""
    local tmp_root
    for tmp_root in "${tmp_roots[@]}"; do
        [ -d "$tmp_root" ] || continue
        case ":$seen_roots:" in
            *":$tmp_root:"*) continue ;;
        esac
        seen_roots="$seen_roots:$tmp_root"

        find "$tmp_root" -maxdepth 1 \( -name 'go2rust-test.*' -o -name 'go2rust-bats-shards.*' -o -name 'go2rust-cargo-target.*' -o -name 'go2rust-test-binary.*' -o -name 'go2rust-go-cache.*' \) -type d -prune -print 2>/dev/null | while IFS= read -r dir; do
            maybe_remove_stale_temp_dir "$dir"
        done
        find "$tmp_root" -maxdepth 1 \( -name 'go2rust-rust-work.*' \) -type d -prune -print 2>/dev/null | while IFS= read -r dir; do
            maybe_remove_stale_temp_dir "$dir"
        done
        find "$tmp_root" -maxdepth 1 \( -name 'go2rust-tests-list.*' -o -name 'go2rust-rust-diff.*' -o -name 'go2rust-stdout.*' -o -name 'go2rust-stderr.*' \) -type f -delete 2>/dev/null
    done
}

# Generate test cases and update the GENERATED TESTS section in tests.bats

# Create temporary file for new test cases
# Note: We use a temp file here because passing multi-line content to awk
# via variables or command substitution can corrupt newlines. A temp file
# is the most reliable way to preserve formatting.
TEMP_FILE=$(mktemp "${TMPDIR:-/tmp}/go2rust-tests-list.XXXXXX")
temp_file="$TEMP_FILE"

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
TEMP_FILE=""

echo "Updated tests.bats with $(grep -c '^@test' tests.bats) tests"

# Parse command line arguments
VERBOSE=false
JOBS=""
TIMEOUT="15s"
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
    echo "  -n, --jobs N       Number of parallel jobs (default: auto-detect from CPU and memory)"
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

# Sweep stale per-test workspaces left over from prior runs killed via SIGKILL
# (OOM, kill -9, etc.) — SIGKILL bypasses the run_test EXIT trap.
cleanup_stale_test_artifacts

if [ -z "${GOCACHE:-}" ]; then
    TEST_GOCACHE_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-go-cache.XXXXXX")
    echo "$$" > "$TEST_GOCACHE_DIR/go2rust-test.pid"
    export GOCACHE="$TEST_GOCACHE_DIR"
fi

# Build the transpiler once before running the suite. Bats parallelism is
# file-level, so sharded runs would otherwise race multiple setup_file builds
# against the same ./go2rust output path. GO2RUST_TEST_BINARY lets the same
# suite validate a self-transpiled Rust binary without changing test behavior.
if [ -n "${GO2RUST_TEST_BINARY:-}" ]; then
    case "$GO2RUST_TEST_BINARY" in
        /*) ;;
        *) GO2RUST_TEST_BINARY="$(pwd)/$GO2RUST_TEST_BINARY" ;;
    esac
    if [ ! -x "$GO2RUST_TEST_BINARY" ]; then
        echo "Error: GO2RUST_TEST_BINARY is not executable: $GO2RUST_TEST_BINARY"
        exit 1
    fi
    export GO2RUST_TEST_BINARY
else
    BUILT_TEST_BINARY_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-test-binary.XXXXXX")
    echo "$$" > "$BUILT_TEST_BINARY_DIR/go2rust-test.pid"
    BUILT_TEST_BINARY="$BUILT_TEST_BINARY_DIR/go2rust"
    go build -o "$BUILT_TEST_BINARY" ./go
    chmod +x "$BUILT_TEST_BINARY"
    GO2RUST_TEST_BINARY="$BUILT_TEST_BINARY"
    export GO2RUST_TEST_BINARY
fi
export GO2RUST_TEST_BINARY_READY=1

detect_available_memory_bytes() {
    if command -v memory_pressure >/dev/null 2>&1; then
        memory_pressure 2>/dev/null | awk '
            /^The system has / {
                total = $4
            }
            /System-wide memory free percentage:/ {
                pct = $5
                gsub(/%/, "", pct)
            }
            END {
                if (total ~ /^[0-9]+$/ && pct ~ /^[0-9]+$/) {
                    printf "%.0f\n", total * pct / 100
                }
            }
        '
        return
    fi

    if [ -r /proc/meminfo ]; then
        awk '/MemAvailable/ { printf "%.0f\n", $2 * 1024 }' /proc/meminfo 2>/dev/null
        return
    fi

    if command -v vm_stat >/dev/null 2>&1; then
        vm_stat 2>/dev/null | awk '
            /page size of/ {
                page_size = $8
                gsub(/[^0-9]/, "", page_size)
            }
            /Pages free:/ {
                free_pages = $3
                gsub(/[^0-9]/, "", free_pages)
            }
            /Pages speculative:/ {
                speculative_pages = $3
                gsub(/[^0-9]/, "", speculative_pages)
            }
            END {
                if (page_size > 0) {
                    printf "%.0f\n", (free_pages + speculative_pages) * page_size
                }
            }
        '
        return
    fi
}

# Set default job count if not specified
if [ -z "$JOBS" ]; then
    # Detect CPU cores but leave some headroom for Rust's memory usage
    CORES=$(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 4)
    # Use 75% of cores (minimum 2) to avoid memory pressure from Rust compilation
    JOBS=$(( CORES * 3 / 4 ))
    [ $JOBS -lt 2 ] && JOBS=2

    MEM_BYTES=$(sysctl -n hw.memsize 2>/dev/null || awk '/MemTotal/ { printf "%.0f\n", $2 * 1024 }' /proc/meminfo 2>/dev/null || echo 0)
    MEMORY_PER_JOB_GB="${GO2RUST_TEST_MEMORY_PER_JOB_GB:-4}"
    case "$MEM_BYTES" in ''|*[!0-9]*) MEM_BYTES=0 ;; esac
    case "$MEMORY_PER_JOB_GB" in ''|*[!0-9]*) MEMORY_PER_JOB_GB=0 ;; esac
    if [ "$MEMORY_PER_JOB_GB" -gt 0 ]; then
        BYTES_PER_JOB=$(( MEMORY_PER_JOB_GB * 1024 * 1024 * 1024 ))
        if [ "$MEM_BYTES" -gt 0 ]; then
            MEM_JOBS=$(( MEM_BYTES / BYTES_PER_JOB ))
            [ "$MEM_JOBS" -lt 1 ] && MEM_JOBS=1
            [ "$JOBS" -gt "$MEM_JOBS" ] && JOBS=$MEM_JOBS
        fi

        AVAILABLE_MEM_BYTES=$(detect_available_memory_bytes)
        case "$AVAILABLE_MEM_BYTES" in ''|*[!0-9]*) AVAILABLE_MEM_BYTES=0 ;; esac
        if [ "$AVAILABLE_MEM_BYTES" -gt 0 ]; then
            AVAILABLE_MEM_JOBS=$(( AVAILABLE_MEM_BYTES / BYTES_PER_JOB ))
            [ "$AVAILABLE_MEM_JOBS" -lt 1 ] && AVAILABLE_MEM_JOBS=1
            [ "$JOBS" -gt "$AVAILABLE_MEM_JOBS" ] && JOBS=$AVAILABLE_MEM_JOBS
        fi
    fi
    JOBS_MAX="${GO2RUST_TEST_JOBS_MAX:-}"
    case "$JOBS_MAX" in ''|*[!0-9]*) JOBS_MAX=0 ;; esac
    if [ "$JOBS_MAX" -gt 0 ]; then
        [ "$JOBS" -gt "$JOBS_MAX" ] && JOBS=$JOBS_MAX
    fi
fi

if [ "$JOBS" -gt 1 ] && ! command -v parallel >/dev/null 2>&1; then
    echo "GNU parallel is not installed; running tests sequentially."
    JOBS=1
fi

create_bats_shards() {
    local shard_dir="$1"
    local jobs="$2"
    local preamble_file="$shard_dir/preamble.bats"

    awk '/^# BEGIN GENERATED TESTS/ { exit } { print }' tests.bats > "$preamble_file"

    for ((shard = 1; shard <= jobs; shard++)); do
        local shard_file
        shard_file=$(printf "%s/tests-shard-%02d.bats" "$shard_dir" "$shard")
        cp "$preamble_file" "$shard_file"
        echo "# BEGIN GENERATED TESTS - SHARD $shard/$jobs" >> "$shard_file"
    done

    awk -v shard_dir="$shard_dir" -v jobs="$jobs" '
        function emit_block() {
            if (block == "") {
                return
            }
            shard = (test_index % jobs) + 1
            file = sprintf("%s/tests-shard-%02d.bats", shard_dir, shard)
            printf "%s", block >> file
            close(file)
            block = ""
            test_index++
        }
        /^# BEGIN GENERATED TESTS/ {
            in_tests = 1
            next
        }
        /^# END GENERATED TESTS/ {
            emit_block()
            in_tests = 0
            next
        }
        in_tests && /^@test / {
            emit_block()
            block = $0 ORS
            next
        }
        in_tests {
            if (block != "") {
                block = block $0 ORS
            }
            next
        }
    ' tests.bats

    for ((shard = 1; shard <= jobs; shard++)); do
        local shard_file
        shard_file=$(printf "%s/tests-shard-%02d.bats" "$shard_dir" "$shard")
        echo "# END GENERATED TESTS" >> "$shard_file"
    done
}

# Function to colorize test output
colorize_output() {
    local in_failure=false
    while IFS= read -r line; do
        if [[ "$line" =~ ^"not ok " ]]; then
            # Start of failing test - red X
            local test_name
            test_name=$(echo "$line" | sed 's/^not ok [0-9]* //')
            # Extract timing if present
            if [[ "$test_name" =~ (.+)" in "([0-9]+ms) ]]; then
                test_name="${BASH_REMATCH[1]}"
                timing_ms="${BASH_REMATCH[2]%ms}"
                timing_s=$(awk "BEGIN {printf \"%.1f\", $timing_ms/1000}")
                if [ "$VERBOSE" = true ]; then
                    echo -e "\033[31m✗ $test_name\033[0m \033[90m(${timing_s}s)\033[0m"
                else
                    echo -e "\033[31m✗ $test_name\033[0m"
                fi
            else
                echo -e "\033[31m✗ $test_name\033[0m"
            fi
            in_failure=true
        elif [[ "$line" =~ ^"ok " ]] && [[ "$line" =~ " XFAIL:" ]]; then
            # XFAIL tests - yellow ⚠ (only show if verbose)
            if [ "$VERBOSE" = true ]; then
                local test_name
                test_name=$(echo "$line" | sed 's/^ok [0-9]* //')
                # Extract timing if present
                if [[ "$test_name" =~ (.+)" in "([0-9]+ms) ]]; then
                    test_name="${BASH_REMATCH[1]}"
                    timing_ms="${BASH_REMATCH[2]%ms}"
                    timing_s=$(awk "BEGIN {printf \"%.1f\", $timing_ms/1000}")
                    echo -e "\033[33m⚠ $test_name\033[0m \033[90m(${timing_s}s)\033[0m"
                else
                    echo -e "\033[33m⚠ $test_name\033[0m"
                fi
            fi
            in_failure=false
        elif [[ "$line" =~ ^"ok " ]]; then
            # Passing test - green checkmark
            local test_name
            test_name=$(echo "$line" | sed 's/^ok [0-9]* //')
            # Extract timing if present
            if [[ "$test_name" =~ (.+)" in "([0-9]+ms) ]]; then
                test_name="${BASH_REMATCH[1]}"
                timing_ms="${BASH_REMATCH[2]%ms}"
                timing_s=$(awk "BEGIN {printf \"%.1f\", $timing_ms/1000}")
                if [ "$VERBOSE" = true ]; then
                    echo -e "\033[32m✓\033[0m $test_name \033[90m(${timing_s}s)\033[0m"
                else
                    echo -e "\033[32m✓\033[0m $test_name"
                fi
            else
                echo -e "\033[32m✓\033[0m $test_name"
            fi
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
export TEST_TIMEOUT_KILL_AFTER="${TEST_TIMEOUT_KILL_AFTER:-5s}"

# Build filter pattern if specific tests requested
FILTER_PATTERN=""
if [ ${#TEST_NAMES[@]} -gt 0 ]; then
    # Build a regex pattern that matches any of the test names
    FILTER_PATTERN="("
    for i in "${!TEST_NAMES[@]}"; do
        if [ "$i" -gt 0 ]; then
            FILTER_PATTERN+="|"
        fi
        # Escape special regex characters and handle both regular and XFAIL tests
        escaped_name=$(echo "${TEST_NAMES[$i]}" | sed 's/[[\.*^$()+?{|]/\\&/g')
        FILTER_PATTERN+="^${escaped_name}$|^XFAIL: ${escaped_name}$"
    done
    FILTER_PATTERN+=")"
    echo "Running tests matching: ${TEST_NAMES[*]}"
fi

# Record start time
START_TIME=$(date +%s)

# Build bats command based on mode
BATS_ARGS=(-T --tap)
BATS_TARGETS=(tests.bats)
SHARD_DIR=""
if [ "$JOBS" -eq 1 ]; then
    echo "Running tests sequentially (timeout: $TIMEOUT per test)..."
else
    echo "Running tests in parallel with $JOBS jobs (timeout: $TIMEOUT per test)..."
    SHARD_DIR=$(mktemp -d "${TMPDIR:-/tmp}/go2rust-bats-shards.XXXXXX")
    echo "$$" > "$SHARD_DIR/go2rust-test.pid"
    create_bats_shards "$SHARD_DIR" "$JOBS"
    BATS_ARGS+=(-j "$JOBS")
    BATS_TARGETS=("$SHARD_DIR"/tests-shard-*.bats)
fi

# Add filter if specified
if [ -n "$FILTER_PATTERN" ]; then
    BATS_ARGS+=(--filter "$FILTER_PATTERN")
fi

# Run tests and capture output
TEST_OUTPUT=$(bats "${BATS_ARGS[@]}" "${BATS_TARGETS[@]}" 2>&1)
BATS_STATUS=$?

# Display the output with colors
echo "$TEST_OUTPUT" | colorize_output

# Display test summary for both modes
echo ""
echo -e "\033[1mTest Summary:\033[0m"
echo "============="

# Calculate and display total time
END_TIME=$(date +%s)
TOTAL_TIME=$((END_TIME - START_TIME))
echo -e "\033[90mTotal time: ${TOTAL_TIME}s\033[0m"

# Count all test types from the captured output
PASSING=$(echo "$TEST_OUTPUT" | grep "^ok " | grep -v "XFAIL" | wc -l | tr -d ' ')
FAILING=$(echo "$TEST_OUTPUT" | grep "^not ok " | grep -v "XFAIL" | wc -l | tr -d ' ')
# Only count XFAIL tests from actual test result lines (ok or not ok), not error messages
XFAIL_TOTAL=$(echo "$TEST_OUTPUT" | grep -E "^(ok |not ok )" | grep "XFAIL" | wc -l | tr -d ' ')
TOTAL=$((PASSING + FAILING + XFAIL_TOTAL))

if [ "$TOTAL" -eq 0 ]; then
    echo -e "\033[31m✗ No test results were reported.\033[0m"
    if [ -n "$TEST_OUTPUT" ]; then
        echo "$TEST_OUTPUT"
    fi
    exit 1
fi

# Display with colors and symbols
echo -e "\033[32m✓ Passing: $PASSING/$TOTAL\033[0m"
if [ "$FAILING" -gt 0 ]; then
    echo -e "\033[31m✗ Failing: $FAILING/$TOTAL\033[0m"
fi
echo -e "\033[33m⚠ XFAIL: $XFAIL_TOTAL/$TOTAL\033[0m"

if [ "$FAILING" -gt 0 ]; then
    echo ""
    echo -e "\033[31mFailed tests:\033[0m"
    echo "$TEST_OUTPUT" | grep "^not ok " | grep -v "XFAIL" | while IFS= read -r line; do
        echo -e "\033[31m  - $(echo "$line" | sed 's/^not ok [0-9]* //')\033[0m"
    done
fi

exit "$BATS_STATUS"
