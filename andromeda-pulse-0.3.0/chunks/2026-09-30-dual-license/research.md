# Codebase Research — 2026-09-30-dual-license

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 22
- **Harness rules consulted:** none — no live leg in this chunk (no boot-path file is touched: `tauri.conf.json`, `pulse-app/src/main.rs`, `crates/ui-bridge/src/`, `pulse-app/src/observability.rs`, `pulse-app/capabilities/*.json` are all outside the modify set)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a's only row, `1dfca74`, was in progress, not red) and no CI-reading entry outside the operator leg
- **Chunk base:** `1dfca7412b0026cfbb5221ebf460bb5665983f9e` (W182 — every diff-shaped probe below and in the plan names it)
- **Code graph:** not queried — the chunk changes no Rust symbol's signature or call graph (manifest values, two new text files, a CI heredoc's text, one new xtask test module); there is no symbol whose callers could thread. Recorded, not skipped by accident.

## Files inspected
- `Cargo.toml` (1-52) — `[workspace.package]` at :22, `license = "MIT"` at :26 (re-derived: `grep -rn --include=Cargo.toml -E '^\s*license' .` → 1 value line + 16 `license.workspace = true` lines, one per workspace member at `{member}/Cargo.toml:6`); `authors = ["andromeda-pulse contributors"]` at :28 (out of scope); `toml = "0.8"` in `[workspace.dependencies]` at :51.
- `crates/*/Cargo.toml`, `pulse-app/Cargo.toml`, `xtask/Cargo.toml` (:6 each) — all 16 members inherit via `license.workspace = true`; none overrides (same grep, 16 hits, 0 literal `license = "…"` outside the root).
- `crates/triage-experimental/Cargo.toml` — `publish = false`, NOT a workspace member (`Cargo.toml:3-19` members list) and untracked (`git ls-files crates/triage-experimental` → empty). Out of scope.
- `pulse-app/ui/package.json` (:6) — `"license": "MIT"`. It is the only tracked npm manifest (`git ls-files '*package.json'` → 1 file).
- `pulse-app/ui/package-lock.json` (1-12, 55-62) — the ROOT entry `packages[""]` carries `"license": "MIT"` at :10; the second `"license": "MIT"` hit at :60 belongs to `node_modules/@asamuzakjp/css-color` (a dependency, untouched).
- `xtask/src/npm_gate.rs` (145-206) — `lockfile_packages` SKIPS the root entry (`if path.is_empty() { continue; // the root project entry }`, :152-154); `check_licenses` walks dependency entries only. So the root package's license value cannot move `check:npm-supply-chain`'s verdict in either direction.
- `deny.toml` (78-97, whole-file key scan) — `[licenses] allow` lists both `"MIT"` (:80) and `"Apache-2.0"` (:81); NO `private` key anywhere (`grep -n private deny.toml` → 0 hits).
- `.github/workflows/update-channels.yml` (185-200, 350-360) — the generated Homebrew formula carries `license "MIT"` (:196) and the generated Scoop manifest `"license": "MIT",` (:357). Both are shipped distribution manifests; neither was named in scope.
- `.github/workflows/ci.yml` (122-123, 464-521) — `capability-drift` step :122; `supply-chain` job :464 runs `cargo deny check bans licenses sources` via cargo-deny-action (:494-497) and `cargo xtask check:npm-supply-chain` (:520-521).
- `pulse-app/tauri.conf.json` — no `license` / `licenseFile` key (`grep -n -i license pulse-app/tauri.conf.json` → 0).
- `tauri-utils-2.9.0/src/config.rs:1632-1634` (the resolved `tauri-utils` per `Cargo.lock:7162-7163`) — `bundle.license`: "If not set, defaults to the license from the Cargo.toml file." So every bundle inherits the `[workspace.package]` value; no `tauri.conf.json` edit (and no boot-smoke trigger).
- `xtask/src/main.rs` (9-20 mod list; 331, 1594 test modules; 658+ `env!("CARGO_MANIFEST_DIR")` workspace-root idiom) — xtask carries 13 modules and co-located `#[cfg(test)]` tests that run under `cargo nextest run --workspace`.
- `conductor` repo `314d68b` (`deny.toml` diff) + `LICENSE-MIT` / `LICENSE-APACHE` at `cdb7082` — Conductor's deny change REMOVED `private = { ignore = true }`; its `LICENSE-MIT` opens `Copyright (c) 2026 Turbolet85` (25 lines); its `LICENSE-APACHE` is the verbatim Apache License 2.0 (201 lines, md5 `1836efb2eb779966696f473ee8540542`) with the appendix template `Copyright [yyyy] [name of copyright owner]` left as the standard text (:189). Both `i/lf w/lf`.
- `.andromeda/architecture.md` — states no license anywhere (`grep -n MIT` → 0 license hits; §Infrastructure Patterns :256 "Public OSS desktop app distributed through GitHub Releases"). The CLAUDE.md overview's "Public OSS (MIT)" originates in `.andromeda/input.md`, not a spec master.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- none — no symbol surface (see Scope). The one new code is a test module in `xtask`, which calls no changed signature and is called by nothing.

## Patterns detected
- **cargo-deny already license-checks every workspace member** — MEASURED, not inferred: a scratch config (`deny.toml` minus its `"MIT",` line, run as `cargo deny --config {scratch} check licenses`, cargo-deny 0.20.2) exits 4 with 225 `error[rejected]` blocks, 16 of them on `path+file:///D:/dev/projects/andromeda-pulse/{member}#0.1.0-synthesized.toml` — one per workspace member (buffer · config-watcher · corpus · curation · ingest · interpretation · mcp-server · plugins · pulse-app · security · snapshot · triage · ui-bridge · viz · workspace-detector · xtask). `cargo deny list -l crate` lists the same 16 under MIT. cargo-deny's default `private.ignore = false` is in force because `deny.toml` sets no `private` key.
- **The equality a deny-based witness would need does NOT hold**: for a deny subject to be RED at `1dfca74` and GREEN after, the base must fail `cargo deny check licenses` — it does not (own crates pass as MIT, which is allowed), and `MIT OR Apache-2.0` also passes (both allowed). A deny-based red-before/green-after reading is vacuous here.
- **xtask repo-root idiom** (`xtask/src/main.rs:658`): `Path::new(env!("CARGO_MANIFEST_DIR")).parent()` resolves the workspace root; `npm_gate.rs:314-319` (`ui_dir()`) is the same shape.
- **Tri-state xtask verdict** (`npm_gate.rs:339`, `staged_gate.rs`) — exit 0 green · 1 findings · 2 cannot-evaluate with one JSON verdict; the precedent IF the witness becomes a verb.
- **Distribution-manifest license syntax** (fetched): Scoop's App-Manifests wiki — "If the entire application is dual licensed, separate licenses with a pipe symbol (|)" → `"license": "MIT|Apache-2.0"`; Homebrew Licence Guidelines — `license any_of: ["MIT", "0BSD"]` is the either-of form → `license any_of: ["MIT", "Apache-2.0"]`.

## Conventions to follow
- **SPDX expression** `MIT OR Apache-2.0` in Cargo (`Cargo.toml:26`) and npm (`package.json:6`, lock root :10); channel-native syntax in the two generated channel manifests (`update-channels.yml:196`, `:357`).
- **LF pin** — `.gitattributes` carries `* text=auto eol=lf` (upgrade U06 `ok`); the two new root files are LF like Conductor's.
- **pulse-app tests are never in `pulse-app/src/`** (tests-history 2026-08-30 ratchet); a witness in `xtask/src/` as a co-located `#[cfg(test)] mod` is the in-repo form and runs under the standard `cargo nextest run --workspace` gate and CI's lint-test job.
- **Lockfile edit is minimal** — the root entry's one line only; no `npm install` regen (tests-history: vitest 4.1.11 resolved versions must stay intact).

## New files to create
- `LICENSE-MIT` — the MIT License text, `Copyright (c) 2026 Turbolet85` (Conductor's shape)
- `LICENSE-APACHE` — the Apache License 2.0 text, verbatim (Conductor's shape)
- `xtask/src/license_check.rs` — the dual-license witness (form decided at P4)

## Files to modify
- `Cargo.toml` — `[workspace.package] license = "MIT OR Apache-2.0"`
- `pulse-app/ui/package.json` — `"license": "MIT OR Apache-2.0"`
- `pulse-app/ui/package-lock.json` — root entry `packages[""]` license, same value
- `.github/workflows/update-channels.yml` — Homebrew `license any_of: ["MIT", "Apache-2.0"]` and Scoop `"license": "MIT|Apache-2.0"`
- `xtask/src/main.rs` — `mod license_check;` (both witness forms); a `Cmd` arm only if the witness is a verb
- `xtask/Cargo.toml` — `toml.workspace = true` if the witness parses TOML (already in the lockfile; no new crate)
- `.github/workflows/ci.yml` — only if the witness becomes a verb wired into the `supply-chain` job

## Open questions
- Witness form: an xtask co-located TEST (rides the standard nextest gate, no registration) vs a new xtask VERB `check:license` wired into CI's `supply-chain` job (arch §Occupied Resources registration at wrap) → blocks: plan-decision
