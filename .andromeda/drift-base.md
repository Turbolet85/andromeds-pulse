# Drift Base — andromeda-pulse

<!--
Drift detectors operated by /andromeda-wrap-session (its fan-out runs each detector against the chunk
report). One detector per entry: { id · doc · invariant · check · severity }. `doc` is one of the 7 spec
sources (arch + the 6 plans). severity: warning (routine — the playbook may auto-apply the amendment) |
escalate (halt + ask the user). Format owned by /andromeda-wrap-session (`references/amendment-flow.md`).

STARTER SET — covers roughly what v2 drift-detection checked, across all 7 greenfield artifacts:
v2 D3 (plan-to-code: crates / IPC / auth-library / test-framework / logging-library mismatches) +
v2 D4 (plan-to-plan: specialist-vs-arch decisions, tests↔obs harness bind, a11y↔obs schema bind).
NOT here (v3 makes them structural, so no drift can accumulate): v2 D1/D2 (artifact staleness —
the code-graph refreshes every wrap / regenerates on demand) · v2 D6 (chunk-progression — the cursor is marker-derived) · v2 D5
(distillation staleness — wrap's cascade re-derives CLAUDE.md/rules/docs on every amendment).
GROWS from dogfood: a resolved escalation that recurs becomes a new detector. §-refs point at the standard
greenfield plan sections.

Report contract: structural detectors read the report's `## Changes` lists (crates / IPC / deps / schema);
the presence detectors (input-validation · instrumentation · PII · tests · a11y · design-tokens) read the
report's `## Coverage of new surfaces` flags. A detector that needs a fact the report doesn't carry must wait
for the report to carry it (extend report-template) — never re-derive from git/code inside the fan-out.
-->

## Detectors

# — architecture —
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

# — security —
- id: D-security-input
  doc: security-plan
  invariant: every new external-input surface (IPC / HTTP / deserialized struct) the chunk adds is validated per §Input Validation.
  check: agent-read — for each new external-input symbol in the report, confirm the validation §Input Validation mandates (e.g. a garde derive) is present; an unvalidated boundary is drift.
  severity: escalate

- id: D-security-auth
  doc: security-plan
  invariant: the auth / crypto / secret handling the chunk uses matches §Authentication + §Secret Management (no off-spec auth library or secret source).
  check: agent-read — if the report touches identity / session / token / keys, confirm the library + flow match §Authentication / §Secret Management; a mismatch is drift.
  severity: warning

- id: D-security-deps
  doc: security-plan
  invariant: every new dependency the chunk adds is allowed by §Dependency Security (not on the ban list).
  check: agent-read — check the report's new Dependencies against the §Dependency Security bans; a banned / unvetted dependency is drift.
  severity: escalate

- id: D-security-logging
  doc: security-plan
  invariant: the PII / scrub / redaction posture stated in §Security Anti-Patterns → Logging matches what the chunk's report says the write + log boundaries now do — including the coverage set (which columns scrub, which are deliberately excluded) and any stated FAILURE MODE of that treatment.
  check: agent-read — if the report's Changes touch a scrubbed column, a redaction counter, a store/log boundary, or a stated failure mode of one, confirm §Security Anti-Patterns → Logging still describes it; a stale coverage or behaviour claim is drift. Grep the restating sites too (§Threat Model Summary → Data classification sensitivity note; §Data Protection → At rest), which have carried duplicates of this posture before.
  severity: warning
  # Added 2026-08-23 (chunk 2026-08-23-metrics-points-identity) WITH the operator. Rationale: this
  # section needed amendment at THREE consecutive chunks — 2026-08-22-pii-scrubber-recall,
  # 2026-08-23-ingestion-scrub-coverage, 2026-08-23-metrics-points-identity — and no detector covered
  # it; each was caught only by the chunk plan's own "Expected amendments (wrap)" list, which depends
  # on the phase author foreseeing the amendment. A recurring uncovered class per P2 step 3.

# — design-system —
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

# — layout-templates —
- id: D-layout-surface
  doc: layout-templates
  invariant: a new user-facing surface / region the chunk adds is described in §Primary Surfaces / the wireframes.
  check: agent-read — if the report adds a UI surface or region, confirm it maps to a §Wireframe entry; an undocumented surface is drift.
  severity: warning

- id: D-layout-status-narrative
  doc: layout-templates
  invariant: a CURRENT-STATUS claim this doc states about an existing surface / layer / signature placement — renders vs unbuilt vs DEFERRED, an owner pointer, cross-surface consistency asserted as maintained — still matches what the chunk's report measured or ruled.
  check: agent-read — read the report's Spec-master-edits bullet, its `Spec claims disproved by measurement` bullet, and any operator RULING it records; if any touches a surface whose status this doc states (§Signature placement on either surface, §Component status blocks, §IA / Cross-surface coordination notes), confirm the doc still states the status and owner correctly — one amendment per restating occurrence (this doc restates status claims across Expression / Signature placement / Primary screens / Wireframe notes / Component / IA on each surface; the 2026-08-21 sweep needed twelve sites). Sketch labels are exempt while a governing section-level status note stands (the standing sketch-lag pattern). Binds to PROSE the chunk moved, not to a new symbol.
  severity: warning
  # Added 2026-08-29 with D-design-status-narrative (same operator approval, same rationale) —
  # layout-templates is the co-owning master of every signature-status claim and historically the
  # widest restater (12-site sweep at 2026-08-21; 11 edits at 2026-08-29).

# — test-plan —
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

# — obs-plan —
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

# — a11y-plan —
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
