You are the drift-detector for obs-plan.md in an attended doc-reconcile pass. Read:
- the chunk report: /home/turbolet/dev/projects/andromeda-pulse/andromeda-pulse-0.3.0/chunks/2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape/report.md
- your document: /home/turbolet/dev/projects/andromeda-pulse/.andromeda/obs-plan.md
- your detectors (the drift-base entries scoped to obs-plan.md):
- id: D-obs-instrumentation
  doc: obs-plan
  invariant: new hot-path operations the chunk adds carry the spans / metrics / logs §4–§6 require.
  check: agent-read — for each new operation symbol in the report, confirm instrumentation per §4 / §5 / §6; an uninstrumented hot path is drift.
  severity: warning
- id: D-obs-stack
  doc: obs-plan
  invariant: the logging / telemetry library the chunk uses matches the obs harness §3.
  check: agent-read — compare the report's telemetry Dependencies / symbols against §3; an off-spec logger or OTel setup is drift.
  severity: warning
- id: D-obs-pii
  doc: obs-plan
  invariant: the chunk does not log raw user input / PII (§8 PII Scrubbing).
  check: agent-read — if the report adds logging that touches user data, confirm redaction per §8; raw PII in logs is drift.
  severity: escalate
- id: D-obs-defect-narrative
  doc: obs-plan
  invariant: a defect / mechanism NARRATIVE §10 states as current — an OPEN-vs-closed status, a measured causal chain, a fixed-vs-open tally, a named owner — still matches what the chunk measured.
  check: agent-read — read the report's `Spec claims disproved by measurement` bullet, its Outcome, and any Coverage row describing a CHANGED failure mode at an existing boundary; if any touches a defect §10 describes, confirm §10 still states its status, its causal chain and its owner correctly. A stale status, a falsified mechanism, or an owner pointer the chunk discharged is drift. §10's LEAD-IN restates the fixed-vs-open tally, so treat it as a second occurrence and propose it as a `dependent-of`. NOTE this detector binds to PROSE the chunk disproved, not to a new symbol — the report's Changes bullets may list nothing new at all and the invariant can still be violated.
  severity: warning
  # Added 2026-08-29 (chunk 2026-08-28-duplicate-span-replay-fails-loudly) WITH the operator.
  # Rationale: §10 defect 4's narrative needed amendment at TWO CONSECUTIVE chunks — its mechanism
  # corrected at 2026-08-28-ingest-consumer-block-under-gap-resume, then corrected AGAIN and the
  # defect closed here — and on BOTH occasions the three existing obs detectors returned
  # `proposals: []`, because instrumentation / logger-stack / PII all bind to something NEW in the
  # report's Changes, and a falsified narrative adds nothing new. Both times only the chunk plan's
  # "Expected amendments (wrap)" floor caught it, which depends on the phase author foreseeing the
  # amendment. A recurring uncovered class per P2 step 3. Scoped to obs-plan deliberately: both
  # measured occurrences are §10's defect list, and a speculative copy in the other six masters
  # would be over-reach — extend it there when evidence appears, not before.

For each detector, evaluate its `invariant` against the report. The report's **Changes**
section is the single source of what changed this chunk — do NOT re-derive from git, the
codebase, or **obs-plan.md's own prose** (obs-plan.md's rationale / history / decisions-log describe the
PAST and are the baseline you verify, never a change made THIS chunk). A fact counts as
changed only if it appears in the report's Changes bullets — e.g. a dependency is "added/
bumped" ONLY if the report's **Dependencies** bullet lists it; obs-plan.md merely mentioning a
library in its prose is NOT a change. If an invariant is violated, propose one amendment
PER VIOLATED OCCURRENCE: after drafting the first, sweep obs-plan.md for every OTHER occurrence
of the CLAIM your change retires — grep for its wording, and read for what it says however
worded (the mechanism it asserts, the actor it names; docs restate a claim in tables, critical
paths, triggers and bans, with or without its tokens) —
each hit is its own additional proposal carrying `dependent-of: {the primary's detector}`,
so a duplicated claim never survives a single-site apply. A proposal's `section` is a section
of obs-plan.md — a keyed contract named by its key, `§3 → {key}` / `§Infrastructure Patterns → {key}`, is one; never a
Decisions Log, which takes no new entry — never a distillation (CLAUDE.md, `.claude/rules/*`, `.claude/docs/*`: the cascade re-derives
them) and never a preserve-verbatim curation home (`## Session Additions`, `USER:session-learnings`,
`docs/session-learnings.md`: curation's channel, not yours).

Return ONLY YAML (or `proposals: []` if no drift):
proposals:
  - detector: D-{slug}
    severity: warning | escalate
    section: {the doc section to edit}
    change: {one line — what the body should now say}
    sidecar: {one line — changelog entry for the amendments sidecar}
    rationale: {why — cite the report}
    basis: {file:line the claim measured — optional; carrying it makes the orchestrator's re-derivation one read}
    dependent-of: {the primary proposal's detector — ONLY on a duplicate-occurrence proposal}
You PROPOSE only. Do not edit any file. The orchestrator validates and applies.