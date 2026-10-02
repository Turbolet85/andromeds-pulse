# Report — 2026-06-28-investigate-actions-functional

**Chunk:** Investigate actions functional — 4 buttons run real L4-backed analysis with visible progress + result/error
**Date:** 2026-06-29
**Commits:** (uncommitted at report time — this chunk commits in this wrap's P7; nothing new since last_wrap 2026-06-28T22:49:57Z)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `pulse-app/src/investigate_router.rs` · NEW `pulse-app/tests/integration_investigate_actions.rs`
  - MOD `pulse-app/src/main.rs` (construct + merge `investigate_impl` in both router branches + the `emit_taurpc_bindings` test) · `pulse-app/src/lib.rs` (`pub mod investigate_router`) · `pulse-app/src/snapshot_runtime.rs` (shared `load_curated_markdown` helper) · `pulse-app/src/observability.rs` (2 AllowList entries) · `xtask/src/main.rs` (`EXPECTED_PROCEDURES += investigate.run_action`) · `pulse-app/capabilities/default.json` (router-level coverage doc note)
  - MOD webview: `pulse-app/ui/src/dashboard/InvestigationModalForm.tsx` (+ `.test.tsx`) · `pulse-app/ui/src/components/PresetPromptList.tsx` (+ `.test.tsx`) · `pulse-app/ui/src/bindings/index.ts` (regenerated)
- **Symbols / APIs:**
  - NEW TauRPC procedure **`investigate.run_action(action_id: String) -> Result<InvestigateResultDto, AppError>`** (dotted `<router>.<verb>`; path `investigate`)
  - NEW cross-bridge types `InvestigateResultDto` / `InvestigateHypothesis` / `InvestigateStep` (serde + `specta::Type`)
  - NEW obs targets **`investigate.run_action.request`** + **`metric.investigate.run_action.duration_ms`** (aggregate-only)
  - NEW `pub(crate) fn snapshot_runtime::load_curated_markdown(&Connection) -> Result<String, AppError>` (shared telemetry-context helper)
  - NEW webview prop `PresetPromptList.busyId` (per-button `aria-busy` + disable-while-running)
  - Behavior change: the 4 Investigate action buttons now call `investigate.run_action` (were: copy prompt template to clipboard, the dead path)
- **Crates / modules:** NEW module `pulse-app::investigate_router` (NO new workspace crate — reuses `interpretation` + `pulse-app` per arch §Inherited Defaults)
- **Dependencies:** none added · none bumped
- **Schema / config:** no new env vars (reuses `ANDROMEDA_PULSE_L4_DETERMINISTIC`, P-073) · no DuckDB/corpus migrations · no new Tauri capability permission (router-level `pulse:default` already covers all TauRPC procedures per security.md 2026-05-03; `default.json` description note only) · no new broadcast topic (result is transient, no incident/persistence)
- **Coverage of new surfaces:**
  - `investigate.run_action` (TauRPC IPC) → validation **✓** (`action_id` bounded to the 4 known ids → `AppError::Validation` on miss; snapshot context already-trusted) · instrumentation **✓** (`investigate.run_action.request` event + `metric.investigate.run_action.duration_ms`, aggregate-only) · PII **redacted✓** (scrubbed via `security::scrubber::scrub_attribute` at egress; obs logs only bounded `action_id`/`status`/`duration_ms`/`deterministic_mode`/`model_tier` — NO prompt/result/telemetry; negative-canary test) · tests **integ✓** (`integration_investigate_actions.rs` 8 tests) · a11y n/a (backend) · tokens n/a
  - Investigate modal action buttons + result/error panels (webview UI, within the existing Investigation modal) → validation n/a · instrumentation n/a · PII n/a · tests **unit✓** (`InvestigationModalForm.test.tsx` 13 + `PresetPromptList.test.tsx`) · a11y **✓** (`aria-busy` per running button, `role="alert"` error, `aria-live` status, `aria-label`-free buttons use visible label as name, focus stays on button, Esc still closes — SC 4.1.3 / 2.1.2 / 2.4.3) · tokens **design-token✓** (`--color-*`/`--spacing-*`/`--font-*`/`--radius-*`; `prefers-reduced-motion` respected; error pairs accent border + primary text, never color alone)

## Deviations from intent
- **arch §Occupied Resources entry NOT added by /implement** — `/implement` is read-only on specs; the `investigate.run_action` §Occupied Resources line is **left for this wrap to add** (D-arch-resources is expected to propose it). Not gate-blocking (capability-drift keys on EXPECTED_PROCEDURES + bindings, both synced).
- **`p13` Playwright axe spec deferred** — the new states' a11y is unit-verified (aria-busy/role=alert/aria-live/label/focus); `p2-investigation-modal` already covers this modal; the a11y Playwright suite is a separate CI job not in this chunk's gate, and shipping an unrun spec risks breaking it. Recommended a11y-suite follow-up.
- **`unit_investigate_router.rs` not created** (plan listed it) — the resolver's private helpers aren't reachable cross-crate and `[lib] test=false` disables source `mod tests`, so all coverage runs through the public API in the single `integration_investigate_actions.rs`. No coverage lost.
- **`InvestigateApiImpl` omits `data_dir`** (plan listed `{conn, data_dir, runner}`) — the resolver writes no files (transient result), so `data_dir` is unused; dropped for minimalism.

## Decisions & corrections
- **Result contract = reuse the incident `L4Output` schema** (phase P4 user decision over a new free-form analysis contract): the resolver calls the existing `generate_constrained(prompt, L4_OUTPUT_JSON_SCHEMA)` + `parse_bounded` → maps `L4Output` analysis fields to `InvestigateResultDto`; zero change to the `LlmInferenceRunner` trait or the P-073 deterministic runner. Under deterministic-L4 all 4 actions return the same canned analysis (acceptance is "a reproducible result", not 4 distinct); real model → 4 distinct per prompt framing.
- **No incident on click** — the investigate path deliberately does NOT call `create_incident_from_l4_output`; the result is transient (no corpus write, no `pulse://stream/incidents` broadcast). This is the behavioral boundary vs the digest→L4→incident path that shares `generate_constrained` (1 prod caller before this chunk; now 2).
- **PII-canary test robustness** — thread-local `tracing::subscriber::set_default` does NOT capture the resolver's post-`spawn_blocking` obs emit under in-process parallel libtest (the continuation thread-hops); an initial canary test was therefore vacuous (captured `[]`). Switched to `set_global_default` (the only subscriber-setting test in the binary → one clean call) so capture is robust under both nextest (per-process) and `cargo test` (parallel). Catch: asserting the investigate event IS captured prevents a silently-vacuous no-leak loop.
- **MCP path considered + rejected** for the backend (intent says "LLM/MCP"): MCP is feature-gated/off-by-default and wouldn't run under the deterministic-L4 acceptance; in-process L4 is the path.

## Outcome
- **P-072 acceptance MET** — under deterministic-L4 each of the 4 actions yields a populated result reproducibly; unknown id → Validation; missing buffer → Storage; runner + parse errors surface (never a silent no-op). Webview: click → `aria-busy` → result panel / `role="alert"` error.
- **Gates green:** `cargo fmt --check` ✓ · `cargo clippy --workspace --all-targets --all-features -D warnings` ✓ · `cargo nextest run --workspace --profile ci` **1694 pass / 1 skip** (+8) · `cargo xtask capability-drift` clean · webview `typecheck`/`lint`/`test` (**642**) ✓ · `bindings.ts` staged-shape `mcp=1 investigate=1` (regen last cargo step).
- **Smoke: skipped-for-cause** — boot-path change is a pure resolver merge + cheap sync construction (no new boot-time reactor/spawn/I-O code); `emit_taurpc_bindings` constructs the full production router incl. `investigate_impl`; Windows GUI-orphan hazard (testing.md 2026-05-19) + P-073/P-074 precedent; runtime covered by the integration + webview suites.
