You are the drift-detector for design-system.md in an attended doc-reconcile pass. Read:
- the chunk report: /home/turbolet/dev/projects/andromeda-pulse/andromeda-pulse-0.3.0/chunks/2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape/report.md
- your document: /home/turbolet/dev/projects/andromeda-pulse/.andromeda/design-system.md
- your detectors (the drift-base entries scoped to design-system.md):
- id: D-design-tokens
  doc: design-system
  invariant: new UI the chunk renders uses design tokens, not hardcoded values.
  check: agent-read — read the Coverage `tokens` flag of each new UI element in the report; `hardcoded✗` is drift against §Color Palette / §Spacing / §Typography.
  severity: warning
- id: D-design-status-narrative
  doc: design-system
  invariant: a CURRENT-STATUS claim this doc states about an existing surface / layer / signature element — renders vs unbuilt vs DEFERRED, an owner pointer, a build-or-retire fork — still matches what the chunk's report measured or ruled.
  check: agent-read — read the report's Spec-master-edits bullet, its `Spec claims disproved by measurement` bullet, and any operator RULING it records; if any touches a surface whose status this doc states (§Brand Identity Signature element, §Motion high-impact moments, §Surface: desktop-native component patterns, §Self-Validation checks), confirm the doc still states the status, the spec-vs-shipped distinction, and the owner pointer correctly — proposing one amendment per restating occurrence (this doc duplicates status claims across Brand Identity / Motion / native / Self-Validation). NOTE this detector binds to PROSE the chunk moved, not to a new symbol — the Changes bullets may list nothing new and the invariant can still be violated (the D-obs-defect-narrative shape, extended here on evidence).
  severity: warning
  # Added 2026-08-29 (chunk 2026-08-29-halo-state-pulse-signature-deferred) WITH the operator.
  # Rationale: THIRD measured instance of the status-change-binds-to-no-detector class (after the
  # D-security-logging and D-obs-defect-narrative rationales' instances): this chunk's 14-edit
  # defer-disposition amendment set across design-system + layout-templates drew proposals: [] from
  # all 7 detectors and entered solely via the plan's Expected-amendments floor — which held only
  # because the amendments WERE the deliverable the phase author enumerated. The gap bites when a
  # status moves unforeseen (e.g. the deferred layer landing next version). D-obs-defect-narrative's
  # own rationale deferred extending beyond obs-plan "when evidence appears, not before" — it appeared.

For each detector, evaluate its `invariant` against the report. The report's **Changes**
section is the single source of what changed this chunk — do NOT re-derive from git, the
codebase, or **design-system.md's own prose** (design-system.md's rationale / history / decisions-log describe the
PAST and are the baseline you verify, never a change made THIS chunk). A fact counts as
changed only if it appears in the report's Changes bullets — e.g. a dependency is "added/
bumped" ONLY if the report's **Dependencies** bullet lists it; design-system.md merely mentioning a
library in its prose is NOT a change. If an invariant is violated, propose one amendment
PER VIOLATED OCCURRENCE: after drafting the first, sweep design-system.md for every OTHER occurrence
of the CLAIM your change retires — grep for its wording, and read for what it says however
worded (the mechanism it asserts, the actor it names; docs restate a claim in tables, critical
paths, triggers and bans, with or without its tokens) —
each hit is its own additional proposal carrying `dependent-of: {the primary's detector}`,
so a duplicated claim never survives a single-site apply. A proposal's `section` is a section
of design-system.md — a keyed contract named by its key, `§3 → {key}` / `§Infrastructure Patterns → {key}`, is one; never a
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