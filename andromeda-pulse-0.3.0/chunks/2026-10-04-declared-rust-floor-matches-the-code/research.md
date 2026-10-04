# Codebase Research — 2026-10-04-declared-rust-floor-matches-the-code

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 16 · **Probes:** `cargo metadata --offline` (dependency floors) + two workspace clippy runs at a scratch `msrv = "1.95.0"` (`CLIPPY_CONF_DIR`, no repo file touched; logs in the session scratchpad: `clippy-msrv195.log`, `clippy-msrv195-all.log`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — structural read at P4 (60 568 B: `grep -n` of every header and all 22 Session-Additions introducers, then the entries bearing on a direct-binary boot smoke read whole) — 2 additions applied: 2026-06-29 (read the date-suffixed `agent-latest.jsonl*` family, never the bare name) and 2026-08-28 (the run window must outlast any threshold the verdict reads — the smoke's verdicts are per-record counts and a boot record, no tick threshold). The smoke form itself is the Linux direct-binary leg of `2026-10-04-linux-launch-stays-up-on-nvidia-wayland`; its port slot is the operator's (4317/4318 shared with conductor-builder)
- **Platform issues consulted:** none — no runner-only bullet (both Setup CI rows were in progress, neither red) and no CI-reading entry outside the operator leg

## Files inspected
- `Cargo.toml` (1-30, 165-200) — `[workspace] resolver = "2"`; `[workspace.package] rust-version = "1.85"` at :25, inherited by all 16 members (`rust-version.workspace = true` in every member manifest, re-derived: `grep -rn 'rust-version' --include=Cargo.toml .` → 1 declaration + 16 inherits); `keyring` features `apple-native` · `windows-native` · `sync-secret-service` · `crypto-rust` (Linux store = Secret Service over D-Bus)
- `rust-toolchain.toml` (full) — `channel = "1.95.0"`
- `pulse-app/Cargo.toml` (25-60) — `plugins = { path = "../crates/plugins" }` at :51, NON-optional, so wasmtime is in every product build
- `crates/corpus/src/keychain.rs` (120-330, 495-700) — `resolve_lock_dir` :131, `lock_file_name` :167, `default_lock_dir` :179, `open_lock_file` :200 (create, no truncate, 0o600), `fetch_with_lock_dir` :269 with the allow at :268 and its comment :266-267 ("the workspace `rust-version` (1.85) trails the pin"); the CARRY test `corpus_key_survives_a_real_process_boundary` :524, skip arm :532-535, lock cleanup :555-557; the race test's reachability probe `empty_test_entry` :583
- `crates/ui-bridge/src/health.rs` (350-366, 805-818) — `app_info` sets `rust_version: env!("CARGO_PKG_RUST_VERSION")` :360; its INFO emit (:351-356) carries `method_name` + `result_type` only; the test :808 asserts `rust_version` non-empty only
- `crates/ui-bridge/src/contract.rs` (2330-2365) — `app_info_serializes_with_required_fields` builds a literal `AppInfo` with `rust_version: "1.85"` :2344 and asserts KEY PRESENCE only — a sample value, not compared to the live floor
- `pulse-app/src/observability.rs` (490-515, 2440-2470) — :505 is the `ui-bridge.app_info` ALLOWLIST leaf (field names), not an emit; `FieldAllowlist::for_target` :2448 holds three of the clippy sites (:2452 / :2457 / :2462)
- `crates/security/src/scrubber.rs` (245-263) — Luhn `sum: u32` :248, `sum % 10 == 0` :262
- `pulse-app/src/window.rs` (225-236) — nested `if let` inside the aspect-debounce task :229
- `crates/mcp-server/tests/incident_events_subprocess.rs` (:12, :38, :79-80) · `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs` (:12, :35, :112-113) — sibling skip arms; both open with the PRODUCTION service `com.andromeda.pulse`
- `xtask/src/license_check.rs` (1-80) — the precedent for a workspace-field pin: `section_lines` text read of `[workspace.package]`, `#[test]` assertions, no TOML parser dependency
- `xtask/src/pre_push.rs` (:138, :151, :504, :653-661) — `pub(crate) fn rust_channel(toml)` reads `rust-toolchain.toml`'s `[toolchain] channel`
- `.github/workflows/ci.yml` (:43-44 and five sibling steps) — every job installs Rust via `dtolnay/rust-toolchain@29eef336…` "respects rust-toolchain.toml"; no step reads `rust-version` (re-derived: `grep -rn 'rust-version\|rust_version\|RUST_VERSION' xtask/src .github/workflows scripts` → 0 floor consumers)
- `.andromeda/test-plan.md` (:334) — the conditional boot-smoke trigger list

## Graph impact
- **fetch_with_lock_dir** — 2 callers: `fetch_from_os_store` @ `crates/corpus/src/keychain.rs:263` and the test `corpus_key_race_free_fetch_fails_closed_when_lock_dir_unusable` @ `crates/corpus/src/keychain.rs:749` (trace `tree-query-{marker}.json`, rust plane, rebuilt 8728n/42529e). The change removes an attribute and a comment — no signature change, no caller threads.
- **rust_channel** — 5 call sites, all in `xtask/src/pre_push.rs` (`drive` :153; its own test :655-662). Reusable by a floor witness inside xtask (`pub(crate)`).

## Patterns detected
- **The declared floor is set by the DEPENDENCY graph, not the code** (`cargo metadata --format-version 1 --offline`, resolved non-workspace packages 892, 636 declaring `rust_version`): the maximum is `1.95.0`, declared by 28 packages — wasmtime 48.0.5 + its internal crates, cranelift 0.135.5, pulley 48.0.5; next tiers 1.88.0 (12), 1.87 (11). Cargo refuses to build a package whose dependency declares a `rust-version` above the active toolchain, so no toolchain below 1.95.0 can build `pulse-app`.
- **Raising the clippy MSRV switches lints ON** (`CLIPPY_CONF_DIR=… cargo clippy --workspace --all-targets --all-features --keep-going`, msrv 1.95.0): 20 distinct sites in 13 files, two lint classes — `manual_is_multiple_of` ×2 (`crates/security/src/scrubber.rs:262`, `crates/ingest/examples/inject_demo.rs:395`) and `collapsible_if` ×18 (let-chain suggestions): `crates/corpus/src/db.rs:21` · `crates/ingest/src/connection.rs:553` · `crates/ingest/tests/grpc_loopback.rs:150` · `crates/mcp-server/src/tracing_setup.rs:110,115,120` · `crates/triage/src/incident/registry.rs:251` · `pulse-app/src/inference_runtime.rs:130,664,869` · `pulse-app/src/observability.rs:2452,2457,2462` · `pulse-app/src/window.rs:229` · `pulse-app/tests/security_mcp_response_body_redaction.rs:148` · `xtask/src/main.rs:1447,1448` · `xtask/src/smoke.rs:66`. Under `-D warnings` the first run stopped at `security` (exit 101). No `incompatible_msrv` fires at 1.95.0 — the allow at keychain.rs:268 is removable. Linux arms only: `cfg(windows)` / `cfg(target_os = "macos")` arms are NOT linted on this host, and CI lint-test runs on all three OSes.
- **The CARRY's mechanism holds at HEAD**: `fetch_with_lock_dir` opens (creates) the lock file at :275 and locks it at :276 BEFORE `create_or_read` :277 reaches the store; with no store the backend returns `Err`, the test's `let … else` returns at :534, and the lock cleanup sits at :555-557 after the child run — never reached. The service name is pid-scoped (`andromeda-pulse-test-{pid}-corpus-key-persistence`), so every skipped run leaves a NEW file. Under `env -i` (no `XDG_RUNTIME_DIR`, no `TMPDIR`) the lock dir is `/tmp`.
- **A store-free child is reachable on Linux by environment alone**: libdbus finds the session bus through `DBUS_SESSION_BUS_ADDRESS`, else `$XDG_RUNTIME_DIR/bus`; a child with a cleared environment and `XDG_RUNTIME_DIR` set to an empty `TempDir` has no bus, so Secret Service is unreachable AND the lock lands in that `TempDir` (`resolve_lock_dir` uses a set, absolute, canonicalizable `XDG_RUNTIME_DIR`, :134-147).
- **Equality the CARRY witness needs**: after a child run of `corpus_key_survives_a_real_process_boundary` with `env_clear()` + `XDG_RUNTIME_DIR={TempDir}` → (child output contains `[skip]`, count of `andromeda-pulse-corpus-key-*.lock` in that TempDir) = (true, 0). At HEAD the second element is 1 — the RED leg.

## Conventions to follow
- **Workspace-field pin as an xtask `#[test]`**: text-read the manifest, no TOML parser dep (`xtask/src/license_check.rs:19-28`); reuse `pre_push::rust_channel` (`xtask/src/pre_push.rs:504`) for the toolchain side.
- **Re-exec child for env differences**: the parent sets the child's env, never `set_var` in-process (`crates/corpus/src/keychain.rs:540-545`, the existing `--exact … --nocapture` form; the race test :634-642).
- **Lock cleanup before asserting**: the existing test-side `remove_file` on the TEST-scoped lock name (:555-557, :668) — test-only; the product path never unlinks.
- **Mechanical let-chain collapse** keeps the branch order and early returns byte-equivalent in effect (Rust 2024 let-chains already used at `keychain.rs:134-136`).

## New files to create
- `xtask/src/rust_floor.rs` — the floor witness test module, only if P4 takes the witness branch

## Files to modify
- `Cargo.toml` — `[workspace.package] rust-version` raised to the derived floor
- `crates/corpus/src/keychain.rs` — drop the `incompatible_msrv` allow and its stale comment; the skip arm leaves no lock file; the re-exec witness for it
- `crates/security/src/scrubber.rs` — `manual_is_multiple_of` fix
- `crates/ingest/examples/inject_demo.rs` — `manual_is_multiple_of` fix
- `crates/corpus/src/db.rs` — `collapsible_if` fix
- `crates/ingest/src/connection.rs` — `collapsible_if` fix
- `crates/ingest/tests/grpc_loopback.rs` — `collapsible_if` fix
- `crates/mcp-server/src/tracing_setup.rs` — `collapsible_if` fixes
- `crates/triage/src/incident/registry.rs` — `collapsible_if` fix
- `pulse-app/src/inference_runtime.rs` — `collapsible_if` fixes
- `pulse-app/src/observability.rs` — `collapsible_if` fixes in `FieldAllowlist::for_target`
- `pulse-app/src/window.rs` — `collapsible_if` fix
- `pulse-app/tests/security_mcp_response_body_redaction.rs` — `collapsible_if` fix
- `xtask/src/main.rs` — `collapsible_if` fixes; the `mod rust_floor;` line on the witness branch
- `xtask/src/smoke.rs` — `collapsible_if` fix

## Sweep records (no-change dispositions)
- `1\.85` over `--include=*.rs --include=*.toml` in crates/ pulse-app/ xtask/: 3 hits · 2 changed (Cargo.toml:25, keychain.rs:267 comment) · 1 no-change (contract.rs:2344 — a sample value in a key-presence test, not the live floor)
- `CARGO_PKG_RUST_VERSION`: 1 hit · 0 changed (health.rs:360 picks the raise up at compile time)
- `skip\]` skip arms over crates/ pulse-app/: 5 files · 1 changed (keychain.rs) · 4 no-change (the mcp-server and pulse-app cross-process legs open the PRODUCTION service `com.andromeda.pulse`, whose lock file is the product's own never-deleted file — not test residue; `integration_real_llama_cli.rs` and `e2e_p3_mcp_resolve_content.rs` take no corpus-key lock by name)

## Open questions
- Does the chunk add a floor witness, and of what strength (none · `declared ≥ every resolved dependency's rust-version` · `declared == the pinned channel`)? → blocks: plan-decision
- Windows / macOS `cfg` arms may carry further MSRV-gated lint sites this Linux host cannot lint → blocks: implementation-scope (the file list is provisional on CI lint-test's macOS and Windows legs)
