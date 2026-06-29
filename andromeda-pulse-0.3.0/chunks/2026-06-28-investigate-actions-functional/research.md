# Codebase Research — 2026-06-28-investigate-actions-functional

## Scope
- **Depth:** deep · **Reads:** 8 (interpretation/contract.rs · deterministic_inference.rs · inference_runtime.rs · model_router.rs · snapshot_runtime.rs · snapshot/contract.rs · InvestigationModalForm.tsx · preset-prompts.ts) · **Globs/Greps:** 4 · **Code-graph queries:** 1

## Files inspected
- `pulse-app/ui/src/dashboard/InvestigationModalForm.tsx` (full) — modal auto-calls `snapshot.generate(preset, null)` on open (works). The 4 action buttons render via `PresetPromptList onPick={handlePresetPick}`; `handlePresetPick` (line 127) ONLY `writeText(prompt.template)` + a status message — **never runs an analysis**. This is the dead behavior to replace. State machine: `phase: "idle"|"capturing"|"result"|"error"` + `aria-busy` + `aria-live` already present (reuse for per-action progress).
- `pulse-app/ui/src/dashboard/preset-prompts.ts` (full) — the 4 actions (`diagnose-latency-outlier` / `find-error-correlation` / `trace-failed-request` / `summarize-service-health`) with label + `template` (the analysis instruction text). Backend mirror at `snapshot_runtime.rs:46` `PRESET_PROMPT_DEFINITIONS` (id+label only).
- `crates/interpretation/src/contract.rs` (full) — `LlmInferenceRunner` trait: `current_status` / `identity` / `tier` / **`generate_constrained(prompt, schema_json) -> InferenceFuture<String>`** (the ONLY inference method; schema-constrained). `InferenceError` enum (8 variants, all sanitized). Object-safe via `Pin<Box<dyn Future>>` (no `async-trait`). NO LLM-runtime import (leaf-crate-clean).
- `pulse-app/src/deterministic_inference.rs` (full) — P-073 `DeterministicInferenceRunner`: `generate_constrained` returns `CANNED_L4_OUTPUT_JSON` **regardless of prompt or schema** (a fixed incident-shaped `L4Output`, `severity: autonomous`). `deterministic_mode_enabled()` reads `ANDROMEDA_PULSE_L4_DETERMINISTIC` (truthy `1|true|yes`). Implication: under deterministic mode every investigate action returns the SAME canned analysis (acceptable for "a reproducible result"; NOT 4 distinct).
- `pulse-app/src/inference_runtime.rs` (full) — the ONLY production consumer of `generate_constrained` today: `handle_digest_outcome` (line 336) builds prompt via `interpretation::prompt::build_primary_tier_prompt(payload_summary, project_context, "")`, calls `generate_constrained(prompt, L4_OUTPUT_JSON_SCHEMA)`, then `interpretation::schema::parse_bounded(bytes) -> L4Output`. `create_incident_from_l4_output` (line 626) PERSISTS+broadcasts an incident — the investigate path must NOT call this (transient result, no incident). `L4Output` fields: title / symptom / timeline / hypotheses[] / investigation_steps[] / evidence_refs[] / fingerprint / decision / severity / is_resolution_summary.
- `pulse-app/src/model_router.rs` (full) — the canonical resolver-injection pattern: `ModelApiImpl::new(runner: Arc<dyn LlmInferenceRunner>, ...)`, `#[taurpc::procedures(path="model")]`, `#[taurpc::resolvers]`, `String`-only payload (`ModelProfilePayload`) to sidestep cross-crate specta. `inference_error_to_app_error(InferenceError) -> AppError` free-fn (the 2026-05-18 cross-crate-error pattern) — directly reusable.
- `pulse-app/src/snapshot_runtime.rs` (full) — resolver that produces the telemetry context: `SnapshotApiImpl { conn: Option<Arc<Mutex<Connection>>>, data_dir, app_handle }`; `load_recent_spans` (5000-row, 5-min window) → `snapshot::contract::curate` → `format_markdown(budget)` → `report.markdown` (in-memory before file write). PII negative-canary test pattern (`CapturingSubscriber` + `FieldCollector`) is the template for the investigate obs test.
- `crates/snapshot/src/contract.rs` (full) — `curate(&[SpanRecord]) -> CurationOutput`, `format_markdown(&CurationOutput, TokenBudget) -> MarkdownReport { markdown, token_count, ... }`. Pure, reproducible. The markdown is the curated-telemetry context string.

## Graph impact (from the code-graph query)
- **`generate_constrained`** — production callers: **exactly 1** (`inference_runtime::handle_digest_outcome` @ `pulse-app/src/inference_runtime.rs:336`) + 3 test callers. → The investigate path is an ADDITIVE second consumer of the same trait; zero blast radius on the existing digest→L4→incident path, no trait change required to add a consumer.

## Patterns detected
- **Resolver injection of the runner** (`model_router.rs:66-81`, boot at `main.rs:476`): `Arc<dyn LlmInferenceRunner>` cloned from the single boot-built `llm_runner` (`main.rs:458-475`, deterministic-vs-llamacli select). A new `InvestigateApiImpl` takes `Arc::clone(&llm_runner)` the same way.
- **Telemetry-context build** (`snapshot_runtime.rs:79-200`): load recent spans → curate → format_markdown produces the curated markdown in-memory. Factor a shared `curated_context()` helper (or call the same primitives) so investigate reuses it without re-implementing.
- **Cross-crate error map** (`model_router.rs:129` `inference_error_to_app_error`): reuse verbatim for the investigate resolver's `generate_constrained` error path.
- **PII negative-canary test** (`snapshot_runtime.rs:495` + `inference_runtime` aggregate-only obs): the obs test asserts no prompt/result/attribute content in any tracing field.
- **L4Output-shaped render** (chunk #88): `pulse-app/ui/src/dashboard/report-types.ts` + `ReportRenderer.tsx` already render an L4Output-shaped report — a reuse candidate if the result is L4Output-shaped (Option A).

## Conventions to follow
- **New TauRPC namespace = full binding checklist** (security.md 2026-05-09 + 2026-05-12): an "operation" (not a Settings get/set) needs its own namespace → (1) router registration (`#[taurpc::procedures(path="investigate")]`), (2) `pulse-app/capabilities/` JSON, (3) `xtask/src/main.rs::EXPECTED_PROCEDURES`, (4) the `emit_taurpc_bindings` test `.merge(...)` chain (`main.rs:~1858`), (5) arch §Occupied Resources. NOT feature-gated → no cfg-gate. Merge into BOTH production router branches (`main.rs:921-934` + `950-960`) AND the test branch.
- **Aggregate-only obs** (observability.md 2026-05-17 + obs extract): bounded `action_id` + `status` enum tags + numeric `duration_ms`/counts; NO prompt template, NO analysis result body, NO per-service identifiers. New `metric.*`/`investigate.*` targets need `pulse-app/src/observability.rs` AllowList entries.
- **bindings.ts regen discipline** (testing.md 2026-05-13/17/06-12): after default-features test runs, regen via `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'`; verify staged copy `git show :…/index.ts | grep -c '"investigate"' >= 1` before commit.
- **pulse-app tests are integration-only** (testing.md 2026-05-20): `[lib] test=false` → put resolver tests in `pulse-app/tests/*.rs`, NOT source `mod tests`.

## New files to create
- `pulse-app/src/investigate_router.rs` — `InvestigateApi` trait (`#[taurpc::procedures(path="investigate")]`) + `InvestigateApiImpl { conn, data_dir, runner }` + `InvestigateResultDto` + resolver (build context → prompt → `generate_constrained` → `parse_bounded` → DTO; NO incident creation) + aggregate-only obs.
- `pulse-app/tests/unit_investigate_router.rs` (+ likely `pulse-app/tests/integration_investigate_actions.rs`) — resolver unit tests (each action_id → result under deterministic mode; bad action_id → Validation; runner-error → AppError) + the e2e acceptance under deterministic-L4 + PII negative-canary.
- `pulse-app/capabilities/` — entry for the `investigate` namespace (or extend the existing default capability per the router-level granularity per security.md 2026-05-03).

## Files to modify
- `pulse-app/src/main.rs` — construct `InvestigateApiImpl` (~line 476, `Arc::clone(&llm_runner)` + `conn` + `data_dir`); `.merge(...)` it into the two production router branches + the `emit_taurpc_bindings` test branch; `mod investigate_router;` + `use`.
- `pulse-app/src/lib.rs` — `pub mod investigate_router;`.
- `pulse-app/src/observability.rs` — AllowList entries for the new `investigate.*` / `metric.investigate.*` targets.
- `xtask/src/main.rs` — add the `investigate.*` procedure(s) to `EXPECTED_PROCEDURES`.
- `pulse-app/ui/src/dashboard/InvestigationModalForm.tsx` — wire each action to `proxy.investigate.run_action(action.id)`; per-action progress (`aria-busy`) + result render + error (`role="alert"`).
- `pulse-app/ui/src/dashboard/preset-prompts.ts` and/or `components/PresetPromptList.tsx` — pass the action id to the run handler; busy/result affordance.
- `.andromeda/architecture.md` §Occupied Resources — new `investigate.*` TauRPC procedure entry (the scope-arch path).
- `andromeda-pulse-0.3.0/verification-matrix.json` — link P-072 `chunk` (at P5).

## Open questions (resolve at P4)
1. **Result contract** — reuse the incident `L4Output` schema + `parse_bounded` (zero trait change; deterministic mode returns the canned analysis for all 4 actions; render via reused/adapted chunk-#88 report shape) VS a new free-form analysis method/schema (more natural "analysis text", but touches the `interpretation` trait + all 3 impls + the P-073 deterministic runner to emit analysis-shaped + per-action output). **Lean: reuse the incident schema** (leanest, no keystone-runtime churn). → AskUserQuestion.
2. Result-display surface — reuse the chunk-#88 `ReportRenderer` (if L4Output-shaped) vs a lean inline result panel in the modal. Follows from Q1; recommend reuse-if-Option-A, otherwise lean panel. (Plan-level recommendation, not a separate user question unless Q1 is a toss-up.)
