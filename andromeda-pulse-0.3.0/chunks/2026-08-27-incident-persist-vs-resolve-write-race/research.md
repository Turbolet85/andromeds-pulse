# Codebase Research — 2026-08-27-incident-persist-vs-resolve-write-race

## Scope
- **Depth:** deep · **Reads:** 13 files · **Graph queries:** 5 (rust plane, `db_state` warm) · **Globs/Greps:** 9

## Headline — the scope's two-writer picture is INCOMPLETE (7 production writers, one cross-process)

The working entry and `scope.md` describe a race between **two** writers. The impact query returns
**seven production call sites** reaching `update_incident_status`, and the seventh lives in a
**different process**. Everything the scope said about the named pair held; what it under-counted is
the exposure — which changes where a guard must sit.

| # | Site | Shape | Registry-sourced? |
|---|---|---|---|
| 1 | `run_incident_persist_cycle` — `crates/triage/src/incident/persistence.rs:170` | **stale snapshot**, per-row loop | yes — snapshot held for the whole write span |
| 2 | `AutoResolveObserver::run_one_tick` — `pulse-app/src/incident_observer.rs:73` | direct, at the resolve moment | fresh (`mark_resolved` return) |
| 3 | `IncidentsApiImpl::acknowledge` — `pulse-app/src/incidents_router.rs:238` | direct | fresh (`acknowledge` return) |
| 4 | `IncidentsApiImpl::mark_resolved` — `pulse-app/src/incidents_router.rs:309` | direct | fresh (`mark_resolved` return) |
| 5 | `attach_resolution_summary_to_incident` — `pulse-app/src/inference_runtime.rs:638` | direct, **writes a RESOLVED row** | fresh (`attach_resolution_summary` return) |
| 6 | `create_incident_from_l4_output` dedupe branch — `pulse-app/src/inference_runtime.rs:825` | direct | fresh (`registry.get`) |
| 7 | `dispatch_mark_incident_resolved` — `crates/mcp-server/src/tools.rs:461` | direct, **CROSS-PROCESS**, calls `CorpusWriter::update_incident_status` and never touches `IncidentPersistence` | **no registry at all** — decodes the row, mutates, re-encodes |

**Consequence for placement.** Sites 1–6 share the `triage::IncidentPersistence` trait; site 7 does not.
A guard installed on the triage trait covers six of seven and leaves the agent-facing one uncovered.
The ONLY choke point all seven traverse is **`CorpusWriter::update_incident_status`**
(`crates/corpus/src/contract.rs:603`). `crate_edges` confirms the placement costs nothing:
inbound edges to `corpus` are already exactly `mcp-server → corpus` and `pulse-app → corpus`, so a
corpus-layer guard adds **no new dependency edge** (satisfies arch §Module dependency direction).

## Second finding — the MCP path is a DETERMINISTIC loss, not a race

`IncidentPersistence::load_active_incidents` has exactly **one** production caller —
`pulse-app/src/main.rs:750`, i.e. **boot restore only** (the other three callers are integration
tests). The in-memory registry is therefore never re-read from the corpus during a session.

So when an agent calls MCP `mark_incident_resolved`, the sidecar writes `status='resolved'` to the
corpus while the app's registry still holds that incident **Active** — and the app's next persist
cycle (≤60 s) lists it as active and writes it back. This is not a timing race: it fires on **every**
MCP-driven resolution while the app is running, with a bounded ≤60 s delay. Same defect class, same
row, same statement — reached by a path the scope did not name.

## Graph impact (rust plane; trace at `{run_dir}/tree-query-{marker}.json`)

- **`update_incident_status`** — 11 `calls` rows / 11 `refs` rows: 7 production + 2 `pulse-app` integration
  tests (`unit_incident_persistence.rs:276,298`) + 1 corpus in-crate test (`contract.rs:1271`) + the
  adapter impl (`pulse-app/src/incident_persistence.rs:64`). The corpus in-crate pin exists and is
  `load_incident_by_id_returns_resolved_row` — answering tests' open question.
- **`LifecyclePersistence#save_all`** — **1** production caller (`run_lifecycle_persist_cycle:111`).
- **`StormPersistence#save`** — **1** production caller (`run_storm_persist_cycle:173`) + 2 test impls.
- **`IncidentPersistence#load_active_incidents`** — 1 production caller (`main.rs:750`) + 3 tests.
- **`crate_edges` on `corpus`** — inbound `mcp-server`, `pulse-app`; no outbound.

## Sibling-violator sweep — ANSWERED: no violator (structural, not incidental)

Both sibling loops are **whole-state single-call saves**, not per-row list-then-write loops, and each
has exactly one writer:
- `run_lifecycle_persist_cycle` (`crates/triage/src/lifecycle/persistence.rs:99-111`) → one
  `persistence.save_all(&entries)`.
- `run_storm_persist_cycle` (`crates/triage/src/pattern/persistence.rs:161-173`) → one
  `persistence.save(&snapshot)`.

The defect requires two writers to the **same row** with one holding a stale snapshot. Neither sibling
has a competing direct writer (graph-confirmed above), and neither writes per-row. **The sweep is a
NO-OP with a stated reason** — the `2026-08-26-ingest-consumer-stall-under-sustained-load` precedent
the scope cited, and no edit should be invented to satisfy the step.

## Patterns detected

- **Every registry mutator bumps `updated_at_unix_nano` to `now`** — `mark_resolved`
  (`registry.rs:266`), `mark_read` (`:278`), `attach_resolution_summary` (`:343`),
  `attach_interpretation_summary` (`:359`). This is what makes a **monotonic** predicate viable: the
  stale snapshot necessarily carries a *smaller* `updated_at` than the write it would clobber.
- **A blanket `status != 'resolved'` guard would BREAK a legitimate write.**
  `attach_resolution_summary` (`registry.rs:336-338`) *requires* `status == Resolved` and is
  documented as "the resolution-summary path stays the final write" — site 5 writes a resolved row by
  design. Any status-only predicate must therefore carve it out; a monotonic predicate does not need to.
- **`status` and `updated_unix_nano` are plaintext scalar columns** (`crates/corpus/src/schema.rs:65-74`
  — only `payload BLOB` is encrypted), and `load_active_incidents` already filters on
  `status != 'resolved'` (`contract.rs:645`). A SQL predicate on either column is expressible **without**
  demoting any encrypted cell — closing security's constraint 3 and arch's at-rest constraint.
- **The `rows == 0` arm is the current sole error path** — `contract.rs:618-620` returns
  `Error::QueryFailed`, which `pulse-app/src/incident_persistence.rs:73` maps to
  `IncidentError::NotFound`, which five call sites log as `triage.incident.persist.error` WARN.
  arch, security and obs converged independently on this: a guarded UPDATE turns a zero-row result
  into a *legitimate no-op*, so this arm must be re-interpreted rather than left as a fault.
- **The determinism seam already exists** — `run_incident_persist_cycle` is a **synchronous `pub fn`**
  taking `&dyn IncidentRegistry` + `&dyn IncidentPersistence` (`persistence.rs:160-164`), and
  `AutoResolveObserver::run_one_tick(now_unix_nano)` is pure w.r.t. time
  (`incident_observer.rs:57`). The collision can be reproduced by **interleaving the two calls
  directly in one test** — no `tokio::time`, no interval, no `sleep`. This satisfies tests' ban on
  sleep-based synchronization without inventing a seam.

## Conventions to follow

- **Bound parameters only** — the existing statement is fully parameterized
  (`contract.rs:606-610`, `?1..?5`); any added predicate follows as `?6` (security §Input Validation).
- **Cross-crate surface via the contract module** — both the statement and the `IncidentPersistence`
  trait already live in their crates' `contract.rs` (`corpus/src/contract.rs:603`,
  `triage/src/incident/persistence.rs:109`).
- **Obs leaves are exact and already correct for the VERIFY leg** —
  `incidents.list_active.request` → `["item_count"]` (`pulse-app/src/observability.rs:2321-2322`,
  emitted at `incidents_router.rs:221-226`) and `triage.incident.persist` →
  `["incident_count","persist_kind","duration_ms"]` (`observability.rs:2325-2329`, emitted at
  `persistence.rs:172-179`). Both leaves match their emit sites field-for-field, so **no allowlist
  edit is needed for the VERIFY leg**.
- **`triage.incident.auto_resolve.tick` resolves to NO allowlist entry** (0 occurrences in
  `observability.rs`) — confirming the obs extract. It is owned by the *Diagnostics un-muting* entry;
  if this chunk's measurement needs `resolved_count`, that is a scope question, not a free edit.

## Files to modify (provisional until the P4 shape is chosen)

- `crates/corpus/src/contract.rs` — the `CorpusWriter::update_incident_status` statement + its trait
  signature if the predicate needs a parameter; the one choke point covering all seven writers.
- `crates/triage/src/incident/persistence.rs` — the `IncidentPersistence` trait doc + whatever the
  refused-write outcome becomes; the per-cycle observable folds here (obs bans per-row emission).
- `pulse-app/src/incident_persistence.rs` — the adapter's `CorpusError → IncidentError` mapping, since
  `QueryFailed → NotFound` currently conflates "row missing" with "write declined".
- **Caller threading (from the impact query, not from memory)** — every site whose error arm changes
  meaning: `pulse-app/src/incident_observer.rs:73` · `pulse-app/src/incidents_router.rs:238,309` ·
  `pulse-app/src/inference_runtime.rs:638,825` · `crates/mcp-server/src/tools.rs:461`.
- **Test companions that PIN the changed artifact** — `pulse-app/tests/unit_incident_persistence.rs`
  (`:276,:298` call the changed method) and `crates/corpus/src/contract.rs`'s in-crate
  `load_incident_by_id_returns_resolved_row` (`:1271`).
- `pulse-app/src/observability.rs` + `pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs`
  — **only if** the chosen shape emits a new field/target (obs §8 exact-leaf rule; guard must live
  under `pulse-app/tests/` because `[lib] test = false`).

## New files to create
- None expected. A new integration test file under `pulse-app/tests/` is likely for the interleaved
  RED/GREEN pin; the exact name is a P4 call.

## Open questions
- **Which remedy shape, and where does the guard sit?** Research narrows the two candidates the scope
  left open to one recommended form (monotonic `updated_unix_nano` predicate at the corpus choke
  point) and shows a status-only predicate breaks site 5 while a triage-layer placement misses site 7.
  The remaining trade-off — a guarded UPDATE silently declines writes, so the decline needs an
  observable, versus delta-persistence which restructures the cycle — is a genuine operator call.
  → blocks: **plan-decision** (P4 resolves before synthesis).

## Scope premise closure (all three `[inferred]` bullets closed; scope.md amended)
1. **Boot-restore re-hydration** → **VERIFIED**, and sharpened: `main.rs:750` is the sole production
   restore caller, so the registry never re-reads mid-session. The follow-on ("exposed to the same race
   a second time") holds — a restored zombie carries a stale `updated_at`, resolves on the next 30 s
   tick, and meets the identical alignment; it is **not** self-healing by construction.
2. **RED-first arm** → **VERIFIED**, with the seam named (sync `run_incident_persist_cycle` +
   time-pure `run_one_tick`, interleaved in-process; no sleep, no interval).
3. **Sibling-violator sweep** → **VERIFIED as a research question and ANSWERED: no violator.**
   Recorded above with the structural reason.
