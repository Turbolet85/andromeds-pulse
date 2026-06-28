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

# — design-system —
- id: D-design-tokens
  doc: design-system
  invariant: new UI the chunk renders uses design tokens, not hardcoded values.
  check: agent-read — read the Coverage `tokens` flag of each new UI element in the report; `hardcoded✗` is drift against §Color Palette / §Spacing / §Typography.
  severity: warning

# — layout-templates —
- id: D-layout-surface
  doc: layout-templates
  invariant: a new user-facing surface / region the chunk adds is described in §Primary Surfaces / the wireframes.
  check: agent-read — if the report adds a UI surface or region, confirm it maps to a §Wireframe entry; an undocumented surface is drift.
  severity: warning

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
