# arch extract

## Relevance
partial — primarily a webview surface consuming already-locked contracts; arch governs the reuse-first backend path (only if research forces a delta), the IPC/stream/capability envelope it reads, and cross-bridge discipline.

## Constraints
1. Reuse the existing IPC surface; mint no new TauRPC namespace unless research proves the three data points unretrievable — new first-party functionality is "new queries/fields inside an existing crate," not a new surface by default (arch §Project Intent §"How new functionality is added" (b); §Occupied Resources is the canonical enumerated procedure list).
2. Any type crossing the bridge is `serde::Serialize` and every `#[taurpc::procedure]` returns `Result<T, AppError>` — so any extended `ConnectionStatePayload`/`ServiceListPayload` field stays Serialize (arch §Cross-cutting Patterns "Cross-bridge data shape"; §Conventions "Error response schema (Tauri IPC)").
3. A new/extended procedure requires BOTH a router registration and a `pulse-app/capabilities/` entry under `pulse:default`, or the call is silently rejected — reusing existing procedures needs no capability change (arch §Cross-cutting Patterns "Webview IPC capability policy").
4. Source the three data points only from occupied resources: `connection.current_state` + `services.list_with_states` + the `ready`/`health` envelope (arch §Occupied Resources §Tauri IPC routes; §Standard Contracts "readiness"/"liveness").
5. Live updates flow over the existing broadcast/Channel contract, not SSE/WebSocket — `pulse://stream/connection-state` already exists; the consumer is in-process (arch §Standard Contracts "Real-time push contract"; §Occupied Resources §Tauri IPC events).
6. Any backend resolver lives in `pulse-app` delegating to library crates (`ingest`/`triage`/`buffer`/`viz`); no library crate depends on `pulse-app`; no new workspace crate; webview consumes only TauRPC-generated `.ts` bindings (arch §Inherited Defaults; §Cross-cutting Patterns "Module dependency direction"; §Conventions "Workspace API style").
7. Buffer-state display ("X min / Y min") derives from `retention_seconds` (seconds, explicit unit; `ANDROMEDA_PULSE_RETENTION_SECONDS`, 300–600 default) — no new unit convention (arch §Conventions "Configuration units"; §Occupied Resources env vars).

## Patterns to follow
1. Connection-surface resolver — `connection.current_state` → `ConnectionApiImpl` returning `ConnectionStatePayload` from `ingest::connection::compute_state()` + `pulse://stream/connection-state` push (arch §Occupied Resources, chunk #59); the worded line and the CARRY reuse this computed state.
2. Services-surface resolver — `services.list_with_states` → `ServicesApiImpl` returning `ServiceListPayload` from `triage::lifecycle::InMemoryServiceRegistry::list` (arch §Occupied Resources, chunk #67); the connected-source count derives here.
3. Readiness envelope as a low-cost health/capacity source — `ready` exposes `ingest_mpsc_capacity_pct` + `broadcast_subscribers` + `duckdb_connection`; check it before minting a rate/buffer field (arch §Standard Contracts).
4. Crate-per-module extension template — add fields/queries inside an existing crate rather than a new crate (arch §Project Intent §"Template patterns").

## Anti-patterns to avoid
1. Do not mint a new TauRPC namespace or new `pulse://stream/*` topic when an existing enumerated procedure/stream already carries the data (arch §Occupied Resources; §Project Intent).
2. Do not introduce SSE/WebSocket for the live line — use the in-process Tauri IPC Channel / existing broadcast topics (arch §Standard Contracts "Real-time push contract").
3. Do not add a new OTLP surface, persistent config, or corpus schema — OTLP is spec-locked and retention is the in-memory ring buffer (arch §Established Decisions); aligns with the chunk's own out-of-scope list.

## Contract bindings
- Standard Contracts envelope ↔ design: the worded line renders design-owned typography tokens over arch-locked payloads — arch defers concrete tokens but locks the data shape (arch §Design Philosophy; §Standard Contracts).
- Standard Contracts envelope ↔ tests: E2E injects synthetic telemetry via OTLP on `:4317`/`:4318` and reads back through the same `connection.*`/`services.*`/`ready` procedures — no test back-door (arch §Cross-cutting Patterns "Test-time telemetry injection").
- Cross-bridge serialize ↔ security: any extended payload field must be `Serialize` and validated at the boundary via `serde`/`TryFrom` (arch §Cross-cutting "Cross-bridge data shape"; §Validation).
- a11y: the line is visible text (screen-reader legible by construction); binding is "must not regress Traces/constellation a11y" — a11y tokens owned by the a11y specialist.

## Acceptance criteria contributions
1. (arch) If a backend delta lands, it reuses/extends `connection.current_state` / `services.list_with_states` / `ready` — NO new TauRPC namespace or `pulse://stream/*` topic unless research proves the three data points unretrievable (arch §Project Intent; §Occupied Resources).
2. (arch) Every type crossing the bridge is `serde::Serialize` and any new/extended procedure returns `Result<T, AppError>` (arch §Cross-cutting Patterns; §Conventions).
3. (arch) Any new/extended TauRPC procedure has a matching `pulse-app/capabilities/` entry under `pulse:default` (arch §Cross-cutting Patterns "Webview IPC capability policy").
4. (arch) Backend resolver code (if any) lives in `pulse-app` delegating to existing library crates; no new workspace crate; frontend consumes only TauRPC-generated bindings (arch §Inherited Defaults; §Conventions).

## Relevant amendment history
- 2026-05-16 — Registry-closure: `connection.current_state` + `pulse://stream/connection-state` (`pulse-app/src/connection_router.rs:46`, `crates/ingest/src/connection.rs:25`, chunk #59 connection FSM). Why relevant: this is the exact connection surface the worded line AND the ConnectionDot CARRY reuse; establishes `compute_state()` as the recency-gated truth source.
- 2026-05-18 — Registry-closure: `services.list_with_states` + `pulse://stream/service-lifecycle` (`pulse-app/src/services_router.rs:61`, chunk #67 service registry + lifecycle FSM). Why relevant: the connected-source count derives from `ServiceListPayload`; must reuse this rather than re-count, keeping the worded count and dot count consistent per the P-067 live-only gate.
