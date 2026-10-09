# obs extract

## Relevance
**partial** — UI/window-management + `core:window` capability chunk; not a core telemetry path (no OTLP / DuckDB / snapshot / MCP / plugin work), but new log lines, any backend window TauRPC handlers, and incident-content-in-logs discipline pull in a modest obs footprint.

## Constraints
- Any new log line (window lifecycle, CARRY poll, IPC-reject) emits NDJSON with required fields `timestamp/level/target/message/fields` + default `service.{name,version,environment}` — per obs-plan.md §6 (Required fields), §3 (Service identity).
- If window create/position/show/hide are backend TauRPC handlers, wrap each in `#[tracing::instrument(skip_all, fields(traceparent = %tp))]`; children inherit via `tracing::Span::current()` — per §4 (IPC-internal row).
- Do NOT log incident/Findings row content — it is OTLP-derived (High classification); log metadata only (`unread_count`, `incident_count`), same posture as DuckDB-query / MCP-response bodies — per §8 (data classification), §6 (boundary = metadata), §11 (Logs).
- CARRY re-poll must NOT log at `info` per tick (~1s cadence = hot-path spam); gate poll-tick detail to `debug`/`trace` or omit — per §6 (log-level mapping), §11 (Logs: no info in hot path).
- Frontend telemetry, if any, routes through the TauRPC `telemetry.frontend.*` bridge to backend `tracing` — never a browser OTel SDK or direct webview file write — per §3 (Frontend bridge), §1 (desktop-webview surface).
- Service identity stays default-subscriber-field sourced; never hardcode `service.name` — per §3, §11 (Universal).

## Patterns to follow
- **P5 UI-lifecycle boundary events** (§4 P5): model window show/dismiss like `ui.layout.transition` + `tray.visibility.toggle` — a low-cardinality boundary `info` event with state fields (`shown`, `dismiss_reason` ∈ {blur, esc, row_select, mark_all_read}, plus `clamp_applied`/`flipped_above` for the off-screen clamp).
- **IPC-internal TauRPC instrumentation** (§4 IPC-internal): `#[tracing::instrument(skip_all, fields(traceparent))]` on any new window/geometry command handler; span naming `{module}.{operation}`, no per-incident-ID names.
- **Boundary log = metadata, not content** (§6 boundary-call wrappers, §5 cardinality discipline): counts/types only; enumerated label values.
- **Frontend-via-TauRPC bridge, single JSON file** (§3, §1): unified self-observation surface; no browser OTel SDK linked into the webview.

## Anti-patterns to avoid
- NEVER emit unstructured stderr text — JSON-per-line to the file sink only (§11 Logs).
- NEVER log incident/Findings bodies (OTLP-derived) or use high-cardinality labels such as per-incident-ID (§11 Logs / Metrics).
- NEVER add per-tick `info` logs to the CARRY re-poll loop, and NEVER link a browser OTel SDK into the webview (§11 Logs / Universal).

## Contract bindings
- **obs ↔ tests harness** (§3 log-format binding contract; §4 P5): window-lifecycle log lines must match the §6 NDJSON schema so the harness parses them; P5's agent-driven surrogate asserts IPC-contract consistency via the TauRPC `health` command — keep `health` response shape valid if window state is added to it.
- **obs ↔ a11y** (focus guide; §6 log format): the chunk #87 cross-window disclosure/focus contract is a11y-owned runtime behavior; binding is latent — only if a11y emits focus/disclosure violations do they ride obs's NDJSON format. Flagged; no new emission required from this chunk.
- **obs ↔ capabilities** (scope item 5; §6 error levels): a `core:window` IPC rejected for a missing grant should surface as a caught `warn`/`error` at the frontend invoke site, not silent — makes grant-misconfig (the "silently rejected IPC" failure mode) debuggable.

## Acceptance criteria contributions
- (obs) New window-lifecycle / CARRY-poll log lines are NDJSON with required fields + `service.{name,version,environment}` defaults (§6).
- (obs) After opening the window + mark-all-read, `agent-latest.jsonl` contains NO incident body text — only counts/metadata (`unread_count`, `incident_count`) (§8/§11).
- (obs) CARRY re-poll emits no per-tick `info` log (spam-free hot path); tick detail gated to `debug`/`trace` (§11 Logs).
- (obs) Any new window TauRPC handler is `#[tracing::instrument]`-wrapped with traceparent propagation; a rejected `core:window` IPC is logged, not silent (§4 / §6).

## Relevant amendment history
- **2026-05-04 — PII grep UI-vocabulary exemption (→ §8):** directly relevant — this chunk is UI-label-heavy ("unread incidents", "Findings" badge, SR text) with a disclosure a11y contract; the no-incident-content-in-logs check must distinguish real OTLP-derived secret *formats* from UI vocabulary, so a PII grep does not false-positive on the badge/label strings.
- **2026-05-08 — heartbeat vs health complementarity (→ §3):** mildly relevant — P5's window-consistency surrogate uses the sync TauRPC `health` command (active liveness), distinct from async heartbeat ticks; if this chunk's window state touches `health`, keep the two roles unconflated.
- (2026-06-10 load-profile/frame-budget and the 2026-05-02 tracing-only-pivot / initial-plan entries are out-of-area here: no DuckDB load, canvas-frame, or self-observation-runtime change in this chunk.)