# arch extract

## Relevance
Relevant — chunk adds new TauRPC procedure surface (backend resolver for investigation actions) + new Tauri capability entries, consumed by frontend; uses established `interpretation` crate runtime.

## Constraints
1. New TauRPC procedure must conform to §Standard Contracts envelope (either bare `snake_case` verb for cross-cutting, or dotted `<router>.<verb>` per §Conventions endpoint naming)
2. Must use one of 12 locked workspace crates (§Inherited Defaults — `interpretation`, `pulse-app` are both occupied); no new crate creation
3. Module visibility: cross-crate utilities use `pub(crate)`, contract exposed via designated `pub` module per §Conventions module visibility discipline
4. Error responses crossing the bridge must be `serde::Serialize` via `AppError` enum (§Standard Contracts error response schema Tauri IPC, §Established Decisions keystone)
5. Observability must be aggregate-only (counts + enum tags + numeric values; NO per-service identifiers / prompt content / result bodies — §Cross-cutting Patterns OS notification policy / Self-observation discipline)
6. Deterministic L4 mode is env-var gated (`ANDROMEDA_PULSE_L4_DETERMINISTIC` truthy); honors the established `LlmInferenceRunner` trait (§Established Decisions — trait-based swap boundary)

## Patterns to follow
- TauRPC router registration: one router per module crate, with bare `snake_case` for envelope (e.g., `app_info`, `health`, `ready`) or dotted `<router>.<verb>` for per-crate routers (e.g., `snapshot.generate`, `traces.query`) — per §Conventions endpoint naming and Occupied Resources procedure list
- IPC error handling: module-internal code uses `thiserror` + `TryFrom` / `From` impls at bridge boundary; `anyhow` at top-level + `AppError` wrapper for serialization per §Established Decisions error handling pattern
- Capability JSON: new procedures require entry in `pulse-app/capabilities/` gating the call per §Cross-cutting Patterns Webview IPC capability policy; missing entry silently rejects at runtime
- Observability telemetry: metrics targeting `pulse/stream/*` broadcast channels or `metric.*` tracing events (§Occupied Resources events + §Conventions database entity naming for self-telemetry — no free-form text payloads, cardinality-bounded)

## Anti-patterns to avoid
- No new workspace crates (all 12 are locked per §Occupied Resources — `interpretation` + `pulse-app` are the two relevant occupied names)
- No direct `tokio` deps in module crates beyond the established set (§Inherited Defaults — tokio owned by daemon binary and `[dependencies]` flow toward `pulse-app`)
- No unregistered env vars (all must be listed in §Occupied Resources environment variables; `ANDROMEDA_PULSE_L4_DETERMINISTIC` is pre-registered per P-073 amendment)

## Contract bindings
- **Obs ↔ Tests harness**: aggregate-only metrics emitted to `metric.*` tracing targets (per §Occupied Resources observability entry for existing procedures like `diagnostics.snapshot`); E2E tests inject synthetic telemetry via OTLP `:4317`/`:4318` (standard pattern per §Cross-cutting Patterns test-time telemetry injection)
- **All domains ↔ Workspace crate naming**: the 12 locked crate names in §Occupied Resources are occupied by all other specialists; new code lands only in `interpretation` + `pulse-app` or existing routers
- **Tests ↔ xtask**: new procedure must be added to `xtask/src/main.rs::EXPECTED_PROCEDURES` list (per scope §TauRPC plumbing acceptance anchor)

## Acceptance criteria contributions
- (arch) New TauRPC procedure conforms to §Standard Contracts envelope (either bare `snake_case` verb OR dotted `<router>.<verb>` namespace per §Conventions) with stable `AppError` response envelope
- (arch) New capability entry created in `pulse-app/capabilities/` JSON file gating the procedure access per §Cross-cutting Patterns Webview IPC capability policy
- (arch) Observability is aggregate-only (bounded counts + enum action-id tags + numeric latency/token values; NO per-service identifiers / prompt template strings / analysis result bodies — per §Cross-cutting Patterns self-observation discipline cardinality rules)
- (arch) Deterministic-mode awareness: resolver honors `ANDROMEDA_PULSE_L4_DETERMINISTIC` truthy env var (pre-registered per P-073 amendment 2026-06-28) and consumes the established `LlmInferenceRunner` trait (§Established Decisions, no trait changes)

## Relevant amendment history
- **2026-06-28** — Registered `ANDROMEDA_PULSE_L4_DETERMINISTIC` env var (chunk 2026-06-28-deterministic-env-gated-l4-mode, P-073): truthy gate selecting deterministic L4 runner (canned `L4Output`, no GPU/model) at `pulse-app` boot for reproducible demos/tests. This chunk (P-072) runs investigation actions under deterministic mode for reproducibility (per scope "no GPU/model"). Resolver must check this flag before calling `LlmInferenceRunner::invoke()` or equivalent.