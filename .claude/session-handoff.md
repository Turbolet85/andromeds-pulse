# Session Handoff

**Last Updated:** 2026-06-01T17:25:00Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** `<session 169 chunk #94 implement commit — pending this turn>` (prior HEAD: `00010f3` chore(wrap): session 168 — chunk #94 route-append META wrap)

## Current State

- **Last completed chunk:** route#94 "MCP server + tool exposure" (Epoch 9 — Foundation v0.2.0; commit `pending` this wrap — advances from #93; State-H auto-heals next wrap per Proposal 16 Option b). **TERMINAL chunk of the 94-chunk route — route now 94/94 implemented.**
- **Next chunk:** none registered. Epoch 9 + the v0.2.0 route are complete. Next forward work is either a deliberate `/andromeda-arch` touch (see Deferred decisions) OR a new chunk via `/andromeda-evolve --allow-route-append` from `docs/v0_2_0/pulse-v0_2_0-route.md` §93 (Export for community training) / §94 (Config hot reload) if the user wants to extend.
- **In-progress phase:** none (phase-91 plan implemented this session).
- **Phase artifacts present:** `.andromeda/phases/phase-91/` (chunk #94 plan/combined/research, implemented).

## Andromeda State Detection (states A-K)

11 of 11 effectively CLEAR (B = N/A; H advances 93→94 this wrap with commit_sha=pending = expected post-wrap state per Proposal 16).

- **A — In-progress runs:** ✓ CLEAR — phase-91 complete; the chunk #94 route-append run-dir markers from session 168 are archived.
- **B — Status drift:** N/A (no project.yaml).
- **C — Architecture staleness:** ✓ CLEAR — arch.md (2026-05-27) < CLAUDE.md (2026-06-01, Tier-1 edit this wrap).
- **D — Pending route:** ✓ CLEAR — route.md present, 94 chunks, all implemented.
- **E — Pending phase planning:** ✓ CLEAR — no unplanned registered chunk (route complete).
- **F — Pending implementation:** ✓ CLEAR — chunk #94 implemented this session.
- **G — Multiple concurrent runs:** ✓ CLEAR.
- **H — Route chunk drift:** ℹ️ info (expected) — last_completed advances 93 → 94 this wrap; commit_sha=pending (Proposal 16 Option b — next wrap Phase 8 step 7 auto-heals to the HEAD-reachable implement commit SHA).
- **I — Specialist plan freshness mismatch:** ✓ CLEAR — no specialist plan edited this session (pure /implement).
- **J — Living artifact staleness:** ⚠️ partial — dep-tree.md reconciled (468 lines, real +5 delta); api-surface.md reconcile FAILED (environmental disk-full; reconcile_failed=true; mcp-server sub-block content preserved-but-stale, catches up next wrap). Not a code fault.
- **K — Multi-chunk in-progress imbalance:** ✓ CLEAR — in_progress null.

## Drift Detection (6 dimensions)

`state.yaml.drift_warnings` = [D1-apisurface-staleness (environmental), D3-arch-mcp-tools (expected chunk-then-amendment)].

- **D1 — Living artifact staleness:** ⚠️ api-surface.md only — reconcile_failed=true (cargo +nightly public-api -p mcp-server failed: libduckdb-sys build-script exit 1, disk 100% full / 675 MB free; nightly target cache needs a cold libduckdb-sys compile). Environmental, not code. dep-tree.md CLEAR (reconciled this wrap). Remediation: free disk + re-run /wrap-session (or it self-clears when the cursor revisits mcp-server with disk headroom).
- **D2 — Living artifact wrong content:** ✓ CLEAR — dep-tree fresh-stdout == LIVING block post-replace.
- **D3 — Plan-to-code drift:** ⚠️ FIRES (EXPECTED chunk-then-amendment) — arch §Stack + §Standard Contracts + §Established Decisions [MCP Server Surface] enumerate 4 MCP `#[tool]` methods; chunk #94 added 4 more (`query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved`). ALSO the structural "MCP = one of three equal-tier output channels, not coupling" §Established Decisions framing is NOT yet in arch. Remediation: (a) `/andromeda-evolve --allow-arch-registry` to acknowledge the 4 new tools in §Established Decisions [MCP Server Surface] + §Stack + §Standard Contracts enumeration (mirrors chunks #78/#82/#86/#87/#88 Type 6 precedent); (b) a deliberate `/andromeda-arch` touch for the structural equal-tier-channel framing (out of scope for registry-only flow). NOTE: capability-drift is CLEAN — `mcp.status` DTO extension changed no TauRPC procedure set, so no TauRPC↔capability-JSON drift.
- **D4 — Plan-to-plan drift:** ✓ CLEAR.
- **D5 — Plan-to-CLAUDE.md drift:** ✓ CLEAR — CLAUDE.md mtime (this wrap) > all upstream mtimes.
- **D6 — Route chunk progression:** ℹ️ self-clears — git log shows no `chunk(94): implement` yet (this wrap creates it); state.yaml advances 93→94 this wrap. Self-consistent post-commit.

## Spec Amendments (this session)

(none this session) — this was a pure `/andromeda-implement` of chunk #94; no specialist-plan or arch amendment was applied. `spec_amendments.active` empty (chunk #94 route-append amendment was archived in session 168). The D3 arch-registry acknowledgment for the 4 new MCP tools is a FUTURE evolve cycle, not applied here.

## Key Decisions This Session

1. **Option A "Corpus-backed subprocess" (user-approved at /phase):** the MCP sidecar stays a separate process; the 4 new incident/report/telemetry tools read/write the on-disk `corpus/corpus.db` cross-process (the only shared substrate between the main app and the stdio sidecar). Accepted consequences: `retrieve_telemetry_slice` returns persisted incident-context (evidence refs), NOT live telemetry rows; `mark_incident_resolved` is eventually-consistent vs the in-memory registry (durable in corpus; re-hydrated at boot); `mcp.status()` "connected" = sidecar-running. Options B (in-process pivot, arch-gated) + C (defer) NOT chosen.
2. **P-038 single-source via assemble_report move:** moved the ~135-line `assemble_report` glue from `pulse-app/src/incidents_router.rs` into `interpretation::markdown` (where `serialize_report` already lived). The MCP `retrieve_report` tool + `incidents.get_report` now project identical Reports; a dedicated test asserts byte-identity. Only new dep edge: `interpretation → security` (leaf crate, acyclic).
3. **HYBRID-RENDER INVERSE (curated Tier 1 this wrap):** 4 planned "add new" steps collapsed to no-ops because the capabilities already existed — `Settings.mcp_server_enabled` (chunk #38) + the SettingsModalForm Switch + the on-demand-only sidecar (no auto-spawn → default-off holds by construction, P-040). Reused the existing field (zero ui-bridge/contract.rs delta), did NOT touch main.rs, reported the no-ops as scope reconciliations.
4. **Skipped redundant `security_mcp_incident_tools.rs`:** response-body redaction is already covered by the existing `security_mcp_response_body_redaction.rs` + the uniform `mcp.*` AllowList exclusion of `result_content`; the command-injection ban is satisfied by-construction (`mark_incident_resolved` is a pure corpus write, no `Command::arg`).

## Files Modified

**Modified (15):** `Cargo.lock`, `crates/corpus/src/contract.rs` (load_incident_by_id), `crates/interpretation/Cargo.toml` (+security dep), `crates/interpretation/src/markdown.rs` (+assemble_report), `crates/mcp-server/Cargo.toml` (+corpus/triage/interpretation/bincode), `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (corpus open + ctx), `crates/mcp-server/src/jsonrpc.rs` (tools_list_with_8_tools), `crates/mcp-server/src/tools.rs` (4 dispatch fns + IncidentToolContext), `crates/mcp-server/src/tracing_setup.rs` (AllowList +3 fields), `crates/mcp-server/tests/sidecar_subprocess.rs` (8-tools assert), `pulse-app/src/incidents_router.rs` (call interpretation::markdown::assemble_report), `pulse-app/src/mcp_router.rs` (McpStatusDto.connected_agent), `pulse-app/ui/src/bindings/index.ts` (regen), `pulse-app/ui/src/report/Report.tsx` + `ReportRenderer.tsx` (SendToAgent wiring).
**New (4):** `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` (P-038 identity), `pulse-app/ui/src/report/SendToAgentButton.tsx` + `.test.tsx`, `pulse-app/ui/src/report/use-mcp-delivery.ts`.
**Wrap (this turn):** CLAUDE.md (Tier 1), `.claude/rules/security.md` (Tier 2), `.andromeda/context/dependency-tree.md` (reconciled), `.andromeda/context/api-surface.md` (deferral note), `.andromeda/state.yaml`, this handoff.

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 1 — HYBRID-RENDER INVERSE (plan assumes "create new" but capability already exists → verify-before-add + report no-op reconciliations).
- **Tier 2 (.claude/rules/security.md):** 1 — cross-process bincode decode-config parity (sidecar decode must match producer encode config; verify at Step 0, else silent decode failure).
- **Tier 3:** 0.
- **Filtered:** 1 deferred (separate-process-MCP-reads-corpus architecture — covered by arch extract + research.md, low standalone novelty); 1 task-specific reject (L4Output test-fixture schema shape).
- **Andromeda pipeline proposals:** 0 (**Mode H — honest healthy**: the /new-session → /phase → /implement → /wrap chain executed as designed; the one disk-full episode is an environmental host constraint already documented across sessions 144/145/153/163/165, not a pipeline gap).

## Pipeline Accumulators

A1 `api_surface_deferral`: IMPLEMENTED steady state preserved (verified_cleared_at_session=135; consecutive_count=0). NOTE this wrap: api-surface per-crate reconcile FAILED (disk-full environmental) — but this is NOT a batch-deferral (the failure is tooling-can't-run, not deferred-to-stay-in-budget), so consecutive_count stays 0 and api_surface_deferred stays false per per-crate-mode semantics; the mcp-server sub-block catches up next wrap. A2 dormant. 0 refactors / 0 patches filed (Mode H).

## Last Failed Command

`cargo +nightly public-api --simplified -p mcp-server` — FAILED (libduckdb-sys build-script exit 1; root cause: disk 100% full / 675 MB free; nightly target cache needs a cold libduckdb-sys compile with no space). **Alternative (do NOT retry as-is):** free disk first (e.g., `cargo clean` recovers ~180 GB, or remove `target/debug/incremental`), THEN the next /wrap-session per-crate reconcile picks up mcp-server cleanly. The api-surface mcp-server sub-block is preserved-but-stale until then.

## Tests Status

**PASS — green per /implement gate set (run during this session's /andromeda-implement):**
- `cargo nextest run --workspace --all-features --profile ci` = **1591/1591 + 1 skip** (was 1559 baseline; +32 new tests: corpus +3, mcp-server incident tools/labels +13, jsonrpc +1, pulse-app e2e +2, plus renames).
- `cargo nextest run --workspace --profile ci` (default-features, P-040 independence) = **1570/1570 + 1 skip**.
- webview: lint clean · typecheck clean · vitest **613/613** (+8 SendToAgentButton).
- `cargo xtask capability-drift` clean (0 missing, 0 extra; bindings.ts carries `mcp` namespace + `connected_agent`).
- `cargo deny check bans licenses sources` ok · `cargo audit` exit 0 (19 allowed warnings).
- Wrap smoke: `cargo nextest run -p security` = 14/14.
- **Phase 2b boot smoke: SKIPPED (environmental disk-full @ <3 GB; tauri dev build would re-hit no-space).** Boot-path-neutral by inspection — `main.rs` + `ui-bridge/contract.rs` NOT modified; prod `pulse-app.exe` compiled clean twice this session.
- Dead-test scan (P15): source-level `#[cfg(test)] mod tests` blocks in `pulse-app/src/` (binary, test=false) — carryover (incidents_router.rs + mcp_router.rs blocks are pre-existing chunk #49/#88 era; chunk #94 added ZERO new source-level test blocks — its tests live in `pulse-app/tests/`). Warning-not-fatal.

## Next Recommended Action

**Priority 1 — `/andromeda-evolve --allow-arch-registry`** to acknowledge the 4 new MCP `#[tool]` methods (`query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved`) in arch §Established Decisions [MCP Server Surface] + §Stack MCP server row + §Standard Contracts MCP server enumeration (closes the D3 chunk-then-amendment drift; mirrors chunks #78/#82/#86/#87/#88 Type 6 precedent).

**Priority 2 — deliberate `/andromeda-arch` touch** for the structural "MCP = one of three equal-tier output channels, not coupling" §Established Decisions framing (the plan flagged this as out-of-scope for the registry-only flow; carries forward from the session-168 handoff). This is a Deferred decision (below).

**Secondary (not blocking):**
- **Free disk** (`cargo clean` or rm `target/debug/incremental`) so the next wrap's api-surface reconcile picks up the stale mcp-server sub-block + so boot smoke can run.
- `git push origin main` — branch is now **6+ commits ahead** of origin (verify with `git status`).
- `spec_amendments.archive` at 76 (over 50 soft-cap; pruning deferred — run-dir markers remain forensic).
- `experiments/` + `ui/` untracked carryover (still in `git status`).
- Pipeline patches awaiting review in `docs/andromeda-improvements.md`: P22–P26.

## Deferred decisions

`2026-06-01 — Deferred arch-body change (chunk #94 flagged, NOT applied):`
- **Gap:** the chunk spec's "MCP is one of three equal-tier output channels, not coupling" is a STRUCTURAL `.andromeda/architecture.md` §Established Decisions body change (a new framing of the [MCP Server Surface] decision).
- **Why deferred:** out of scope for the registry-only `--allow-arch-registry` flow (which only touches §Occupied Resources / §Architecture Registry Updates), and out of scope for `/andromeda-implement` (cannot edit arch body as a delta). Requires a deliberate `/andromeda-arch` touch OR an explicit arch-body edit + `/andromeda-setup-project --delta` cascade.
- **User decision:** defer to a dedicated arch session. Carries forward from the session-168 handoff (registered there at chunk #94 route-append; still unactioned).

## Session Goals (carry-over)

(none — this session's goal completed: implement chunk #94 "MCP server + tool exposure" end-to-end. Route now 94/94 implemented; Epoch 9 + the v0.2.0 route are functionally complete pending the deferred arch-framing touch.)
