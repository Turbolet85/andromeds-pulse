# Session Handoff

**Last Updated:** 2026-08-26T13:25:00Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 29 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-26-cadence-runaway-blocking-pool): the amplifier gets a bound, and the loop stops turning one fault into a flood`

## Position
- Done: **2026-08-26-cadence-runaway-blocking-pool** — the cue→cadence amplifier is bounded at emission, and the two diagnostics that made the wedge unreadable are un-muted. The INITIATING freeze is **not** fixed and never claimed to be.
- Next (first markerless): **L4 runtime security residuals** — it carries the re-pinned `cargo audit` PREREQ, **next interval point 49** (points 46 discharged here, 47–48 skip per the ratified interval).
- Then: Interpretation brief completeness · **Ingest consumer initiating freeze** (new, position 3) · Halo State Pulse canvas disposition · Advisory backlog · npm advisory coverage · Diagnostics un-muting · Staged-bindings assertion · Metrics label surface.

## Work done
**The bound.** `CueLatch` keys on `(kind, scope_id)` — the tuple incidents already coalesce on per arch §Fault Identity — so what it refuses is what the registry would have merged downstream anyway; the dedup moves ahead of the expensive work instead of after it. It admits on a NEW condition, on tier ESCALATION (so a worsening fault is never delayed), or after a 60s refractory, and evicts cleared conditions so a genuine recurrence admits immediately.

**Step 1 was the chunk's pivot.** The plan warned that a cue-side-only bound would miss most triggers, and it was right: `cadence.trigger` 2,683 = **tier1 2,065 · tier2 606 · tier3 12**. The third driver is the coordinator's **Tier-1 Autonomous arm**, a separate subscription to the general cue broadcast — so a bound at `emit_cue`'s Suggested branch (where research pointed) would have covered **23%**. Placing it upstream of *emission* covers both arms with one mechanism; `coordinator.rs` needed no bound. `cadence.tick`=51 was never a cycle at all — it is the 15s heartbeat, so the scope's `606+51` arithmetic compared unlike things.

**Measured at the wire**, not just in unit tests: 887 cues refused vs 22 admitted; 176 ticks at `cues_latched:5` (the wedge's exact shape); **24** cadence triggers and **240** L1a queries against the wedge's 2,683 / 26,821. The 240/24 = 10.0 per-trigger ratio is unchanged, confirming trigger *frequency* was the defect, not per-cycle cost.

**Gates:** fmt · clippy all-features 0 warnings · **nextest 1963/1963 + 1 skip** · capability-widening clean · check:ingest-progress PASS · capability-drift clean (after the documented regen) · deny bans/licenses/sources exit 0 · advisories designed-red. **Three mutation checks, all discriminating** — neutralising the bound reddened 7 pins (all five refuse-arms + both re-pointed fan-out tests) while the three admit-arms stayed green, which is the conditional-property asymmetry.

**Smoke:** Direct-binary, fresh data dir, release binary rebuilt. 9,720 spans / 425 exceptions, 0 ERROR / 0 panic across 123,706 lines, ports released, 0 orphans.

## Drift resolved
**9 amendments across 3 masters · 2 escalations resolved · 6 leaf edits across 4 files · cascade closed · drift = 0.**
- **obs-plan ×5** — the `triage.cue.tick` leaf (5→9 fields) and a new `cadence.tick` leaf (8), two §5 tick-fold rows, the backlog FIVE→FOUR, and the bare-`interpretation` invariant breach recorded as measured.
- **test-plan ×3** — two coverage triggers minted (`cadence-cycle-rate-in-crate-coverage`, `cue-latch-emitter-wiring-coverage`) and §3's direct-binary smoke broadened to either profile.
- **security-plan ×1** — audit interval 46 discharged, re-pinned to 49.
- **Cascade:** `rules/observability.md` ×2, `docs/obs-summary.md` ×2, `docs/services/interpretation.md`, `rules/security.md`. Verified-and-correctly-skipped: `docs/tests-summary.md`, `rules/verification-harness.md`, `docs/security-summary.md`, CLAUDE.md, playbook, drift-base. `rules/testing.md`'s hit sat in preserve-verbatim Session Additions **and was not stale** — routed to P3, not cascade.

## Notes
- **The initiating freeze is un-fixed and now owns route entry #3** (operator-placed): consumer stopped at **17:20:50 under ~1.7 blocking queries/sec**, cause unidentified. The amplifier bound makes it survivable; the stall detector makes the next occurrence self-documenting. **Durable evidence: `D:/dev/evidence/pulse-l4run-171923/logs/agent-latest.jsonl.2026-08-25`** (131,324,444 bytes; signatures 2,683 / 4,599 / 26,821, verified first-hand this wrap). The temp scratchpad copy is NOT durable — cite the durable path only.
- **A documented invariant is violated in code:** a bare `interpretation` allowlist key exists at `pulse-app/src/observability.rs:1934` while obs-plan §8 and `rules/observability.md` both state one may not. The docs are right; the code is out of compliance. Latent (every live `interpretation.*` target has its own exact leaf). CARRY'd to "Diagnostics un-muting + harness-truth sweep" with two siblings — the corrected backlog mechanism and `triage.cue.suppression_check`'s per-cue hot-path emission.
- **Bookkeeping correction:** this chunk's +13 test delta (grep-verified per file) puts the total at 1963. **The prior session's recorded baseline of 1949 was off by one** — actual at that HEAD was 1950. Stated, not silently absorbed.
- **Residual the bound does NOT cover:** condition *cardinality*. The latch limits repetition per condition, not the number of distinct conditions; N simultaneously-silent services still admit N cues per window. `latch_tracked` is the field that makes it visible. A hard per-tick ceiling was considered and rejected — the acceptance requires triggers "proportionate to real cue volume", which a ceiling violates by construction.
- **Curation:** T1 0 · T2 3 · T3 0, all in-place extensions of matched entries (Filter 1 additive-facet), 1 deferred by the cap. CLAUDE.md unchanged at **156/200**.
- Audit trail: `.andromeda/runs/2026-08-26T12-15-00Z-wrap/` (+ the phase run dir).
- Last failed command: none.

## Deferred learnings
- **`vacuous-guard-behind-a-conditional`** (Filter-5 cap) — a plan can name an existing test as "the regression guard" when its whole assertion sits behind an `if` that never fires. `run_one_emit_cycle_tier_two_fans_to_cadence_triggers` seeded 4 errors FIRST then 66 clean spans — a recovery, not a spike — so short/long EWMA fell below 1.0, zero cues were produced, and it had been passing vacuously since chunk #62. Sibling of the existing empty-collection and stub-payload vacuity entries; the new facet is the conditional wrapper. Repaired in-chunk by strengthening the fixture (probed: 50 clean + 100 error observations lands in the Suggested band), never by relaxing the assertion.
- **`derived-copy-instead-of-the-file`** — surfaced THREE times in one session, three different mechanisms: an Edit built from the context-loaded copy of `rules/security.md` failed on mismatch; a curation candidate was tier-misrouted because the session-start context renders CLAUDE.md and each rules file adjacently, so an entry read there carries no reliable file attribution; and the allowlist enumeration produced two false answers from a wrong grep basis. The third was curated (rules/observability.md); the first two are the same family aimed at files rather than greps. Remedy is a check in the owning step, not a fourth restatement.
- Still open from prior wraps: the **deferral-destination generalization** (a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes — a destination can complete WITHOUT absorbing the deferred evidence, leaving a hollow `verified`).
- Standing: the **bindings clobber** recurred again (capability-drift red → documented regen → clean). The playbook's 2026-08-22 ordering rule handled it correctly; the mechanical fix is owned by the "Staged-bindings assertion" entry.
