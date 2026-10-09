#!/usr/bin/env bash
# Overnight mutation supplement — the three units declared as skips[declined] in audit record #1.
# Pinned recipe (collectors.md C1): --test-tool=nextest pass-through, jobs=1, scratch copy.
# Budget: 100-min hard ceiling per unit (operator-set); rc=124 => budget-exhausted, partials parsed.
# Resumable by artifact presence (outcomes.json OR a partial txt list).
SD=".andromeda/runs/2026-08-31T08-07-26Z-code-audit/supplement-heavy-trio"
CEIL=6000

for u in buffer triage pulse-app; do
  if [ -f "$SD/mutants-$u/mutants.out/outcomes.json" ]; then
    echo "$u: skip (outcomes.json present)" >> "$SD/progress.txt"
    continue
  fi
  free=$(powershell -NoProfile -Command '"{0:N1}" -f ((Get-PSDrive D).Free/1GB)' 2>/dev/null | tr -d '\r')
  echo "$u START $(date -u +%FT%TZ) D_free=${free}GB" >> "$SD/progress.txt"
  start=$(date +%s)
  timeout $CEIL cargo mutants -p "$u" --test-tool=nextest -o "$SD/mutants-$u" > "$SD/mutants-$u.log" 2>&1
  rc=$?
  end=$(date +%s)
  echo "$u rc=$rc dur=$((end-start))s $(date -u +%FT%TZ)" >> "$SD/progress.txt"
done
echo "DONE $(date -u +%FT%TZ)" >> "$SD/progress.txt"
