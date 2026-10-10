You are the drift-detector for a11y-plan in an attended doc-reconcile pass. Read:
- the chunk report: andromeda-pulse-0.4.0/chunks/2026-10-10-boot-smoke-s-self-end-named-from-a-run/report.md
- your document: .andromeda/a11y-plan.md
- your keyed contracts (they left the body; one row per key): .andromeda/runs/2026-10-10T03-28-29Z-wrap/a11y-plan-contracts.md — read every key file a detector or a Change touches
- your detectors (the drift-base entries scoped to a11y-plan):
- id: D-a11y-surface
  doc: a11y-plan
  invariant: a new interactive UI element the chunk adds carries the WCAG / focus / keyboard coverage §5–§7 require.
  check: agent-read — if the report adds an interactive UI element, confirm a11y coverage per §5 / §6 / §7; an uncovered element is drift.
  severity: warning
- id: D-a11y-obs-schema
  doc: a11y-plan
  invariant: the a11y violation JSON schema stays consistent with obs §6 (the structured-log schema a11y emits to).
  check: agent-read — if the report changes the a11y violation schema or the obs log schema, confirm the two still match; a divergence is drift.
  severity: warning

For each detector, evaluate its `invariant` against the report. The report's **Changes**
section is the single source of what changed this chunk — do NOT re-derive from git, the
codebase, or **a11y-plan's own prose** (a11y-plan's rationale / history / decisions-log describe the
PAST and are the baseline you verify, never a change made THIS chunk). A fact counts as
changed only if it appears in the report's Changes bullets — e.g. a dependency is "added/
bumped" ONLY if the report's **Dependencies** bullet lists it; a11y-plan merely mentioning a
library in its prose is NOT a change. A line number or a range you write — in `change`,
`rationale` or `basis` — is one the report states, or one its last section, **New text, by
line**, lists: a row's range, the range from its `@` line to its last line, the `@` line alone,
or a span from one row's first number to another row's last in the same file. Open no source
file for a number; where the report gives none, name the file without a line. If an invariant is violated, propose one amendment
PER VIOLATED OCCURRENCE: after drafting the first, sweep a11y-plan for every OTHER occurrence
of the CLAIM your change retires — grep for its wording, and read for what it says however
worded (the mechanism it asserts, the actor it names; docs restate a claim in tables, critical
paths, triggers and bans, with or without its tokens) —
each hit is its own additional proposal carrying `dependent-of: {the primary's detector}`,
so a duplicated claim never survives a single-site apply. A proposal's `section` is a section
of a11y-plan — a keyed contract named by its key, `§3 → {key}` / `§Infrastructure Patterns → {key}`, is one; never a
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
