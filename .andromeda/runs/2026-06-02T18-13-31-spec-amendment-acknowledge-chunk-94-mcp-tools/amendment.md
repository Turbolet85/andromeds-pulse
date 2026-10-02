# Spec amendment — acknowledge chunk #94 MCP tools (manual structural arch edit)

Authored manually (NOT via /andromeda-evolve, which correctly refused this as a
Refuse 1 structural-section change; NOT via /andromeda-arch, which is greenfield
write-once). Manual arch-body edit per CLAUDE.md session-learning 2026-05-16
("manual edits to arch.md ... where evolve refuses + /andromeda-setup-project
--delta") + session-144 precedent (mistralrs→llama.cpp structural §Established
Decisions amendment, applied the same way).

## Identity

- **Amendment ID:** 2026-06-02T18-13-31-acknowledge-chunk-94-mcp-tools
- **Trigger:** manual structural amendment (D3 drift closure — arch enumerated 4 MCP `#[tool]` methods; chunk #94 added 4 more). No chunk/phase/harness; user-directed after /andromeda-evolve --allow-arch-registry refused (structural sections out of flag scope).
- **Authority:** user judgment (D3 capability-drift; code reality is canonical — `crates/mcp-server` exposes 8 tools, `jsonrpc.rs::tools_list_with_8_tools`).

## Plans amended

### Plan: architecture.md (STRUCTURAL body — manual edit)
- **Sections:** §Stack (MCP server row, line 25), §Established Decisions [MCP Server Surface] (line 53), §Conventions (External wire — MCP server, line 75), §Standard Contracts (MCP server — spec-conformant, line 142).
- **Decisions Log entry:** (N/A — structural body edit; not a §Decisions Log / §Architecture Registry Updates entry. The latter is reserved for --allow-arch-registry registry-section acknowledgments per its cleanup convention; this is a structural-section edit.)
- **Before → After:** each of the 4 enumerations listed 4 `#[tool]` methods (`query_traces` / `query_metrics` / `query_logs` / `generate_snapshot`) → now lists 8 (adds `query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved`, chunk #94 corpus-backed incident/report tools).

## Implementation files synced

(N/A — this amendment does not modify implementation; chunk #94 already shipped the 8-tool surface. This edit makes arch reflect existing code reality. Code evidence: `crates/mcp-server/src/tools.rs` 4 dispatch fns + `crates/mcp-server/src/jsonrpc.rs::tools_list_with_8_tools` + `crates/mcp-server/tests/sidecar_subprocess.rs` 8-tools assert.)

## Expected downstream propagation

- CLAUDE.md `<!-- GENERATED:setup:modules -->` mcp-server entry — enumerates the 4 tools; should cascade to 8.
- `.claude/docs/services/mcp-server.md` — per-module notes (tool enumeration), if present.
- (CLAUDE.md @imports architecture.md directly, so the imported body reflects the edit automatically; the cascade targets above are the *derived/distilled* mirrors that /andromeda-setup-project --delta regenerates.)

## Lifecycle status

- [x] Applied — 2026-06-02T18:13:31Z (4 arch.md structural edits landed)
- [ ] Noted
- [x] Propagated — 2026-06-02 (manual 2-file mirror cascade: CLAUDE.md :modules line 30 + .claude/docs/services/mcp-server.md tool enum, 4→8; --delta refused this structural amendment, so cascaded manually per session-144/145 pattern)
- [ ] Archived (pending next /andromeda-wrap-session)

## Verification

- **Scope:** specs + Tier 2/3 (NOT impl — code already implements 8 tools).
- **Status:** clean — arch now matches code (8 tools); no orphans. Closes the session-169 D3 drift (arch-mcp-tools enumeration).
- **NOT covered:** the structural "MCP = one of three equal-tier output channels, not coupling" framing (session-169 handoff Priority 2 / Deferred decision) — explicitly OUT of scope per user's "Tools only (mechanical)" choice; remains deferred.

## Cross-references

- Refusal audit trail (why evolve couldn't do this): `.andromeda/runs/2026-06-01T22-25-08-evolve-acknowledge-chunk-94-mcp-tools/refused.md`
- state.yaml: spec_amendments.active entry `2026-06-02T18-13-31-acknowledge-chunk-94-mcp-tools`
