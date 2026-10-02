# security extract

## Relevance — partial
New CLI input vector (`ANDROMEDA_PULSE_*` env var) gating runner selection, exercising the existing env-var validation boundary. Canned L4Output is first-party deterministic content consumed by the existing chunk #92 path; no new threat boundaries.

## Constraints
1. Per security-plan §Input Validation: the new env var MUST validate at startup via enum parse / boolean literal (following `ANDROMEDA_PULSE_MCP_ENABLED`); invalid values fail cleanly; no unbounded string accepted.
2. Per security-plan §Threat Model (user-content): the canned L4Output MUST be schema-valid + MUST NOT contain secrets, real telemetry, or host-app context (deterministic first-party content only).
3. Per security-plan §Error Handling: if the deterministic runner can fail at the IPC boundary, errors collapse to stable `AppError` variants (no stack traces / impl details).
4. Per security-plan §Logging & Monitoring: any runner-selection / output logging uses `tracing` JSON; canned output not logged verbatim.

## Patterns to follow
1. Per security-plan §Input Validation: validate via `TryFrom<&str>`/`FromStr` into an enum (e.g. `RunnerMode::Deterministic | Real`), reusing the `ANDROMEDA_PULSE_LOG_LEVEL` / `_MCP_ENABLED` approach.
2. Per security-plan §Threat Model: deterministic runner is another `LlmInferenceRunner` impl; honor the trait unchanged; chunk #92 consumes output schema unchanged.
3. Per security-plan §API Security: gate operates at the `pulse-app` startup boundary (no TauRPC procedure / capability JSON change).

## Anti-patterns to avoid
1. Per security-plan §Input Validation (Input bans): the env var MUST NOT be string-interpolated into DuckDB queries, log fields, or file paths — bounded-enum flag only.
2. Per security-plan §Logging & Monitoring: canned L4Output content MUST NOT be logged if mistakable for real instrumented traces.

## Contract bindings
- **security ↔ chunk #92 incident creation**: canned L4Output must be schema-valid + consumable unchanged; a drop-in for real inference output at the contract level.

## Acceptance criteria contributions
1. (security) Startup validation gate: env var validates via enum parse; invalid rejected cleanly; no unbounded string.
2. (security) Schema validity: canned L4Output matches chunk #92's contract; integration test produces a real incident with deterministic severity.
3. (security) No PII in canned output: all stub fields are first-party constants/metadata (verified by inspection).

## Relevant amendment history
- **2026-06-28 — `opentelemetry-stdout` → `tracing-only` self-observation** (security-plan-amendments.md): any runner-selection/output logging uses `tracing` JSON → `~/.andromeda-pulse/logs/agent-latest.jsonl`. No prior amendments touch the L4 path; orthogonal to chunks #68–#77.
