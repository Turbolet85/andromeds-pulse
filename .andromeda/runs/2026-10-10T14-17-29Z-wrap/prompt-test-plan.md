You are the drift-detector for test-plan in an attended doc-reconcile pass. Read:
- the chunk report: andromeda-pulse-0.4.0/chunks/2026-10-10-console-engine-entry-point/report.md
- your document: .andromeda/test-plan.md
- your keyed contracts (they left the body; one row per key): .andromeda/runs/2026-10-10T14-17-29Z-wrap/test-plan-contracts.md — read every key file a detector or a Change touches
- your detectors (the drift-base entries scoped to test-plan):
- id: D-tests-coverage
  doc: test-plan
  invariant: new code paths the chunk adds carry tests at the tier the test-plan requires (§2 Test Strategy).
  check: agent-read — compare the report's new symbols / modules against its Outcome (tests run); a new path with no test at the mandated tier is drift.
  severity: warning
- id: D-tests-framework
  doc: test-plan
  invariant: the test framework / runner the chunk uses matches the test-plan (§2 + §4 unit strategy).
  check: agent-read — compare the report's test commands / runner against §2 / §4; an off-spec framework or runner is drift.
  severity: warning
- id: D-tests-obs-harness
  doc: test-plan
  invariant: the 5-command harness / status shape / log format stays consistent between test-plan §3 and obs-plan §3.
  check: agent-read — if the report changes the harness, status endpoint, or log format, confirm §3 ↔ obs-plan §3 still agree; a one-sided change is drift.
  severity: warning

For each detector, evaluate its `invariant` against the report. The report's **Changes**
section is the single source of what changed this chunk — do NOT re-derive from git, the
codebase, or **test-plan's own prose** (test-plan's rationale / history / decisions-log describe the
PAST and are the baseline you verify, never a change made THIS chunk). A fact counts as
changed only if it appears in the report's Changes bullets — e.g. a dependency is "added/
bumped" ONLY if the report's **Dependencies** bullet lists it; test-plan merely mentioning a
library in its prose is NOT a change. A line number or a range you write — in `change`,
`rationale` or `basis` — is one the report states, or one its last section, **New text, by
line**, lists: a row's range, the range from its `@` line to its last line, the `@` line alone,
or a span from one row's first number to another row's last in the same file. Open no source
file for a number; where the report gives none, name the file without a line. If an invariant is violated, propose one amendment
PER VIOLATED OCCURRENCE: after drafting the first, sweep test-plan for every OTHER occurrence
of the CLAIM your change retires — grep for its wording, and read for what it says however
worded (the mechanism it asserts, the actor it names; docs restate a claim in tables, critical
paths, triggers and bans, with or without its tokens) —
each hit is its own additional proposal carrying `dependent-of: {the primary's detector}`,
so a duplicated claim never survives a single-site apply. A proposal's `section` is a section
of test-plan — a keyed contract named by its key, `§3 → {key}` / `§Infrastructure Patterns → {key}`, is one; never a
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
