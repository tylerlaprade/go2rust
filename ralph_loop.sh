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

queue_files() {
    local path
    shopt -s nullglob
    for path in "$ANALYSIS_DIR"/*.md; do
        [ "$(basename "$path")" = "README.md" ] && continue
        printf '%s\n' "$path"
    done
    shopt -u nullglob
}

count_queue_files() {
    local count=0
    local _
    while IFS= read -r _; do
        count=$((count + 1))
    done < <(queue_files)
    printf '%s\n' "$count"
}

count_passing_tests() {
    local count=0
    local path
    shopt -s nullglob
    for path in tests/*; do
        [ -d "$path" ] || continue
        [ "$(basename "$path")" = "XFAIL" ] && continue
        count=$((count + 1))
    done
    shopt -u nullglob
    printf '%s\n' "$count"
}

count_xfail_tests() {
    local count=0
    local path
    shopt -s nullglob
    for path in tests/XFAIL/*; do
        [ -d "$path" ] || continue
        count=$((count + 1))
    done
    shopt -u nullglob
    printf '%s\n' "$count"
}

file_mtime() {
    if stat -f '%m' "$1" >/dev/null 2>&1; then
        stat -f '%m' "$1"
    else
        stat -c '%Y' "$1"
    fi
}

next_analysis_task() {
    local path
    local oldest_path=""
    local oldest_mtime=""
    local mtime

    while IFS= read -r path; do
        mtime="$(file_mtime "$path")" || continue
        if [ -z "$oldest_path" ] || [ "$mtime" -lt "$oldest_mtime" ] || { [ "$mtime" -eq "$oldest_mtime" ] && [[ "$path" < "$oldest_path" ]]; }; then
            oldest_path="$path"
            oldest_mtime="$mtime"
        fi
    done < <(queue_files)

    printf '%s\n' "$oldest_path"
}

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
BASELINE_PASS="$(count_passing_tests)"

DEFAULT_MODEL="sonnet"
ANALYSIS_MODEL="opus"
BASE_PROMPT='Fix XFAIL tests. Follow the protocol in LOOP_PROTOCOL.md. HARD RULE: if ./test.sh shows any previously-passing test now failing, revert your changes immediately with git checkout -- go/ before trying anything else.'

# Returns "prompt|model" — caller splits on |
build_prompt() {
    local task="$1"
    local prompt="$BASE_PROMPT"
    local model="$DEFAULT_MODEL"

    if [ -n "$task" ]; then
        local task_name
        task_name=$(basename "$task")
        prompt="$prompt

REQUIRED ANALYSIS TASK: Read .analysis/$task_name first and handle it before any XFAIL work. Follow LOOP_PROTOCOL.md exactly:
- Address the queued task first.
- Delete .analysis/$task_name when done.
- Commit that work.
- If turns remain and there are still XFAIL tests, continue with normal XFAIL work."
        model="$ANALYSIS_MODEL"
    fi

    printf '%s|%s' "$prompt" "$model"
}

echo "ralph loop — baseline: $BASELINE_PASS passing — max $MAX_TURNS turns, ${TIMEOUT_MINS}m timeout"
echo "logs: $LOGDIR/"
echo ""

LOOP_START=$SECONDS

for i in $(seq 1 "$MAX_ITERATIONS"); do
    LOGFILE="$LOGDIR/$(date +%Y%m%d-%H%M%S).log"

    # Count current passing/failing
    PASSING="$(count_passing_tests)"
    XFAIL="$(count_xfail_tests)"
    QUEUED="$(count_queue_files)"
    ANALYSIS_TASK=""
    if [ "$QUEUED" -gt 0 ]; then
        ANALYSIS_TASK="$(next_analysis_task)"
    fi

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

    if [ "$XFAIL" -eq 0 ] && [ "$QUEUED" -eq 0 ]; then
        event "all tests passing and analysis queue empty — nothing left to do"
        break
    fi

    # Preflight: verify claude works and check overage
    PREFLIGHT_OUT=$(timeout 30s claude --dangerously-skip-permissions -p "ok" --output-format stream-json --max-turns 1 2>&1)
    PREFLIGHT_EXIT=$?
    if [ $PREFLIGHT_EXIT -ne 0 ]; then
        if [ $PREFLIGHT_EXIT -eq 124 ]; then
            event "ABORT: claude preflight timed out (30s)"
        else
            event "ABORT: claude preflight failed (exit $PREFLIGHT_EXIT): $(echo "$PREFLIGHT_OUT" | tail -1)"
        fi
        break
    fi
    if echo "$PREFLIGHT_OUT" | grep -q '"isUsingOverage":true'; then
        event "OVERAGE: rate limit exhausted, stopping to avoid charges"
        break
    fi

    SESSION_START=$SECONDS
    PROMPT_MODEL=$(build_prompt "$ANALYSIS_TASK")
    PROMPT="${PROMPT_MODEL%|*}"
    MODEL="${PROMPT_MODEL##*|}"

    # Background spinner
    _spinner_iter=$i _spinner_max=$MAX_ITERATIONS _spinner_pass=$PASSING _spinner_xfail=$XFAIL _spinner_queue=$QUEUED _spinner_model=$MODEL
    (
        FRAMES=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')
        TICK=0
        while true; do
            F=${FRAMES[$(( TICK % ${#FRAMES[@]} ))]}
            DUR=$(format_duration "$TICK")
            printf "\r\033[K%s  iter %d/%d  ✓%d ✗%d q%d  %s  %s  %s" \
                "$F" "$_spinner_iter" "$_spinner_max" "$_spinner_pass" "$_spinner_xfail" "$_spinner_queue" \
                "$_spinner_model" "$DUR" "running…"
            sleep 1
            TICK=$((TICK + 1))
        done
    ) &
    SPINNER_PID=$!

    timeout --foreground --kill-after=30s "${TIMEOUT_MINS}m" claude \
        --dangerously-skip-permissions \
        --verbose \
        --model "$MODEL" \
        --max-turns "$MAX_TURNS" \
        -p "$PROMPT" \
        --output-format stream-json \
        2>&1 | tee "$LOGFILE" | jq -r --unbuffered '.message.content[]? | select(.type == "tool_use") | "  → " + .description // .name' 2>/dev/null
    EXIT_CODE=${PIPESTATUS[0]}

    # Kill spinner
    kill "$SPINNER_PID" 2>/dev/null
    wait "$SPINNER_PID" 2>/dev/null

    ELAPSED=$(( SECONDS - SESSION_START ))
    LINES=$(wc -l < "$LOGFILE" | tr -d ' ')

    if [ "$EXIT_CODE" -eq 124 ]; then
        event "TIMEOUT  iter $i  $(format_duration $ELAPSED)"
    elif [ "$ELAPSED" -le 5 ] && [ "$EXIT_CODE" -ne 0 ]; then
        event "ABORT: claude crashed immediately (exit $EXIT_CODE)"
        break
    else
        event "done  iter $i/$MAX_ITERATIONS  ✓$PASSING ✗$XFAIL q$QUEUED  $(format_duration $ELAPSED)  ${LINES}L"
    fi

    # Safety net: auto-stage test file changes that claude forgot
    git add tests/ tests.bats 2>/dev/null
    if ! git diff --cached --quiet 2>/dev/null; then
        git commit -q -m "Auto-commit: test output updates (session $i)"
    fi

    sleep 2
done

TOTAL=$(( SECONDS - LOOP_START ))
PASSING="$(count_passing_tests)"
XFAIL="$(count_xfail_tests)"
QUEUED="$(count_queue_files)"
event "done — $PASSING passing, $XFAIL xfail, $QUEUED queued (started at $BASELINE_PASS) — $(format_duration $TOTAL) total"
