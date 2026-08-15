# Report — 2026-08-14-workspace-key-alignment

**Chunk:** Workspace-key alignment — one workspace-key derivation reached by both the app and the MCP sidecar, observed before it is fixed and normalization proven for canonicalized-vs-raw entry forms; carries the Conductor arm-zero classification leg
**Date:** 2026-08-15T00:20Z
**Commits:** none since last wrap (this is the first commit of the session; last wrap was `b08e10a`)

## Changes (structured — detectors read this)

- **Files:** 16 changed per `git status` — 9 source (`crates/workspace-detector/src/contract.rs` +175/−1 · `crates/workspace-detector/src/lib.rs` · `crates/mcp-server/Cargo.toml` · `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` · `pulse-app/src/digest_runtime.rs` · `pulse-app/src/main.rs` · `pulse-app/src/observability.rs` · `pulse-app/tests/integration_constellation_severity_workspace_key.rs` · `Cargo.lock`), 295 insertions / 21 deletions; plus route/matrix/state bookkeeping and the new chunk folder (scope · research · plan · report · 3 evidence artifacts).

- **Symbols / APIs:** NEW public surface, all in `workspace_detector::contract` (re-exported from `lib.rs`):
  `workspace_key(Option<&WorkspaceContext>, &Path) -> String` · `publish_workspace_key(&Path, &str) -> Result<(), Error>` · `read_published_workspace_key(&Path) -> Option<String>` · `workspace_key_path(&Path) -> PathBuf` · `WORKSPACE_KEY_BASENAME: &str` · `MAX_WORKSPACE_KEY_BYTES: usize` (4096).
  CHANGED: `pulse_app::digest_runtime::resolve_workspace_for_incidents` — signature and name unchanged, body now delegates to `workspace_key`.
  NEW private: `pulse_app::main::publish_workspace_key_for_sidecar`.
  **No new IPC method, no new TauRPC procedure, no new MCP tool method** (the surface stays at the registered 8), **no new port/socket, no new env var.**

- **Crates / modules:** none added or removed. `workspace-detector` gains the key-derivation surface on its existing contract module; `mcp-server` gains one dependency edge.

- **Dependencies:** one workspace-internal path dependency added — `mcp-server → workspace-detector` (normal, not dev-only). No third-party dependency added or bumped; `Cargo.lock` +1 line (the internal edge only). The new edge introduces no transitive third-party crates (`workspace-detector` depends on `thiserror`/`serde`/`strict-path`/`tracing`, all already in the graph). Dependency graph remains an acyclic DAG rooted at `pulse-app`.

- **Schema / config:** no DB migration, no config key. ONE new filesystem resource: `{data_dir}/run/workspace-key` — a plaintext file holding the resolved incident workspace key, written by the app at boot (atomic `.tmp`+rename, canonicalize-and-confine guard), read by the MCP sidecar.

- **Spec-master edits:** none by /implement (implement is read-only on specs). Two are due in THIS wrap's P2 — see *Spec claims disproved by measurement* and the Expected amendment below.

- **Counts / qualifiers moved:** workspace test count 1734 → **1745** (+11: 8 in `workspace-detector`, 2 parity tests in the P-079 binary, 1 arising from the feature-gated build); `workspace-detector` crate suite 18 → 26.

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** three designs were considered and rejected at plan time and none shipped: a shared `detect(current_dir())` on both ends (would derive Conductor's repo root under the target topology — a different wrong key), a new `ANDROMEDA_PULSE_WORKSPACE_KEY` env var (arch bans an incidental env var; would also need a change inside Conductor's tree), and deriving the key from the corpus's existing `workspace` values (unsound when one data dir holds multiple workspaces — `e2e_p043_two_workspace_switch.rs`).

- **Spec claims disproved by measurement:**
  1. **The corpus encryption key is NOT sourced from an OS keychain and does NOT survive a process boundary.** Claimed by `architecture.md` §Occupied Resources ("Corpus SQLite … At-rest posture: cell-level AES-256-GCM encryption with key sourced from OS keychain (macOS Keychain / Linux Secret Service / Windows DPAPI) via `keyring` crate") and by `security-plan.md` §Secret Management (OS keychain primary, `BackendKind::PassphraseFallback` warn-at-boot posture). **Measured:** `Cargo.lock` resolves `keyring 3.6.3` with dependencies `[log, zeroize]` ONLY — no platform credential-store backend is linked, so keyring's non-persisting mock store is in use. `cmdkey /list` lists zero andromeda entries. First app process: 0 decryption failures. Second app process against the same corpus: **13** `decryption_failed` warnings (`target: digest.corpus.retrieve`, `error_category: decryption_failed`) against rows its own predecessor wrote — emitted by the app under `service.name: com.andromeda.pulse`, on a code path unrelated to this chunk. **Data-loss implication (part of the measured truth):** every corpus payload written under an ephemeral key is unreadable by any later process — all historical encrypted incident/digest content is orphaned until a migration exists. Pre-existing; this chunk did not cause it, it unmasked it (the workspace-key mismatch previously meant the filter never returned a row to decrypt). Evidence: `evidence/after-query-incident-list.json` §blocked_by.
  2. **Expected amendment (planned):** `{data_dir}/run/workspace-key` needs registering in `architecture.md` §Occupied Resources → Filesystem locations, following the 2026-06-29 `window-geometry.json` precedent.

- **Coverage of new surfaces:**
  - `workspace_detector::contract::{workspace_key, publish_workspace_key, read_published_workspace_key}` → validation **✓** (read path: bounded ≤4096 bytes, UTF-8 checked, trailing-newline trimmed, empty and control-character values rejected; value used ONLY as an opaque filter string, never opened or joined as a path; write path canonicalize-and-confine guard) · instrumentation **✓** (`app.boot.workspace_key` INFO on publish / WARN on failure) · PII **redacted✓** (basename only — `workspace_root_basename` + `key_bytes`; full path never emitted; verified 0 occurrences in the live boot log) · tests **unit ✓ 8 + integration ✓ 2** · a11y **n/a** (no UI) · tokens **n/a** (no UI)
  - `{data_dir}/run/workspace-key` (new filesystem resource, cross-process read boundary) → validation **✓** (as above) · instrumentation **✓** · PII **redacted✓** · tests **✓** (publish/read round-trip, absent, oversized, empty, control-char, trailing-newline) · a11y **n/a** · tokens **n/a**
  - `mcp-server` sidecar `init_incident_context` (changed cross-process read) → validation **✓** (falls back to `data_dir` on any invalid/absent key) · instrumentation **n/a** (no new emission; existing `mcp.tools.call.*` chain unchanged) · PII **✓** (no response bodies logged; counts only) · tests **✓** (parity regression) · a11y **n/a** · tokens **n/a**

## Deviations from intent

1. **Step 7's parity test went into the existing P-079 file** (`pulse-app/tests/integration_constellation_severity_workspace_key.rs`) rather than a new file. *Justification:* the plan named a placement RULE but its New-files list contained no test file; extending the already-in-scope file was the only in-boundary resolution. The 5 pre-existing P-079 tests are untouched and still pass.
2. **`cargo clean` (62,102 files / 239.4 GiB) was run mid-gate — not in the plan.** *Justification:* D: reached 100% (133 MB free) and `rust-lld` crashed with `LLVM ERROR: IO failure on output stream: no space on device`; the crash text reads like a memory/parallelism fault and was first treated as one (one wasted retry at `--jobs 2`). `cargo clean` is operator-pre-authorized; it restored 217 GB and cost a ~9 min cold rebuild. Kept visible here for the next diagnosis rather than turned into a rule.
3. **Boot smoke driven directly, not via `bash scripts/agent-run.sh run`.** *Justification:* the operator's standing warm-boot directive prescribes the direct sequence; the harness `boot` verb is a cold `cargo run --release` and would have forced a full release rebuild of a binary already built.
4. **Acceptance criterion "the SAME query returns `total` = the corpus incident count" is NOT met.** *Justification:* blocked by the surfaced keyring defect above, which makes ANY cross-process encrypted-payload read fail for EVERY workspace key. What was proven instead is the failure-mode change: `load_active_incidents` decrypts only rows its `WHERE workspace = ?1` clause returned, so reaching decryption proves the filter now matches. Pre-fix, the same query against the same data dir with 2 active incidents returned `total: 0` with no error.
5. **Acceptance criterion "the arm-zero failure class is NAMED" is UNMET-PENDING-OPERATOR.** *Justification:* operator-gated by design (external instrument, headful half). Recorded in `evidence/arm-zero-classification.md` with the ready-to-run recipe — never silently skipped. Note: the `buffer.tick` counter classification is log-side and needs no decryption, so it can run at any time; its full preflight-end-to-end value lands with the keyring fix.

## Decisions & corrections

- **Operator, at plan review (P5):** the review card showed neither evidence leg. Resolution 1 applied — both legs were plan steps, but the acceptance list carried only negative hygiene criteria; three `(evidence)` criteria were added with producers in FIRING form, `evidence/` named three concrete artifacts, and step 6b made the proof the same query returning `0 → N`.
- **Operator, at plan P4:** chose "app publishes key to data dir" over shared-`detect(cwd)` and over a new env var, after research showed Conductor propagates `ANDROMEDA_PULSE_DATA_DIR` but not cwd.
- **Correction discovered at P5 val-1:** scope's "both processes call it" was falsified; only the app runs the derivation, the sidecar reads its published output.
- **Correction discovered at P3 research:** the fallback branch already agreed cross-process (both `resolve_data_dir()` copies are byte-identical); only the detected-root branch diverged. And `detect` canonicalizes exactly once — re-canonicalizing downstream is the documented divergence trap.
- **Recurring tax observed:** `cargo xtask capability-drift` went red purely because the default-features workspace nextest rewrote `pulse-app/ui/src/bindings/index.ts` to the no-mcp shape; the documented regen restored it. This has recurred across many sessions.
- **Operator, at wrap:** carry forward a testing.md addition about the wrap's own P7 light gate being structurally a default-features cargo op that runs AFTER P2's regen (origin: the `b08e10a` wrap's near-miss, where drift appeared only in the staged copy).

## Outcome

**Gates — all green** (the plan's `## Test Commands`, run by /implement):
`cargo fmt --check` ✓ · `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓ · `cargo nextest run -p workspace-detector` ✓ 26/26 · `cargo nextest run --workspace --profile ci` ✓ **1745 passed, 1 skip** · `cargo build -p mcp-server --features mcp-server --bin andromeda-pulse-mcp` ✓ · `cargo test --test integration_constellation_severity_workspace_key -p pulse-app --features mcp-server` ✓ 7/7 · `cargo xtask capability-drift` ✓ clean (after the documented bindings regen) · `cargo xtask capability-widening-check` ✓ clean, 0 violations · `bash scripts/agent-run.sh status` ✓ exit 0.
No gate deferrals — there was Rust delta, so the full workspace suite ran. Zero code-fix iterations.

**Smoke — fired (boot-path changed) and passed:** real boot of the current binary against a fresh data dir; OTLP up in 1s; 2187 spans seeded; 11 obs families present (`app.boot.webview.init` 1 · `app.boot.workspace_key` 1 · `metric.webgpu.frame_duration_ms` 17249 · `viz.query.traces` 61 · `duckdb.append` 168 · six `.tick` families 4 each); **0 ERROR, 0 panics**; key file published at 35 bytes; `:4317` closed on clean shutdown. The new `app.boot.workspace_key` line emitted `workspace_root_basename="andromeda-pulse"` **unredacted** (proving the allowlist leaf entry resolves) with **0** full-path occurrences in the log.

**Acceptance criteria:** 13 of 15 met. Two unmet, both recorded above with cause — one blocked by the surfaced keyring defect, one operator-gated pending.

**Verification matrix:** this chunk claims **no** capabilities (declined at plan P5; `P-075` was partially advanced and stays pooled at `chunk:null` with a `notes` line recording the declined concretization).
