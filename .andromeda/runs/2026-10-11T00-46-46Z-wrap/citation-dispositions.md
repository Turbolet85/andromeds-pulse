sweep: base: 303fa9a8 — the previous sweep (2026-10-10T22-04-24Z-wrap) · re-pointed 0 · changed 0 · stretched 0

# Citation dispositions — 2026-10-10-door-inside-the-engine-s-process

The sweep (`cites.py apply`, cites v1.4, the second window of this wrap; the dry run first, then the write) wrote no
number: `re-pointed 0 citation(s) in 0 file(s)`. Its summary: sites 85 over 72 files, unmoved 63, moved 0 written,
out-of-range 2, unresolved 17, `held 0`, listed and not written 3. It printed five rows, one of them in a master. Each
row was read at its citing line; a citing line over 2 000 chars was read by offset (`cascade.py window`).

moved .andromeda/obs-plan.md:454@c680 observability.rs:1934 → 1958 — listed, not written: suffix → not a citation of the tree — no change
  (read at the offset, chars 480-880 of 2219: the number is the dated coordinate of a measurement, "measured VIOLATED
  2026-08-26 at `observability.rs:1934`", of a bare `interpretation` key the same sentence says was REMOVED. The base
  line 1934 (`git show 303fa9a8:pulse-app/src/observability.rs`) reads `);` and the mapped line 1958 in the work tree
  reads `);`, the close of an unrelated allowlist entry, neither the removed key: a re-point would name an unrelated
  line. The same row took the same disposition at the 2026-10-10T14-17-29Z wrap. The move is this chunk's 24 added
  allowlist lines, `observability.rs:1022-1045` in the report's last section.)

moved .claude/rules/observability.md:69 observability.rs:1934 → 1958 — listed, not written: leaf → leaf — re-derived
  (line 69 stands in the body, under `## PII scrubbing (Vectors 1–6)`, above `## Session Additions` at line 124. It
  restates obs-plan §8's sentence, whose dated coordinate stands by the row above, so the re-derived sentence carries
  the same number. The leaf joins cascade step 3's set.)

out-of-range .claude/rules/observability.md:149@c2263 pulse-app/src/heartbeat.rs:1078 — the file held 435 lines at the base · naked: bound by position → curation home — P3
  (line 149 stands under `## Session Additions`, which opens at line 124. Read by offset, chars 2063-2463 of 2823: the
  entry itself says the two numbers are lines of `pulse-app/src/observability.rs` as it stood on 2026-08-26, not of
  any file as it stands now; the position binding to `heartbeat.rs` is the tool's, not the sentence's.)

out-of-range .claude/rules/observability.md:149@c2272 pulse-app/src/heartbeat.rs:1934 — the file held 435 lines at the base · naked: bound by position → curation home — P3
  (the same line and the same reading as the row above)

moved .claude/docs/obs-summary.md:151 observability.rs:1934 → 1958 — listed, not written: leaf → leaf — re-derived
  (the same dated coordinate of the removed key as obs-plan §8's; the re-derived sentence is the same. The leaf joins
  cascade step 3's set.)

One row stands in a master (`.andromeda/obs-plan.md:454`) and is left as it stands with its reason: it was listed, not
written, so the tool holds nothing for it. No `held` row (`held 0`; the main overseer's `held 3` did not occur). No
`claim false` row, so nothing is raised at Validate from this sweep.
