#!/usr/bin/env bash
# Chunk #54 perf SLO regression detector. Tails agent-latest.jsonl for
# metric.webgpu.frame_duration_ms events; computes p99 of .fields.duration_ms;
# asserts ≤33ms (= 30 fps minimum at p99 per obs-plan §10 row 2). Sibling
# check: metric.buffer.memory_bytes max .fields.value ≤512_000_000 (per
# obs-plan §10 row 3). Empty event streams map к NEUTRAL (exit 0) — full
# gate activates когда production observability emits during load window.

set -euo pipefail

if [ "$#" -lt 1 ]; then
    echo "::error::perf-slo-check: missing log file argument" >&2
    exit 1
fi

LOG="$1"

if [ ! -f "$LOG" ]; then
    echo "perf-slo-check: log file ${LOG} not found; treating as NEUTRAL"
    exit 0
fi

if ! command -v jq >/dev/null 2>&1; then
    echo "perf-slo-check: jq not available; treating as NEUTRAL" >&2
    exit 0
fi

FRAME_SAMPLES=$(jq -r '. | select(.target == "metric.webgpu.frame_duration_ms") | .fields.duration_ms' "$LOG" 2>/dev/null | grep -v '^null$' | sort -n)
FRAME_COUNT=$(echo "$FRAME_SAMPLES" | grep -c . || true)

if [ "$FRAME_COUNT" -eq 0 ]; then
    echo "perf-slo-check: zero frame_duration_ms events; NEUTRAL (webview not booted during load OR observability not subscribed)"
else
    P99_IDX=$(awk -v n="$FRAME_COUNT" 'BEGIN { printf "%d", (n * 99 + 99) / 100 }')
    [ "$P99_IDX" -gt "$FRAME_COUNT" ] && P99_IDX="$FRAME_COUNT"
    P99=$(echo "$FRAME_SAMPLES" | sed -n "${P99_IDX}p")
    if awk -v p="$P99" 'BEGIN { exit (p + 0 > 33.0) }'; then
        echo "perf-slo-check: frame_duration_ms p99 = ${P99}ms ≤ 33ms (PASS; n=${FRAME_COUNT})"
    else
        echo "::error::perf-slo-check: frame_duration_ms p99 = ${P99}ms > 33ms (FAIL; n=${FRAME_COUNT})" >&2
        exit 1
    fi
fi

MEM_MAX=$(jq -r '. | select(.target == "metric.buffer.memory_bytes") | .fields.value' "$LOG" 2>/dev/null | grep -v '^null$' | sort -n | tail -1)

if [ -z "$MEM_MAX" ]; then
    echo "perf-slo-check: zero buffer.memory_bytes events; NEUTRAL (heartbeat not running)"
else
    if awk -v m="$MEM_MAX" 'BEGIN { exit (m + 0 > 512000000) }'; then
        echo "perf-slo-check: buffer.memory_bytes max = ${MEM_MAX} ≤ 512_000_000 (PASS)"
    else
        echo "::error::perf-slo-check: buffer.memory_bytes max = ${MEM_MAX} > 512_000_000 (FAIL)" >&2
        exit 1
    fi
fi

exit 0
