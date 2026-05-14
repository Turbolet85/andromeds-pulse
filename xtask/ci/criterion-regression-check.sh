#!/usr/bin/env bash
# Chunk #56 criterion bench regression detector. Compares current
# target/criterion/<bench>/new/estimates.json against base-branch
# criterion-baseline/<bench>/new/estimates.json; uses jq к extract
# .mean.point_estimate; fails if mean regresses by more than +10%
# (default; CRITERION_REGRESSION_THRESHOLD_PCT override). NEUTRAL when
# baseline missing (first PR / new bench / no criterion runs yet).
#
# Usage: criterion-regression-check.sh <current-criterion-dir> <baseline-criterion-dir>

set -euo pipefail

CURRENT_DIR="${1:-target/criterion}"
BASELINE_DIR="${2:-}"
THRESHOLD_PCT="${CRITERION_REGRESSION_THRESHOLD_PCT:-10.0}"

if [ ! -d "$CURRENT_DIR" ]; then
    echo "criterion-regression-check: current dir ${CURRENT_DIR} not found; NEUTRAL (no criterion bench output yet)"
    exit 0
fi

if [ -z "$BASELINE_DIR" ] || [ ! -d "$BASELINE_DIR" ]; then
    echo "criterion-regression-check: baseline ${BASELINE_DIR:-(unset)} not found; NEUTRAL (first PR / new branch / local dev)"
    exit 0
fi

if ! command -v jq >/dev/null 2>&1; then
    echo "criterion-regression-check: jq not available; treating as NEUTRAL" >&2
    exit 0
fi

mapfile -t CURRENT_ESTIMATES < <(find "$CURRENT_DIR" -type f -name 'estimates.json' -path '*/new/*' 2>/dev/null | sort)

if [ "${#CURRENT_ESTIMATES[@]}" -eq 0 ]; then
    echo "criterion-regression-check: no estimates.json found under ${CURRENT_DIR}; NEUTRAL (no benches present)"
    exit 0
fi

fail=0
checked=0
for cur_path in "${CURRENT_ESTIMATES[@]}"; do
    rel="${cur_path#$CURRENT_DIR/}"
    base_path="$BASELINE_DIR/$rel"
    if [ ! -f "$base_path" ]; then
        echo "criterion-regression-check: skipping ${rel} (no baseline equivalent at ${base_path})"
        continue
    fi
    cur_mean=$(jq -r '.mean.point_estimate // empty' "$cur_path" 2>/dev/null || true)
    base_mean=$(jq -r '.mean.point_estimate // empty' "$base_path" 2>/dev/null || true)
    if [ -z "$cur_mean" ] || [ -z "$base_mean" ]; then
        echo "criterion-regression-check: skipping ${rel} (mean.point_estimate missing in current or baseline)"
        continue
    fi
    bench_name=$(printf '%s' "$rel" | sed -E 's|/new/estimates\.json$||')
    pct_change=$(awk -v c="$cur_mean" -v b="$base_mean" 'BEGIN {
        if (b + 0 == 0) { print "0.00"; exit }
        printf "%.2f", ((c - b) / b) * 100
    }')
    checked=$((checked + 1))
    if awk -v p="$pct_change" -v t="$THRESHOLD_PCT" 'BEGIN { exit (p + 0 > t + 0) }'; then
        echo "  ${bench_name}: mean ${cur_mean}ns vs baseline ${base_mean}ns (${pct_change}% change; threshold ${THRESHOLD_PCT}%) PASS"
    else
        echo "::error::criterion-regression-check: ${bench_name} regressed by ${pct_change}% (current mean ${cur_mean}ns vs baseline ${base_mean}ns; threshold ${THRESHOLD_PCT}%)" >&2
        fail=1
    fi
done

if [ "$checked" -eq 0 ]; then
    echo "criterion-regression-check: NEUTRAL (no benches with matching baseline)"
    exit 0
fi

if [ "$fail" -eq 0 ]; then
    echo "criterion-regression-check: PASS (${checked} bench(es) within ${THRESHOLD_PCT}% threshold)"
fi

exit $fail
