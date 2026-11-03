# Curation — 2026-11-03-advisories-since-2026-10-12-closed (P3)

Sources: this window; this report's *Decisions & corrections* (10 entries) and its two addenda; the prior report's *Decisions & corrections* (`doc:2026-11-03-stdio-sidecar-retired/report`, 10 entries — its wrap's P3 never ran, carry `cry-rdhtyshr`); the cascade's curation rows (the prior wrap's 7, this wrap's sweep 0) and `citation-dispositions.md`'s 2 `curation home — P3` rows.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + security.md: "dry-run `cargo update -p {holder}` for the old copy's holders before writing a duplicate-version skip" · + testing.md: "a probe over a tree holding a committed generated artifact runs after its regen"
  Tier 3 (.claude/docs/session-learnings.md): + "A `git diff` scope guard cannot read a recorded companion"
  Corrections (cap-exempt, in place, `[corrected 2026-11-03: …]`): security.md 2026-05-19, 2026-06-12, 2026-08-17 · testing.md 2026-05-13, 2026-05-17, 2026-05-22, 2026-05-25, 2026-06-28, 2026-07-02, 2026-08-15 · session-learnings.md «Regenerate `bindings.ts` after:» (11)
  Judgment base: rul-47x4hguw superseded (the founder's word, carry `cry-rdhtyshr`); rul-beeokpoq already superseded; rul-kzdqmcs7 left standing, PROVISIONAL (inputs#I7)
  Filters: 0 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "dry-run the old copy's holders before a skip" · "a probe over a committed generated artifact runs after its regen" · "a git-diff scope guard cannot read a recorded companion"

## Applied
- T2 security.md 2026-11-03 (dry-run the holders) — confidence 0.8 (measured +0.4: it changed the chunk's design, eight skips replaced by one in-range update; technical detail +0.2; no other home +0.2 — P2 amended no master with it, fct-hhbgn5ow kept).
  Proof: `cargo update -p notify-rust --dry-run` printed «Removing windows v0.61.3 … windows-threading v0.1.0» (report Deviations; inputs#I3).
- T2 testing.md 2026-11-03 (probe after the regen) — confidence 0.8 (measured +0.4: a real gate failure; technical +0.2; no other home +0.2).
  Proof: the prior chunk's identifier grep over `pulse-app/ui/src` red on 3 lines of `bindings/index.ts`, green after the regen (`doc:2026-11-03-stdio-sidecar-retired/report`, Decisions & corrections).
- T3 session-learnings.md 2026-11-03 (scope guard vs a recorded companion) — confidence 0.8 (measured +0.4; technical +0.2; no other home +0.2).
  Proof: the prior chunk's entry 3 (`git diff` scope guard) red on `xtask/src/staged_gate.rs` while `gate.py scope` read clean (same report).
- Corrections ×11 — the «no-mcp shape» / `--features mcp-server` regen / `grep -c '"mcp":'` claims (7), a sibling-binary example and two feature-graph examples naming the retired feature (3), and the designed-red `cargo deny check advisories` claim (1).
  Proof: `pulse-app/Cargo.toml [features]` holds `default` and `otap-ingest` (read at this wrap); the prior chunk retired the feature and the sidecar (its report, Retired); this chunk's gate `git diff --quiet 0e13e255 -- pulse-app/ui/src/bindings/index.ts` green after the featureless regen; `cargo deny check advisories` exit 0, `advisories ok` (this report, Spec claims disproved).

## Each Decisions & corrections entry — its disposition
This report:
1. notify-rust in place of eight skips → applied (T2 security.md).
2. two provenance sentences from recall false → recurrence-despite-learning: the 2026-08-29 Tier 2 entry (`cargo tree -i` host-target reach; derive holders from the lockfile) → handoff.
3. the designed-red advisories line → corrected in place (security.md 2026-08-17).
4. sweep hazard `\bzip\b` matches `extract-zip` → rejected — Filter 4 (0.4: technical +0.2, one-off −0.3, measured +0.4, could be task-specific −0.2… below 0.6).
5. research's «19 ids» → not a learning (a chunk-artifact count, a report entry).
6. guard refusals → not a learning (telemetry, evolve).
7. the lean-checks directive → not a learning (a chunk directive).
8. the operator pass under no skill → not a learning (a record of the pass).
9. the split wrap → not a learning (a record).
10. carry `cry-rdhtyshr` rev 3 → freight, worked phase by phase (P2 · P3 · P6 · evolve).
Addendum 1 / 2 (inputs#I5, #I6) → not learnings (the operator's rulings, carried by the masters' amendments and sidecars).
The prior report (`2026-11-03-stdio-sidecar-retired`):
1. lean checks → not a learning (a chunk directive).
2. diff-shaped probes anchored to the chunk base → duplicate (Filter 1): `rules/testing.md` 2026-10-04 states it.
3. the overseer's companion word (staged_gate.rs) → not a learning (a ruling, confirmed by the founder — carry).
4. a probe fires before its regen → applied (T2 testing.md).
5. a `git diff` scope guard cannot read a recorded companion → applied (T3).
6. a plan step naming a test mechanism the surface does not answer (`TraceRow`) → rejected — Filter 2 (task-specific: one struct's fields).
7. Bash guard cwd arm refused a leading `cd` twice → recurrence-despite-learning: `rules/host-linux.md` Session Additions 2026-10-05 (recurred again in this window, twice) → handoff.
8. a Write call swallowed the next call's tail into a file → not a learning (a tool fault, caught by `gate.py hygiene`; evolve's).
9. census none running → not a learning.
10. staged_gate.rs re-point keeps discrimination → not a learning (a fact of that chunk).
Citation rows: `.claude/rules/observability.md:149` @c2263, @c2272 (out-of-range, bound by position) → no change: a dated reading inside a preserved entry, its numbers lines of `pulse-app/src/observability.rs` at its own date, never patched.
