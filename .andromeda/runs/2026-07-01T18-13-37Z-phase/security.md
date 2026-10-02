# security extract

## Relevance
Partial — chunk modifies an existing TauRPC procedure surface (data contract + filtering logic) within the already-established IPC boundary; no new input surfaces, no new secrets, no new external network exposure.

## Constraints
1. `services.list_with_states` TauRPC procedure must remain capability-gated via `pulse:default` (per security-plan §API Security TauRPC capability authorization) — no removal of capability enforcement.
2. Service metadata displayed must not leak corpus access internals, filesystem paths, or OTLP attribute values incidentally captured during instrumentation (per security-plan §Logging & Monitoring NEVER-log list).
3. Filtered `ServiceListItem[]` responses are serialized via the existing `serde` schema; no unvalidated struct shape changes that bypass the TauRPC argument deserialization layer (per security-plan §Input Validation TauRPC bridge row).
4. Lifecycle state classification logic (determining which `ServiceLifecycleState` values count as "live") must be deterministic and non-racy when called from concurrent IPC requestors; state transitions in `InMemoryServiceRegistry` remain thread-safe per existing design (per security-plan §Data Protection concurrent access discipline).

## Patterns to follow
1. Service lifecycle filtering — read `ServiceLifecycleState` enum from registry at procedure-call time; do not cache filtered lists across IPC calls (stale state lie per chunk scope observed gap).
2. Timestamp/recency accuracy — carry forward `last_seen_unix_nano` as-is from corpus restoration, never reset to boot time; filtering decision uses this actual timestamp, not boot-time fabrication.
3. Webview filter integration — filtering can occur backend / frontend / hybrid; whichever path chosen must NOT add new IPC calls that re-query registry per filtered state subset (avoid TOCTOU race between call and display).

## Anti-patterns to avoid
1. Do not add new serializable enum variants to `ServiceListItem` or `ServiceLifecycleState` without corresponding security-plan §Input Validation boundary + capability audit — scope says this is NOT a state-machine rebuild, so variants must be pre-existing.
2. Do not log the full service metadata / IPC response payload when a filtering decision is made (per §Logging & Monitoring NEVER-log list).
3. Do not introduce a separate "display state" enum distinct from the canonical `ServiceLifecycleState` that could diverge at runtime (single source of truth per chunk scope boundary).

## Contract bindings
- **API Security** — `services.list_with_states` resolver must honor existing `pulse:default` capability gate; backend filtering is a procedure-internal detail, not a new procedure.
- **Data Protection** — filtered data originates from encrypted corpus (`service_registry` cell-level AES-256-GCM); filtering does not create a new plaintext cache.
- **Logging & Monitoring** — filtering must not log OTLP attributes incidentally embedded in `ServiceListItem` metadata; recency-label fix must not emit a "timestamp reset on boot" flag into logs.

## Acceptance criteria contributions
1. **(security) Filtering respects existing capability gate:** `services.list_with_states` remains enforced by `pulse:default`; no new procedures; CI xtask drift-check continues to pass (security-plan §API Security).
2. **(security) No timestamp leakage on corpus restore:** `set_state_on_corpus_restore` does NOT stamp `last_seen_unix_nano = boot_time`; corpus timestamps preserved; recency label reads "last seen: {prior-session}", never "just now" for a restored-but-unobserved service.
3. **(security) Filtered response does not log sensitive service metadata:** integration test confirms the filtering code path emits no OTLP attribute values / corpus paths / raw payloads to `~/.andromeda-pulse/logs/agent-latest.jsonl` (only lifecycle-state enum values).

## Relevant amendment history
**(none)** — No amendments touch the constellation/service-registry display or recency-label surfaces. Adjacent (window-geometry / env-var) amendments do not apply; the 2026-05-22 corpus encryption amendment is background for at-rest protection of `service_registry`, not a display-side rule.
