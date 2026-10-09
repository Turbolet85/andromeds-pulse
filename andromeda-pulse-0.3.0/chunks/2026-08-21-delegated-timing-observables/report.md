# Report — 2026-08-21-delegated-timing-observables

**Chunk:** Delegated timing observables — the three delegated timing bounds (P-025 halo hue · P-027 constellation discovery · P-045 counter refresh) become measurable at the wire with real values, so Conductor can grade them
**Date:** 2026-08-21
**Commits:** (none yet — this wrap's commit is the chunk's first; the preceding `edb949f chore(route): operator-requested adaptation — 0-pending wrap` minted this chunk's route entry)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/ui-bridge/src/telemetry.rs` (+383) — three procedures, three input structs, one bounded enum, one validator, 9 colocated tests
  - `pulse-app/src/observability.rs` (+20) — three exact allowlist leaves
  - `xtask/src/main.rs` (+3) — three `EXPECTED_PROCEDURES` entries
  - `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs` (NEW, 3 tests) — allowlist field-completeness + fallback-discrimination + PII guard
  - `pulse-app/ui/src/canvas/frame-metrics.ts` (+72) — three bridge functions + `clampDiscoveredCount` + a shared `invokeTelemetry` helper
  - `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (+82/−…) — the P-027 discovery mark and the P-025 hue mark
  - `pulse-app/ui/src/hooks/use-findings.ts` (+6) — the P-045 poll→commit mark
  - `pulse-app/ui/src/widget/ConstellationCanvas.test.tsx` (+36) — mock extension + 2 emitter-fires tests
  - `pulse-app/ui/src/canvas/frame-metrics.test.ts` (+9 tests) — bridge invocation, `clampDiscoveredCount` bound, reject-safe, stale-bindings no-op (added at wrap P2)
  - `pulse-app/ui/src/hooks/use-findings.test.tsx` (+2 tests) — P-045 emits on a committed refresh; does NOT emit on a rejected poll (added at wrap P2)
  - `pulse-app/ui/src/bindings/bindings.test.ts` (+16/−…) — the `telemetry.frontend` procedure-list pin
  - `pulse-app/ui/src/bindings/index.ts` — REGENERATED (never hand-edited)
  - `pulse-app/ui/tests-a11y/helpers/mock-tauri.ts` (+6) — three mocked commands
  - `docs/v0_2_0/capability-verification-matrix.json` — 3 `notes` values only (ids + scenarios byte-identical, verified by parse-compare)
  - `andromeda-pulse-0.3.0/verification-matrix.json` — P-075 `notes` only

- **Symbols / APIs:**
  - **3 NEW TauRPC procedures** on the EXISTING `telemetry.frontend` router (no new namespace):
    `telemetry.frontend.record_constellation_hue_latency` · `record_constellation_discovery_latency` · `record_findings_counter_refresh`
  - **3 NEW `metric.*` tracing targets**, each with its OWN exact allowlist leaf:
    - `metric.constellation.hue_update_ms` → leaf `["duration_ms", "severity_tier"]`
    - `metric.constellation.discovery_ms` → leaf `["duration_ms", "discovered_count"]`
    - `metric.findings.counter_refresh_ms` → leaf `["duration_ms"]`
  - New public Rust types in `ui_bridge::telemetry`: `HueSeverityTier` (bounded enum: none/curious/suggested/autonomous) · `ConstellationHueLatencyInput` · `ConstellationDiscoveryInput` · `FindingsCounterRefreshInput` · `validate_discovered_count`
  - New TS exports in `canvas/frame-metrics`: `recordConstellationHueLatency` · `recordConstellationDiscoveryLatency` · `recordFindingsCounterRefresh` · `clampDiscoveredCount` · `HueSeverityTierKind` + 3 input interfaces
  - **Remaining-caller facts:** `record_frame_ms` is UNCHANGED and keeps its callers (`CanvasContainer` + 3 colocated tests). The new procedures are ADDITIVE siblings — the code-graph impact query returned 3 callers for `record_frame_ms`, all its own colocated tests (`telemetry.rs:313,336,356`), i.e. zero production Rust callers, so no caller threading was required. `pulse-app/capabilities/*.json` was NOT touched: this project has no per-procedure capability entries (`default.json` grants `core:default` + 7 `core:window:*` + `updater:default`; TauRPC dispatches every method through one IPC handler).

- **Crates / modules:** none added · none removed · changed: `ui-bridge` (telemetry surface), `pulse-app` (allowlist + new test), `xtask` (expected-procedure list)

- **Dependencies:** none added · none bumped (no `Cargo.toml` / `Cargo.lock` / `package.json` delta)

- **Schema / config:** no migrations · no config keys · no violation-schema change. Two verification-ledger `notes` updates (see Spec-master edits — these are ledgers, not spec masters).

- **Spec-master edits:** **none** — no `.andromeda/` master was touched by this chunk. Four are QUEUED as expected amendments for this wrap's reconcile (see *Spec claims disproved by measurement* + the plan's Implementation notes).

- **Counts / qualifiers moved:** the workspace test count moved 1807 → 1819 (+12: 9 colocated ui-bridge + 3 allowlist guard); the webview suite moved 788 → **801** (+2 emitter-fires tests at implement, +11 bridge/P-045 tests added at wrap P2 after the coverage correction). The obs allowlist's registered `metric.*` leaf count moved 34 → 37. No doc states these counts.

- **Dev-tool versions:** none

- **Reverted / negative API facts:** the P-025 surface was FIRST implemented against `metric.halo.hue_update_ms` / `record_halo_hue_latency` / `HaloSeverityTier` and then fully renamed after the orphan finding (see below). No `halo`-named procedure, target, type, or leaf ships — verified by grep across `crates/`, `pulse-app/src`, `pulse-app/tests`, `xtask/src`, `pulse-app/ui/src`, `pulse-app/ui/tests-a11y`, and the capability matrix.

- **Spec claims disproved by measurement:**
  1. **`design-system.md` §Brand Identity ("Signature element") and §Motion, and `layout-templates.md` §Component — Halo State Pulse canvas, describe the Halo State Pulse canvas as a LIVE rendered surface. It has no production render site.** Evidence, three independent probes at HEAD: `grep -rn '<HaloCanvas' pulse-app/ui/src --include=*.tsx | grep -v '\.test\.'` → zero hits; `grep -n 'Halo\|halo' pulse-app/ui/src/dashboard/Dashboard.tsx` and the same over `widget/CompactWidget.tsx` → zero hits; every non-test reference to `HaloCanvas` is a `vi.mock` factory (`Dashboard.test.tsx:18`, `router.test.tsx:25`) or a comment. The severity→hue semantic IS live, via `severityToHueFraction` imported at `pulse-app/ui/src/widget/constellation-types.ts:13` and applied to the constellation DOT hue.
  2. **`layout-templates.md` §Component — Halo State Pulse canvas describes the hue as LCH-interpolated by ERROR RATE.** `design-system-amendments.md` 2026-05-29 re-drove the hue to CUMULATIVE INCIDENT SEVERITY, and the code follows severity (`severity-to-halo.ts::severityToHueFraction(tier: PriorityTier | null)`; `ConstellationDot.priorityTier`). The layouts body is stale against both its sibling spec and the code. (Queued as an expected amendment by the plan.)
  3. Both of the above need disposition by this wrap's reconcile; neither was authored by /implement.

- **Coverage of new surfaces:**
  - `telemetry.frontend.record_constellation_hue_latency` → validation **✓** (serde + bounded `HueSeverityTier` closed enum + `validate_duration_ms`; `AppError::Validation{field,reason}` on reject) · instrumentation **✓** (`metric.constellation.hue_update_ms`, level-gated, exact leaf) · PII **redacted✓** (aggregate-only; no service identifier — guarded by test) · tests **unit** (colocated resolver + rejection tests; webview emitter-fires test) · a11y **n/a** (no UI element added) · tokens **n/a**
  - `telemetry.frontend.record_constellation_discovery_latency` → validation **✓** (+ `validate_discovered_count`, bound 10_000) · instrumentation **✓** (`metric.constellation.discovery_ms`) · PII **redacted✓** (aggregate count, never `service`) · tests **unit** · a11y **n/a** · tokens **n/a**
  - `telemetry.frontend.record_findings_counter_refresh` → validation **✓** · instrumentation **✓** (`metric.findings.counter_refresh_ms`) · PII **redacted✓** · tests **unit** · a11y **n/a** · tokens **n/a**
  - webview mark sites (`ConstellationCanvas.tsx`, `use-findings.ts`) + the `frame-metrics.ts` bridge → validation **n/a** (client-side clamp, backend re-validates) · instrumentation **✓** · PII **n/a** · tests **unit** (vitest — see the correction note below) · a11y **✓** (no new focusable element, no added live-region announcement, no visible chrome; `cargo xtask self-verify` a11y chain green with 0 new violation tuples vs baseline) · tokens **n/a** (no rendered value added)

  **Coverage correction (wrap P2).** This bullet first graded the webview leg `tests unit` while
  `frame-metrics.ts`'s five new exports never executed in any test (the only webview test touching them
  `vi.mock`s the module) and `use-findings.ts`'s P-045 mark had no emitter-fires test — the webview suite
  had moved exactly +2, both in `ConstellationCanvas.test.tsx`. The claim outran its evidence. Caught by the
  test-plan drift detector against this report; the gap was CLOSED rather than documented (operator-decided):
  `frame-metrics.test.ts` gained 9 tests (the `clampDiscoveredCount` 10_000 bound, per-bridge invocation +
  field-shape, reject-does-not-throw, and the stale-bindings no-op path) and `use-findings.test.tsx` gained 2
  (P-045 emits on a committed refresh; does NOT emit when the poll rejects, the absence paired with a
  positive `listActiveFn` fired-assertion so it cannot pass vacuously). Webview suite 790 → **801**.

## Deviations from intent

1. **P-025's mark site and metric name changed from the plan.** Plan step 4 directed the halo mark to `HaloCanvas.tsx:145–147` with target `metric.halo.hue_update_ms`. That component is orphaned (evidence above), so timing it would have shipped an emitter that never fires — the producer-never-ran vacuous class. **Justification:** surfaced to the operator at /implement P4 as a spec↔reality gap; the operator independently verified the orphan and directed option (a) — point the mark at the constellation dot hue where `severityToHueFraction` actually runs, name the metric for the real surface, keep the exact-leaf discipline, leave no forward-inert emitter, and do not invent a halo surface. Applied: `metric.constellation.hue_update_ms` via `record_constellation_hue_latency`, firing on a per-service tier CHANGE measured from that service's `last_seen_unix_nano`. The halo canvas's own fate is deferred to a future route entry.

2. **One companion file edited beyond the plan's Files-to-modify list:** `pulse-app/ui/src/widget/ConstellationCanvas.test.tsx` — its `vi.mock("../canvas/frame-metrics")` factory needed the new exports or 7 pre-existing tests fail at import. **Justification:** the documented test-fan-out class; a co-located test of a listed file, judged in-scope per the gray-area rule rather than soft-exited.

3. **`docs/v0_2_0/capability-verification-matrix.json` was reformatted then restored.** A `json.dumps(indent=2)` rewrite exploded the file's compact one-line-per-capability style into a 1068-line diff. **Justification:** caught at wrap P1 before commit; the file was restored to HEAD's formatting with only the 3 `notes` string values re-applied, verified by parse-compare (ids + scenarios byte-identical; only P-025/P-027/P-045 `notes` differ). Final diff: 3 lines.

## Decisions & corrections

- **Corrected at wrap P7 — a gate command I authored named the wrong config.** The plan's suite-health probe read `npx playwright test --list --config pulse-app/ui/playwright.config.ts`. That root config has `testDir: "./tests-e2e"`, a directory that does not exist yet (the P-076 tauri-driver suite is unbuilt), so the probe reported **0 tests in 0 files** — a false green for the exact suite-health check whose whole purpose is catching a silently-collected-to-zero suite. The a11y suite is driven by a SEPARATE config, `playwright-a11y.config.ts` (per the `test:a11y` script). The correct probe, `(cd pulse-app/ui && npx playwright test --list --config=playwright-a11y.config.ts)`, reports **33 tests in 15 files**. The plan's Test Commands line was corrected to the working form; the discrepancy was caught by disbelieving the 0 against a self-verify run that had just executed 33 playwright tests.

- **Operator decision (P-025 fork, 2026-08-21):** option (a) — preserve the capability's semantic on the surface users actually see. Recorded in both matrices' `notes`.
- **Operator directive:** never leave a forward-inert emitter (the producer-never-ran vacuous class); do not invent a halo surface.
- **Non-vacuous negation applied unprompted** (operator-confirmed): the "no hue update at the calm baseline" absence assertion is paired with a positive `recordConstellationDiscoveryLatency` fired-assertion first, so the absence cannot pass vacuously on an effect that never ran. Same discipline as the epoch's 2026-08-17 curated rules.
- **Mutation check discharged:** neutralizing the hue allowlist leaf turned 2 of 3 guards red (including the fallback-discrimination test); restore returned 3/3 green. The third (PII bans) correctly stayed green — the bare `metric` fallback carries no identifier either, so it guards a different property.
- **Corrected mid-implement:** a first draft of the field-capture test helper used `Handle::block_on` inside `#[tokio::test]` (panics — "cannot block from within a runtime"); switched to the documented `set_default` guard pattern before running.
- **Corrected mid-implement:** clippy was first run without `-D warnings` and passed while emitting a complex-type warning the real gate form would have failed; caught by comparing the invocation to the plan text, fixed with a type alias, re-run in gate form.

## Outcome

**All plan acceptance criteria met.** Gates, all run in their plan-listed form:

| Gate | Result |
|---|---|
| `cargo fmt --check` | ✓ |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | ✓ |
| `cargo nextest run --workspace --profile ci` | ✓ **1819 passed / 1 skipped** (from 1807, exactly +12) |
| `cargo xtask capability-drift` | ✓ clean (0 missing, 0 extra) |
| `cargo xtask capability-widening-check` | ✓ clean (0 violations / 3 inspected) |
| `cargo xtask verify:capability-matrix` | ✓ clean (60/60, 0 violations) |
| `npm run lint --prefix pulse-app/ui` | ✓ |
| `npm run typecheck --prefix pulse-app/ui` | ✓ |
| `npm run test --prefix pulse-app/ui` | ✓ **790 passed** (77 files; from 788, +2) |
| `npx playwright test --list` | ✓ (suite collects; ran inside the self-verify chain) |
| `cargo xtask self-verify` | ✓ **PASS** — 33 playwright, Lighthouse 7 surfaces ≥90, pa11y 7/7, regression-detector **0 new violation tuples vs baseline** |
| `cargo deny check bans licenses sources` | ✓ ok |
| `cargo deny check advisories` | observed separately (designed-red) — see below |
| `cargo audit` | fired (PREREQ) — see below |

**Smoke:** the boot-path/UI condition fired; `cargo xtask self-verify` ran as a listed P2 gate (boot the real binary → assert shell health → a11y/contrast chain → clean quit, zero orphan), after a warm re-embed (`npm run build --prefix pulse-app/ui` + `cargo build -p pulse-app`) so the binary carried the fresh frontend. No re-run at P3 — recorded, not repeated. Zero fix-loop re-entries.

**`cargo audit` — the folded PREREQ probe FIRED (session 31, the ratified interval point).** Result: **exit 1**, `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`. The basis **reproduced byte-identical** and is upstream — no repo change can clear it. The standing deferral therefore CONTINUES and the interval resets.

**`cargo deny check advisories` — the named overlap MOVED: the owned upgradeable set is now EIGHT IDs, not seven.** Observed: `RUSTSEC-2026-0189 · 0190 · 0194 · 0195 · 0204 · 0222 · 0253` (the seven standing) **+ `RUSTSEC-2026-0258` (NEW)**. The new one is `h2 0.4.14` — the crate accepts and queues empty DATA frames without limit, so an undrained stream can drive unbounded memory use or a length-overflow panic; **low severity; stated safe upgrade `>=0.4.16` (`cargo update -p h2`)**. Reached via `hyper 1.9.0 → axum 0.8.9`. Per the visible-disposition rule a finding WITH a stated safe upgrade is never ignore-listed — it stays RED under a named owner. **Not actioned here:** upgrading it touches `Cargo.lock`, which is outside this chunk's Files-to-modify and would invalidate the gate run; it belongs to the **Advisory backlog** route entry, annotated at this wrap's route-resolve.
