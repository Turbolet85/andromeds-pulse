# Fan-out results — 2026-08-26-ingest-consumer-stall-under-sustained-load

7 doc-agents, one per spec source. **12 proposals · 0 escalations.**

| doc | verdict | proposals |
|---|---|---|
| arch | clean | `proposals: []` |
| security-plan | clean | `proposals: []` |
| design-system | clean | `proposals: []` |
| layout-templates | clean | `proposals: []` |
| a11y-plan | clean | `proposals: []` |
| test-plan | drift | 3 (1 primary + 1 primary + 1 dependent) |
| obs-plan | drift | 9 (4 primary + 5 dependent) |

Raw twins: none warranted — every return parsed cleanly and the five empty returns are recorded
here per `amendment-flow.md` (the consolidated file IS their sanctioned audit artifact). The
security and obs returns carried HTML entity escapes (`&amp;` / `&lt;` / `&gt;`) from the return
transport; entity-decoded on read, which changed no proposal content.

## Clean returns — what each verified

- **arch** — Changes close every registrable class (no ports, no env vars, no TauRPC procedure, no
  crate delta, no dependency). Declined TWO pre-existing near-misses as out-of-scope per the
  2026-06-28/06-30 over-reach rules: arch's "single in-memory `:memory:` DuckDB connection"
  (already inaccurate before this chunk — retention and L1a clone) and the "twelve library crates"
  count word vs the sixteen names in §Occupied Resources. **Both recorded as handoff notes.**
- **security-plan** — no external-input surface, no dependency, no identity/key/crypto touch. Checked
  both restating sites the D-security-logging detector names (§Threat Model Summary → Data
  classification; §Data Protection → At rest) and both still match: the `try_clone`d read connection
  keeps "in-memory `:memory:`, no persistent disk database" true, and the scrubber coverage set is
  untouched (no DuckDB DDL change).
- **design-system / layout-templates / a11y-plan** — no UI element, route, region or interactive
  surface; the sole `ui/**` path has an EMPTY content diff. a11y additionally confirmed the
  a11y↔obs schema bind holds: the movements are inventory additions to obs-plan's target listing,
  not changes to the structured-log record envelope a11y-plan §1/§3 bind to.

## Proposals — grouped, with validation verdicts

All matched existing playbook rules; **none escalated**.

**Group A — `buffer.tick` field pair, TRIPLE-site** (obs §5 primary · §1 · §8 dependent)
→ routine-APPLY. 2026-08-16 D-obs-pii rule, both load-bearing conditions met: (a) the leaf
enumerates every field the emit site emits — the guard asserts set equality in BOTH directions;
(b) the guard lives in `pulse-app/tests/`, where it runs under `[lib] test = false`.

**Group B — `buffer.consumer.stalled` WARN, DUAL-site** (obs §6 primary · §8 exact leaf dependent)
→ routine-APPLY, same rule. The report measures the fields bounded non-PII, and the leaf carries a
fallback-set discriminator pin proving it is load-bearing rather than decorative.

**Group C — progress-vs-liveness, THREE restating sites** (obs §10 primary · §1 · §3 dependent)
→ routine-APPLY per 2026-08-14 (doc-only correction of a claim a measurement disproved). The
discriminator question — "is there an impl half to fix?" — answers NO: the drain-progress signal
IS this chunk's shipped impl, so the doc fix completes in one artifact.
*Orchestrator-verified:* the stall definition restates at obs-plan `:101` (§1), `:257` (§3),
`:598` (§10). `:618` was checked and deliberately EXCLUDED — it describes what
`heartbeat-gap-check.sh` does, which remains true and asserts no exclusivity.

**Group D — connection-isolation topology** (obs §10)
→ routine-APPLY per 2026-07-08 (accurate this-chunk addition inside an existing structure).

**Tests T1 — `check:ingest-progress` into the §3 standard gate set**
→ routine-APPLY per 2026-07-08. Matches the plan's Expected-amendments entry.

**Tests T2/T3 — new §1 pending trigger `viz-read-connection-router-wiring-coverage` + the §4 viz bullet**
→ routine-APPLY (atomic group). **Absence claim verified first-hand by the orchestrator** rather
than accepted: `grep read_connection` outside `crates/viz/src/query.rs` returns ONLY the three
`viz_routers.rs` call sites, and no test constructs `TracesApiImpl` / `MetricsApiImpl` /
`LogsApiImpl` (production `main.rs:986-988,1970-1974` are the only constructors). So reverting the
call sites would silently restore the §10 violation with every test green. Real gap; the trigger is
its sanctioned home. **Not fixed in code** — one wrap-time code edit was already a recorded process
deviation; a second would compound it.

## Validation checks

1. **Playbook** — all 12 matched a routine rule (2026-08-16 · 2026-08-14 · 2026-07-08). No "no rule
   but strange" cases.
2. **Cross-contradiction** — none. Two proposals touch obs §8 (whitelist entry vs a new exact leaf —
   complementary, different entries); two touch obs §10 (Standard+ invariants vs the DuckDB
   Connection Isolation subsection — different subsections); two touch obs `:101` (the field list and
   the stall clause live on the SAME line, so they apply as ONE edit — an ordering fact, not a
   contradiction).
3. **Intent-consistency** — the report matches the working-route entry + plan acceptance criteria.
   The proposals follow from the report. No divergence.
4. **Absence-needs-evidence** — two absence claims, both independently re-derived (Tests T2 above;
   the tests↔obs lateral bind, where `grep '45s|stall' test-plan.md` confirms test-plan does NOT
   restate the stall definition, so amending obs §3 cannot break the bind).
5. **Expected-amendments reconciliation** — the plan's list (obs §1/§5/§6/§8/§10 · test-plan §3) is
   FULLY covered. The fan-out additionally found obs §3 (a 4th stall-clause site the plan missed)
   and the two tests-coverage items — extras, never an under-run.
6. **Disproved-claims disposition** — all four report entries disposed: #1 and #2 (the falsified
   prime hypothesis and the phantom second §10 violator) route to the REPORT per the operator's
   2026-08-26 ruling — `research.md` is never mutated and `scope.md`'s windows are closed, so no
   spec amendment is owed and neither artifact was edited; #3 (plan's wrong binary path) is a chunk
   artifact, disposed in the report; #4 (obs-plan §10 invariant violated in code) was fixed in the
   impl this chunk and its topology change is Group D.
