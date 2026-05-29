# andromeda-pulse

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
Cross-platform Tauri 2 desktop dashboard for local OpenTelemetry — receives OTLP telemetry from any local app, visualizes traces/metrics/logs in a GPU-accelerated webview, runs as a quarter-screen always-visible glance widget plus a full expanded dashboard, and one-click "Investigate" generates a token-efficient curated markdown snapshot for paste-to-AI debug sessions. Optional rmcp MCP stdio sidecar lets AI agents query telemetry directly. Public OSS (MIT) on GitHub Releases.

**Stack:** Rust 2024 / rustc 1.85+ + Tauri 2.x (single-process modular monolith, 13 library crates + `pulse-app` binary + `xtask`) — `tonic` 0.14 OTLP/gRPC `:4317`, `axum` 0.8 OTLP/HTTP `:4318` on `hyper` 1 + `tower`, DuckDB 1.5 in-memory ring buffer with Apache Arrow zero-copy, persistent SQLite incident corpus (chunk #68; OS-keychain-encrypted cell-level AES-256-GCM), React 19 + Tailwind v4 + WebGPU/WGSL webview via TauRPC, `wasmtime` 25+ Component Model plugins (current pin 43.0.2 — chunk #45 upgrade per security audit), `rmcp` (feature-gated) MCP sidecar, `llama.cpp` prebuilt binaries (b9305-pinned series) invoked via subprocess D1 spawn-per-generation as the L4 LLM interpretation runtime (chunk #82 trait surface; session 144 runtime swap from `mistralrs` per upstream issue #1134 CPU deadlock — empirical cross-check showed llama.cpp generates cleanly at 122.3 tok/sec under L4 GBNF constraint on RTX 3090 vs mistralrs zero-token deadlock on the same host; `interpretation` crate hosts the trait surface, concrete impl swaps `MistralRsInference` → `LlamaCliInference` at the binary boundary).

**Key directories:**
- `crates/` — 13 library crates (`ingest` / `buffer` / `viz` / `ui-bridge` / `snapshot` / `curation` / `triage` / `workspace-detector` / `plugins` / `mcp-server` / `corpus` / `security` / `interpretation`)
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
- **`interpretation`** — L4 LLM interpretation layer (chunk #82 — Epoch 9 Foundation v0.2.0): `LlmInferenceRunner` async trait (Pin<Box<dyn Future + Send + 'a>> returns mirroring 2026-05-23 `SqlQueryRunner` precedent) + `ModelTier` / `ModelStatus` / `ModelIdentity` / `ModelLoadEvent` / `InferenceError` contract types in `interpretation::contract` + `HardwareProfileDetector` implementing chunk #80 `HardwareProfileSource` trait (cross-platform GPU probe: Metal on macOS, libcuda.so on Linux, nvcuda.dll on Windows; env var override via `ANDROMEDA_PULSE_HARDWARE_PROFILE`) + `ModelStatusBroadcast` for `pulse://stream/model-status` topic. NO direct `mistralrs` import at this crate level — concrete `MistralRsInference` impl lives at the binary boundary in `pulse-app/` per arch §Established Decisions [LLM Inference Runtime — L4 interpretation layer] bus factor mitigation entry. Capabilities P-053 (Fallback Model Tier — detection side) + P-054 (Hardware Profile Awareness).
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
- Every transition (chrome AND data-driven Halo State Pulse) MUST respect `prefers-reduced-motion: reduce` — Halo degrades to static glow (hue still updates per cumulative incident severity); WCAG 2.1 AAA SC 2.3.3.
<!-- GENERATED:setup:warnings end -->

## Where to Look
<!-- GENERATED:setup:pointer-table start -->
| Topic | Source |
|---|---|
| Architecture overview + Established Decisions | `.andromeda/architecture.md` |
| Roadmap (9 epochs / 91 chunks) | `.andromeda/route.md` |
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
Local-first, zero-infrastructure modular monolith: every byte of telemetry stays on the developer's machine; twelve library crates linked into the `pulse-app` Tauri binary (fourteen workspace members total: twelve library crates + the `pulse-app` binary + the `xtask` task-runner crate) share memory via tokio mpsc + broadcast (ingest → buffer → viz / MCP / snapshot subscribers) so the OS sees one process and ingest→viz latency is microseconds. Standards-track at the edges (OTLP at `:4317`/`:4318`, MCP over stdio, WASM Component Model plugins) and tightly opinionated in the middle (TauRPC bridge, Arrow zero-copy hand-off, `serde`-friendly `AppError`) — external tooling Just Works while agent-driven development stays unambiguous. Capability-scoped extensibility: WASM plugins receive only the host imports declared in their WIT; Tauri's `pulse:default` capability is the negative-default trust model (no auth required because there are no user accounts).

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
- 2026-05-20: Migration helpers MUST preserve source data on failure for retry safety — only delete the legacy source on FULL success. Partial migration (data written to new sink but cleanup failed) emits a degraded-success outcome WITHOUT removing the legacy file. Failed migration (deserialize / encryption / write error) leaves the legacy file fully intact so next boot can retry. The lesson generalizes to any future schema migration / persistence relocation / file relocation chunk — never auto-delete source on partial state. Verified at chunk #70 BaselineState→corpus migration where `migrate_legacy_baseline_if_present` distinguishes Completed (legacy_file_deleted true/false) vs Failed (legacy file preserved) outcomes; the at-rest test seeded canary substrings + asserted both data-side integrity AND legacy-side preservation paths.
- 2026-05-20: `bincode` 1.3.x default `deserialize` is UNSAFE on untrusted input — pre-allocates `Vec` / `HashMap` capacity from the length prefix BEFORE attempting к read entries, so a crafted byte stream with a valid-looking 8-byte usize prefix (e.g., `b"\x00\x01\x02 garbage"` decodes its first 8 bytes as ~7e18) triggers an immediate OOM process abort rather than graceful `Err`. `bincode::Options::with_limit(N)` only protects FIXED-SIZE sequences (serialized_size predictable per entry); map-shaped types whose entries have variable size (`DashMap<String, ServiceBaseline>`, `Vec<TemplateRecord>`) STILL OOM. Mitigation at any untrusted-input boundary calling `bincode::deserialize`: add type-specific prefix plausibility validator that parses the u64 length prefix at known struct offset + rejects implausible counts BEFORE bincode sees the bytes. Encrypted-corpus load paths are mitigated by AES-256-GCM integrity tag (attacker needs key compromise to weaponize), but legacy/migration paths reading unencrypted files are fully exposed. Verified at chunk #72 `pulse-app/src/baseline_persistence.rs::baseline_bytes_prefix_plausible` (rejects services-map prefix > 10M). Real fix is bincode 2.x upgrade (try_reserve-based safer allocations) — track via `pulse-app/src/bincode_bounded.rs` upgrade hook.
- 2026-05-20: When external code needs к scrub `pub(crate)` fields on a struct exported via `pub use` from a lower crate, add `pub fn scrubbed_clone<F: Fn(&str) -> String>(&self, scrub: F) -> Self` method ON the lower-crate side (same-crate access permits writing the field). The closure is dep-injected by the upper-crate adapter (e.g., pulse-app passes `security::scrubber::scrub_attribute` as the closure body), preserving the leaf-crate-no-deps invariant per arch §Module dependency direction (leaf crates like `triage` MUST NOT depend on sibling crates like `security` к maintain DAG discipline). Verified at chunk #72 `crates/triage/src/pattern/persistence.rs::StormStateSnapshot::scrubbed_clone` + `crates/triage/src/baseline/mod.rs::BaselineState::scrubbed_clone`. Pairs naturally с the 2026-05-16 trait-in-lower-crate state pattern + 2026-05-18 free-fn map_err: trait-injection for state + closure-injection for transforms + free-fn for errors → all three keep the leaf-crate surface narrow while exposing the operation upper crates need. Apply к any future cross-crate transform where the lower crate's data shape has `pub(crate)` fields.
- 2026-05-25: When choosing between in-process bindings vs subprocess-spawn vs persistent-server for a heavy upstream dep, ground the choice in the ACTUAL invocation rate observed in the codebase (read the subscriber/scheduler logic + cadence config defaults), not theoretical maximum throughput. Bindings ship build-toolchain dep (e.g., Rust bindings via `bindgen` need `cmake` + `libclang.dll` + MSVC on Windows — a 3-tool stack discoverable only by spike); subprocess pays a per-call cold-start tax but zero idle cost (releases RAM/VRAM between calls); persistent-server amortizes load but adds lifecycle complexity (port + health probe + restart loop + always-resident memory). For background-cadence-driven invocation with no user-waiting UX (≤few calls/min), subprocess often wins on simplicity + local-first respect for the user's machine. The trait abstraction at the binary boundary keeps the swap path open between strategies regardless of which is chosen first. Verified at chunk #84 LLM runtime swap (session 144): empirical L4 invocation rate 1-4/min cadence-driven (no user-action trigger в the codebase) → subprocess D1 chosen over both in-process bindings (libclang+cmake+MSVC build cost not justified) and persistent llama-server (lifecycle complexity not justified).
- 2026-05-26: HYBRID RENDER PATTERN — when implementing а chunk whose plan assumes а capability that doesn't yet exist (e.g., chunk #88 Diagnostic Report assumed retrievable L4Output for active incidents, but reality persisted L4Output ONLY for Resolved incidents via chunk #86 `resolution_summary_text` JSON-encode path), three resolutions are possible: (a) **extend scope** к add the missing capability (expands plan; touches "untouched" files); (b) **defer chunk** к а follow-up persistence chunk first (slowest path; introduces /andromeda-evolve route-append cycle); (c) **hybrid render** — render с available data + explicit degraded-mode notice in place of the missing data (zero-delta scope; reuses existing degraded-mode UX from chunk #86 FSM). Hybrid often wins on simplicity AND preserves graceful degradation as а first-class UX pattern (per-incident render branches based on data availability: Resolved-with-L4 → full six-section, Active/Acknowledged OR Resolved-but-unparseable → degraded-mode с explicit "Interpretation pending" notice). Decision criterion: if а documented degraded-mode invariant already exists (chunk #86 FSM, the user-visible "retry interpretation" path), Hybrid leverages an existing pattern; if not, extension OR defer are appropriate. Apply whenever а chunk's plan.md Step 1 inspection surfaces "capability assumed but not yet implemented" as the Open Question — surface к user via AskUserQuestion с the three options + clear cost/benefit framing rather than silently extending OR silently deferring. Verified at chunk #88 implementation: Phase 1 user-dialogue selected Hybrid → ~1500 LOC implementation landed end-to-end в single session с zero new workspace deps + zero new corpus tables + zero arch-§Occupied-Resources additions beyond the single TauRPC procedure + clean integration with chunk #86 degraded-mode UX (workspace nextest 1523→1544 + 1 skip + capability-drift clean).
<!-- USER:session-learnings end -->
