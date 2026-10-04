# Scope — The declared Rust floor matches the code

**Marker:** `2026-10-04-declared-rust-floor-matches-the-code` · **Version:** andromeda-pulse-0.3.0 · **Taken up:** 2026-10-04 · **Mode:** normal take-up (the blocked head was skipped)

## Intent (working entry, verbatim title + hint)
The declared Rust floor matches the code — the workspace's declared minimum Rust version is the one the code actually
needs.

## Take-up record
- **Head skipped:** "L4 framing measured on the real model" (`working-route.md:164`) stays in place as the
  head-blocked entry, per the phase directive (overseer, founder-delegated, measured 2026-10-04 ~22:50). Its block
  re-checked at this take-up (2026-10-04T21:25Z): STANDING — no `llama-cli` on PATH, no `nvcc`, no `/opt/cuda`,
  `pacman -Q cuda cmake` → both not found, `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` unset. One narrowing of the
  directive's wording: `AI-Model/llama-b9305-cuda/llama-cli.exe` exists under `~` — the Windows PE build, not
  runnable here, so the premise (a LINUX CUDA binary) still stands. Nothing in the head's text or freight (2 CONTEXT,
  1 BLOCKED-ON) is doable without that binary; it carries no PREREQ / WATCH, so nothing folds into this chunk from it.
- **Order:** this entry is placed directly before "pre-push:linux runs natively on Linux" (founder ruling
  2026-10-04), so taking it up does not move the Epoch 4 close, which stays at that entry's wrap.

## What this chunk builds
1. **Raise the declared floor.** The workspace `Cargo.toml:25` `rust-version = "1.85"` (inherited by all 16 members via
   `rust-version.workspace = true`) becomes the floor the code actually needs. The entry states ≥ 1.89
   (`std::fs::File::lock`; let-chains need 1.88).
2. **Remove the guard the raise makes unnecessary.** The function-scoped `#[allow(clippy::incompatible_msrv)]` on
   `fetch_with_lock_dir` (`crates/corpus/src/keychain.rs:268-269`, the only `incompatible_msrv` site in the tree) is
   removed; clippy `-D warnings` stays green without it.
3. **The CARRY (folded, an obligation):** `corpus_key_survives_a_real_process_boundary`
   (`crates/corpus/src/keychain.rs:524`) leaves an empty lock file in the lock dir on its skip arm (no reachable
   credential store): it takes the lock before the store call fails and returns before its cleanup (measured at
   2026-10-04-corpus-key-creation-is-race-free: two such files in `/tmp` from the `env -i` stage 5 and its arm
   probe); the same file is reopened here. This chunk makes the skip arm leave no lock file behind.
4. **Sites stating the old floor as a minimum** (named by the entry; the spec sites are wrap amendments, never
   phase edits): security-plan §Security Anti-Patterns → Universal (`security-plan.md:470`, "NEVER let the
   rust-toolchain drift below 1.85.0"); `.claude/rules/security.md` §Rust toolchain (`:87`, "MUST pin to `1.85.0`
   minimum"); arch §Standard Contracts `app_info` example `rust_version` (`architecture.md:103`, reads `"1.84.0"`).

5. **The `/tmp` residue is removed and recorded** [val-1 2026-10-04, intent-incomplete: added at the P5 review by
   the overseer (founder-delegated) — "remove it inside the chunk and record it"]. The one 0-byte
   `andromeda-pulse-corpus-key-c2875066785cd2b7.lock` standing in `/tmp` at take-up is removed once, unheld, by exact
   name, with before/after readings in evidence; its origin (test residue vs the product's own lock re-created by
   the production-service MCP legs under `env -i`) is read by name recurrence after native pre-push stage 5.
6. **The boot smoke runs unattended under Xvfb** [val-1 2026-10-04, P5 review, overseer (founder-delegated)]: ports
   4317/4318 granted for this chunk; the founder is not at the desk, so the observability.rs-triggered boot smoke runs
   in the host's standing Xvfb form with the posture value recorded, not asserted; the native-Wayland leg is not run.

## Boundaries
- Not a toolchain bump: `rust-toolchain.toml` stays `channel = "1.95.0"`.
- No dependency upgrade (wasmtime 49 needs Rust 1.96 — out of scope; arch Stack line).
- No product behaviour change beyond what the declared floor itself feeds.

## Folded freight (from the working entry)
- **CONTEXT (verbatim facts, coordinates re-verified at HEAD):** `Cargo.toml` declares `rust-version = "1.85"`
  (verified `Cargo.toml:25`) while the toolchain is pinned at 1.95.0 (verified `rust-toolchain.toml`); the allow on
  `fetch_with_lock_dir` (verified `keychain.rs:268`); the three spec sites (verified, coordinates above).
- **CONTEXT mechanism claim (marker kept verbatim):** "the code needs ≥ 1.89 (`std::fs::File::lock`,
  measured at 2026-10-04-corpus-key-creation-is-race-free by clippy `incompatible_msrv` going red; let-chains
  already needed 1.88)" — VERIFIED as the OWN-CODE lower bound (`File::lock` at keychain.rs:276; let-chains at
  :134-136), and superseded as the floor by the dependency bound (1.95.0, below).
- **CONTEXT mechanism claim (verbatim):** "clippy checks APIs, not syntax, so a green clippy never proved the
  declared floor builds" — VERIFIED (rustc does not enforce `rust-version` against syntax; at msrv 1.95.0 clippy
  raised no `incompatible_msrv`). A declared floor EQUAL to the pinned channel is proven by construction by every
  pinned-toolchain build; a lower one would need a witness beyond clippy.
- **CONTEXT (ruling):** founder ruling 2026-10-04 (relayed live by the overseer): its own WHAT-only entry, placed
  directly before "pre-push:linux runs natively on Linux" so Epoch 4 still closes at that entry's wrap.
- **CARRY:** item 3 above.

## Inferred scope (not stated by the entry — closed at P3, 2026-10-04)
- The floor the code "actually needs" is the MAX of (a) the newest std API / language feature the workspace's own
  code uses and (b) the declared `rust-version` of every resolved dependency — VERIFIED, and (b) decides it: 28
  resolved packages (wasmtime 48.0.5 and its internal crates, cranelift 0.135.5, pulley 48.0.5) declare
  `rust-version = "1.95.0"` (`cargo metadata --offline`, research.md), and `pulse-app` depends on `plugins`
  non-optionally (`pulse-app/Cargo.toml:51`) — no toolchain below 1.95.0 builds the product. Item 1's target floor
  is therefore 1.95.0, not 1.89.
- `[premise-corrected: the app_info emit carries no rust_version field (health.rs:351-356); observability.rs:505 is
  the allowlist leaf; contract.rs:2344 asserts key presence only]` The declared floor reaches the product only as the
  `app_info` IPC envelope value (`crates/ui-bridge/src/health.rs:360`, `env!("CARGO_PKG_RUST_VERSION")`), which
  changes at compile time with no code edit; no log record carries it, and the `"1.85"` fixture is a sample value
  needing no change.
- No CI job or xtask verb builds against or reads the declared floor today — VERIFIED (every CI job installs the
  pinned toolchain via `dtolnay/rust-toolchain` "respects rust-toolchain.toml"; 0 `rust-version` consumers in
  xtask / workflows / scripts). Whether this chunk adds a floor witness is a P4 question.
- **Added at P3 (research finding, not an entry claim):** raising the clippy MSRV to 1.95.0 switches ON two
  MSRV-gated lints at 20 sites in 13 files (`manual_is_multiple_of` ×2, `collapsible_if` ×18 — research.md), so the
  raise owes those fixes for clippy `-D warnings` to stay green; one site set is in `pulse-app/src/observability.rs`
  (`FieldAllowlist::for_target`), a test-plan §3 boot-smoke trigger path. Windows/macOS `cfg` arms are unlinted here.
- **CARRY residue re-derived:** one `andromeda-pulse-corpus-key-*.lock` (0 B, 0o600) stands in `/tmp` at take-up,
  not the two the handoff names; the CARRY's mechanism is VERIFIED at HEAD (lock opened at keychain.rs:275 before the
  store call :277; the skip arm returns at :534, before the cleanup at :555-557).

## CI read at take-up (Setup 5a; base = last flip `268ae86`)
- `46b600a` (chore(setup-project) U02): ci#37235916991 — verdict not yet available (in progress; queued); secret-scan
  completed. Wall-clock: not yet available.
- `268ae86` (chunk wrap of 2026-10-04-l4-interpretation-names-its-triggering-cue): ci#37235592557 — verdict not yet
  available (in progress); secret-scan completed. Wall-clock: not yet available.
- Neither is folded as green; neither is red. Re-read at /implement or wrap.
