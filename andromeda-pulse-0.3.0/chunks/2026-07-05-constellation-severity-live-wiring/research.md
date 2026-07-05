# Codebase Research — 2026-07-05-constellation-severity-live-wiring

## Scope
- **Depth:** deep (boot-wiring + full producer→incident→filter data-flow traced) · **Reads:** 6 files · **Greps:** 5 · **Code-graph:** skipped (targeted greps + reads already resolved every symbol precisely; the mismatch is a boot-wiring value, not a blast-radius question — noted per cookbook cold-path option).

## The confirmed defect (single cause)
A **workspace-key mismatch** between where incidents are STAMPED and where they are FILTERED. The two sides derive the key from different sources:

**Producer / stamp side (already uses workspace-detector — correct):**
- `pulse-app/src/main.rs:1324-1329` — `std::env::current_dir()` → `workspace_detector::detect::detect(&cwd)` → `.map(digest_runtime::workspace_to_digest_context)` → `DigestProjectContext`.
- `pulse-app/src/digest_runtime.rs:90-98` `workspace_to_digest_context` — `workspace_canonical_path: ctx.root.to_string_lossy().into_owned()`. `ctx.root` is `candidate_root.canonicalize()` (`crates/workspace-detector/src/detect.rs:31`) → on Windows the **`\\?\C:\…` extended-length form**.
- `assemble()` sets `digest.workspace` from the project context (`crates/triage/src/digest/assembler.rs:467` `workspace: workspace.clone()`).
- `pulse-app/src/inference_runtime.rs:686` — `create_incident_from_l4_output` sets `workspace: digest.workspace.clone()`. **⇒ every incident.workspace = canonicalized detected project root.**

**Filter / persist side (the bug):**
- `pulse-app/src/main.rs:686` — `let incident_workspace_key: String = data_dir.to_string_lossy().to_string();` — this is the **data dir** (`%APPDATA%\andromeda-pulse` / `ANDROMEDA_PULSE_DATA_DIR`), a *different directory entirely* (not merely a `\\?\` difference). The stale comment at `main.rs:683-685` even says: *"future chunks integrate workspace-detector for proper per-project keying."* — **this is that chunk.**
- Threaded into: `IncidentsApiImpl::new(…, incident_workspace_key.clone())` (`main.rs:721` → `workspace_root`), `ServicesApiImpl::new(…, incident_workspace_key.clone(), …)` (`main.rs:731` → `workspace_root`), boot restore `load_active_incidents(&incident_workspace_key)` (`main.rs:694`), and the persist loop `vec![incident_workspace_for_persist.clone()]` (`main.rs:1435`, = `incident_workspace_key`).
- The live filter is an **in-memory string equality**, NOT SQL: `crates/triage/src/incident/registry.rs:195-203` `list_active` → `entry.workspace == workspace && entry.status != Resolved`. `services_router.rs:95` + `incidents_router.rs:210` both call `self.…list_active(&self.workspace_root)`.
- ⇒ `list_active(data_dir)` over incidents stamped `detected_root` → **zero rows** → all `priority_tier` fall to healthy, incidents panel empty.
- The corpus SQL path is ALSO broken by the same key: `pulse-app/src/incident_persistence.rs:87` `load_active_incidents(workspace)` + `:135` `count_active_unread(workspace)` query the corpus (rows stored with `incident.workspace` = detected root) using the data_dir key → zero. (So boot-restore + the P-045 unread counter are mis-keyed too — the fix un-breaks all of them together.)

## Files inspected
- `pulse-app/src/main.rs` (655-745, 1310-1354, + greps) — the `incident_workspace_key = data_dir` site (686) and the producer's `current_dir()/detect()` site (1324-1329). The two derivations are ~640 lines apart; `current_dir()` is stable across boot so a single hoisted detection serves both.
- `pulse-app/src/digest_runtime.rs` (full) — `workspace_to_digest_context` (90-98) is the producer's canonical-path derivation; co-locating the key resolver here makes the parity test import both from one module.
- `pulse-app/src/inference_runtime.rs` (626-690) — `create_incident_from_l4_output`; `incident.workspace = digest.workspace.clone()` (686) is the stamp.
- `crates/triage/src/incident/registry.rs` (74, 112-116, 195-203, 263-267) — `list_active` (in-memory `==`) + `mark_all_read` (also workspace-keyed → benefits from the fix).
- `crates/workspace-detector/src/detect.rs` (21-31) + `contract.rs` (26) — `detect()` returns `WorkspaceContext { root: canonicalize(candidate) }`; canonicalization is `std::path::Path::canonicalize` (`\\?\` on Windows).
- `pulse-app/src/incident_persistence.rs` (87-138) — corpus load/count keyed by workspace (same key, same fix).

## Graph impact
- **`incident_workspace_key`** — sole producer at `main.rs:686`; 4 consumers (`main.rs:694,721,731,1008/1435`). Changing its *value* (not type) has zero signature blast radius; all consumers already take `&str`/`String`.
- **`list_active` / `workspace_root`** — resolver fields unchanged; the fix changes the *value* injected at construction, not the resolver code. `services.list_with_states` / `incidents.list_active` DTO wire shapes unchanged (arch extract constraint satisfied).

## Patterns detected
- **Single-source-at-boot** (`main.rs` config-at-boot): both `incident_workspace_key` and the producer `project_context` must derive from ONE `detect(current_dir())` so they are provably equal (the current code calls `detect()` only on the producer side; the filter side never detects).
- **Parity-by-construction** (`digest_runtime.rs:90` `workspace_to_digest_context` ↔ a new key resolver): the key string and `workspace_canonical_path` are the SAME derivation `ctx.root.to_string_lossy()` — co-locate + unit-test their equality.
- **`DigestProjectContext: Default`** — the producer already does `.unwrap_or_default()` (`main.rs:1329`); a fallback context can use `DigestProjectContext { workspace_canonical_path: <fallback>, ..Default::default() }` to keep parity on the detection-failure path.

## Conventions to follow
- Reuse `crates/workspace-detector` output; NO new env var, NO new TauRPC procedure, NO §Occupied Resources entry (arch extract).
- `[lib] test = false` — the proof test MUST live in `pulse-app/tests/*.rs`, not a source `#[cfg(test)] mod tests` (session-learnings 2026-05-20 + 2026-06-04: source mod tests don't RUN under nextest but DO compile under clippy).
- Windows `\\?\` string-equality trap (session-learnings 2026-06-04): parity holds only because BOTH sides use the identical `ctx.root.to_string_lossy()` derivation — do not re-canonicalize one side differently.
- Obs (extract): no raw workspace path in query-boundary logs — `query_id`/`param_count`/`row_count_returned` only; workspace-detector boot span already emits `workspace_root_basename` (observability.rs:1088), keep that discipline.
- Boot-smoke: `main.rs` is touched → the warm re-embed + storm boot smoke is REQUIRED (session-learnings 2026-07-05; testing.md boot-smoke trigger).

## New files to create
- `pulse-app/tests/integration_constellation_severity_workspace_key.rs` — the regression proof (parity + storm→list_active).
- (optional) a small resolver home — recommend adding `resolve_incident_workspace_key(detected: Option<&WorkspaceContext>, data_dir: &Path) -> String` to `pulse-app/src/digest_runtime.rs` (co-located with `workspace_to_digest_context` for the parity test); a dedicated `pulse-app/src/workspace_key.rs` is an acceptable alternative.

## Files to modify
- `pulse-app/src/main.rs` — hoist `detect(current_dir())` to before line 686; derive `incident_workspace_key` from the detected root via the new resolver (fallback data_dir); reuse the SAME hoisted context at 1324-1329 with a parity-preserving fallback (`workspace_canonical_path = incident_workspace_key` on detection failure); update the stale 683-685 comment.
- `pulse-app/src/digest_runtime.rs` — add `resolve_incident_workspace_key` (+ unit tests) next to `workspace_to_digest_context`.

## Open questions
- **Detection-failure fallback value** (resolve at P4, not user-facing): fall back to `data_dir` on BOTH sides (parity preserved; matches prior behavior) vs. empty/default. Recommend data_dir-on-both — guarantees the two keys always agree and no incident is ever silently unfilterable. Decided in-plan; not a scope ambiguity.
