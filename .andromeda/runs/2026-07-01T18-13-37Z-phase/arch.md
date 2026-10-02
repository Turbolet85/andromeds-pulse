# arch extract

## Relevance
Relevant — refines the visibility and recency-label semantics of the pre-registered `services.list_with_states` resolver and `pulse://stream/service-lifecycle` event.

## Constraints
1. **Workspace placement** — code resides in `crates/triage` (library) + `pulse-app/src/services_router.rs` (binary) per architecture.md §Inherited Defaults (module boundary rules).
2. **Module visibility discipline** — `InMemoryServiceRegistry` contract exposed via `pub`; internal state remains `pub(crate)` per §Conventions module visibility discipline.
3. **IPC procedure shape** — `services.list_with_states` conforms to `<router>.<verb>` dotted namespace per §Conventions endpoint naming; TauRPC-derived response must be `Serialize`-able.
4. **Response envelope conformance** — `ServiceListPayload` follows the standard paginated-list shape (items, total, next_cursor) per §Standard Contracts common response envelope.
5. **Broadcast event lock** — `pulse://stream/service-lifecycle` is the canonical event name (registered chunk #67 §Occupied Resources); only this event emits state transitions; no new events.
6. **Corpus table stability** — `service_registry` SQLite schema in `corpus/corpus.db` is immutable per §Occupied Resources; the chunk refines filtering/display only, not schema.
7. **Cross-crate dependency direction** — `triage` library crate exposes contract; `pulse-app` binary consumes via resolver, following the DAG rule (§Cross-cutting Patterns module dependency direction).

## Patterns to follow
1. **Resolver shape** — mirror `incidents.list_active` / `incidents.get_report` (chunk #88) pattern: accept optional filter params, return `Result<ServiceListPayload>`, broadcast on state change via the pre-existing state machine (not the resolver itself).
2. **State classification** — reuse the 7-state lifecycle enum + liveness logic from `crates/triage/src/lifecycle/state_machine.rs`; the chunk adds only the filter condition (which states = "live").
3. **Truthful recency labels** — consult actual `last_seen_unix_nano` from the registry entry, never a boot-time-reset timestamp; mirror the timestamp-surfacing pattern in `incidents.get_report` (chunk #88 Occupied Resources).
4. **Broadcast fan-out** — the state machine broadcasts when service state changes; the resolver reads the current snapshot. Test via the same OTLP-injection E2E pattern used for buffer queries (§Cross-cutting Patterns test-time telemetry injection).

## Anti-patterns to avoid
1. **Do NOT fabricate liveness.** Zero telemetry since boot → no live dot, no "just now" label, even if persisted. This is the cardinal sin (§Design Philosophy "every byte of telemetry stays on the developer's machine" — UI must be truthful).
2. **Do NOT modify the 7-state lifecycle machine or `service_registry` schema** — filtering is strictly at the resolver/display layer per scope boundaries.
3. **Do NOT invent IPC procedures or event names** beyond the locked pair `services.list_with_states` + `pulse://stream/service-lifecycle` (§Occupied Resources).

## Contract bindings
- **Buffer ↔ Triage spine** — untouched per scope boundaries (§Design Philosophy "working data spine").
- **Triage ↔ UI-bridge (TauRPC)** — resolver at `pulse-app/src/services_router.rs` is the binding site; the chunk refines filtering logic here.
- **Tests ↔ Buffer/Ingest** — E2E tests inject synthetic service telemetry via OTLP to `:4317`/`:4318` (§Cross-cutting Patterns test-time telemetry injection).

## Acceptance criteria contributions
1. **(arch) Code lives in `crates/triage` + `pulse-app/src/services_router.rs` per workspace boundary rules (arch §Inherited Defaults).**
2. **(arch) `services.list_with_states` response conforms to standard Tauri IPC paginated-list envelope and returns only currently-live `ServiceListItem` entries (arch §Standard Contracts common response envelope).**
3. **(arch) Recency labels (`last_seen_unix_nano`) reflect actual service last-observed span time, never boot-time reset (arch §Design Philosophy — UI truth).**
4. **(arch) With zero live telemetry the constellation/list shows zero live services; persisted entries hidden or explicitly marked historical/inactive (scope acceptance condition).**

## Relevant amendment history
**2026-05-18 — Acknowledge `services.list_with_states` + `pulse://stream/service-lifecycle`** (chunk #67)
- Registered the IPC procedure + broadcast event in §Occupied Resources.
- This chunk refines the filtering behavior and recency-label semantics of these already-locked surfaces.
- No schema or decision changes — resolver/display behavior only.
