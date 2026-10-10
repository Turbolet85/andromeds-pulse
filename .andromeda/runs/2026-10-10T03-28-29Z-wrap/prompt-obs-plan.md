You are the drift-detector for obs-plan in an attended doc-reconcile pass. Read:
- the chunk report: andromeda-pulse-0.4.0/chunks/2026-10-10-boot-smoke-s-self-end-named-from-a-run/report.md
- your document: .andromeda/obs-plan.md
- your keyed contracts (they left the body; one row per key): .andromeda/runs/2026-10-10T03-28-29Z-wrap/obs-plan-contracts.md — read every key file a detector or a Change touches
- your detectors (the drift-base entries scoped to obs-plan):
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

For each detector, evaluate its `invariant` against the report. The report's **Changes**
section is the single source of what changed this chunk — do NOT re-derive from git, the
codebase, or **obs-plan's own prose** (obs-plan's rationale / history / decisions-log describe the
PAST and are the baseline you verify, never a change made THIS chunk). A fact counts as
changed only if it appears in the report's Changes bullets — e.g. a dependency is "added/
bumped" ONLY if the report's **Dependencies** bullet lists it; obs-plan merely mentioning a
library in its prose is NOT a change. A line number or a range you write — in `change`,
`rationale` or `basis` — is one the report states, or one its last section, **New text, by
line**, lists: a row's range, the range from its `@` line to its last line, the `@` line alone,
or a span from one row's first number to another row's last in the same file. Open no source
file for a number; where the report gives none, name the file without a line. If an invariant is violated, propose one amendment
PER VIOLATED OCCURRENCE: after drafting the first, sweep obs-plan for every OTHER occurrence
of the CLAIM your change retires — grep for its wording, and read for what it says however
worded (the mechanism it asserts, the actor it names; docs restate a claim in tables, critical
paths, triggers and bans, with or without its tokens) —
each hit is its own additional proposal carrying `dependent-of: {the primary's detector}`,
so a duplicated claim never survives a single-site apply. A proposal's `section` is a section
of obs-plan — a keyed contract named by its key, `§3 → {key}` / `§Infrastructure Patterns → {key}`, is one; never a
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
