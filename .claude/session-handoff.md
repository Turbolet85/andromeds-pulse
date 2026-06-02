# Session Handoff

**Last Updated:** 2026-06-02T18:50:00Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** `<session 170 chore(wrap) commit — pending this turn>` (prior HEAD: `44af47b` docs(arch): acknowledge chunk #94 MCP tools (8 #[tool] methods))

## Current State

- **Last completed chunk:** route#94 "MCP server + tool exposure" (Epoch 9 — Foundation v0.2.0; commit `9663b75`, State-H healed this wrap from "pending"). **TERMINAL chunk — route is 94/94 implemented.**
- **Next chunk:** none registered. Epoch 9 + the v0.2.0 route are complete. Forward options: (a) deliberate `/andromeda-arch` touch for the deferred "MCP = equal-tier output channel" structural framing (see Deferred decisions); (b) `/andromeda-evolve --allow-route-append` to extend from `docs/v0_2_0/pulse-v0_2_0-route.md` §93 (Export for community training) / §94 (Config hot reload); (c) resume the L4 "red-dot" debug (setup preserved — see below).
- **In-progress phase:** none.
- **Phase artifacts present:** `.andromeda/phases/phase-91/` (chunk #94, implemented session 169).

## Andromeda State Detection (states A-K)

All 11 CLEAR.
- **A** ✓ no in-progress runs. **B** N/A (no project.yaml). **C** ✓ arch.md (18:12) < CLAUDE.md (18:31). **D** ✓ route present, 94 chunks. **E** ✓ no unplanned registered chunk (route complete). **F** ✓ nothing planned-unimplemented. **G** ✓ single run. **H** ✓ commit_sha HEALED "pending" → `9663b75` this wrap (HEAD-reachable). **I** ✓ arch.md edited this session but plan_freshness.arch_mtime re-captured (2026-06-02T18:12:47Z) + the MCP-tools amendment archived → no freshness mismatch. **J** ✓ dep-tree + api-surface both reconciled this wrap. **K** ✓ in_progress null.

## Drift Detection (6 dimensions)

All 6 CLEAR — `state.yaml.drift_warnings = []`. Both session-169 warnings resolved this wrap:
- **D1** (session-169 api-surface mcp-server reconcile FAILURE, disk-full) — RESOLVED: `plugins` reconciled cleanly this wrap (158 lines, 34s, disk ~178 GB headroom), `reconcile_failed=false`. The mcp-server chunk-#94 surface remains uncaptured = **R1-accepted per-crate lag** (cursor revisits mcp-server later in cycle-3), NOT drift.
- **D3** (arch enumerated 4 MCP `#[tool]` methods; chunk #94 added 4 more) — RESOLVED by `44af47b` manual STRUCTURAL arch amendment: §Stack + §Established Decisions [MCP Server Surface] + §Conventions + §Standard Contracts now enumerate all 8 tools (matches code). The "MCP = one of three equal-tier output channels" framing is a DEFERRED arch-body enhancement (Deferred decisions), tracked separately — a conceptual doc framing, not a code-drift.
- D2/D4/D5/D6 clear (D5: CLAUDE.md 18:31 > arch.md 18:12 > route.md 05-31 → no upstream staleness).

## Spec Amendments (this session)

Archived this session: **1** amendment.
- **Plan(s):** `.andromeda/architecture.md` (§Stack / §Established Decisions [MCP Server Surface] / §Conventions / §Standard Contracts)
- **Decisions Log:** N/A — structural body edit (not a §Decisions Log / §Architecture Registry Updates entry; `manual_structural_amendment: true`)
- **Trigger:** manual structural amendment (D3 closure — arch enumerated 4 MCP tools; chunk #94 added 4 more). No chunk/phase/harness; user-directed after `/andromeda-evolve --allow-arch-registry` refused the structural sections.
- **Lifecycle:** applied 2026-06-02T18:13:31Z (commit `44af47b`) | noted 2026-06-02T18:50:00Z | propagated `manual-cascade-2026-06-02` (CLAUDE.md `:modules` line 30 + `.claude/docs/services/mcp-server.md`, 4→8) | archived 2026-06-02T18:50:00Z (this wrap)
- **Marker:** `.andromeda/runs/2026-06-02T18-13-31-spec-amendment-acknowledge-chunk-94-mcp-tools/amendment.md` (gitignored, forensic)

`spec_amendments.active` empty post-archive; archive 76 → 77 (over 50 soft-cap; pruning deferred — run-dir markers remain forensic).

## Key Decisions This Session

1. **Manual STRUCTURAL arch amendment (committed `44af47b`):** acknowledged chunk #94's 4 new MCP `#[tool]` methods (4→8) across arch §Stack + §Established Decisions [MCP Server Surface] + §Conventions + §Standard Contracts + Tier-2/3 mirrors (CLAUDE.md :modules + mcp-server.md). `/andromeda-evolve --allow-arch-registry` correctly refused (structural ⊄ registry-only scope); used the manual edit + marker + state.yaml entry + manual cascade path (per CLAUDE.md 2026-05-16 after-MVP + session-137/144/145 precedent).
2. **Demo / L4 "red-dot" thread REVERTED:** the 2026-06-01→06-02 chase proved the full L0→storm→digest→L4-GPU-inference loop works on injected telemetry, but could not produce a clean incident→red-service-dot (3B-model dismiss + over-cranked back-pressure). 4 demo source edits reverted via `git checkout HEAD`; source tree clean. Debug setup KEPT (untracked/gitignored): `crates/ingest/examples/inject_demo.rs` + `AI-Model/` (b9305 llama-cli + CUDA DLLs + tokenizer.json + RESUME-NOTE.md).
3. **build.rs 8 MB tokenizer-cap bug found** (Tier-3 curated): `crates/triage/build.rs` truncates the ~9 MB Llama-3 tokenizer.json at 8 MB → digest-assembler tokenizer init fails. Worked around via `ANDROMEDA_LLAMA3_TOKENIZER_PATH`; proper fix = raise the cap.

## Files Modified

**This session (committed `44af47b`):** `.andromeda/architecture.md`, `.andromeda/state.yaml`, `.claude/docs/services/mcp-server.md`, `CLAUDE.md`.
**This wrap (pending commit):** `.claude/docs/session-learnings.md` (Tier 3), `docs/andromeda-improvements.md` (P27), `.andromeda/context/dependency-tree.md` (reconcile), `.andromeda/context/api-surface.md` (reconcile), `.andromeda/state.yaml` (archive + bookkeeping), `.claude/session-handoff.md`.

## Curation Summary (this wrap)

- **Tier 1** (CLAUDE.md USER:session-learnings): 0 — the pipeline working-path candidate dedup'd against the existing 2026-05-16 entry.
- **Tier 2** (.claude/rules/*): 0.
- **Tier 3** (.claude/docs/session-learnings.md): 1 — `crates/triage/build.rs` 8 MB tokenizer-cap gotcha (confidence 0.65).
- **Filtered:** 1 duplicate (pipeline working-path) + 0 task-specific + 0 conflicts + 0 deferred.
- **Andromeda pipeline:** 1 patch filed — **P27** (evolve Refuse-1 + setup-project --delta arch-halt both stale-redirect STRUCTURAL arch amendments to /andromeda-arch, which is greenfield-only). Mode P.
- api-surface per-crate: `plugins` reconciled cycle-3 (158 lines zero-diff); cursor → pulse-app. mcp-server chunk-#94 surface = R1-accepted lag.

## Last Failed Command

(none) — the session-169 `cargo +nightly public-api -p mcp-server` disk-full failure is RESOLVED (disk freed; `plugins` reconciled cleanly this wrap).

## Tests Status

**PASS (no tracked code changed this session — `.md`/`.yaml` only):**
- `cargo nextest run -p security --profile ci` smoke = **14/14** (0.141s).
- Full suite unchanged from session-169's `9663b75` green: `cargo nextest run --workspace --all-features --profile ci` = **1591/1591 + 1 skip**; default-features = 1570/1570 + 1 skip; webview vitest 613/613; capability-drift clean.
- Dead-test scan (P15): 16 `#[cfg(test)]` files in `pulse-app/src/` (binary, test=false) — carryover, +0 new this session; warning-not-fatal.

## Next Recommended Action

Route is **94/94 — Epoch 9 + the v0.2.0 route are functionally complete.** No drift, no pending amendment, all states clear. Pick one:
1. **`git push origin main`** — branch is **8+ commits ahead** of origin (verify with `git status`).
2. **Deliberate `/andromeda-arch` touch** for the deferred "MCP = one of three equal-tier output channels" structural framing (see Deferred decisions).
3. **`/andromeda-evolve --allow-route-append`** to extend the route from `docs/v0_2_0/pulse-v0_2_0-route.md` §93 (Export for community training) / §94 (Config hot reload).
4. **Resume the L4 red-dot debug** using the preserved `crates/ingest/examples/inject_demo.rs` + `AI-Model/` setup; first fix the `build.rs` 8 MB tokenizer cap (Tier-3 learning) so L4 works on a fresh build.

## Session Goals (carry-over)

(none — this session's goal completed: wrap the session after the demo + the manual MCP-tools arch amendment. Route 94/94; D1 + D3 cleared; amendment archived.)

## Deferred decisions

1. **`2026-06-01 — arch-body "equal-tier output channel" framing` (carries forward):** the chunk-#94 spec's "MCP is one of three equal-tier output channels, not coupling" is a STRUCTURAL `.andromeda/architecture.md` §Established Decisions body change. Out of scope for `--allow-arch-registry` and for `/implement`. Requires a deliberate `/andromeda-arch` touch OR an explicit arch-body edit + `/andromeda-setup-project --delta` cascade. (P27 proposes fixing the misleading skill redirects that surfaced while doing the chunk-#94 manual amendment.)
2. **Manual `cargo +nightly public-api -p mcp-server` reconcile:** the chunk-#94 mcp-server tool surface (4 TOOL_* consts + IncidentToolContext + dispatch_tool 5-arg + tools_list_with_8_tools) is uncaptured in api-surface.md = R1-accepted per-crate lag. Cursor revisits mcp-server later in cycle-3, but since the route is 94/94 (few future wraps), a manual reconcile when convenient would capture it sooner.
3. **`spec_amendments.archive` pruning:** archive at 77 (> 50 soft-cap); pruning deferred — run-dir markers remain forensic.
4. **Untracked carryover** still in `git status`: `crates/ingest/examples/` (inject_demo.rs debug tool), `experiments/`, `ui/`. Intentional (debug setup for the deferred L4 work).
