# Session Handoff

**Last Updated:** 2026-08-29T16:10:49Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 41 ahead after this wrap's commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-29-app-registry-reconciliation): an externally-resolved incident leaves the running app without a restart`

## Position
- Done: **2026-08-29-app-registry-reconciliation** — the app-side half of the persist-vs-resolve residual is CLOSED. The 60 s persist cycle now RECONCILES before it writes, reading the durable active-id set through a new narrow `DurableActiveIncidents` port and resolving registry rows absent from it. An incident an agent resolves through MCP now leaves the running app within two cycles, **no restart**.
- Next (first markerless): **Halo State Pulse canvas disposition** — the design signature either renders or the specs stop claiming it does. Carries the **audit PREREQ (pin #22)**; **session 55 DISCHARGED the full-form probe, so 56 and 57 are between-points and session 58 is the next INTERVAL POINT.**
- Then: Advisory backlog · npm advisory coverage · Diagnostics un-muting + harness-truth sweep (**+2 CARRYs this wrap**) · Staged-bindings assertion · Metrics label surface · ACL-rejection logging.

## Work done
Research collapsed the route entry's three-way fork before any code was written: **`IncidentRegistry::list_active` is a single choke point** feeding all three consumers (findings, the constellation dot-hue join, the persist cycle), and **both UI surfaces already poll at ~1 s** — so the fix is backend-only and reaches the screen with no frontend change. That pruned the sidecar-notification fork (no sanctioned cross-process transport) and the surface-the-staleness fork (a working reconciler removes the state rather than displaying it).

**Delivered:** the `DurableActiveIncidents` read port in `triage` (ids only — nothing decrypts or decodes a payload there, and no new crate edge) + the adapter as a second trait view over the same `Arc<dyn CorpusWriter>` + the reconcile step with its **fail-safe** (a durable-read `Err` reconciles NOTHING; treating it as an empty set would resolve the entire active list) + `reconciled_count` folded once per cycle + the committed cross-process CONTENT test + `cargo xtask smoke:external-resolve`.

**Acceptance MET.** GREEN: `item_count 2 → 0 · reconciled 1 · declined 0 · 2 persist cycles · 0 panics`, a real `andromeda-pulse-mcp` subprocess having resolved id=1. RED (mutation, reconciliation removed): `reconciled 0 · declined 2` → FAIL.

Gates: nextest **2074/2074 + 1 skip** (+18 = the tests added exactly, each verified **collected by name**) · clippy `-D warnings` 0 · fmt clean · cross-process test 2/2 with **neither skipped** · `capability-widening-check` clean · `check:ingest-progress` PASS · `deny bans licenses sources` **ok** · `capability-drift` clean LAST after bindings regen · **1 fix-loop iteration** · no gate deferred.

## Drift resolved
**6 amendments across 4 masters · 1 escalation resolved · drift = 0.** architecture.md ([Corpus Write Arbitration] Accepted residual → CLOSED) · obs-plan §8 (persist leaf 4 → 5 fields) · test-plan §1 + §6 P3 (MCP cross-process trigger NARROWED — `mark_incident_resolved` discharged, three tools still owed) + §3 (scenario leg registered) · security-plan §Dependency Security (session-55 probe).

Cascade re-derived 5 leaves by provenance: `docs/obs-summary.md` · `docs/tests-summary.md` · `rules/observability.md` · `rules/verification-harness.md` · `rules/security.md`. Arch's own leaves (`conventions.md`, `stack.md`) correctly excluded — neither restates the amended clause.

**Escalation (resolved with the operator):** test-plan proposed a new `durable-incidents-adapter-and-wiring-coverage` trigger whose premise — "the trait's pins live in triage against a test double" — is **false at HEAD**: this chunk strengthened `integration_incident_write_guard.rs` to drive the REAL adapter over a real corpus. Rejected; the narrow residual is below.

## Notes
- **Curation:** T1 0 · T2 3 · T3 0 · 1 CORRECTION (cap-exempt) · 0 conflicts. `rules/verification-harness.md` gains the harness-negative-findings-need-a-second-source entry; `rules/testing.md` gains two in-place extensions (the independent-producer verdict half; the `--features` second-native-compile disk facet). The correction: `docs/session-learnings.md`'s 2026-08-27 entry claimed the app "still shows such an incident active until restart" — this chunk measured that false and the entry is tagged `[corrected 2026-08-29]`. CLAUDE.md untouched at **156/200**.
- **Coverage:** chunk claimed 0 caps; version stays **21/22 verified, P-075 pooled** (Conductor's).
- **THE finding worth carrying forward:** the mutation check disproved one of the leg's own verdict halves. `item_count` went **2 → 0 in BOTH arms** — the finite storm's auto-resolve produces that drop with or without reconciliation — so a verdict keyed on it would have passed in both worlds. Only `reconciled_count` / `declined_count` discriminate. **For each half of a verdict, name what else in the system could produce that same signal in the window.**
- **A harness typo produced a confident FALSE claim about the product.** The leg printed "the storm did not form" while the corpus held 2 `active` rows and the log showed 3 `incident.created`; `query_incident_list` returns `items[].incident_id`, not `items[].id`. Cost 3 leg runs. Fixed, and the INCONCLUSIVE text reworded to stop asserting a cause it cannot distinguish.
- **Surfaced, not fixed — now CARRY'd** onto "Diagnostics un-muting + harness-truth sweep": (a) `pulse://stream/incidents` is a registered event topic with **producers but no consumer** (nothing subscribes, nothing bridges it to the webview) — which is why the reconciler deliberately emits no lifecycle event; (b) security-plan's §Input Validation MCP row and §Threat Model MCP vector enumerate **4 tools while 8 have shipped** since chunk #94 (pre-existing; the security detector correctly declined to propose it).
- **Residual from the rejected trigger (handoff, not a route row):** `main.rs` derives both trait views from the SAME `Arc` rather than two. Unpinned — but two `Arc`s over one `Arc<dyn CorpusWriter>` are behaviourally identical, so no test could distinguish them and there is no measured defect to guard.
- **Audit PREREQ (session 55):** INTERVAL POINT, **full-form probe RAN**. `cargo audit` true exit 1 read directly, basis byte-identical (`duplicate advisory ID: RUSTSEC-2026-0244`, cargo-audit 0.22.2); `deny advisories` exit 1 at **8 DISTINCT** ids 0189/0190/0194/0195/0204/0222/0253/0258, set identical to the prior enumeration; `bans licenses sources` exit 0; block-vs-id rule re-confirmed at 10 blocks for 8 ids. Deferral does NOT end. **Next interval point: session 58.**
- **Disk:** `target/debug` reached 301.6 GB on a 300 GB drive; the `--features mcp-server` graph triggers a SECOND full DuckDB C++ compile, so `cargo clean -p pulse-app` (97.4 GiB) was not enough and the pre-authorized full `cargo clean` (310.1 GiB) was taken. 115 GB free at wrap.
- Audit trail: `.andromeda/runs/2026-08-29T15-48-43Z-wrap/` (+ phase run dir `2026-08-29T11-35-47Z-phase/`).
- Last failed command: none.

## Deferred learnings
- None deferred by the cap — 3 Tier-2 entries + 1 correction against a cap of 3 (corrections are cap-exempt).
- **`recurrence-despite-learning`: `inject_demo --sustained` is structurally incapable of forming an incident** (constant error rate → the cue evaluator's short/long EWMA ratio converges and never trips the gate). `rules/testing.md`'s 2026-08-22 extension already records this class AND its remedy, and it bit again on a NEW leg this chunk, costing one 240 s run. The corpus entry did not prevent the recurrence — a third restatement is not the remedy; if it recurs again the fix belongs in the leg-authoring reference as a CHECK.
- Still open from prior wraps: the **deferral-destination generalization** (a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes).
