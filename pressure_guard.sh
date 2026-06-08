#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: ./pressure_guard.sh [--available-bytes]
       ./pressure_guard.sh --min-env NAME --default-min-mb N --label LABEL --skip-env NAME [--hint TEXT]
       ./pressure_guard.sh --min-mb N --label LABEL [--skip-env NAME] [--hint TEXT]

Refuse to start memory-sensitive work when currently available memory is below
the requested floor. If available memory cannot be detected, the guard exits 0
so platform differences do not block validation.

Options:
  --available-bytes     Print detected available memory in bytes and exit.
  --min-env NAME        Read the memory floor from environment variable NAME.
  --default-min-mb N    Default floor when --min-env is unset.
  --min-mb N            Literal memory floor in MiB.
  --label LABEL         Work label used in refusal output.
  --skip-env NAME       Environment variable that bypasses the guard when true.
  --hint TEXT           Final diagnostic hint on refusal.
  -h, --help            Show this help.
EOF
}

detect_available_memory_bytes() {
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
}

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

available_only=false
min_env=""
default_min_mb=""
min_mb=""
min_desc=""
skip_env=""
label="work"
hint="Run ./cleanup.sh --pressure --quick to inspect current pressure."

while [ "$#" -gt 0 ]; do
    case "$1" in
        --available-bytes)
            available_only=true
            shift
            ;;
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
        --hint)
            if [ "$#" -lt 2 ]; then
                echo "error: --hint requires a value" >&2
                exit 2
            fi
            hint="$2"
            shift 2
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

if [ "$available_only" = true ]; then
    detect_available_memory_bytes
    exit 0
fi

if truthy_env "$skip_env"; then
    exit 0
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

[ "$min_mb" -eq 0 ] && exit 0

available_bytes=$(detect_available_memory_bytes)
case "$available_bytes" in
    ''|*[!0-9]*)
        exit 0
        ;;
esac

min_bytes=$(( min_mb * 1024 * 1024 ))
if [ "$available_bytes" -lt "$min_bytes" ]; then
    available_mb=$(( available_bytes / 1024 / 1024 ))
    echo "Error: available memory is ${available_mb} MiB, below ${min_desc}=${min_mb} MiB." >&2
    echo "Refusing to start $label while the machine is under memory pressure." >&2
    echo "$hint" >&2
    exit 1
fi
