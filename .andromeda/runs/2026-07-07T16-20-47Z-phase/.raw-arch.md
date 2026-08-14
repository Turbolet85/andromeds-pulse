# arch extract

## Relevance
Partial — the core re-poll (React hook lifecycle, filter/sort preservation) is design-owned; arch governs the IPC/capability reuse boundary and the conditional Rust `viz`-crate CARRY.

## Constraints
- **Reuse the existing viz query router — no new IPC surface.** The `traces.*` query routers live in the `viz` crate and are already reserved; the re-poll re-invokes `traces.query` (the scope's `viz.query.traces`), read-only and unmodified (per arch §Occupied Resources Tauri IPC routes + §Conventions Endpoint naming). Note: arch's canonical label is `traces.query` (router=`traces`, verb=`query`); the scope's `viz.query.traces` refers to the same read-only traces query.
- **Capability reuse under `pulse:default`.** The main webview runs under `pulse:default`, which permits exactly the enumerated `<router>.<verb>` procedures and nothing else; re-polling an already-permitted procedure requires no new `pulse-app/capabilities/` entry (per arch §Cross-cutting Patterns Webview IPC capability policy + §Occupied Resources Tauri capability identifiers).
- **Workspace crate boundary for the conditional CARRY.** If the pagination/cursor fix activates, it lands in the reserved (LOCKED) `viz` crate — no new crate, no sibling dep beyond declared contract (per arch §Occupied Resources Cargo workspace crate names + §Inherited Defaults Module boundaries + §Cross-cutting Patterns Module dependency direction).
- **Paginated-list envelope + opaque cursor.** `traces.query` wraps results in `{ items, total, next_cursor }` with an opaque cursor (`null` = no more); the CARRY's `next_cursor` keying (start-time `ts_unix_nano` vs ORDER BY `end_time`) must stay inside this contract (per arch §Standard Contracts Common response envelope + §Conventions Timestamp handling, where start time is the `BIGINT ts_unix_nano` sibling to `TIMESTAMPTZ`).
- **Cross-bridge shape + error schema (only if Rust is touched).** The procedure returns `Result<T, AppError>` and every type crossing the TauRPC bridge must be `serde::Serialize`; `AppError` variants are stable (per arch §Conventions Error response schema (Tauri IPC) + §Cross-cutting Patterns Cross-bridge data shape).

## Patterns to follow
- **Query-router read-back for verification.** Prove empty→populated by injecting synthetic spans over the OTLP surface and reading buffer state back through the `traces.*` router — the same path external SDKs and E2E tests use (per arch §Cross-cutting Patterns Test-time telemetry injection).
- **Two live-data shapes to the webview.** Live push exists as a broadcast stream via Tauri Channel + binary Arrow IPC (`pulse://stream/spans`); the scope instead picks plain IPC query re-poll (mirroring the constellation ~1s). Either is legitimate, but a stream, if ever used, must be the Channel+Arrow contract, not SSE/WebSocket (per arch §Standard Contracts Real-time push contract).
- **Crate template shape (if CARRY lands in viz).** Public contract module as the only `pub` surface, `pub(crate)` internals, `thiserror`-derived error enum (per arch §Project Intent Template patterns + §Conventions Module visibility discipline).

## Anti-patterns to avoid
- **Do not mint a new TauRPC procedure or new capability for a re-poll** — reuse `traces.query` / `pulse:default` (per arch §Cross-cutting Patterns Webview IPC capability policy; scope confirms both are out of scope).
- **Do not add an in-process / test-mode ingest back-door** to simulate spans for the empty→populated check; drive telemetry through OTLP `:4317`/`:4318` (per arch §Cross-cutting Patterns Test-time telemetry injection).
- **Do not leak viz internals cross-crate or add a sibling dep** to fix the cursor — keep any CARRY change inside the viz contract (per arch §Cross-cutting Patterns Module dependency direction).

## Contract bindings
- **arch ↔ design**: the re-poll lifecycle (hook, interval start/stop on mount/unmount, no leaked timers, filter/sort-state preservation) is design-owned frontend; arch owns the IPC surface + `pulse:default` capability it calls (arch §Cross-cutting Patterns Webview IPC capability policy; focus guide — webview files follow the design convention).
- **arch ↔ tests**: the `traces.*` query router + OTLP inject surface is the E2E read-back path for the empty→populated acceptance (arch §Cross-cutting Patterns Test-time telemetry injection).
- **arch(envelope) ↔ viz(impl)**: if the CARRY activates, `next_cursor` keying must remain inside the `{ items, total, next_cursor }` opaque-cursor contract (arch §Standard Contracts).

## Acceptance criteria contributions
- (arch) Re-poll reuses `traces.query` (`viz.query.traces`) under `pulse:default`: zero new TauRPC procedures, zero new `pulse-app/capabilities/` entries, zero capability drift (arch §Cross-cutting Patterns Webview IPC capability policy).
- (arch) If the CARRY activates, the cursor fix lands only in the `viz` crate — no new workspace crate, no new cross-crate dep (arch §Inherited Defaults Module boundaries + §Occupied Resources crate names).
- (arch) If Rust is touched, `traces.query` still returns `{ items, total, next_cursor }` (opaque cursor) and `Result<T, AppError>` with `serde::Serialize` types across the bridge (arch §Standard Contracts + §Cross-cutting Patterns Cross-bridge data shape).
- (arch) Empty→populated is verified by injecting synthetic spans over OTLP `:4317`/`:4318` and reading back through `traces.*` — no in-process bypass (arch §Cross-cutting Patterns Test-time telemetry injection).

## Relevant amendment history
- No amendment touches the viz `traces.*` query router or its pagination/cursor path. Nearest-adjacent: **2026-05-09 — Acknowledge `streams.*` namespace** (`streams.subscribe_spans` → `pulse://stream/spans`, chunk #23) — the live-span-stream path that the scope's periodic re-poll is an explicit alternative to (scope §Observed gap notes "no stream subscription").
- Active-memory note: the recurring "D3 capability-drift closure" amendments (e.g. 2026-05-18, 2026-05-24, 2026-06-04) establish that every *new* procedure/capability/env-var is registered post-land. This chunk reuses an existing procedure and (even under the CARRY) only modifies existing viz query internals — it mints no new arch resource, so, matching the P-079/P-080 frontend-bug precedent, **no arch amendment is expected**.
