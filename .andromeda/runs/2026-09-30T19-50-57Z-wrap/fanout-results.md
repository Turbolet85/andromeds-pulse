# Fan-out results — 2026-09-30-perf-instruments-measure-their-budgets

Seven Explore doc-agents, one batch, prompts from `amendment-flow.md` sent verbatim (no `{contracts_line}`: every
`registry.py contracts` call returned `NOT MIGRATED` or `n/a`). Report: `chunks/2026-09-30-perf-instruments-measure-their-budgets/report.md`.
No return needed entity-decoding (no `&lt;` / `&gt;` / `&amp;` in any return) and every return parsed on first read; no raw twin warranted.

## Verdicts

| doc | proposals | stripped |
|---|---|---|
| architecture | 3 | trailing `# D-arch-decisions: no drift` comment (no dependency, locked TauRPC + serde-enum shapes followed) |
| security-plan | 2 | trailing comments: D-security-auth / D-security-deps no drift |
| design-system | 0 | comments: both detectors no drift; NEAR-MISS noted — design-system.md:265 Canvas Container names only `navigator.gpu` undefined as the fallback trigger |
| layout-templates | 0 | comments: both detectors no drift |
| test-plan | 3 | comments: D-tests-framework / D-tests-obs-harness no drift; routed §3 procedure-changing close + §9 release row to the orchestrator |
| obs-plan | 10 | notes: D-obs-stack / D-obs-pii no drift; §11 line 725 (hosted runner exposes no adapter) not proposed — not re-measured this chunk |
| a11y-plan | 0 | comments: both detectors no drift; routed a11y-plan.md:381 bare `--list` claim to the orchestrator |

## Proposals + dispositions

### architecture
- **A1** D-arch-resources · §Occupied Resources → Tauri IPC routes · add `telemetry.frontend.record_webgpu_adapter` (6th TelemetryApi method, roster 5 → 6, input shape, target `ui.webgpu.adapter`, exact leaf, dev-host frame leg as only live witness, no capability change, EXPECTED_PROCEDURES 43 → 44, «Да, делай») · basis architecture.md:178.
  → **apply** — check 1: Boundary widening (escalate) RESOLVED by the founder's P4 ratification «Да, делай» for this exact shape (scope.md §rulings), quoted in the sidecar; check 5 plan entry "arch IPC routes 5 → 6".
- **A2** D-arch-resources · §Occupied Resources → xtask CLI surfaces · retire the fixed `no WebGPU adapter in this run`; state the `frame_cause` derivation and the unrequired / required / `perf:frame-sample` line forms · basis architecture.md:243.
  → **apply** — check 1: Accurate this-chunk addition (routine); check 5 plan entry "xtask CLI surfaces".
- **A3** D-arch-resources · §Infrastructure Patterns → CI/CD · `release-{os}` saves on failure; the post-change cache re-read (10 605 172 169 B / 8, 1.23 %, watch) · basis architecture.md:321.
  → **apply** — check 1: Accurate this-chunk addition (routine); check 5 plan entry "CI/CD".

### security-plan
- **S1** D-security-input (escalate) · §Input Validation TauRPC row · record `record_webgpu_adapter`'s closed `WebgpuAdapterOutcome` + coerced `window_label` + pin 44, quoting «Да, делай» · basis security-plan.md:132.
  → **apply** — check 1: Boundary widening (escalate) RESOLVED by the P4 ratification (recorded, quoted in the sidecar); the boundary IS validated (report Outcome "(security) Validated input + pin — MET").
- **S2** D-security-logging · §Security Anti-Patterns → Logging · a sibling paragraph after `ui.ipc.rejection`: `ui.webgpu.adapter` deliberate no-scrub boundary, exact leaf, raw error text excluded, dev-host leg only live witness, «Да, делай» · basis security-plan.md:442. Restating sites :48 / :166 checked by the agent, unchanged.
  → **apply** — check 1: Boundary widening RESOLVED by the P4 ratification; check 5 plan entry "security §Logging".

### test-plan
- **T1** D-tests-coverage · §1 `ipc-rejection-wire-coverage` row · widen to the second fire-and-forget wire (`record_webgpu_adapter`), its only witness the dev-host frame leg · basis test-plan.md:142.
  → **apply** — check 1: Accurate this-chunk addition (routine) — the report measures the silent-no-op mode (Deviation 2) and the CI non-witness (disproved 1).
- **T2** D-tests-coverage · §1 `perf-slo-check-arm-coverage` row · pins 15 → 21, frame line cause-derived, snapshot times whole generation · basis test-plan.md:135.
  → **apply** — routine; check 5 plan entry "test-plan §1 row".
- **T3** D-tests-coverage (dependent-of T-primary) · §1 performance-budget frame row · retire the fixed string · basis test-plan.md:100.
  → **apply** — routine, rides with T2.

### obs-plan
- **O1** D-obs-defect-narrative · §10 snapshot row · timer scope → load + curate + format (GenerationTimer), p99 94.0 ms, owner discharged · basis obs-plan.md:628. → **apply** (routine: a measured narrative corrected; check 5 entry 1).
- **O2** dependent · §1 perf-budget-instruments snapshot row (:125). → **apply** with O1.
- **O3** dependent · §5 Snapshot generation row (:370). → **apply** with O1.
- **O4** dependent · §10 CI gates snapshot bullet (:642). → **apply** with O1.
- **O5** D-obs-defect-narrative · §10 frame row · `frame_cause`, the three cause texts, CI reads `no adapter record in this log`, the boot job cannot witness, the dev-host frame leg the only live witness, owner discharged (:629). → **apply** (routine; check 5 entry 2; check 6 disproved 1 — relay directive).
- **O6** dependent · §1 perf-budget-instruments frame row (:126). → **apply** with O5.
- **O7** dependent · §1 multi-platform-exporter-compat row (:132) — the adapter-state record now exists. → **apply** with O5.
- **O8** dependent · §10 CI gates perf-budget bullet (:641). → **apply** with O5.
- **O9** D-obs-instrumentation · §6 warn row · `ui.webgpu.adapter` (INFO obtained / WARN otherwise, once per mount), «Да, делай» (:418). → **apply** — check 1: Boundary widening RESOLVED by the P4 ratification + the new-logging-target rule's conditions (a) all fields in the leaf (b) guard under `pulse-app/tests/` both hold.
- **O10** D-obs-instrumentation · §8 exact leaf `ui.webgpu.adapter` (:532). → **apply** — same as O9 (dual-site).

## Orchestrator raises (Validate checks 5 + 6)

- **R1** check 5 — test-plan §3 Per-chunk gate discipline: the close for a chunk that CHANGES the procedure set (not proposed by D-tests-*). The playbook bindings rule (line 118) says "escalate that case until one is [specified]" → **ESCALATE** (with the playbook rule's own extension).
- **R2** check 5 — test-plan §9 release row: `release-${{ runner.os }}` now saves on failure (`cache-on-failure: true`) → **apply** routine (report Schema / config substantiates).
- **R3** check 6 (disproved 2) — a11y-plan §3 "Adding a surface" bullet (:381): the suite-health probe is the config-named `npx playwright test --config=playwright-a11y.config.ts --list`; the bare form reads the inert default config (0 tests, exit 1) → **apply** routine (playbook: accurately correcting a doc claim a pre-existing reality falsifies; test-plan.md:56 already states the correct form).
- **R4** design-system near-miss — §Surface: desktop-webview → Component Patterns (:265): the fallback renders on ANY `unavailable` adapter result (`navigator.gpu` undefined · a null adapter · a rejected adapter request — this chunk · a failed device request) → **apply** routine (report Symbols: "The existing consumers render the shared `<Fallback />` on any `unavailable`"; pre-existing narrowness plus this chunk's new reason).
- check 6 disproved 1 (CI boot job witnesses the adapter record) → DISPOSED: chunk-artifact claims (research / plan / scope, closed) recorded in the report; the master-side truth lands via O5 / O7 / S2 / A1 naming the dev-host frame leg as the only live witness.
- check 3 intent-consistency: scope record empty (`gate.py scope` clean); no divergence from the working-route entry or the plan's acceptance criteria beyond Deviations 1-3, each justified.
- check 4 absence: A1's "0 hits for record_webgpu_adapter / ui.webgpu in architecture.md" — probe re-run by the orchestrator below the cascade (the sweep's own pattern).
- check 2 cross-contradiction: none (no two proposals edit one section in opposing directions).

## Escalations resolved (operator, 2026-09-30, this wrap)
- **R1** → "Extend the rule in place" — overseer: "yes, one rule sharpened in place replaces its escalate clause; a sibling would leave two rules on one subject." test-plan §3 + playbook line-118 rule note amended with the specified close.
- **Boundary widening (A1 · S1 · S2 · O9/O10)** → "P4 ratification resolves" — overseer: "the founder ratified this exact shape live at P4 («Да, делай»); quote it with its date in each of the four sidecar entries."
