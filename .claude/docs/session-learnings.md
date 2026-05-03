# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

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
