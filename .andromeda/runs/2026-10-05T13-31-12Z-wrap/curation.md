# Curation — 2026-10-05-l4-model-chosen-by-pattern-discrimination

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   correction testing.md (the 2026-09-30 evidence-capture entry) · extended testing.md (the 2026-08-29 `[[example]]` entry)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred · 2 one-off (below the bar)
  No-other-home: "an example file is a crate root, so its `mod x;` resolves to `examples/x.rs` (auto-discovered as a new example); use `#[path]`"
  Extended: T2/testing.md: "CARGO DEFAULTS AN AUTO-DISCOVERED `[[example]]` TARGET TO `test = false`" + the crate-root module-resolution facet
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B

## Applied

- **Correction** (cap-exempt; the operator's curation note): testing.md 2026-09-30 entry, its "Extended 2026-10-04"
  clause said a saved `gate.py run` capture trips hygiene on the tool's own `root` / `logs` header lines. Rewritten to
  the measured truth with `[corrected 2026-10-05: …]`: gate v1.10 prints the root as `.`, its log dir as
  `$TMPDIR/andromeda-gate` and the home as `~`; a host path the plan's own text holds still prints as written.
  Proof: this session's gate runs printed `root . · marker … · entries 39` and `logs
  $TMPDIR/andromeda-gate/2026-10-05-l4-model-chosen-by-pattern-discrimination/implement-2026-10-05T10-22-12Z` (the
  /implement run's listing), matching gate-contract.md §Tool "The printed form (v1.10)"; the operator flagged it
  ("testing.md:302's scrub rule is stale for gate v1.10 captures").
- **Extension** (confidence 0.8: measured by a real compile failure +0.4 · specific technical detail +0.2 · no other
  durable home +0.2 — the fact sits in no route annotation, master, playbook rule, matrix note or contract
  document; the report and a source comment carry it): testing.md 2026-08-29 `[[example]]` entry, one sentence
  tagged `Extended 2026-10-05`.
  Proof: the first compile of `mod patterns;` in `pulse-app/examples/l4_decision_probe.rs` failed E0583 "file not
  found for module `patterns` — create file pulse-app/examples/patterns.rs"; research.md's convention had predicted
  `examples/l4_decision_probe/patterns.rs`. Fixed with `#[path = "l4_decision_probe/patterns.rs"]` (report §Spec
  claims disproved).

## Rejected

- "A parser over a hand-written verdict / annotation file must strip trailing `#` notes, or an annotated file is
  refused" — 0.6 (measured by the refused grade +0.4, detail +0.2) −0.2 could be task-specific → 0.4. Recorded in
  `evidence/preregistration-addendum.md`.
- "A word-start keyword match still fires inside a compound (`down` → downstream)" — 0.6 −0.2 task-specific (this
  probe's grader) → 0.4. Recorded in `evidence/audit.md` (finding 2).
- "Stamp a durable record from the clock at write time, never a placeholder" — one-off self-correction (−0.3);
  covered in substance by the Tier 1 durable-text entry.
- "`cargo fmt --manifest-path {root}/Cargo.toml` at the workspace root prints `Failed to find targets`; run `cargo
  fmt` from the root" — one-off (−0.3).
