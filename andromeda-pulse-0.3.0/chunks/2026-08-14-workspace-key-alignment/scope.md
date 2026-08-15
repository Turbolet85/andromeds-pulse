# Scope — 2026-08-14-workspace-key-alignment

**Version:** andromeda-pulse-0.3.0 · **Epoch 4 — Polish & ship: verification**
**Promoted:** 2026-08-14 (out-of-order by operator directive — see §Order below)

**Working entry:** _Workspace-key alignment — the MCP sidecar reads incidents by the same workspace key
derivation the app writes them under (operator-directed 2026-08-14; evidence: Conductor
two-launch-verdict.md §Re-run) · FIRST STEP: incidents are now locally creatable, so the mismatch is
directly observable — a `query_incident_list` read against a data dir holding incidents; and one
Conductor arm re-run under the new `buffer.tick` feed counters names the arm-zero failure class on
sight (`span_events_seen` / `fingerprints_computed` / `observer_invocations` discriminate where the
path stops)_

---

## Order — why this chunk, not the first markerless one

The first markerless entry is **Baseline-family reachability**. The operator took this piece up out of
order after the `2026-08-14-fingerprint-feed-capture-repair` capture: that chunk's discovery (the storm
path is healthy at HEAD; storm→cue→incident flows in seconds on a fresh dir) flipped the value order.
Workspace-key alignment now directly unblocks Conductor's preflight-green path and owns **both** open
mysteries; Baseline-family reachability serves Epoch-3 family proofs and waits. The route line for
Baseline-family reachability stays markerless and mutable.

## The defect (verified, not inferred)

Two processes derive the incident workspace key differently, so the sidecar filters on a key no
incident was ever stamped with.

- **App side** — `pulse-app/src/main.rs:688-693` calls
  `pulse_app::digest_runtime::resolve_workspace_for_incidents(detected.as_ref(), &data_dir)`
  (`pulse-app/src/digest_runtime.rs:109`). With detection succeeding it returns
  `ctx.root.to_string_lossy()` — the **detected project root**, canonicalized by
  `workspace_detector::detect::detect` (`crates/workspace-detector/src/detect.rs:31` calls
  `candidate_root.canonicalize()`). Only on detection failure does it fall back to `data_dir`. This
  single-sourcing landed at `2026-07-05-constellation-severity-live-wiring` (P-079) so the FILTER key
  equals the STAMPED `digest.workspace`.
- **Sidecar side** — `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:74` sets
  `workspace_root = data_dir.to_string_lossy().to_string()` **unconditionally**. Its comment
  (lines 70-73) claims it "matches the main process's workspace key derivation
  (`incident_workspace_key = data_dir.to_string_lossy()` at pulse-app/src/main.rs)" — a claim that was
  true **before** P-079 and is stale now. The key flows into `IncidentToolContext.workspace_root`
  (`crates/mcp-server/src/tools.rs:73`) and is consumed by
  `.load_active_incidents(&ctx.workspace_root)` (`tools.rs:340`).

Consequence: `query_incident_list` returns zero rows against a corpus that holds active incidents.

## The structural constraint (verified — closes the naive path)

`pulse-app/Cargo.toml` declares `[dependencies.mcp-server-crate] package = "mcp-server" … optional = true`
— **pulse-app depends on mcp-server**. The sidecar importing
`pulse_app::digest_runtime::resolve_workspace_for_incidents` would therefore be a dependency **cycle**.
Single-sourcing must move the derivation DOWN to a crate both ends reach.

- `crates/mcp-server/Cargo.toml` depends on: `buffer`, `viz`, `snapshot`, `corpus`, `triage`,
  `interpretation` (+ optional `rmcp`) — **not** `workspace-detector`, **not** `pulse-app`.
- `crates/workspace-detector/Cargo.toml` depends on no sibling workspace crate (`thiserror`, `serde`,
  `strict-path`, `tracing` only) — it is already the shared detection home, so it is the natural
  candidate and adding an edge to it from `mcp-server` introduces no cycle.

**The exact home is research's design call, not pre-decided here.** P3 recorded the seam facts and they
clear the candidate: the code-graph shows `workspace-detector` has **zero outbound** workspace edges (a
leaf; inbound only from `pulse-app` and `ui-bridge`) and `mcp-server` has **no edge either way** with it
today, so `mcp-server → workspace-detector` cannot form a cycle. `WorkspaceContext` is already `pub`
through that crate's contract module and the dependency would be normal (not dev-only), so it can back a
shipped signature. P4 picks the home on those facts.

## What this chunk delivers

1. **OBSERVE before fixing.** Drive incidents into a data dir locally (they are creatable now — the
   storm path is healthy at HEAD per the prior chunk's capture), then read `query_incident_list`
   against that dir and record the zero-vs-nonzero result. This is the **before-evidence** the fix's
   acceptance compares against: the mismatch has only ever been DERIVED from code reading (intent F10),
   never observed. `crates/ingest/examples/inject_demo.rs` exists at that path and its own header
   documents the timing: ~10s healthy warmup, then ~10 error spans/s sharing one fixed
   `exception.type`/`.stacktrace` fingerprint crosses the Autonomous storm threshold "within ~1s" →
   cue → digest → L4 → incident. The prior chunk's live capture measured 10 incidents created, so the
   ~30s order holds.
2. **One derivation, one runner, both ends agree.**
   **[premise-corrected at P5 val-1: "both processes call it" cannot work under the target topology]** —
   the derivation does move down into a crate both `pulse-app` and `mcp-server` reach, but only the app
   RUNS it. Conductor spawns the sidecar with `ANDROMEDA_PULSE_DATA_DIR` propagated and **cwd NOT set**,
   so a sidecar calling `detect(current_dir())` would derive *Conductor's* repo root — a different wrong
   key, leaving the arm blocked. Instead the app publishes its resolved key under the data dir (the one
   value both sides already agree on, because Conductor's contract mandates propagating it) and the
   sidecar reads it, falling back to `data_dir` when absent. Operator-selected at P4. The sidecar's
   stale parity comment goes either way.
3. **Normalization proven, not assumed.** The app's key passes through `canonicalize()` (Windows
   returns a `\\?\` verbatim-prefixed path); the sidecar's `data_dir` is a raw env-derived string. A
   unit MUST prove the two entry forms (canonicalized vs raw) resolve to the **same** key — or the
   derivation normalizes explicitly. This closes the probe's open question **by construction** rather
   than by observation on one host.
4. **Arm-zero named.** One Conductor arm re-run under the `buffer.tick` feed counters added by
   `2026-08-14-fingerprint-feed-capture-repair` classifies the second open mystery on sight:
   `span_events_seen=0` → nothing reached the appender; `fingerprints_computed=0` with
   `span_events_seen>0` → attributes lost in ingest; `observer_invocations=0` with
   `fingerprints_computed>0` → wiring. This leg needs the operator's headful half — schedule it with
   them rather than assuming it can run unattended. (Conductor is locally reachable at
   `D:\dev\projects\conductor`; the recipe is read-only reference material there.)

## Boundaries

- **Conductor is an INSTRUMENT here, not a session.** Run its preflight binary from its own repo root
  (`D:\dev\projects\conductor` — reachable locally); launch `pulse-app` from OUTSIDE that repo; land
  **all** evidence in THIS chunk's folder. **Never write into Conductor's tree** — its own wraps would
  meet unattributable dirt. The six-item live-leg recipe on Conductor's capture-entry CARRY is a
  **read-only** reference.
- Scope is the workspace-key derivation and its two consumers. Not in scope: the baseline-family
  reachability gate (its own markerless entry), the P-075 Conductor e2e closure (a later entry), or
  any change to how incidents are stamped by the producer.
- No new TauRPC procedure is anticipated — verified: `IncidentToolContext.workspace_root` has exactly
  ONE production consumer (`crates/mcp-server/src/tools.rs:340`), and the app side threads the key
  through existing constructors only. No IPC surface is involved.
- **[premise-corrected: the app's key is `canonicalize()`'s output kept verbatim, locked by
  `resolver_preserves_windows_extended_length_prefix`]** — the §What this chunk delivers item 3 framing
  "or the derivation normalizes explicitly" must NOT become a downstream re-canonicalization: `detect()`
  canonicalizes exactly once and the key is that output verbatim; re-canonicalizing on either side is
  the documented divergence trap the P-079 test guards. The normalization the acceptance proves is that
  two *entry forms* of the same directory (raw `C:\dev\x` and verbatim `\\?\C:\dev\x`) reach one key
  through that single `canonicalize()`.
- **[premise-corrected: the fallback branch already agrees; only the detected branch diverges]** — both
  `resolve_data_dir()` copies (`pulse-app/src/main.rs:181` and
  `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:27`) are byte-identical raw `PathBuf::from(env)`, so
  on detection failure the two processes already produce the same key. The defect is confined to the
  detected-root branch, where the sidecar never calls `detect` at all.

## Surfaces and contracts touched

| Surface | Site |
|---|---|
| App key resolution | `pulse-app/src/digest_runtime.rs::resolve_workspace_for_incidents` · `pulse-app/src/main.rs:688` |
| Sidecar key resolution | `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs::init_incident_context` |
| Sidecar consumer | `crates/mcp-server/src/tools.rs::IncidentToolContext.workspace_root` · `tools.rs:340` |
| Detection primitive | `crates/workspace-detector/src/detect.rs` · `contract.rs::WorkspaceContext.root` |
| Corpus filter | `CorpusWriter::load_active_incidents(workspace_key)` |
| Existing regression guard | `pulse-app/tests/integration_constellation_severity_workspace_key.rs` (P-079) |

## Evidence lineage

- `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` §Re-run
  (read-only, external).
- The prior chunk's capture: `span_events_seen=936 / fingerprints_computed=936 /
  observer_invocations=936`, `storms_detected_total` 0→2, 10 incidents created — the reason incidents
  are locally creatable and the observation leg is possible at all.
