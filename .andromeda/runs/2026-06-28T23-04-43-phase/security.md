# security extract

## Relevance
partial — new TauRPC procedure + observability, no threat-boundary expansion

## Constraints
1. Any new TauRPC procedure MUST add a capability JSON entry in `pulse-app/capabilities/` + register in `xtask` EXPECTED_PROCEDURES + wire xtask drift-check per plan §API Security (TauRPC capability authorization) + bootstrap §dep-security-ci-gate
2. Analysis result payload must NOT contain prompt templates, OTLP attribute values, or per-service identifiers in any observable metric/log per plan §Logging & Monitoring (NEVER-log list) + §Anti-Patterns §Logging
3. Procedure arguments that carry snapshot/telemetry context inherit validation from snapshot.generate boundary (already-trusted data per plan §Input Validation TauRPC boundary)
4. Error responses MUST use AppError enum variants (Validation / NotFound / Internal / Plugin / Storage / Ingest) per plan §Error Handling + Conventions
5. ANDROMEDA_PULSE_L4_DETERMINISTIC env var is input-validated at startup per plan §Input Validation (CLI / env var inputs row); procedure may gate deterministic-mode behavior to this pre-validated bool per amendment 2026-06-28-deterministic-env-gated-l4-mode

## Patterns to follow
1. Snapshot context (already validated by snapshot.generate) may pass to LLM analysis path without re-validation; document the delegation at procedure boundary per existing TauRPC input-validation patterns
2. Aggregate cardinality discipline per existing obs patterns: bounded enum tags (action_id, status) + numeric counts, never result bodies or prompt content

## Anti-patterns to avoid
1. Do NOT log OTLP attribute values, prompt templates, or per-service identifiers (plan §Logging & Monitoring NEVER-log list)
2. Do NOT attempt prompt-injection sanitization (chunk scope explicit: deferred post-v1 per intent §5)

## Contract bindings
- **Observability** ↔ obs §PII Scrubbing + cardinality (aggregate-only, no leaked attribute values or prompt content)
- **TauRPC capability** ↔ arch §Occupied Resources (new procedure entry + capability binding)
- **Tests** ↔ test harness (e2e under P-073 deterministic-L4)

## Acceptance criteria contributions
- (security) New TauRPC procedure has matching capability JSON + xtask EXPECTED_PROCEDURES entry, drift-checked in CI per §dep-security-ci-gate
- (security) Grep verifies NO prompt / result / OTLP-attribute / per-service fields in new observability metrics
- (security) Error paths return AppError, not internal error details

## Relevant amendment history
**2026-06-28 — Register ANDROMEDA_PULSE_L4_DETERMINISTIC in §Input Validation:** Env var added by P-073 (deterministic-mode gating); registered in Input Validation boundary with bounded truthy-parse (1|true|yes, default false). This chunk uses the env var to gate LLM result reproducibility; no new env-var registration needed.