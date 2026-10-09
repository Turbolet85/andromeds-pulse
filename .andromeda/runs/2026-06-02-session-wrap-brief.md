# Session wrap brief — 2026-06-02 (READ THIS FIRST at /wrap-session)

This session was NOT a chunk implementation. Two threads. Use this to wrap.

## Committed this session
- **`44af47b` docs(arch): acknowledge chunk #94 MCP tools (8 #[tool] methods)** —
  manual STRUCTURAL arch amendment: `architecture.md` §Stack + §Established
  Decisions [MCP Server Surface] + §Conventions + §Standard Contracts went
  4 → 8 tools (added `query_incident_list` / `retrieve_report` /
  `retrieve_telemetry_slice` / `mark_incident_resolved`). Tier-2/3 cascade:
  CLAUDE.md §Modules (line 30) + `.claude/docs/services/mcp-server.md`.
  Closes the session-169 **D3 drift** (arch-mcp-tools enumeration).
- `main` is ~7 commits ahead of origin (unpushed). User aware.

## Amendment lifecycle — WRAP SHOULD ARCHIVE
- `state.yaml.spec_amendments.active` has 1 entry:
  `2026-06-02T18-13-31-acknowledge-chunk-94-mcp-tools` — **applied + propagated**
  (`propagated_by_run: manual-cascade-2026-06-02`), NOT archived.
  → Wrap Phase 8: move active→archive, set `noted_at` + `archived_at`.
- Marker: `.andromeda/runs/2026-06-02T18-13-31-spec-amendment-acknowledge-chunk-94-mcp-tools/amendment.md`
  (manual provenance; `manual_structural_amendment: true`, NO `flag_used`).
- Refusal audit trail: `.andromeda/runs/2026-06-01T22-25-08-evolve-acknowledge-chunk-94-mcp-tools/refused.md`

## Deferred (NOT done)
- "MCP = one of three equal-tier output channels, not coupling" structural
  framing (session-169 Priority 2). Needs a dedicated arch-body edit later.

## Demo thread (REVERTED this session; debug setup preserved)
- L4 "red dot" chase (2026-06-01→06-02): proved the full
  L0→storm→digest→**L4 GPU inference** loop works on real injected telemetry;
  could NOT produce a clean incident→red-service-dot (3B model dismiss +
  over-cranked tier1 back-pressure). Header connection dot did go red =
  Stalled, not an incident.
- 4 demo source edits REVERTED via `git checkout HEAD` (bootstrap 3600→20 in
  `activity_floor.rs` + `thresholds.rs`; `[DBG]` eprintlns in `appender.rs` +
  `storm.rs`). **Source tree is clean.**
- KEPT (untracked/gitignored, for "debug later"): `crates/ingest/examples/inject_demo.rs`
  (OTLP injector) + `AI-Model/` (b9305 llama-cli + CUDA DLLs + tokenizer.json +
  RESUME-NOTE.md).

## Curation candidates (learnings for wrap to evaluate)
1. **REAL BUG (Tier 2/gotcha worth filing):** `crates/triage/build.rs` caps the
   tokenizer download at 8 MB (`.take(8*1024*1024)`); the Llama-3 tokenizer.json
   is 9.08 MB → silent truncation → digest assembler fails "EOF line 382099" →
   L4 never works on a fresh build. Worked around via
   `ANDROMEDA_LLAMA3_TOKENIZER_PATH` override; proper fix = raise the cap (e.g. 16 MB).
2. **PIPELINE-FRICTION lesson (Tier 1/feedback):** a STRUCTURAL `architecture.md`
   amendment after MVP has NO clean skill path — `/andromeda-evolve` refuses
   (Refuse 1, structural out of `--allow-arch-registry` scope), `/andromeda-arch`
   is greenfield write-once (can't surgically amend; would regenerate), and
   `/andromeda-setup-project --delta` refuses architecture.md amendments unless
   Type 6 flag-authorized. BOTH the evolve and --delta diagnostics stale-redirect
   to `/andromeda-arch` which CANNOT do it. Working path = manual arch edit +
   manual marker + state.yaml entry + manual Tier-2/3 cascade (or full
   setup-project) + wrap archives. Per CLAUDE.md 2026-05-16 after-MVP path +
   session-144/145 precedent. Consider an andromeda-improvements proposal:
   fix the two stale `/andromeda-arch` redirects to point at the manual path.
3. **L4 red-dot domain finding:** cue→incident→red-service-dot is gated by
   real-telemetry signal conditions (1h `BOOTSTRAP_WINDOW_SECONDS`,
   baseline-relative EWMA spikes, exception-fingerprint storm count) — synthetic
   injection fights all three; storm path (exception events → RetryStorm cue) is
   most deterministic but L4 still judges the digest.

## State-detection notes for wrap
- last_completed_chunk stays **94**; route 94/94. No chunk implemented.
- Drift: **D3 (arch-mcp-tools) now RESOLVED** by 44af47b — re-derive as cleared.
  D1 (api-surface mcp-server stale, environmental disk-full from session 169) —
  disk now has headroom (cargo clean freed ~64 GB this session), reconcile can run.
- Living artifacts: no code landed (demo reverted) — reconcile as normal.
- Disk: was full during session 169; `cargo clean` (64 GB) + zip removal freed it;
  D: now ~178 GB free.
