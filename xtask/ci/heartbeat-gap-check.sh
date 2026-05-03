#!/usr/bin/env bash
# xtask/ci/heartbeat-gap-check.sh — obs-plan §10 SLO gate (heartbeat stall detection)
#
# Parses {ingest|buffer|viz|plugins}.tick records from the JSON log, computes
# consecutive timestamp deltas, and asserts max ≤45000ms.
#
# INACTIVE state (chunk #5/#6): when no tick records are present (Foundation
# epoch — no subsystems exist yet to emit ticks), exits 0. Activates organically
# when first subsystem ships in Epoch 2 (route#16+).
#
# Usage: heartbeat-gap-check.sh [<log-path>]
#   Default log-path: $ANDROMEDA_PULSE_DATA_DIR/logs/agent-latest.jsonl* glob
#                     (tracing-appender daily-rolling appends YYYY-MM-DD suffix)

set -euo pipefail

LOG_DIR="${ANDROMEDA_PULSE_DATA_DIR:-}/logs"
LOG_ARG="${1:-}"
THRESHOLD_MS="${HEARTBEAT_GAP_THRESHOLD_MS:-45000}"

if [ -n "$LOG_ARG" ] && [ -f "$LOG_ARG" ]; then
  LOG_FILES=("$LOG_ARG")
elif [ -d "$LOG_DIR" ]; then
  mapfile -t LOG_FILES < <(find "$LOG_DIR" -maxdepth 1 -type f -name 'agent-latest.jsonl*' 2>/dev/null | sort)
else
  LOG_FILES=()
fi

if [ "${#LOG_FILES[@]}" -eq 0 ]; then
  echo "heartbeat-gap-check: no log file present (INACTIVE state — pre-subsystem chunk; gate trivially passes)"
  exit 0
fi

# Extract timestamps for {module}.tick targets, grouped by target.
# Format: target<TAB>epoch_ms (one line per matching record)
ticks_per_target() {
  for f in "${LOG_FILES[@]}"; do
    while IFS= read -r line; do
      [ -z "$line" ] && continue
      target=$(printf '%s' "$line" | sed -nE 's/.*"target":"([^"]+)".*/\1/p')
      case "$target" in
        ingest.tick|buffer.tick|viz.tick|plugins.tick)
          ts=$(printf '%s' "$line" | sed -nE 's/.*"timestamp":"([^"]+)".*/\1/p')
          if [ -n "$ts" ]; then
            ms=$(date -d "$ts" +%s%3N 2>/dev/null || python3 -c "import datetime,sys; print(int(datetime.datetime.fromisoformat(sys.argv[1].replace('Z','+00:00')).timestamp()*1000))" "$ts" 2>/dev/null || echo '')
            if [ -n "$ms" ]; then
              printf '%s\t%s\n' "$target" "$ms"
            fi
          fi
          ;;
      esac
    done < "$f"
  done
}

mapfile -t TICK_LINES < <(ticks_per_target)

if [ "${#TICK_LINES[@]}" -eq 0 ]; then
  echo "heartbeat-gap-check: no {ingest|buffer|viz|plugins}.tick records found (INACTIVE state — pre-subsystem chunk; gate trivially passes)"
  exit 0
fi

# Compute max gap per target via awk
max_gap=$(printf '%s\n' "${TICK_LINES[@]}" | awk -v threshold="$THRESHOLD_MS" '
  {
    target = $1
    ts = $2 + 0
    if (target in last_ts) {
      gap = ts - last_ts[target]
      if (gap > max_gap[target]) max_gap[target] = gap
    } else {
      max_gap[target] = 0
    }
    last_ts[target] = ts
  }
  END {
    overall_max = 0
    overall_target = ""
    for (t in max_gap) {
      if (max_gap[t] > overall_max) {
        overall_max = max_gap[t]
        overall_target = t
      }
    }
    printf "%d\t%s\n", overall_max, overall_target
  }
')

gap_ms=$(printf '%s' "$max_gap" | cut -f1)
gap_target=$(printf '%s' "$max_gap" | cut -f2)

if [ "$gap_ms" -gt "$THRESHOLD_MS" ]; then
  echo "::error::heartbeat-gap-check: max gap ${gap_ms}ms in ${gap_target} exceeds ${THRESHOLD_MS}ms threshold" >&2
  exit 1
fi

echo "heartbeat-gap-check: max gap ${gap_ms}ms in ${gap_target} (threshold ${THRESHOLD_MS}ms) PASS"
exit 0
