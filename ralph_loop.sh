#!/usr/bin/env bash
# Autonomous Claude Code loop for go2rust
# Usage: ./run_loop.sh [phase] [max_iterations]
#
# Phases:
#   a - Pick off easy/medium XFAIL tests (default)
#   b - Build concurrency subsystem (channels, goroutines, select)
#   c - Build multi-file package support
#   d - Add stdlib mappings
#   e - Extend selective wrapping (rearchitecture stages 2-5)
#
# Examples:
#   ./run_loop.sh        # Phase A, 50 iterations
#   ./run_loop.sh b 10   # Phase B, 10 iterations
#   ./run_loop.sh e      # Phase E, 50 iterations

PHASE="${1:-a}"
MAX_ITERATIONS="${2:-50}"
LOGDIR="$(pwd)/.claude-loop-logs"
mkdir -p "$LOGDIR"

# Ctrl+C kills the whole loop, not just the current claude session
trap 'echo ""; echo "Loop interrupted."; exit 130' INT

# Record baseline test count before starting (count test dirs)
BASELINE_PASS=$(ls tests/ | grep -v XFAIL | grep -v '\.bats' | grep -v README | wc -l | tr -d ' ')

case "$PHASE" in
  a)
    PHASE_NAME="Easy/Medium XFAIL tests"
    PROMPT='AUTONOMOUS PHASE A: Pick off easy XFAIL tests. Follow the phase A protocol in .claude/loop-phases/phase-a.md. HARD RULE: if ./test.sh shows any previously-passing test now failing, revert your changes immediately with git checkout -- go/ before trying anything else.'
    ;;
  b)
    PHASE_NAME="Concurrency subsystem"
    PROMPT='AUTONOMOUS PHASE B: Build the concurrency subsystem. Follow the phase B protocol in .claude/loop-phases/phase-b.md. HARD RULE: if ./test.sh shows any previously-passing test now failing, revert your changes immediately with git checkout -- go/ before trying anything else.'
    ;;
  c)
    PHASE_NAME="Multi-file package support"
    PROMPT='AUTONOMOUS PHASE C: Build multi-file package support. Follow the phase C protocol in .claude/loop-phases/phase-c.md. HARD RULE: if ./test.sh shows any previously-passing test now failing, revert your changes immediately with git checkout -- go/ before trying anything else.'
    ;;
  d)
    PHASE_NAME="Stdlib mappings"
    PROMPT='AUTONOMOUS PHASE D: Add stdlib mappings. Follow the phase D protocol in .claude/loop-phases/phase-d.md. HARD RULE: if ./test.sh shows any previously-passing test now failing, revert your changes immediately with git checkout -- go/ before trying anything else.'
    ;;
  e)
    PHASE_NAME="Selective wrapping (rearchitecture)"
    PROMPT='AUTONOMOUS PHASE E: Extend selective wrapping. Follow the phase E protocol in .claude/loop-phases/phase-e.md. HARD RULE: if ./test.sh shows any previously-passing test now failing, revert your changes immediately with git checkout -- go/ before trying anything else.'
    ;;
  *)
    echo "Unknown phase: $PHASE"
    echo "Valid phases: a, b, c, d, e"
    exit 1
    ;;
esac

echo "=== Go2Rust Autonomous Loop ==="
echo "Phase: $PHASE — $PHASE_NAME"
echo "Max iterations: $MAX_ITERATIONS"
echo "Baseline passing: $BASELINE_PASS"
echo "Logs: $LOGDIR/"
echo "Press Ctrl+C to stop"
echo ""

for i in $(seq 1 "$MAX_ITERATIONS"); do
    echo "--- Iteration $i / $MAX_ITERATIONS ($(date)) ---"
    LOGFILE="$LOGDIR/phase-${PHASE}-$(date +%Y%m%d-%H%M%S).log"

    # Count current passing/failing
    PASSING=$(ls tests/ | grep -v XFAIL | grep -v '\.bats' | grep -v README | wc -l | tr -d ' ')
    XFAIL=$(ls tests/XFAIL/ 2>/dev/null | wc -l | tr -d ' ')
    echo "Status: $PASSING passing, $XFAIL XFAIL (baseline: $BASELINE_PASS)"

    # REGRESSION GATE: if passing count dropped below baseline, stop
    if [ "$PASSING" -lt "$BASELINE_PASS" ]; then
        echo "REGRESSION DETECTED: $PASSING passing < $BASELINE_PASS baseline"
        echo "Stopping loop. Check git log and fix manually."
        exit 1
    fi

    # Update baseline if we gained tests
    if [ "$PASSING" -gt "$BASELINE_PASS" ]; then
        BASELINE_PASS="$PASSING"
        echo "New baseline: $BASELINE_PASS"
    fi

    if [ "$XFAIL" -eq 0 ]; then
        echo "All tests passing! Nothing left to do."
        break
    fi

    # Run Claude Code in print mode (non-interactive, runs to completion)
    # --dangerously-skip-permissions: no confirmation prompts
    # -p: print mode (agentic, exits when done)
    # --max-budget-usd: cost guardrail per session
    # --output-format text ensures plain text output (not JSON)
    # tee splits to stdout (real-time in terminal) and log file
    claude --dangerously-skip-permissions -p "$PROMPT" --output-format text 2>&1 | tee "$LOGFILE"
    EXIT_CODE=${PIPESTATUS[0]}

    echo ""
    echo "Session $i complete (exit code: $EXIT_CODE). Log: $LOGFILE"
    LINES=$(wc -l < "$LOGFILE" | tr -d ' ')
    echo "Log size: $LINES lines"

    if [ "$LINES" -eq 0 ]; then
        echo "WARNING: Empty log — claude may have failed to start. Check above for errors."
    fi

    # Auto-stage test file changes: promoted tests, regenerated .rs/.toml output, tests.bats
    git add tests/ tests.bats 2>/dev/null
    if ! git diff --cached --quiet 2>/dev/null; then
        echo "Auto-committing test file changes..."
        git commit -m "Auto-commit: test output updates (phase $PHASE, session $i)"
        echo "Auto-committed."
    fi

    echo ""
    sleep 2
done

echo "=== Loop finished ==="
PASSING=$(ls tests/ | grep -v XFAIL | grep -v '\.bats' | grep -v README | wc -l | tr -d ' ')
XFAIL=$(ls tests/XFAIL/ 2>/dev/null | wc -l | tr -d ' ')
echo "Final: $PASSING passing, $XFAIL XFAIL (started at: $BASELINE_PASS)"
