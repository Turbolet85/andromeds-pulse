#!/usr/bin/env bash
# Tier C runner — 7 operator-chosen units, sequential, 25-min hard ceiling per unit
# (15-min budget + cold-build allowance post cargo-clean; rc=124 => budget-exhausted, partials parsed).
# Resumable by artifact presence (outcomes.json). jobs=1 (memory floor). --test-tool=nextest pinned.
RD=".andromeda/runs/2026-08-31T08-07-26Z-code-audit"
for u in curation security snapshot workspace-detector interpretation viz config-watcher; do
  if [ -f "$RD/mutants-$u/mutants.out/outcomes.json" ]; then
    echo "$u: skip (outcomes.json present)" >> "$RD/mutants-progress.txt"
    continue
  fi
  start=$(date +%s)
  timeout 1500 cargo mutants -p "$u" --test-tool=nextest -o "$RD/mutants-$u" > "$RD/mutants-$u.log" 2>&1
  rc=$?
  end=$(date +%s)
  echo "$u rc=$rc dur=$((end-start))s" >> "$RD/mutants-progress.txt"
done
echo "DONE" >> "$RD/mutants-progress.txt"
