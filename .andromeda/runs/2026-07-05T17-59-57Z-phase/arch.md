# arch extract

## Relevance
Partial — a webview legibility feature reusing an existing arch-reserved data contract; arch supplies guardrails (no-new-namespace, capability policy, dark-mode/density) while the substance is design + a11y.

## Constraints
- Dot identity + health data come from the existing `services.list_with_states` procedure (pulse-app crate, `ServicesApiImpl` → `ServiceListPayload` from `crates/triage::lifecycle::InMemoryServiceRegistry::list`, chunk #67) — per architecture.md §Occupied Resources. This is the reserved surface the scope's low-cost path reuses.
- Frontend consumes ONLY TauRPC-generated `.ts` bindings; no manual TS re-declaration of the payload — per architecture.md §Conventions (Workspace API style).
- Main webview runs under `pulse:default`, which permits EXACTLY the enumerated TauRPC procedures; a NEW procedure needs both a router registration AND a `pulse-app/capabilities/` entry or it is silently rejected — per architecture.md §Cross-cutting Patterns (Webview IPC capability policy). Reusing `services.*` (already reserved) needs no capability change.
- If the escape hatch adds a health/severity field to `ServiceListItem`, that type crosses the bridge and must be `serde::Serialize` and stay inside the reserved `services.*` router — per architecture.md §Cross-cutting Patterns (Cross-bridge data shape) + §Conventions.
- Dot color + Halo must honor the dark-mode default color scheme and the dense, low-chrome developer-tool surface; concrete color/motion tokens are design-owned but arch-constrained — per architecture.md §Design Philosophy (Developer-tool surface).
- If the design chooses live-updating dots over the polling hook, live push uses the Tauri `Channel` + binary-Arrow contract on the reserved `pulse://stream/service-lifecycle` topic (chunk #67) — per architecture.md §Standard Contracts (Real-time push) + §Occupied Resources.

## Patterns to follow
- Reuse the resolver payload fields already surfaced (`name`, `state: ServiceLifecycleState`, `priority_tier`) rather than minting a new query — per architecture.md §Occupied Resources (chunk #67; `priority_tier` chunk #91).
- Rust router → TauRPC-generated TS → webview hook is the established data path (`use-service-constellation.ts` polls the resolver) — per architecture.md §Conventions + §Inherited Defaults (API style — internal IPC).
- Any additive field on an existing reserved payload stays within the owning crate's contract module; cross-crate leakage is barred via `pub`/`pub(crate)` discipline — per architecture.md §Conventions (Module visibility) + §Cross-cutting Patterns (Module dependency direction).

## Anti-patterns to avoid
- Do NOT introduce a new TauRPC namespace/procedure or new Tauri capability (scope default is no-new-namespace); a new procedure without a `pulse-app/capabilities/` entry is silently rejected at runtime — per architecture.md §Cross-cutting Patterns (Webview IPC capability policy).
- Do NOT invent an ad-hoc frontend↔backend wire or hand-declare payload types in TS — per architecture.md §Conventions.
- Do NOT add a new lifecycle state, DuckDB/corpus table, or schema for dot health (also a scope boundary) — those name sets are reserved — per architecture.md §Occupied Resources (DuckDB / Corpus SQLite schema names).

## Contract bindings
- arch ↔ design: health/severity color scale + Halo State Pulse mapping live in `.andromeda/design-system.md` (design-owned) but are bound by arch's dark-mode-default + density constraint (§Design Philosophy).
- arch ↔ a11y: the severity value that a11y's non-color cue (SC 1.4.1) and contrast (SC 1.4.11) act on is carried by arch's `services.list_with_states` payload (§Occupied Resources) — arch owns the transport, a11y owns the encoding.
- arch ↔ triage (data origin): `ServiceListPayload` is produced from `crates/triage::lifecycle::InMemoryServiceRegistry::list`; any new health field originates there and flows through the pulse-app `services_router` (§Occupied Resources).

## Acceptance criteria contributions
- (arch) Dot identity + health sourced from the existing `services.list_with_states` procedure; no new TauRPC namespace unless /implement proves the health field absent from `ServiceListItem` (arch §Occupied Resources).
- (arch) If a health field is added, it lands inside the reserved `services.*` router (pulse-app), is `serde::Serialize`, and needs no new `pulse-app/capabilities/` entry (arch §Cross-cutting Patterns).
- (arch) Webview consumes only TauRPC-generated `.ts` bindings — no hand-written payload types (arch §Conventions).
- (arch) Dot color + Halo respect the dark-mode default + dense developer-tool surface (arch §Design Philosophy).

## Relevant amendment history
- 2026-05-18 — Registry-closure acknowledging `services.list_with_states` (`pulse-app/src/services_router.rs:61`) + `pulse://stream/service-lifecycle` (`crates/triage/src/lifecycle/broadcast.rs:16`), chunk #67. This is the exact data source this chunk consumes — the reserved query surface + live topic were legitimized then. Its predecessor P-067 (live-only dot set) and the chunk #91 `priority_tier` field addition do NOT appear in this append-only history, confirming that additive fields within the already-reserved `services.*` surface are the low-cost, no-arch-amendment path this scope should prefer.
