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
- **Visualization:** Webview WebGPU (`<canvas>` + `navigator.gpu`, WGSL shaders) for trace timeline / flamegraph / metrics charts / service constellation. (The Halo State Pulse canvas layer is specified but unbuilt on desktop-webview — measured 2026-08-21.)
- **Error handling:** `thiserror` 2.x (modules) + `anyhow` 1.x (boundaries) + `serde`-friendly `AppError` enum at the IPC bridge.
- **Validation:** `serde` + smart enum types + `TryFrom<u16>` (no validation library by default; defer `garde` 0.20+ to plugin manifest cross-field validation if/when needed).

## AI / LLM Inference (v0.2.0+, chunk #82 trait surface + session 144 runtime swap)
- **Local LLM runtime:** `llama.cpp` prebuilt binaries (b9305-pinned series) invoked via subprocess (D1 spawn-per-generation pattern), swapped from the original `mistralrs = "=0.8.0"` choice per session 144 (2026-05-25) empirical invalidation: upstream issue [EricLBuehler/mistral.rs#1134](https://github.com/EricLBuehler/mistral.rs/issues/1134) ("Endless inferencing with cpu", OPEN since 2025-02-12) deadlocks `mistralrs` CPU sampler threads on Windows MSVC hosts; cross-checked `llama.cpp` against the EXACT same GGUF on the EXACT same host generated tokens cleanly (28.2 tok/sec CPU, 231.2 tok/sec unconstrained CUDA RTX 3090, 122.3 tok/sec under L4 `--json-schema-file` GBNF constraint, ~4.3 s for a complete schema-conformant L4 output).
- **Invocation pattern (D1 — chosen over D2 `llama-server` HTTP):** spawn `llama-cli.exe` per L4 digest via `tokio::process::Command` with `kill_on_drop(true)`; bound by explicit `-n {max_tokens}` cap + `-st` single-turn flag + outer `tokio::time::timeout` wall-clock guard (defense-in-depth — two unbounded-generation runaways during the spike work; never repeat). D2 rejected because real L4 invocation rate is 1-4/min (cadence-coordinator-driven background subscriber at `pulse-app/src/inference_runtime.rs::spawn_l4_inference_subscriber` consuming `pulse://stream/digests`; no user-action L4 trigger in the codebase), well under D1's ~6-10 calls/min capacity, and D1 releases RAM + VRAM between cadence ticks (local-first respect for the user's machine).
- **Quantization formats supported:** GGUF (2-8 bit) is the production runtime format; the b9305+ build line is the upstream-most GGUF runtime + tracks llama.cpp's ongoing kernel optimizations.
- **Backends:** CUDA + CPU prebuilt binaries from official `ggml-org/llama.cpp` GitHub releases. Tier routing follows chunk #80 `HardwareProfileSource` trait — gpu-primary / gpu-fallback → CUDA binary + `-ngl 99`; cpu-primary / cpu-fallback → CPU binary + `-ngl 0`. CUDA build matched to the host driver's max-supported toolkit (currently CUDA 13.1 against driver 596.36 on the spike host). Metal / Vulkan / SYCL / HIP available in llama.cpp's upstream prebuilt series for future hardware-profile expansion.
- **JSON-schema constraint mechanism:** llama.cpp's native `--json-schema-file <path>` flag → GBNF grammar enforcement at the sampler layer. The L4 schema at `crates/interpretation/src/schema.json` is passed directly; no custom JSON-Schema → GBNF bridge needed (the spike confirmed full structural + enum + length-bound conformance with all 14 required fields populated on a real model run).
- **Tokenizer:** llama.cpp's built-in tokenizer per the loaded GGUF (the GGUF format includes tokenizer metadata); no separate `tokenizers` crate dep for the inference path. (The `tokenizers` workspace dep from chunk #81 remains for prompt-side token-budget estimation pre-LLM, unrelated to the runtime swap.)
- **Abstraction:** `pub trait LlmInferenceRunner: Send + Sync` in `crates/interpretation/src/contract.rs` with `Pin<Box<dyn Future + Send + 'a>>` return types (async-trait pattern matching the 2026-05-23 `SqlQueryRunner`); unchanged from chunk #82 — only the concrete impl swaps from `MistralRsInference` to a new `LlamaCliInference` sibling at the `pulse-app/` binary boundary. Three documented sibling-impl swap paths through the same trait: `llama-server` HTTP (D2) if invocation rate / UX demands warrant amortizing the ~5 s cold-start tax, in-process `llama-cpp-2` bindings if Windows libclang + cmake + MSVC build-toolchain cost is later justified, `candle` if needed (original chunk #82 escape hatch remains valid).
- **Pin discipline:** `b9305` exact at v0.2.0 swap-in; bumps as deliberate chunk-scoped events, never via "latest tag" drift. Mirrors the original `mistralrs = "=0.8.0"` pin-exact discipline.
- **Binary distribution path:** **flagged-pending** — the runtime-swap chunk MUST decide whether the CUDA + CPU `llama-cli.exe` binaries (+ `cudart64_13.dll` / `cublas64_13.dll` for the CUDA build) ship in the Tauri bundle, are downloaded by an `xtask` step at boot, or rely on a user-set `ANDROMEDA_PULSE_LLAMA_CUDA_BIN` / `ANDROMEDA_PULSE_LLAMA_CPU_BIN` env var pair pointing at user-managed install (the current dev pattern from session 144 spike). The distribution decision is parallel to the model-file distribution (`ANDROMEDA_PULSE_MODEL_PATH` env var) and should be co-designed.
- **Implementation status:** chunk #82 trait surface + stub concrete `MistralRsInference` substrate landed (session 139 commit cf6686b); chunk #83 prompt scaffolding + subscriber substrate landed (session 142 commit 0e37159); runtime-swap chunk (replace `MistralRsInference` → `LlamaCliInference`) is the next implementation work to plan via `/andromeda-phase`, positioned in route §Epoch 9 before chunk #84 fallback-tier work.

## Data Storage
- **Storage engine:** DuckDB 1.5.x via `duckdb` crate 1.10500.x — embedded columnar OLAP, in-memory `:memory:` ring buffer (5–10 min retention, configurable via `ANDROMEDA_PULSE_RETENTION_SECONDS`).
- **Columnar interchange:** Apache Arrow via `Appender::append_record_batch()` / `stream_arrow()` — zero-copy hand-off between OTLP decode → DuckDB → viz/MCP.
- **ORM / Migrations:** None — direct SQL via `duckdb` crate `Connection` + `Appender`; schema created on startup.
- **Schema name:** `pulse_buffer` (single in-memory connection, schema `main`).
- **Reserved tables:** `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes` (+ `log_templates`). Reservation ≠ a live write path: only `spans`, `span_events`, `metrics_points` and `log_records` are written; `span_links`, `resources` and `instrumentation_scopes` are declared and swept but producer-less (measured 2026-08-23).
- **Secret / key storage:** `keyring` 3.x declared with the explicit platform feature set (`apple-native` / `windows-native` / `sync-secret-service` / `crypto-rust`) — the OS credential store holding the corpus AES-256-GCM cell key (macOS Keychain / Linux Secret Service / Windows Credential Manager), plus `blake3` as the KDF for the opt-in `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` fallback. **The feature set is load-bearing:** keyring 3.x declares no `default` feature, so a bare `keyring = "3"` links no backend and silently yields a per-process key (the defect fixed at chunk 2026-08-15-corpus-key-persistence).

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
- **GUI verification harness (dev-only):** `@crabnebula/tauri-driver` 2.x + `webdriverio` 9.x (`pulse-app/ui` devDependencies), driven by `cargo xtask webview-drive [--expect-absent <STAGE>] [--no-inject]` — a headful WebDriver drive of the assembled 16-stage path (launch → traces-empty → traces-populate → traces-scroll → native-menu-suppressed → connection-status → storm-incident → findings-window → report-window → report-copy → investigate → empty-states → dashboard-toggle → dashboard-close → widget-close → signpost-repeat; 7 → 13 at 2026-08-23-headful-leg-extension, 13 → 15 at 2026-08-24-headful-mechanics-probe-race-disposition, 15 → 16 at 2026-08-27-report-window-copy-affordance, where `widget-close` stopped being terminal) in the live Tauri window, under the deterministic-L4 gate the driver sets into the spawned app's env, asserting each stage against the app's own obs log and/or the driver's DOM report; `--no-inject --expect-absent <STAGE>` is the code-driven RED mutation arm. The win32 native driver arrives via the napi optional dep; the host `msedgedriver` must match the installed WebView2 Runtime and is located via `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` (never committed). No Rust dependency, no runtime/bundle impact.
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
