# Codebase Research — 2026-08-14-workspace-key-alignment

## Scope
- **Depth:** deep · **Reads:** 8 · **Globs/Greps:** 12 · **Code-graph queries:** 5 (trace at `.andromeda/runs/2026-08-14T22-35-07Z-phase/tree-query-2026-08-14-workspace-key-alignment.json`)

## Files inspected
- `pulse-app/src/digest_runtime.rs` (80–127) — `resolve_workspace_for_incidents` returns `(key, DigestProjectContext)` from ONE call so filter and stamp are byte-equal. Detected → `ctx.root.to_string_lossy()`; `None` → `data_dir.to_string_lossy()` on **both** halves. No re-canonicalization on either branch.
- `pulse-app/src/main.rs` (181–202, 680–745) — `resolve_data_dir()` returns `PathBuf::from(env::var("ANDROMEDA_PULSE_DATA_DIR"))` **raw, uncanonicalized**; the key is resolved at boot from `std::env::current_dir()` + `workspace_detector::detect::detect`, then cloned into `IncidentsApiImpl`, `ServicesApiImpl`, the boot restore, and the persist path.
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (1–89) — the sidecar's `resolve_data_dir()` is **byte-identical** to the app's (same raw `PathBuf::from(env)`, same per-OS fallbacks). `init_incident_context` sets `workspace_root = data_dir.to_string_lossy()` and **never calls `detect`**. The stale parity comment sits at lines 70–73.
- `crates/mcp-server/src/tools.rs` (60–75, 320–410) — `IncidentToolContext { corpus, workspace_root }`; `workspace_root` has **exactly one production consumer**: `dispatch_query_incident_list` at line 340. `load_previously_seen` filters by `incident.workspace` (the row's stamped value), not by `ctx.workspace_root`.
- `pulse-app/src/mcp_router.rs` (160–213) — the app spawns the sidecar with **only** `.env("ANDROMEDA_PULSE_MCP_ENABLED","true")`; no `.current_dir(...)`, no `ANDROMEDA_PULSE_DATA_DIR` passthrough. The child therefore **inherits the parent's cwd and full environment**.
- `crates/workspace-detector/src/detect.rs` (1–60) — `detect(candidate_root)` rejects `..` traversal, then `candidate_root.canonicalize()`; already `#[instrument(skip_all, fields(detection_latency_ms))]`; its doc states it never logs the full path and the **caller** emits basename-only.
- `pulse-app/tests/integration_constellation_severity_workspace_key.rs` (1–90) — the P-079 guard. `resolver_preserves_windows_extended_length_prefix` **locks** `\\?\C:\dev\payments` as the key verbatim, explicitly because "a re-canonicalized side would diverge".
- `pulse-app/src/observability.rs` (368–380, 1095–1112) — two relevant allowlist entries: module `workspace-detector` permits `workspace.root_path` / `workspace.project_name` / …; leaf `workspace.detect` (chunk #43) permits **basenames only** (`workspace_root_basename`, `vcs_root_basename`).
- `crates/ingest/examples/inject_demo.rs` (1–35) — ~10s healthy warmup, then payment-service emits ~10 error spans/s each carrying a FIXED `exception.type`/`.stacktrace`, so one repeated fingerprint crosses the Autonomous storm threshold "within ~1s" → cue → digest → L4 → incident.

## Graph impact
- **`resolve_workspace_for_incidents`** — 7 call sites: 1 production (`pulse-app/src/main.rs:691`) + 6 in `pulse-app/tests/integration_constellation_severity_workspace_key.rs` (lines 15, 42, 59, 69, 157, 197). Relocating it means threading all 7, and the 6 test callers are the P-079 contract that must keep passing.
- **`load_active_incidents`** — 14 call sites: the sidecar's `tools/dispatch_query_incident_list` at `crates/mcp-server/src/tools.rs:339`, the app's `CorpusIncidentPersistence` impl at `pulse-app/src/incident_persistence.rs:89` and boot restore at `pulse-app/src/main.rs:700`, plus 11 test sites across `e2e_incidents_lifecycle` / `e2e_p043_two_workspace_switch` / `integration_findings_counter_persists_across_restart` / `integration_incident_producer_persists_across_restart` / `unit_incident_persistence`.
- **`crate_edges` for `workspace-detector`** — inbound `pulse-app`, `ui-bridge`; **zero outbound** workspace edges. It is a leaf; adding `mcp-server → workspace-detector` cannot form a cycle.
- **`crate_edges` for `mcp-server`** — outbound to `interpretation`, `snapshot`, `corpus`, `buffer`, `curation`, `triage`, `viz`; inbound from `ui-bridge`. **No edge either way with `workspace-detector` today.**
- **Seam facts** — `workspace-detector` is a NORMAL (non-dev) dependency of `pulse-app` and `ui-bridge`, so a shipped signature may cross that boundary. `mcp-server` would take it as a normal dependency too. `WorkspaceContext` is `pub` in `crates/workspace-detector/src/contract.rs` and re-exported through the crate's contract module — already visible to shipped code, no test-only isolation involved. `mcp-server`'s own `rmcp` dep is `optional` and feature-gated, but `workspace-detector` would be unconditional — it pulls no new transitive deps beyond `thiserror`/`serde`/`strict-path`/`tracing`, all already in the workspace graph.

## Patterns detected
- **Single-call parity** (`pulse-app/src/digest_runtime.rs:109`): returning filter key and stamp context from one call is the P-079 mechanism — parity holds by construction, not by two call sites agreeing. Any relocation must preserve that shape.
- **Deliberate non-re-canonicalization** (`pulse-app/tests/integration_constellation_severity_workspace_key.rs:57-63`): `canonicalize()` runs exactly once inside `detect`; the key is that output verbatim. Re-canonicalizing downstream is the documented divergence trap.
- **Basename-only workspace logging** (`pulse-app/src/observability.rs:1097-1112`): the chunk-#43 `workspace.detect` leaf entry is the standing precedent — a workspace path reaches the log as a basename, never in full.
- **Duplicated `resolve_data_dir`** (`pulse-app/src/main.rs:181` ≡ `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:27`): the two are already byte-identical copies. This is why the **fallback** branch of the key already agrees across processes; only the detected branch diverges.
- **Child inherits cwd** (`pulse-app/src/mcp_router.rs:181`): `Command::new(..).env(..)` without `.current_dir(..)` means an app-spawned sidecar sees the app's cwd, so a cwd-based derivation reproduces the app's key for that launch mode.

## Conventions to follow
- **Test home:** `pulse-app` carries `[lib] test = false`, so any pulse-app-side probe lives in `pulse-app/tests/*.rs` reaching internals via `pub` + `#[doc(hidden)]`; a library crate takes co-located `#[cfg(test)] mod tests` (`pulse-app/tests/integration_constellation_severity_workspace_key.rs:9` states this inline).
- **Contract module surface:** a shared derivation exposed from `workspace-detector` belongs in its `contract.rs` alongside `WorkspaceContext` (`crates/workspace-detector/src/contract.rs:31`).
- **Sidecar stdout is reserved:** all sidecar diagnostics go to stderr-JSON; the existing warn at `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:81` is the shape to copy.
- **Corpus filter is parameter-bound:** `load_active_incidents(&key)` passes the key as a bound argument, never interpolated — preserve that.

## New files to create
- *(none anticipated — the derivation relocates into an existing crate; the normalization unit is a co-located `mod tests` addition if the home is a library crate.)*

## Files to modify
- `crates/workspace-detector/src/contract.rs` (or a sibling module in that crate) — host the shared derivation + its `\\?\`-vs-raw normalization unit.
- `crates/workspace-detector/src/lib.rs` — export it from the contract surface.
- `crates/mcp-server/Cargo.toml` — add the `workspace-detector` path dependency (**manifest rides this list**).
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` — `init_incident_context` calls the shared derivation; delete the stale parity comment (lines 70–73).
- `pulse-app/src/digest_runtime.rs` — `resolve_workspace_for_incidents` delegates to (or is replaced by) the shared derivation.
- `pulse-app/src/main.rs` (~line 691) — the single production caller.
- `pulse-app/tests/integration_constellation_severity_workspace_key.rs` — 6 caller sites (lines 15, 42, 59, 69, 157, 197); the P-079 contract must keep passing unchanged in meaning.
- `pulse-app/src/observability.rs` — allowlist entry ONLY if the chunk emits a new target/field for the resolved key (basename-only per the chunk-#43 precedent).

## Open questions
- **Which launch mode must parity hold for?** An app-spawned sidecar inherits the app's cwd (`pulse-app/src/mcp_router.rs:181`) so a cwd-based shared derivation gives byte-equal keys; an **externally**-launched sidecar (an MCP client spawning the binary directly) uses the client's cwd, which may differ. Options: cwd-based derivation alone (no new resource; arch's anti-pattern bans an incidental new env var), or an explicit key passthrough for the app-spawned mode (needs an arch §Occupied Resources amendment). → blocks: **plan-decision**.
- **Does this chunk owe the obs P7 span set?** Grep for `app.boot.workspace.detect` / `workspace.marker.check` / `workspace.identity.resolve` returns **zero hits** — the P7 target names in obs-plan §4 are not implemented at HEAD, though the `workspace.detect` leaf allowlist (chunk #43) and `detect`'s own `#[instrument]` exist. Satisfying obs' full P7 acceptance contribution would be scope expansion beyond key alignment. → blocks: **plan-decision**.
- **Does the observation leg run as a committed test or as a one-off capture?** Tests' constraint requires a deterministic machine-parseable signal and self-bootstrapped corpus; the before-evidence is by nature a one-time pre-fix measurement that cannot survive the fix. → blocks: **implementation-scope**.

## Resolved by research (no longer open)
- **obs' flagged plan-internal conflict** (§5 Vector 5 "do not log workspace-detector output as-is" vs §4 P7 / §8 naming `workspace.root_path`): the codebase already settled it — the chunk-#43 `workspace.detect` leaf entry permits **basenames only** (`pulse-app/src/observability.rs:1097-1112`). Any new emission in this chunk follows basename-only.
- **The Windows verbatim-prefix acceptance surface:** `canonicalize()` runs once inside `detect` and its output is the key verbatim (locked by `resolver_preserves_windows_extended_length_prefix`). Two entry FORMS of the same directory — raw `C:\dev\x` and verbatim `\\?\C:\dev\x` — both canonicalize to one output, which is exactly the assertion the normalization unit must make.
- **The fallback branch already agrees:** both `resolve_data_dir()` copies are byte-identical raw `PathBuf::from(env)`, so on detection failure the app and sidecar keys are already equal given the same environment. The divergence is confined to the **detected** branch.
