# Codebase Research — 2026-06-28-deterministic-env-gated-l4-mode

## Scope
- **Depth:** moderate · **Reads:** 3 files (llamacli_inference.rs full · main.rs §420–490 · interpretation/contract.rs full) + targeted (inference_runtime.rs §270–340) · **Greps:** 4 · **Code-graph queries:** 6 (trace: `tree-query-2026-06-28-deterministic-env-gated-l4-mode.json`).

## Files inspected
- `pulse-app/src/llamacli_inference.rs` (full) — the concrete `LlmInferenceRunner` impl (`LlamaCliInference`) the deterministic runner sits beside. Trait surface: `current_status` / `identity` / `tier` / `generate_constrained(prompt, schema_json) -> InferenceFuture<String>` — returns the **raw extracted JSON String**, NOT an `L4Output`. Construction `LlamaCliInference::new(tier, profile, broadcast)`; graceful-degraded `Error` state when env paths unset.
- `pulse-app/src/main.rs` §420–490 — the runner SELECTION site: lines **442–448** build `LlamaCliInference` → cast to `Arc<dyn LlmInferenceRunner>` as `llm_runner` (+ `ModelApiImpl::new(llm_runner, …)`). THE branch point for the env gate. Boot readiness `load_from_env_if_configured` fire-and-forget at 458–463.
- `crates/interpretation/src/contract.rs` (full) — `LlmInferenceRunner` trait (object-safe, `Send + Sync`; `InferenceFuture<'a,T> = Pin<Box<dyn Future + Send>>`) + `InferenceError` + `ModelTier`/`ModelStatus`/`ModelIdentity`. Trait only — no prod stub here.
- `pulse-app/src/inference_runtime.rs` §270–340 — `handle_digest_outcome(runner, digest)`: assembles prompt → `runner.generate_constrained(&prompt, L4_OUTPUT_JSON_SCHEMA).await` → parses → `L4DigestOutcome` → (downstream) incident. The canned JSON lands here UNCHANGED.

## Graph impact (from tree.db; trace recorded)
- **LlmInferenceRunner** — 49 call/ref sites: prod construction `main.rs:442–448`; invocation `inference_runtime.rs` (`spawn_l4_inference_subscriber`/`handle_digest`/`handle_digest_outcome` 119–336); held by `diagnostics_router.rs` (`DiagnosticsApiImpl#runner`, retry_interpretation). → an additive impl + one main.rs selection branch is the whole surface; NO trait change.
- **crate_edges(interpretation)** — outbound interpretation→{security, triage}; inbound {mcp-server, pulse-app}→interpretation. The new runner lives in pulse-app (already inbound) → **zero new cross-crate edges**.
- **L4Output** — defined `crates/interpretation/src/schema.rs:158`; 182 refs. The canned output must decode to this type via the existing `handle_digest_outcome` parse path.

## Patterns detected
- **Runner = binary-boundary impl** (`llamacli_inference.rs:499`): the concrete `LlmInferenceRunner` lives in pulse-app, not interpretation (arch §Module dependency direction). The deterministic runner follows — a new pulse-app module.
- **Env-gate truthy parse** (`crates/mcp-server/src/feature_gate.rs:3,17`): `pub const ENV_X: &str = "…"` + `match env::var(ENV_X) { Ok(v) if truthy => … }`. The new gate copies this shape.
- **`generate_constrained` returns RAW JSON String** (`llamacli_inference.rs:516`, `contract.rs:183`): the runner returns schema-constrained JSON TEXT; `inference_runtime::handle_digest_outcome` owns parse→L4Output→incident. So the deterministic runner returns a canned JSON **String** (not an `L4Output` struct).
- **Test-only stub pattern** (`pulse-app/tests/unit_inference_runtime.rs:99–130` `StubInferenceRunner::new_ok(tier, canned_json)`; also `StubResolutionRunner` / `CanaryMalformedRunner` in sibling tests): the intent's "StubInferenceRunner pattern" is TEST-only. **No production deterministic runner exists** → this chunk PRODUCTIONIZES the pattern (new prod impl modeled on the test stub). This is a "create new", not a "reuse existing prod code" — the intent's word "pattern" already anticipates it.

## Conventions to follow
- Env var `SCREAMING_SNAKE_CASE` `ANDROMEDA_PULSE_*`, no collision with the reserved set (arch §Occupied Resources) → propose `ANDROMEDA_PULSE_L4_DETERMINISTIC` (truthy → deterministic mode). New var → arch §Occupied Resources registration (wrap amendment; phase is read-only on arch).
- Canned L4 JSON must be schema-valid per `crates/interpretation/src/schema.json` + decode to `L4Output` (schema.rs:158) + carry a deterministic **non-`info` severity** so the red-dot / Findings climax fires.
- Obs (per obs extract): boot-time `inference_mode` field; NO canned-output content logging; p99 SLO neutral on deterministic mode.
- Tests (per tests extract): source-level `mod tests` in pulse-app are dead (`[lib] test=false`) → unit tests go in `pulse-app/tests/*.rs`; standard gate baseline + boot-smoke (main.rs touched).

## New files to create
- `pulse-app/src/deterministic_inference.rs` — `DeterministicInferenceRunner` (impl `LlmInferenceRunner`): `current_status`=`Loaded`, `identity`=`Some("deterministic-stub")`, `tier`, `generate_constrained` returns a FIXED canned schema-valid L4 JSON String. + `ANDROMEDA_PULSE_L4_DETERMINISTIC` const + truthy parse helper. Modeled on the test `StubInferenceRunner`.
- `pulse-app/tests/integration_deterministic_l4_mode.rs` — env-gated: digest → deterministic L4 → incident completes reproducibly (drives `inference_runtime::handle_digest_outcome`).
- `pulse-app/tests/unit_deterministic_inference.rs` — the runner returns valid schema-conformant JSON, `Loaded`, deterministic severity; gate truthy-parse table.

## Files to modify
- `pulse-app/src/main.rs` (~442–448) — branch `llm_runner` selection: env gate truthy → `DeterministicInferenceRunner`; else the existing `LlamaCliInference`. (boot path → boot-smoke gate applies.)
- `pulse-app/src/lib.rs` — `pub mod deterministic_inference;` (integration-test reach).

## Open questions
- Env var name `ANDROMEDA_PULSE_L4_DETERMINISTIC` (recommended) — finalize at P4; arch-registered at wrap. (0 blocking.)
- Canned-output content (severity/title) — pick a deterministic `error`-severity incident shape; tunable. Resolve at P4 — no blocking ambiguity (intent is explicit on "canned L4Output").
