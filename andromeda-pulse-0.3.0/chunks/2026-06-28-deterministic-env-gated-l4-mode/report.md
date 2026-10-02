# Report — 2026-06-28-deterministic-env-gated-l4-mode

**Chunk:** Deterministic env-gated L4 mode (canned L4Output, no GPU/3B) — P-073 (intent F13a), keystone
**Date:** 2026-06-28
**Commits:** (not yet committed — this wrap commits the chunk)

## Changes (structured — detectors read this)
- **Files:** `pulse-app/src/deterministic_inference.rs` (new) · `pulse-app/src/main.rs` (mod) · `pulse-app/src/lib.rs` (`pub mod`) · `pulse-app/tests/unit_deterministic_inference.rs` (new) · `pulse-app/tests/integration_deterministic_l4_mode.rs` (new) · `andromeda-pulse-0.3.0/verification-matrix.json` (P-073 ledger).
- **Symbols / APIs:** new `pulse_app::deterministic_inference::{DeterministicInferenceRunner (impl interpretation::contract::LlmInferenceRunner), deterministic_mode_enabled(), deterministic_mode_enabled_for(), ENV_L4_DETERMINISTIC, CANNED_L4_OUTPUT_JSON}`. **NEW ENV VAR: `ANDROMEDA_PULSE_L4_DETERMINISTIC`** (truthy gate selecting the deterministic L4 runner at the `pulse-app` boot boundary). NO new IPC method / endpoint / port / socket / broadcast topic / capability.
- **Crates / modules:** added one `pulse-app` module (`deterministic_inference`). NO new workspace crate. `LlmInferenceRunner` trait unchanged (a third impl behind it, alongside `LlamaCliInference` + the test `StubInferenceRunner`).
- **Dependencies:** none added · none bumped.
- **Schema / config:** none — the canned output reuses the existing `interpretation::schema::L4Output` (schema_version 2.0); no new corpus table, no config.toml key, no violation schema.
- **Coverage of new surfaces:**
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC` env input → validation {bounded truthy-parse ✓ — `1`/`true`/`yes`, no unbounded string} · instrumentation {boot `inference_mode` log on `interpretation.model.load` ✓} · PII {n/a — boolean gate} · tests {unit (`env_gate_truthy_parse_table`) ✓} · a11y n/a · tokens n/a
  - deterministic L4 runner (canned output) → validation {schema-valid via existing parse+`validate()` ✓} · instrumentation {existing `handle_digest_outcome` L4 tracing unchanged ✓} · PII {canned output first-party constants only — `canned_output_carries_no_pii` ✓} · tests {unit 4 + integration 1 ✓} · a11y n/a · tokens n/a

## Deviations from intent
- **`main.rs` constructs `LlamaCliInference` in both modes** (the gate switches which becomes the active `llm_runner`; in deterministic mode llamacli is built-but-unused-as-runner). Justification: keeps the existing `model_status_broadcast` + the cheap file-existence readiness check intact with a minimal diff; the "no model" guarantee holds because `generate_constrained` is never called on llamacli when gated (no subprocess).
- **Integration test digest carries an `attention_cue`.** Justification: research surfaced that `create_incident_from_l4_output` only creates an incident for a cue-bearing (or Reflection) digest — a test-fixture requirement mirroring production cue-triggered digests + the existing `unit_incident_producer.rs`, NOT a production gap.
- **Boot smoke skipped for cause.** Justification: backend-only chunk; the default boot path (gate unset) is byte-identical to the prior code (guarded `else`); the deterministic runtime path is exercised green by `integration_deterministic_l4_mode` + the workspace e2e tests; the Tauri release-boot carries the documented Windows orphan hazard (verification-harness.md 2026-05-19) + a ~20-min release build, disproportionate for a branch that adds no new fallible boot-time op.

## Decisions & corrections
- Env var named `ANDROMEDA_PULSE_L4_DETERMINISTIC` (SCREAMING_SNAKE_CASE, no collision with the reserved set); truthy-parse follows the `crates/mcp-server/src/feature_gate.rs` pattern.
- Canned output = a FIXED `Decision::Surface` + `Severity::Autonomous` (→ `IncidentSeverity::Error`, the red-dot) incident shape, so Conductor (P-075) + the integration UX test (P-076) can assert a KNOWN incident.
- "Reuse the StubInferenceRunner pattern" = PRODUCTIONIZE it — the stub is test-only (`unit_inference_runtime.rs`); created a new production `DeterministicInferenceRunner` modeled on it.
- Build-infra de-race: the workspace nextest hit the documented rlib/incremental/OOM race; resolved via `cargo clean -p pulse-app` + `CARGO_INCREMENTAL=0` + targeted `cargo test --test` (sessions 165/183) + `--jobs 4` to cap parallel-compile memory.

## Outcome
- **Acceptance criteria met.** Gates green: `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` · `cargo nextest run --workspace --profile ci` (**1681 passed**, +5) · `cargo xtask capability-drift` (clean 0/0; bindings regen confirmed) · new binaries `unit_deterministic_inference` 4/4 + `integration_deterministic_l4_mode` 1/1.
- **Verification matrix:** P-073 → `status: implemented`, ref `integration_deterministic_l4_mode.rs::deterministic_mode_yields_reproducible_red_dot_incident`.
- **Smoke:** skipped for cause (backend-only; see Deviations).
- **For wrap:** the new env var needs arch §Occupied Resources registration (`D-arch-resources`).
