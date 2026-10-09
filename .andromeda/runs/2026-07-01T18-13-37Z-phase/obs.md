# obs extract

## Relevance
Relevant — chunk affects service liveness telemetry + observability surfaces (constellation, list) + heartbeat/health contract.

## Constraints
1. Per §1: Service identity (`com.andromeda.pulse`) + `ServiceLifecycleState` (7 states) are instrumentation anchors; state transitions emit structured logs (§2: "boundary calls + state transitions + errors").
2. Per §3 amendment 2026-05-08: Heartbeat ticks (async, 15s, retroactive stall detection >45s) and health command (sync active polling) are complementary, not redundant — must not conflate; the chunk's liveness classification affects both.
3. Per §10 critical path P5: `ipc.health.check` span + response shape (`health.status` + `health.subsystems`) bind to tests; chunk must preserve/extend the contract, never break the schema.
4. Per §1 logging vector 5: Never log full registry state or query parameters; emit `service_count_live`, `service_count_historical`, `state_transition_reason` metadata only, not the registry payload.
5. Per §1 Telemetry surfaces "desktop-webview": the constellation renders dots from `services.list_with_states`; only live services shown → instrument the liveness classifier.
6. Per §3 Service identity: state-transition logs carry `service.name` / `service.version` / `deployment.environment` via subscriber default fields.

## Patterns to follow
1. Per §1 P5: span `ipc.health.check` wraps the check; fields `health.status`, `health.subsystems`, `service_count_live`, `service_count_total` — agent-readable, no state dumps.
2. Per §2: state-transition logs emit `tracing::info!(target: "registry.state_transition", …)` with required fields; NOTE cross-check the aggregate-only cardinality discipline (per `.claude/rules/observability.md` 2026-05-17 session-84: NO per-service `service_name`/`scope_id` in self-observation events — aggregate counts + enum tags only).
3. Per §1 heartbeat ticks: `registry.tick` every 15–30s with `service_count_live`, `service_count_silent`, `last_seen_skew_max_seconds`.

## Anti-patterns to avoid
1. No boot-time reset of `last_seen` on corpus restore — preserve actual last-observed time; a prior-session service reads "earlier session", not "just now".
2. No raw registry payload in logs (vector 5) — metadata only.
3. No liveness-label ambiguity: constellation UI and `health.subsystems` show identical liveness classification; a service hidden from the constellation must not appear as live in health.

## Contract bindings
- **Test harness binds to tests §3**: `health` response shape must pass agent-driven assertions; the liveness-filter logic must not break the schema.
- **Critical path P5 binds to §10**: `ipc.health.check` span + health response are observability surfaces; must remain traced + in contract.

## Acceptance criteria contributions
1. "(obs) Service state transitions logged in NDJSON with required fields (aggregate: state_from/state_to/reason/service_count_live); no raw registry dump; no per-service identifier in self-observation events."
2. "(obs) Recency honest: `last_seen` for a re-observed service reflects actual span arrival; a never-re-observed restored service is not labeled 'just now'."
3. "(obs) Health response shape preserved and consistent with the constellation's liveness classification."
4. "(obs) Zero-telemetry test: boot with an empty/persisted registry + zero live spans → constellation shows zero live services; no ambiguous 'last seen · just now'."

## Relevant amendment history
- **2026-05-08 — Heartbeat ticks vs health command complementarity**: ticks (async, retroactive) and health (sync polling) are complementary; the liveness classifier must be consistent across both surfaces; they must agree on current liveness. (Applies if the chunk emits liveness to health / ticks.)
