# Report — 2026-06-29-predictable-close-self-verify

**Chunk:** Predictable close + honest tray (close→tray with a clear running indication, dashboard closable) + a minimal agent-headful self-verify harness (P-063 + P-078)
**Date:** 2026-06-29
**Commits:** (uncommitted at report time — wrap commits as `feat(2026-06-29-predictable-close-self-verify)`)

## Changes (structured — detectors read this)
- **Files:** `pulse-app/src/window.rs` · `pulse-app/src/main.rs` · `xtask/src/main.rs` (modified) · `xtask/src/self_verify.rs` · `pulse-app/tests/unit_close_signpost.rs` (new) · route/ledger from /phase: `.andromeda/master-route.md` · `andromeda-pulse-0.3.0/{requirements.md, working-route.md, verification-matrix.json}`
- **Symbols / APIs:**
  - NEW `pub fn window::should_show_close_signpost(notifications_enabled: bool, already_shown: bool) -> bool` (pure decision seam; unit-tested)
  - NEW private `window::maybe_show_close_signpost` (emits the first-close OS notification via `NotificationExt`, Rust-side)
  - CHANGED signature `window::on_window_event` (+1 param `signpost_shown: &AtomicBool`) — internal, 1 caller (`main.rs:1021`)
  - NEW xtask subcommand `cargo xtask self-verify` (+ `self_verify::run_self_verify` + pure helpers `parse_shell_health` / `headless_reason` / `pulse_binary_candidates` / `read_log_lines`)
  - NEW tracing target `tray.signpost.shown` (message-only event, NO fields — needs no AllowList entry)
  - **NO** new IPC methods / TauRPC procedures / endpoints / events / ports / sockets / env vars / Tauri capabilities
- **Crates / modules:** NEW module `xtask/src/self_verify.rs`. **No** new workspace crates.
- **Dependencies:** **NONE added.** `tauri-plugin-notification` was already a `pulse-app` dep (init at `main.rs:1020`); no new workspace deps; xtask reuses its existing `tokio`/`anyhow`/`serde_json`/`tempfile`.
- **Schema / config:** none — no new env var, no new IPC arg struct, no new DuckDB/corpus table, no new capability JSON (`pulse-app/capabilities/*.json` untouched; `capability-widening-check` clean).
- **Coverage of new surfaces:**
  - `close→tray first-close OS-notification signpost` → validation n/a · instrumentation span✓ (`tray.signpost.shown`, message-only per obs aggregate-only) · PII redacted✓ (notification body is a static string passed only to `.body(...)`, NEVER to a tracing macro — no log path carries it, by construction) · tests unit✓ (4 truth-table) + e2e✓ (self-verify clean-quit) · a11y n/a-web (OS-native notification → OS a11y APIs; close-to-tray preserves the existing P5 focus-restore contract, unchanged) · tokens n/a (OS-native, no web UI element)
  - `cargo xtask self-verify` harness (dev/test tool) → validation n/a · instrumentation n/a (test harness, not a product hot-path) · PII n/a · tests unit✓ (8 helper tests) + e2e✓ (the harness run itself) · a11y composes the existing `npm run test:a11y` chain · tokens n/a

## Deviations from intent
- **Premise correction (research-corrects-intent — the chunk's defining frame, recorded at /phase).** Intent F3's "the dashboard cannot be closed / X does not quit" is the SYMPTOM of an unsignposted hide-to-tray: `window.rs:100-124` already does `prevent_close()`+`hide()` (chunk #24, the arch §Tray-icon-policy default). The fix is the missing signpost, not a new close mechanism; the OUTCOME (predictable, signposted close) is unchanged. Recorded on the P-063 matrix entry `notes` + `scope.md`; the working-route line + `intent.md` F3 stay as historical source. Justified — proven by reading the code + the code-graph (`handle_close_to_tray` 1 caller).
- **Obs span message-only, not an allowlisted field** (plan step 3 partial). Emitted a message-only `tray.signpost.shown` info rather than a `signpost_shown: bool` field. *Why:* an allowlisted field needs an edit to `pulse-app/src/observability.rs` (the AllowList), which is OUTSIDE the plan's Files-to-modify; a message-only event needs no AllowList entry (the `message` field is always allowed) and carries no PII. The decision is unit-verified instead.
- **PII-canary test → by-construction** (plan step 9 partial). Omitted as a test because the notification body never reaches a tracing macro (only `.body(...)`), so a canary test would pass vacuously (fix-loop anti-pattern). Documented in the test-file comment.
- **In-scope mid-loop fix — self-verify log filename.** First self-verify run failed (no spans): I read bare `agent-latest.jsonl`, but the obs sink is `tracing_appender::rolling::daily` → files are `agent-latest.jsonl.<date>`. Fixed `read_log_lines` to glob `agent-latest.jsonl*` (mirrors the obs `collect_log_files` pattern). The smoke caught a real bug the synthetic-name unit tests hid — exactly its purpose.
- **No capability-JSON delta** (plan anticipated "maybe"). None needed — the notification is Rust-side (`NotificationExt`, host code, ungated by the webview ACL).

## Decisions & corrections
- **P4 AskUserQuestion (user-selected, both recommended):** (1) close behavior = hide-to-tray + first-close OS-notification signpost (no arch amendment); (2) self-verify mechanism = `xtask self-verify` orchestrator (no new deps; full tauri-driver DOM/drag-delta stays in P-076).
- **Directed authoring:** P-078 (new cap) authored in `requirements.md` + `verification-matrix.json` at /phase per the working-route directive (folded into the P-063 chunk; close-fix is its enabling prerequisite).
- **Latent sibling bug (surfaced, NOT fixed — out of scope):** `xtask/src/smoke.rs::run_smoke` (l.95) has the identical bare-`agent-latest.jsonl` filename bug; rarely exercised (release-workflow bundle smoke only); flagged as a future-cleanup handoff note, not touched.
- **bindings.ts** regenerated to canonical (mcp present, matches HEAD) after the default-features test runs clobbered it — operational hygiene, not a net change.

## Outcome
- **Acceptance met:** P-063 (first-close signpost gated on `notifications_enabled`; hide-to-tray preserved; tray→Quit terminates) + P-078 (`cargo xtask self-verify` boots → asserts shell health → a11y → clean quit, deterministic exit) — both `status:implemented` in the matrix with test refs.
- **Gates green (commands run):** `cargo build -p pulse-app -p xtask` · `cargo test --test unit_close_signpost -p pulse-app` (4/4) · `cargo nextest run -p xtask` (43/43; 8 new self_verify) · `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` · `cargo nextest run --workspace --profile ci` (**1713/1713 + 1 skip**) · `cargo xtask capability-drift` (clean) · `cargo xtask capability-widening-check` (clean) · `cargo build -p pulse-app` (embedded ACL validated).
- **Smoke (boot-path changed):** `cargo xtask self-verify` **PASS** — boot real binary → receivers ready (:4317/:4318) → shell health OK (207 log lines; boot trio + window-shown + heartbeat; zero panics) → clean quit, zero orphan → a11y/contrast PASS (7 routes, 0 errors).
