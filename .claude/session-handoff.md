# Session Handoff

**Last Updated:** 2026-05-03T11:05:00Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** 6b7821b feat(foundation): xtask agent-run harness + health TauRPC + JSON log subscriber (chunk #4)

## Current State

- **Last completed chunk:** route#4 "xtask agent-run harness — 5-command discipline (boot/run/status/cleanup/logs) + health TauRPC command (status/subsystems/pid/uptime_ms) + PID file + JSON log format"
- **Next chunk:** route#5 "Base CI workflow — matrix Linux/macOS/Windows + harden-runner SHA-pinned + cargo-nextest + cargo-llvm-cov coverage gate (≥75% line / ≥70% branch / ≥85% function)"
- **In-progress phase:** no active phase (phase-3 implemented + committed in this wrap; phase-4 not yet planned)
- **Phase artifacts present:** `.andromeda/phases/{phase-1, phase-2, phase-3}/{combined.md, research.md, plan.md}` + audit trails for each phase under `.andromeda/runs/`

## Andromeda State Detection (states A-L)

⚠️ J — Specialist plan freshness mild mismatch: `route.md` was edited at phase-2 wrap (Decisions Log entry "Chunk #3 scope split"); CLAUDE.md ingests route §1-§2 structure only — Decisions Log appendage is benign mtime drift. Remediation: no action needed unless route §2 chunk list changes.

All other states (A, B, C, D, E, F, G, H, I, K, L) — no warnings. State K cleared at 2026-05-03T11:05:00Z reconcile after `cargo install cargo-modules` + `cargo install cargo-public-api --locked` + `rustup toolchain install nightly`.

## Drift Detection (6 dimensions)

⚠️ D5 — route.md newer than CLAUDE.md (mild, carried over): Decisions Log entry from phase-2 wrap (chunk #3 scope split). CLAUDE.md ingests §1-§2 structural sections only. Remediation: no action unless route §2 chunk list changes.

D1, D2-D4, D6 — no drift detected. D1 cleared at reconcile (2026-05-03T11:05:00Z): both `dep-tree.md` (cargo tree --workspace --depth 2 output) and `api-surface.md` (per-crate cargo public-api iteration) now reflect actual code state.

## Key Decisions This Session

- **TauRPC integrates at IPC channel level, not per-procedure capability gating** — single `taurpc::create_ipc_handler(...)` invoke handler dispatches all router methods. Capability JSON `permissions` array contains `core:default` + plugin-level permissions (e.g., `updater:default`), NOT per-procedure entries. This is a real tension with arch §Cross-cutting Patterns "Webview IPC capability policy" which assumes per-procedure capability JSON pairing. The `xtask capability-drift` check (route#22) must introspect the TauRPC router as the unit of granularity, not per-procedure. Documented as Tier 2 entry in `.claude/rules/security.md`.
- **Tauri-dependent crates fail xtask runtime on Windows GNU** — any binary that transitively depends on `tauri` links against Windows DLLs (WebView2 / DirectX) at link time and fails at startup with `STATUS_ENTRYPOINT_NOT_FOUND` even without invoking Tauri runtime code. Solution: feature-gate tauri-dependent code in shared crates (default = ["taurpc-runtime"]; downstream consumers like xtask use default-features = false to import only data types). The `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` pattern on data types is part of the split. Documented as Tier 3 entry.
- **Cargo alias `xtask = "run --quiet --package xtask --"` in `.cargo/config.toml`** — required for `cargo xtask <subcommand>` to work. Without it, cargo errors with "no such command: xtask". `--quiet` suppresses Cargo build noise so subcommand stdout is clean for `jq` / shell pipelines. Documented as Tier 3 entry.
- **specta = "=2.0.0-rc.22" exact pin** — taurpc 0.7.1 hard-pins specta to ^2.0.0-rc.22; rc versions don't satisfy ^ semver naturally so workspace dep needs explicit `=` prefix. Plus specta requires `features = ["chrono"]` for `DateTime<Utc>` field support.
- **`tracing::info!` macro can't parse dotted field names** — `tracing::info!(service.name = ...)` fails to compile. Workaround: use underscored field names (`service_name`) or build a custom subscriber Layer that maps to dotted JSON keys at serialization time. Defer custom Layer to route#7 tracing self-observation harness chunk; current implementation uses underscored names (mild deviation from obs-plan §6 dotted-key requirement). [DEFERRED learning per Filter 5 cap.]

## Files Modified

(13 entries)

- `.andromeda/route.md` (no change this session)
- `.andromeda/context/dependency-tree.md` (METADATA Last reconciled timestamp refreshed; still stale-tooling-failed)
- `.andromeda/context/api-surface.md` (same as above)
- `.andromeda/state.yaml` (session_count → 3; last_wrap → 2026-05-03T10:56:57Z; last_completed_chunk → route_index 4; drift_warnings refreshed)
- `.cargo/config.toml` (NEW — cargo xtask alias)
- `Cargo.lock` (regenerated; +taurpc + specta + chrono + clap + strict-path + transitive deps)
- `Cargo.toml` (workspace deps: + taurpc + specta=rc.22 + chrono + clap + strict-path)
- `crates/ui-bridge/Cargo.toml` (taurpc-runtime feature gate; serde + chrono unconditional)
- `crates/ui-bridge/src/contract.rs` (canonical AppError enum: 6 variants with sanitization)
- `crates/ui-bridge/src/health.rs` (NEW — HealthEnvelope + procedure trait + resolver gated by feature)
- `crates/ui-bridge/src/lib.rs` (re-exports for HealthEnvelope/AppError)
- `deny.toml` (skip-list +4: ctor + windows-core + windows-result + windows-strings)
- `pulse-app/Cargo.toml` (subscriber stack + taurpc + ui-bridge + strict-path)
- `pulse-app/capabilities/default.json` (`core:default` + `updater:default`; description refreshed)
- `pulse-app/src/main.rs` (data dir resolve + tracing JSON subscriber + panic hook + PID write + TauRPC handler chain)
- `xtask/Cargo.toml` (anyhow + clap + tokio + ui-bridge no-default)
- `xtask/src/main.rs` (clap dispatcher + harness:status body)
- `.claude/rules/security.md` (Tier 2 Session Additions: TauRPC capability granularity)
- `.claude/docs/session-learnings.md` (Tier 3 +2 entries: Tauri Windows GNU runtime split + cargo xtask alias)
- `.claude/session-handoff.md` (this file)

## New files

- `.cargo/config.toml` (cargo xtask alias)
- `crates/ui-bridge/src/health.rs` (HealthEnvelope module)
- `.andromeda/phases/phase-3/{combined,research,plan}.md` (planning artifacts)

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 1 addition
  - `.claude/rules/security.md`: TauRPC capability granularity differs from arch's per-procedure assumption
- **Tier 3 (`.claude/docs/session-learnings.md`):** 2 additions
  - "Tauri-dependent crates fail xtask runtime on Windows GNU; feature-gate the runtime to allow type-only consumers"
  - "Cargo alias for `cargo xtask <subcommand>` shortcut"
- **Filters applied:** 0 duplicates · 0 task-specific · 0 conflicts · 2 deferred (Filter 5 cap)

## Last Failed Command

`cargo run -p xtask -- harness:status` (with tauri::test::mock_builder enabled)

**Error:** `error: process didn't exit successfully: \`target\debug\xtask.exe 'harness:status'\` (exit code: 0xc0000139, STATUS_ENTRYPOINT_NOT_FOUND)`

**Root cause:** xtask binary depended on `ui-bridge` which had unconditional tauri dep, pulling Windows DLL deps (WebView2 / DirectX) into the xtask link. Windows GNU host without WebView2 fails to load these DLLs at startup. Even though xtask code didn't invoke any tauri runtime, the binary linked against tauri at compile time → DLL load fails before main().

**Resolution applied this session:** feature-gate tauri-dependent code in `ui-bridge` (`default = ["taurpc-runtime"]`); xtask uses `default-features = false` to import types only without tauri. xtask `harness:status` calls `ui_bridge::health::current_health()` directly — same envelope shape, no IPC roundtrip. Full IPC roundtrip via `tauri::test::get_ipc_response` deferred to integration-test chunk when actual subsystems exist (chunks #15-#18).

Do NOT retry the original `mock_builder` approach on Windows GNU host without first switching to MSVC toolchain (with WebView2 installed via `webview2-runtime` package). See `.claude/docs/session-learnings.md` "Tauri-dependent crates fail xtask runtime on Windows GNU".

## Tests Status

12 plan AC gates pass (1 fix-loop iteration — Windows DLL fix via feature-gate refactor):

- ✓ cargo check --workspace
- ✓ cargo fmt --check (auto-fix applied)
- ✓ cargo clippy --workspace --all-targets --all-features -- -D warnings
- ✓ cargo nextest run --workspace --profile ci --no-tests=pass (0 tests, Epoch 1)
- ✓ cargo build -p pulse-app
- ✓ cargo build -p xtask
- ✓ cargo deny check bans (after skip-list +4: ctor / windows-core / windows-result / windows-strings)
- ✓ bash scripts/agent-run.sh status returns canonical envelope JSON
- ✓ jq shape gate: `.status == "ok" and .subsystems.{4 keys} and (.pid | type == "number") and (.uptime_ms | type == "number")` succeeds
- ✓ bash scripts/agent-run.sh cleanup × 2 (idempotency)
- ✓ workspace metadata: 10 crates unchanged
- ✓ no OTel SDK linked into pulse-app
- ✓ secret-leak grep: 3 matches are regex-as-docs (self-referential); no actual secret values leaked

## Deferred Learnings

2 learnings analyzed but not applied due to Filter 5 max-3 cap (manual review with `/wrap-session --review` if any should be applied):

- **`tracing::info!` macro doesn't parse dotted field names** — `tracing::info!(service.name = ...)` fails to compile (macro can't parse `.` in field identifiers). Workaround: use underscored alternatives (`service_name`) or build a custom subscriber Layer mapping to dotted JSON keys at serialization time. Affects obs-plan §6 schema dotted-key requirements (semantic conventions). Candidate Tier 3 destination: `.claude/docs/session-learnings.md` OR Tier 2: `.claude/rules/observability.md ## Session Additions`.
- **specta + taurpc version coupling** — taurpc 0.7.1 hard-pins specta to ^2.0.0-rc.22; rc versions don't satisfy ^ semver naturally, so workspace dep needs explicit `=` prefix (`specta = "=2.0.0-rc.22"`). Plus specta requires `features = ["chrono"]` for `DateTime<Utc>` field support. When bumping taurpc, audit specta version + features; rc-version pinning is fragile across minor-version bumps. Candidate Tier 3 destination: `.claude/docs/session-learnings.md`.

## Next Recommended Action

`/andromeda-phase` to plan chunk #5 "Base CI workflow — matrix Linux/macOS/Windows + harden-runner SHA-pinned + cargo-nextest + cargo-llvm-cov coverage gate (≥75% line / ≥70% branch / ≥85% function)". This is the next natural chunk in the Foundation epoch; depends on no chunk-#4 outputs except the existing scripts/agent-run.{sh,ps1} harness which CI will invoke.

Optional pre-#5 (clears state K + drift D1):
- `cargo install cargo-modules` (~2-3 min compile)
- `cargo install cargo-public-api --locked` (~2-3 min compile)

After both install, next `/wrap-session` Phase 5 reconcile produces real living-artifact content.

## Session Goals (carry-over)

(none — phase-3 implementation complete; chunk #5 is the next natural starting point)

## Session End Status

clean
