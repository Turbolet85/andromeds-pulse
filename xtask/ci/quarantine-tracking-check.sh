#!/usr/bin/env bash
# Chunk #55 quarantine convention enforcement. Greps for #[ignore] in Rust
# source under crates/*/src/, pulse-app/{src,tests}/, and xtask/src/; for
# each match, scans the surrounding 5-line window for a GitHub issue URL
# (https://github.com/.../issues/N). Fails with file:line citation if any
# quarantine lacks tracking link. NEUTRAL (exit 0) when zero #[ignore]
# found in source (current state — establishes the gate for future
# quarantines per test-plan §11 "NEVER commit #[ignore] tests without
# open GitHub issue").
#
# Usage: quarantine-tracking-check.sh [<workspace-root>]
#   Default workspace-root: $(pwd)

set -euo pipefail

WORKSPACE_ROOT="${1:-$(pwd)}"
SEARCH_DIRS=(
    "$WORKSPACE_ROOT/crates"
    "$WORKSPACE_ROOT/pulse-app/src"
    "$WORKSPACE_ROOT/pulse-app/tests"
    "$WORKSPACE_ROOT/xtask/src"
)

matches=()
for dir in "${SEARCH_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then continue; fi
    while IFS= read -r line; do
        matches+=("$line")
    done < <(grep -rn -E '^[[:space:]]*#\[ignore' "$dir" --include='*.rs' 2>/dev/null || true)
done

if [ "${#matches[@]}" -eq 0 ]; then
    echo "quarantine-tracking-check: NEUTRAL (zero #[ignore] in source; gate establishes convention for future quarantines)"
    exit 0
fi

fail=0
for match in "${matches[@]}"; do
    file=$(printf '%s' "$match" | cut -d: -f1)
    line=$(printf '%s' "$match" | cut -d: -f2)
    if ! [[ "$line" =~ ^[0-9]+$ ]]; then continue; fi
    window_start=$((line > 2 ? line - 2 : 1))
    window_end=$((line + 5))
    if ! sed -n "${window_start},${window_end}p" "$file" 2>/dev/null | grep -q -E 'https://github\.com/[^/]+/[^/]+/issues/[0-9]+'; then
        printf '::error::quarantine-tracking-check: %s:%s — #[ignore] lacks GitHub issue URL in surrounding 5-line window\n' "$file" "$line" >&2
        fail=1
    fi
done

if [ "$fail" -eq 0 ]; then
    echo "quarantine-tracking-check: PASS (${#matches[@]} quarantine(s) tracked via GitHub issue URL)"
fi

exit $fail
