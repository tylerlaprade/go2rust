#!/usr/bin/env bash
# Ralph analyst — Codex-powered strategic analysis for go2rust
# Produces analysis documents in .analysis/ for ralph_loop.sh to consume.
#
# Usage: ./ralph_analyst.sh [max_iterations] [cooldown_mins]
#
# Examples:
#   ./ralph_analyst.sh           # 20 iterations, 5m cooldown
#   ./ralph_analyst.sh 10 10     # 10 iterations, 10m cooldown

# Prevent sleep while looping (background, tied to our PID)
caffeinate -dims -w $$ &

MAX_ITERATIONS="${1:-20}"
COOLDOWN_MINS="${2:-5}"
ANALYSIS_DIR="$(pwd)/.analysis"
LOGDIR="$(pwd)/.codex-loop-logs"
mkdir -p "$ANALYSIS_DIR" "$LOGDIR"

trap 'printf "\r\033[K"; echo "Ralph analyst interrupted."; exit 130' INT

# Analysis angles — each iteration picks the next one in rotation
ANGLES=(
    "architecture|Architecture review: Read the transpiler source in go/*.go (especially expr.go, stmt.go, types.go, decl.go). Identify the top 3 patterns or approaches that will hit a wall as coverage increases. For each, explain the problem, which XFAIL tests it blocks, and what the fix looks like. Be specific — name functions, line ranges, and concrete Go patterns that break."

    "test-gaps|Test gap analysis: Read all XFAIL test main.go files AND all passing test main.go files. Identify Go language patterns that aren't covered by ANY test (passing or XFAIL). For each gap, write a short main.go that exercises the pattern. Focus on patterns the transpiler likely already handles but nobody tested, or patterns that are common in real Go code."

    "xfail-triage|XFAIL triage: For each test in tests/XFAIL/, run a quick analysis: read main.go, identify which Go features it uses, estimate how many transpiler changes are needed to fix it, and whether fixing it would also fix other tests. Produce a ranked list from easiest to hardest, with specific notes on what each one needs."

    "code-quality|Code quality review: Read go/*.go source files. Find bugs, incorrect translations, or fragile patterns that will cause regressions. Focus on: hardcoded string matching that should use TypeInfo, missing cases in switch statements, assumptions about wrapping that don't hold for all types. For each issue, name the file, function, and what's wrong."

    "stdlib-mapping|Stdlib mapping analysis: Read the XFAIL tests that need stdlib support (file_io, file_operations, os_args, random_numbers, time_operations, json_marshal, regex_basic, etc). For each, identify exactly which Go stdlib functions are called and what the Rust equivalent would be. Produce a concrete mapping table with the Go call, the Rust replacement, and whether it needs an external crate."

    "dependency-graph|Dependency analysis: Look at the remaining XFAIL tests and identify clusters — groups of tests that share a common missing feature. Which single transpiler change would unlock the most tests? Produce a dependency graph: feature X blocks tests [A, B, C], feature Y blocks [D, E], feature X+Y together block [F]. Prioritize by impact."
)

echo "ralph analyst — ${#ANGLES[@]} analysis angles, ${COOLDOWN_MINS}m cooldown between iterations"
echo "output: $ANALYSIS_DIR/"
echo "logs: $LOGDIR/"
echo ""

for i in $(seq 1 "$MAX_ITERATIONS"); do
    # Rotate through angles
    IDX=$(( (i - 1) % ${#ANGLES[@]} ))
    ANGLE_ENTRY="${ANGLES[$IDX]}"
    ANGLE_NAME="${ANGLE_ENTRY%%|*}"
    ANGLE_PROMPT="${ANGLE_ENTRY#*|}"

    # Skip if there are already 5+ unprocessed files (don't flood the queue)
    QUEUED=$(ls "$ANALYSIS_DIR"/*.md 2>/dev/null | grep -v README.md | wc -l | tr -d ' ')
    if [ "$QUEUED" -ge 5 ]; then
        echo "[$i/$MAX_ITERATIONS] queue full ($QUEUED files) — waiting ${COOLDOWN_MINS}m"
        sleep "${COOLDOWN_MINS}m"
        continue
    fi

    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    OUTFILE="$ANALYSIS_DIR/${ANGLE_NAME}-${TIMESTAMP}.md"
    LOGFILE="$LOGDIR/${ANGLE_NAME}-${TIMESTAMP}.log"

    # Build the full prompt
    FULL_PROMPT="You are analyzing the go2rust transpiler codebase. Your job is to produce a single, actionable analysis document. Do NOT make code changes — only produce analysis.

Read AGENTS.md first for project context.

$ANGLE_PROMPT

IMPORTANT:
- Check .analysis/ for existing files and don't duplicate work that's already queued.
- Check recent git log to understand what's been fixed recently.
- Be specific and actionable. Every recommendation should name files, functions, and concrete changes.
- Write your analysis to: $OUTFILE
- Keep it under 200 lines. One focused document, not a brain dump."

    echo -n "[$i/$MAX_ITERATIONS] $ANGLE_NAME... "

    timeout 10m codex exec \
        --full-auto \
        "$FULL_PROMPT" \
        >"$LOGFILE" 2>&1
    EXIT_CODE=$?

    if [ "$EXIT_CODE" -eq 124 ]; then
        echo "timeout"
    elif [ -f "$OUTFILE" ]; then
        LINES=$(wc -l < "$OUTFILE" | tr -d ' ')
        echo "done (${LINES}L)"
    else
        echo "no output (exit $EXIT_CODE)"
    fi

    # Cooldown between iterations
    if [ "$i" -lt "$MAX_ITERATIONS" ]; then
        sleep "${COOLDOWN_MINS}m"
    fi
done

echo ""
echo "ralph analyst done — $(ls "$ANALYSIS_DIR"/*.md 2>/dev/null | grep -v README.md | wc -l | tr -d ' ') files queued"
