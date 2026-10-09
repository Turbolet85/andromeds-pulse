# Fan-out results — 2026-09-29-p-025-hue-shift-observable-made-gradable (wrap P2, resumed 2026-09-29)

Detectors run: 7 Explore agents, one per master, prompt per amendment-flow.md. No master is U35-migrated
(`registry.py contracts` → `NOT MIGRATED` ×4), so `{contracts_line}` was dropped.

## Verdicts
- architecture — 9 proposals
- security-plan — 2 proposals (both escalate severity); trailing commentary stripped (the sweep notes: the "cannot load" claim occurs only at :219; GHSA-ggr8 only at :221)
- design-system — `proposals: []`; commentary stripped → `.raw-fanout-design-system.md`
- layout-templates — `proposals: []`; commentary stripped → `.raw-fanout-layout-templates.md`
- test-plan — 14 proposals
- obs-plan — 6 proposals; trailing commentary stripped (D-obs-stack / D-obs-pii / new-hot-path: no drift)
- a11y-plan — `proposals: []` (bare, no stripping)

Entity probe: no `&lt;` `&gt;` `&amp;` in any return (entities=0).

## Proposals + dispositions

### architecture
A1 D-arch-decisions · §Stack Plugin runtime row (:22) — wasmtime "46"/46.0.3 → "48.0.3"/48.0.3, Cranelift 0.135.3.
  → REJECT as proposed (re-derivation tell: `basis` cites `Cargo.lock:1393/:8543`, which the report does not carry) → the
  wasmtime retirement RAISED by the orchestrator under check 5 (report §Expected amendments, "Beyond the plan's list":
  `wasmtime 46|"46"`: architecture 3), text re-derived from report §Dependencies; the Cranelift figure read by the
  orchestrator at `Cargo.lock:1393` (0.135.3) and written `as measured at` that coordinate — APPLY (orchestrator-raised).
A2 dependent-of A1 · §Established Decisions [Plugin Runtime] (:52) → APPLY with A1 (orchestrator-raised group).
A3 dependent-of A1 · §Inherited Defaults Plugin runtime (:359) → APPLY with A1 (orchestrator-raised group).
A4 D-arch-resources · §Occupied Resources xtask CLI surfaces — register `smoke:hue-shift` → APPLY. Playbook: Accurate
  this-chunk addition. Registry over-reach's precondition fails — this registry DOES enumerate formalized xtask CLI
  contracts (the 2026-08-25 formalized-CLI-contract rule; harness:status / check:npm-supply-chain / check:staged-artifacts
  registered), and smoke:hue-shift carries an exit contract. Plan expected-amendment. (Pre-existing gap noted, not this
  chunk's: `smoke:external-resolve` was never registered → handoff note.)
A5 D-arch-resources · agent-run entry — `ci.yml:12` workflow-level env → per-job `$GITHUB_ENV` export, xvfb-run, boot
  failure diagnosis + cleanup → APPLY (Accurate this-chunk addition; plan expected-amendment).
A6 D-arch-resources · harness:status entry — pid liveness → APPLY (Accurate this-chunk addition).
A7 dependent-of A6 · §Occupied Resources Filesystem locations pidfile (:214) → APPLY with A6.
A8 D-arch-resources · Tauri IPC `services.list_with_states` — register the `tier_effective_at_unix_nano` payload field
  and the snapshot read → REJECT. Playbook: Registry over-reach (a payload FIELD inside an already-registered procedure;
  the Tauri IPC route list tracks procedures — chunk #91's `priority_tier` join was never registered here either). The
  entry's standing claim ("returning `ServiceListPayload` from `InMemoryServiceRegistry::list`") is still true.
A9 D-arch-resources · Tauri IPC `record_constellation_hue_latency` (:176) — P-025 interval → APPLY (plan expected-amendment).

### security-plan
S1 D-security-deps (escalate) · §Dependency Security → CI integration standing-deferral bullet (:219) — deferral ENDED,
  pin #22 discharged, history retired, CARGO_HOME note → APPLY. Playbook: escalate-severity-outside-its-class (2026-08-23):
  (a) the escalate condition is affirmatively absent — no banned/unvetted dependency (every bump is an existing dependency;
  the one new dev dep `@types/node` passes `check:npm-supply-chain`, `cargo deny check bans licenses sources` green);
  (b) the actual class is a measured-disproved doc claim with a correct impl → routine APPLY (2026-08-14). The bullet's own
  terminating clause ("ends the first time `cargo audit` loads") fired; the operator relay (part 1) directs the retirement.
S2 D-security-deps (escalate) · §Dependency Security → npm channel (:221) — GHSA-ggr8 pruned, GHSA-7pqw accepted → APPLY.
  Same two-condition rule; GHSA-7pqw has no fixed release (the no-safe-upgrade class the exception form exists for), and
  the operator relay names it "the one accepted residual".

### test-plan
T1 D-tests-framework · §4 Framework vitest 3.x → 4.x → APPLY (Accurate this-chunk addition). The `restoreAllMocks`
  hazard in the change line is collateral to the framework invariant → not written to the body; routed to P3 curation.
T2 dependent-of T1 · §1 surfaces row driver → APPLY with T1.
T3 D-tests-framework · §9 A11y suite row — Playwright install + "runs in CI today" measured → APPLY (plan expected-amendment;
  `runs in CI today` has exactly 1 test-plan hit, this row).
T4 D-tests-obs-harness · §3 status — pid liveness → APPLY (Accurate this-chunk addition; obs-plan §3 does not describe the
  harness:status verdict, so the bind stays consistent).
T5 dependent-of T4 · §1 Test harness requirements (:66) → APPLY with T4.
T6 dependent-of T4 · §3 PID file lifecycle (:267) → APPLY with T4.
T7 D-tests-obs-harness · §3 boot failure path → APPLY (Accurate this-chunk addition).
T8 D-tests-obs-harness · §9 Boot smoke row — ci.yml:12 → per-job export + xvfb-run → APPLY (plan expected-amendment).
T9 D-tests-obs-harness · §3 scenario legs — register smoke:hue-shift → APPLY (plan expected-amendment). The RED/GREEN
  readings stay in the evidence files (collateral readings not written as body fact beyond the contract).
T10 D-tests-coverage · §10 cumulative coverage — TEMPORARY xtask exclusion → APPLY (founder ruling, recorded verbatim in
  the report; the review point carried).
T11 dependent-of T10 · §9 Coverage row → APPLY with T10.
T12 dependent-of T10 · §4 Coverage target → APPLY with T10.
T13 D-tests-coverage · §1 Pending coverage triggers — widen `harness-cleanup-verdict-and-boot-spawn-shell-coverage` to
  the boot failure-diagnosis branch → APPLY. Absence evidenced: `grep -rln 'agent-run' xtask/src pulse-app/tests crates`
  (--include=*.rs) → 0 test hits; the report's Coverage of new surfaces lists no test for the branch.
T14 D-tests-coverage · §1 Pending coverage triggers — new row for perf-slo-check's NEUTRAL arm → APPLY. Absence evidenced:
  `grep -rln perf-slo-check` over *.rs/*.sh/*.ts → the script itself, its xtask call site (`xtask/src/main.rs:429–629`),
  a doc comment (`pulse-app/tests/perf_slo_10k_spans.rs:17`), `load_profiles.rs` — none asserts an arm.

### obs-plan
O1 D-obs-instrumentation · §8 DELEGATED TIMING leaf `hue_update_ms` interval (:542) → APPLY (plan expected-amendment;
  metric-fallback clause and dot-hue clause kept).
O2 D-obs-defect-narrative · §10 CI gates perf-budget p99 bullet (:641) — measured vacuous, owner named → APPLY. Playbook:
  2026-08-28 recording-a-measured-OPEN-defect rule (the amendment records it open with a named owner — the working-route
  entry "Perf-budget gate reads real samples" minted at this wrap's P5 per the operator relay's part 2 order).
O3 dependent-of O2 · §10 snapshot p99 CI gate (:642) → APPLY with O2.
O4 dependent-of O2 · §10 WebGPU frame budget row (:629) → APPLY with O2.
O5 dependent-of O2 · §10 buffer memory row (:630) → APPLY with O2.
O6 dependent-of O2 · §1 perf-budget trigger row (:126) → APPLY with O2.

## Validate — the six checks
1. Playbook — per proposal above.
2. Cross-contradiction — none: arch A6/A7 and test-plan T4–T6 state the same liveness rule; arch A5 and test-plan T8 the
   same export; A4 and T9 the same leg contract.
3. Intent-consistency — scope record: 17 lines, all `widening`, each carrying an operator/founder word (justified branch).
   The plan acceptance "no `Cargo.toml` changes" is literally broken (dev profile, wasmtime 48.0.3); its widening lines
   (`scope-record.md:1–2`) carry the founder's and the overseer's words, and the operator relay (part 1) names the broken
   clause for the report → justified, intent was incomplete; no escalation.
4. Absence needs evidence — T13/T14 searches above; A8's "never registered" read from arch :182 (no `priority_tier` in the
   entry). Long lines resolved by offset reads (arch :214 2608c, :241 4843c; security :219 8756c).
5. Expected amendments — every plan entry is matched: obs §8 (O1) · arch hue (A9) · arch smoke + ci.yml:12 (A4/A5) ·
   test-plan §9 boot (T8) · test-plan §3 scenario (T9) · §9 runs-in-CI (T3). Beyond the plan: §10 coverage (T10–T12) ·
   security pin #22 + npm (S1/S2) · wasmtime (A1–A3, orchestrator-raised) · harness:status (A6/A7, T4–T6).
6. Disproved claims —
   - Conductor fall source / ack; job-level env; U05 reflow → premise-corrected in `scope.md` at /phase (owned there).
   - Plan gate `cargo audit` expects exit 1 → S1 + the pin retirement at P5; the gate itself is P7.1's to treat.
   - Plan gate advisory-db fetch reads `$HOME/.cargo` → a chunk-artifact claim: the report's record + S1's CARGO_HOME note +
     P3 curation (the pin that owned the probe retires with S1).
   - agent-run.sh 100644 / harness:status 60 s → fixed in-chunk; A5–A7, T4–T7.
   - "No Cargo.toml changes" → check 3 (justified widening).
   - Research "checks 0/0" signature → a chunk-artifact claim: recorded in the report, no amendment owed.
   - security-plan standing-deferral text → S1.
   - CI perf-budget gate vacuous → O2–O6 + T14 + the P5 route entry.

Escalations: 0.
