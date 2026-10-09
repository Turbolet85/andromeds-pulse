# Scope — 2026-08-27-incident-persist-vs-resolve-write-race

**Working-route entry (verbatim intent):** Incident persist-vs-resolve write race — a corpus resolution stays
resolved: the stale-snapshot persist loop stops silently reverting direct writers.

## Outcome

An incident that resolves STAYS resolved in the corpus. Today a resolution written by the auto-resolve
observer is silently overwritten back to `active` by the periodic persist cycle's stale snapshot, and because
a resolved incident never re-enters that snapshot, the reverted row is a **permanent zombie-active** — every
corpus reader (MCP `query_incident_list`, forensic read-back, `storage.inspect`, and next-boot restore) sees
actives that do not exist, while the in-app registry, the UI, and every log observable read correctly.

## The defect — mechanism re-verified first-hand at HEAD (2026-08-27, this promotion)

Every coordinate the working entry names was re-derived against the artifact before it entered this scope
(promotion.md: annotations and directive-supplied coordinates fold as HYPOTHESES). All nine held; two are
cited-not-re-measured and are marked as such.

| Claim (working entry) | Re-derived at HEAD | Verdict |
|---|---|---|
| `run_incident_persist_cycle` snapshots the ACTIVE list then writes every row | `crates/triage/src/incident/persistence.rs:160-181` — `let actives = registry.list_active(workspace)` then, per row, `persistence.update_incident_status(incident.id, &incident)?` | ✓ exact |
| `AutoResolveObserver` writes resolutions directly to the corpus | `pulse-app/src/incident_observer.rs:57-96` `run_one_tick` — `registry.mark_resolved(...)` → `self.persistence.update_incident_status(id, &updated)` → broadcast. It is the **same trait method**, not a sibling path | ✓ exact, and stronger than stated |
| Both tasks spawned ~34ms apart at boot | `pulse-app/src/main.rs:1504` (`run_incident_persist_loop`) and `:1515` (`run_auto_resolution_loop`) — spawned 11 lines apart inside ONE `if let Some(persistence)` block | ✓ structural adjacency exact; the **34ms is a runtime measurement** carried from the 2026-08-27 log, not re-derivable from source |
| Their interval epochs collide permanently | persist `DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS = 60` (`persistence.rs:33`); auto-resolve `DEFAULT_AUTO_RESOLVE_TICK_INTERVAL = 30s` (`incident_observer.rs:24`). 30 divides 60, so **every** persist tick coincides with an auto-resolve tick | ✓ exact — systematic, not rare |
| Resolved rows never re-persist (the cycle lists actives only) | `crates/triage/src/incident/registry.rs:213-221` — `list_active` filters `entry.status != IncidentStatus::Resolved`. Once in-memory status is Resolved the row leaves the snapshot **forever** | ✓ exact |
| Last-writer-wins with zero errors | `crates/corpus/src/contract.rs:603-623` — `UPDATE incidents SET status = ?1, updated_unix_nano = ?2, resolved_unix_nano = ?3, payload = ?4 WHERE id = ?5`. **Unconditional**: no status guard, no version/CAS predicate. The only error arm is `rows == 0`, and a clobber updates exactly 1 row — so the write cannot fail and nothing logs | ✓ exact — this is *why* it is silent |
| `19:21:59.942 count=5 dur=74ms` vs resolutions at `19:21:59.87` | Log values from the preserved 2026-08-27 record, reproduced verbatim in the curated rule | **cited, not re-measured** — evidence, not a HEAD fact |
| The curated T3 rule records the class | `.claude/docs/session-learnings.md:2460` — "STALE-SNAPSHOT PERSIST LOOPS SILENTLY REVERT DIRECT WRITERS", naming the 30s/60s epochs and the diagnostic method | ✓ exact |
| intake-#12's registered observable exists | `.andromeda/obs-plan.md:533` — `incidents.list_active.request`: `item_count` (count), an EXACT allowlist leaf, "Live-verified unredacted — the count is observable forming in the log (0 → 1 → 2) on a storm run" | ✓ exact |

**One consequence the entry did not state, found while re-deriving:** `load_active_incidents`
(`crates/corpus/src/contract.rs:641-648`) reads `WHERE workspace = ?1 AND status != 'resolved'`, and it is the
boot-restore path. So a zombie row does not merely mislead readers — it **re-hydrates into the in-memory
registry as Active on the next boot**, where auto-resolve must resolve it again against a stale
`updated_at`, exposing it to the same race a second time. *(P3-VERIFIED, `[inferred]` dropped:
`pulse-app/src/main.rs:750` is the SOLE production caller of the restore path — the registry never re-reads
mid-session — and the second pass is **not** self-healing by construction: the restored row carries a stale
`updated_at`, resolves on the next 30 s tick, and meets the identical alignment.)*

## PREMISE CORRECTION — the writer count (P3, measured at HEAD)

*[premise-corrected: the mechanism table above is accurate for the pair it names, but the EXPOSURE is
wider than "two writers". The code-graph impact query returns **seven production call sites** reaching
`update_incident_status`, and the seventh is in a **different process**.]* The two consequences that
change this chunk's design:

- **The choke point is the CORPUS layer, not the triage trait.** Sites 1–6 (the persist cycle, the
  auto-resolve observer, `incidents.acknowledge`, `incidents.mark_resolved`, and both
  `inference_runtime` L4 writers) share `triage::IncidentPersistence`. Site 7 —
  `crates/mcp-server/src/tools.rs:461` `dispatch_mark_incident_resolved` — calls
  `CorpusWriter::update_incident_status` **directly** and never touches that trait. Only
  `crates/corpus/src/contract.rs:603` is traversed by all seven, and placing a guard there adds **no new
  crate edge** (`mcp-server → corpus` and `pulse-app → corpus` already exist).
- **The MCP path is a DETERMINISTIC loss, not a race.** The sidecar resolves the row in the corpus while
  the app's in-memory registry still holds it Active; the app's next persist cycle lists it and writes it
  back. That fires on **every** MCP-driven resolution while the app runs, bounded by ≤60 s — same defect
  class, same row, same statement, reached by a path the entry did not name. It is in scope under the
  entry's own outcome ("a corpus resolution stays resolved") and is covered for free by a corpus-layer
  guard — but NOT by a triage-layer one.

Full detail, including the sibling-sweep answer and the `attach_resolution_summary` carve-out that rules
out a status-only predicate, is in `research.md`.

## In scope

1. **The persist write stops reverting a fresher writer.** The remedy SHAPE is **OPEN by the entry's own
   text** — it names two candidates, "make the persist write status-aware (conditional update / re-read at
   write)" or "persist deltas not snapshots" — and deliberately does not choose. This chunk decides it in P4
   with research in hand, on the record; if the artifacts do not decide it materially, it is a costed fork
   put to the operator at the P4/P5 gate, not an author's silent pick.
2. **Proven at the wire, per the entry's own VERIFY clause.** intake-#12's registered observable
   (`incidents.list_active.request` → `item_count`) and a **corpus read-back leg** must agree after the fix —
   i.e. the in-memory truth and the on-disk truth stop diverging, measured rather than argued.
3. **Measured RED before the fix** — a race whose whole signature is "every write succeeds and nothing
   logs" cannot be claimed fixed without first showing it failing (the standing house discipline across
   the `2026-08-22` → `2026-08-27` chunks). *(P3-VERIFIED, `[inferred]` dropped, and the seam is NAMED:
   `run_incident_persist_cycle` is a synchronous `pub fn` over injected `&dyn` deps and
   `AutoResolveObserver::run_one_tick(now_unix_nano)` is pure w.r.t. time, so the collision reproduces by
   **interleaving the two calls directly in one test** — no `tokio::time`, no interval, no `sleep`, which
   is what test-plan §11's sleep ban requires.)*
4. **Sibling-violator sweep — ANSWERED: no violator, for a structural reason.** *(P3-VERIFIED,
   `[inferred]` dropped.)* `run_lifecycle_persist_cycle` and `run_storm_persist_cycle` are **whole-state
   single-call saves**, not per-row list-then-write loops, and the graph shows each has exactly **one**
   production writer (`save_all` ← 1 caller; `save` ← 1 caller). The defect needs two writers to the same
   row with one holding a stale snapshot, so neither sibling can exhibit it. **The sweep therefore lands
   as an explicit NO-OP with that finding stated** — the `2026-08-26-ingest-consumer-stall-under-sustained-load`
   precedent, which did not invent an edit to satisfy the step.

## Boundaries — explicitly NOT in scope

- **The in-app ledger is not being redesigned** — but it is no longer claimed to be *correct in every case*.
  *[premise-corrected at P5 val-1: the original wording said the ledger "is already correct". That holds for
  the in-process race the entry describes — registry, UI, broadcast and every log observable read correctly
  there — but NOT for the cross-process path research found: after an MCP `mark_incident_resolved` the
  registry still holds the row Active, because it never re-reads the corpus after boot.]* This chunk stays a
  corpus **write-path** repair and touches no `IncidentRegistry` semantics. **Operator decision at the P4
  fork: the resulting registry-stale-until-restart behaviour is ACCEPTED as a documented residual** — the
  corpus becomes correct, the running app may disagree until restart, and its persist cycle is declined
  every 60 s for that row. Reconciliation is owned by a NEW route entry minted at this chunk's wrap, not by
  this chunk.
- **No incident identity / dedupe semantics change.** arch §Fault Identity (`(kind, scope, scope_id)`
  coalescing; the `fingerprint_match` arm) is settled and stays untouched.
- **No DDL / `SCHEMA_VERSION` bump is expected** — a status-guarded UPDATE needs none. If the shape chosen in
  P4 turns out to require one, that is a P4/P5 escalation stated on the review card, never a silent
  migration.
- **The BLOB-vs-column `read_unix_nano` divergence is a different, pre-existing concern.**
  `update_incident_status` does not touch the `read_unix_nano` column (`mark_incident_read` owns it), so a
  snapshot write cannot revert read-state; but the bincode payload embeds a possibly-stale
  `read_at_unix_nano`. Out of scope, and named here so implement does not wander into it.
- **The `Ingest consumer initiating freeze` entry is not absorbed.** It is the next markerless entry and
  waits on its armed detector by design.
- **No route reordering, no other markerless entry absorbed.**

## Folded annotations

**PREREQ (from the working entry): re-check `cargo audit`.** Standing deferral since
`2026-08-15-corpus-key-persistence`; ratified at the `2026-08-16-fault-identity-semantics-decided` wrap, pin
**#17** (count re-derived from the route line at this promotion, not carried from memory), re-pinned onto this
entry from `2026-08-27-report-window-copy-affordance` with origin preserved.
- Basis: upstream RustSec DB duplicate-id parse error (`RUSTSEC-2026-0244`) — external decay, re-verified and
  unchanged at the last wrap.
- Overlap signal: `cargo deny check advisories`, designed-red at the same eight owned DISTINCT ids
  0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258; `cargo deny check bans licenses sources` exit 0.
  Sixth consecutive identical result at the last wrap.
- **Interval position: session 49 RAN the probe full-form; session 50 was a between-point that recorded
  `probe skipped per ratified interval (next: 52)`. The next interval point is 52.** This chunk's wrap is
  session **51** — also a BETWEEN-point. The owed action is therefore: re-verify basis + overlap first-hand
  and record `probe skipped per ratified interval (next: 52)` in the chunk report. Not a full probe. Never a
  silent skip.
- Re-derive the owned-ID count first-hand at each probe (it read 7 at session 28, 8 at session 40) and count
  DISTINCT `RUSTSEC-` ids, never error blocks (10 blocks for 8 ids at session 43).

**CARRY resolution — read at its SOURCE, not taken from this entry's summary of it.** The
`Diagnostics un-muting + harness-truth sweep` entry (working-route L114) carries intake-#12 and states the
hand-off verbatim: *"**MECHANISM FOUND** (2026-08-27, idle-observer-generation-damper wrap): the
persist-vs-resolve stale-snapshot write race; the `Incident persist-vs-resolve write race` entry (this tail)
OWNS the fix, and this sweep's item narrows to re-reading the observable clean after that entry lands."*
Ownership is therefore confirmed on both sides, and the sweep entry keeps only a post-landing re-read.

Reading the source added two things this entry's own summary omitted:

- **The ORIGINAL symptom, observed externally five weeks before root-cause:** *"corpus/in-app ACTIVE-LIST
  divergence after auto-resolve — read-back listed **2 active where the in-app ledger showed 1**, and they
  agreed **until the auto-resolutions**"* (intake #12, relay 2026-08-21; evidence: Conductor severity chunk
  leg A). That "they agreed until the auto-resolutions" is an independent confirmation of the mechanism from
  before it was understood, and it is the shape the RED arm should reproduce.
- **The observable's exact coordinates — one of which is STALE and is corrected here.** The CARRY cites the
  leaf at `pulse-app/src/observability.rs:2242`; at HEAD that line sits inside a different target's field
  list. The **claim is true and the pointer is not**: the exact leaf is at **`observability.rs:2321`**,
  `"incidents.list_active.request"` → `["item_count"]`, guarded by
  `pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs` (six pins, incl.
  `no_widening_prefix_key_shadows_the_incident_leaves`). The emit site cited at `incidents_router.rs:221`
  holds — the emit block runs `:221-226`. Corrected at fold time per the standing rule that a cited
  coordinate is a pointer and pointers rot independently of the claims they support; **no new instrumentation
  is needed for this chunk's VERIFY leg.**

No `BLOCKED-ON:` annotation on this entry (scanned at promotion at an annotation position — absent; the
`SHAPE (open, not decided)` clause is a design fork, not a block).

## Why this is worth a chunk

A silent, systematic corruption of the one durable record the product keeps. It has zero error signal by
construction, it is permanent per row, it crosses a process boundary into the MCP surface an agent reads, and
it survives reboots. It was root-caused with exact coordinates last session and explicitly handed to its own
entry rather than fixed in place — this is that entry.
