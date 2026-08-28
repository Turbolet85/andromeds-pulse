# Session Handoff

**Last Updated:** 2026-08-28T20:50:00Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 38 ahead after this wrap's commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-28-ingest-consumer-block-under-gap-resume): the idle gap is not the trigger, and the hang is inside flush`

## Position
- Done: **2026-08-28-ingest-consumer-block-under-gap-resume** — shipped the `--reconnect-only` third arm and **attributed the wedge**: the block is inside `Appender::flush()` on the batch AFTER a constraint-violating flush, reached because the injector re-emits its own primary keys on every restart. The repair was blocked by scope (T3) and moves forward.
- Next (first markerless): **Duplicate-span replay fails loudly** (NEW, operator-placed FIRST this wrap) — modify-set = the appender + the demo injector; acceptance = `smoke:gap-resume` arm A exits 0 with its preconditions satisfied. Carries the audit PREREQ (pin **#20**, next interval point **55**; sessions 53 and 54 are between-points).
- Then: App-registry reconciliation with externally-resolved rows · Halo State Pulse canvas disposition · Advisory backlog · npm advisory coverage · Diagnostics un-muting + harness-truth sweep · Staged-bindings assertion · Metrics label surface · ACL-rejection logging.

## Work done
Partial delivery by design, ratified by the operator's wrap directive ("wrap AS IS; the repair is the NEXT chunk"). **Delivered:** the third harness arm (`LegArm` enum, third `judge` arm, `StormEvidence::fed()` so a stormless arm cannot pass vacuously) and the attribution. **Not delivered:** the repair, its pin, and a green arm A — all downstream of a soft-exit trigger 3, because the true cause lives in two files outside the plan's modify-set.

**The attribution.** `inject_demo` derives `trace_id`/`span_id` purely from a batch counter that restarts at 0 each process, so process 2 replays process 1's exact keys. The first replayed batch fails LOUDLY at `Appender::flush()` (one `duckdb.append` ERROR, `reject_reason: "append_failed"`); the **next** one hangs unbounded holding the shared connection mutex → channel 0 → 100 %, `ingest.channel.full`, `buffer.consumer.stalled` at 450 s, 24k+ spans unpersisted, at 0 ERROR after the first. **A retrying OTLP exporter resending spans is normal client behaviour that reaches this hang.**

Gates: nextest **2044/2044 + 1 skip** (+2 = the two new pins exactly) · clippy 0 · fmt clean · `capability-widening-check` clean (0/3) · `check:ingest-progress` PASS · `deny bans licenses sources` exit 0 · `capability-drift` clean LAST after bindings regen (`bindings/index.ts` byte-identical to HEAD) · **0 fix-loop iterations**. Four real boots, clean teardown, 0 orphans. `smoke:gap-resume` arm A **red by design** — the documented scenario-not-gate treatment, recorded and never weakened.

## Drift resolved
**3 amendments across 3 masters · 0 escalations · drift = 0.** obs-plan §10 (lead-in + defect 4 — mechanism corrected to the measured chain, defect kept OPEN, owner moved to the new entry, three prior claims explicitly retired) · test-plan §3 (leg registered at THREE arms, per-arm non-borrowable preconditions, observe-window gap recorded with its owner) · security-plan §Dependency Security (session-52 discharge recorded, pointer moved to 55, running "Nth consecutive" ordinal retired per operator ruling). Fan-out returned 1 detector proposal (test-plan); the other two were orchestrator-raised under Validate check 5/6 — **no drift-base detector covers a falsified narrative mechanism in a spec body**, so the plan's Expected-amendments floor is what caught the obs-plan one. Cascade re-derived 5 leaves: `rules/observability.md` · `docs/obs-summary.md` · `rules/verification-harness.md` · `rules/security.md` · `docs/tests-summary.md`. CLAUDE.md warnings block checked, correctly unchanged.

## Notes
- **Curation:** T1 0 · T2 2 (1 new + 1 in-place extension) · T3 1 · 0 filtered · 0 conflicts · 0 deferred. `rules/verification-harness.md` (a leg's run window must outlast the threshold its verdict reads, else the verdict is structurally incapable of failing; plus the general rule that a new arm on a self-proving leg needs its OWN precondition) + `rules/testing.md` extension (the `| tail` truncation that masks an exit code also poisons a WAIT PREDICATE — key the wait on task completion, never on a string the producer may have dropped) + `docs/session-learnings.md` T3 (collapse a candidate field with an env-gated step probe on stderr, not by inference; read the LAST marker, not the torn counts). CLAUDE.md untouched at **156/200**.
- **Coverage:** chunk claimed 0 caps; version stays **21/22 verified, P-075 pooled** (Conductor's).
- **Audit PREREQ (session 53):** between-point. Basis + overlap RE-VERIFIED first-hand at this wrap — `cargo audit` still cannot load (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`), `cargo deny check advisories` exit 1 at 8 DISTINCT ids 0189/0190/0194/0195/0204/0222/0253/0258 (set identical to the prior enumeration), `bans licenses sources` exit 0. Recorded `probe skipped per ratified interval (next: 55)`. **The running "Nth consecutive" ordinal is retired** — two cadences (the 3-wrap probe, the per-wrap overlap) were both narrated that way and drifted.
- **THE finding worth carrying forward:** the chunk was scoped to repair a connection-isolation defect and instead proved there wasn't one. Three durable artifacts had encoded a causal mechanism nobody had re-derived — "both shared-connection users die" named a retention sweep that has held its own cloned connection since chunk #99. A step probe then excluded all three planned candidates in one run. **Generalization: when a defect's mechanism is described in prose across several artifacts, re-derive it before building on it; the cost is one grep, and the alternative is a repair aimed at the wrong subsystem.**
- **Stated rather than implied:** WHY `flush()` hangs after a failed flush is characterized but not explained at the DuckDB level — the repair entry owns that. Arm A stays red until it lands. The reproduction logs are under `target/gap-resume/`, gitignored and volatile.
- **Surfaced, not actioned:** the drift-base has no detector for a falsified narrative mechanism in a spec body (this is arguably its second occurrence). Proposing one needs operator approval; not minted unilaterally.
- Audit trail: `.andromeda/runs/2026-08-28T20-25-00Z-wrap/` (+ phase run dir `2026-08-28T19-05-00Z-phase/`).
- Last failed command: none.

## Deferred learnings
- None deferred by the cap — 2 new entries + 1 in-place extension against a cap of 3.
- Still open from prior wraps: the **deferral-destination generalization** (a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes).

## Session End Status
Completed normally at 2026-08-28 22:50:00
