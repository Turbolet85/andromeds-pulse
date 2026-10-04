# Cascade dispositions — 2026-10-04-retry-storm-interpretation-names-its-cause

## The search

The pass's amendments, both in architecture:
- §Occupied Resources `ANDROMEDA_PULSE_L4_DETERMINISTIC` (`:236`): the cue-grounded title in every mode.
- §Established Decisions [Fault Identity] (`:72`): the cue KIND reaches the incident text, identity unchanged.

Both EXTEND current truth, and no prior claim is retired. The sweep therefore looked for any site stating the opposite
or a now-incomplete form:
- the title as model-only;
- the canned title `Deterministic verification incident` as what an incident reads;
- the `Incident.title` / `incident.title` field;
- the `cue_kind_label` tracing label as the only cue label;
- "names the retry" phrasing.

**Tool sweep** (`cascade.py sweep`, `cascade-patterns.toml`, baseline `e71dba53`). Two patterns ran with a fired
control: `model-authored` (control `architecture.md:72`) and
`title-from-model` = `(title|symptom)[^.\n]{0,60}(from the model|model text|by the model|the model wrote)` (control
`obs-plan.md:543`).

**Hand-controlled.** The tool refused five patterns because their controls never fired over the seven pre-pass
masters: `Deterministic verification incident` · `Incident.title` · `incident.title` · `cue_kind_label` ·
`names? (the )?retry`. That is a 0-hit statement about the seven masters, and it is the master-side answer: no master
states the canned title, the title field or the label. They were then grepped by hand (`grep -rn -E`) over the leaves,
curation homes and judgment bases (`CLAUDE.md`, `.claude/rules`, `.claude/docs`, `docs/session-learnings.md`,
`.andromeda/playbook.md`, `.andromeda/drift-base.md`), giving 1 hit.

**Line profile:** `splice.py summary` on architecture gives 13 lines over 2 000 chars, longest 15 413c. The `:72` hits
were read by offset (`cascade.py window`).

## Rows

| row | class | disposition |
|---|---|---|
| `architecture.md:72` model-authored ×2 @c2747 | standing | no change — "`L4Output.fingerprint` — model-authored", a true claim about the fingerprint, not the title |
| `architecture.md:72` model-authored ×2 @c4904 | standing (edited line) | no change — this pass's own new text ("The model-authored symptom, timeline and ranked hypotheses are untouched"), accurate; no intra-line duplicate of a retired claim (none retired) |
| `.claude/rules/testing.md:270` model-authored ×2 | curation | no change — the 2026-08-17 producer-fingerprint repair entry, true history; preserve-verbatim |
| `.claude/docs/session-learnings.md:2551` model-authored | curation | no change — the same fingerprint history; preserve-verbatim |
| `CLAUDE.md:50` model-authored | leaf | re-derived — the GENERATED:setup:warnings Fault-identity bullet, recomputed from arch `:72`: its fingerprint clause still stands, plus the new clause that the cue kind reaches the incident TEXT (`{cue_cause_label(kind)}: {model title}`), never its identity |
| `obs-plan.md:543` title-from-model | standing | no change — the `interpretation.incident.skipped` leaf "never scope_id, title, symptom … or model text": a LOGGING ban, still true (no emit changed; census 0) |
| `.claude/rules/observability.md:69` title-from-model | leaf | no change — the same logging ban distilled; still true |
| `.claude/docs/obs-summary.md:141-142` title-from-model (wrap) | leaf | no change — the same logging ban; still true |
| `.claude/rules/security.md:150` (hand grep: `incident.title`) | curation | no change — the 2026-05-26 resolver-side defense-in-depth scrub over `incident.title / .detail / …`, still true: the grounded title still rides `scrub_string` at projection; preserve-verbatim |

## Leaves re-derived (step 3)

**Arch leaves by provenance header** (`_Extracted from .andromeda/architecture.md_`, first 6 lines): `conventions.md`
and `stack.md`. Both were recomputed for the two amended sections and have no change: neither restates Fault Identity
or the `L4_DETERMINISTIC` entry (`grep -n -i 'fault identity|L4_DETERMINISTIC|incident.*title'` gives 0 hits in both).

**CLAUDE.md GENERATED:setup blocks:**
- `warnings`: the Fault-identity bullet was re-derived (row above).
- `overview` / `modules` / `pointer-table` / `architecture`: recomputed against the two amended sections, no change.
  No module line states incident-title content, and the pointer table and overview carry no claim the amendments
  move.

**Module leaves** (the arch workspace/modules row, read for the amended facts):
- `.claude/docs/services/interpretation.md:30`: re-derived. The deterministic-mode bullet now states the cue-grounded
  title (`Retry storm: Deterministic verification incident`), and that only the title discriminates the cause in this
  mode because the canned rank-1 hypothesis names a retry storm for every incident.
- `.claude/docs/services/triage.md:32`: re-derived. The `contract` bullet names `cue_cause_label(CueKind)` and its
  distinction from the un-re-exported snake_case `cue::classify::cue_kind_label`.

**Not looked for:** the investigate path (`investigate.run_action` DTO `title`, arch `:195`). It does not go through
the incident producer and is outside the report's Changes. It was left standing, as checked at Validate.

**Lateral binds:** test-plan §3 ↔ obs-plan §3 and the a11y ↔ obs schema were untouched (no plan amended).

**Judgment bases** (`playbook.md`, `drift-base.md`): 0 hits for any pattern.
