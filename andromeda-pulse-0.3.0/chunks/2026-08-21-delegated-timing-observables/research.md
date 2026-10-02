# Codebase Research — 2026-08-21-delegated-timing-observables

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 14 · **Code-graph queries:** 1

## Files inspected
- `crates/ui-bridge/src/telemetry.rs` (1–130) — the whole existing frontend→backend telemetry surface: `TelemetryApi` trait (one method), `FrameDurationInput`, `validate_duration_ms`, the level-gated emit.
- `pulse-app/src/incidents_router.rs` (548–570) — the P-037 backend-measured precedent; `metric.report.render_ms` emitted beside a richer `incidents.get_report.request` event.
- `pulse-app/src/observability.rs` (130–200, 2245–2262) — `AllowList::production()` registrations and the `for_target` resolution chain.
- `pulse-app/src/main.rs` (199–219, 985–1009) — the router merge chains, production *and* the `emit_taurpc_bindings` test.
- `xtask/src/main.rs` (1045) — `EXPECTED_PROCEDURES`.
- `pulse-app/capabilities/default.json` (full) — the actual granted permission set.
- `pulse-app/ui/src/canvas/frame-metrics.ts` (1–70) — the webview half of the existing bridge.
- `pulse-app/ui/src/halo/HaloCanvas.tsx` (grep) — severity→hue inside the rAF loop.
- `pulse-app/ui/src/hooks/use-findings.ts` (grep) — the ~1s PULL re-poll.
- `pulse-app/ui/src/bindings/index.ts` (364–381, 132, 441, 478) — wire shapes.
- `docs/v0_2_0/capability-verification-matrix.json` — the three caps' scenarios + grep anchors.

## Graph impact (from the code-graph query)
- **`record_frame_ms`** — 3 callers, **all three are its own colocated tests** (`crates/ui-bridge/src/telemetry.rs:313,336,356`). Zero production Rust callers: the webview is the only real caller, over IPC. An additive sibling method therefore has **no Rust-side caller-threading blast radius at all**; the threading files are the registration sites listed under *Files to modify*, not call sites.

## Patterns detected
- **The webview→backend timing bridge already exists end-to-end** (`pulse-app/ui/src/canvas/frame-metrics.ts` → `crates/ui-bridge/src/telemetry.rs:99` → `metric.webgpu.frame_duration_ms`). It clamps client-side *and* validates server-side, level-gates the emit, and maps loose strings into bounded enums with a documented defensive fallback rather than an `"unknown"` variant (frame-metrics.ts, `normalizeWgpuBackend`).
- **The existing bridge already emits from the reduced-motion branch.** `pulse-app/ui/src/canvas/CanvasContainer.test.tsx:154` pins "invokes recordFrameMs from the reduced-motion static-paint branch (stable 4-field shape)" — so the design/a11y constraint that a mark must not be silenced by `prefers-reduced-motion` is already solved by this pattern, not an open problem.
- **Bounded-enum discriminants on a single input struct** (`FrameDurationInput`: `duration_ms: f64` + `WgpuBackend` + `WebviewBackend` + `TimingMethod`, `crates/ui-bridge/src/telemetry.rs:62–67`) — one procedure already carries three closed enums.
- **Halo hue is computed inside the rAF loop from a ref** (`HaloCanvas.tsx:145–147`: `cumulativeSeverityRef.current` → `severityToHueFraction` → `lchInterpolate`), i.e. the hue update is observable at a specific line inside the frame loop, not at a React commit.
- **The findings counter is PULL with a ~1s `setInterval` re-poll** (`use-findings.ts:9–15,81`), explicitly documented as a silent background refresh whose public API would survive a later swap to push.
- **Four sibling allowlist-guard test files already exist** under `pulse-app/tests/` (`unit_observability_allowlist_{feed,corpus_key,bootstrap_window,incident_diagnostics}.rs`) — the placement precedent is unambiguous.

## Conventions to follow
- **Exact allowlist leaf per target, and it is load-bearing here** (`pulse-app/src/observability.rs`): a bare **`"metric"`** key IS registered, carrying exactly `["value", "unit", "module"]`. Because `for_target` falls back to the first `.`-segment (`observability.rs:2255`), a new `metric.foo.bar` target with no exact leaf resolves to that 3-field set — `value` survives, and **every other field is redacted**. This is the measured mechanism behind the recorded l1a backlog, which is **still open at HEAD**: `metric.pipeline.l1a.query_count_total` / `query_latency_p99_milliseconds` / `q7_timeout_count_total` / `q7_fallback_count_total` are emitted at `crates/triage/src/baseline/sql.rs:484,494,716,722` with no leaf among the 34 registered `metric.*` leaves. Any observable carrying a surface discriminant MUST have its own exact leaf or the discriminant silently disappears.
- **pulse-app probes live in `pulse-app/tests/*.rs`** — never a colocated `mod tests` (`[lib] test = false`).
- **Bounded-parse + explicit rejection + named unit test per rejection** (`validate_duration_ms`, `crates/ui-bridge/src/telemetry.rs:77–90`), returning `AppError::Validation { field, reason }`.
- **Level-gate the emit** (`tracing::enabled!(tracing::Level::INFO)`) before formatting a payload on a frame-rate path.

## New files to create
- `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs` — field-completeness guard: each new target resolves to its own exact leaf enumerating every emitted field (mirrors the four existing sibling guards).

## Files to modify
- `crates/ui-bridge/src/telemetry.rs` — the new procedure + its input struct + bounded-enum discriminant + validation + the emit arms. Colocated `#[cfg(test)] mod tests` extends here (library crate).
- `pulse-app/src/observability.rs` — one exact allowlist leaf per new target, enumerating every field.
- `xtask/src/main.rs` — add the new procedure to `EXPECTED_PROCEDURES` (currently line 1045 holds the sole `telemetry.frontend.*` entry).
- `pulse-app/ui/src/bindings/index.ts` — regenerated, not hand-edited (`ARGS_MAP` line 441 + the proxy type at 478).
- `pulse-app/ui/tests-a11y/helpers/mock-tauri.ts` — the `TauRPC__<router.path>` mock must learn the new command or the a11y suite breaks silently.
- `pulse-app/ui/src/canvas/frame-metrics.ts` (or a sibling module) — the webview-side call.
- The three call sites: `pulse-app/ui/src/halo/HaloCanvas.tsx` (hue update in the rAF loop), the constellation canvas, and `pulse-app/ui/src/hooks/use-findings.ts` (counter refresh).
- `pulse-app/ui/src/bindings/bindings.test.ts` — pins the `telemetry.frontend` procedure list verbatim (`:35–38`), so it fails until updated. A **data pin, not a call site** — a caller query would not have found it.
- **NOT modified:** `pulse-app/capabilities/*.json` — see premise 2 below.

## Scope premise closure
1. **Measurement side — VERIFIED as webview-originated, decisively.** obs-plan §1/§4 makes it a domain *rule* ("any webview-originated measurement must reach the log **only** through a TauRPC `telemetry.frontend.*` command"), the infrastructure already exists end-to-end, and it already emits from the reduced-motion branch. A backend-only timestamp cannot measure these bounds at all: all three end instants are paints inside the webview, invisible to the Rust side. The fork is settled by the artifacts, not by preference.
2. **Binding cost — `[premise-corrected]` the route entry was too pessimistic.** There is **no per-procedure capability JSON in this project**: `pulse-app/capabilities/default.json` grants `core:default` + seven `core:window:*` + `updater:default` — TauRPC dispatches every router method through one IPC handler. `telemetry.frontend` is already in `EXPECTED_PROCEDURES` and `TelemetryApiImpl` is already merged in **both** the production router (`main.rs:990`) and the `emit_taurpc_bindings` test (`main.rs:206`). So a sibling method costs: trait+resolver, one `EXPECTED_PROCEDURES` line, a bindings regen, the a11y mock, and the `bindings.test.ts` pin — **not** "a full TauRPC quadruple binding".
3. **Three procedures vs one — unresolved by research; a real design choice.** `FrameDurationInput` is a working precedent for one procedure carrying bounded enums, and security is explicitly neutral between the shapes provided the discriminant is a closed enum. obs independently requires **three distinct `metric.{module}.{measure}` targets** either way. Goes to the operator at P4.
4. **Start instants — VERIFIED available for all three.** `ServiceListItem.last_seen_unix_nano` (bindings `:364`) is a backend timestamp already on the wire for P-027; `IncidentRecord.opened_at_unix_nano` / `updated_at_unix_nano` (`:132`) back the severity that drives P-025's hue; P-045 needs no backend reference — its bound is the webview-local poll→commit interval (`use-findings.ts:81`).

## Open questions
- Does the P-027 mark belong on the **widget** constellation (aggregate by design) or the **dashboard** hero (per-service)? Security bans `service.name` as a label (an OTLP resource attribute = user content) and obs bans per-service cardinality, so a per-service dot-arrival metric cannot be labelled by service — the measure must be aggregate (e.g. newly-discovered-count + latency) regardless of surface. → blocks: **implementation-scope**.
- Does `verify:capability-matrix` need the three caps' `notes` clauses edited once the observables land? All four anchor files currently exist and resolve, so nothing is dangling today. → blocks: **implementation-scope** (a wrap-time reconciliation, not a plan decision).
