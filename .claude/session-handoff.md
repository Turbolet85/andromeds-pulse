# Session Handoff

**Last Updated:** 2026-08-25T16:11:09Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 26 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `chore(route): operator-requested adaptation — 0-pending wrap`

## Position
- Done: **no chunk** — this was a 0-pending operator-requested route-adaptation wrap (session 43). No P1 report, no P2 fan-out, no master write, no matrix write.
- Next (first markerless, **changed this wrap**): **Demo injector formalized + api-surface retire** (P-077) — lifted from tail-last. Its verification leg is now the FIRST live REAL-model chain proof (injector-driven sustained scenario under real L4, incident forms, operator judges the brief — ManualCheck class); det-L4 stays the deterministic arm. It carries the re-pinned `cargo audit` PREREQ, **next interval point 46** (43 was discharged here).
- Then: Halo State Pulse canvas disposition · Advisory backlog · npm advisory coverage · Diagnostics un-muting · Staged-bindings assertion · Metrics label surface. Order otherwise unchanged.

## Work done
**Item 1 — the lift (trajectory, operator-ruled).** `Demo injector formalized + api-surface retire` moved tail-last → first markerless, ahead of the Halo entry. Annotations travelled: the `cargo audit` PREREQ re-pinned off the Halo entry (origin + pin #15 ratification preserved), five CARRYs rode along with one augmented — the CONTINUOUS-UNIQUE-STREAM carry is now **load-bearing, not optional**, since a finite ~600-batch storm cannot hold a sustained scenario across the L2→L3 20–60s window plus the L4 queue. Operator's scope hint folded in as `SCOPE:`.

**The operator's three cited blockers were re-derived first-hand before entering route text** (verify-at-HEAD): bootstrap env-override at `crates/triage/src/cue/thresholds.rs:12` · `2961f4e` = the workspace-key-alignment commit · seq/identity family `8e9856c` + `d4b432b`. All three reproduced; no premise correction needed.

**Item 2 — `cargo audit` point 43, IN FULL FORM.** Probe RAN: exit **1** read directly (never through a pipe), cargo-audit 0.22.2; basis **byte-identical** (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`, upstream, DB still unloadable); overlap re-enumerated first-hand — `deny check bans licenses sources` → **ok** (exit 0), `deny check advisories` → exit 1 at the **same 8** owned IDs (0189/0190/0194/0195/0204/0222/0253/0258), set unchanged. Deferral continues; **next point 46**.

## Drift resolved
**3 body writes + 1 sidecar, all from this wrap's own measurement** (no detector fan-out runs on this path):
- `.andromeda/security-plan.md` §Dependency Security probe clause said "next at session 43" — falsified by the probe run here; corrected to the session-43 discharge + next point 46, plus two measured counting rules.
- Same correction cascaded to the leaf `.claude/rules/security.md`; history appended to `.andromeda/security-plan-amendments.md`.
- **Counting rule (a):** advisory ERROR BLOCKS ≠ ID count — this run showed **10 blocks for 8 distinct IDs** (a crate at two lockfile versions raises one block per version; quick-xml's 0194/0195 twice each, `Cargo.lock:509`/`:510`). Reporting blocks as owned-set size manufactures a false "the set grew" alarm.
- **Counting rule (b):** an interval POINT belongs to the wrap that actually OCCURS — inserting a wrap moves the count. Generalizes the operator's instruction so the next inserted wrap need not re-derive it.

## Notes
- **A defect I introduced and caught:** the anchored edit removing the PREREQ from the Halo line also swallowed the following line's break, merging the `↓` separator onto it. The prose read fine; the **separator count (39 vs 40)** caught it, and a second edit restored it. Post-repair integrity: 104 lines · frozen set **byte-identical** (37) · 7 markerless · 40 separators · **0 frozen lines touched**. Curated Tier 3.
- **Surfaced, not absorbed — P-077's matrix bar.** The entry reads `method: by-construction` with acceptance "inject_demo.rs is tracked and builds; the integration UX e2e (P-076) uses it to drive telemetry" — and **P-076 is already `verified`**, so that acceptance is satisfiable **without ever running the real model**. The operator's scope hint raises it to a live ManualCheck proof. Matrix deliberately NOT written here (no cap claimed; concretization is the claiming chunk's at promotion) — recorded as a `NOTE:` on the entry so `/andromeda-phase` must confront it.
- **Curation:** T1 0 · T2 0 · T3 ×1 (the line-terminus edit hazard). Two further candidates were routed OUT of curation into the spec master — their proper home was the probe clause they correct, and curating them too would have duplicated a fact written to the source the leaf derives from. Filters: 0 dup · 1 task-specific · 0 conflict · 0 deferred. CLAUDE.md **155/200**, unchanged.
- Coverage unchanged: **20/22 verified**, 2 unclaimed (P-075 declined-with-notes, P-077 — now the next chunk).
- Audit trail: `.andromeda/runs/2026-08-25T16-11-09Z-wrap/adaptation-record.md`.
- Last failed command: none.

## Deferred learnings
Still open from the previous wrap (not re-surfaced this session, no new evidence): the **deferral-destination generalization** — a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes, since a destination can complete WITHOUT absorbing the deferred evidence, leaving a hollow `verified`. Curate if it recurs.
