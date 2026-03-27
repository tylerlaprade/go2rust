#!/usr/bin/env bash
# Autonomous Ralph loop for go2rust
# Usage: ./ralph_loop.sh [phase] [max_iterations]
#
# Phases:
#   a - Pick off easy/medium XFAIL tests (default)
#   b - Build concurrency subsystem (channels, goroutines, select)
#   c - Build multi-file package support
#   d - Add stdlib mappings
#   e - Extend selective wrapping (rearchitecture stages 2-5)
#
# Examples:
#   ./ralph_loop.sh        # Phase A, 50 iterations
#   ./ralph_loop.sh b 10   # Phase B, 10 iterations
#   ./ralph_loop.sh e      # Phase E, 50 iterations

PHASE="${1:-a}"
MAX_ITERATIONS="${2:-50}"
LOGDIR="$(pwd)/.ralph-loop-logs"
mkdir -p "$LOGDIR"

# Status line helpers
status() {
    printf "\r\033[K%s" "$1"
}

event() {
    printf "\r\033[K%s\n" "$1"
}

format_duration() {
    local secs=$1
    if [ "$secs" -ge 3600 ]; then
        printf "%dh%02dm" $((secs/3600)) $((secs%3600/60))
    elif [ "$secs" -ge 60 ]; then
        printf "%dm%02ds" $((secs/60)) $((secs%60))
    else
        printf "%ds" "$secs"
    fi
}

# Ctrl+C kills the whole loop, not just the current ralph session
trap 'printf "\r\033[K"; echo "Loop interrupted."; exit 130' INT

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

echo "ralph loop — phase $PHASE ($PHASE_NAME) — baseline: $BASELINE_PASS passing"
echo "logs: $LOGDIR/"
echo ""

# Pre-flight: verify claude can start
if ! claude --dangerously-skip-permissions -p "ok" --output-format text --max-turns 1 >/dev/null 2>&1; then
    echo "ERROR: claude preflight failed — check auth/network"
    exit 1
fi

LOOP_START=$SECONDS

for i in $(seq 1 "$MAX_ITERATIONS"); do
    LOGFILE="$LOGDIR/phase-${PHASE}-$(date +%Y%m%d-%H%M%S).log"

    # Count current passing/failing
    PASSING=$(ls tests/ | grep -v XFAIL | grep -v '\.bats' | grep -v README | wc -l | tr -d ' ')
    XFAIL=$(ls tests/XFAIL/ 2>/dev/null | wc -l | tr -d ' ')

    # REGRESSION GATE: if passing count dropped below baseline, stop
    if [ "$PASSING" -lt "$BASELINE_PASS" ]; then
        event "REGRESSION: $PASSING passing < $BASELINE_PASS baseline — stopping"
        exit 1
    fi

    # Update baseline if we gained tests
    if [ "$PASSING" -gt "$BASELINE_PASS" ]; then
        event "promoted: $BASELINE_PASS → $PASSING passing"
        BASELINE_PASS="$PASSING"
    fi

    if [ "$XFAIL" -eq 0 ]; then
        event "all tests passing — nothing left to do"
        break
    fi

    # Run claude, output to log only. Spinner updates status line in background.
    SESSION_START=$SECONDS

    # Background spinner — updates status line with elapsed time
    _spinner_iter=$i _spinner_max=$MAX_ITERATIONS _spinner_pass=$PASSING _spinner_xfail=$XFAIL
    (
        FRAMES=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')
        TICK=0
        while true; do
            F=${FRAMES[$(( TICK % ${#FRAMES[@]} ))]}
            if [ "$TICK" -ge 3600 ]; then
                DUR=$(printf "%dh%02dm" $((TICK/3600)) $((TICK%3600/60)))
            elif [ "$TICK" -ge 60 ]; then
                DUR=$(printf "%dm%02ds" $((TICK/60)) $((TICK%60)))
            else
                DUR=$(printf "%ds" "$TICK")
            fi
            printf "\r\033[K%s  iter %d/%d  ✓%d ✗%d  %s  %s" \
                "$F" "$_spinner_iter" "$_spinner_max" "$_spinner_pass" "$_spinner_xfail" \
                "$DUR" "running session…"
            sleep 1
            TICK=$((TICK + 1))
        done
    ) &
    SPINNER_PID=$!

    ERRFILE="${LOGFILE%.log}.err"
    claude --dangerously-skip-permissions -p "$PROMPT" --output-format text >"$LOGFILE" 2>"$ERRFILE"
    EXIT_CODE=$?

    # Kill spinner
    kill "$SPINNER_PID" 2>/dev/null
    wait "$SPINNER_PID" 2>/dev/null

    ELAPSED=$(( SECONDS - SESSION_START ))
    LINES=$(wc -l < "$LOGFILE" | tr -d ' ')

    if [ "$LINES" -eq 0 ]; then
        if [ -s "$ERRFILE" ]; then
            event "FAILED (exit $EXIT_CODE): $(head -1 "$ERRFILE")"
        else
            event "warning: empty log — ralph may have failed to start (exit $EXIT_CODE)"
        fi
    else
        status "done  iter $i/$MAX_ITERATIONS  ✓$PASSING ✗$XFAIL  $(format_duration $ELAPSED)  ${LINES}L logged"
    fi

    # Auto-stage test file changes: promoted tests, regenerated .rs/.toml output, tests.bats
    git add tests/ tests.bats 2>/dev/null
    if ! git diff --cached --quiet 2>/dev/null; then
        git commit -q -m "Auto-commit: test output updates (phase $PHASE, session $i)"
    fi

    sleep 2
done

TOTAL=$(( SECONDS - LOOP_START ))
PASSING=$(ls tests/ | grep -v XFAIL | grep -v '\.bats' | grep -v README | wc -l | tr -d ' ')
XFAIL=$(ls tests/XFAIL/ 2>/dev/null | wc -l | tr -d ' ')
event "done — $PASSING passing, $XFAIL xfail (started at $BASELINE_PASS) — $(format_duration $TOTAL) total"
