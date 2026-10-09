# Report — 2026-08-26-cadence-runaway-blocking-pool

**Chunk:** Cadence runaway starves the blocking pool — the cue→cadence→digest loop gets a stated ceiling, and the buffer keeps draining by repair rather than by measurement
**Date:** 2026-08-26
**Commits:** (none yet — this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/triage/src/cue/emitter.rs` — the bound + its wiring + tests
  - `crates/triage/src/cue/mod.rs` — re-export
  - `crates/triage/src/contract.rs` — re-export
  - `crates/triage/src/cadence/coordinator.rs` — cycle-rate field on the existing heartbeat
  - `pulse-app/src/observability.rs` — two allowlist leaves
  - `pulse-app/tests/unit_observability_allowlist_cue_tick.rs` — **new**, 5 guards

- **Symbols / APIs:**
  - **New public (crate `triage`):** `CueLatch` (struct; `new`, `with_refractory_nanos`, `admit_cycle`) · `LatchOutcome` (struct: `admitted` / `latched` / `tracked`) · `CUE_LATCH_REFRACTORY_NANOS` (`i64`, 60 s in nanos). Private: `LatchEntry`, `tier_rank`, `latch_key`.
  - **Changed public signature:** `run_one_emit_cycle` gains a 6th parameter `latch: &CueLatch` (before `now_nanos`). **Remaining-caller fact:** its sole NON-test caller is `start_emitter` in the same file, which now constructs the latch internally; 14 in-crate test call sites were threaded. It is re-exported by `triage::cue` and `triage::contract`, so the new types are re-exported alongside it to keep those paths usable. **`pulse-app/src/main.rs` was deliberately NOT touched** — scoping the latch inside `start_emitter` kept the boot wiring unchanged.
  - **Changed public struct:** `EmitCycleStats` gains `cues_latched: usize` + `latch_tracked: usize` (7 → 9 fields).
  - **No IPC/TauRPC procedure added or changed** — `pulse-app/ui/src/bindings/index.ts` is content-identical to HEAD; `capability-drift` clean.
  - **No endpoints, ports, sockets, or env vars added.** (`ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` was *used* by the smoke; it is pre-existing and registered.)

- **Crates / modules:** none added, none removed. Changed: `triage::cue`, `triage::cadence`, `pulse-app::observability`.

- **Dependencies:** **none added, none bumped.** `dashmap` was already a `triage` dependency (used by `pattern::SuppressionState`); no `Cargo.toml` or `Cargo.lock` was touched.

- **Schema / config:** none. No migration, no config key, no `Settings` field, no violation-schema change. The refractory interval is a module `pub const`, deliberately not a config surface.

- **Spec-master edits:** none at report time (P2 applies them).

- **Counts / qualifiers moved:**
  - `triage.cue.tick` **emit-site field count 7 → 9**; its allowlist leaf **5 → 9**. Stated in obs-plan §8 (the muted-diagnostic backlog) and `.claude/rules/observability.md`.
  - `cadence.tick` **emit-site field count 7 → 8** (`cycles_executed` added); its allowlist leaf 7 → 8. **`cadence.tick` is not enumerated anywhere in obs-plan** (0 hits) — a pre-existing doc gap this chunk widens.
  - obs-plan §8's muted-diagnostic backlog: **FIVE targets → FOUR** (`triage.cue.tick` leaves it). The same five-target list is restated in `.claude/rules/observability.md`.
  - Workspace test count **1950 → 1963** (+13).

- **Dev-tool versions:** none. (`cargo-audit` 0.22.2 was *probed*, not installed or upgraded.)

- **Reverted / negative API facts:** A per-tick **fan-out ceiling** was considered alongside the latch and **deliberately not shipped**: the acceptance requires `cadence.trigger` to stay "proportionate to real cue volume", and a hard ceiling makes triggers *dis*proportionate by construction. The residual it would have addressed (condition *cardinality*, as opposed to repetition) is stated under Outcome and made observable via `latch_tracked`.

- **Spec claims disproved by measurement:**
  1. **obs-plan §8:540 (SPEC-MASTER)** — states five targets "carry fields that **resolve to no allowlist entry**", naming `triage.cue.tick → bypass_triggered, cues_suppressed`. **Measured false in mechanism:** `triage.cue.tick` *did* resolve, to an exact leaf at `pulse-app/src/observability.rs:1306` that named 5 of the 7 fields the site emitted. The target was **partly** redacted, not unresolved. This matters because the stated mechanism implies the wrong repair ("add a leaf" vs "complete the leaf"), and it casts doubt on the same census's mechanism claim for the remaining four targets. Evidence: mutation C — with the pre-fix 5-field leaf, `cue_tick_resolves_to_an_exact_leaf` **passes** while the field-set pin fails. **Disposition: amendment owed (P2).**
  2. **`.claude/rules/observability.md` (LEAF)** — restates the same five-target list and the same "resolve to no entry" mechanism. **Disposition: cascade (P2).**
  3. **`plan.md` step 7 (CHUNK ARTIFACT)** — "`triage.cue.tick` currently resolves to nothing". Same measurement as (1). **Disposition: recorded here, no amendment owed** — a chunk artifact has no sanctioned writer at wrap.
  4. **`plan.md` step 6 (CHUNK ARTIFACT)** — asserts both named tests "PIN the current 1:1 semantics". **One pinned nothing:** `run_one_emit_cycle_tier_two_fans_to_cadence_triggers` seeded 4 errors *first* then 66 clean spans — a recovery, not a spike — so short/long EWMA fell below 1.0, zero cues were produced, and its entire assertion sat behind an `if` that never ran. **Disposition: recorded here, no amendment owed**; the fixture was repaired in-chunk (strengthened, not relaxed).
  5. **`scope.md` premise-closure item 5 (CHUNK ARTIFACT)** — "`cadence/` may need no change at all." Held for the *bound* (no bound landed there) but not for instrumentation: `cadence.tick` gained the cycle-rate field. **Disposition: recorded here, no amendment owed.**

- **Coverage of new surfaces:**
  - `triage::cue::CueLatch` (in-process bound; no external surface) → validation n/a (no external input) · instrumentation ✓ (`cues_latched` + `latch_tracked` folded onto the existing `triage.cue.tick` heartbeat — **zero new tracing call sites**, verified by delta) · PII redacted✓ (aggregate `usize` counts only; the latch's map key holds `scope_id` in memory and is **never logged** — the pre-existing `run_one_emit_cycle_pii_canary_does_not_leak_into_tracing_fields` guard scans all captured fields and passes) · tests unit✓ (8 in-crate, clock injected as explicit `now_nanos`) + e2e✓ (live smoke) · a11y n/a · tokens n/a
  - `cadence.tick.cycles_executed` (obs field) → validation n/a · instrumentation ✓ (tick-aggregated; the value was already counted and discarded) · PII redacted✓ (bounded `u64` counter) · tests unit✓ (`cadence_tick_carries_the_cycle_rate`, live under `pulse-app/tests/`) · a11y n/a · tokens n/a
  - `triage.cue.tick` allowlist leaf (log boundary) → validation n/a · instrumentation ✓ · PII redacted✓ (all nine fields are aggregate numerics; no `scope_id`, no service name, no query text) · tests unit✓ (5 guards incl. a fallback discriminator, all three mutation-checked) · a11y n/a · tokens n/a

## Deviations from intent

1. **Implementation steps 2 and 4/5 executed in reverse order.** Step 2 (complete the leaf) enumerates fields the step-5 instrumentation creates; doing 2 first would have hard-coded a stale list. The leaf was enumerated from the final emit site, per the emit-site-is-authority rule.
2. **Bound placed at cue emission, not at either named site.** Research pointed at `emitter.rs:189-192` (the Suggested→cadence fan-out) and the plan allowed `coordinator.rs`. Step 1 measured that the Suggested branch carries only **606 of 2,683** triggers (23 %), so a bound there would have missed the majority. The latch sits upstream of *emission*, which feeds both cadence arms, so one mechanism covers both and `coordinator.rs` needed no bound.
3. **`cadence.tick` gained `cycles_executed`** — `coordinator.rs` was listed only conditionally. The condition was met (step 1 found the third driver enters there) and step 5 requires a cadence *cycle rate*, which was computed into `cumulative_cycles` and then discarded via `let _ =`. Without it the rate is readable only by counting per-trigger records out of a 131 MB log — the forensic read this chunk exists to remove.
4. **Boot-smoke command substituted.** Plan: `inject_demo --sustained --minutes=15`. Ran: `--sustained --minutes=3`, then the producer **stopped** for 210 s, with `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS=60`. Justification: under a *constant* error rate the short/long EWMA converge and the spike ratio falls below the 3.0 gate, so no cue repeats — the planned form leaves `cues_latched` at 0 and proves plumbing only (confirmed live: Phase-A ticks read `cues_emitted 0 / cues_latched 0`). Phase A still satisfies "under sustained load" and `check:ingest-progress`; Phase B reproduces the wedge's exact cue shape so the bound is exercised.
5. **Deterministic L4** (`ANDROMEDA_PULSE_L4_DETERMINISTIC=true`) in the smoke. The bound is entirely upstream of L4; this removes the ~4.3 s inference confound. Stated rather than implied.
6. **`crates/triage/src/cue/mod.rs` edited**, not named in research's file list. It is the re-export chain for a listed file's new public types; `contract.rs` *was* anticipated ("caller threading").
7. **A test fixture repaired beyond "re-point".** See Spec claims (4).

## Decisions & corrections

- **The latch key is `(kind, scope_id)` by design, not convenience** — the same tuple incidents already coalesce on per architecture.md §Established Decisions [Fault Identity]. So what the latch refuses is what the incident registry would have merged downstream anyway; the dedup moves *ahead* of the expensive work instead of after it. This is the justification for applying the latch uniformly across cue kinds rather than only to the silence family.
- **Escalation must never be delayed** — the bound admits immediately on a tier increase (`Curious → Suggested → Autonomous`), so a worsening condition reaches the coordinator on the tick it worsens. This is what keeps the scope boundary "must not flatten accelerated cadence into baseline" intact, and it is pinned by a test.
- **Refractory = 60 s, matched to the tier-3 cadence baseline** so a latched condition can never go quieter than the ticker that would run anyway.
- **A hard per-tick ceiling was rejected** (see Reverted / negative API facts).
- **Operator ruling carried in:** the durable evidence path for the predecessor's wedge log is `D:/dev/evidence/pulse-l4run-171923/logs/agent-latest.jsonl.2026-08-25` — verified first-hand at this wrap (131,324,444 bytes; signature counts 2,683 / 4,599 / 26,821 match). The temp scratchpad copy is not durable and must not be cited.
- **Bookkeeping correction:** this chunk's +13 test delta (grep-verified per file: `emitter.rs` 15→23, new file 5, no other file moved) puts the total at 1963. The **prior session's recorded baseline of 1949 was off by one** — actual at that HEAD was 1950. Stated rather than silently absorbed.

## Outcome

**Acceptance criteria — met.**

- *(outcome)* Cue emissions stay bounded; the flat per-service-per-tick pattern does not recur. Measured live: **887 cues refused vs 22 admitted**, 176 ticks sat at `cues_latched:5` (the wedge's shape), and `cadence.trigger` totalled **24** against the wedge's 2,683, with **240** L1a queries against 26,821. The 240/24 = 10.0 per-trigger ratio is unchanged, confirming trigger *frequency* — not per-cycle cost — was the defect.
- *(outcome)* The third cadence entry point is **NAMED**: the coordinator's **Tier-1 Autonomous arm**, a separate subscription to the general cue broadcast. Wedge breakdown tier1 **2,065** · tier2 606 · tier3 12 = 2,683 exactly (`cadence.tick`=51 was never a cycle — it is the 15 s heartbeat, so the scope's `606+51` arithmetic compared unlike things). **The shipped bound covers it**, because both arms are fed by emission.
- *(outcome, stated limitation)* **This chunk does not prevent the initiating freeze.** What stopped the consumer at **17:20:50 under ~1.7 blocking queries/sec is unidentified**; the amplifier bound makes it survivable and the stall detector makes the next occurrence self-documenting. It is proposed as its own route entry at P5.
- *(obs)* `triage.cue.tick` resolves to an exact leaf carrying every emitted field; `cues_suppressed` and `bypass_triggered` verified **unredacted on a live run** (`"cues_suppressed":0,"bypass_triggered":0` as integers, where the wedge log shows `"<redacted>"`). No bare `triage` or `triage.cue` prefix key exists — pinned by a discriminator test.
- *(obs)* Every new signal is a **field folded onto an existing heartbeat** — **zero new tracing call sites** (delta 0 in both touched files). No per-trigger record.
- *(obs)* New fields are aggregate numerics only; no `scope_id`, service name, query text or parameters.
- *(arch)* Changes land only in `crates/triage` + `pulse-app`; no new crate, no library→binary edge, no second runtime, no channel-substrate swap.
- *(arch)* The remedy is a stated ceiling, not a larger pool — `max_blocking_threads` untouched.
- *(tests)* The bound carries direct in-crate tests with the instant injected as an explicit parameter.
- *(tests)* **Three mutation checks, all discriminating.** Neutralising the bound reddened **7** pins — all five refuse-arms plus **both** re-pointed fan-out tests — while the three admit-arms stayed green (the conditional-property asymmetry: only the negative half can carry the guard). Deleting the leaf reddened 3 of 4 (the discriminator correctly stayed green). The narrowed-leaf mutation reddened the field-set and un-mute pins while the resolver pin passed — proving a resolver-only guard would have passed vacuously at HEAD.
- *(security)* `cargo deny check bans licenses sources` **exit 0**; `advisories` observed separately as designed-red. No new dependency.
- *(security)* **`cargo audit` interval point 46 DISCHARGED in full form** — probe RAN; true exit **1** read directly (no pipe); basis byte-identical (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`, DB still unloadable) under cargo-audit 0.22.2; owned set re-enumerated as **distinct ids: 8 — 0189/0190/0194/0195/0204/0222/0253/0258, unchanged** — against **10 error blocks** (quick-xml at two lockfile versions), exactly the miscount the discipline warns about. **Next interval point: 49.**
- *(security)* Receiver-side load bounds unchanged.
- *(a11y)* **The chunk changes no rendered DOM** — zero `pulse-app/ui/**` paths. The FindingsCounter polite 0→N announcement and P4 focus stability are unaffected; the a11y chain is recorded as untouched rather than silently skipped.
- *(design / layouts)* No domain coverage — no rendered surface created or modified.

**Gates green:** `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` (0 warnings) · `cargo nextest run --workspace --profile ci` (**1963 passed, 1 skipped**) · `cargo xtask capability-widening-check` (clean, 0/3) · `cargo xtask check:ingest-progress` (PASS) · `cargo xtask capability-drift` (clean, after the documented bindings regen) · `cargo deny check bans licenses sources` (ok) · `cargo deny check advisories` (designed-red) · `cargo audit` (PREREQ probe).

**Smoke:** Direct-binary variant, fresh data dir, release binary rebuilt to include this chunk. Boot in 2 s; 9,720 spans / 425 exceptions injected; **0 ERROR, 0 panic** across 123,706 lines; all seven heartbeats present; shutdown by pidfile PID with `:4317`/`:4318` released and **0 orphan processes**. Incidental confirmation of the predecessor's work: `check:ingest-progress` PASS reporting "longest zero-delta run 13" with **zero** `buffer.consumer.stalled` — the silence phase's non-draining ticks were correctly classified **ProducerIdle**, not a stall.

**Residuals (carried, not fixed):**
1. **The initiating freeze** — see the stated limitation above; proposed as a route entry.
2. **Condition cardinality is not bounded.** The latch limits *repetition per condition*, not the number of distinct conditions: N simultaneously-silent services still admit N cues per refractory window. `latch_tracked` is the field that makes this visible.
3. **`triage.cue.suppression_check` still emits one record per raw cue per tick** (4,599 in the wedge log) — an obs-plan §11 hot-path concern found while reading the emit path, left untouched because the plan bans absorbing the muted-diagnostics sweep. Belongs to "Diagnostics un-muting + harness-truth sweep".
4. **`cadence.tick` is enumerated in no spec master** — a pre-existing doc gap this chunk widens by one field.
