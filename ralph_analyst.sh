#!/usr/bin/env bash
# Ralph analyst — Codex-powered strategic analysis for go2rust
# Produces actionable queue items in .analysis/ for ralph_loop.sh to consume.
#
# Usage: ./ralph_analyst.sh [max_iterations] [pause_mins]
#
# Examples:
#   ./ralph_analyst.sh           # 20 iterations, no fixed pause
#   ./ralph_analyst.sh 10 10     # 10 iterations, 10m pause between runs

set -euo pipefail

MAX_ITERATIONS="${1:-20}"
PAUSE_MINS="${2:-0}"
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ANALYSIS_DIR="$ROOT_DIR/.analysis"
LOGDIR="${RALPH_ANALYST_LOGDIR:-$ROOT_DIR/.codex-loop-logs}"
QUEUE_LIMIT="${RALPH_ANALYST_QUEUE_LIMIT:-5}"
RUN_TIMEOUT_MINS="${RALPH_ANALYST_TIMEOUT_MINS:-10}"
POLL_SECS="${RALPH_ANALYST_POLL_SECS:-30}"
KEEP_AWAKE_PID=""

fail() {
    echo "ralph analyst: $*" >&2
    exit 1
}

require_command() {
    command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

validate_non_negative_integer() {
    local value="$1"
    local name="$2"
    [[ "$value" =~ ^[0-9]+$ ]] || fail "$name must be a non-negative integer (got: $value)"
}

validate_positive_integer() {
    local value="$1"
    local name="$2"
    [[ "$value" =~ ^[1-9][0-9]*$ ]] || fail "$name must be a positive integer (got: $value)"
}

resolve_timeout_bin() {
    if command -v timeout >/dev/null 2>&1; then
        printf '%s\n' "timeout"
    elif command -v gtimeout >/dev/null 2>&1; then
        printf '%s\n' "gtimeout"
    else
        fail "missing required command: timeout (or gtimeout)"
    fi
}

start_keep_awake() {
    if command -v caffeinate >/dev/null 2>&1; then
        caffeinate -dims -w $$ >/dev/null 2>&1 &
        KEEP_AWAKE_PID=$!
    fi
}

cleanup() {
    local exit_code=$?
    if [ -n "$KEEP_AWAKE_PID" ]; then
        kill "$KEEP_AWAKE_PID" 2>/dev/null || true
        wait "$KEEP_AWAKE_PID" 2>/dev/null || true
    fi
    if [ "$exit_code" -eq 130 ]; then
        printf "\r\033[K"
        echo "Ralph analyst interrupted."
    fi
}

trap cleanup EXIT
trap 'exit 130' INT TERM

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

snapshot_queue() {
    local outfile="$1"
    queue_files | LC_ALL=C sort >"$outfile"
}

queue_summary() {
    local path
    local title
    local found=0

    while IFS= read -r path; do
        found=1
        title="$(sed -n 's/^# //p;q' "$path")"
        if [ -n "$title" ]; then
            printf -- '- %s :: %s\n' "$(basename "$path")" "$title"
        else
            printf -- '- %s\n' "$(basename "$path")"
        fi
    done < <(queue_files)

    if [ "$found" -eq 0 ]; then
        printf -- '- (none)\n'
    fi
}

build_prompt() {
    local queued_items="$1"
    cat <<EOF
You are generating actionable analysis queue items for the go2rust project.

Read these files first:
- AGENTS.md
- .analysis/README.md
- LOOP_PROTOCOL.md

Your task:
- Find the highest-leverage next tasks from the repo's current state.
- Create zero or more new markdown files in .analysis/ as warranted by what you find.
- If the queue already contains the relevant work, or you cannot find a high-value scoped task, create no file and exit cleanly.

Do not rotate through canned categories or force variety for its own sake.
Pick what matters most right now.

Possible lenses to consider if they help:
- architecture bottlenecks that block multiple future tests
- missing regression tests for already-supported Go patterns
- XFAIL cases with the best effort-to-impact ratio
- concrete translation bugs or brittle code paths in go/*.go
- missing stdlib mappings that unblock real tests
- shared prerequisites that unlock clusters of XFAILs

Current queued items:
$queued_items

Queue item requirements:
- One task per file, scoped so ralph_loop.sh can plausibly address it in one session.
- Use a descriptive kebab-case filename like .analysis/fix-map-zero-value-lookup.md.
- Do not use timestamps, counters, generic names, or the angle name as the filename.
- Do not edit or delete existing queue items.
- Keep the file concise and actionable.
- Include these sections exactly:
  - # <short title>
  - ## Why Now
  - ## Evidence
  - ## Suggested Change
  - ## Acceptance

Quality bar:
- Reference concrete files, functions, tests, and missing behaviors.
- Be explicitly aware of the queued items listed above and avoid restating them with different wording.
- Read any queued file that looks adjacent before proposing overlapping work.
- Check recent git history and avoid recently-completed work.
- Prefer a small set of high-leverage tasks, not a broad report or a flood of low-value items.
- Prefer prerequisites or complementary follow-up tasks when they clearly strengthen the existing queue.
- If the current queue already captures the best next work, create no file.
- If the task is to add a test, include a proposed main.go snippet in ## Suggested Change.

Constraints:
- Write only inside .analysis/.
- Output a short status line to stdout describing what you created or why you skipped.
- Do not make code changes outside .analysis/.
EOF
}

validate_non_negative_integer "$MAX_ITERATIONS" "max_iterations"
validate_non_negative_integer "$PAUSE_MINS" "pause_mins"
validate_positive_integer "$QUEUE_LIMIT" "RALPH_ANALYST_QUEUE_LIMIT"
validate_positive_integer "$RUN_TIMEOUT_MINS" "RALPH_ANALYST_TIMEOUT_MINS"
validate_positive_integer "$POLL_SECS" "RALPH_ANALYST_POLL_SECS"

mkdir -p "$ANALYSIS_DIR" "$LOGDIR"
cd "$ROOT_DIR"

require_command codex
TIMEOUT_BIN="$(resolve_timeout_bin)"
start_keep_awake

echo "ralph analyst — adaptive task selection, ${PAUSE_MINS}m fixed pause"
echo "output: $ANALYSIS_DIR/"
echo "logs: $LOGDIR/"
echo ""

for ((i = 1; i <= MAX_ITERATIONS; i++)); do
    # Skip if there are already too many unprocessed files.
    QUEUED="$(count_queue_files)"
    while [ "$QUEUED" -ge "$QUEUE_LIMIT" ]; do
        echo "[$i/$MAX_ITERATIONS] queue full ($QUEUED files) — polling every ${POLL_SECS}s"
        sleep "$POLL_SECS"
        QUEUED="$(count_queue_files)"
    done

    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    LOGFILE="$LOGDIR/analysis-${TIMESTAMP}.log"
    BEFORE_SNAPSHOT="$(mktemp)"
    AFTER_SNAPSHOT="$(mktemp)"
    NEW_SNAPSHOT="$(mktemp)"
    snapshot_queue "$BEFORE_SNAPSHOT"

    FULL_PROMPT="$(build_prompt "$(queue_summary)")"

    echo -n "[$i/$MAX_ITERATIONS] analyzing... "

    set +e
    "$TIMEOUT_BIN" "${RUN_TIMEOUT_MINS}m" codex exec \
        --full-auto \
        "$FULL_PROMPT" \
        >"$LOGFILE" 2>&1
    EXIT_CODE=$?
    set -e

    snapshot_queue "$AFTER_SNAPSHOT"
    comm -13 "$BEFORE_SNAPSHOT" "$AFTER_SNAPSHOT" >"$NEW_SNAPSHOT"

    if [ "$EXIT_CODE" -eq 124 ]; then
        echo "timeout"
    elif [ "$EXIT_CODE" -ne 0 ]; then
        echo "failed (exit $EXIT_CODE)"
    else
        NEW_COUNT="$(wc -l < "$NEW_SNAPSHOT" | tr -d ' ')"
        if [ "$NEW_COUNT" -eq 0 ]; then
            echo "no new task"
        else
            SUMMARY=""
            SHOWN=0
            while IFS= read -r NEW_FILE; do
                [ -n "$NEW_FILE" ] || continue
                if [ "$SHOWN" -lt 3 ]; then
                    if [ -n "$SUMMARY" ]; then
                        SUMMARY="$SUMMARY, "
                    fi
                    SUMMARY="$SUMMARY$(basename "$NEW_FILE")"
                fi
                SHOWN=$((SHOWN + 1))
            done < "$NEW_SNAPSHOT"
            if [ "$NEW_COUNT" -gt 3 ]; then
                SUMMARY="$SUMMARY, ..."
            fi
            if [ "$NEW_COUNT" -eq 1 ]; then
                echo "queued 1 task: $SUMMARY"
            else
                echo "queued $NEW_COUNT tasks: $SUMMARY"
            fi
        fi
    fi

    rm -f "$BEFORE_SNAPSHOT" "$AFTER_SNAPSHOT" "$NEW_SNAPSHOT"

    if [ "$EXIT_CODE" -ne 0 ]; then
        echo "  log: $LOGFILE"
    fi

    # Optional pause between iterations.
    if [ "$i" -lt "$MAX_ITERATIONS" ]; then
        sleep "${PAUSE_MINS}m"
    fi
done

echo ""
echo "ralph analyst done — $(count_queue_files) files queued"
