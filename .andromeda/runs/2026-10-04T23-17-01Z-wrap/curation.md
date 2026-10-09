# Curation — 2026-10-04-l4-framing-measured-on-the-real-model

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a zero-Rust-delta deferral never holds for the workspace nextest while any file is uncommitted — the two walks-tree-marked xtask tests void it; defer clippy alone"
    Proof: implement run 2026-10-04T23-09-00Z-implement. The gate header printed `walk-class rust 2 (xtask/src/source_lint.rs, xtask/src/staged_gate.rs) · uncommitted 38` against a plan whose entries 10–11 carried `defer = 'zero Rust delta …'`. Both deferrals were voided and run green: nextest 2630/2630, the bindings regen exit 0. The untracked chunk folder alone keeps `uncommitted` at ≥ 1 for every chunk before its wrap commit. Confidence 0.8: measured (a plan claim falsified, +0.4) · specific technical detail (+0.2) · reached no other durable home this wrap (+0.2; the report records it, which is not one of the excluded homes).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold ("drive a pre-registered operator leg from a script asserted byte-equal to the plan's run": one-off, no failure proved it; 0.2 − 0.3)
  Load-bearing: none
  No-other-home: "zero-Rust-delta deferral never holds for the workspace nextest while any file is uncommitted"
  Extended: none
