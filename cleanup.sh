#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: ./cleanup.sh [--dry-run] [--sizes] [--summary] [--pressure] [--show-active] [--age-minutes N] [--top-temp N] [--keep-repo-artifacts]

Remove stale go2rust temporary roots and ignored local build artifacts.
With no arguments, print pressure diagnostics and cleanup candidates without
removing anything.

Options:
  --dry-run             Print paths that would be removed.
  --sizes               Include each path's disk usage in cleanup output.
  --summary             Print matching paths, sizes, and the total reclaimable
                        space without removing anything.
  --pressure            Print disk/memory/process pressure plus cleanup
                        candidates.
                        Does not remove anything. Defaults to --age-minutes 0
                        unless --age-minutes is also passed.
  --show-active         With --summary or --dry-run, print active marked temp
                        roots that cleanup skips.
  --age-minutes N       Only remove temp paths older than N minutes (default: 60).
  --top-temp N          With --pressure, print the N largest top-level temp
                        paths from TMPDIR, /tmp, and /private/tmp (default: 8).
  --keep-repo-artifacts Keep ignored root build artifacts such as ./go2rust.
  -h, --help            Show this help.
EOF
}

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
dry_run=false
show_sizes=false
summary=false
pressure=false
show_active=false
age_minutes="${GO2RUST_CLEANUP_AGE_MINUTES:-60}"
age_minutes_explicit=false
remove_repo_artifacts=true
candidate_count=0
total_kib=0
active_count=0
active_kib=0
invoked_without_args=false
top_temp_count="${GO2RUST_CLEANUP_TOP_TEMP_COUNT:-8}"

if [ "$#" -eq 0 ]; then
    invoked_without_args=true
fi

while [ "$#" -gt 0 ]; do
    case "$1" in
        --dry-run)
            dry_run=true
            shift
            ;;
        --sizes)
            show_sizes=true
            shift
            ;;
        --summary)
            summary=true
            dry_run=true
            show_sizes=true
            shift
            ;;
        --pressure)
            pressure=true
            summary=true
            dry_run=true
            show_sizes=true
            show_active=true
            shift
            ;;
        --show-active)
            show_active=true
            show_sizes=true
            shift
            ;;
        --age-minutes)
            if [ "$#" -lt 2 ]; then
                echo "error: --age-minutes requires a value" >&2
                exit 2
            fi
            age_minutes="$2"
            age_minutes_explicit=true
            shift 2
            ;;
        --top-temp)
            if [ "$#" -lt 2 ]; then
                echo "error: --top-temp requires a value" >&2
                exit 2
            fi
            top_temp_count="$2"
            shift 2
            ;;
        --keep-repo-artifacts)
            remove_repo_artifacts=false
            shift
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

case "$age_minutes" in
    ''|*[!0-9]*)
        echo "error: --age-minutes must be a non-negative integer" >&2
        exit 2
        ;;
esac

case "$top_temp_count" in
    ''|*[!0-9]*)
        echo "error: --top-temp must be a non-negative integer" >&2
        exit 2
        ;;
esac

if [ "$pressure" = true ] && [ "$age_minutes_explicit" = false ]; then
	age_minutes=0
fi

if [ "$invoked_without_args" = true ]; then
	pressure=true
	summary=true
	dry_run=true
	show_sizes=true
	show_active=true
	remove_repo_artifacts=false
	age_minutes=0
fi

path_size_kib() {
    du -sk "$1" 2>/dev/null | awk '{ print $1 }' || true
}

format_kib() {
    local kib="$1"
    awk -v kib="$kib" 'BEGIN {
        split("K M G T", units, " ")
        value = kib
        unit = 1
        while (value >= 1024 && unit < 4) {
            value = value / 1024
            unit++
        }
        if (unit == 1 || value >= 10) {
            printf "%.0f%s", value, units[unit]
        } else {
            printf "%.1f%s", value, units[unit]
        }
    }'
}

vm_stat_pages_for_label() {
    local snapshot="$1"
    local label="$2"
    printf '%s\n' "$snapshot" | awk -F: -v label="$label" '
        $1 == label {
            gsub(/[^0-9]/, "", $2)
            print $2
            exit
        }
    '
}

format_pages_kib() {
    local pages="$1"
    local page_size="$2"
    [ -n "$pages" ] || pages=0
    format_kib "$((pages * page_size / 1024))"
}

process_snapshot_from_aux() {
    local snapshot
    snapshot=$(ps auxww 2>/dev/null || true)
    [ -n "$snapshot" ] || return 0
    printf '%s\n' "$snapshot" | awk '
        NR == 1 {
            printf "%5s %5s %5s %5s %6s %s\n", "PID", "PPID", "%CPU", "%MEM", "RSS", "COMMAND"
            next
        }
        {
            command = ""
            for (i = 11; i <= NF; i++) {
                command = command " " $i
            }
            printf "%5s %5s %5s %5s %6s%s\n", $2, 0, $3, $4, $6, command
        }
    '
}

process_snapshot_by_cpu() {
    local snapshot
    snapshot=$(ps -axo pid,ppid,%cpu,%mem,rss,command -r 2>/dev/null || true)
    if [ -n "$snapshot" ]; then
        printf '%s\n' "$snapshot"
        return
    fi
    process_snapshot_from_aux || true
}

process_snapshot_by_memory() {
    local snapshot
    snapshot=$(ps -axo pid,ppid,%mem,%cpu,rss,command -m 2>/dev/null || true)
    if [ -n "$snapshot" ]; then
        printf '%s\n' "$snapshot"
        return
    fi
    process_snapshot_from_aux || true
}

print_size_row() {
    local label="$1"
    local path="$2"
    [ -e "$path" ] || return 0

    local size_kib
    size_kib=$(path_size_kib "$path")
    [ -n "$size_kib" ] || return 0

    printf "%8s  %s\n" "$(format_kib "$size_kib")" "$label"
}

print_disk_hotspots() {
    echo
    echo "Disk usage quick scan:"
    print_size_row "repo" "$repo_root"
    print_size_row "TMPDIR" "${TMPDIR:-/tmp}"
    print_size_row "/private/tmp" "/private/tmp"
    print_size_row "~/Library/Caches" "${HOME:-}/Library/Caches"
    print_size_row "~/Library/Developer" "${HOME:-}/Library/Developer"
    print_size_row "~/.cargo" "${HOME:-}/.cargo"
}

print_top_temp_paths() {
    [ "$top_temp_count" -gt 0 ] || return

    echo
    echo "Largest temp paths:"

    local rows
    rows=$(
        local seen_roots=""
        for root in "${tmp_roots[@]}"; do
            [ -n "$root" ] || continue
            case ":$seen_roots:" in
                *":$root:"*) continue ;;
            esac
            seen_roots="$seen_roots:$root"
            [ -d "$root" ] || continue

            while IFS= read -r path; do
                local size_kib
                size_kib=$(path_size_kib "$path")
                [ -n "$size_kib" ] || continue
                printf '%s\t%s\n' "$size_kib" "$path"
            done < <(find "$root" -mindepth 1 -maxdepth 1 \( -type d -o -type f \) -print 2>/dev/null)
        done | sort -nr | awk -v limit="$top_temp_count" 'NR <= limit'
    )

    if [ -z "$rows" ]; then
        echo "none found"
        return
    fi

    while IFS=$'\t' read -r size_kib path; do
        printf "%8s  %s\n" "$(format_kib "$size_kib")" "$path"
    done <<< "$rows"
}

remove_path() {
    local path="$1"
    local size=""
    if [ "$show_sizes" = true ]; then
        local size_kib
        size_kib=$(path_size_kib "$path")
        if [ -n "$size_kib" ]; then
            total_kib=$((total_kib + size_kib))
            size=" ($(format_kib "$size_kib"))"
        fi
    fi
    candidate_count=$((candidate_count + 1))
    if [ "$dry_run" = true ]; then
        echo "would remove: $path$size"
    else
        echo "removing: $path$size"
        rm -rf "$path"
    fi
}

pid_is_active() {
    local pid_file="$1"
    [ -f "$pid_file" ] || return 1

    local pid
    pid=$(cat "$pid_file" 2>/dev/null || true)
    [ -n "$pid" ] || return 1
    kill -0 "$pid" 2>/dev/null
}

pid_command() {
    local pid="$1"
    ps -p "$pid" -o command= 2>/dev/null || true
}

print_pressure_report() {
    echo "Cleanup script: $repo_root/cleanup.sh"
    echo
    echo "Filesystem:"
    df -h "$repo_root" /private/tmp "${TMPDIR:-/tmp}" 2>/dev/null | awk 'NR == 1 || !seen[$1, $9]++'

    local vm_stat_snapshot
    vm_stat_snapshot=$(vm_stat 2>/dev/null || true)

    echo
    echo "Memory:"
    if [ -n "$vm_stat_snapshot" ]; then
        local page_size
        page_size=$(printf '%s\n' "$vm_stat_snapshot" | awk -F'of | bytes' '/page size of/ { print $2; exit }')
        case "$page_size" in
            ''|*[!0-9]*) page_size=4096 ;;
        esac

        local free_pages speculative_pages compressed_pages compressor_pages pageouts swapouts
        free_pages=$(vm_stat_pages_for_label "$vm_stat_snapshot" "Pages free")
        speculative_pages=$(vm_stat_pages_for_label "$vm_stat_snapshot" "Pages speculative")
        compressed_pages=$(vm_stat_pages_for_label "$vm_stat_snapshot" "Pages stored in compressor")
        compressor_pages=$(vm_stat_pages_for_label "$vm_stat_snapshot" "Pages occupied by compressor")
        pageouts=$(vm_stat_pages_for_label "$vm_stat_snapshot" "Pageouts")
        swapouts=$(vm_stat_pages_for_label "$vm_stat_snapshot" "Swapouts")

        echo "Free: $(format_pages_kib "$free_pages" "$page_size")"
        echo "Speculative: $(format_pages_kib "$speculative_pages" "$page_size")"
        echo "Compressed: $(format_pages_kib "$compressed_pages" "$page_size") stored / $(format_pages_kib "$compressor_pages" "$page_size") occupied"
        echo "Pageouts: ${pageouts:-0} page(s)"
        echo "Swapouts: ${swapouts:-0} page(s)"
    else
        echo "unavailable: vm_stat was denied or returned no data"
    fi

    local process_cpu_snapshot
    process_cpu_snapshot=$(process_snapshot_by_cpu)

    local process_mem_snapshot
    process_mem_snapshot=$(process_snapshot_by_memory)

    echo
    echo "Process group summary:"
    if [ -n "$process_cpu_snapshot" ]; then
        printf '%s\n' "$process_cpu_snapshot" | awk '
            NR == 1 {
                next
            }
            {
                cpu = $3 + 0
                mem = $4 + 0
                rss = $5 + 0
                command = ""
                for (i = 6; i <= NF; i++) {
                    command = command " " $i
                }

                group = "Other"
                if (command ~ /(^|[[:space:]])(rustc|cargo)([[:space:]]|$)/ ||
                    command ~ /(^|[[:space:]])(\.\/)?test\.sh([[:space:]]|$)/ ||
                    command ~ /(^|[[:space:]])self_transpile_check\.sh([[:space:]]|$)/) {
                    group = "go2rust validation"
                } else if (command ~ /ANECompilerService/ ||
                           command ~ /siriinferenced/ ||
                           command ~ /SiriSuggestions/) {
                    group = "Apple ML/Siri services"
                } else if (command ~ /(^|[[:space:]\/])codex([[:space:]]|$)/ ||
                           command ~ /\/Codex\.app\//) {
                    group = "Codex"
                } else if (command ~ /(^|[[:space:]\/])claude([[:space:]]|$)/ ||
                           command ~ /\/Claude\.app\//) {
                    group = "Claude"
                } else if (command ~ /\/Ghostty\.app\//) {
                    group = "Ghostty"
                } else if (command ~ /\/Brave Browser\.app\//) {
                    group = "Brave"
                } else if (command ~ /\/Google Chrome\.app\//) {
                    group = "Chrome"
                }

                cpu_sum[group] += cpu
                mem_sum[group] += mem
                rss_sum[group] += rss
                count[group]++
            }
            function print_group(group, rss_mib) {
                if (!(group in count)) {
                    return
                }
                rss_mib = rss_sum[group] / 1024
                if (rss_mib >= 1024) {
                    rss = sprintf("%.1fG", rss_mib / 1024)
                } else {
                    rss = sprintf("%.0fM", rss_mib)
                }
                printf "%-30s %7.1f %7.1f %8s %6d\n",
                    group, cpu_sum[group], mem_sum[group], rss, count[group]
            }
            END {
                printf "%-30s %7s %7s %8s %6s\n", "Group", "%CPU", "%MEM", "RSS", "Count"
                print_group("Apple ML/Siri services")
                print_group("go2rust validation")
                print_group("Codex")
                print_group("Claude")
                print_group("Ghostty")
                print_group("Brave")
                print_group("Chrome")
                print_group("Other")
            }
        '
    else
        echo "unavailable: process listing was denied or returned no data"
    fi

    echo
    echo "Top CPU processes:"
    if [ -n "$process_cpu_snapshot" ]; then
        printf '%s\n' "$process_cpu_snapshot" | awk 'NR <= 15'
    else
        echo "unavailable: process listing was denied or returned no data"
    fi

    echo
    echo "Top memory processes:"
    if [ -n "$process_mem_snapshot" ]; then
        printf '%s\n' "$process_mem_snapshot" | awk 'NR <= 15'
    else
        echo "unavailable: process listing was denied or returned no data"
    fi

    echo
    echo "Active go2rust validation processes:"
    local validation_processes
    validation_processes=$(printf '%s\n' "$process_cpu_snapshot" |
        awk 'NR > 1 && /(^|[[:space:]])(go2rust|cargo|rustc|test\.sh|self_transpile_check\.sh)([[:space:]]|$)/' |
        awk 'NR <= 25')
    if [ -n "$validation_processes" ]; then
        printf '%s\n' "$process_cpu_snapshot" | awk 'NR == 1'
        printf '%s\n' "$validation_processes"
    else
        echo "none found, or process listing was denied"
    fi

    print_disk_hotspots
    print_top_temp_paths

    echo
}

active_pid_from_file() {
    local pid_file="$1"
    [ -f "$pid_file" ] || return 1

    local pid
    pid=$(cat "$pid_file" 2>/dev/null || true)
    [ -n "$pid" ] || return 1
    kill -0 "$pid" 2>/dev/null || return 1
    printf '%s\n' "$pid"
}

report_active_path() {
    local path="$1"
    local pid_name="$2"
    local pid="$3"
    local size=""
    if [ "$show_sizes" = true ]; then
        local size_kib
        size_kib=$(path_size_kib "$path")
        if [ -n "$size_kib" ]; then
            active_kib=$((active_kib + size_kib))
            size=" ($(format_kib "$size_kib"))"
        fi
    fi
    active_count=$((active_count + 1))

    local command
    command=$(pid_command "$pid")
    if [ -n "$command" ]; then
        echo "active: $path$size (pid $pid via $pid_name: $command)"
    else
        echo "active: $path$size (pid $pid via $pid_name)"
    fi
}

maybe_remove_temp_dir() {
    local dir="$1"

    for pid_name in self_transpile_check.pid go2rust-test.pid pid; do
        local active_pid
        active_pid=$(active_pid_from_file "$dir/$pid_name" || true)
        if [ -n "$active_pid" ]; then
            if [ "$show_active" = true ]; then
                report_active_path "$dir" "$pid_name" "$active_pid"
            fi
            return
        fi
    done

    remove_path "$dir"
}

cleanup_temp_root() {
    local root="$1"
    [ -d "$root" ] || return 0

    local -a age_args=()
    if [ "$age_minutes" -gt 0 ]; then
        age_args=(-mmin +"$age_minutes")
    fi

    while IFS= read -r dir; do
        maybe_remove_temp_dir "$dir"
    done < <(find "$root" -maxdepth 1 "${age_args[@]}" -type d \( \
        -name 'go2rust-self.*' -o \
        -name 'go2rust-test.*' -o \
        -name 'go2rust-bats-shards.*' -o \
        -name 'go2rust-cargo-home' -o \
        -name 'go2rust-cargo-home.*' -o \
        -name 'go2rust-cargo-source-*' -o \
        -name 'go2rust-cargo-target.*' -o \
        -name 'go2rust-cargo-current' -o \
        -name 'go2rust-shared-cargo-target' -o \
        -name 'go2rust-source-stdlib-*-target' -o \
        -name 'go2rust-heap-target' -o \
        -name 'go2rust-self-cargo-home' -o \
        -name 'go2rust-self-cargo-home.*' -o \
        -name 'go2rust-test-binary.*' -o \
        -name 'go2rust-bats-gocache' -o \
        -name 'go2rust-go-cache' -o \
        -name 'go2rust-go-cache.*' -o \
        -name 'go2rust-go-cache-current' -o \
        -name 'go2rust-gen.*' -o \
        -name 'go2rust-token-probe.*' -o \
        -name 'go2rust-typeid.*' -o \
        -name 'go2rust-anyptr.*' -o \
        -name 'go2rust-debug-*' -o \
        -name 'go2rust-*-debug' -o \
        -name 'go2rust-rust-work.*' -o \
        -name 'go2rust-*' \
    \) -print 2>/dev/null)

    while IFS= read -r file; do
        remove_path "$file"
    done < <(find "$root" -maxdepth 1 "${age_args[@]}" -type f \( \
        -name 'go2rust-tests-list.*' -o \
        -name 'go2rust-current' -o \
        -name 'go2rust-probe-bin' -o \
        -name 'go2rust-debug-*.log' -o \
        -name 'go2rust-sample-*' -o \
        -name 'go2rust-rust-diff.*' -o \
        -name 'go2rust-stdout.*' -o \
        -name 'go2rust-stderr.*' \
    \) -print 2>/dev/null)
}

if [ "$remove_repo_artifacts" = true ]; then
    for path in "$repo_root/go2rust" "$repo_root/transpiler" "$repo_root/test" "$repo_root/go/go" "$repo_root/target"; do
        if [ -e "$path" ] && ! git -C "$repo_root" ls-files --error-unmatch "${path#$repo_root/}" >/dev/null 2>&1; then
            remove_path "$path"
        fi
    done
fi

tmp_roots=()
add_tmp_root() {
    local root="$1"
    [ -n "$root" ] || return
    case "$root" in
        */) root="${root%/}" ;;
    esac
    [ -n "$root" ] || return
    tmp_roots+=("$root")
}

add_tmp_root "${TMPDIR:-}"
add_tmp_root "/tmp"
add_tmp_root "/private/tmp"

if [ "$pressure" = true ]; then
    print_pressure_report
    echo "Cleanup candidates:"
fi

seen_roots=""
for root in "${tmp_roots[@]}"; do
    [ -n "$root" ] || continue
    case ":$seen_roots:" in
        *":$root:"*) continue ;;
    esac
    seen_roots="$seen_roots:$root"
    cleanup_temp_root "$root"
done

if [ "$summary" = true ]; then
    echo "Total reclaimable: $(format_kib "$total_kib") across $candidate_count path(s)"
    if [ "$show_active" = true ]; then
        echo "Active skipped: $(format_kib "$active_kib") across $active_count path(s)"
    fi
elif [ "$candidate_count" -eq 0 ]; then
    echo "No cleanup candidates found."
    if [ "$invoked_without_args" = true ]; then
        echo "For disk/memory/process diagnostics, run: ./cleanup.sh --pressure --keep-repo-artifacts"
    fi
fi
