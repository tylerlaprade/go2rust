#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: ./pressure_run.sh --min-env NAME --default-min-mb N --label LABEL --skip-env NAME -- command [args...]
       ./pressure_run.sh --min-mb N --label LABEL [--skip-env NAME] -- command [args...]

Run a command while monitoring available memory. If memory drops below the
configured floor, terminate the command and its child processes before the OS
gets deeper into swap pressure.

Options:
  --min-env NAME        Read the memory floor from environment variable NAME.
  --default-min-mb N    Default floor when --min-env is unset.
  --min-mb N            Literal memory floor in MiB.
  --label LABEL         Work label used in refusal output.
  --skip-env NAME       Environment variable that bypasses the monitor when true.
  --interval-seconds N  Seconds between memory checks (default: 2).
  -h, --help            Show this help.
EOF
}

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
min_env=""
default_min_mb=""
min_mb=""
min_desc=""
skip_env=""
label="work"
interval_seconds="${GO2RUST_PRESSURE_RUN_INTERVAL_SECONDS:-2}"
command_args=()

truthy_env() {
    local name="$1"
    [ -n "$name" ] || return 1
    local value="${!name:-}"
    case "$value" in
        1|true|TRUE|yes|YES)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --min-env)
            if [ "$#" -lt 2 ]; then
                echo "error: --min-env requires a value" >&2
                exit 2
            fi
            min_env="$2"
            shift 2
            ;;
        --default-min-mb)
            if [ "$#" -lt 2 ]; then
                echo "error: --default-min-mb requires a value" >&2
                exit 2
            fi
            default_min_mb="$2"
            shift 2
            ;;
        --min-mb)
            if [ "$#" -lt 2 ]; then
                echo "error: --min-mb requires a value" >&2
                exit 2
            fi
            min_mb="$2"
            min_desc="minimum"
            shift 2
            ;;
        --label)
            if [ "$#" -lt 2 ]; then
                echo "error: --label requires a value" >&2
                exit 2
            fi
            label="$2"
            shift 2
            ;;
        --skip-env)
            if [ "$#" -lt 2 ]; then
                echo "error: --skip-env requires a value" >&2
                exit 2
            fi
            skip_env="$2"
            shift 2
            ;;
        --interval-seconds)
            if [ "$#" -lt 2 ]; then
                echo "error: --interval-seconds requires a value" >&2
                exit 2
            fi
            interval_seconds="$2"
            shift 2
            ;;
        --)
            shift
            command_args=("$@")
            break
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown argument: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [ "${#command_args[@]}" -eq 0 ]; then
    echo "error: missing command after --" >&2
    usage >&2
    exit 2
fi

if truthy_env "$skip_env"; then
    exec "${command_args[@]}"
fi

if [ -n "$min_env" ]; then
    min_mb="${!min_env:-$default_min_mb}"
    min_desc="$min_env"
fi

case "$min_mb" in
    ''|*[!0-9]*)
        if [ -n "$min_env" ]; then
            echo "error: $min_env must be a non-negative integer" >&2
        else
            echo "error: --min-mb must be a non-negative integer" >&2
        fi
        exit 2
        ;;
esac

case "$interval_seconds" in
    ''|*[!0-9]*)
        echo "error: --interval-seconds must be a positive integer" >&2
        exit 2
        ;;
esac
if [ "$interval_seconds" -eq 0 ]; then
    echo "error: --interval-seconds must be a positive integer" >&2
    exit 2
fi

[ "$min_mb" -eq 0 ] && exec "${command_args[@]}"

min_bytes=$((min_mb * 1024 * 1024))
pressure_status_file=$(mktemp "${TMPDIR:-/tmp}/go2rust-pressure-run.XXXXXX")

cleanup_status_file() {
    [ -n "${pressure_status_file:-}" ] && rm -f "$pressure_status_file"
}
trap cleanup_status_file EXIT

terminate_tree() {
    local pid="$1"
    local child
    while IFS= read -r child; do
        [ -n "$child" ] || continue
        terminate_tree "$child"
    done < <(ps -o pid= -P "$pid" 2>/dev/null || true)
    kill -TERM "$pid" 2>/dev/null || true
}

kill_tree() {
    local pid="$1"
    local child
    while IFS= read -r child; do
        [ -n "$child" ] || continue
        kill_tree "$child"
    done < <(ps -o pid= -P "$pid" 2>/dev/null || true)
    kill -KILL "$pid" 2>/dev/null || true
}

available_memory_bytes() {
    "$repo_root/pressure_guard.sh" --available-bytes 2>/dev/null || true
}

child_pid=""
monitor_pid=""

cleanup_child() {
    [ -n "$monitor_pid" ] && kill "$monitor_pid" 2>/dev/null || true
    [ -n "$child_pid" ] || return
    kill -0 "$child_pid" 2>/dev/null || return
    terminate_tree "$child_pid"
}

trap 'cleanup_child; exit 130' INT
trap 'cleanup_child; exit 143' TERM

monitor_pressure() {
    while kill -0 "$child_pid" 2>/dev/null; do
        available_bytes=$(available_memory_bytes)
        case "$available_bytes" in
            ''|*[!0-9]*)
                ;;
            *)
                if [ "$available_bytes" -lt "$min_bytes" ]; then
                    available_mb=$((available_bytes / 1024 / 1024))
                    echo "Error: available memory is ${available_mb} MiB, below ${min_desc}=${min_mb} MiB." >&2
                    echo "Terminating $label to prevent deeper memory pressure." >&2
                    echo "Run ./cleanup.sh --pressure --quick to inspect current pressure." >&2
                    echo pressure > "$pressure_status_file"
                    terminate_tree "$child_pid"
                    sleep 2
                    if kill -0 "$child_pid" 2>/dev/null; then
                        kill_tree "$child_pid"
                    fi
                    return
                fi
                ;;
        esac
        sleep "$interval_seconds"
    done
}

"${command_args[@]}" &
child_pid=$!
monitor_pressure &
monitor_pid=$!

set +e
wait "$child_pid"
status=$?
set -e

kill "$monitor_pid" 2>/dev/null || true
wait "$monitor_pid" 2>/dev/null || true

if [ "$(cat "$pressure_status_file" 2>/dev/null || true)" = "pressure" ]; then
    exit 137
fi
exit "$status"
