# arch extract

## Relevance
partial — arch governs the read-path contract + workspace placement of any viz-query field addition; the bulk (row ordering, semantic error token, filter control) is design/a11y/frontend, out of arch domain.

## Constraints
- **Workspace placement**: frontend ordering/token/filter work lives in the webview source root `pulse-app/ui/src/` (design-owned tooling); any Rust-side error-count/status/anomaly field addition lives in the `viz` crate that owns the `traces.*` query routers — no sibling crate hosts the trace-row query (per architecture.md §Occupied Resources "Tauri IPC routes → `traces.*` … viz crate" + §Infrastructure Patterns directory structure).
- **Reuse `traces.*`, add no procedure**: the webview runs under `pulse:default`, which permits exactly the enumerated TauRPC procedures; reusing `traces.*` requires no new `pulse-app/capabilities/` entry, whereas a new procedure would need both a router registration and a capabilities entry or it is silently rejected at runtime (per §Cross-cutting Patterns Webview IPC capability policy; matches scope Boundaries).
- **Cross-bridge Serialize**: if the trace-row DTO gains an error/status/anomaly field, that type crosses the TauRPC bridge and MUST be `serde::Serialize`, enforced at compile time by the bridge derive macros (per §Cross-cutting Patterns Cross-bridge data shape).
- **Bindings + naming**: the frontend consumes only TauRPC-generated `.ts` bindings (no manual TS re-declaration); a new Rust field is `snake_case`, its DTO type `UpperCamelCase` (per §Conventions Workspace API style + Variable/function naming).
- **Surface honesty**: the Traces table is part of the dense, chart-first, dark-mode-default developer-tool shell; the semantic error token must respect that color-scheme + density constraint (concrete color/badge tokens are design-owned, not arch's to set) (per §Design Philosophy Developer-tool surface).
- **Don't touch the data spine**: surface the existing signal read-side through the `viz` query layer only — no change to the OTLP receiver contract, DuckDB reserved tables (`spans`, …), or ring-buffer retention; if ordering/filtering is threaded server-side it stays inside the `{ items, total, next_cursor }` list envelope (per §Standard Contracts OTLP receiver + Common response envelope; §Established Decisions Telemetry Retention Surface).

## Patterns to follow
- **viz `traces.*` query-router path** — the established DuckDB buffer → viz query → webview read path; extend it to carry the error/status field rather than opening a new surface (per §Occupied Resources + §Conventions Endpoint naming).
- **Per-crate template shape** — public contract module + `thiserror` enum + `Serialize` DTO + TauRPC router mounted by `pulse-app`; a field addition extends the existing DTO, it does not create a new crate or router (per §Project Intent Template patterns).
- **TauRPC-generated `.ts` bindings** — the frontend↔Rust type contract is auto-generated from the Rust command surface; regenerate and consume, never hand-declare (per §Conventions Workspace API style).
- **Real-time push contract (conditional)** — if the table live-updates, spans arrive over `pulse://stream/spans` as binary Arrow IPC on a Tauri `Channel`, not JSON (per §Standard Contracts Real-time push contract).

## Anti-patterns to avoid
- **No new TauRPC procedure / no new `pulse:*` capability** for this feature — reuse `traces.*`; a procedure added without a `pulse-app/capabilities/` entry is silently rejected at runtime (§Cross-cutting Patterns Webview IPC capability policy).
- **No mutation of ingest/buffer/OTLP to surface the signal** — no new DuckDB table/column, no OTLP-contract change, no retention change; the signal already exists in the spine and is surfaced read-side (scope Non-goals; §Established Decisions Telemetry Retention Surface).
- **No manual TypeScript re-declaration** of the row DTO — consume the generated bindings only (§Conventions Workspace API style).

## Contract bindings
- **arch ↔ design**: the `viz` row DTO's error/status field is the binding point design's row renderer + semantic error token consume; arch fixes the dark-mode-default/dense surface envelope, design owns the concrete color+badge tokens (§Design Philosophy).
- **arch ↔ tests**: §Standard Contracts + §Cross-cutting Patterns Test-time telemetry injection — E2E fixtures inject a mixed dataset via OTLP on `:4317`/`:4318` and read back through `traces.*`; the scope val-1 affordance-honesty check drives a REAL DOM event on the filter control. A new `Serialize` DTO field changes the fixture-observed wire shape.
- **arch ↔ a11y**: the cross-bridge status field carries the signal; a11y governs its non-color-only encoding (badge/icon/text, a11y-plan SC 1.4.1 per scope).

## Acceptance criteria contributions
- (arch) Any Rust-side trace-row field lives in the `viz` crate per §Occupied Resources query-router ownership; frontend changes stay under `pulse-app/ui/src/`.
- (arch) No new TauRPC procedure and no new `pulse-app/capabilities/` entry — the feature reuses `traces.*` (§Cross-cutting Patterns Webview IPC capability policy).
- (arch) Any DTO field crossing the bridge is `serde::Serialize` and consumed via TauRPC-generated `.ts` bindings only (§Cross-cutting Patterns Cross-bridge data shape).
- (arch) No change to the OTLP receiver contract, DuckDB reserved tables, or ring-buffer retention — surfacing is viz-query read-side only (§Standard Contracts + §Established Decisions Telemetry Retention Surface).

## Relevant amendment history
(none) — no entry in architecture-amendments.md touches the `viz`/`traces.*` query path or the webview Traces table. Adjacent problem-surfacing amendments exist but are excluded by this chunk's non-goals (no detection-logic change, reuse `traces.*`): `incidents.*` + `pulse://stream/incidents` (chunk #78, 2026-05-23), `pulse://stream/attention-cues` (chunk #62, 2026-05-17), and `services.list_with_states` + `pulse://stream/service-lifecycle` (chunk #67, 2026-05-18) all surface problems through separate triage-domain namespaces, not the Traces read path this chunk touches.
