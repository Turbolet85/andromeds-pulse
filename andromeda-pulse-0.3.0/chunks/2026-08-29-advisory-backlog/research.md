# Codebase Research — 2026-08-29-advisory-backlog

## Scope
- **Depth:** moderate · **Reads:** 9 files/sections · **Globs/Greps:** ~14 (incl. 4 crates.io sparse-index probes + 4 `cargo tree` inversions)

## Files inspected
- `Cargo.lock` (targeted) — resolved versions at HEAD: rmcp 0.6.4 · wasmtime 43.0.2 · quick-xml 0.37.5 AND 0.39.2 · crossbeam-epoch 0.9.18 · anyhow 1.0.102 · lru 0.16.4 · h2 0.4.14 · strict-path 0.2.2.
- `Cargo.toml` (workspace) — the three direct requirements needing edits: `rmcp = { version = "0.6", features = ["server", "transport-io"] }` (:116, with the 0.6-pin rationale comment block :107–115 that must be rewritten with the bump), `wasmtime = { version = "43", features = ["component-model"] }` (:130, with the 43-pin rationale block :119–129), `lru = "0.16"` (:203). `anyhow = "1"` (:36) covers 1.0.103 → lock-only. `strict-path = "0.2"` (:49). `wat = "1"` (:131, version-independent of wasmtime).
- `deny.toml` (full) — carries the AUTHORITATIVE owned-set mapping in its own comment block (the "DELIBERATELY NOT IGNORED — 7 findings" list, + h2 0258 as the later 8th): 0189 rmcp VULN · 0222 wasmtime VULN · 0194+0195 quick-xml VULN ×2 · 0204 crossbeam-epoch VULN · 0190 anyhow unsound · 0253 lru unsound. Post-upgrade obligations INSIDE this file: rewrite/retire that comment block; refresh the rmcp-0.6-citing `darling`/`schemars` skip provenance; check-and-prune the `paste`/`proc-macro-error` ignores (cite rmcp 0.6) and the `im-rc` ignore cluster 0247/0250/0251 ("closes when wasmtime drops im-rc"); re-check the wasmtime-43 skip block (redox_users/wasm-encoder/fixedbitset/petgraph/target-lexicon). Prune-unnecessary-skip precedent already in-file (windows-* removal note, 2026-08-29).
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (:1–80) — **rmcp is DECLARED-BUT-UNUSED**: the single source contact repo-wide is `#[cfg(feature = "mcp-server")] use rmcp as _;` (:20). The bin runs its own tokio stdio loop over `mcp_server::jsonrpc` + `mcp_server::tools`.
- `crates/mcp-server/src/jsonrpc.rs` (:1–40) — hand-rolled JSON-RPC 2.0 + MCP protocol `2024-11-05` (serde structs, error codes, `tools_list_with_8_tools`); 421 lines. `tools.rs` (988 lines) dispatches the 8 tools by NAME (`ALL_TOOL_NAMES`, `dispatch_tool`) — no macro.
- `crates/mcp-server/Cargo.toml` — `rmcp = { workspace = true, optional = true }`; `mcp-server = ["dep:rmcp"]`; `[[bin]] andromeda-pulse-mcp` `required-features = ["mcp-server"]`.
- `crates/plugins/src/engine.rs` (full) + grep across `plugins/src` — wasmtime API surface is SMALL and core-stable: `Config::{wasm_component_model, epoch_interruption}` + `Engine` (engine.rs), `component::Linker` (capability.rs), `component::Component` (loader.rs), `ResourceLimiter`/`Store` (sandbox.rs). `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` (64 KiB) declared at engine.rs:40 with a compile-time bound assert (:47–48) — a const, deliberately NOT a `wasmtime::Config` method (2026-05-11 decision).
- `crates/buffer/src/drain.rs` (targeted) — the lru consumer: `LruCache::new(NonZeroUsize)` (:313), `.put` (:349, :531), `.get` (:495–496, promote-on-hit), `.pop_lru` (:536). No `.pop(&k)` call (the 0253 UAF site), but the bump lands regardless; these four methods are the API-compat check surface for 0.16→0.18.
- `crates/ingest/src/{http,grpc}.rs` (targeted grep) — the h2-adjacent security controls verified in force at HEAD: `CorsLayer::new()` (http.rs:145), `DefaultBodyLimit::max(MAX_DECODING_BODY_SIZE)` (:146), `max_decoding_message_size` on all three gRPC services (grpc.rs:391–398).
- `.github/workflows/ci.yml` (targeted grep) — supply-chain job runs `rustsec/audit-check` + `EmbarkStudios/cargo-deny-action` (both SHA-pinned) + `cargo auditable build`. **The test-plan §9 Cranelift assertion (`cargo tree -p wasmtime | grep -q cranelift`) is NOT wired** — zero `cranelift` occurrences in ci.yml.
- `.claude/docs/services/mcp-server.md` (targeted grep) — Tier-2 leaf is stale on two axes: "rmcp 0.3.x published line; reconciliation open" (published line reaches 3.1.4) and "JSON-RPC framing: `crates/mcp-server/src/rpc.rs` (rmcp-provided)" — the file is `jsonrpc.rs` and it is hand-rolled. Leaf refresh rides wrap curation/cascade, never a touchpoint.

## Graph impact (from the code-graph query; trace at `.andromeda/runs/2026-08-29T21-14-14Z-phase/tree-query-2026-08-29-advisory-backlog.json`, 2 queries, rust plane, both `db_state: fresh`)
- **`strict_path` refs** — 0 rows per the trace (corroborates the grep: zero `.rs` users; the drop arm has zero code blast radius).
- **`crate_edges` for `mcp-server` + `plugins`** — 11 rows per the trace; inbound: `pulse-app → plugins`, `ui-bridge → mcp-server`, `ui-bridge → plugins`; outbound: `mcp-server → {buffer, viz, snapshot, corpus, triage, interpretation, workspace-detector}`. The bumps' cross-crate exposure is compile-level only (no signature this chunk changes crosses a crate boundary).

## Upstream reachability (crates.io sparse-index probes — the lock-only path is proven, not assumed)
- **quick-xml 0.37.5 copy** — parent chain `tauri-winrt-notification 0.7.2 → notify-rust 4.17.0 (requires ^0.7) → tauri-plugin-notification 2.3.3`. `tauri-winrt-notification` **0.7.3 DROPS quick-xml entirely** → `cargo update -p tauri-winrt-notification` removes this copy.
- **quick-xml 0.39.2 copy** — parent `plist 1.9.0 → tauri-utils 2.9.0`. `plist` **1.10.0 requires `quick_xml ^0.41.0`** (the dep key is RENAMED `quick_xml` — a first index probe filtered on `name == "quick-xml"` and read plist as having no quick-xml dep at any version, contradicting the lockfile; re-probing with the `package` field surfaced the rename) → `cargo update -p plist` moves this copy to the fixed line.
- **crossbeam-epoch 0.9.20** published (parents crossbeam-deque ← rayon-core + `assert_fs` dev-chain; ^0.9-compatible) → `cargo update -p crossbeam-epoch`.
- **h2 0.4.16**: `hyper 1.9.0 → axum 0.8.9` (+ `axum-test` dev-dep in ingest) → `cargo update -p h2` (entry-stated; Conductor lock-only precedent 2026-08-18).
- **rmcp**: published line reaches **3.1.4**; `server` + `transport-io` features exist at BOTH 1.4.0 and 3.1.4 (index-verified) — the manifest line works at either target, and since no API is used, the jump is zero-code-churn at any version.
- **wasmtime**: **46.0.3** (the advisory-stated safe line, patched) and **48.0.1** (latest) both published; `component-model` feature exists at both.
- **lru**: 0.18.2 / 0.18.3 published.

## Patterns detected
- **Declared-but-unused dependency, second instance** (`bin/andromeda-pulse-mcp.rs:20`): rmcp mirrors strict-path's shape — linked into the graph, zero API usage — except rmcp is additionally load-bearing for the MASTERS' mechanism claim ("8 `#[tool]` methods", "rmcp's macros are the canonical surface"). The wire contract (JSON-RPC 2.0, MCP `2024-11-05`, exactly 8 tools) IS honoured by the hand-rolled layer; the MECHANISM claim is not implemented. This is the 2026-06-28 research-corrects-intent family: surface + record, never silently follow or silently deviate.
- **Workspace-dep single edit point** (`Cargo.toml` `[workspace.dependencies]`): all three requirement bumps are one-file edits; member manifests inherit via `.workspace = true`.
- **Pin-rationale comment blocks ride the requirement lines** (Cargo.toml:107–115 rmcp, :119–129 wasmtime): both cite the superseded state (0.6 decision; 43-pin RUSTSEC list) and must be rewritten WITH the bumps — the in-manifest analog of the deny.toml comment obligations.

## Conventions to follow
- **ID-scoped provenance on every deny.toml delta** (deny.toml throughout; rules/security.md 2026-05-03/2026-05-20): new skips owner-named + reasoned + closing-condition; unnecessary skips PRUNED (windows-* precedent in-file).
- **Split gate pair** (rules/security.md 2026-08-17): `bans licenses sources` = pass/fail; `advisories` = separately-observed, exit read directly.
- **Run scope matches build scope** (test-plan §3; measured 2026-08-15 naming `wasmtime_internal_cache` + `libduckdb_sys`): narrowed runs use `-E` under `--workspace`, never `-p pulse-app`.
- **Bindings regen LAST** (rules/security.md 2026-06-12): `capability-drift` reads the staged copy after all cargo ops.

## New files to create
- (none — unless the P4 fork lands ADOPT for strict-path, which would add no new file either; a conversion edits an existing guard site)

## Files to modify
- `Cargo.toml` (workspace) — rmcp / wasmtime / lru requirement lines + their rationale comment blocks; strict-path line 49 removed on DROP.
- `Cargo.lock` — via targeted `cargo update -p` set (anyhow, crossbeam-epoch, h2, tauri-winrt-notification, plist) + the three manifest-driven bumps.
- `deny.toml` — owned-set comment block retirement + skip/ignore provenance refresh + prunes per post-bump measurement.
- `crates/config-watcher/Cargo.toml` · `crates/corpus/Cargo.toml` · `crates/triage/Cargo.toml` · `crates/workspace-detector/Cargo.toml` · `pulse-app/Cargo.toml` — the five `strict-path.workspace = true` lines removed on DROP (untouched on ADOPT).
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` — `use rmcp as _;` line: kept under bump (still binds the dep), removed under drop-rmcp; `crates/mcp-server/Cargo.toml` feature list adjusted only under drop.
- `.github/workflows/ci.yml` — the missing test-plan §9 Cranelift assertion step (mandated, wasmtime-crossing chunk is its natural owner); SHA-pinning rules apply if any action is touched (none needed for a `run:` step).
- (possible, measured at implement) `crates/buffer/src/drain.rs` — only if lru 0.18 moved one of the four used methods; the 2026-05-20 API-stability check pattern applies.

## Open questions
- **strict-path: adopt or drop?** → blocks: plan-decision (P4 fork; operator-owed per the CARRY; research supplies: zero users verified, decline precedent, docs-already-aligned, DROP has zero code blast radius).
- **rmcp disposition: bump-to-3.x / bump-to-1.4 / drop-the-dep?** → blocks: plan-decision (P4 fork; the declared-but-unused finding makes all three zero-code-churn on the sidecar's executed paths, but drop crosses arch §Established Decisions [MCP Server Surface] wording and changes the wrap-amendment set).
- **wasmtime 46 vs 48** → blocks: plan-decision — RESOLVED BY LEAN, not asked: the entry states the target line (≥46.0.2) and the in-manifest 43-pin precedent chose the advisory-patched line deliberately; the plan leans 46 ("46" requirement → 46.0.3), operator can override at review.
