## 1. Data Classification

- **Type:** user-content (telemetry payloads — traces, metrics, logs)
  **Where:** OTLP receivers (`ingest` crate on `:4317`/`:4318`), DuckDB in-memory ring buffer (`buffer` crate, schema `pulse_buffer.main`, tables `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`), broadcast fan-out to viz/MCP, and curated snapshot markdown files written to `~/.andromeda-pulse/snapshots/`.
  **Volume:** transient (5–10 min in-memory ring buffer, configurable via `ANDROMEDA_PULSE_RETENTION_SECONDS`); aggregate within retention window (Established Decisions states "10k spans/sec" target). Snapshots persist to disk under the per-platform data dir.
  **Sensitivity note:** Telemetry can contain incidentally captured secrets, IDs, URLs, error messages, and SQL fragments from the host application being instrumented (Stack: OTLP receiver; Project Intent: "local-dev iteration loop"). Architecture does not classify these explicitly, but OTLP attributes are user-controlled content.

- **Type:** config (user settings)
  **Where:** `~/.andromeda-pulse/config.toml` (per-platform resolved root from Occupied Resources), env vars `ANDROMEDA_PULSE_*`. Surface is small per Established Decisions: enum settings, port ranges, file paths, retention seconds, log level, MCP enabled flag, plugin dir.
  **Volume:** per-user (single-user local app per Project Intent).

- **Type:** config (signing/release credentials — out of app runtime)
  **Where:** GitHub Actions encrypted secrets and Azure Key Vault (Cross-cutting Patterns config management; Established Decisions code signing). Apple Developer ID for notarization. Updater public key is baked into the Tauri config (Occupied Resources).
  **Volume:** N/A at runtime (build/release pipeline only); the binary ships only the public verification key.

- **Type:** user-content (third-party WASM plugin binaries)
  **Where:** `~/.andromeda-pulse/plugins/` loaded by `wasmtime` Component Model host (`plugins` crate). Capability-scoped per WIT imports (Design Philosophy + Established Decisions: Plugin Runtime).
  **Volume:** per-user; arbitrary number; signed-plugin verification deferred post-v1 (Established Decisions: Plugin Distribution).

- **Type:** none — credentials / PII / payment / health
  **Where:** N/A. No user accounts (Project Intent: "single-machine, single-user; no tenancy"). No payment SDK in Stack. No identity provider in Stack. No health-domain framing in Project Intent.

## 2. Attack Surface

- **Vector:** public API (network-bound on loopback only)
  **Entry point:** OTLP/gRPC on `:4317` and OTLP/HTTP on `:4318`, both bound `127.0.0.1` only per Occupied Resources. Routes `POST /v1/traces`, `POST /v1/metrics`, `POST /v1/logs`. Wire formats: protobuf via `prost`/`tonic` decode, plus HTTP/JSON.
  **Trust boundary:** `prost`/`tonic` protobuf decode validates wire shape (Stack: Validation row). Loopback-only binding means any local process (including unprivileged user processes, container sidecars on the same host, browser tabs via DNS rebinding considerations) can post telemetry. No auth declared at this surface.

- **Vector:** IPC (Tauri TauRPC bridge — webview ↔ Rust)
  **Entry point:** Procedures enumerated in Occupied Resources: top-level `app_info`, `health`, `ready`, `get_settings`, `update_settings`; routers `traces.*`, `metrics.*`, `logs.*`, `snapshot.{generate,list_recent,copy_to_clipboard}`, `plugins.{list,reload,invoke}`, `mcp.{status,start,stop}` (feature-gated), `workspace.{detect,list}`. Real-time push events `pulse://stream/*` via Tauri IPC `Channel` API with binary Arrow IPC payloads.
  **Trust boundary:** Tauri 2 capability gating — `pulse:default` permits exactly the enumerated procedures and nothing else (Cross-cutting Patterns: Webview IPC capability policy). Errors crossing the bridge are forced to be `Serialize`-able `AppError` enum variants. Validation surface relies on `serde` deserialization plus `TryFrom<u16>` for ports/enums.

- **Vector:** plugin host (WASM Component Model)
  **Entry point:** `~/.andromeda-pulse/plugins/` filesystem load by `wasmtime` 25+ via `plugins` crate; `plugins.invoke` IPC dispatches into a guest. Capability `pulse:plugin-fs` gates host's read of the plugin dir (Inherited Defaults).
  **Trust boundary:** Capability-scoped sandboxing — guests receive only host imports declared in their WIT (Design Philosophy: Capability-scoped extensibility; Established Decisions: Plugin Runtime). No syscalls, file, or socket access unless explicitly granted in WIT. Signed-plugin verification deferred post-v1 (Established Decisions: Plugin Distribution Channel).

- **Vector:** MCP stdio surface (feature-gated)
  **Entry point:** `andromeda-pulse-mcp` rmcp sidecar (Occupied Resources). JSON-RPC 2.0 over stdin/stdout: `initialize`, `tools/list`, `tools/call`, `notifications/*`. Tools: `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`. Only present when `--features mcp-server` AND `ANDROMEDA_PULSE_MCP_ENABLED=true`.
  **Trust boundary:** Stdio caller is whatever process the user launched the sidecar from (typically an LLM client). No network exposure. JSON-RPC framing per MCP spec; no project-specific envelope.

- **Vector:** filesystem reads (config + workspace detection)
  **Entry point:** `config.toml` under per-platform data dir; env-var-overridable paths (`ANDROMEDA_PULSE_CONFIG_PATH`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_PLUGIN_DIR`); workspace-detector crate inspects host project context (`workspace.detect`, `workspace.list`).
  **Trust boundary:** No declared validation in arch beyond `serde` + smart enum types + `TryFrom<u16>` (Stack: Validation; Established Decisions: Validation Library). Path validation discipline not declared.

- **Vector:** in-app updater
  **Entry point:** `tauri-plugin-updater` 2.x consuming `latest.json` from GitHub Releases (Stack: Updater; Cross-cutting Patterns: Webview IPC capability policy).
  **Trust boundary:** Updater public key baked into Tauri config (Occupied Resources); updater capability `pulse:updater` is bound to the plugin flow and "is not exposed to webview JavaScript" (Cross-cutting Patterns: Webview IPC capability policy).

- **Vector:** OS notification / tray surfaces
  **Entry point:** `tauri-plugin-notification` (capability `pulse:notification`); tray icon (capability `pulse:tray`). Outbound user-facing only.
  **Trust boundary:** Outbound surfaces; not an input vector. Capability-gated.

- **Vector:** CLI input (env vars + binary launch)
  **Entry point:** Reserved env vars enumerated in Occupied Resources (`ANDROMEDA_PULSE_*`, `RUST_LOG`).
  **Trust boundary:** `serde` + `TryFrom<u16>` for ports; per Occupied Resources note, an MCP feature-flag env-var mismatch logs a warning rather than failing — explicit graceful-degrade behavior. Path env vars have no declared canonicalization.

- **Vector:** webview content (host-side)
  **Entry point:** WebView2 (Windows) / WKWebView (macOS/Linux). WebGPU canvas rendered from Arrow data. Webview source under `pulse-app/ui/`.
  **Trust boundary:** Tauri capability model (`pulse:default`) gates which IPC procedures are callable; no `fs`, `shell`, `dialog`, or `http` core APIs granted (Cross-cutting Patterns). WGSL shaders are first-party.

- **Vector:** none — public internet API, OAuth flow, webhook, file upload from internet
  All N/A. Local-only desktop app; no inbound internet exposure (Design Philosophy: Local-first, zero-infrastructure; Project Intent: single-machine, single-user).

## 3. Auth Model

- **Approach:** none
- **Provider:** N/A
- **Scope:** N/A

**Reasoning:** Project Intent declares "single-machine, single-user; no tenancy, no orchestration, no clustering." Stack lists no identity SDK, no auth framework, no session store. Network surfaces (`:4317`, `:4318`) are bound to `127.0.0.1` only per Occupied Resources, treating loopback access as trusted. The Tauri IPC bridge uses capability-based authorization (`pulse:default`) rather than user authentication — the trust model is "the OS user owns the app." MCP stdio inherits trust from whichever process spawned the sidecar. WASM plugin guests are authorized via WIT capability declarations, not identity. No login UX, no token issuance, no per-user scope is required by the architecture.

Open consideration for downstream phases: loopback-only OTLP binding is the de facto authorization boundary — anyone with local user privilege (including other apps on the same machine, IDE extensions, and browser-driven local-DNS-rebinding scenarios against `localhost:4318`) can write telemetry and, via TauRPC indirectly, read it back through the snapshot/query surface. Phase 2/3 will need to research whether browser-origin `fetch('http://localhost:4318/v1/traces', ...)` from arbitrary websites is a real exposure for HTTP/JSON OTLP.

## 4. Infrastructure

- **Hosting:** local-only — public OSS desktop app distributed via GitHub Releases; runs as a single Tauri process on the user's Windows/macOS/Linux machine (Infrastructure Patterns: Deployment model).
- **Database:** embedded — DuckDB 1.5.x via `duckdb` crate, in-memory `:memory:` connection (`pulse_buffer`), no persistent disk database (Stack: Storage engine; Established Decisions: Telemetry Retention Surface).
- **Networking:** local only — OTLP receivers bound `127.0.0.1` only (Occupied Resources); MCP over stdio (no socket); updater is outbound HTTPS to GitHub Releases (Stack: Updater); no inbound internet exposure (Infrastructure Patterns: "no Docker, no Kubernetes, no serverless, no docker-compose").
- **CI/CD:** GitHub Actions — `ci.yml` (fmt + clippy + xtask test + workspace build, matrix Linux/macOS/Windows); `release.yml` (`tauri-action` builds `.msi`/`.dmg`/`.AppImage`/`.deb`, signs Windows via Azure Key Vault EV, notarizes macOS via Apple Developer ID, uploads bundles + `latest.json` to GitHub Releases); `update-channels.yml` (Homebrew tap + Scoop manifest). Code-signing secrets live in GitHub Actions encrypted secrets and Azure Key Vault (Infrastructure Patterns CI/CD; Cross-cutting Patterns config management).

## 5. Compliance Triggers

None — no compliance-regulated data detected.

- No payment SDK in Stack → no PCI DSS trigger.
- No user accounts, no email/PII collection by the app itself; telemetry payloads originate from the local user's own host applications and never leave the local machine within the v1 architecture (no outbound network exporter; OS notifications are local; updater is outbound to GitHub but ships no user data) → no GDPR-as-controller posture for the app vendor.
- No health-domain framing in Project Intent → no HIPAA trigger.
- No children's-product framing in Project Intent → no COPPA trigger.

Caveat for downstream phases: telemetry from the host application *being instrumented* may incidentally contain PII or secrets; this is a data-handling concern (snapshot generation, clipboard copy via `snapshot.copy_to_clipboard`, MCP responses to external LLM clients) but does not impose compliance obligations on this app since the data originates and stays on the user's own machine unless the user explicitly exports it.

## 6. Security Tier

**Minimal (0)**

**Justification:** This is a local-first, zero-infrastructure single-user desktop app (Design Philosophy + Project Intent) with no user accounts, no persistent user data store (in-memory DuckDB ring buffer with 5–10 min retention), no internet-exposed network surface (OTLP receivers bound to `127.0.0.1` only per Occupied Resources), and no compliance-regulated data classifications. The dominant risks derive from (a) loopback OTLP receivers that any local process can post to, (b) untrusted third-party WASM plugins with deferred signature verification, (c) trusted-publisher code-signing keys held in Azure Key Vault and GitHub Actions secrets, and (d) the in-app updater consuming `latest.json` from GitHub Releases. Standard tier is not warranted because there is no public API, no user authentication surface, and no multi-tenant data; Hardened tier is not warranted because no payment, health, or compliance-regulated data is present. Phase 2/3 work should focus on dependency audit (Cargo + GitHub Actions), local-loopback API authorization considerations (DNS rebinding, cross-origin `fetch` to `localhost:4318`), WASM capability sandbox verification, plugin-load path-traversal hardening, snapshot/clipboard secret-leakage hygiene, and code-signing key custody — not on OWASP Top 10 web defenses.
