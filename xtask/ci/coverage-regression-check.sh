#!/usr/bin/env bash
# Chunk #55 coverage regression detector. Compares current lcov.info against
# base-branch lcov-baseline.info; computes line/branch/function delta via awk
# over LCOV LF/LH/BRF/BRH/FNF/FNH counters. Fails if any metric regresses by
# more than the threshold (default +0.0pp — strict no-decrease per test-plan
# §11 "NEVER lower coverage thresholds to pass build"). NEUTRAL (exit 0) when
# baseline missing — first PR / new branch / local dev runs without artifact.
#
# Usage: coverage-regression-check.sh <current-lcov.info> <baseline-lcov.info>

set -euo pipefail

CURRENT="${1:-lcov.info}"
BASELINE="${2:-}"
THRESHOLD_PP="${COVERAGE_REGRESSION_THRESHOLD_PP:-0.0}"

if [ ! -f "$CURRENT" ]; then
    echo "coverage-regression-check: current lcov.info ${CURRENT} not found; treating as NEUTRAL"
    exit 0
fi

if [ -z "$BASELINE" ] || [ ! -f "$BASELINE" ]; then
    echo "coverage-regression-check: baseline ${BASELINE:-(unset)} not found; NEUTRAL (first PR / new branch / local dev)"
    exit 0
fi

# Sum LCOV counters per file: LF=line found, LH=line hit, BRF=branch found,
# BRH=branch hit, FNF=function found, FNH=function hit.
sum_counters() {
    local file="$1"
    awk -F: '
        /^LF:/  { lf += $2 }
        /^LH:/  { lh += $2 }
        /^BRF:/ { brf += $2 }
        /^BRH:/ { brh += $2 }
        /^FNF:/ { fnf += $2 }
        /^FNH:/ { fnh += $2 }
        END { printf "%d %d %d %d %d %d\n", lf+0, lh+0, brf+0, brh+0, fnf+0, fnh+0 }
    ' "$file"
}

read -r CUR_LF CUR_LH CUR_BRF CUR_BRH CUR_FNF CUR_FNH <<< "$(sum_counters "$CURRENT")"
read -r BASE_LF BASE_LH BASE_BRF BASE_BRH BASE_FNF BASE_FNH <<< "$(sum_counters "$BASELINE")"

pct() {
    awk -v h="$1" -v t="$2" 'BEGIN { if (t==0) print "100.00"; else printf "%.2f", (h/t)*100 }'
}

CUR_LINE_PCT=$(pct "$CUR_LH" "$CUR_LF")
CUR_BRANCH_PCT=$(pct "$CUR_BRH" "$CUR_BRF")
CUR_FN_PCT=$(pct "$CUR_FNH" "$CUR_FNF")
BASE_LINE_PCT=$(pct "$BASE_LH" "$BASE_LF")
BASE_BRANCH_PCT=$(pct "$BASE_BRH" "$BASE_BRF")
BASE_FN_PCT=$(pct "$BASE_FNH" "$BASE_FNF")

echo "coverage-regression-check: threshold=${THRESHOLD_PP}pp"
echo "  Line:     current=${CUR_LINE_PCT}% baseline=${BASE_LINE_PCT}%"
echo "  Branch:   current=${CUR_BRANCH_PCT}% baseline=${BASE_BRANCH_PCT}%"
echo "  Function: current=${CUR_FN_PCT}% baseline=${BASE_FN_PCT}%"

check_regression() {
    local metric="$1"
    local cur="$2"
    local base="$3"
    awk -v c="$cur" -v b="$base" -v t="$THRESHOLD_PP" -v m="$metric" 'BEGIN {
        regression = b - c
        if (regression > t + 0.0) {
            printf "::error::coverage-regression-check: %s regressed by %.2fpp (current %.2f%% vs baseline %.2f%%; threshold %spp)\n", m, regression, c, b, t > "/dev/stderr"
            exit 1
        }
    }'
}

fail=0
check_regression "line" "$CUR_LINE_PCT" "$BASE_LINE_PCT" || fail=1
check_regression "branch" "$CUR_BRANCH_PCT" "$BASE_BRANCH_PCT" || fail=1
check_regression "function" "$CUR_FN_PCT" "$BASE_FN_PCT" || fail=1

if [ "$fail" -eq 0 ]; then
    echo "coverage-regression-check: PASS (no metric regressed by more than ${THRESHOLD_PP}pp)"
fi

exit $fail
