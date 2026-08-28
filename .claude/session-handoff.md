# Session Handoff

**Last Updated:** 2026-08-28T18:50:00Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 37 ahead after this wrap's commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-28-ingest-consumer-initiating-freeze): the wedge reproduces on demand, and the bound is measured not to be the fix`

## Position
- Done: **2026-08-28-ingest-consumer-initiating-freeze** — built `cargo xtask smoke:gap-resume`, which **reproduces the ingest wedge on demand at HEAD for the first time ever**, and used it to falsify the chunk's own selected premise: the `CueLatch` + damper bound does NOT prevent the consumer death under gap→resume.
- Next (first markerless): **Ingest consumer block under gap→resume** (NEW, operator-placed FIRST this wrap) — carries the audit PREREQ (pin **#19**, next interval point **55**; point 52 was discharged FULL-FORM at this wrap, so the next two wraps are between-points).
- Then: App-registry reconciliation with externally-resolved rows · Halo State Pulse canvas disposition · Advisory backlog · npm advisory coverage · Diagnostics un-muting + harness-truth sweep · Staged-bindings assertion · Metrics label surface · ACL-rejection logging.

## Work done
All acceptance criteria met, with the smoke's finding replacing the expected PASS on arm A. nextest **2042/2042 + 1 skip** (+8 = the 8 new pins exactly) · clippy 0 · fmt clean · `capability-widening-check` clean (0/3) · `check:ingest-progress` PASS · `deny bans licenses sources` exit 0 · `capability-drift` clean LAST after bindings regen (`bindings/index.ts` byte-identical to HEAD) · **1 fix-loop iteration** (the documented rustfmt-hook/edition mismatch). No gate deferrals — Rust delta present, so every workspace gate ran. Webview + a11y gates omitted (zero `pulse-app/ui/**` delta).

**Smoke — two arms, one binary, one host.** Arm A (gap→resume): **FAIL**, `buffer.consumer.stalled` at 450s, `rows_ingested` frozen at 3,240, channel 0 → 85.7 %, `ingest.channel.full` fired, 24k+ accepted spans unpersisted, **0 ERROR / 0 panics**. Arm B (`--sustained` control): **PASS**, 29,160 rows, 1 zero-delta tick of 38, 0 stall records. Clean teardown both arms (0 orphans, ports released).

## Drift resolved
**3 amendments across 2 masters · 1 escalation resolved · drift = 0.** test-plan ×2 (§3 registers `smoke:gap-resume` as a SCENARIO explicitly outside the gate set; §3 Direct-binary smoke variant's RED-leg rule QUALIFIED by defect shape) · obs-plan ×1 (§10 gains a fourth, explicitly **OPEN** isolation defect with a named owner, plus a detector-exercised-on-a-live-reproduction note). Escalation: whether §10 should record an open defect at all — the directive framed the obs amendment as "no semantic change" and the measurement required one; operator chose to record it. Cascade: `rules/observability.md` · `docs/obs-summary.md` · `rules/verification-harness.md` · `docs/tests-summary.md`. CLAUDE.md warnings block checked, correctly unchanged. Four docs clean (arch, security, design, layouts).

## Notes
- **Curation:** T1 0 · T2 2 · T3 1 · 1 in-place extension · 0 filtered · 0 conflicts · 0 deferred. `rules/verification-harness.md` (a leg reproducing a known open defect is a SCENARIO, not a gate — never green it; explicitly distinguished from the `#[ignore]`-is-not-a-fix ban, which forbids HIDING a failure) + `rules/testing.md` (what a RED leg PRODUCES depends on the defect's shape — a BLOCK-shaped defect emits nothing, so the clean-log assertion goes vacuous) + `docs/session-learnings.md` T3 (a "spec claims disproved" list must scan the claims governing the METHOD, not only the domain). Extension: the 2026-07-05 boot-smoke entry now records that the stale-binary trap belongs to the shared `locate_pulse_binary` resolver, not to two named harnesses, and that the leg should print its selected binary + age. CLAUDE.md untouched at **156/200**.
- **Coverage:** chunk claimed 0 caps; version stays **21/22 verified, P-075 pooled** (Conductor's).
- **Audit PREREQ (session 52):** FULL-FORM probe RAN. `cargo audit` exit 1 read directly under cargo-audit 0.22.2; basis reproduced byte-identical (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`); overlap re-enumerated first-hand — `bans licenses sources` exit 0, `advisories` exit 1 at the same **8 DISTINCT ids** 0189/0190/0194/0195/0204/0222/0253/0258, **sixth** consecutive identical probe result. 10 error BLOCKS vs 8 distinct ids re-confirms the counting rule. **Next interval point: 55.**
- **THE finding worth carrying forward:** a chunk built to CONFIRM a fix instead disproved it, and that was only possible because the scenario was built before the conclusion. The predecessors' "does not reproduce" rested on a falsified premise — their matched leg ran `--sustained`, which never disconnects, so it reproduced the stated preconditions faithfully and the wrong scenario. Once the real shape was reconstructed from the corpus, the wedge reproduced first try. **Generalization: when something "does not reproduce", re-derive the preconditions before trusting the negative — a repro attempt is only as good as the premise it encodes.**
- **Stated rather than implied:** WHY the first post-resume batch blocks is NOT established, and arm A changes two variables at once (180s idle AND a new gRPC connection). The repair entry's first arm exists to separate them. The reproduction logs (81 MB + 59 MB) are under `target/gap-resume/`, gitignored and volatile — the scenario regenerates them in ~13 min.
- **A playbook rule was added** (operator-approved): recording a measured OPEN defect into a section framed as closed is routine-APPLY when the entry names an owner — refining the 2026-08-14/2026-06-29 impl-half test, whose "YES → HANDOFF" branch assumed the amendment would CLAIM a fix.
- Audit trail: `.andromeda/runs/2026-08-28T18-30-00Z-wrap/` (+ phase run dir `2026-08-28T17-05-00Z-phase/`).
- Last failed command: none.

## Deferred learnings
- None deferred by the cap — 3 candidates passed Filter 4 against a cap of 3, plus one additive-facet extension that does not consume a slot.
- **Worth noting for the pipeline:** curation deduped twice against text THIS SAME WRAP had just written (the cascade's rule-body edits minutes earlier). Filter 1's DEDUP-REJECT-ONLY clause covers it, but the self-collision only exists because cascade and curation share a wrap; both calls were underdetermined by the written filter.
- Still open from prior wraps: the **deferral-destination generalization** (a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes).
