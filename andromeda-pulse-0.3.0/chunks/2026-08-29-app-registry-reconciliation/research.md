# Codebase Research — 2026-08-29-app-registry-reconciliation

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11 · **Graph queries:** 2 (rust plane, `db_state` warm)

## Headline finding

**`IncidentRegistry::list_active` is the single choke point, and both user-visible surfaces already
poll it at ~1 s.** Every consumer of "which incidents are active" reads that one method:

| Consumer | Site | Reaches |
|---|---|---|
| `incidents.list_active` resolver | `pulse-app/src/incidents_router.rs:213` | `use-findings.ts` → findings badge + findings window |
| `services.list_with_states` resolver | `pulse-app/src/services_router.rs:95` (`item.priority_tier = active…max`, `:97-103`) | `ServiceListItem.priority_tier` → `visibleDots` → constellation DOT hue |
| the 60 s persist cycle | `crates/triage/src/incident/persistence.rs` `run_incident_persist_cycle` | the corpus catch-up write |

`pulse-app/ui/src/hooks/use-findings.ts:41` sets `FINDINGS_REPOLL_MS = 1000` and its header records the
posture verbatim: *"PULL with a ~1s background re-poll … A future chunk wiring
`streams.subscribe_incidents` can swap the poll for push-driven invalidation without changing the public
API."* The constellation follows the same ~1 s live-refresh precedent that comment cites.

**Consequence for the plan:** a registry-side reconciliation propagates to BOTH surfaces within ~1 s with
**no frontend change, no new TauRPC procedure, no new `pulse://stream/*` topic and no new broadcast**. The
scope's fork (b) (sidecar → app notification) buys latency the poll already provides, at the cost of the
unregistered cross-process transport arch forbids; fork (c) (make staleness visible) describes a state that
a working fork (a) removes rather than surfaces.

## Files inspected
- `pulse-app/src/main.rs` (735-775, 1496-1520) — boot restore is the SOLE `IncidentPersistence::load_active_incidents` production call; the persist loop is spawned with `vec![incident_workspace_for_persist.clone()]` (exactly one workspace) at `DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS`, and `AutoResolveObserver` is constructed 11 lines later from the same registry + persistence pair.
- `crates/triage/src/incident/persistence.rs` (170-215) — `run_incident_persist_cycle` is a synchronous `pub fn` over `&dyn` deps: `registry.list_active(workspace)` then per-row `update_incident_status`, folding `persisted_count` / `declined_count` and emitting ONCE on `TARGET_INCIDENT_PERSIST`.
- `crates/triage/src/incident/registry.rs` (31-48, 96, 177-260) — `ResolutionTrigger { AutoResolve, ExplicitResolve }`; `IncidentRegistryError { NotFound, InvalidTransition, CooldownActive }`; `from_persisted` is the boot hydration constructor.
- `pulse-app/src/incidents_router.rs` (212-231, 296-350) — the `list_active` resolver and its `item_count` emit; `mark_resolved` drives `registry.mark_resolved(…, ExplicitResolve)` then persists then broadcasts.
- `pulse-app/src/services_router.rs` (18, 55-103) — holds `Arc<dyn IncidentRegistry>` and joins `list_active` onto the lifecycle snapshot by `Incident.scope_id`, taking the MAX `priority_tier` per service.
- `pulse-app/src/incident_observer.rs` (40-100) — `run_one_tick` is pure w.r.t. `now_unix_nano`: evaluate → `mark_resolved(AutoResolve)` → persist → broadcast, returning `(evaluated, resolved)`.
- `crates/corpus/src/contract.rs` (360-385, `CorpusReader` / `CorpusWriter` blocks) — trait ownership (see the correction below).
- `crates/mcp-server/src/tools.rs` (340, 445-476) — `query_incident_list` reads the corpus live per call; `dispatch_mark_incident_resolved` sets `updated_at_unix_nano = now` and passes the same `now` to the guard.
- `pulse-app/src/observability.rs` (2318-2340) — the three incident-path allowlist leaves.
- `pulse-app/ui/src/hooks/use-findings.ts` (1-80) — poll cadence + silent-refresh posture.
- `pulse-app/ui/src/widget/constellation-types.ts` (165-190) — `visibleDots` maps `item.priority_tier` → `severityToHueFraction`.

## Graph impact (rust plane)
- **`mark_resolved`** — 2 production callers: `pulse-app/src/incident_observer.rs:66` (auto-resolve) and `pulse-app/src/incidents_router.rs:300` (the `incidents.mark_resolved` resolver). All other hits are `crates/triage/src/incident/registry.rs` unit tests and `pulse-app/tests/*`. A reconciler becomes the **third** production caller of an existing transition — it introduces no new registry verb.
- **`crate_edges` for `corpus`** — inbound exactly `mcp-server → corpus` and `pulse-app → corpus`; **no `triage → corpus` edge exists**. A `pulse-app`-sited reconciler that reads the corpus adds **zero** new crate edges, satisfying arch §Module dependency direction by construction.

> **Graph caveat — one mis-attribution, verified by reading.** The `calls` view attributed the
> `mark_resolved` call at `pulse-app/src/incidents_router.rs:299` to caller
> `IncidentsApiImpl::list_active()`. Reading the file, the call is at line **300**, inside
> `async fn mark_resolved` (declared `:296`); `list_active` spans `:212-231`. The *caller* is wrong, not
> just the line. Impact here is nil (the site was going to be read anyway), but a caller-set enumerated
> from the graph alone would have named the wrong enclosing function. Treated per the cookbook's own
> posture that graph output is a pointer to verify, not a fact to cite.

## Patterns detected
- **Reconciliation-shaped tick already exists** (`pulse-app/src/incident_observer.rs:57-96`): evaluate a predicate over the registry → `mark_resolved` → persist → broadcast, driven by a 30 s loop and pure w.r.t. an injected `now_unix_nano`. A corpus-divergence reconciler is the same shape with a different predicate source.
- **Synchronous cycle + thin async loop wrapper** (`persistence.rs` `run_incident_persist_cycle` / `run_incident_persist_loop`): the testable unit is the sync `pub fn` over `&dyn` deps; the loop only supplies the interval. test-plan §4's clock-by-parameter convention falls out of this shape for free.
- **Aggregate-once emission per cycle** (`persistence.rs:196-201`): counters folded across the cycle and emitted once, never per row — the obs constraint and the existing code agree.
- **Reads live on `CorpusWriter`** (`crates/corpus/src/contract.rs`): `load_active_incidents` / `load_incident_by_id` / `load_all_incidents` / `count_active_unread` are all `CorpusWriter` methods despite being reads.

## Conventions to follow
- **Registry mutation goes through the declared trait verb**, never a new bespoke state — `mark_resolved` returns the updated `Incident` snapshot for the caller to persist/broadcast (`registry.rs`, both existing callers).
- **Persist failures warn on `triage.incident.persist.error` with a `persist_kind` discriminator** — `incident_mark_resolved` (`incidents_router.rs:314`), `incident_auto_resolve` (`incident_observer.rs:77`), `incident_boot_restore` (`main.rs:757`). A reconciler adds its own bounded `persist_kind` token rather than a new target.
- **`pulse-app` tests live in `pulse-app/tests/*.rs`** (`[lib] test = false`); `triage` takes co-located `#[cfg(test)] mod tests` (test-plan §4).

## New files to create
- *(none required by the finding above — decided at P4 with the fork.)*

## Files to modify
Provisional pending the P4 fork; the members below are the ones the choke-point finding makes load-bearing regardless of shape:
- `crates/triage/src/incident/persistence.rs` — if reconciliation folds into the existing cycle (new predicate + counter + leaf field).
- `pulse-app/src/main.rs` — the wiring site for any new task OR the extra dependency handed to an existing one (**on test-plan §3's boot-smoke trigger list**, so editing it obliges that gate).
- `pulse-app/src/observability.rs` — the allowlist leaf, if a field or target is added (`:2318-2340`).
- `pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs` — the field-set-equality guard, if the leaf changes.
- *Caller threading:* the reconciler needs `Arc<dyn CorpusWriter>` (or a narrow read trait over it) alongside the registry. `main.rs` already constructs both (`corpus_writer`, `incident_registry`), so threading is a constructor argument at one site, not a chain.
- *Crate-local companions:* `pulse-app/tests/integration_incident_write_guard.rs` already exercises the app↔corpus guard pair and is the natural home for a divergence test; `crates/triage/src/incident/persistence.rs`'s own `mod tests` holds the cycle pins.

## Open questions
- **Which fork, and where does the reconciler sit?** → blocks: **plan-decision**. The choke-point finding prunes (b) and (c), but leaves a real cost fork *inside* (a): fold the corpus re-read into the existing 60 s persist cycle (no new task, no new heartbeat, no new wiring trigger — but couples reconciliation to persistence cadence) versus a separate reconcile task (independent cadence, but owes obs-plan §3's heartbeat-with-progress-field and mints a test-plan §1 wiring trigger). P4 resolves this before synthesis.
- **Does the chunk discharge or restate `mcp-incident-read-back-cross-process-coverage`?** → blocks: **plan-decision**. test-plan §6 P3 records the debt on this exact tool; a RED leg that drives a real sidecar resolve is close to, but not identical with, the committed subprocess CONTENT assertion the trigger asks for.
- **Two UI-effect details the fork must not assume away** → blocks: **implementation-scope**. (i) `ResolutionTrigger` is discarded today, so a reconciler may pass either variant; if the report or an observable should distinguish an externally-resolved row, that is new plumbing, and its absence must be stated rather than implied by adding a variant. (ii) The layouts extract's own question — whether the findings window RE-SIZES on a mid-session count change or only at open (`use-findings-window.ts`) — was **not** resolved by this research: the ~1 s poll settles the row CONTENT, which is a different property from the window's height. Left explicitly open rather than assumed, since a fork that removes a row could leave an over-sized window.
