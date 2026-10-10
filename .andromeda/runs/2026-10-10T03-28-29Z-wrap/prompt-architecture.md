You are the drift-detector for architecture in an attended doc-reconcile pass. Read:
- the chunk report: andromeda-pulse-0.4.0/chunks/2026-10-10-boot-smoke-s-self-end-named-from-a-run/report.md
- your document: .andromeda/architecture.md
- your keyed contracts (they left the body; one row per key): .andromeda/runs/2026-10-10T03-28-29Z-wrap/architecture-contracts.md — read every key file a detector or a Change touches
- your detectors (the drift-base entries scoped to architecture):
- id: D-arch-resources
  doc: arch
  invariant: every new IPC method / endpoint / event / socket / port / env var / workspace crate the chunk lands is registered in arch §Occupied Resources / §Standard Contracts / §Inherited Defaults workspace crates.
  check: agent-read — for each new symbol / API / crate in the report's Changes, confirm it appears in the arch registry sections; an unregistered resource is drift.
  severity: warning
- id: D-arch-decisions
  doc: arch
  invariant: the chunk uses only the stack / runtime / patterns arch §Stack + §Established Decisions allow.
  check: agent-read — compare the report's Dependencies + symbols against §Stack / §Established Decisions; a new library, runtime, or a contradicted locked decision is drift.
  severity: warning

For each detector, evaluate its `invariant` against the report. The report's **Changes**
section is the single source of what changed this chunk — do NOT re-derive from git, the
codebase, or **architecture's own prose** (architecture's rationale / history / decisions-log describe the
PAST and are the baseline you verify, never a change made THIS chunk). A fact counts as
changed only if it appears in the report's Changes bullets — e.g. a dependency is "added/
bumped" ONLY if the report's **Dependencies** bullet lists it; architecture merely mentioning a
library in its prose is NOT a change. A line number or a range you write — in `change`,
`rationale` or `basis` — is one the report states, or one its last section, **New text, by
line**, lists: a row's range, the range from its `@` line to its last line, the `@` line alone,
or a span from one row's first number to another row's last in the same file. Open no source
file for a number; where the report gives none, name the file without a line. If an invariant is violated, propose one amendment
PER VIOLATED OCCURRENCE: after drafting the first, sweep architecture for every OTHER occurrence
of the CLAIM your change retires — grep for its wording, and read for what it says however
worded (the mechanism it asserts, the actor it names; docs restate a claim in tables, critical
paths, triggers and bans, with or without its tokens) —
each hit is its own additional proposal carrying `dependent-of: {the primary's detector}`,
so a duplicated claim never survives a single-site apply. A proposal's `section` is a section
of architecture — a keyed contract named by its key, `§3 → {key}` / `§Infrastructure Patterns → {key}`, is one; never a
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
