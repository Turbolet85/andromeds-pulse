# Session Handoff

**Last Updated:** 2026-08-29T06:35:13Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 39 ahead after this wrap's commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-28-duplicate-span-replay-fails-loudly): a replayed batch is rejected loudly and ingest keeps draining`

## Position
- Done: **2026-08-28-duplicate-span-replay-fails-loudly** — the consumer-wedge defect is CLOSED, and **the fix was a stale lockfile, not code**. `duckdb`/`libduckdb-sys` 1.10502.0 → 1.10505.0 under an untouched `^1.10500` caret requirement makes a constraint-violating `Appender::flush()` return `Err` where it previously blocked forever. `append_record_batch_to_table` ships byte-identical to HEAD.
- Next (first markerless): **App-registry reconciliation with externally-resolved rows** — after an external MCP resolve the UI shows the incident active until restart. Carries the **audit PREREQ (pin #21)** — and **session 55 is the next INTERVAL POINT, so it owes the FULL-FORM `cargo audit` probe** (53 and 54 were between-points).
- Then: Halo State Pulse canvas disposition · Advisory backlog · npm advisory coverage · Diagnostics un-muting + harness-truth sweep · Staged-bindings assertion · Metrics label surface · ACL-rejection logging.

## Work done
The chunk began as a code repair and ended as a dependency bump, via a **SURFACE** at /implement: the plan's approved fork ("reset the connection after a failed `flush()`") was unimplementable, because a step-marker probe measured the **FIRST** violating flush blocking with no error ever returned — there was nothing to hook a reset onto. `duckdb` 1.10502's `Appender` exposes only `flush`/`add_column`/`clear_columns` over a direct FFI call with no abort or timeout, so no Rust-side API could interrupt it. On the operator's "2 then 1" direction (bump first, re-plan only if it failed), the bump fixed it and no re-plan was needed.

**Delivered:** the upstream fix + the injector's per-run identity salt (`fresh_run_salt`, folded into `trace_id`/`span_id` ONLY — never `seq`, which also drives `error_roll` and jitter, so the storm budget is untouched) + a deliberate `--replay` flag + arm A's two-half verdict + the observe-window guard + 12 tests.

**Acceptance MET:** `smoke:gap-resume` arm A exits 0 with both halves — storm formed (15 cues / 5 tier1) AND **120 of 2270 appends rejected on replayed identities**, consumer draining across 54 ticks. The rejection count is self-checking: run 1 emits 120 batches, run 2 replays exactly those under the pinned salt, and exactly 120 were rejected. Arms B and C also PASS. 249,514 log lines, **0 panics**, ERROR set exactly those 120 `duckdb.append` / `append_failed` records.

Gates: nextest **2056/2056 + 1 skip** (+12 = the tests added exactly) · clippy 0 · fmt clean · `capability-widening-check` clean (0/3) · `check:ingest-progress` PASS · `deny bans licenses sources` **ok** · `capability-drift` clean LAST after bindings regen · **2 fix-loop iterations** · no gate deferred.

## Drift resolved
**17 amendments across 4 masters · 2 escalations resolved · 1 proposal REJECTED on measurement · drift = 0.** architecture.md (`--replay` registered; byte-identity → budget-identity at 2 sites; duckdb requirement-plus-resolved at 3 sites) · obs-plan §10 (defect 4 OPEN → CLOSED, mechanism corrected a SECOND time + lead-in) · security-plan §Dependency Security (`ureq` build-only carve-out, 3 stale skips pruned, session-54 between-point) · test-plan §1/§3/§4 (arm A gate-grade, observe-window closed, trigger discharged, hang-rationale retired ×2, buffer pin listed).

Cascade re-derived 6 leaves by provenance: `docs/stack.md` · `docs/obs-summary.md` · `rules/observability.md` · `docs/tests-summary.md` · `rules/verification-harness.md` · `rules/security.md`. `docs/conventions.md` correctly excluded (arch §Conventions untouched).

## Notes
- **Curation:** T1 0 · T2 2 · T3 1 · 0 filtered · 0 conflicts · 0 deferred. `rules/testing.md` gains the `[[example]] test = false` dead-test entry (the third member of that family, after `[lib] test = false` and its clippy corollary) plus a foreground-shape extension of the `| tail` exit-masking entry; `docs/session-learnings.md` gains the check-the-cheap-explanation-first entry. CLAUDE.md untouched at **156/200**.
- **Coverage:** chunk claimed 0 caps; version stays **21/22 verified, P-075 pooled** (Conductor's).
- **THE finding worth carrying forward:** the workspace test count moved **+4 when 12 tests were added**, and that arithmetic was the only thing that exposed 8 dead tests. Cargo defaults an auto-discovered `[[example]]` target to `test = false`, so its `#[cfg(test)] mod tests` runs green under `cargo test --example` and is collected zero times by the workspace gate. Every gate was green in both worlds. **Check that the suite count moved by the amount you wrote.**
- **A detector proposal was rejected on measurement.** D-security-logging proposed writing "the whole-batch ERROR claim never held on this project's Arrow appender path" into security-plan, generalizing this chunk's spans-path result. Re-derivation: at commit `7608217` duckdb was the SAME 1.10502, and `2026-08-23-metrics-points-identity` measured one `duckdb.append` ERROR on a `metrics_points` collision. So at one library version, `metrics_points` errored while `spans` hung — the sentence is accurate for its path and the proposal was false. Recorded in obs-plan §10 and `rules/observability.md` as **mechanism never established**, moot at 1.10505.
- **Stated rather than implied:** the per-table divergence above is a recorded observation, not an explanation — nobody established why. It is moot at 1.10505, where both paths return `Err`.
- **Surfaced, not actioned:** the disproved comment at `crates/buffer/src/schema.rs:320–323` still stands (wrap does not edit source) — CARRY'd onto the "Diagnostics un-muting + harness-truth sweep" entry. The spec that cited it (test-plan §4) WAS corrected this wrap.
- **Audit PREREQ (session 54):** between-point. Basis + overlap re-verified first-hand anyway — `cargo audit` still cannot load (`duplicate advisory ID: RUSTSEC-2026-0244`, cargo-audit 0.22.2); `deny advisories` exit 1 at 8 DISTINCT ids 0189/0190/0194/0195/0204/0222/0253/0258, set identical; `bans licenses sources` exit 0. Recorded `probe skipped per ratified interval (next: 55)`. **Session 55 owes the full-form probe.**
- Audit trail: `.andromeda/runs/2026-08-28T23-20-00Z-wrap/` (+ phase run dir `2026-08-28T21-25-02Z-phase/`).
- Last failed command: none.

## Deferred learnings
- None deferred by the cap — 2 Tier-2 entries + 1 Tier-3 against a cap of 3.
- Still open from prior wraps: the **deferral-destination generalization** (a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes).
- **CLOSED this wrap — `D-obs-defect-narrative` minted WITH the operator** (post-commit, in its own follow-up commit). The gap: obs-plan returned `proposals: []` while its own §10 defect 4 owed this chunk's amendment, because all three existing obs detectors bind to something NEW in the report's Changes and a falsified narrative adds nothing new. It bit at TWO consecutive chunks (mechanism corrected at `2026-08-28-ingest-consumer-block-under-gap-resume`, corrected again + closed here), and both times only the plan's Expected-amendments floor caught it. The new detector binds to the report's `Spec claims disproved by measurement` bullet + Outcome + changed-failure-mode Coverage rows, and treats §10's lead-in tally as a `dependent-of` occurrence. **Scoped to obs-plan only** — both measured occurrences are §10's defect list; extend to the other six masters when evidence appears, not speculatively.
