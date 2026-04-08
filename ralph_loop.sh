#!/usr/bin/env bash
# Autonomous Ralph loop for go2rust
# Usage: ./ralph_loop.sh [max_iterations] [max_turns]
#
# Examples:
#   ./ralph_loop.sh           # 50 iterations, 25 turns each
#   ./ralph_loop.sh 10        # 10 iterations
#   ./ralph_loop.sh 50 40     # 50 iterations, 40 turns each

# Prevent sleep while looping (background, tied to our PID)
caffeinate -dims -w $$ &

MAX_ITERATIONS="${1:-50}"
MAX_TURNS="${2:-25}"
TIMEOUT_MINS=15
LOGDIR="$(pwd)/.ralph-loop-logs"
ANALYSIS_DIR="$(pwd)/.analysis"
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

SPINNER_PID=""
trap 'printf "\r\033[K"; [ -n "$SPINNER_PID" ] && kill "$SPINNER_PID" 2>/dev/null; echo "Loop interrupted."; exit 130' INT

# Record baseline test count before starting (count test dirs)
BASELINE_PASS=$(ls tests/ | grep -v XFAIL | grep -v '\.bats' | grep -v README | wc -l | tr -d ' ')

ANALYSIS_EVERY=5  # check analysis queue every Nth iteration
DEFAULT_MODEL="sonnet"
ANALYSIS_MODEL="opus"
BASE_PROMPT='Fix XFAIL tests. Follow the protocol in LOOP_PROTOCOL.md. HARD RULE: if ./test.sh shows any previously-passing test now failing, revert your changes immediately with git checkout -- go/ before trying anything else.'

# Returns "prompt|model" — caller splits on |
build_prompt() {
    local iter="$1"
    local prompt="$BASE_PROMPT"
    local model="$DEFAULT_MODEL"

    # Only check analysis queue every Nth iteration
    if [ $(( iter % ANALYSIS_EVERY )) -eq 0 ] && [ -d "$ANALYSIS_DIR" ]; then
        local task
        task=$(ls -t "$ANALYSIS_DIR"/*.md 2>/dev/null | grep -v README.md | tail -1)
        if [ -n "$task" ]; then
            local task_name
            task_name=$(basename "$task")
            prompt="$prompt

OPTIONAL ANALYSIS: If you have turns to spare after fixing an XFAIL test, also read .analysis/$task_name and address it. Delete the file when done. XFAIL work comes first."
            model="$ANALYSIS_MODEL"
        fi
    fi

    printf '%s|%s' "$prompt" "$model"
}

echo "ralph loop — baseline: $BASELINE_PASS passing — max $MAX_TURNS turns, ${TIMEOUT_MINS}m timeout"
echo "logs: $LOGDIR/"
echo ""

# Pre-flight: verify claude can start
if ! claude --dangerously-skip-permissions -p "ok" --output-format text --max-turns 1 >/dev/null 2>&1; then
    echo "ERROR: claude preflight failed — check auth/network"
    exit 1
fi

LOOP_START=$SECONDS

for i in $(seq 1 "$MAX_ITERATIONS"); do
    LOGFILE="$LOGDIR/$(date +%Y%m%d-%H%M%S).log"

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

    SESSION_START=$SECONDS
    PROMPT_MODEL=$(build_prompt "$i")
    PROMPT="${PROMPT_MODEL%|*}"
    MODEL="${PROMPT_MODEL##*|}"

    # Background spinner
    _spinner_iter=$i _spinner_max=$MAX_ITERATIONS _spinner_pass=$PASSING _spinner_xfail=$XFAIL _spinner_model=$MODEL
    (
        FRAMES=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')
        TICK=0
        while true; do
            F=${FRAMES[$(( TICK % ${#FRAMES[@]} ))]}
            DUR=$(format_duration "$TICK")
            printf "\r\033[K%s  iter %d/%d  ✓%d ✗%d  %s  %s  %s" \
                "$F" "$_spinner_iter" "$_spinner_max" "$_spinner_pass" "$_spinner_xfail" \
                "$_spinner_model" "$DUR" "running…"
            sleep 1
            TICK=$((TICK + 1))
        done
    ) &
    SPINNER_PID=$!

    ERRFILE="${LOGFILE%.log}.err"
    timeout "${TIMEOUT_MINS}m" claude \
        --dangerously-skip-permissions \
        --verbose \
        --model "$MODEL" \
        --max-turns "$MAX_TURNS" \
        -p "$PROMPT" \
        --output-format text \
        >"$LOGFILE" 2>"$ERRFILE"
    EXIT_CODE=$?

    # Kill spinner
    kill "$SPINNER_PID" 2>/dev/null
    wait "$SPINNER_PID" 2>/dev/null

    ELAPSED=$(( SECONDS - SESSION_START ))
    LINES=$(wc -l < "$LOGFILE" | tr -d ' ')

    if [ "$EXIT_CODE" -eq 124 ]; then
        event "TIMEOUT  iter $i  $(format_duration $ELAPSED)"
    elif [ "$LINES" -eq 0 ]; then
        if [ -s "$ERRFILE" ]; then
            event "FAILED (exit $EXIT_CODE): $(head -1 "$ERRFILE")"
        else
            event "warning: empty log (exit $EXIT_CODE)"
        fi
    else
        event "done  iter $i/$MAX_ITERATIONS  ✓$PASSING ✗$XFAIL  $(format_duration $ELAPSED)  ${LINES}L"
    fi

    # Safety net: auto-stage test file changes that claude forgot
    git add tests/ tests.bats 2>/dev/null
    if ! git diff --cached --quiet 2>/dev/null; then
        git commit -q -m "Auto-commit: test output updates (session $i)"
    fi

    sleep 2
done

TOTAL=$(( SECONDS - LOOP_START ))
PASSING=$(ls tests/ | grep -v XFAIL | grep -v '\.bats' | grep -v README | wc -l | tr -d ' ')
XFAIL=$(ls tests/XFAIL/ 2>/dev/null | wc -l | tr -d ' ')
event "done — $PASSING passing, $XFAIL xfail (started at $BASELINE_PASS) — $(format_duration $TOTAL) total"
