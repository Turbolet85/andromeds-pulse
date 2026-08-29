# Scope — 2026-08-29-advisory-backlog

**Chunk:** Advisory backlog
**Promoted:** 2026-08-29 · working-route line 116 (first markerless) · Epoch 4 — Polish & ship: verification
**Intent anchor (working entry, condensed):** the RustSec findings that have a stated safe upgrade are UPGRADED rather than accepted (operator-directed 2026-08-15 at the corpus-key-persistence wrap), plus the `strict-path` adopt-or-drop CARRY — both dependency-manifest decisions touching `Cargo.toml` and the `cargo deny` surface, landed as one pass to avoid two lockfile churns.

## Outcome 1 — the 8 owned RustSec upgrades land

Every advisory in the owned set — kept designed-RED under the visible-disposition rule precisely because each states a safe upgrade — is upgraded, never ignore-listed:

| Crate | Target | Advisory substance (per entry) |
|---|---|---|
| `rmcp` | ≥1.4.0 | DNS rebinding, Streamable HTTP transport (Pulse uses stdio — context, not an excuse) |
| `wasmtime` | ≥46.0.2 | stores mix up type indices between engines |
| `quick-xml` | ≥0.41.0 | TWO ids: quadratic duplicate-attribute scan + unbounded namespace allocation |
| `crossbeam-epoch` | ≥0.9.20 | (entry states target only) |
| `anyhow` | ≥1.0.103 | (entry states target only) |
| `lru` | ≥0.18.2 | (entry states target only) |
| `h2` | ≥0.4.16 | RUSTSEC-2026-0258: empty DATA frames accepted/queued without limit → unbounded memory / length-overflow panic. Low severity. Stated upgrade is LOCK-ONLY (`cargo update -p h2`); reached via `hyper 1.9.0 → axum 0.8.9`. Cross-project precedent: Conductor resolved the same id 2026-08-18 lock-only — cite its chunk record for the shape, never copy content |

- Owned id set at last first-hand enumeration (2026-08-29 wrap): RUSTSEC-2026-**0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258** — 8 DISTINCT ids, 10 deny blocks (quick-xml's two ids ride two lockfile versions). Five of the eight are vulnerabilities.
- Id→crate mapping VERIFIED (deny.toml's own owned-set comment block + lockfile at HEAD): **0189** rmcp 0.6.4 VULN · **0222** wasmtime 43.0.2 VULN · **0194 + 0195** quick-xml 0.37.5/0.39.2 VULN ×2 · **0204** crossbeam-epoch 0.9.18 VULN (invalid pointer deref in `fmt::Pointer`) · **0190** anyhow 1.0.102 unsound (`Error::downcast_mut`) · **0253** lru 0.16.4 unsound (use-after-free in `LruCache::pop`) · **0258** h2 0.4.14 low-severity vuln. The entry's "five are vulnerabilities" counts the original 7-id set (0189/0222/0194/0195/0204); with h2 the vuln-class count is six.
- [premise-corrected: per-crate requirement analysis at HEAD] Only THREE targets need manifest requirement changes — `rmcp` (`"0.6"`, workspace Cargo.toml:116), `wasmtime` (`"43"`, :130), `lru` (`"0.16"`, :203). FIVE close lock-only: `anyhow` (`"1"` covers 1.0.103), `crossbeam-epoch` (transitive, ^0.9 covers 0.9.20 — published, verified), `h2` (transitive via hyper 1.9.0, ^0.4 covers 0.4.16), and BOTH `quick-xml` copies via semver-compatible PARENT updates — `tauri-winrt-notification` 0.7.2→0.7.3 (notify-rust requires ^0.7; 0.7.3 DROPS quick-xml entirely, killing the 0.37.5 copy) and `plist` 1.9.0→1.10.0 (requires `quick_xml ^0.41.0`, moving the 0.39.2 copy to the fixed line). Both parent moves verified against the crates.io sparse index.

## Outcome 2 — the `strict-path` adopt-or-drop decision is made and executed (CARRY, 2026-08-26-l4-runtime-security-residuals)

Hypothesis to re-verify at research (folded from the CARRY, not yet re-derived at HEAD):
- `strict-path` 0.2.2 is declared and linked with ZERO `.rs` users repo-wide — workspace `Cargo.toml` + FIVE crate manifests (`config-watcher` · `corpus` · `triage` · `workspace-detector` · `pulse-app`) — while every shipping confinement is hand-rolled `std` (`publish_workspace_key` both-sides-canonicalize + `starts_with`; the plugin loader rejects `..` then canonicalizes). Evaluated as the L4 guard's primitive at that chunk and DECLINED at its P4 review.
- The decision owed here: **adopt it at ≥1 real boundary, or drop all declarations**. The doc side is already aligned to reality (security-plan §Bootstrap phases records the zero-user state; the `workspace-detector` service leaf names `std` explicitly) — what is open is the dependency, not its description.
- Hypothesis VERIFIED at HEAD: workspace `Cargo.toml:49` (`strict-path = "0.2"`, lockfile 0.2.2) + exactly the five named member manifests (`.workspace = true` each) + ZERO `.rs` users (grep AND code-graph refs query, 0 rows). If DROP is chosen: remove the workspace dep + the five member declarations; the CLAUDE.md §Critical Warnings line "MUST canonicalize via `strict-path`", the `rules/security.md` wording, and security-plan §Bootstrap carve-out (b) + §Anti-Patterns → Input ("the intended primitive for the other product path boundaries") become wrap-amendment / cascade material (verified: all still name strict-path as intended primitive at HEAD) — surfacing, not pre-deciding.
- If ADOPT is chosen: one shipping boundary converts (candidate: a data-dir-confined path var), with behaviour pinned before/after — but the prior chunk's decline (unmeasured Windows `\\?\` behaviour, zero users) weighs against adopt, and every shipping confinement is measured hand-rolled `std`; the decision is the operator's (a true fork, asked at P4).

## Outcome 3 — post-upgrade re-enumeration + pin #22 FULL-FORM probe (PREREQ, rides to wrap)

- Pin #22 rides this chunk; **this entry's wrap = session 58 = the INTERVAL POINT owing the FULL-FORM `cargo audit` probe.**
- Basis as last verified first-hand (2026-08-29 wrap, carried as hypothesis): `cargo audit` cannot load the RustSec DB (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`, cargo-audit 0.22.2, true exit 1 read directly); overlap signal `cargo deny check advisories` designed-red at the 8 ids; `bans licenses sources` exit 0.
- The bumps may CHANGE the owned set → after the upgrades, **re-enumerate DISTINCT `RUSTSEC-` ids from scratch** (never blocks, never a carried count, no "Nth consecutive" ordinal). Any NEW finding surfaced by the re-enumeration gets a visible disposition (named owner if upgradeable; ID-scoped ignore with provenance ONLY if no-safe-upgrade).
- The deferral ENDS the first time `cargo audit` loads (security-plan clause) — the full-form probe at wrap checks this.

## Blast radius / surfaces touched

- **`wasmtime` 43→46+ crosses the PLUGIN HOST** (`crates/plugins` — Engine+Config posture: Cranelift-on-x86_64, `epoch_interruption(true)`, per-Store ResourceLimiter, WIT for 3 categories). Needs its own verification per the entry. Cranelift-on-x86_64 must remain the backend and no non-Cranelift feature flag may appear (security rule — verified: `component-model` is the only feature the workspace manifest requests, and it exists at 46.0.3/48.0.1). API surface measured SMALL: `Config`/`Engine` (engine.rs), `component::Linker` (capability.rs), `component::Component` (loader.rs), `ResourceLimiter`/`Store` (sandbox.rs) — long-stable core APIs. ALSO measured: the test-plan §9 Cranelift build-time CI assertion (`cargo tree -p wasmtime | grep -q cranelift`) is NOT wired in `ci.yml` at HEAD — a mandated-but-missing gate this chunk's wasmtime verification naturally lands.
- **`rmcp` crosses the MCP SIDECAR surface — but only its BUILD GRAPH.** [premise-corrected: rmcp is DECLARED-BUT-UNUSED at HEAD — the sidecar's single source contact is `#[cfg(feature = "mcp-server")] use rmcp as _;` (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:20`); the entire MCP protocol is HAND-ROLLED serde JSON-RPC (`jsonrpc.rs`, 421 lines, protocol 2024-11-05, `tools_list_with_8_tools`) with name-dispatch in `tools.rs` (988 lines). The masters' "8 `#[tool]` methods via rmcp macros" describes a mechanism the code does not implement — the WIRE contract (JSON-RPC 2.0 / MCP spec) IS honoured. Consequence: the bump is ZERO code churn; `server`+`transport-io` features exist at both 1.4.0 and 3.1.4 (index-verified); the version target and even drop-the-dep become open dispositions (asked at P4). The entry-mandated sidecar verification narrows to: feature-gated build compiles + the committed cross-process legs still pass.] Double-gate preserved either way.
- Manifests: workspace `Cargo.toml` + `Cargo.lock` (all three requirement changes live in the workspace manifest; member manifests use `.workspace = true` and need no edit). `deny.toml` churn is CONCRETE, not speculative: the owned-set comment block (the "DELIBERATELY NOT IGNORED — 7 findings" list) goes obsolete on upgrade; the `darling`/`schemars` skips and `paste`/`proc-macro-error` ignores cite rmcp 0.6; the `im-rc` ignore cluster (0247/0250/0251) cites the wasmtime stack ("closes when wasmtime drops im-rc" — check post-bump and prune if closed); the wasmtime-43 skip block (wasm-encoder/fixedbitset/petgraph/target-lexicon/redox_users) may move under 46. Handled per the ID-scoped-provenance discipline + the prune-unnecessary-skip precedent; `multiple-versions = "deny"` never relaxed; the tonic canary never skipped.
- The cargo-deny gate pair stays split: `bans licenses sources` = pass/fail gate; `advisories` = observed separately (2026-08-17 rule).

## Boundaries (NOT in scope)

- npm advisory coverage — the next tail entry owns it.
- Ignore-listing any finding WITH a stated safe upgrade — banned (green-by-ignore is a lie).
- Relaxing `multiple-versions = "deny"`, skipping the tonic canary, or weakening the designed-red discipline.
- Diagnostics un-muting / staged-bindings / ACL-rejection items — later tail entries.
- Rewriting shipped hand-rolled `std` guards wholesale: the CARRY owes a DECISION plus its minimal execution, not a confinement-primitive migration across the codebase (boundary confirmed against the CARRY's own framing; unchallenged by research).

## Verification expectations (outcome level — concretized at P4/P5)

1. Post-bump `cargo deny check advisories`: none of the 8 owned ids remain; the from-scratch re-enumeration is recorded with any new ids dispositioned visibly.
2. `cargo deny check bans licenses sources` exit 0 after the bumps.
3. Workspace suite green (nextest); the plugin host and MCP sidecar surfaces each get their OWN verification (entry-mandated), not just compile-green. Refined by research: for the sidecar, the feature-gated build + the committed cross-process subprocess legs (`pulse-app/tests/e2e_p3_mcp_resolve_content.rs` — feature-gated, OUTSIDE the default workspace run, so it must be explicitly invoked); for the plugin host, the `crates/plugins` suite (engine/sandbox/capability/loader tests incl. the fixture-WASM boundary tests) + a post-bump Cranelift-presence assertion (`cargo tree -p wasmtime | grep cranelift`).
4. strict-path end-state is binary and verifiable: ≥1 real `.rs` user at a shipping boundary, OR zero `strict-path` declarations across the six manifests.
5. Wrap-side: session-58 full-form `cargo audit` probe recorded (basis re-read directly), owned set re-enumerated from scratch.
