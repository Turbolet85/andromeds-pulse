# Technology Stack

_Extracted from `.andromeda/architecture.md` Stack and Technologies section by `/setup-project`. Primary source: architecture.md — this file is a convenience reference that Claude can read on demand when it needs stack details._

## Languages & Runtimes
- **Primary language:** Rust 2024 edition (rustc 1.85+ minimum — Edition 2024 cannot parse below 1.85; security-positive defaults `unsafe_op_in_unsafe_fn`, `unsafe extern`, `static mut` reference denial, tightened `if let` temporary scopes require this).
- **Async runtime:** `tokio` (current stable) — single multi-threaded runtime serving both OTLP ports + IPC.
- **Frontend runtime:** WebView2 (Windows) / WKWebView (macOS) / GTK WebKit (Linux) via Tauri 2.x; React 19.x for UI.
- **Package manager:** Cargo (workspace).

## Core Frameworks
- **Desktop shell:** Tauri 2.x — native window + webview, OS bundlers, updater, tray, notifications. Bundle id `com.andromeda.pulse`.
- **gRPC server (OTLP `:4317`):** `tonic` 0.14.x with `prost` 0.14 codegen against `opentelemetry-proto`.
- **HTTP server (OTLP `:4318`):** `axum` 0.8.x on `hyper` 1.x + `tower` (sharing tokio runtime with tonic).
- **Tauri IPC bridge:** TauRPC (`taurpc` crate) — derive macros generate fully-typed TypeScript bindings; eliminates manual TS re-declaration drift.
- **Plugin runtime:** `wasmtime` 25+ with WASM Component Model + WIT — capability-scoped third-party extensions.
- **MCP server:** `rmcp` (official Rust SDK) over stdio — `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` `#[tool]` methods (gated by `--features mcp-server`).
- **Frontend:** React 19.x + Vite + TanStack Router + Tailwind CSS v4.x + shadcn/ui (Radix UI primitives + Tailwind, copy-not-install).
- **Visualization:** Webview WebGPU (`<canvas>` + `navigator.gpu`, WGSL shaders) for trace timeline / flamegraph / metrics charts / Halo State Pulse.
- **Error handling:** `thiserror` 2.x (modules) + `anyhow` 1.x (boundaries) + `serde`-friendly `AppError` enum at the IPC bridge.
- **Validation:** `serde` + smart enum types + `TryFrom<u16>` (no validation library by default; defer `garde` 0.20+ to plugin manifest cross-field validation if/when needed).

## AI / LLM Inference (v0.2.0+, pending chunk #82 onwards)
- **Local LLM runtime:** `mistralrs` 0.8.0 (pin-exact, no caret) — chosen over `candle` per Pre-D1 decision (2026-05-24) because native JSON-constrained generation via grammar enforcement + strict schema mode (llguidance integration) is essential for the L4 interpretation pipeline that emits structured digest reports.
- **Quantization formats supported:** GGUF (2-8 bit), GPTQ, AWQ, HQQ, FP8, BNB.
- **Backends:** Metal (FlashAttention V2/V3 + PagedAttention), CUDA, CPU — matches hardware-profile-aware tier model (`gpu-primary` / `gpu-fallback` / `cpu-primary` / `cpu-fallback` per chunk #82 detection).
- **Tokenizer:** `tokenizers` crate (already pulled in via chunk #81 digest assembler; shared workspace dep).
- **Abstraction:** `pub trait LlmInferenceRunner: Send + Sync` in `crates/triage/src/contract` with `Pin<Box<dyn Future + Send + 'a>>` return types (async-trait pattern matching the 2026-05-23 `SqlQueryRunner`); concrete `MistralRsInference` impl at `pulse-app/` binary boundary. Bus factor mitigation against mistral.rs smaller community (7,171 stars vs candle's 20,341); swap to `candle` + `outlines-rs` if maintenance falters is a single-impl change, not a workspace rewrite.
- **Implementation status:** scheduled in chunks #82–#85 per route §Epoch 9 (Foundation v0.2.0); not yet in code. Workspace dep `mistralrs = "=0.8.0"` lands with chunk #82.

## Data Storage
- **Storage engine:** DuckDB 1.5.x via `duckdb` crate 1.10500.x — embedded columnar OLAP, in-memory `:memory:` ring buffer (5–10 min retention, configurable via `ANDROMEDA_PULSE_RETENTION_SECONDS`).
- **Columnar interchange:** Apache Arrow via `Appender::append_record_batch()` / `stream_arrow()` — zero-copy hand-off between OTLP decode → DuckDB → viz/MCP.
- **ORM / Migrations:** None — direct SQL via `duckdb` crate `Connection` + `Appender`; schema created on startup.
- **Schema name:** `pulse_buffer` (single in-memory connection, schema `main`).
- **Reserved tables:** `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`.

## Messaging & Events
- **In-process channels:** `tokio::sync::mpsc` (ingest → DuckDB appender hand-off, built-in backpressure) + `tokio::sync::broadcast` (buffer → live UI subscribers fan-out).
- **External message broker:** N/A — single-process desktop app.
- **Real-time push contract:** Tauri 2 IPC `Channel` API with binary Arrow IPC payloads; events `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`, `pulse://stream/snapshot-progress`, `pulse://stream/plugin-events`. SSE/WebSocket NOT used (consumer is in-process).

## Observability
- **Self-observation runtime:** `tracing` 0.1 + `tracing-subscriber` 0.3 (JSON formatter + `EnvFilter`) + `tracing-appender` 0.2 (daily-rolling file sink, non-blocking writer) + `tracing-error` 0.2 (SpanTrace).
- **NO OTel SDK linked** into the self-observation runtime (recursion-free by construction; product IS the local observer).
- **Log file:** `~/.andromeda-pulse/logs/agent-latest.jsonl` (per-platform per arch §Filesystem locations); JSON-per-line; daily rotation.
- **Optional error reporting:** `sentry-rust` 0.46 + `sentry-tauri` 0.5 — opt-in via `ANDROMEDA_PULSE_SENTRY_DSN`, default OFF, requires `before_send` scrubbing.
- **Frontend telemetry:** `web-vitals` 5.x + DOM `performance.now()` + `device.queue.onSubmittedWorkDone()` → TauRPC `telemetry.frontend.record_*` → backend `tracing` log. NO browser OTel SDK.

## Development & CI
- **Lint:** `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- **Typecheck:** `cargo check --workspace --all-targets` (implicit) + `tsc --noEmit` for TauRPC-generated `.d.ts` in webview.
- **Test framework:** `cargo test` (libtest, rustc 1.85+) + `cargo-nextest` 0.9.x for per-process isolation.
- **Coverage:** `cargo-llvm-cov` 0.8.5 (LLVM source-based, cross-platform).
- **Property testing:** `proptest` 1.10.0 (regression files in `proptest-regressions/`).
- **CI task runner:** `cargo-xtask` (release / sign / notarize / changelog + agent-run harness).
- **CI platform:** GitHub Actions with `tauri-action` + `harden-runner` (SHA-pinned).
- **Supply chain:** `cargo-audit` 0.22.1 + `cargo-deny` 0.19.4 + `cargo-auditable` 0.7.4 + Dependabot (cargo + github-actions ecosystems).

## Infrastructure
- **Deployment target:** Cross-platform desktop app (Windows / macOS / Linux). NOT a web app, NOT a CLI fleet, NOT a microservice cluster.
- **Bundler:** `tauri-action` GitHub Action + Tauri 2 native bundlers — produces `.msi` (Windows) / `.dmg` (macOS, Apple silicon + x86_64; `.app` wrapped inside) / `.AppImage` + `.deb` (Linux).
- **Code signing:** Azure Key Vault Premium SKU (HSM-RSA Windows EV cert via DigiCert/GlobalSign) + Apple Developer ID (macOS notarization) + Tauri updater Minisign Ed25519. GitHub OIDC federation; no long-lived service principal secret in GitHub Actions.
- **Distribution:** GitHub Releases (primary) + Homebrew tap + Scoop manifest via `update-channels.yml` triggered on `release.yml` completion.
- **Updater:** `tauri-plugin-updater` 2.x consuming `latest.json` from GitHub Releases; updater public key baked into `tauri.conf.json`.
- **Capability identifiers:** `pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs` — concrete JSON in `pulse-app/capabilities/`.

## Third-party services
- N/A — local-only OSS desktop app. No payment provider, no auth provider, no email, no analytics. Outbound network is updater HTTPS to GitHub Releases only (rustls TLS 1.2+).

## Rationale

For architectural rationale behind these choices, see `.andromeda/architecture.md` Established Decisions section. Each `[Tag]` in that section explains why a specific technology was chosen over alternatives considered during `/andromeda-arch`.

## Open reconciliations (deferred)
- `tonic 0.14.x` vs `opentelemetry-otlp 0.31` (which still pins `tonic 0.13` in some feature combinations) — `cargo deny check bans` enforces; resolve before tagging v0.1.0.
- `rmcp` "1.5.0" reference vs published `0.3.x` line — verify whether forward-looking, internal spec name, or unrelated `4t145/rmcp` fork.
- `rust-toolchain.toml` minimum was `1.84` — bump to `1.85.0` to align with `Cargo.toml edition = "2024"`.

## Version updates

When updating a dependency version:
1. Update `.andromeda/architecture.md` Stack table first
2. Update `Cargo.toml` (or `pulse-app/ui/package.json` for webview deps)
3. Re-run `/andromeda-setup-project` to refresh this file and any rule files that reference the tool version
4. Test that hooks and tooling still work
5. Commit as `chore: bump {tool} to {version}`
