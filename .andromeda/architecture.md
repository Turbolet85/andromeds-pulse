## Design Philosophy

- **Local-first, zero-infrastructure** — every byte of telemetry stays on the developer's machine; no Docker, no collector cluster, no cloud backend means the install-to-first-trace loop is one binary launch, not a stack.
- **Single-process modular monolith** — twelve library crates linked into the `pulse-app` Tauri binary (fourteen workspace members total: twelve library crates + the `pulse-app` binary + the `xtask` task-runner crate) share memory via tokio channels rather than network hops, so ingest→buffer→viz latency is measured in microseconds and the OS only sees one process. Occupied Resources is the canonical workspace-member list; any claim of a count word elsewhere refers back to it.
- **Standards-track at the edges, opinionated in the middle** — OTLP at `:4317`/`:4318`, MCP over stdio, and WASM Component Model plugins are all spec-conformant so external tooling Just Works; internal contracts (TauRPC bridge, Arrow zero-copy hand-off) are tightly opinionated to keep agent-driven development unambiguous.
- **Token-efficient curation as a first-class output** — the snapshot generator is treated as a peer of the visualization surface, not a side feature, because turning local telemetry into LLM context is the differentiator.
- **Capability-scoped extensibility** — WASM Component Model plugins receive only the host imports they declare in WIT, so third-party plugins cannot escalate beyond explicitly granted resources, and the security posture remains auditable.
- **Developer-tool surface** — the visualization shell is a dense, chart-first, low-chrome dashboard targeting developers staring at telemetry for hours; dark-mode is the default color scheme, and the WebGPU canvas + tray icon + OS-notification surfaces are the visual touchpoints arch commits to. Concrete tokens (color, motion, typography, density scale) are owned by the design specialist but must respect this density and color-scheme constraint.

## Stack and Technologies

| Layer | Technology | Role |
|---|---|---|
| Primary language / runtime | Rust 2024 edition (rustc 1.84+) | Single language across receivers, buffer, viz host, IPC, plugin host, MCP server |
| Desktop shell | Tauri 2.x | Native window + webview, OS bundlers, updater, tray, notifications |
| gRPC server | `tonic` 0.14.x | OTLP/gRPC receiver on `:4317` (with `prost` 0.14 codegen against `opentelemetry-proto`) |
| HTTP server | `axum` 0.8.x on `hyper` 1.x + `tower` | OTLP/HTTP receiver on `:4318` (protobuf and JSON) sharing the Tokio runtime with tonic |
| Async runtime | `tokio` (current stable) | Single multi-threaded runtime serving both OTLP ports and IPC |
| In-process channels | `tokio::sync::mpsc` + `tokio::sync::broadcast` | Ingest→appender hand-off (mpsc, backpressure) and buffer→subscribers fan-out (broadcast) |
| Storage engine | DuckDB 1.5.x via `duckdb` crate 1.10500.x | Embedded columnar OLAP, in-memory ring buffer (5–10 min, configurable) |
| Columnar interchange | Apache Arrow (via `Appender::append_record_batch()` / `stream_arrow()`) | Zero-copy hand-off between OTLP decode → DuckDB → viz/MCP |
| Plugin runtime | `wasmtime` 25+ with WASM Component Model + WIT | Capability-scoped third-party extensions loaded from `~/.andromeda-pulse/plugins/` |
| Visualization surface | Webview WebGPU (`<canvas>` + `navigator.gpu`, WGSL) | GPU-accelerated charts inside WebView2 (Windows) and WKWebView (macOS/Linux equivalents) |
| Tauri IPC bridge | TauRPC (`taurpc` crate) | Router-style Rust ↔ webview commands with auto-generated TypeScript bindings |
| MCP server | `rmcp` (official Rust SDK) over stdio | 8 `#[tool]` methods — `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` (telemetry) + `query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved` (corpus-backed incident/report, chunk #94) — gated by `--features mcp-server` |
| OS notifications | `tauri-plugin-notification` 2.x | Native "Snapshot ready" toasts (Notification Center / Action Center / freedesktop) |
| Updater | `tauri-plugin-updater` 2.x with `latest.json` | In-app updates from GitHub Releases |
| Mobile framework | N/A — desktop only (Windows / macOS / Linux) | — |
| Message broker | N/A — single-process desktop app | — |
| AI/ML serving | `llama.cpp` prebuilt binaries (b9305-pinned series) invoked via subprocess (D1 spawn-per-generation); v0.2.0 Epoch 9+; v0.1.0 had no in-process model | L4 interpretation layer — local on-device inference of curated digests via llama.cpp native JSON-schema-constrained GBNF sampling (`--json-schema-file`); CUDA + CPU prebuilt `llama-cli.exe` selected per hardware-profile tier (CUDA build + `-ngl 99` for gpu-primary/gpu-fallback, CPU build + `-ngl 0` for cpu-primary/cpu-fallback); bounded `-st` single-turn + explicit `-n {max_tokens}` + outer wall-clock timeout discipline. v0.1.0 paste-to-AI snapshot/markdown surface preserved alongside |
| Push notifications | N/A — OS-native local toasts only, no FCM/APNS/web push | — |
| Secret / key storage | `keyring` 3.x with the explicit platform feature set (`apple-native` / `windows-native` / `sync-secret-service` / `crypto-rust`) + `blake3` KDF for the fallback | OS credential store holding the corpus AES-256-GCM cell key (macOS Keychain / Linux Secret Service / Windows Credential Manager), with an opt-in `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` BLAKE3-derived fallback. **The feature set is load-bearing** — keyring 3.x declares no `default` feature, so a bare `keyring = "3"` links no backend and silently yields a per-process key |
| Error handling | `thiserror` 2.x (modules) + `anyhow` 1.x (boundaries) | Module-internal enums; boundary catch-all at Tauri commands and `main.rs` |
| Validation | `serde` + smart enum types + `TryFrom<u16>` | Validation surface is small (enum settings, port ranges, file paths); OTLP validated by `prost`/`tonic` decode |
| Module boundary enforcement | Cargo workspace, one crate per module | Compile-time `[dependencies]` graph + `pub`/`pub(crate)` visibility |
| Build / package manager | Cargo (workspace) | One workspace, multiple library crates, one binary crate |
| CI task runner | `cargo-xtask` | Release / sign / notarize as Rust binaries inside the same workspace |
| Release pipeline | `tauri-action` GitHub Action + Tauri 2 native bundlers | Builds `.msi` / `.dmg` / `.AppImage` / `.deb` distribution artifacts (macOS `.app` bundle is wrapped into the `.dmg` rather than published standalone) and uploads `latest.json` per release |
| Code signing | Azure Key Vault (Windows EV) + Apple Developer ID (macOS notarization) | Trusted-publisher signing for all distributed bundles |
| Distribution channels | GitHub Releases (primary) + Homebrew tap + Scoop manifest | Three channels driven from the same workflow |
| Code quality (lint/typecheck) | `cargo fmt` + `cargo clippy` (Rust); TauRPC-generated `.d.ts` + `tsc --noEmit` (webview) | Lint and typecheck on every CI run; clippy lints fail the build |

## Established Decisions

- **[Platform] Tauri 2 desktop shell**: chosen over Electron because bundle size is 5–10× smaller (~10–20 MB vs 100 MB+), the security model is capability-scoped, and a Rust backend means the receiver, buffer, and UI bridge live in one process with no Node bridge tax.
- **[Primary Language] Rust 2024 edition (rustc 1.84+)**: matches the high-performance OTel-receiver trend (Rotel benchmarks: 75% less memory, 50% less CPU than the Go Collector) and means one language end-to-end through ingest, buffer, viz host, plugin host, and MCP server.
- **[Backend Framework — OTLP receiver] `tonic` 0.14.x + `axum` 0.8.x on shared `hyper` 1.x + `tower`**: a single Tokio runtime serves both `:4317` gRPC and `:4318` HTTP without two unrelated substrates; axum's middleware/extractor ergonomics outweigh the ~30 transitive deps for a 12-module monolith. Alternatives (raw hyper, poem+tonic, actix+tonic) either lost on ergonomics or fought Tauri's `tokio::main` integration. **Caveat to reconcile before locking versions**: `tonic` 0.14 vs `opentelemetry-otlp` 0.31 (which still pins `tonic` 0.13 in some feature combinations) — read both `Cargo.toml`s before tagging.
- **[Database] DuckDB embedded via `duckdb` crate 1.10500.x (DuckDB 1.5.x bundled), in-memory ring buffer**: columnar+SQL+Arrow trifecta is exactly what aggregating spans/metrics by service/time-bucket needs at 10k spans/sec. SQLite was rejected because aggregating 10k spans/sec into per-service p99 series in row-store SQL turns the query into the bottleneck. DataFusion remains a documented swap-out only if DuckDB's C++ FFI complicates Tauri cross-compile.
- **[ORM / Migrations] None — direct SQL via `duckdb` crate's `Connection` and `Appender`**: ring-buffer schema is small, lifecycle is "create on startup" + periodic `DELETE WHERE ts < cutoff`, and Arrow zero-copy ingest sidesteps any ORM marshalling tax.
- **[In-Process Channel Architecture] `tokio::sync::mpsc` + `tokio::sync::broadcast`**: mpsc for the OTLP-ingest → DuckDB-appender hand-off (built-in backpressure absorbs receiver bursts); broadcast for fan-out from buffer to live UI subscribers (compact widget, full dashboard, tray icon, MCP server can all subscribe to the same span stream). `crossbeam-channel` was rejected because mixing tokio mpsc with crossbeam adds bridging complexity for a pure-tokio stack.
- **[Plugin Runtime] WASM Component Model via `wasmtime` 25+ with WIT interfaces**: capability-based sandboxing (no syscalls, file, or socket access unless explicitly granted) and type-safe host imports match the standards path. Extism was considered for easier multi-language guest support, but Component Model is the standards-track choice the input.md explicitly calls out and aligns with Bytecode Alliance LTS.
- **[WebGPU Visualization Surface] Webview WebGPU (`<canvas>` + `navigator.gpu`)**: WGSL shaders run unchanged across WebView2 (Windows) and WKWebView (macOS); IPC pushes Arrow data into the canvas. Native `wgpu` 25+ surface was rejected for v1 because Tauri 2 native-overlay flickering is documented (issue #9220), and "ship on three OSes with one codebase" beats "avoid theoretical browser GC pauses." **Documented upgrade path**: native `wgpu` 25+ surface for the compact widget if browser GC jank becomes a measured problem (hybrid Option C).
- **[Tauri IPC Bridge — Surface 2] TauRPC (`taurpc` crate)**: derive macro auto-generates fully-typed TypeScript bindings from the Rust command surface, eliminating the drift bugs that a 12-module monolith with 30+ commands would otherwise produce. Stock `#[tauri::command]` was rejected because manual TS re-declaration is error-prone at this module count; tauri-specta is composable but more ceremonial per command. **Keystone**: this choice forces all error types crossing the bridge to be `Serialize`-able.
- **[MCP Server Surface] `rmcp` (official Rust SDK) over stdio with `#[tool]`-annotated methods**: `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` map to tool methods; chunk #94 added 4 corpus-backed incident/report tools — `query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved` — bringing the surface to 8 `#[tool]` methods. Gated by `--features mcp-server` (default off) so the rmcp dependency only enters the build when explicitly enabled. **Caveat to verify before locking**: the input-cited "rmcp 1.5.0" must be reconciled against the currently published 0.3.x line — verify whether input is forward-looking, refers to an internal spec, or refers to the unrelated `4t145/rmcp` fork.
- **[OTLP Wire Format Coverage] Classic OTLP for v1 (protobuf over HTTP `:4318` + gRPC `:4317`)**: every SDK in the wild emits classic OTLP today; OTAP (Arrow-encoded) is the 2025–2026 performance frontier but server-side and emerging. OTAP ingest is implemented behind a feature flag, activated when SDK clients catch up.
- **[API Style — OTLP Receiver Surface 1] OpenTelemetry spec, no fork**: `:4317` = gRPC over `opentelemetry-proto`; `:4318` = HTTP/protobuf or HTTP/JSON. Wire format is fixed by the spec.
- **[API Style — MCP Server Surface 3] JSON-RPC 2.0 over stdio per MCP spec**: rmcp's macros are the canonical surface; do not invent a custom wire format.
- **[Error Handling Pattern] `thiserror` 2.x for module-internal error enums + `anyhow` 1.x at the Tauri command boundary and `main.rs` + a `serde`-friendly `AppError` enum wrapper for IPC**: matches Rust 2026 community consensus; AppError is lossy-but-explicit because TauRPC requires `Serialize`-able errors. **Documented upgrade path**: add `miette` 7.x for plugin-loading diagnostics if WIT resolution UX surfaces source-span-worthy failures.
- **[Validation Library] None — `serde` + smart enum types + `TryFrom<u16>` impls**: validation surface is tiny (enum settings, port ranges, file paths) and OTLP protobuf payloads are validated by `prost`/`tonic` decode itself. **Documented upgrade path**: reach for `garde` 0.20+ when the plugin manifest gains cross-field validation (any rule that must inspect more than one field together — e.g. "version range required if capability list non-empty"), or when the manifest exceeds ~10 declared fields, whichever comes first; until then `serde` + `TryFrom` covers each field independently.
- **[Module Boundaries] Cargo workspace, one library crate per module + `pulse-app/` binary**: a 12-module project with "modular monolith" stated intent must enforce boundaries via the dependency graph (not review discipline); single-crate `mod` directories let any internal `pub(crate)` leak across modules. **Deferred**: `cargo-deny` 0.16+ `bans` rules are added when a boundary is actually breached, not from day 1.
- **[Snapshot Curation Default] Balanced — 25k tokens, full dedupe + critical-path + p50/p95/p99 + anomaly highlighting**: matches the "AI-debug snapshot" differentiator and is small enough to fit in any frontier-model context budget while large enough to preserve a meaningful distributed trace.
- **[Plugin Distribution Channel — v1] Built-in templates + filesystem loading from `~/.andromeda-pulse/plugins/`**: signed-plugin verification deferred post-v1; no marketplace UI in v1. Lowest-friction path to "first plugin in 30 minutes."
- **[Telemetry Retention Surface] In-memory DuckDB ring buffer only (5–10 min, configurable)**: matches the "local-dev iteration loop" use case; persistent disk storage is reserved for the heavier backends and is out of scope.
- **[Mobile / Message Broker / Push Notifications] N/A**: single-process desktop app on Windows/macOS/Linux only; OS-native local notifications only. (v0.1.0 also had AI/ML serving = N/A — v0.2.0 introduces local LLM inference per [LLM Inference Runtime — L4 interpretation layer] entry below.)
- **[Self-Observation] Use `opentelemetry-stdout` (or file exporter) for the product's own telemetry; never dial the product's own OTLP ports**: instrumenting an OTLP receiver with an OTLP network exporter pointed back at itself creates an infinite loop. Any future `ANDROMEDA_OBSERVER_URL`-shaped variable must explicitly distinguish "outbound observer" (we *are* the observer — do not dial) from "inbound receivers" (the OTLP ports we listen on).
- **[Deployment / Release Pipeline] `tauri-action` GitHub Action + Tauri 2 native bundlers**: one workflow file produces `.msi` / `.dmg` / `.AppImage` / `.deb` (the macOS `.app` bundle is wrapped into the `.dmg` rather than uploaded standalone) and auto-uploads `latest.json` for the Tauri updater plugin. Hand-rolled matrix and cross-rs-with-custom-packagers were rejected because they lose Tauri-updater integration.
- **[Code Signing] Azure Key Vault (Windows EV) + Apple Developer ID (macOS notarization)**: the standard trusted-publisher path for OSS desktop apps in 2026.
- **[CI Task Runner] `cargo-xtask`**: lightest fit for a Rust-only repo; release/sign/notarize tasks are Rust binaries in the same workspace, matching agent-driven harness ergonomics. `just` and `cargo-make` add DSL surface area without buying anything for one app with no non-Rust contributors.
- **[Distribution Channels] GitHub Releases (primary) + Homebrew tap + Scoop manifest**: three channels driven from the same workflow via `taiki-e/upload-rust-binary-action`-style patterns. Rationale: GitHub Releases is the minimum viable distribution; Homebrew + Scoop match developer-tool install norms (`brew install andromeda-pulse` / `scoop install andromeda-pulse`) for the agent-driven-developer target user. **Deferral path**: if the Homebrew tap or Scoop bucket setup blocks the v0.1.0 ship, drop them to v0.2.0 and ship GitHub Releases alone.
- **[LLM Inference Runtime — L4 interpretation layer] `llama.cpp` prebuilt binaries (b9305-pinned series) invoked via subprocess (D1 — spawn-per-generation)**: local on-device L4 inference of curated digests via llama.cpp native JSON-schema-constrained GBNF sampling (`--json-schema-file`). CUDA + CPU prebuilt `llama-cli.exe` selected per hardware-profile tier (chunk #80 `HardwareProfileSource`): GPU-primary/fallback → CUDA + `-ngl 99`; CPU-primary/fallback → CPU + `-ngl 0`. Bounded-invocation discipline (defense-in-depth against unbounded generation): explicit `-n {max_tokens}` cap + `-st` single-turn + outer `tokio::time::timeout` wall-clock guard, spawned per-L4-digest via `tokio::process::Command` with `kill_on_drop(true)`. D1 (spawn-per-generation) over D2 (long-lived `llama-server`) because Pulse's real L4 rate is 1–4 invocations/min (cadence-coordinator background subscriber on `pulse://stream/digests`, chunk #81; no user-action trigger), so the ~5s cold-start tax (~1–2s warm) hides behind no spinner and D2's permanent-lifecycle complexity buys throughput Pulse won't exercise. **Pin discipline**: lock to a specific build tag (`b9305` exact at v0.2.0 swap-in); bumps are deliberate chunk-scoped events, never "latest tag" drift. **Bus-factor / swap paths**: the `pub trait LlmInferenceRunner: Send + Sync` (`crates/interpretation/src/contract.rs`) is the swap boundary — only the concrete impl changes. Three documented sibling impls behind the same trait: `llama-server` HTTP (D2) if invocation rate/UX warrants amortizing load tax; in-process `llama-cpp-2` bindings once the Windows libclang+cmake+MSVC toolchain cost is justified; `candle` (+ `outlines-rs`/`llguidance` for constrained sampling) if llama.cpp's maintenance posture changes. **Open caveat (decide during any runtime chunk)**: prebuilt-binary distribution is unsettled — whether the CUDA+CPU `llama-cli.exe` (+ `cudart64_13.dll`/`cublas64_13.dll`) ship in the Tauri bundle, are downloaded by an `xtask` boot step, or rely on the `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`/`_CPU_BIN_PATH` env pair (current dev pattern); co-design with the `ANDROMEDA_PULSE_MODEL_PATH` model-file distribution decision. *(History — the original `mistralrs = "=0.8.0"` choice and its empirical invalidation: see `architecture-amendments.md` § Decision-history 2026-05-25.)*
- **[Fault Identity — what makes two faults ONE fault] Two layers, decided independently: L1 fingerprint normalization is TOKEN-LEADING; L2 incident identity is COALESCE-PER-CUE-IDENTITY**. **L1** — stacktrace normalization strips a path token only when the absolute marker STARTS a token (position 0, or preceded by a non-path byte); both the Unix `/` arm and the Windows drive-letter arm carry that precondition (`crates/buffer/src/fingerprint.rs::is_token_boundary`). Absolute paths are stripped for host-stability, while **relative path structure is identity-significant** — `src/a.rs` and `src/b/c.rs` are DISTINCT fingerprints. Rationale: host-stability is what absolute-path stripping buys; stripping relative structure serves nothing and destroys identity resolution, and for a diagnostic tool over-coalescing hides distinct faults. Blast radius: the fingerprint value reaches the appender, the consumer, the storm-observer adapter and boot wiring, and — since `2026-08-17-incident-fingerprint-producer-repaired` — onward through the attention cue, the digest cue ref and the incident producer into `Incident.fingerprint` (persisted to the corpus, read by the `fingerprint_match` retrieval arm and the digest assembler). No viz / MCP / UI consumer exists (the field reaches no rendered surface — the webview `IncidentRecord` view drops it and `PreviouslySeenMatch` omits it), and `span_events.fingerprint` is still never SELECTed. **L2** — incidents coalesce on the cue tuple `(kind, scope, scope_id)`, which for storms is effectively per-service; a storm carrying a different fingerprint on a service that already has an open incident is absorbed BY DESIGN. Fingerprint is absent from that key on the ground that survives: every downstream surface is already N-safe (the per-service constellation join reduces by max tier over all matching active incidents; the digest assembler reads `list_active` only as a boolean), so a second concurrent incident would change no rendering and no L4 behaviour — it would only add rows. Per-fingerprint dedupe is therefore **possible-but-declined** rather than unavailable: the cue now carries a real fingerprint, so this decision is REVISITABLE by a future entry, and until one takes it the predicate stays as written. **REJECTED**: keying on `L4Output.fingerprint` — model-authored, and a constant under the deterministic runner, so per-fingerprint dedupe would be a no-op in exactly the mode used for reproducible e2e verification. **SHIPPED 2026-08-17** (was DEFERRED — the documented path if incident-per-identity is ever wanted): threading a real `ExceptionFingerprint` through `AttentionCue` → `DigestCueRef` → the producer. `AttentionCue.fingerprint` and `DigestCueRef.fingerprint` (`Option<String>`, `#[serde(default)]`, full 32-char lowercase hex; `None` for baseline-derived families, which detect a statistical condition rather than a specific fault) now carry the L1 value to the incident site. **Producer repaired**: `Incident.fingerprint` is contracted (`crates/triage/src/contract.rs`) and consumed (digest retrieval / assembler) as a lowercase-hex grouping hash, and is now PRODUCED from the threaded L1 fingerprint via the crate-internal `triage::contract::hex_lower` — the SAME encoder the assembler applies to Q3 rows, deliberately not the 4-byte `pattern::storm::fingerprint_to_hex_prefix` (8 chars, bounded-cardinality tracing only; a prefix here could never match). The corpus-retrieval `fingerprint_match` arm is therefore FED, not starved — its do-not-simplify-it-away guard now rests on the arm being live rather than on a contracted-but-starved mismatch. Incidents with no triggering cue (reflection cadence, baseline families) carry an empty fingerprint and match on scope only. *(Decision rationale + the external-harness switching cost: see `architecture-amendments.md` § Decision-history 2026-08-16.)*

## Conventions

- **Workspace API style — internal Rust↔webview**: TauRPC routers, one router per module crate (`ingest`, `buffer`, `viz`, `ui-bridge`, `snapshot`, `curation`, `triage`, `workspace-detector`, `plugins`, `mcp-server`, `corpus`, `security`); no manual TypeScript re-declaration of command signatures. Frontend consumes only TauRPC-generated `.ts` bindings.
- **External wire — OTLP receiver**: OpenTelemetry spec exactly. `:4317` accepts gRPC over `opentelemetry-proto`; `:4318` accepts HTTP/protobuf at `/v1/traces`, `/v1/metrics`, `/v1/logs` (HTTP/JSON encoding optional). No URL versioning prefix beyond what the OTLP spec defines.
- **External wire — MCP server**: JSON-RPC 2.0 over stdio per MCP spec; tool methods are `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`, `query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved` (last 4 added chunk #94).
- **Error response schema (Tauri IPC)**: every `#[taurpc::procedure]` returns `Result<T, AppError>` where `AppError` is a `serde`-friendly enum with stable variants (`Validation { field, reason }`, `NotFound { resource }`, `Internal { message }`, `Plugin { plugin_id, message }`, `Storage { message }`, `Ingest { message }`). Module-internal code uses `thiserror`-derived enums and converts at the bridge via `From` impls. `main.rs` and any non-IPC top-level uses `anyhow::Result`.
- **Error response schema (OTLP receiver)**: per OpenTelemetry HTTP spec — non-2xx responses carry `Status` proto in body for HTTP/protobuf, or HTTP/JSON for HTTP/JSON; gRPC errors use standard `tonic::Status` codes.
- **Error response schema (MCP)**: standard JSON-RPC 2.0 error object with stable `code` (numeric), `message` (string), and optional `data`; MCP-spec error codes are honored.
- **File naming**: Rust files are `snake_case.rs`; one library crate per module under `crates/<module-name>/`; binary crate is `pulse-app/`. WIT interface files are `kebab-case.wit`. Webview source files follow the design specialist's convention (out of scope here).
- **Variable / function naming**: Rust `snake_case` for functions/variables/modules, `UpperCamelCase` for types/traits/enums, `SCREAMING_SNAKE_CASE` for constants — standard `cargo clippy` `style` group enforced in CI.
- **Endpoint naming**: OTLP endpoints are spec-fixed (`/v1/traces`, `/v1/metrics`, `/v1/logs`). Tauri IPC procedures use one of two authorized shapes: (a) top-level bare `snake_case` verbs for the cross-cutting envelope (`app_info`, `health`, `ready`, `get_settings`, `update_settings`); (b) `<router>.<verb>` dotted namespaces for per-crate routers (e.g., `traces.query`, `snapshot.generate`, `plugins.list`, `mcp.start`, `workspace.detect`). Both segments are `snake_case`. Occupied Resources is the canonical procedure list; the Convention defines the shape, not the enumeration.
- **Database entity naming**: DuckDB tables use plural `snake_case` matching the OTLP entity they hold (reserved set in Occupied Resources: `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`); columns are `snake_case`; ring-buffer cutoff is enforced via a periodic `DELETE FROM <table> WHERE ts < now() - INTERVAL '<retention> minutes'` task. Occupied Resources is the canonical table-name list; the Convention defines the shape, not the enumeration.
- **Primary key convention**: spans use the OTLP-native 16-byte `trace_id` + 8-byte `span_id` as composite identity (no surrogate); metric points and log records use OTLP-native identity (timestamp + resource hash + name); no UUID surrogate keys are introduced because OTLP entities already have spec-defined IDs.
- **Timestamp handling**: all DuckDB timestamp columns are `TIMESTAMPTZ` (DuckDB native, microsecond precision, UTC-stored); incoming OTLP nanosecond timestamps are stored as `TIMESTAMPTZ` (microsecond truncation accepted) plus a sibling `BIGINT` column `ts_unix_nano` when nanosecond precision is required for spec round-tripping.
- **Nullable patterns**: nullable columns are reserved for OTLP-spec-optional fields (e.g., `parent_span_id`, optional resource attributes); required spec fields are `NOT NULL`.
- **Module visibility discipline**: each crate exposes only its public contract via `pub`; cross-crate utilities use `pub(crate)`; no `pub use` re-exports across crate boundaries except in the explicit contract module of each crate.
- **Feature flags**: Cargo features are `kebab-case` (`mcp-server`, `otap-ingest`); default features are minimal (no `mcp-server`, no `otap-ingest`).
- **Configuration units**: durations in user-facing config are seconds with explicit units in field names (`retention_seconds`); ports are `u16` with `TryFrom<u16>` validating non-privileged-or-explicitly-allowed ranges.

## Standard Contracts

- **Tauri IPC introspection — `app_info` command**: every TauRPC router exposes a top-level `app_info` returning the application identity envelope.

  ```json
  {
    "name": "andromeda-pulse",
    "version": "0.1.0",
    "rust_version": "1.84.0",
    "tauri_version": "2.x",
    "features": ["mcp-server"],
    "build_profile": "release"
  }
  ```

- **Tauri IPC liveness — `health` command**: returns liveness state of the in-process subsystems.

  ```json
  {
    "status": "ok",
    "checked_at": "2026-05-02T12:34:56Z",
    "subsystems": {
      "otlp_grpc_receiver": "ok",
      "otlp_http_receiver": "ok",
      "buffer": "ok",
      "ingest_channel": "ok"
    }
  }
  ```

  When degraded, `status` becomes `"degraded"` and any failing subsystem string becomes the failure reason (e.g., `"otlp_grpc_receiver": "bind_failed: address in use"`); HTTP-style status is implicit via the IPC `Result` (a hard failure returns `Err(AppError::Internal { ... })`).

- **Tauri IPC readiness — `ready` command**: returns whether the app is ready to ingest and serve queries.

  ```json
  {
    "ready": true,
    "checked_at": "2026-05-02T12:34:56Z",
    "checks": {
      "duckdb_connection": "ok",
      "ingest_mpsc_capacity_pct": 0,
      "broadcast_subscribers": 2,
      "plugins_loaded": 0,
      "mcp_server_enabled": false,
      "rows_ingested": 12000,
      "buffer_used_seconds": 120,
      "retention_seconds": 600
    }
  }
  ```

  When not ready, `ready` is `false` and any failing check is the offending reason string (e.g., `"duckdb_connection": "init_in_progress"`).

- **OTLP receiver — spec-conformant**: `:4317` gRPC implements the `TraceService`, `MetricsService`, `LogsService` from `opentelemetry-proto`; `:4318` HTTP exposes `POST /v1/traces`, `POST /v1/metrics`, `POST /v1/logs` accepting `Content-Type: application/x-protobuf` (and `application/json` per spec). No project-specific envelope.

- **MCP server — spec-conformant**: standard MCP `tools/list` returns `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`, `query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved` (last 4 added chunk #94); standard `tools/call` invokes them with JSON-RPC 2.0 framing over stdio. Capabilities, prompts, resources, sampling sections follow the MCP spec defaults.

- **Common response envelope (Tauri IPC paginated lists)**: list-returning IPC commands wrap results in:

  ```json
  {
    "items": [],
    "total": 0,
    "next_cursor": null
  }
  ```

  Cursor is an opaque string; absent (`null`) means "no more results."

- **Real-time push contract**: live span/metric streams to the webview use Tauri 2 IPC `Channel` API with binary Arrow IPC payloads (no JSON-stringify tax); event names are `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`, `pulse://stream/snapshot-progress`, `pulse://stream/plugin-events`. SSE/WebSocket are not used because the consumer is in-process.

## Occupied Resources

- **Network ports**:
  - `:4317` — OTLP/gRPC receiver (spec-fixed; bound on `127.0.0.1` only)
  - `:4318` — OTLP/HTTP receiver (spec-fixed; bound on `127.0.0.1` only)
  - No frontend dev server port reserved at arch level (managed by webview, not exposed)
- **Tauri IPC routes (TauRPC procedures)**:
  - `app_info`, `health`, `ready`, `get_settings`, `update_settings` — top-level (ui-bridge crate)
  - `traces.*`, `metrics.*`, `logs.*` — query routers (viz crate)
  - `streams.subscribe_spans`, `streams.subscribe_metrics`, `streams.subscribe_logs` — pulse-app crate (Tauri Channel<Vec<u8>> binding to `buffer::BroadcastSenders` for binary Arrow IPC; chunk #23)
  - `telemetry.frontend.record_frame_ms` — ui-bridge crate (`FrameDurationInput` → `metric.webgpu.frame_duration_ms` tracing event per obs-plan §3 Logging stack > Frontend bridge; chunk #29)
  - `snapshot.generate` — snapshot crate (`snapshot.list_recent` and `snapshot.copy_to_clipboard` deferred — no runtime emitter as of chunk #74; only `snapshot.generate` implemented at `pulse-app/src/snapshot_runtime.rs`)
  - `plugins.list`, `plugins.reload`, `plugins.invoke` — plugins crate
  - `mcp.status`, `mcp.start`, `mcp.stop` — mcp-server crate (only when `--features mcp-server`)
  - `workspace.detect` — workspace-detector crate (`workspace.list` deferred — no runtime emitter as of chunk #74; only `workspace.detect` implemented at `crates/ui-bridge/src/workspace_ipc.rs:47`)
  - `connection.current_state` — pulse-app crate (`ConnectionApiImpl` returning `ConnectionStatePayload` from `crates/ingest::connection::compute_state()`; chunk #59)
  - `services.list_with_states` — pulse-app crate (`ServicesApiImpl` returning `ServiceListPayload` from `crates/triage::lifecycle::InMemoryServiceRegistry::list`; chunk #67)
  - `storage.inspect`, `storage.path` — pulse-app crate (`StorageApiImpl` returning corpus inspection metadata + data-dir absolute path from `crates/corpus::CorpusReader`; chunk #68)
  - `storage.export_for_training` — pulse-app crate (`StorageApiImpl::export_for_training` resolver producing an anonymized JSONL incident corpus dump from `crates/corpus::CorpusWriter::load_all_incidents` via `pulse-app/src/training_export.rs`; `confirm=false` previews without writing, `confirm=true` writes to `target_path` or the default `~/Downloads` egress sink; chunk #95)
  - `diagnostics.template_distribution` — pulse-app crate (`DiagnosticsApiImpl` returning `TemplateDistributionPayload` from `crates/buffer::DrainMiner::template_distribution()`; chunk #69)
  - `diagnostics.retry_interpretation` — pulse-app crate (`DiagnosticsApiImpl` returning `RetryInterpretationPayload` via the L4 degraded-mode FSM manual-override path; chunk #86)
  - `diagnostics.reevaluate_recent_window` — pulse-app crate (`DiagnosticsApiImpl` opt-in FULL retrospective re-classification of the recent window via `RecentWindowReevaluator`/`LiveReevaluator` in `pulse-app/src/reevaluation.rs`; chunk #96)
  - `diagnostics.snapshot` — pulse-app crate (`DiagnosticsApiImpl` returning `DiagnosticsSnapshotPayload` — HYBRID-RENDER L6 self-observability point-in-time aggregate of Model/Hardware/Pipeline live state from `interpretation::LlmInferenceRunner` + `triage::HardwareProfileSource` + `buffer::DrainMiner`; sub-fields with no production producer render explicit "not yet recorded" never fabricated; chunk #97)
  - `diagnostics.history` — pulse-app crate (`DiagnosticsApiImpl` returning `DiagnosticsHistoryPayload` — validated stub: bounded-allowlist `metric_name` + clamped `window_seconds` → empty series + "not yet recorded" notice, NO corpus query; chunk #97)
  - `config.reload`, `config.status` — pulse-app crate (`ConfigApiImpl` backed by the `crates/config-watcher` notify-watcher + `tokio::sync::watch` fan-out; `config.reload` forces an immediate re-read + hot-apply of `config.toml`, `config.status` returns the last-reload timestamp + last error category; chunk #96)
  - `incidents.list_active`, `incidents.acknowledge`, `incidents.mark_resolved`, `incidents.mark_all_read`, `incidents.get_report` — pulse-app crate (`IncidentsApiImpl` resolver backed by `crates/triage::contract::IncidentRegistry` + `IncidentPersistence` + `IncidentLifecycleBroadcast`; chunk #78, `mark_all_read` chunk #87, `get_report` chunk #88)
  - `model.current_profile` — pulse-app crate (`ModelApiImpl` returning `ModelProfilePayload` { profile_label, tier_label, load_status, model_identity_name } from `interpretation::contract::LlmInferenceRunner` + `triage::contract::HardwareProfileSource`; chunk #82)
  - `investigate.run_action` — pulse-app crate (`InvestigateApiImpl` returning `InvestigateResultDto` { action_id, title, symptom, timeline, hypotheses[], investigation_steps[] } — runs a real `interpretation::contract::LlmInferenceRunner::generate_constrained` analysis of the curated telemetry context for one of the 4 bounded Investigate actions (diagnose-latency-outlier / find-error-correlation / trace-failed-request / summarize-service-health), reusing the incident `L4Output` schema + `parse_bounded`, and returns a TRANSIENT scrubbed result — NO incident created/persisted, NO broadcast; honours the deterministic env-gated L4 mode `ANDROMEDA_PULSE_L4_DETERMINISTIC` (P-073) for reproducibility; chunk 2026-06-28-investigate-actions-functional, P-072)
- **External HTTP routes (OTLP HTTP)**: `POST /v1/traces`, `POST /v1/metrics`, `POST /v1/logs` on `:4318`.
- **MCP stdio surface**: standard MCP `initialize`, `tools/list`, `tools/call`, `notifications/*` over stdin/stdout when the rmcp sidecar is started.
- **Tauri IPC events (broadcast channels)**: `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`, `pulse://stream/snapshot-progress`, `pulse://stream/plugin-events` (deferred — no runtime emitter as of chunk #74; pending plugin invocation telemetry chunk), `pulse://stream/connection-state` (chunk #59), `pulse://stream/attention-cues` (chunk #62), `pulse://stream/restart-events` (chunk #63), `pulse://stream/service-lifecycle` (chunk #67), `pulse://stream/incidents` (chunk #78), `pulse://stream/cadence-events` (chunk #80 — cadence coordinator L6-visibility topic, `crates/triage/src/cadence/broadcast.rs:9`), `pulse://stream/digests` (chunk #81), `pulse://stream/model-status` (chunk #82), `pulse://stream/config-events` (chunk #96). **Cross-window UI-coordination events** (a distinct class from the `pulse://stream/*` Rust→webview broadcast channels above — first-party webview↔webview frontend events via `@tauri-apps/api/event` emit/listen, gated by `core:event` in `pulse:default`, carrying no telemetry): `findings:dismissed` / `report:open` (`{incidentId}`) / `report:closed` (chunk 2026-07-10-incidents-floating-window-disclosure — the `findings` + `report` floating windows coordinate dismiss / open-report / restore-focus across windows).
- **Process / service identity**:
  - Tauri app bundle identifier: `com.andromeda.pulse`
  - Binary name: `andromeda-pulse` (Linux/macOS), `andromeda-pulse.exe` (Windows)
  - Binary crate name: `pulse-app`
  - rmcp sidecar binary (when feature enabled): `andromeda-pulse-mcp`
- **Cargo workspace crate names**: `ingest`, `buffer`, `viz`, `ui-bridge`, `snapshot`, `curation`, `triage`, `workspace-detector`, `plugins`, `mcp-server`, `corpus`, `security`, `interpretation`, `config-watcher`, `pulse-app`, `xtask` — these names are reserved at the workspace level and cannot be reused by scopes.
- **DuckDB database / schema names**:
  - In-memory database identity: `pulse_buffer` (single in-memory `:memory:` DuckDB connection, schema `main`)
  - Reserved tables: `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`, `log_templates` (8th table added per chunk #69 Phase B; `crates/buffer/src/schema.rs:18`)
- **Corpus SQLite database / schema names**:
  - On-disk database identity: `corpus/corpus.db` under data dir root (single SQLite connection; chunk #68 origin)
  - Reserved tables: `baseline_state`, `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive` (`crates/corpus/src/schema.rs:26-33`)
  - At-rest posture: cell-level AES-256-GCM encryption is in force; table + column names plaintext, BLOB cell payloads opaque without key (per security-plan.md §Data Protection §At rest; chunk #68 substrate). **Key sourcing is the OS credential store, primary** (macOS Keychain / Linux Secret Service / Windows Credential Manager via the `keyring` crate) — `keyring` 3 is declared with the explicit platform feature set (`apple-native` / `windows-native` / `sync-secret-service` / `crypto-rust`), because keyring 3.x declares NO `default` feature and a bare `keyring = "3"` links no backend at all. The key therefore **persists across processes**: measured at chunk `2026-08-15-corpus-key-persistence` — a second boot decrypted the rows its predecessor wrote with 0 `decryption_failed` (against the 13 that defined the defect), and the Windows entry `corpus-key.com.andromeda.pulse` is present. **Fallback:** an OPT-IN passphrase supplied via `ANDROMEDA_PULSE_CORPUS_PASSPHRASE`, BLAKE3-derived, nothing written to disk, emitting a once-per-boot WARN announcing reduced key-custody guarantees; it engages only when configured, so a transient store failure can never silently switch keys. On a host with neither a credential store nor a configured passphrase the corpus degrades to absent rather than inventing a key. **Content orphaned by the pre-fix ephemeral-key window is retired at boot** by the inventory-then-purge disposition (`corpus::disposition`) — recovery is cryptographically closed, so the keyless inventory is written before deletion; no DDL and no `SCHEMA_VERSION` bump were needed.
- **Filesystem locations** (the `~/.andromeda-pulse/` notation below is the Linux canonical form; per-platform resolution is fixed and applies to every path under this root):
  - Linux: `~/.andromeda-pulse/` (i.e. `$XDG_CONFIG_HOME/andromeda-pulse/` if set, else `$HOME/.andromeda-pulse/`)
  - macOS: `~/Library/Application Support/com.andromeda.pulse/`
  - Windows: `%APPDATA%\andromeda-pulse\` (i.e. `%APPDATA%\andromeda-pulse\config.toml`, `...\plugins\`, `...\snapshots\`, `...\logs\`)
  - Subpaths under the resolved root: `config.toml` (user settings), `plugins/` (WASM Component Model plugin loading directory), `snapshots/` (generated snapshot markdown files), `logs/` (stdout-exporter destination for self-telemetry), `corpus/corpus.db` (persistent incident corpus SQLite; cell-level AES-256-GCM encrypted with a key sourced from the OS credential store, with an opt-in `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` fallback — see §Corpus SQLite → At-rest posture; chunk #68, key persistence landed 2026-08-15-corpus-key-persistence), `corpus/orphaned-inventory-{unix_nano}.md` (the keyless audit artifact the orphan disposition writes — atomic `.tmp`+rename — BEFORE deleting content orphaned by a lost key; carries only columns already plaintext in the corpus, so it exposes nothing the database did not; chunk 2026-08-15-corpus-key-persistence), `run/andromeda-pulse.pid` (PID file written by production binary at boot; consumed by `scripts/agent-run.{sh,ps1}` harness for status/cleanup commands — `pulse-app/src/main.rs:196`), `run/workspace-key` (the resolved incident workspace key, published by the app at boot via atomic `.tmp`+rename with a canonicalize-and-confine guard and read cross-process by the `andromeda-pulse-mcp` sidecar so both filter incidents by the identity the app stamps; single line, bounded ≤ `MAX_WORKSPACE_KEY_BYTES` (4096) on read, consumed as an opaque filter string and never joined as a path; `workspace_detector::contract::{workspace_key, publish_workspace_key, read_published_workspace_key}` — chunk 2026-08-14-workspace-key-alignment), `window-geometry.json` (remembered per-window positions — Rust-owned JSON map of window label → integer x/y; atomic `.tmp`+rename, written by `pulse-app/src/window_geometry.rs` on window-move + restored at boot; chunk 2026-06-29-window-geometry-movable-shell, P-061).
  - `ANDROMEDA_PULSE_DATA_DIR` overrides the resolved root on every platform; subpath layout under the override is identical to the per-platform default.
  - **Out-of-data-dir egress sink (training export):** `storage.export_for_training` (chunk #95) writes an anonymized JSONL incident corpus dump to `<home>/Downloads/pulse-corpus-export-{ts_unix_nano}.jsonl` by default (or a user-supplied `target_path`). This is the ONE deliberate exception to the under-data-dir path-confinement rule (CLAUDE.md §Critical Warnings — path env vars MUST canonicalize + resolve under the data dir): the export is a user-initiated egress sink, NOT a data-dir-managed runtime file. Pre-write validation rejects `..` parent-directory traversal + requires the parent directory to exist (`pulse-app/src/training_export.rs`); PII is scrubbed at the egress boundary via `security::scrubber::scrub_attribute` (defense-in-depth on top of the chunk #72 producer scrub). No native file picker (no `tauri-plugin-dialog`) — preserves the `pulse:default` capability surface. No auto-submission — the user manually shares the file.
- **Environment variables (reserved at arch level)**:
  - `ANDROMEDA_PULSE_CONFIG_PATH` — override path to `config.toml` (deferred — no runtime consumer as of chunk #74; Settings uses fixed `<data_dir>/config.toml` per `crates/ui-bridge/src/contract.rs`)
  - `ANDROMEDA_PULSE_DATA_DIR` — override `~/.andromeda-pulse/` root
  - `ANDROMEDA_PULSE_OTLP_GRPC_PORT` — override `:4317`
  - `ANDROMEDA_PULSE_OTLP_HTTP_PORT` — override `:4318`
  - `ANDROMEDA_PULSE_RETENTION_SECONDS` — override ring-buffer retention (300–600 default range)
  - `ANDROMEDA_PULSE_LOG_LEVEL` — `trace|debug|info|warn|error`
  - `ANDROMEDA_PULSE_MCP_ENABLED` — `true|false` (only honored when binary built with `--features mcp-server`). When set to `true` against a binary built without the feature, startup logs a warning at `warn` level naming the missing feature flag and proceeds with MCP disabled (rather than failing to start), so a misconfigured environment variable does not block the rest of the app.
  - `ANDROMEDA_PULSE_PLUGIN_DIR` — override `~/.andromeda-pulse/plugins/`
  - `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` — path to prebuilt `llama-cli.exe` CUDA build (b9305-pinned series); consumed by `pulse-app/src/llamacli_inference.rs` for GPU-primary / GPU-fallback tier inference per chunk #80 `HardwareProfileSource` routing (chunk #84)
  - `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` — path to prebuilt `llama-cli.exe` CPU build (b9305-pinned series); consumed by `pulse-app/src/llamacli_inference.rs` for CPU-primary / CPU-fallback / Unknown tier inference (chunk #84)
  - `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` — opt-in fallback passphrase for the corpus AES-256-GCM cell key, used only when the OS credential store is unreachable. Bounded parse (non-empty, ≤ 1024 bytes; unset / empty / over-long ⇒ not-configured, so the fallback stays opt-in and a transient store failure cannot silently switch keys). BLAKE3-derived with the service id folded in; nothing is written to disk. Secret-class: never logged, never in `Debug` (`crates/corpus/src/keychain.rs::CORPUS_PASSPHRASE_ENV`; chunk 2026-08-15-corpus-key-persistence)
  - `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` — overrides the per-service cold-start (bootstrap) window that gates the silence family, so a fresh service becomes reachable inside a bounded warm-up instead of an hour of wall-clock. **Production-consumed** (it changes the running app's gate — NOT harness-only). Bounded parse: a non-zero integer strictly below `WINDOW_DURATION_SECONDS` (86400), whitespace-trimmed; unset / empty / unparseable / zero / out-of-range ⇒ the 3600s default, never a panic. Resolved ONCE at boot by `resolve_bootstrap_window_seconds()` / `Thresholds::from_env()` (`crates/triage/src/cue/thresholds.rs`) into `Thresholds::bootstrap_window_seconds`, which `pulse-app/src/main.rs` hands to `BaselineState::set_bootstrap_window_seconds` — so the silence evaluator, the emitter's bootstrap-state counters and the lifecycle registry all gate on ONE bound. Env layer only: not surfaced through `Settings` / `config.toml`, so no TauRPC contract and no hot-reload delta. A non-default resolved bound emits one WARN per boot on `triage.baseline.bootstrap_window.override` (chunk 2026-08-16-baseline-family-reachability)
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC` — `true|false` truthy gate (`1|true|yes`, default false) selecting the deterministic L4 runner (canned `L4Output`, no GPU/model) at `pulse-app` boot for reproducible demos / tests / external (Conductor) verification; bounded truthy-parse per the `crates/mcp-server/src/feature_gate.rs` pattern; consumed by `pulse-app/src/deterministic_inference.rs` (chunk 2026-06-28-deterministic-env-gated-l4-mode, P-073)
  - `RUST_LOG` — honored as fallback for log level filter
  - `ANDROMEDA_PULSE_PIDFILE` — harness-only override of PID file location for `scripts/agent-run.{sh,ps1}` test harness; NOT consumed by production binary (`scripts/agent-run.sh:22` + `scripts/agent-run.ps1:15`)
  - `ANDROMEDA_PULSE_LOGFILE` — harness-only override of log file location for `scripts/agent-run.{sh,ps1}` test harness; NOT consumed by production binary (`scripts/agent-run.sh:23` + `scripts/agent-run.ps1:16`)
  - `ANDROMEDA_PULSE_DATA_DIR_KEEP` — harness-only flag for `scripts/agent-run.{sh,ps1}` test harness to preserve TempDir on cleanup; NOT consumed by production binary (`scripts/agent-run.sh:99` + `scripts/agent-run.ps1:71`)
  - `PYTHONUTF8` (`=1`) + `PYTHONIOENCODING` (`=utf-8`) — harness-only UTF-8 encoding relay SET (not read) by `scripts/agent-run.sh` + `scripts/agent-run.ps1`, and mirrored in the `.claude/settings.json` `env` block so agent-invoked Python helpers decode their own output as UTF-8 rather than the host's legacy codepage; the PowerShell half additionally sets `[Console]::OutputEncoding`/`InputEncoding`. NOT consumed by the production binary
- **Tauri capability identifiers (reserved at arch level)**: `pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs`, `pulse:clipboard` — concrete capability JSON files live in `pulse-app/capabilities/`.
- **Updater channel**: `latest.json` published to GitHub Releases under the canonical repository; updater public key is shipped baked into the Tauri config.
- **Bundle artifact names** (per release): `andromeda-pulse_<version>_x64-setup.msi`, `andromeda-pulse_<version>_x64.dmg`, `andromeda-pulse_<version>_aarch64.dmg`, `andromeda-pulse_<version>_amd64.AppImage`, `andromeda-pulse_<version>_amd64.deb`.
- **Docker volumes**: N/A — no Docker.

## Infrastructure Patterns

**Build system.**
- Cargo workspace; package manager is `cargo`.
- Lint: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Typecheck: implicit in `cargo check --workspace --all-targets`; webview TypeScript bindings emitted by TauRPC are typechecked with `tsc --noEmit`.
- Build: `cargo tauri build` (invoked by `tauri-action`); release profile is workspace default with `lto = "thin"`, `codegen-units = 1`, `strip = true` for distribution bundles.

**Deployment model.**
- Public OSS desktop app distributed through GitHub Releases; "deployment" is the release pipeline, not server hosting.
- No Docker, no Kubernetes, no serverless, no docker-compose.
- Runtime topology on the user's machine: one Tauri process hosting all twelve library crates and the embedded webview; one optional rmcp sidecar process (only when `--features mcp-server` is enabled at build time and `ANDROMEDA_PULSE_MCP_ENABLED=true` at runtime).

**Project directory structure.**

```
andromeda-pulse/
├── Cargo.toml                      # workspace manifest
├── Cargo.lock
├── rust-toolchain.toml             # pin rustc 1.84+
├── .github/
│   └── workflows/
│       ├── release.yml             # tauri-action: build + sign + notarize + publish
│       ├── ci.yml                  # fmt + clippy + cargo-xtask test
│       └── update-channels.yml     # Homebrew tap + Scoop manifest jobs
├── crates/
│   ├── ingest/                     # OTLP receivers (tonic + axum)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── buffer/                     # DuckDB ring buffer + Arrow appender
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── viz/                        # query layer feeding webview WebGPU charts
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── ui-bridge/                  # TauRPC routers
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── snapshot/                   # curated markdown generator
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── workspace-detector/         # detect host project context
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── plugins/                    # wasmtime Component Model host
│   │   ├── Cargo.toml
│   │   ├── wit/                    # WIT interface definitions
│   │   └── src/
│   └── mcp-server/                 # rmcp stdio sidecar (feature-gated)
│       ├── Cargo.toml
│       └── src/
├── pulse-app/                      # Tauri binary crate that wires the workspace
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/               # Tauri 2 capability JSON files
│   ├── icons/
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   └── ui/                         # webview source root (frontend tooling owned by design specialist)
│       └── src/
├── xtask/                          # cargo-xtask: release/sign/notarize/changelog tasks
│   ├── Cargo.toml
│   └── src/
├── plugins-examples/               # built-in plugin templates shipped with v1
│   └── README.md
├── docs/
└── README.md
```

**CI/CD approach.**
- Platform: GitHub Actions.
- `ci.yml` runs on every PR and push to main: matrix over Linux/macOS/Windows; steps `cargo fmt --check` → `cargo clippy ... -D warnings` → `cargo xtask test` → `cargo build --workspace` (release profile smoke).
- `release.yml` runs on tag push (`v*`): `tauri-action` builds `.msi` / `.dmg` / `.AppImage` / `.deb` per OS in matrix (macOS `.app` bundle is built then wrapped into `.dmg`, not published standalone); signs Windows artifacts via Azure Key Vault EV cert; notarizes macOS artifacts via Apple Developer ID; uploads bundles + `latest.json` to GitHub Releases.
- `update-channels.yml` runs on completion of `release.yml`: updates Homebrew tap and Scoop manifest with new version + sha256.
- All shared CI logic that needs Rust lives in the `xtask` crate so contributors can run identical commands locally with `cargo xtask <task>`.

## Cross-cutting Patterns

- **Config management**: layered with explicit precedence (highest wins) — process env vars (`ANDROMEDA_PULSE_*`) > user `~/.andromeda-pulse/config.toml` > built-in defaults. No external secrets manager (this is a local OSS desktop app); code-signing secrets live only in GitHub Actions encrypted secrets and Azure Key Vault.
- **Self-observation discipline**: `opentelemetry-stdout` (or file exporter targeting `~/.andromeda-pulse/logs/`) is the only exporter the product itself uses for its own telemetry; the OTLP network exporter is never pointed at the product's own `:4317`/`:4318` ports. Any future "outbound observer" variable must be explicitly distinct from the inbound receiver port variables.
- **Feature-gate hygiene**: `--features mcp-server` and `--features otap-ingest` (and any future flags) are off by default; flag-specific dependencies (`rmcp`, OTAP-specific crates) only enter the build graph when the feature is enabled, preserving the small-binary baseline.
- **Cross-bridge data shape**: any type that crosses the TauRPC bridge or appears in an MCP tool response must be `serde::Serialize`; this is enforced at compile time by the bridge derive macros. Internal-only types are not constrained.
- **OS notification policy**: native OS notifications (via `tauri-plugin-notification`, gated by capability `pulse:notification`) fire on completion of `pulse://stream/snapshot-progress` and on updater state transitions; no other subsystem emits notifications without an explicit decision. User opt-out is a single boolean `notifications_enabled` in `~/.andromeda-pulse/config.toml` (default `true`); when disabled, the snapshot generator and updater still complete their work but suppress the toast. Notification text shape and per-locale strings are owned by the design specialist.
- **Tray icon policy**: a single tray icon (gated by capability `pulse:tray`) is the always-on surface that signals app-running state and offers a minimal action menu — at minimum: open/focus the main window, show ingest summary (current spans/sec, retention used), toggle MCP server (only when `--features mcp-server` is built and the `mcp-server` crate is loaded), generate snapshot (invokes `snapshot.generate`), quit. Tray click on the icon focuses or restores the main window; closing the main window minimizes to tray rather than terminating the process (process termination requires the explicit Quit menu item or OS-level kill). Tray icon glyphs and per-locale menu labels are owned by the design specialist; menu accessibility (keyboard navigation, screen-reader labels) is owned by the a11y specialist.
- **Webview IPC capability policy**: the main webview window runs under capability `pulse:default`, which permits exactly the TauRPC procedures enumerated in Occupied Resources (the top-level envelope plus every `<router>.<verb>` in the reserved list) and nothing else; adding a new TauRPC procedure requires both a router registration in the owning crate and an entry in `pulse-app/capabilities/` so the bridge call is not silently rejected at runtime. The updater capability `pulse:updater` is bound to the `tauri-plugin-updater` flow that consumes `latest.json` from GitHub Releases and is not exposed to webview JavaScript; Tauri core APIs beyond the enumerated TauRPC surface (filesystem, shell, dialog, http) are not granted to `pulse:default` and require an explicit per-feature capability addition with a stated rationale.
- **Module dependency direction**: dependencies in `Cargo.toml` flow toward the `pulse-app` binary; no library crate depends on the binary crate; no library crate depends on a sibling unless its declared contract requires it. The dependency graph forms a DAG with `pulse-app` as the only root.
- **Test-time telemetry injection**: end-to-end tests inject synthetic spans/metrics/logs by speaking OTLP to the running receiver on `:4317` (gRPC) or `:4318` (HTTP) — the same surface external SDKs use. No in-process test-mode bypass, no separate "test ingest" feature flag, no mock-channel back-door. Tests that need a non-default port resolve it via `ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `ANDROMEDA_PULSE_OTLP_HTTP_PORT`. Tests that need to inspect buffer state read it back through the TauRPC `traces.*` / `metrics.*` / `logs.*` query routers.
- **Development Style**: agent-driven. Downstream specialist plans (tests, obs, setup-project) should branch their research toward agent-driven development workflows (deterministic harness invocations, machine-parseable outputs, schema-stable contracts, `cargo xtask` task surfaces).

## Project Intent

- **Product type**: cross-platform desktop application (Windows / macOS / Linux) shipped as native bundles. Not a web app, not a CLI, not a microservice fleet.
- **Scale intent**: startup. Single-machine, single-user; no tenancy, no orchestration, no clustering.
- **Growth model**: modular monolith (twelve Rust crates wired into one Tauri binary) with a WASM Component Model plugin extension layer for third-party additions. Internal modules are the unit of growth for first-party functionality; WASM components are the unit of growth for community/third-party functionality.
- **How new functionality is added**:
  - First-party features: new scopes via `/andromeda-scope-arch`, which generally adds either (a) a new Cargo crate to the workspace following the established crate-per-module pattern, or (b) new TauRPC routers/queries/snapshot strategies inside an existing crate. The twelve reserved crate names cannot be re-purposed.
  - Third-party features: WASM Component Model plugins dropped into `~/.andromeda-pulse/plugins/`, exposing functionality through host-imported WIT interfaces with capability-scoped access.
- **Template patterns**: each library crate follows the same shape — a public contract module (the only `pub` surface), `pub(crate)` internals, `thiserror`-derived error enum, and (where applicable) a TauRPC-router module mounted by `pulse-app`. Plugin authors follow the WIT-interface template under `crates/plugins/wit/` and the example plugins under `plugins-examples/`.

## Inherited Defaults

- Language / runtime: Rust 2024 edition (rustc 1.84+), single Tokio multi-threaded runtime.
- Desktop shell: Tauri 2.x with TauRPC IPC bridge.
- Backend framework (in-process receivers): `tonic` 0.14.x (gRPC, `:4317`) + `axum` 0.8.x on `hyper` 1.x + `tower` (HTTP, `:4318`). Open question carried from Established Decisions: if `opentelemetry-otlp` 0.31's transitive pin on `tonic` 0.13 cannot be resolved at lock time, the fallback is to downgrade Stack to `tonic` 0.13.x (matching `opentelemetry-otlp`) rather than fork or wait for upstream; the Stack/Decisions/Defaults rows are then re-pinned in the same iteration.
- Database: DuckDB 1.5.x via `duckdb` crate 1.10500.x, in-memory ring buffer (5–10 min retention, configurable).
- ORM / migrations: none — direct SQL via `duckdb` crate `Connection` + `Appender`; schema created on startup.
- Columnar interchange: Apache Arrow zero-copy via `Appender::append_record_batch()` / `stream_arrow()`.
- Channels: `tokio::sync::mpsc` for ingest→appender, `tokio::sync::broadcast` for buffer→subscribers.
- API style — internal IPC: TauRPC routers, one per crate, with two authorized procedure shapes — top-level bare `snake_case` verbs for the cross-cutting envelope (`app_info`, `health`, `ready`, `get_settings`, `update_settings`) and `<router>.<verb>` dotted namespaces for per-crate routers (`traces.query`, `snapshot.generate`, `plugins.list`, etc.). See Conventions for the shape rule and Occupied Resources for the canonical procedure list.
- API style — external OTLP: spec-fixed (`/v1/traces`, `/v1/metrics`, `/v1/logs` on `:4318`; gRPC services on `:4317`).
- API style — external MCP: JSON-RPC 2.0 over stdio with rmcp `#[tool]` methods (feature-gated). Open question carried from Established Decisions: the input-cited "rmcp 1.5.0" must be reconciled against the published `0.3.x` line before locking — verify whether the reference is forward-looking, an internal spec name, or the unrelated `4t145/rmcp` fork; if 1.5.0 cannot be sourced, fall back to the latest published `0.3.x` and re-pin Stack accordingly.
- Plugin runtime: `wasmtime` 25+ Component Model with WIT interfaces, capability-scoped. The Tauri-side capability `pulse:plugin-fs` gates the host's read of `~/.andromeda-pulse/plugins/` for module discovery and is independent of the per-guest WIT capability grants (which never include host filesystem unless a plugin's WIT explicitly imports a fs interface).
- Visualization: webview WebGPU (`<canvas>` + `navigator.gpu`, WGSL).
- LLM inference runtime: `llama.cpp` prebuilt CUDA + CPU binaries (b9305-pinned series) invoked via subprocess (D1 spawn-per-generation), behind the `LlmInferenceRunner` trait (chunk #82 surface); native JSON-schema-constrained generation via GBNF `--json-schema-file`; CUDA/CPU binary per hardware-profile tier (`-ngl 99` GPU / `-ngl 0` CPU); bounded single-turn `-st` + `-n {max_tokens}` + outer wall-clock timeout. Three documented sibling-impl swap paths through the trait (`llama-server` D2 / in-process `llama-cpp-2` / `candle`). *(Superseded the Pre-D1 `mistralrs = "=0.8.0"` choice — see `architecture-amendments.md`.)*
- Error handling: `thiserror` 2.x in modules, `anyhow` 1.x at boundaries, `serde`-friendly `AppError` enum at the IPC bridge.
- Validation: `serde` + smart enum types + `TryFrom<u16>`; no validation library by default.
- Module boundaries: Cargo workspace, one crate per module, dependency-graph enforcement.
- Build: Cargo + `cargo-xtask` for release/sign/notarize tasks.
- Deployment: `tauri-action` GitHub Action + Tauri 2 native bundlers + Tauri updater plugin (`latest.json`).
- Distribution: GitHub Releases (primary) + Homebrew tap + Scoop manifest.
- Code signing: Azure Key Vault (Windows EV) + Apple Developer ID (macOS notarization).
- Config precedence: env vars > `~/.andromeda-pulse/config.toml` > built-in defaults.
- Development Style: agent-driven.

## Existing Scopes

- **`pulse-v0_2_0-route`** — Pulse v0.2.0 evolution scope. Defined at `docs/v0_2_0/pulse-v0_2_0-route.md`. Active scope drives Epoch 9 (Foundation v0.2.0) work: chunks #57 (widget real-data binding) → #58 (curation crate extraction) → #59 (connection state machine) → #60 (triage crate scaffold + attention cue contract types) → subsequent v0.2.0 chunks (#61+ streaming baseline trackers / #62 attention cue emitter / #63 restart event detector / etc.). Registered 2026-05-16 per chunk #60 substrate landing (session 74 wrap commit `589225f`). Supporting documents: `docs/v0_2_0/pulse-capability-spec.md` (capability spec P-001 through P-060+), `docs/v0_2_0/pulse-distillation-architecture.md` (L1-L4 layered pipeline), `docs/v0_2_0/pulse-vision-and-backlog.md` (product framing + backlog).
