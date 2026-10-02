# Scope — 2026-08-29-app-registry-reconciliation

**Working-route entry (verbatim intent):** App-registry reconciliation with externally-resolved rows — an
incident resolved outside the app stops showing active until restart.

## Outcome

An incident resolved by a writer OUTSIDE this process — today, the `andromeda-pulse-mcp` sidecar's
`mark_incident_resolved`, which an agent calls — stops being displayed as active by the running app. Today the
corpus becomes correct immediately (the predecessor's monotonic guard made sure of that) while the app's
in-memory registry keeps the row Active until the process restarts, so the two truths disagree for the rest of
the session and the UI shows a finding that no longer exists.

## The defect — mechanism re-verified first-hand at HEAD (2026-08-29, this promotion)

Every coordinate the working entry names was re-derived against the artifact before it entered this scope
(promotion.md: annotations and directive-supplied coordinates fold as HYPOTHESES).

| Claim (working entry) | Re-derived at HEAD | Verdict |
|---|---|---|
| `IncidentPersistence::load_active_incidents` has exactly ONE production caller, at boot | `pulse-app/src/main.rs:751` — inside the boot `restored_incidents` block, feeding `InMemoryIncidentRegistry::from_persisted`. Every other hit is the trait declaration, the impl, a doc comment, or a `tests/` file | ✓ exact |
| The registry therefore never re-reads the corpus during a session | No second production call site exists for that trait method. A mid-session read is nonetheless *reachable* in-process — but *(P3 premise-correction: not via the trait this row originally named)* the incident read methods live on **`CorpusWriter`**, not `CorpusReader`. `CorpusReader` carries only `inspect()` and `path()`; `CorpusWriter` owns `load_active_incidents` / `load_incident_by_id` / `load_all_incidents` / `count_active_unread` (`crates/corpus/src/contract.rs`). `pulse-app` holds `Arc<dyn CorpusWriter>` at boot, so the reachability conclusion stands via that handle | ✓ exact, reachability corrected to `CorpusWriter` |
| An agent calling MCP `mark_incident_resolved` resolves the row cross-process | `crates/mcp-server/src/tools.rs:51` (`TOOL_MARK_INCIDENT_RESOLVED`) → `:177` dispatch → `:445` `dispatch_mark_incident_resolved`. Separate process, calls `CorpusWriter` directly | ✓ exact |
| The 60s persist cycle re-offers the stale row every cycle | `crates/triage/src/incident/persistence.rs` `run_incident_persist_cycle` — `registry.list_active(workspace)` then per-row `update_incident_status`; `run_incident_persist_loop` ticks at `DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS` (60s default), spawned `pulse-app/src/main.rs:1504` | ✓ exact |
| …where the guard declines it — visible as a non-zero `declined_count` on `triage.incident.persist` | `persistence.rs:184` `let mut declined_count: u64 = 0` → `:190` `IncidentWriteOutcome::DeclinedStale => declined_count += 1` → `:200` emitted on `TARGET_INCIDENT_PERSIST` (`"triage.incident.persist"`, `persistence.rs:46`); the exact allowlist leaf is `pulse-app/src/observability.rs:2325-2330`, pinned by `pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs` | ✓ exact — **but see the caveat below** |

**Caveat the entry does not carry, and it matters for the RED leg.** The predecessor's own report records
`declined_count` as *"renders unredacted at the wire but read 0 throughout, so its plumbing is proven and its
arithmetic is test-only."* So the observable this entry offers as the defect's signature has **never been seen
non-zero in production**. Driving it above zero is therefore itself part of what this chunk must demonstrate —
it is the RED evidence, not a pre-existing reading to point at. *(P3-VERIFIED, `[inferred]` dropped, and the
seam is structural rather than probable: `dispatch_mark_incident_resolved` sets `incident.updated_at_unix_nano
= now` and passes that same `now` as the guard's comparison value (`crates/mcp-server/src/tools.rs:454-463`),
so the corpus row is necessarily FRESHER than the registry snapshot the app still holds; the app's next persist
cycle lists that stale row and its write is declined by the monotonic predicate, incrementing `declined_count`.
The observable is reachable by construction, not by luck.)*

**Two asymmetries found while re-deriving, neither stated by the entry:**

- **The sidecar reads live; the app does not.** `crates/mcp-server/src/tools.rs:340` calls the *corpus-level*
  `CorpusReader::load_active_incidents` on every `query_incident_list`. So an agent querying through MCP sees
  the corrected truth immediately while the app's own UI shows the stale one — the divergence is visible from
  outside the app, not only inside it.
- **`IncidentRegistry` already has the transition this needs.** `mark_resolved(id, now_unix_nano, trigger)`
  (`crates/triage/src/incident/registry.rs`) is a declared trait method with a `ResolutionTrigger` argument, so
  a reconciler would drive an existing transition rather than introduce a new registry state.
  *(P3 premise-correction, `[inferred]` resolved but NOT as either branch anticipated: the enum has exactly two
  variants — `AutoResolve` and `ExplicitResolve` (`registry.rs:31-34`) — and the question of whether they can
  express "resolved externally" is **MOOT**, because the argument is bound as `_trigger` at BOTH impl sites
  (`registry.rs:96` and `:256`) and discarded. It is not persisted, not broadcast, and not observable. A
  reconciler may pass either variant with zero behavioural difference; making the distinction visible would be
  NEW work — adding a variant alone would change nothing and would read as if it had.)*

## In scope

1. **The running app stops showing an externally-resolved incident as active.** The remedy SHAPE is **OPEN by
   the entry's own text**, which names three candidates and deliberately does not choose: (a) a periodic
   corpus re-read of resolved ids, (b) a notification path from the sidecar, or (c) accept restart as the
   reconciliation point and make the staleness VISIBLE instead. This chunk decides it in P4 with research in
   hand, on the record. Option (c) is a legitimate outcome, not a cop-out — the entry offers it explicitly —
   but it changes what "stops showing active" means, so it is a costed fork put to the operator at the P4/P5
   gate, never an author's silent pick.
2. **Measured RED before the fix.** The divergence must be shown first: an external resolve, then the app
   still listing the row active (`incidents.list_active.request` → `item_count`) and its persist cycle
   declining that row (`triage.incident.persist` → `declined_count` > 0). Both observables already exist and
   are allowlisted, so no new instrumentation is presumed. *(This is the standing house discipline across the
   `2026-08-22` → `2026-08-28` chunks.)*
3. **The two truths agree after the fix** — the app's own observable and a corpus read-back for the same
   workspace key report the same active set, measured rather than argued. This mirrors the predecessor's
   VERIFY clause, pointed at the half it deliberately left open.
4. *(Added at P5 val-1 as intent-incomplete — planning surfaced two deliverables this scope did not name.)*
   **(a) A committed cross-process assertion on `mark_incident_resolved`'s response CONTENT**, discharging
   `mcp-incident-read-back-cross-process-coverage` (test-plan §1 · §6 P3). Operator-selected at the P4 fork:
   the chunk must stand up a real sidecar subprocess for its RED leg anyway, so the marginal cost is one
   committed test, and leaving it out would mean building exactly the harness the standing debt asks for and
   then not committing it. **(b) `cargo xtask smoke:external-resolve`**, the producing invocation for the live
   legs — required rather than optional, because this scope's own item 2 and 3 assert log field values, and a
   plan may not assert a runtime artifact no listed command produces.

5. **The residual the predecessor documented is closed or re-stated honestly.** Its scope recorded
   "the corpus becomes correct, the running app may disagree until restart" as an ACCEPTED residual owned by
   this entry. Whichever fork wins, this chunk's report must say plainly which of those two sentences is now
   true — including, under option (c), that the disagreement persists by decision and is merely surfaced.

## Boundaries — explicitly NOT in scope

- **The corpus write guard is not revisited.** `2026-08-27-incident-persist-vs-resolve-write-race` landed the
  monotonic last-writer predicate at the corpus choke point all seven writers traverse; the entry's own NOTE
  names it a PREREQUISITE that already landed and scopes this entry to the app-side half. No change to
  `update_incident_status`, its predicate, or `IncidentWriteOutcome`.
- **No incident identity / dedupe semantics change.** arch §Fault Identity (`(kind, scope, scope_id)`
  coalescing; the `fingerprint_match` arm) stays untouched.
- **No new TauRPC procedure is presumed.** If a fork requires one it drags the full binding set (router
  registration + `EXPECTED_PROCEDURES` pin + the `emit_taurpc_bindings` test merge) and that cost is stated on
  the P5 review card before approval, never absorbed silently.
- **No DDL / `SCHEMA_VERSION` bump is expected.** Reconciliation reads rows that already exist. If a fork
  turns out to need one, that is a P4/P5 escalation.
- **The sidecar's own behaviour is not redesigned.** `dispatch_mark_incident_resolved` already writes
  correctly; fork (b) would ADD a notification path, not alter the write.
- **No other markerless entry is absorbed** — `Halo State Pulse canvas disposition` and the rest of the tail
  stay untouched, and no route reordering happens here.

## Folded annotations

**PREREQ (from the working entry): re-check `cargo audit`.** Standing deferral since
`2026-08-15-corpus-key-persistence`; ratified at the `2026-08-16-fault-identity-semantics-decided` wrap, pin
**#21** (read first-hand off the working-route line at this promotion, not carried from memory), re-pinned onto
this entry from `2026-08-28-duplicate-span-replay-fails-loudly` with origin preserved.

- Basis: upstream RustSec DB duplicate-id parse error (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`,
  cargo-audit 0.22.2) — external decay.
- Overlap signal: `cargo deny check advisories`, designed-red; last enumeration eight owned DISTINCT ids
  0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258, with `cargo deny check bans licenses sources` exit 0.
- **Interval position — this chunk owes the FULL-FORM probe.** Session 52 ran it full-form; sessions **53 and
  54 were between-points** that recorded the skip. `state.yaml` `session_count` is **54**, so this chunk's wrap
  is session **55**, the next INTERVAL POINT. The owed action is the full-form probe: run `cargo audit`, read
  its true exit directly (never through a pipe that masks it), reproduce the basis byte-identically, and
  re-enumerate the overlap first-hand.
- Re-derive the owned-ID count first-hand at the probe (it read 7 at session 28, 8 at session 40 — the set
  GROWS) and count DISTINCT `RUSTSEC-` ids, never error blocks (10 blocks for 8 ids at session 43).
- Probe records carry **no** running "Nth consecutive" ordinal — two different cadences were both narrated that
  way and the counts drifted; write `set identical to the prior enumeration` instead.

**NOTE (from the working entry): the guard chunk is a PREREQUISITE that already landed; this entry owns only
the app-side half.** Verified: `2026-08-27-incident-persist-vs-resolve-write-race` is `complete` in
master-route, and its own scope records the app-side residual as owned by a new entry minted at its wrap —
this one. Both sides agree on the ownership split.

**No `BLOCKED-ON:` annotation on this entry** — scanned at an annotation position (the token immediately after
`\s{2,}` or ` · `); the line carries only `NOTE:` and `PREREQ:`. The `SHAPE (open)` clause is a design fork,
not a block.

## Why this is worth a chunk

The product's one durable record and the surface a developer actually looks at disagree for a whole session,
in the direction that wastes attention: the app shows a finding that has already been resolved. It crosses a
process boundary — an agent resolves through MCP and sees its own change land, while the human's window does
not — so the two audiences of the same incident see different states. The predecessor deliberately fixed the
durable half and handed this half forward rather than widening its own blast radius; this is that entry.
