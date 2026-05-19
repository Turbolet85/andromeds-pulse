# andromeda-pulse

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
Cross-platform Tauri 2 desktop dashboard for local OpenTelemetry — receives OTLP telemetry from any local app, visualizes traces/metrics/logs in a GPU-accelerated webview, runs as a quarter-screen always-visible glance widget plus a full expanded dashboard, and one-click "Investigate" generates a token-efficient curated markdown snapshot for paste-to-AI debug sessions. Optional rmcp MCP stdio sidecar lets AI agents query telemetry directly. Public OSS (MIT) on GitHub Releases.

**Stack:** Rust 2024 / rustc 1.85+ + Tauri 2.x (single-process modular monolith, 12 library crates + `pulse-app` binary + `xtask`) — `tonic` 0.14 OTLP/gRPC `:4317`, `axum` 0.8 OTLP/HTTP `:4318` on `hyper` 1 + `tower`, DuckDB 1.5 in-memory ring buffer with Apache Arrow zero-copy, persistent SQLite incident corpus (chunk #68; OS-keychain-encrypted cell-level AES-256-GCM), React 19 + Tailwind v4 + WebGPU/WGSL webview via TauRPC, `wasmtime` 25+ Component Model plugins (current pin 43.0.2 — chunk #45 upgrade per security audit), `rmcp` (feature-gated) MCP sidecar.

**Key directories:**
- `crates/` — 12 library crates (`ingest` / `buffer` / `viz` / `ui-bridge` / `snapshot` / `curation` / `triage` / `workspace-detector` / `plugins` / `mcp-server` / `corpus` / `security`)
- `pulse-app/` — Tauri binary crate; `tauri.conf.json` + `capabilities/` JSON + `src/main.rs` + `ui/` webview source
- `xtask/` — cargo-xtask: release / sign / notarize / capability-drift / agent-run harness
- `.github/workflows/` — `ci.yml` matrix Linux/macOS/Windows + `release.yml` (`tauri-action`) + `update-channels.yml` (Homebrew + Scoop)
- `.andromeda/` — planning artifacts (arch / 6 specialist plans / route / scopes / runs / `context/` living artifacts)
<!-- GENERATED:setup:overview end -->

## Modules
<!-- GENERATED:setup:modules start -->
- **`ingest`** — OTLP/gRPC receiver `:4317` (tonic 0.14) + OTLP/HTTP receiver `:4318` (axum 0.8 on hyper 1 + tower); `prost` decode + post-decode invariants; mpsc hand-off to `buffer`.
- **`buffer`** — DuckDB 1.5 in-memory ring buffer + Apache Arrow zero-copy appender; periodic retention via `DELETE WHERE ts < cutoff`; broadcast fan-out.
- **`viz`** — query layer for webview WebGPU charts (`traces.*` / `metrics.*` / `logs.*` TauRPC routers); Arrow IPC channel emission.
- **`ui-bridge`** — TauRPC routers + `AppError` `serde`-friendly enum at the bridge + IPC introspection (`app_info` / `health` / `ready` / `get_settings` / `update_settings`).
- **`snapshot`** — curated markdown generator (token budget 10k/25k/50k + attribute filter + markdown formatter); orchestrates `curation::contract::*` primitives + delegates dedup/anomaly/critical-path/aggregation to `curation`; `snapshot.{generate,list_recent,copy_to_clipboard}`.
- **`curation`** — Algorithmic primitives extracted from snapshot (chunk #58 — Epoch 9 Foundation v0.2.0): `dedupe` (logical span collapse via service+name+duration bucket), `anomaly` (latency outliers / error correlation / cardinality spikes), `critical_path` (longest-duration branch extraction), `aggregation` (p50/p95/p99/max per-service + global percentiles). Exposed via `curation::contract` re-exports; consumed by `snapshot` for L3 markdown rendering + future L1a/L2 distillation chunks.
- **`triage`** — L1 streaming distillation scaffold (chunk #60 — Epoch 9 Foundation v0.2.0): 7 module skeletons (`baseline` / `pattern` / `cue` / `digest` / `interpretation` / `incident` / `lifecycle`) + 10 contract types in `triage::contract` (`AttentionCue`, `CueKind`, `CueScope`, `PriorityTier`, `Severity`, `Incident`, `IncidentStatus`, `EvidenceRefs`, `Digest`, `DigestKind`). Empty implementations; consumed by chunks #61+ for streaming baseline trackers + attention cue emitter + restart event detector per pulse v0.2.0 plan Phase 2. Capabilities P-019 (Three-Tier Severity Model contract types) + P-021 (Algorithmic Attention Cues).
- **`workspace-detector`** — host project context (`.andromeda/` marker + VCS metadata); `workspace.{detect,list}`.
- **`plugins`** — `wasmtime` 25+ Component Model host with WIT capability-scoping; loader from `~/.andromeda-pulse/plugins/`; `plugins.{list,reload,invoke}`. Chunk #45 substrate: Engine + Config posture (Cranelift on x86_64; epoch_interruption(true)) + WIT for 3 categories (custom-dashboard/data-transform/snapshot-template).
- **`mcp-server`** — `rmcp` stdio sidecar (feature-gated `--features mcp-server`); `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` `#[tool]` methods; `mcp.{status,start,stop}`.
- **`corpus`** — Persistent incident corpus SQLite backend (chunk #68 — Epoch 9 Foundation v0.2.0): 6 schema tables (`baseline_state` / `service_registry` / `pipeline_metrics` / `incidents` / `incident_events` / `digest_archive`) per dist-arch v3; first-launch idempotent migration via PRAGMA user_version; cell-level AES-256-GCM encryption with key sourced from OS keychain (macOS Keychain / Linux Secret Service / Windows DPAPI) via `keyring` crate + fake in-memory backend for tests. `CorpusReader` trait + `Corpus` connection root + `Error` enum. Consumed by `pulse-app::storage_router` for `storage.{inspect,path}` TauRPC. Capabilities P-041 / P-047–P-051.
- **`security`** — PII scrubber primitive (chunk #68 — Epoch 9 Foundation v0.2.0): 7 P-047 categories (JWT / bearer token / API key / secret KV / email / credit card / SSN) via OnceLock-cached compiled regex set; `ScrubbedValue::{Allowed, Redacted}` enum + `scrub_attribute()` fn. Consumed by `corpus` at ingestion AND eventually by `pulse-app::observability` subscriber Layer (defense-in-depth; deferred to chunk #70+).
- **`pulse-app`** — Tauri 2 binary crate; tokio runtime owner; `pulse-app/capabilities/` JSON files; bundle id `com.andromeda.pulse`.
- **`xtask`** — cargo-xtask: release / sign / notarize / changelog + agent-run 5-command harness (boot/run/status/cleanup/logs).
<!-- GENERATED:setup:modules end -->

## Critical Warnings (universal invariants)
<!-- GENERATED:setup:warnings start -->
- OTLP receivers MUST bind to `127.0.0.1` only — loopback is the de-facto authorization boundary; never `0.0.0.0` or any non-loopback interface.
- DuckDB queries MUST use prepared statements with `?` placeholders — never `format!("SELECT … WHERE service_name = '{}'", user_input)`. (2026 DuckDB CVE cluster.)
- OTLP payloads MUST pass post-`prost` invariant checks — `span_id` is 8 bytes, `trace_id` is 16 bytes, attribute keys/values bounded; `prost` only validates wire format.
- Path env vars (`ANDROMEDA_PULSE_*_PATH` / `*_DIR`) MUST canonicalize via `strict-path` and assert resolved path lives under the data dir (CWE-22 defense).
- Every TauRPC procedure MUST have a matching entry in `pulse-app/capabilities/` JSON — Tauri capabilities are negative-default; missing entry = silent runtime rejection. `xtask capability-drift` enforces.
- Self-observation NEVER dials own OTLP — `tracing` ecosystem only (no OTel SDK in self-runtime); a network exporter pointed at own `:4317`/`:4318` is infinite recursion.
- NEVER log raw OTLP attribute values, snapshot file contents, clipboard contents, MCP tool response bodies, full plugin paths, or DuckDB query parameter values — incidentally captured secrets from instrumented host apps.
- Pin every third-party GitHub Action by 40-char SHA — never `@v2` or floating tag (`tj-actions/changed-files` CVE-2025-30066 anchor).
- Errors crossing the TauRPC bridge MUST be `serde`-friendly `AppError` enum variants — convert from `thiserror`/`anyhow` via `From` impls; strip stack traces, file paths, library versions, Rust struct names.
- Every transition (chrome AND data-driven Halo State Pulse) MUST respect `prefers-reduced-motion: reduce` — Halo degrades to static glow (hue still updates per error rate); WCAG 2.1 AAA SC 2.3.3.
<!-- GENERATED:setup:warnings end -->

## Where to Look
<!-- GENERATED:setup:pointer-table start -->
| Topic | Source |
|---|---|
| Architecture overview + Established Decisions | `.andromeda/architecture.md` |
| Roadmap (9 epochs / 70 chunks) | `.andromeda/route.md` |
| Workspace crates + Occupied Resources (ports / IPC routes / env vars / tables / capabilities) | `.andromeda/architecture.md` §Inherited Defaults / §Occupied Resources |
| Standard Contracts (`app_info` / `health` / `ready` envelopes; OTLP / MCP / IPC error schemas) | `.andromeda/architecture.md` §Standard Contracts |
| Threat model + tier (Minimal) + data classifications | `.andromeda/security-plan.md` §Threat Model Summary |
| Bootstrap phases (security) + supply-chain CI gates | `.andromeda/security-plan.md` §Bootstrap phases |
| Security anti-patterns (Universal / Input / API / Code / Secrets / Logging) | `.andromeda/security-plan.md` §Security Anti-Patterns |
| Test plan (Standard tier) + 5-command harness | `.andromeda/test-plan.md` §3 |
| E2E P1–P7 critical paths | `.andromeda/test-plan.md` §6 |
| Quality gates (≥75% line / ≥70% branch / ≥85% function) + perf budgets | `.andromeda/test-plan.md` §10 |
| Observability plan + tracing self-observation harness (NO OTel SDK) | `.andromeda/obs-plan.md` §3 |
| SLO invariants + perf budgets (snapshot p99 ≤500ms / WebGPU frame p99 ≤33ms) | `.andromeda/obs-plan.md` §10 |
| A11y plan (WCAG 2.1 AA + SC 2.3.3 AAA) + harness (axe / Lighthouse / pa11y / Playwright / colorjs.io) | `.andromeda/a11y-plan.md` §3 |
| Design system (NASA Deep Space palette + Halo State Pulse) | `.andromeda/design-system.md` |
| Layout templates (compact widget + full dashboard + tray) | `.andromeda/layout-templates.md` |
| Module dependency graph (living artifact) | `.andromeda/context/dependency-tree.md` |
| API surface (living artifact) | `.andromeda/context/api-surface.md` |
| Specialist summaries (security / design / tests / obs / a11y) | `.claude/docs/{specialist}-summary.md` |
| Per-module implementation notes (12 crates) | `.claude/docs/services/{module}.md` |
| Stack / commands / conventions / gotchas / workflow | `.claude/docs/{topic}.md` |
| Path-scoped rules (security / testing / observability / a11y / verification-harness / design-tokens / frontend) | `.claude/rules/{rule}.md` |
| Session learnings (curated) + handoff (state across sessions) | `.claude/docs/session-learnings.md` + `.claude/session-handoff.md` |
| Andromeda post-MVP workflow (4 patterns + drift table + decision tree per chunk + skill mechanics refs) | `.claude/docs/andromeda-after-mvp-playbook.md` |
| Andromeda improvement proposals + dogfood friction log (where to record pipeline gaps as they surface during chunk work) | `docs/andromeda-improvements.md` |
| Pulse v0.2.0 planning material (vision, capability spec, distillation pipeline architecture, 33-chunk route plan) | `docs/v0_2_0/` |
<!-- GENERATED:setup:pointer-table end -->

## Workflow
<!-- GENERATED:setup:workflow start -->
**Key commands:**
- `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` — lint gate
- `cargo nextest run --workspace --profile ci` — test suite (per-process isolation)
- `cargo llvm-cov nextest --workspace --lcov` — coverage gate (≥75% line / ≥70% branch / ≥85% function)
- `cargo tauri build` (or `cargo xtask release`) — produce `.msi` / `.dmg` / `.AppImage` / `.deb` bundles
- `cargo audit` + `cargo deny check bans licenses sources` — supply-chain gates (catches `tonic 0.14 ↔ 0.13` duplicate)

See `.claude/docs/commands.md` for the full reference.
<!-- GENERATED:setup:workflow end -->

## Architecture
<!-- GENERATED:setup:architecture start -->
Local-first, zero-infrastructure modular monolith: every byte of telemetry stays on the developer's machine; eight library crates linked into the `pulse-app` Tauri binary share memory via tokio mpsc + broadcast (ingest → buffer → viz / MCP / snapshot subscribers) so the OS sees one process and ingest→viz latency is microseconds. Standards-track at the edges (OTLP at `:4317`/`:4318`, MCP over stdio, WASM Component Model plugins) and tightly opinionated in the middle (TauRPC bridge, Arrow zero-copy hand-off, `serde`-friendly `AppError`) — external tooling Just Works while agent-driven development stays unambiguous. Capability-scoped extensibility: WASM plugins receive only the host imports declared in their WIT; Tauri's `pulse:default` capability is the negative-default trust model (no auth required because there are no user accounts).

**Primary source:** architecture.md (imported below).
<!-- GENERATED:setup:architecture end -->

<!-- GENERATED:setup:imports start -->
@.andromeda/architecture.md
@.andromeda/route.md
@.claude/session-handoff.md
<!-- GENERATED:setup:imports end -->

<!-- Maintainer note: The @ imports above MUST each be on their own line — Claude Code only recognizes standalone @path lines as import directives. Inline references like `See @path` or `- @path` are NOT expanded. Imported files may be 300-800 lines each; the 200-line limit applies to CLAUDE.md itself, not post-expansion total. Keep @ imports minimal (3 standalone lines: arch.md / route.md / session-handoff.md). Each scope adds one more standalone line. This comment is stripped from Claude's runtime context per Anthropic comment-stripping rule. See section-markers.md. -->

## Deeper Topics
<!-- GENERATED:setup:deeper-topics start -->
On-demand references in `.claude/docs/` (Claude reads when relevant):
- Specialist summaries: `security-summary.md` / `design-summary.md` / `tests-summary.md` / `obs-summary.md` / `a11y-summary.md`
- Core: `stack.md` / `conventions.md` / `commands.md` / `gotchas.md` / `workflow.md`
- `services/{name}.md` — per-module implementation notes (`ingest` / `buffer` / `viz` / `ui-bridge` / `snapshot` / `curation` / `triage` / `corpus` / `security` / `workspace-detector` / `plugins` / `mcp-server`)
- `session-learnings.md` — curated by /wrap-session

Path-scoped rules in `.claude/rules/` (auto-load when matching files touched):
- `security.md` (universal)
- `testing.md` (Rust source + tests)
- `observability.md` (Rust source — tracing discipline)
- `a11y.md` (webview frontend)
- `verification-harness.md` (xtask + scripts/agent-run.*)
- `design-tokens.md` (webview UI)
- `frontend.md` (webview React)

For complete Andromeda documentation: `/andromeda-help`
<!-- GENERATED:setup:deeper-topics end -->

<!-- GENERATED:setup end -->

<!-- USER:session-learnings start -->
## Session Learnings
_This section is curated by `/wrap-session`. It accumulates universal (Tier 1) rules captured from work sessions — rules that apply to every file and every task. Do not edit manually during wrap-session runs — changes are preserved but wrap-session appends new entries here._

- The product IS the local observer — `tracing` ecosystem only for self-observation; an OTel network exporter pointed at own `:4317`/`:4318` is infinite recursion.
- Tauri capabilities are negative-default: every new TauRPC procedure needs both router registration AND a `pulse-app/capabilities/` JSON entry, or the bridge call is silently rejected at runtime.
- Telemetry is user-controlled content — incidentally captured secrets in OTLP attribute values mean snapshot files / clipboard / MCP tool responses / DuckDB query parameters NEVER appear verbatim in self-observation logs.
- 2026-05-16: After-MVP evolution path is `/andromeda-evolve` (within Refuse 1-6 scope) + manual edits to arch.md / specialist plans (where evolve refuses) + `/andromeda-setup-project --delta` (propagates to Tier 2/3 + CLAUDE.md ecosystem). Greenfield skills (`/andromeda-arch`, `/andromeda-route`, 6 specialists) are write-once and do NOT re-derive existing docs; `/andromeda-scope-arch` + `/andromeda-scope-route` are referenced in docs as redirect targets but are NOT implemented as separate skill folders (intentional pipeline-simplicity choice). For new chunks that introduce architecture-level concepts (new Stack member / new Established Decision / new Cross-cutting Pattern) — manual arch edit is the path; evolve flags only cover registry-section additions, not structural sections (confidence 0.9).
<!-- USER:session-learnings end -->
