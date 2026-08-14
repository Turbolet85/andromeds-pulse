## 1. Architecture Excerpt

### Stack (a11y reach)
- **Tauri 2.x** â€” native window + webview desktop shell, determines a11y testing reach (web surface via axe-core / Lighthouse / pa11y through webview).
- **`tonic` 0.14.x** â€” OTLP/gRPC receiver, provides backend service surface (non-UI, no a11y testing scope).
- **`axum` 0.8.x on `hyper` 1.x + `tower`** â€” OTLP/HTTP receiver, provides backend service surface (non-UI, no a11y testing scope).
- **`tokio` (current stable)** â€” async runtime, supports event-driven UI updates via channels (no direct a11y dimensions, enables IPC responsiveness).
- **WebGPU (`<canvas>` + `navigator.gpu`, WGSL)** â€” GPU-accelerated visualization surface in webview, defines chart rendering pattern and canvas accessibility reach (no built-in ARIA; depends on design layer for accessible labels and interactive controls).
- **TauRPC (`taurpc` crate)** â€” IPC bridge with auto-generated TypeScript bindings, determines webview-to-Rust command surface and error serialization shape (affects a11y of error messaging and focus management during async operations).
- **DuckDB 1.5.x via `duckdb` crate** â€” columnar storage, backend-only (no a11y testing scope).
- **`wasmtime` 25+ with WASM Component Model** â€” plugin runtime with capability-scoped sandboxing, determines third-party plugin reach (plugins can extend UI surfaces if granted WIT imports; a11y testing of plugin-provided UI is plugin-author responsibility).

### Surfaces
**Product type** â€” cross-platform desktop application (Windows / macOS / Linux) shipped as native bundles. Not a web app, not a CLI, not a microservice fleet.

### Project Intent Summary
- **Core functionality:** "every byte of telemetry stays on the developer's machine; no Docker, no collector cluster, no cloud backend means the install-to-first-trace loop is one binary launch."
- **Target users:** "developers staring at telemetry for hours; dark-mode is the default color scheme." Agent-driven development workflow signals that professional users on developer tools are the target; no explicit a11y-priority signals (elderly, low-vision, cognitive disability, or international language proficiency) named in Project Intent.
- **Critical paths hint:** No flows enumerated in arch â€” derive from input.md or tests' critical paths in Phase 1.

### CI/CD Platform
- **Platform:** GitHub Actions
- **Pipeline note:** `ci.yml` runs fmt + clippy + xtask test on every PR/push; `release.yml` builds `.msi` / `.dmg` / `.AppImage` / `.deb` bundles on tag push via `tauri-action`, signs Windows via Azure Key Vault, notarizes macOS via Apple Developer ID, publishes to GitHub Releases.

### A11y-Relevant Conventions
- **Module dependency direction:** dependencies flow toward the `pulse-app` binary; no circular dependencies; enforces boundary via Cargo workspace (determines architecture's isolation of concerns relevant to a11y scope segregation).
- **Cross-bridge data shape:** any type crossing the TauRPC IPC bridge must be `serde::Serialize`; enforced at compile time (affects a11y of error serialization and dynamic content in IPC responses).
- **Tray icon policy:** single tray icon offers minimal action menu (open/focus window, show ingest summary, toggle MCP, generate snapshot, quit); tray click focuses main window; closing main window minimizes to tray; menu accessibility and keyboard navigation are owned by a11y specialist; glyphs and locale strings owned by design specialist.
- **Webview IPC capability policy:** main webview runs under capability `pulse:default`, which permits only enumerated TauRPC procedures; adding new procedures requires Tauri capability JSON entry (affects a11y scope of webview JavaScript surface).
- **Module visibility discipline:** each crate exposes only public contract via `pub`; no cross-crate re-exports except in explicit contract module (isolates a11y testing scope per module boundary).
## 2. Security Plan Excerpt

### Security Tier

- **Tier:** Minimal
- **Justification:** "This is a local-first, zero-infrastructure single-user desktop app (Design Philosophy + Project Intent) with no user accounts, no persistent user data store (in-memory DuckDB ring buffer with 5â€“10 min retention), no internet-exposed network surface (OTLP receivers bound to `127.0.0.1` only per Occupied Resources), and no compliance-regulated data classifications."

### Anti-Patterns Rejected (a11y-relevant)

(No a11y-relevant anti-patterns in security plan â€” Phase 3 will apply default a11y discipline to all auth / verification UI.)

### A11y Compliance Triggers

(No a11y compliance triggers in security plan â€” Phase 1 will derive a11y tier from creator brief + project intent + surface count.)
## 3. Design System Excerpt

### Surfaces

- **desktop-webview** (React 19 + Tailwind CSS v4 + WebGPU canvas on Windows/macOS/Linux) â€” compact widget (quarter-screen, custom frameless titlebar) + full dashboard expansion; a11y testing via axe-core + Lighthouse + screen reader (NVDA / VoiceOver); keyboard navigation via Tab, focus-visible on all interactive elements
- **desktop-native** (Tauri 2 tray icon via NotifyIcon / NSStatusItem / AppIndicator) â€” system tray surface on Windows/macOS/Linux; no automated a11y tools; OS-native keyboard navigation via arrow keys / Return in menu; manual inspection required

### Loading / Error / Empty State Patterns

- **Skeleton state** â€” visibility: skeleton with animated opacity pulse (discrete on/off cadence per expression budget defined in design) â€” a11y impact: aria-busy=true during loading; skeleton regions must be perceivable but semantically marked as loading
- **Empty state** â€” visibility: centered text ("No traces yet", "Snapshot not generated") + optional telescope icon â€” a11y impact: clear text messaging; icon is decorative supplement (not color-alone indicator)
- **Error state** â€” visibility: error text color (via --color-accent token defined in design) + optional icon + message (one or two lines) â€” a11y impact: focus moves to error message or input field on form submit failure; error text must supplement color-only signaling; aria-invalid=true on form inputs with errors

### User-Facing Error Surfaces

- **In-page error message** (location: inline within form / input field) â€” appears for: validation failure, form submission rejection; a11y requirements: focus moves to error text or invalid input field; aria-invalid=true on input; aria-describedby links input to error message
- **Toast notification** (location: bottom-right or top-right corner per OS convention) â€” appears for: snapshot generation failure, MCP server toggle failure; a11y requirements: aria-live=polite for non-blocking status; keyboard reachable via Tab if action button provided; auto-dismiss cadence or manual close defined in design
- **Error color in semantic state** (location: border + text in inputs, cards, alerts) â€” appears for: form validation, alert states, error conditions; a11y requirements: color is not the only signifier; text label or icon supplement required; sufficient contrast for colorblind users (error color via --color-accent token defined in design)

### A11y-Relevant Design Tokens

#### Color tokens (foreground / background pairs)

- **--color-text-primary / --color-base** â€” context: body text / headlines on page background â€” meets SC 1.4.3 4.5:1 minimum (measured ratio defined in design)
- **--color-text-secondary / --color-base** â€” context: descriptions / supporting text on page background â€” meets SC 1.4.3 3:1 large text minimum (measured ratio defined in design)
- **--color-text-tertiary / --color-base** â€” context: metadata / timestamps / captions on page background â€” meets SC 1.4.3 3:1 large text minimum for large text only (measured ratio defined in design)
- **--color-primary / --color-base** â€” context: focus indicator / active state accent on page background
- **--color-feedback-success / --color-inset** â€” context: success state border / text on input background
- **--color-accent / --color-base** â€” context: error state text / alert border on page background

#### Focus ring tokens

- **--border-focus** â€” focus indicator color token (outline composition and offset specified in design)

#### Target size tokens

- **--target-button-min** â€” primary button tap target token (serves SC 2.5.5 44Ã—44 minimum)
- **--target-input-min** â€” input field tap target token (serves SC 2.5.5 44Ã—44 minimum and SC 2.5.8 24Ã—24 minimum; platform-native minimums defined in design)

#### Motion / transition tokens

- **--duration-fast** â€” micro-interaction duration token (hover feedback, focus ring appearance, button state transitions); reduced-motion override defined in design
- **--duration-standard** â€” panel transition duration token (panel open/close, page fade, state confirmation); reduced-motion override defined in design
- **--duration-investigation-collapse** â€” supporting moment duration token (Investigation Capture Collapse scale + opacity on snapshot generation); exempt from chrome expression budget; reduced-motion override defined in design
- **--easing-out** â€” primary easing curve token (standard for focus/hover feedback and transitions; ease-out semantic role: state-confirming feedback without bounce)
- **--easing-in-out** â€” panel transition easing curve token (panel emergence and state changes)

#### State color tokens (error / warning / success / info)

- **--color-accent** â€” state: error; not-color-alone supplement: text label (e.g., "Invalid field"), icon, or border underline
- **--color-feedback-success** â€” state: success; not-color-alone supplement: text label (e.g., "Validation successful"), icon, or checkmark
- **--color-primary** â€” state: info; not-color-alone supplement: text label or icon
- **--color-secondary** â€” state: warning; not-color-alone supplement: text label or border emphasis

#### Typography tokens (readability)

- **--font-display** â€” display heading font token (typeface, size, and weight defined in design; semantic role: page titles, hero text)
- **--font-body** â€” body prose font token (typeface, size, and weight defined in design; semantic role: paragraphs, descriptions, control labels, form labels)
- **--font-code** â€” monospace code font token (typeface, size, and weight defined in design; semantic role: code blocks, telemetry data, technical values, trace IDs)
- **Line-height for body text** â€” prose readability token (meets SC 1.4.8 line-height â‰¥ 1.5 minimum)
- **--spacing-md / --spacing-lg** â€” paragraph/section spacing tokens (responsive spacing adjustable without loss of content; serves SC 1.4.12 Text Spacing)

### ARIA-Relevant Component Patterns

- **Button (primary action)** â€” ARIA role: button; key states/props: aria-pressed (for toggle variants); keyboard contract: Enter or Space to activate
- **Input / Form field** â€” ARIA role: N/A (native `<input>` / `<textarea>`); key states/props: aria-label or associated `<label>`, aria-describedby (for error messages / hints), aria-invalid=true on validation failure; keyboard contract: Tab for focus, Enter/arrow keys context-dependent
- **Disclosure / Expandable section** â€” ARIA role: button (for toggle); key states/props: aria-expanded=true/false to indicate collapsed region state; keyboard contract: Enter / Space to toggle
- **Data table (telemetry)** â€” ARIA role: table (native `<table>`); key states/props: aria-label or `<caption>` for table purpose, header row marked with `<th>`, tabular numerals enabled for numeric alignment; keyboard contract: Tab to navigate cells, arrow keys for row/column navigation per custom implementation
## 4. Layout Templates Excerpt

### Layout Types per Surface

- **desktop-webview:** compact-widget, full-dashboard-traces, full-dashboard-metrics, full-dashboard-logs, full-dashboard-snapshots, settings-modal, investigation-modal
- **desktop-native:** tray-icon, tray-menu, notifications, file-picker

### Error Boundary Placement

(No explicit error boundary placement in layouts â€” Phase 3 will recommend defaults per layout category, typically section-level for dashboards / page-root for forms with focus-on-error pattern.)

### Focus Management Anchors

- **compact-widget:** Esc key minimizes to tray (focus returns to tray icon on restoration)
- **full-dashboard:** Tab navigates through tabs/sidebar items; Enter / Space activates tab; Esc closes modal (Settings or Investigation)
- **settings-modal:** Tab cycles through form controls (theme selector, widget snap position, retention input, MCP toggle); primary Save button and secondary Cancel button are keyboard-accessible; focused inputs display --border-focus token (focus ring opacity and composition defined in design)
- **investigation-modal:** close button (âœ• glyph) in top-right corner; Esc closes modal (explicit escape path for keyboard users)
- **tray-menu:** arrow keys navigate menu items, Return / Space selects, Escape closes menu

### Heading Hierarchy Anchors

(No explicit heading hierarchy in layouts â€” Phase 3 will recommend WCAG SC 1.3.1 + SC 2.4.6 defaults: single h1 per page, sequential h2 / h3 nesting, main + navigation + contentinfo landmarks.)
## 5. Test Plan Excerpt

### Tests Tier
- **Tier:** Standard
- **Justification:** Andromeda Pulse is a cross-platform desktop application (Windows/macOS/Linux via Tauri 2) with two primary surfaces and persistent in-memory data, spanning 19 entities with 7 critical user-facing flows.

### Test Harness Contract Summary

#### 5-command names:
- `boot` â€” Start the Tauri desktop application; initialize OTLP receivers on `:4317` (gRPC) and `:4318` (HTTP); spawn DuckDB in-memory buffer
- `run` â€” Invoke test suites via `cargo nextest run --workspace`
- `status` â€” Query product state via TauRPC `health` command
- `cleanup` â€” Terminate app process, close ports, verify graceful shutdown
- `logs` â€” Fetch product logs from `~/.andromeda-pulse/logs/` in JSON lines format

#### Status JSON shape:
```json
{
  "status": "ok" | "degraded" | "unhealthy",
  "subsystems": {
    "otlp_grpc_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "otlp_http_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "buffer": { "status": "ready" | "error", "rows_ingested": 1250, "retention_seconds": 600 },
    "ingest_channel": { "status": "ready" | "error", "broadcast_subscribers": 3 }
  },
  "uptime_ms": 5432,
  "pid": 12345
}
```

### Critical Paths (must-be-accessible)

- **P1: Receive OTLP telemetry (gRPC), visualize in WebGPU dashboard:** Send gRPC trace spans to `:4317`; query via TauRPC `traces.query`; assert response rows contain matching trace_ids.

- **P2: Generate token-efficient curated snapshot (not raw dump):** Populate buffer with 500 synthetic spans; invoke TauRPC `snapshot.generate` with 25000 token budget; assert response contains anomaly markers and dedup evidence.

- **P3: MCP server query (agent-accessible telemetry):** Spawn app with `ANDROMEDA_PULSE_MCP_ENABLED=true`; send JSON-RPC 2.0 `query_traces` call to stdin; assert `result.traces` array returned with correct schema.

- **P4: Plugin lifecycle (load, reload, invoke with capability scoping):** Stage fixture WASM module; invoke TauRPC `plugins.reload` and `plugins.invoke`; assert capability gating prevents disallowed operations.

- **P5: Widget compact mode â†” dashboard expansion â†” tray icon visibility toggle:** Verify IPC contract consistency via TauRPC `health` command; window/tray UI automation deferred to tauri-driver headful E2E.

- **P6: Real-time push of spans/metrics/logs via Tauri IPC Channel:** Subscribe to channel `pulse://stream/spans`; trigger gRPC ingest; assert binary Arrow IPC payload received with valid schema.

- **P7: Workspace detection (host project context correlation):** Invoke TauRPC `workspace.detect`; assert response contains detected project name and VCS info.

### Coverage Triggers Summary

- **OTLP loopback-only binding** (security-vector-coverage) â€” a11y implication: validate loopback-only binding enforcement via negative test (reject non-loopback source)

- **DuckDB SQL injection prevention** (security-vector-coverage) â€” a11y implication: validate parameter binding in prepared statements (no format-string SQL injection)

- **OTLP post-prost invariant checks** (security-vector-coverage) â€” a11y implication: validate span_id/trace_id length constraints (8/16 bytes); reject malformed payloads

- **Tauri IPC capability gating** (security-vector-coverage) â€” a11y implication: validate undeclared procedures are rejected; declared procedures succeed

- **Path canonicalization + confinement** (security-vector-coverage) â€” a11y implication: validate symlink chains and escape attempts are blocked

- **DefaultBodyLimit on OTLP HTTP** (security-vector-coverage) â€” a11y implication: validate oversized payloads rejected at HTTP 413 level

- **Cranelift-only WASM** (security-vector-coverage) â€” a11y implication: validate Cranelift backend enforced at build time

- **MCP feature + runtime gating** (security-vector-coverage) â€” a11y implication: validate both compile-time and runtime gates prevent unauthorized sidecar spawn

- **Minisign updater signature verification** (security-vector-coverage) â€” a11y implication: validate valid signatures accepted, invalid signatures rejected

- **No self-OTLP dialing** (security-vector-coverage) â€” a11y implication: validate receiver does not create infinite loop via self-dialing

- **OTLP protocol compliance (gRPC)** (compliance-test) â€” a11y implication: validate TraceService/MetricsService/LogsService RPCs conform to OpenTelemetry spec v1.x

- **OTLP protocol compliance (HTTP)** (compliance-test) â€” a11y implication: validate POST `/v1/traces`, `/v1/metrics`, `/v1/logs` with protobuf/JSON bodies conform to OpenTelemetry spec

- **WebGPU canvas throughput** (performance-budget) â€” a11y implication: validate 10k spans/sec injection sustained without buffer overflow or frame rate degradation

- **Snapshot token budget enforcement** (performance-budget) â€” a11y implication: validate snapshot generation respects token_budget <= 25000 limit

- **Buffer overflow / retention window enforcement** (chaos-test) â€” a11y implication: validate ring buffer eviction at 5â€“10 min window; bounded memory under sustained load

- **TauRPC â†” IPC Channels consistency** (cross-surface-coordination) â€” a11y implication: validate TauRPC query results include rows from Channel events (no desync)

- **Snapshot generation + Notification emit** (cross-surface-coordination) â€” a11y implication: validate notification is emitted upon snapshot completion with token count

- **Multi-platform compat (Windows/macOS/Linux WebView2/WKWebView/GTK)** (multi-platform-compat) â€” a11y implication: validate boot, IPC, WebGPU availability, tray icon, notifications consistent across platforms

- **OTLP receiver saturation (HTTP vs gRPC trade-off)** (load-test) â€” a11y implication: validate concurrent HTTP and gRPC paths maintain < 100ms p99 latency without cross-protocol interference

- **OpenTelemetry protobuf evolution** (contract-test-against-OTLP-sandbox) â€” a11y implication: validate forward/backward compat with multiple protobuf versions

- **No manual verification checkpoints** (agent-driven-discipline) â€” a11y implication: all test assertions must be deterministic (exit code, structured output); no visual inspection required

- **Self-bootstrapping test data (no pre-baked DB)** (agent-driven-discipline) â€” a11y implication: all fixture data generated at runtime via OTLP ingest or Rust builders; no pre-baked snapshots

### Quality Gates Summary

- **Zero-flakiness statement:** Flaky tests are NOT tolerated. If a test flakes once: quarantine immediately via `#[ignore]` or CI conditional; root-cause investigation required; fix or delete before unquarantining. Rationale: agent-driven dev cannot distinguish flake from real bug; retry policies mask actual failures.

- **Coverage thresholds:** 
  - Line coverage â‰¥ 75%
  - Branch coverage â‰¥ 70%
  - Function coverage â‰¥ 85%
  - Exclude generated code (prost protobuf stubs, taurpc IPC bindings), test fixtures, and mock implementations from coverage metrics
## 6. Obs Plan Excerpt

### Obs Tier

- **Tier:** Standard
- **Justification:** "Andromeda Pulse is a cross-platform desktop application (Windows/macOS/Linux via Tauri 2) with 8 instrumentable modules and 8 major surfaces with multi-surface coordination requiring cross-surface trace context propagation, creator brief explicitly asking for token-efficient snapshot guarantees, real-time push via binary Arrow IPC and frontend telemetry flow, and security plan specifying 6 logging-sensitive vectors driving structured instrumentation."

### Log Format JSON Schema (binding)

```json
{
  "timestamp": "2026-05-02T16:18:34.567Z",
  "level": "INFO",
  "target": "ingest::grpc",
  "message": "TraceService.Export received 10 spans",
  "fields": {
    "span_count": 10,
    "service": "my-app"
  }
}
```

**Extensions (optional fields per telemetry triggers):**
- `trace_id` / `span_id` â€” W3C traceparent string fields
- `duration_ms`, `span_count`, `service`
- Per-trigger fields: `plugin_path_basename`, `query_id`, `param_count`, `token_count_actual`, `token_budget_limit`, `body_size_bytes`, `webview_backend`, `tray_api`, `wgpu_backend`, `dedup_count`, `anomaly_markers`, `p50_ms`/`p95_ms`/`p99_ms`/`max_ms`, `error_rate_percent`, `value`
- `service.name` / `service.version` / `deployment.environment` â€” populated as default subscriber fields

### Service Identity

- **service.name:** `"com.andromeda.pulse"` (compile-time constant, Tauri bundle identifier; for mcp-server sidecar: `"andromeda-pulse-mcp"`)
- **service.version:** compile-time `env!("CARGO_PKG_VERSION")` (or runtime read from `tauri.conf.json` for the bundled desktop build)
- **deployment.environment:** hardcoded `"production"` (desktop app, no staging/dev distinction at runtime)

### Sentry User-Feedback Widget (if applicable)

(No error reporting platform in obs plan â€” a11y Phase 3 will derive default a11y discipline for user-feedback if Phase 1 introduces error reporting; otherwise N/A.)

### Focus-Relevant Span Coverage (filtered)

(No focus-relevant spans in obs plan Section 4 â€” a11y Phase 3 will recommend focus tracing spans that obs may add later: focus.shift / focus.trap.enter / focus.trap.exit / focus.restore.)
## 7. Creator Brief Excerpt

### Must-Work Scenarios

- "**Quarter-screen widget mode** â€” default surface; window snaps Ðº side of screen (left / right / corner); always-on-top toggle; remembers position per display." (compact glance-monitor; primary surface â€” must-be-accessible flow)
- "**Glance-readable Ð¾Ñ‚ 2 meters** â€” typography legible at distance; high-contrast palette; motion convey state (pulse / flow / steady) Ð±ÐµÐ· requiring focused attention." (distance-legibility flow â€” sets contrast / typography accessibility expectations)
- "**Click Ðº expand** â€” opens full dashboard window; widget remains mounted (returns Ðº compact mode on close)." (compact â†’ expanded transition â€” focus management between surfaces)
- "**Tray icon (secondary)** â€” traffic-light status; click cycles widget visibility (visible / minimized / hidden)." (tray flow â€” keyboard/menu accessibility)
- "'Investigate' button on widget + main window + context menu." + "Generates **token-efficient curated snapshot** (not raw OTLP dump)." (Investigate workflow CORE feature â€” primary user-initiated flow that must be keyboard-reachable)
- "**Notification** â€” `Snapshot ready ({N} tokens). Paste in {AI tool} to investigate.`" (OS notification flow on snapshot completion â€” content must be screen-reader accessible)
- "AI agents query telemetry via MCP `tools/call` requests" (MCP server flow â€” agent-driven, no a11y surface itself but must not affect UI focus/state)
- Settings flows: "Buffer size / retention window", "Snapshot preset template (Claude Code / Cursor / ChatGPT / Custom)", "Theme (auto / light / dark)", "Widget mode (compact / hidden / disabled)", "Widget snap position", "MCP server toggle", "Plugin manager" (settings modal â€” form controls must be label-associated and keyboard-navigable)

### Rigor Hints

- "**Color:** dark + light themes. Status colors not-color-alone (paired Ñ iconography per WCAG)." (explicit WCAG reference; signals SC 1.4.1 Use of Color discipline; WCAG mention without specifying conformance level â€” Standard tier WCAG 2.1 AA is a reasonable default mapping)
- "**Motion:** every state change is Ð° transition; respects `prefers-reduced-motion` Ð´Ð»Ñ accessibility (degrades Ðº instant); motion-as-data principle (motion reflects telemetry character, Ð½Ðµ decoration)." (explicit accessibility callout; SC 2.3.3 Animation from Interactions / SC 2.3.1 Three Flashes discipline implied)
- "**Glance-readable Ð¾Ñ‚ 2 meters** â€” typography legible at distance; high-contrast palette" (high-contrast palette commitment; aligns to SC 1.4.3 minimum, possibly SC 1.4.6 enhanced for the always-visible widget surface)
- "**Aesthetic stance:** quiet ambient telemetry presence. Not a noisy dashboard demanding attention; a glanceable surface that conveys state through motion and color quality." (signals motion-budget discipline; reduced-motion override is a first-class concern)
- "**Development Style:** agent-driven (built via Andromeda v2 pipeline â€” recursive dogfood validates our OTel mandate on its own creator tool)." (agent-driven verification mandate â€” every WCAG criterion must have machine-runnable verification; no manual-only tests)

### A11y Anti-Patterns (creator's explicit asks)

- "**NOT:** Datadog / New Relic / Grafana enterprise-dashboard density (information overload). Not Status Hero / Pingdom (vacant marketing-app sterility). Not Neon / Supabase (heavy gradient SaaS aesthetic)." (information-overload + decorative-gradient anti-patterns â€” implies cognitive accessibility discipline: scannable hierarchy, restraint over density)
- "motion-as-data principle (motion reflects telemetry character, Ð½Ðµ decoration)" (decorative-motion anti-pattern â€” motion must carry semantic meaning; aligns with reduced-motion respect)
- "Status colors not-color-alone (paired Ñ iconography per WCAG)" (color-alone signaling anti-pattern â€” supplemental icon/label required for every status color)