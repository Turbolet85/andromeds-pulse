# /andromeda-evolve — REFUSED (audit trail)

- **Timestamp:** 2026-06-01T22-25-08
- **Flag:** `--allow-arch-registry`
- **Slug:** acknowledge-chunk-94-mcp-tools
- **Halt phase:** Phase 2 (refuse check, before classification)
- **Matched category:** Refuse 1 — Architecture structural body modification

## User intent (verbatim)

> acknowledge chunk #94's 4 new MCP #[tool] methods in arch §Stack +
> §Standard Contracts + §Established Decisions [MCP Server Surface].
> Closes the D3 drift; matches the handoff's Priority 1.

(The 4 new tools: `query_incident_list` / `retrieve_report` /
`retrieve_telemetry_slice` / `mark_incident_resolved`.)

## Phase 0 grounding

- `state.yaml` schema_version=2; `spec_amendments.active` empty.
- `drift_warnings` includes D3 (arch enumerates 4 MCP tools; chunk #94 added 4 more) — flagged EXPECTED chunk-then-amendment by the session-169 wrap.

## Why refused (Refuse 1 — structural, flag does not apply)

The `--allow-arch-registry` flag permits additions to **registry** sections
only (§Occupied Resources / §Workspace / §Capability Registry). Per
`validation-checks.md` Check 7.2, **§Established Decisions / §Stack /
§Project Intent / §Cross-cutting Patterns** and any decision-text/narrative
section are **disallowed even with the flag**.

Grep of `.andromeda/architecture.md` confirms the 4 MCP tool names are
enumerated ONLY in structural sections:
- line 25 — §Stack table (MCP server row)
- line 53 — §Established Decisions [MCP Server Surface]
- line 75 — §Conventions (External wire — MCP server)
- line 142 — §Standard Contracts (MCP server — spec-conformant)

The single registry-style entry — §Occupied Resources "MCP stdio surface"
(line 181) — does **not** enumerate tool names (only `initialize` /
`tools/list` / `tools/call` / `notifications/*`). There is no registry list
for the flag to append to.

Additionally: acknowledging 4 NEW tools means **modifying** the existing
4-tool enumeration (fails Check 7.1 purely-additive), and the session-169
handoff itself flagged a NEW architectural concept ("MCP = one of three
equal-tier output channels") that belongs with this change (fails Check 7.4
no-new-concept).

## Redirect

Architecture structural-body changes require **`/andromeda-arch`** (re-plan
touch), per spec-amendment-protocol.md Part D Architecture exception.

A single `/andromeda-arch` touch handles BOTH:
1. The 4-tool enumeration update across §Stack / §Established Decisions /
   §Standard Contracts / §Conventions (this request — handoff "Priority 1");
2. The structural "MCP = one of three equal-tier output channels, not
   coupling" framing (handoff "Priority 2" / Deferred decision).

The session-169 handoff's split (Priority 1 = registry flag; Priority 2 =
arch touch) was mis-scoped: the 4 tools live in structural sections, so
Priority 1 is also `/andromeda-arch` territory, not `--allow-arch-registry`.

No artifacts written. Project state unchanged.
