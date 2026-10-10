#!/usr/bin/env bash
# Quarantine convention enforcement (test-plan §11: NEVER commit #[ignore]
# tests without an open GitHub issue). Greps for #[ignore] in the Rust
# sources under crates/, pulse-app/{src,tests}/ and xtask/src/; for each
# match, scans the surrounding 5-line window for a GitHub issue URL
# (https://github.com/.../issues/N) and fails with a file:line citation when
# a quarantine lacks one.
#
# A check that read nothing has nothing to pass: a missing search dir fails,
# and so does a scan of zero .rs files. The PASS line says how many files
# were scanned.
#
# Usage: quarantine-tracking-check.sh [<workspace-root>]
#   Default workspace-root: $(pwd)

set -euo pipefail

WORKSPACE_ROOT="${1:-$(pwd)}"
SEARCH_DIRS=(
    "crates"
    "pulse-app/src"
    "pulse-app/tests"
    "xtask/src"
)

missing=0
for dir in "${SEARCH_DIRS[@]}"; do
    if [ ! -d "$WORKSPACE_ROOT/$dir" ]; then
        printf '::error::quarantine-tracking-check: search dir %s is missing\n' "$dir" >&2
        missing=1
    fi
done
if [ "$missing" -ne 0 ]; then
    exit 1
fi

files=0
matches=()
for dir in "${SEARCH_DIRS[@]}"; do
    count=$(find "$WORKSPACE_ROOT/$dir" -type f -name '*.rs' | wc -l)
    files=$((files + count))
    while IFS= read -r line; do
        matches+=("$line")
    done < <(grep -rn -E '^[[:space:]]*#\[ignore' "$WORKSPACE_ROOT/$dir" --include='*.rs' 2>/dev/null || true)
done

if [ "$files" -eq 0 ]; then
    printf '::error::quarantine-tracking-check: no .rs file under %s\n' "${SEARCH_DIRS[*]}" >&2
    exit 1
fi

if [ "${#matches[@]}" -eq 0 ]; then
    echo "quarantine-tracking-check: PASS (0 quarantine(s) across ${files} file(s))"
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
    echo "quarantine-tracking-check: PASS (${#matches[@]} quarantine(s) tracked via GitHub issue URL across ${files} file(s))"
fi

exit $fail
