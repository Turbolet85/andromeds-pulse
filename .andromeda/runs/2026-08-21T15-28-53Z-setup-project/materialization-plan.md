# Materialization Plan — andromeda-pulse

**Run:** `2026-08-21T15-28-53Z-setup-project` · **Mode:** RE-RUN (18 `GENERATED:setup` markers present)
**Development Style:** agent-driven (arch §Cross-cutting Patterns)
**Trigger:** pipeline-template improvement — the code-graph gained a TypeScript plane (per-plane DBs under `cache/{plane}/`)

Phase 0 is the only synthesis pass. Phases 1–6 read ONLY this file.

**Upstreams read (9/9):** `input.md` (375L) · `architecture.md` (366L) · `security-plan.md` (443L) · `design-system.md` (426L) · `layout-templates.md` (397L) · `test-plan.md` (774L) · `obs-plan.md` (711L) · `a11y-plan.md` (699L) · `master-route.md` (47L). Five masters were amended 2026-08-21 (arch · design-system · layout-templates · obs-plan · security-plan); test-plan 2026-08-17; a11y-plan 2026-07-10.

---

## Tier 1 — CLAUDE.md

Target ≤200 lines (current 155). All `GENERATED:setup:*` regenerated; `USER:session-learnings` preserved verbatim.

### `setup:overview`
Cross-platform Tauri 2 desktop dashboard for local OpenTelemetry. Receives OTLP from any local app, visualizes traces/metrics/logs in a GPU-accelerated webview, runs as a quarter-screen always-visible glance widget plus a full expanded dashboard, and one-click "Investigate" generates a token-efficient curated markdown snapshot for paste-to-AI debugging. Optional rmcp MCP stdio sidecar lets AI agents query telemetry directly. Public OSS (MIT) on GitHub Releases.

**Stack line:** Rust 2024 / rustc 1.85+ + Tauri 2.x (single-process modular monolith, 14 library crates + `pulse-app` + `xtask`) — `tonic` 0.14 OTLP/gRPC `:4317`, `axum` 0.8 OTLP/HTTP `:4318` on `hyper` 1 + `tower`, DuckDB 1.5 in-memory ring buffer + Arrow zero-copy, persistent SQLite incident corpus (cell-level AES-256-GCM; key from OS credential store via `keyring` 3 with its explicit platform feature set), React 19 + Tailwind v4 + WebGPU/WGSL via TauRPC, `wasmtime` Component Model plugins, `rmcp` (feature-gated) MCP sidecar, `llama.cpp` prebuilt binaries (b9305) via subprocess D1 spawn-per-generation as the L4 LLM runtime.

**Key directories:** `crates/` (14 library crates) · `pulse-app/` (Tauri binary + `capabilities/` + `ui/`) · `xtask/` (release/sign/notarize/capability-drift/agent-run) · `.github/workflows/` · `.andromeda/` (planning artifacts).

**Count reconciliation (carry forward, do not "fix" here):** `Cargo.toml [workspace] members` lists exactly 14 library crates + `pulse-app` + `xtask` = **16 workspace members**. arch §Occupied Resources §Cargo workspace crate names matches that list and is declared canonical by arch itself ("Occupied Resources is the canonical workspace-member list; any claim of a count word elsewhere refers back to it"). arch §Design Philosophy's "twelve library crates / fourteen workspace members" phrasing is stale but explicitly deferred by that rule — arch is not setup-project's territory. `crates/triage-experimental/` is on disk but is NOT a workspace member (`publish = false`, a spike) — correctly absent from the module map. **CLAUDE.md says 14 library crates — unchanged.**

### `setup:modules`
14 library crates + the binary + xtask, one line each, from arch §Occupied Resources + §Modules: `ingest` · `buffer` · `viz` · `ui-bridge` · `snapshot` · `curation` · `triage` · `workspace-detector` · `plugins` · `mcp-server` · `corpus` · `security` · `interpretation` · `config-watcher` · `pulse-app` · `xtask`. Content carried from current CLAUDE.md (already derived from these same masters, re-verified this pass).

### `setup:warnings` — top 11 universal invariants
Severity priority security > a11y > obs > tests > design; every entry must hold for **every** file.

1. OTLP receivers MUST bind `127.0.0.1` only — loopback is the de-facto authorization boundary; never `0.0.0.0`. *(security §API)*
2. DuckDB queries MUST use prepared statements with `?` placeholders — never `format!` into SQL. *(security §Input; 2026 DuckDB CVE cluster)*
3. OTLP payloads MUST pass post-`prost` invariant checks — `span_id` 8 bytes, `trace_id` 16 bytes, attribute keys/values bounded. *(security §Input)*
4. Path env vars (`ANDROMEDA_PULSE_*_PATH` / `*_DIR`) MUST canonicalize via `strict-path` and resolve under the data dir. *(security §Input; CWE-22)*
5. Every TauRPC procedure MUST have its `EXPECTED_PROCEDURES` pin in `xtask/src/main.rs` plus a validated argument struct — NOT a per-procedure `pulse-app/capabilities/` entry (TauRPC dispatches through one invoke handler; measured 2026-08-21). Capabilities stay NEGATIVE-DEFAULT at IPC-layer and per-CORE-API granularity: `fs`/`shell`/`dialog`/`http` and each `core:window:*` need an explicit grant and ARE silently rejected when missing. *(security §API — corrected form, preserved verbatim)*
6. Self-observation NEVER dials own OTLP — `tracing` ecosystem only, no OTel SDK in the self-runtime. *(arch §Cross-cutting + obs §Universal)*
7. NEVER log raw OTLP attribute values, snapshot file contents, clipboard contents, MCP tool response bodies, full plugin paths, or DuckDB query parameter values. *(security §Logging + obs §Logs)*
8. Pin every third-party GitHub Action by 40-char SHA — never a floating tag. *(security §Secrets; `tj-actions/changed-files` CVE-2025-30066)*
9. Errors crossing the TauRPC bridge MUST be `serde`-friendly `AppError` variants — strip stack traces, file paths, library versions, Rust struct names. *(security §Error + arch §Conventions)*
10. Every transition (chrome AND data-driven Halo State Pulse) MUST respect `prefers-reduced-motion: reduce`. *(a11y §Motion; WCAG SC 2.3.3 AAA)*
11. Fault identity is DECIDED, not inferable from code shape — token-leading stacktrace normalization; incidents coalesce per cue identity `(kind, scope, scope_id)`; `Incident.fingerprint` uses `triage::contract::hex_lower`, never `fingerprint_to_hex_prefix`; the `fingerprint_match` retrieval arm is FED. *(arch §Established Decisions [Fault Identity])*

**Rejected candidates (audit trail — path-scoped, so Tier 2/3 instead):** wasmtime Cranelift-only on x86_64 (→ `rules/security.md` §Plugin host) · MCP compile+runtime double-gate (→ `rules/security.md` §MCP) · axum `DefaultBodyLimit` 8 MB + tonic `max_decoding_message_size` (→ `rules/security.md` §Loopback) · `bincode` 1.3 `deserialize` OOM on untrusted input (already a `USER:session-learnings` Tier-1 entry — not duplicated) · ARIA-on-non-semantic-HTML ban (→ `rules/a11y.md`) · banned generic fonts / Tailwind default palette (→ `rules/design-tokens.md`) · zero-flakiness / no retry-once (→ `rules/testing.md`) · split `cargo deny` invocations (→ `rules/security.md` §Session Additions 2026-08-17).

### `setup:pointer-table`
Carry the current 29-row table with **two corrections** and **one required update**:

- **UPDATE (the template-absorption delta):** the code-graph row becomes per-plane —
  `| Code map / impact (symbols · callers · crate deps) | `.andromeda/cache/{plane}/tree.db` (planes: `rust` · `ts`) — query via `scripts/code-graph.py query <run_dir> <marker> "<sql>" [plane]`; cookbook `scripts/code-graph-cookbook.md` |`
- **REMOVE (dead pointers — verified absent at HEAD):** the two living-artifact rows naming `.andromeda/context/dependency-tree.md` and `.andromeda/context/api-surface.md`. **`.andromeda/context/` does not exist** (probed this pass). The artifacts were retired when `tree.db` landed; only the CLAUDE.md pointers lingered. Health Check 3 validates `^@` imports only, so pointer-table rows are unguarded — this is precisely the "does the documented thing exist at HEAD?" blind class named in the 2026-08-21 Tier-1 rule. Removing a pointer to a nonexistent file is regeneration of setup's own section; the working-route entry "Demo injector formalized + api-surface retire" keeps its remaining (demo-injector) work.
- No baked version-workspace paths: the roadmap rows cite `.andromeda/master-route.md` (version-agnostic) plus the pre-v3 forensic `route.md` / `phases/`; `docs/v0_2_0/` is a docs dir, not a version workspace. Compliant with the template's never-bake rule.

Result: **28 rows.**

### `setup:workflow`
`cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` · `cargo nextest run --workspace --profile ci` · `cargo llvm-cov nextest --workspace --lcov` · `cargo tauri build` (or `cargo xtask release`) · `cargo audit` + `cargo deny check bans licenses sources`. Pointer to `.claude/docs/commands.md`.

### `setup:architecture`
Two paragraphs from arch §Design Philosophy: local-first zero-infrastructure modular monolith; tokio mpsc/broadcast in-process wiring (ingest → buffer → viz/MCP/snapshot); standards-track at the edges (OTLP, MCP, WASM Component Model) and opinionated in the middle (TauRPC, Arrow zero-copy, `AppError`); capability-scoped extensibility with `pulse:default` as the negative-default trust model.

### `setup:imports` (exactly 3 standalone lines)
`@.andromeda/architecture.md` · `@.andromeda/master-route.md` · `@.claude/session-handoff.md`

### `setup:deeper-topics`
5 specialist summaries · 5 core docs · `services/{name}.md` for **14** modules (2 new — see Tier 3) · `session-learnings.md` · the 7 path-scoped rules · `/andromeda-help`.

### Preserved verbatim
`USER:session-learnings` — 19 accumulated Tier-1 entries (2026-05-16 through 2026-08-21). Never regenerated.

---

## Tier 2 — `.claude/rules/`

| File | Action | Path globs | Template |
|---|---|---|---|
| `security.md` | regenerate above `## Session Additions` | (none — unconditional) | `rules-templates/security.md` |
| `testing.md` | regenerate above `## Session Additions` | `crates/**/src/**/*.rs`, `pulse-app/**/*.rs`, `xtask/**/*.rs`, `tests/**/*.rs` | `rules-templates/testing.md` |
| `observability.md` | regenerate above `## Session Additions` | `crates/**/src/**/*.rs`, `pulse-app/**/*.rs`, `xtask/**/*.rs`, `pulse-app/ui/**/*.{ts,tsx}` | `rules-templates/observability.md` |
| `a11y.md` | regenerate above `## Session Additions` | `pulse-app/ui/**/*.{ts,tsx,jsx,js}`, `pulse-app/ui/**/*.css`, `**/*.tsx` | `rules-templates/a11y.md` |
| `verification-harness.md` | regenerate above `## Session Additions` (Phase 4) | `scripts/agent-run.*`, `xtask/**/*.rs`, `tests/integration/**`, `tests/e2e/**` | `rules-templates/verification-harness.md` |
| `frontend.md` | regenerate above `## Session Additions` | `pulse-app/ui/**/*.{ts,tsx,jsx,js}`, `pulse-app/ui/**/*.css` | `rules-templates/frontend.md` |
| `design-tokens.md` | **PRESERVE WHOLE — no template exists** | `pulse-app/ui/**/*.{ts,tsx,jsx,js}`, `pulse-app/ui/**/*.css`, `pulse-app/ui/**/tailwind.config.*` | — (project-added rule; `rules-templates/` has no `design-tokens.md`) |

Not planned (no arch marker): `api.md` · `events.md` · `migrations.md`.
`security.md` carries no `paths:` frontmatter by design (unconditional load) — Health Check 4 treats absent frontmatter as PASS.

**Every `## Session Additions` block is preserved verbatim** — `security.md` alone carries 20 accumulated entries (2026-05-03 → 2026-08-17), including the 2026-06-11 correction that closes the capability-widening static-analysis gap and the 2026-08-21 in-place correction retiring the per-procedure capability claim.

---

## Tier 3 — `.claude/docs/`

**Core (5, regenerate):** `stack.md` (mirrors arch §Stack verbatim — now including the `keyring` 3 explicit-feature-set row and the llama.cpp b9305 D1 row) · `conventions.md` · `commands.md` · `gotchas.md` (documented architectural traps only; runtime learnings stay in `session-learnings.md`) · `workflow.md`.

**Specialist summaries (5, regenerate ~100 lines each, each ending "Full plan: …"):** `security-summary.md` (Minimal tier; loopback + WASM sandbox + code-signing custody; the standing `cargo audit` deferral with its every-3rd-wrap interval) · `design-summary.md` (NASA Deep Space palette; Halo State Pulse **specified-but-unbuilt on desktop-webview**, live signature is the constellation dot hue via `severityToHueFraction`) · `tests-summary.md` (Standard tier; ≥75% line / ≥70% branch / ≥85% function; P1–P7 + the standard per-chunk gate set + boot-smoke gate) · `obs-summary.md` (tracing-only; `metric.{module}.{measure}`; exact-leaf allowlist discipline incl. the three delegated-timing leaves) · `a11y-summary.md` (WCAG 2.1 AA + SC 2.3.3 AAA; P1–**P12** surface set per chunk #99).

**Per-module `services/{module}.md` — 12 present, 14 required. TWO NEW:**
- `services/interpretation.md` — L4 LLM interpretation layer (chunk #82): `LlmInferenceRunner` async trait via manual `Pin<Box<dyn Future + Send + 'a>>`, `ModelTier`/`ModelStatus`/`ModelIdentity`/`ModelLoadEvent`/`InferenceError`, `HardwareProfileDetector`, `ModelStatusBroadcast` on `pulse://stream/model-status`. No `llama.cpp` import at crate level — the concrete impl lives at the binary boundary. Caps P-053 / P-054.
- `services/config-watcher.md` — configuration hot-reload (chunk #96): `notify` 8.x watcher on `<data_dir>/config.toml` + `tokio::sync::watch` fan-out at boot; `partition_changed_keys` → hot_applied / restart_required / silent; prospective-only re-application + opt-in `diagnostics.reevaluate_recent_window`; aggregate-only `pulse://stream/config-events`. Caps P-055 / P-056.

Existing 12 regenerated from the same masters; each pre-existing file backed up before overwrite.
**Untouched (not in the planned set, project-added):** `andromeda-after-mvp-playbook.md`, `session-learnings.md` (wrap territory — create-only-if-missing; it exists).

---

## Agent harness (Phase 4) — agent-driven

`scripts/agent-run.sh` (4982 B, exec bit set) and `scripts/agent-run.ps1` (6684 B) **BOTH PRESENT → PRESERVE (only-if-missing).** Never blind-overwrite: the project legitimately evolves these (the 5-command discipline boot/run/status/cleanup/logs plus the `PYTHONUTF8`/`PYTHONIOENCODING` encoding relay and the `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` / `_DATA_DIR_KEEP` harness-only overrides). If a fresh render would differ, back up to `.claude/backup/` and surface the drift for manual merge — do NOT regenerate over project edits.

`.claude/rules/verification-harness.md` regenerated above its `## Session Additions` (rule-file convention).

---

## Hooks + code-reviewer + .gitignore (Phase 5)

- **Code reviewer:** `agent-templates/code-reviewer-rust.md` → `.claude/agents/code-reviewer.md` (arch §Stack primary language = Rust).
- **Hooks (`.claude/settings.json`):** formatter `rustfmt` · linter `cargo clippy --fix` (+ `prettier`/`eslint --fix` for `pulse-app/ui`) · type-checker `tsc --noEmit` (webview) — resolved from obs-plan §3 + the existing settings. PreToolUse blocks writes to `dist|build|.next|node_modules|coverage|target|vendor`. **`env` block already carries `PYTHONUTF8: "1"` + `PYTHONIOENCODING: "utf-8"` — merge, never replace.** Expected outcome: no change.
- **.gitignore:** base ignores present (`.claude/backup/` :101, `.claude/settings.local.json` :100, `.andromeda/cache/` :8, `scripts/__pycache__/` :10) + rust fragment (`target/` :14) + node fragment (`node_modules/` :39). Idempotent — expected no change.

---

## Code-graph pipeline (Phase 6) — THE TRIGGER FOR THIS RUN

**Detection (two-source rule, integrity-protocol §Detection):**
- *Planning truth* — arch primary language Rust + desktop platform ⇒ `rust`; design-system §Surface `desktop-webview` (React 19 + Vite + Tailwind v4) ⇒ `ts`.
- *Manifest scan* — root `Cargo.toml` present ⇒ `rust`; `git ls-files '*tsconfig.json'` → `pulse-app/ui/tsconfig.json` ⇒ `ts`.
- **Both planes REAL, both indexers on PATH** (`rust-analyzer` ✓ · `scip-typescript` ✓ · python `duckdb`+`protobuf` importable ✓). No forward-note plane.

**Per-file verdicts (checked individually, not as a unit):**

| File | Verdict | Action |
|---|---|---|
| `scripts/code-graph.py` | **DRIFTED** — 196 L present vs 348 L template | back up → replace (per-plane rewrite: `refresh [plane]`, `query … [plane]`, per-plane vouch model) |
| `scripts/code-graph-views.sql` | **DRIFTED** — 46 L vs 47 L | back up → replace (plane-agnostic path rewrite at load time) |
| `scripts/code-graph-cookbook.md` | **DRIFTED** — 55 L vs 77 L | back up → replace **above** the `<!-- Project-specific query learnings accumulate below via wrap curation. -->` marker; tail preserved verbatim (measured: the present tail is the marker line alone — **no accumulated learnings are at risk**) |
| `scripts/scip_pb2.py` | **IDENTICAL** | preserve (vendored, only-if-missing) |
| `scripts/requirements.txt` | **IDENTICAL** | preserve (only-if-missing) |

**Cache layout note (informational, not an action):** `.andromeda/cache/` currently holds the FLAT single-plane layout (`tree.db`, `defs.ndjson`, `occ.ndjson`, `index.scip`, `tree.db.commit`). The new script builds per-plane under `cache/{plane}/`. The stale flat artifacts are gitignored and harmless; the first refresh writes the new layout. **Do NOT build the DB in this phase, and do not run `refresh` manually** — the next wrap's background refresh or a phase query does it on its own.

**Seeded-only-if-missing (all present → no-op):** `.andromeda/state.yaml` (schema 3 lean, `session_count: 31`) · `.claude/session-handoff.md` · `.andromeda/drift-base.md` · `.andromeda/playbook.md` · `.claude/docs/session-learnings.md`.

---

## Internal consistency check

- CLAUDE.md budget: current 155 lines; net delta = −2 pointer rows, +0 elsewhere ⇒ ~153 lines, well under 200. ✓
- `setup:imports` = exactly 3 standalone `@` lines, all resolving. ✓
- Every rule file listed either has a template or is explicitly marked PRESERVE-WHOLE. ✓
- `services/` count (14) equals the module-map library-crate count (14). ✓
- Every Tier-1 warning is universal (path-scoped candidates enumerated in the rejected list). ✓
- Code-graph verdicts derive from per-file byte comparison against extracted template bodies, not from a pipeline-level assumption. ✓
