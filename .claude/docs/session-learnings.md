# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-05-03 — Tauri-dependent crates fail xtask runtime on Windows GNU; feature-gate the runtime to allow type-only consumers

Any binary that transitively depends on the `tauri` crate links against WebView2 / DirectX / etc. Windows DLLs at link time. On Windows GNU rustup-toolchain hosts without WebView2 installed (or any DLL load-path issue), the resulting binary fails at startup with `STATUS_ENTRYPOINT_NOT_FOUND` (exit code `0xc0000139`) — **even if the binary never actually invokes any Tauri runtime code**. This blocks shared-crate designs where the data types live alongside the procedure implementation: an `xtask` binary that imports `ui-bridge` for `HealthEnvelope` (a pure data type) inherits the tauri DLL deps and crashes.

**Solution**: split runtime vs. types via Cargo features.

```toml
# crates/ui-bridge/Cargo.toml
[features]
default = ["taurpc-runtime"]
taurpc-runtime = ["dep:taurpc", "dep:tauri", "dep:specta", "dep:tokio"]

[dependencies]
thiserror.workspace = true
serde.workspace = true
chrono.workspace = true
taurpc = { workspace = true, optional = true }
tauri = { workspace = true, optional = true }
specta = { workspace = true, optional = true }
tokio = { workspace = true, optional = true }
```

Source code uses `#[cfg(feature = "taurpc-runtime")]` to gate the `#[taurpc::procedures]` trait and resolver impl, leaving the data types (`HealthEnvelope`, `AppError`, `SubsystemStatus`, etc.) compiled unconditionally.

```toml
# xtask/Cargo.toml — non-Tauri binary consumes types only
[dependencies]
ui-bridge = { path = "../crates/ui-bridge", default-features = false }
```

The pulse-app binary keeps default features (taurpc-runtime enabled) so the procedure trait + resolver are available for `taurpc::create_ipc_handler(...)` registration in the Tauri Builder chain.

The `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` pattern lets data types acquire the `specta::Type` derive only when the runtime feature is active — required for taurpc procedure parameter/return types but useless for type-only consumers.

This is also the cleanest architectural split — types belong in the contract module, runtime belongs in the runtime module.

See: `.andromeda/phases/phase-3/plan.md` Implementation note 1; `crates/ui-bridge/Cargo.toml`; `crates/ui-bridge/src/health.rs` `#[cfg(feature = "taurpc-runtime")] mod runtime`.

---

## 2026-05-03 — Cargo alias for `cargo xtask <subcommand>` shortcut

Without an alias, `cargo xtask harness:status` fails with "no such command: xtask" because cargo doesn't know `xtask` is a workspace member shortcut. The fix is `.cargo/config.toml` (project-root):

```toml
[alias]
xtask = "run --quiet --package xtask --"
```

After this, `cargo xtask <subcommand>` works equivalently to `cargo run --package xtask -- <subcommand>` from any directory inside the project. The `--quiet` flag suppresses Cargo's "Compiling … / Finished …" output so the subcommand's stdout (e.g. JSON envelope from `harness:status`) is the only thing on the pipe — important for `jq` / shell-script chains.

The agent-run scripts (`scripts/agent-run.{sh,ps1}`) invoke `cargo xtask harness:status` and depend on this alias being present.

See: `.cargo/config.toml`; `xtask/src/main.rs` clap dispatcher; `scripts/agent-run.sh` `status` case body.

---

## 2026-05-03 — Andromeda chunk scope-split for paid-prereq operator steps

When a route chunk's full scope requires paid external accounts (e.g., chunk #3 code-signing wants Azure Key Vault Premium ~$5/month + Windows EV cert from DigiCert/GlobalSign $300-500/year + Apple Developer ID $99/year + 1-2 weeks of legal-entity verification) but the project is in dogfooding/iteration phase, **split chunk scope** rather than skip the chunk or pay prematurely.

The pattern: `plan.md` divides Implementation Steps + Acceptance Criteria into **ACTIVE** (free + local + reversible work that `/andromeda-implement` runs now — e.g., generate Minisign keypair locally, add deps, edit `tauri.conf.json`, write rotation runbook) and **DEFERRED** (paid + external + bureaucracy items that become pre-v0.1.0 release blockers — Azure Key Vault provisioning, EV cert enrollment, Apple Developer ID, GitHub Environment with secrets). DEFERRED items are tracked in `plan.md §Acceptance Criteria → Deferred` + the runbook describing operator procedure + a `route.md` Decisions Log entry recording the scope-split rationale.

Pipeline integrity is preserved: `state.yaml.last_completed_chunk.route_index` advances when ACTIVE scope lands; DEFERRED items are explicit pre-release blockers tracked across artifacts (not lost). This is better than (a) skipping the chunk entirely (breaks route progression heuristics + state.yaml continuity) or (b) running paid prereqs before the project demonstrates value (premature commitment).

Apply when: chunk has clear paid-vs-free dependency split AND project is in pre-public-release dogfooding phase AND user explicitly states preference to defer paid commitments. Don't apply when: chunk's value depends entirely on paid prereqs (rare for solo OSS projects).

See: `.andromeda/route.md` Decisions Log 2026-05-03 entry "Chunk #3 scope split"; `.andromeda/phases/phase-2/plan.md` §Acceptance Criteria → Active vs Deferred; `docs/runbooks/updater-key-rotation.md` as DEFERRED procedure document.

---

## 2026-05-03 — Standalone minisign 0.12 as Tauri-cli fallback when Windows GNU mingw blocks compile

The rustup `x86_64-pc-windows-gnu` toolchain bundles a minimal mingw-w64 set in `<sysroot>\lib\rustlib\x86_64-pc-windows-gnu\lib\self-contained\` that does NOT include `libktmw32.a` (Windows Kernel Transaction Manager API import library). Modern Tauri 2.x ecosystem crates link transitively against `ktmw32` so `cargo install tauri-cli --version "^2.0" --locked` fails with `ld: cannot find -lktmw32`.

**Refreshing rust-mingw component does NOT fix it** — `rustup component remove rust-mingw && rustup component add rust-mingw` re-downloads the same minimal libset; `libktmw32.a` is not bundled by design.

Two viable paths:
- **MSVC switch (permanent fix)**: install Visual Studio 2022 Build Tools (~5 GB) + `rustup toolchain install stable-x86_64-pc-windows-msvc` + update `rust-toolchain.toml` channel to MSVC variant. Also resolves the `profiler_builtins` issue for `cargo llvm-cov` (separate Tier-3 entry from previous session). ~30 minutes including download.
- **Standalone minisign 0.12** (jedisct1, Frank Denis): download `minisign-0.12-win64.zip` from `https://github.com/jedisct1/minisign/releases` (~500 KB), unpack `x86_64/minisign.exe` into `~/.cargo/bin/` (already in PATH), use `minisign -G -W -f -s ~/.tauri/{name}.key -p ~/.tauri/{name}.key.pub` for no-password keypair (the `-W` flag = "do not encrypt secret key with a password" — acceptable for local dogfooding scope where private key stays in `~/.tauri/` gitignored; production HSM custody re-generates with password before public release).

Both `tauri signer generate` and standalone `minisign -G` produce **interoperable Minisign Ed25519 keypairs** — the tools both follow the public Minisign spec (`https://jedisct1.github.io/minisign/`). The verbatim base64 line from the `.pub` file (line 2, after `untrusted comment:` header) goes into `tauri.conf.json plugins.updater.pubkey` regardless of which tool generated it; `tauri-plugin-updater 2.x` accepts and verifies signatures from either.

Implication: when blocked on `cargo install tauri-cli` due to Windows GNU mingw limitations, the standalone-minisign fallback unblocks keypair generation without committing to the heavyweight MSVC switch. Document in the chunk's runbook that production-ready key custody re-generates the keypair WITH a password and uploads private + password to the production secrets manager (Azure Key Vault Premium SKU per security plan §Code-signing key custody).

See: `.andromeda/security-plan.md` §Code-signing key custody; `docs/runbooks/updater-key-rotation.md` Phase 1 (operator-side keypair generation); `pulse-app/tauri.conf.json` `plugins.updater.pubkey` field; previous Tier-3 entry "Windows GNU rustup toolchain doesn't bundle profiler_builtins for cargo-llvm-cov" (related rustup-toolchain limitation pattern).

---

## 2026-05-03 — Tauri 2.x transitively requires rustc ≥ 1.88

The architecture's security plan pins minimum rustc to 1.85 for Edition 2024 security-positive defaults (`unsafe_op_in_unsafe_fn`, tightened `if let` temporary scopes, `static mut` reference denial). Tauri 2.11.0's transitive dependency tree (`darling 0.23` requires 1.88, `plist 1.9` requires 1.88, `serde_with 3.19` requires 1.88, `time 0.3.47` requires 1.88, `icu_* 2.2` requires 1.86, `icu_normalizer_data 2.2` requires 1.86) pushes the effective floor to rustc 1.88+ for any project that compiles Tauri 2.

Phase 1 implementation chose `channel = "1.95.0"` in `rust-toolchain.toml` to match the host installation while satisfying the security plan's `1.85+` minimum (the AC's grep regex `^channel = "1\.(8[5-9]|9[0-9])'` matches 1.95). Future Tauri version bumps may push the floor higher — bumping `rust-toolchain.toml` is not a security-plan violation as long as the channel stays ≥1.85.

Implication for future Tauri-related chunks: when adding/upgrading Tauri 2.x or its plugins, check if transitive deps require a rustc bump. Coordinate the bump with the security-plan minimum (≥1.85) and the CI matrix runners.

See: `.andromeda/security-plan.md` §Anti-Patterns Universal + §Decisions Log open question on 1.84 → 1.85 bump; `rust-toolchain.toml`.

---

## 2026-05-03 — Tauri 2 capability JSON `identifier` field uses simple kebab-case names

Architecture §Occupied Resources references Tauri capability identifiers as `pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs` — these are **conceptual fully-qualified namespaced** names. The actual Tauri 2 capability JSON `identifier` field uses **simple kebab-case local** names (`default`, `tray`, `notification`, `updater`, `plugin-fs`); Tauri 2 does not accept colons in identifiers, and the bundle id `com.andromeda.pulse` provides implicit namespacing at runtime.

Filenames in `pulse-app/capabilities/` map 1:1 to the local identifiers (`default.json` → identifier `default`). The `pulse:` prefix is preserved in the architecture and security plans as the conceptual reference (e.g., when discussing "do not expose `pulse:updater` to webview JavaScript"), but the JSON file's `identifier` field uses just `updater`.

Implication: any future capability JSON edit (new TauRPC procedure → matching capability entry per security plan §API Security) should use the simple form. The `xtask capability-drift` check (route#22) will diff TauRPC routers against the file inventory by filename / local identifier — not against the namespaced form.

See: `.andromeda/architecture.md` §Occupied Resources Tauri capability identifiers; `.andromeda/security-plan.md` §API Security TauRPC capability authorization; `pulse-app/capabilities/*.json`.

---

## 2026-05-03 — Windows GNU rustup toolchain doesn't bundle profiler_builtins for cargo-llvm-cov

`cargo-llvm-cov` requires the `profiler_builtins` crate (provided by the Rust standard library precompiled with profiler runtime support). The Rust standard library precompiled binaries for `x86_64-pc-windows-gnu` do NOT include `profiler_builtins`, even with the `llvm-tools-preview` rustup component installed. Running `cargo llvm-cov nextest --workspace ...` fails with `error[E0463]: can't find crate for 'profiler_builtins'` during build-script compilation of common deps (e.g., `serde`, `typeid`, `zmij`).

Phase 1 acceptance criterion T8 (`cargo llvm-cov nextest --workspace --lcov --output-path lcov.info --summary-only`) fails on the local Windows GNU host for this reason. The other 11 acceptance test commands pass. Workarounds: (a) install MSVC toolchain — `rustup toolchain install stable-x86_64-pc-windows-msvc` (requires Visual Studio 2022 Build Tools install) and update `rust-toolchain.toml` channel to `1.95.0-x86_64-pc-windows-msvc`; (b) defer coverage to CI Linux/macOS runners where profiler runtime is bundled — route#5 base CI workflow will primarily exercise the coverage gate on those targets; (c) switch to `cargo-tarpaulin` as alternative (different coverage tool, not in plan AC).

Implication: the route#5 CI matrix workflow should run the coverage gate primarily on Linux + macOS. A Windows MSVC runner can also pass; a Windows GNU runner cannot without rebuilding std with profiler support (nightly-only via `-Z build-std`).

See: `.andromeda/test-plan.md` §3 Bootstrap phase 7 "coverage-tooling-install" + §10 Coverage thresholds; route#5 Base CI workflow chunk.

---

## Entry format

Each entry follows this structure:

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}

See: `.claude/docs/services/{service}.md` (or similar cross-reference)
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.

Other tiers:
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` section (critical, short)
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` sections (loaded when matching files touched)
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read)

See the classification gallery in the refactor plan `§3.5` for which tier a given learning belongs to. wrap-session applies this classification automatically during curation.

## Promotion

When this file grows beyond ~200 lines, `/wrap-session` suggests promoting some entries to topic-specific files (e.g., `.claude/docs/services/{service}.md` if the learning is about a specific service). Promotion is a user action, not automatic — wrap-session never moves entries without approval.

## Demotion from CLAUDE.md

If `CLAUDE.md` `USER:session-learnings` section gets too large (≥ 180 lines total CLAUDE.md), wrap-session suggests promoting old Tier 1 entries down to this file (Tier 3) to keep CLAUDE.md within size budget. This is also a user action.
