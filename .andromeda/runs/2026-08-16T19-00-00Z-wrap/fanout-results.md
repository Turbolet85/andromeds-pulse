# Fan-out results — 2026-08-16-baseline-family-reachability

7 Explore doc-agents, one batch. Returns are recorded verbatim below (proposal-carrying returns inline here
rather than in separate `.raw-fanout-{doc}.md` twins — none needed stripping, so this consolidated file IS
the raw record for all seven).

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 1 (D-arch-resources, warning) |
| security-plan | drift | 1 (D-security-input, **escalate**) |
| design-system | clean | `proposals: []` |
| layout-templates | clean | `proposals: []` |
| test-plan | drift | 3 (D-tests-coverage, warning — 1 primary + 2 `dependent-of`) |
| obs-plan | drift | 2 (D-obs-pii, **escalate** — 1 primary + 1 `dependent-of`) |
| a11y-plan | clean | `proposals: []` |

**Totals:** 7 proposals across 4 docs · 3 docs clean · 2 escalate-severity families.

## Verbatim returns

### arch — D-arch-resources (warning)
- section: Occupied Resources → Environment variables
- change: register `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` — production-consumed, bounded parse
  (non-zero, `< WINDOW_DURATION_SECONDS`, fallback to 3600s), resolved once at boot and handed to
  `BaselineState`; env layer only; emits a once-per-boot WARN when non-default.
- rationale: report §Changes → Symbols/APIs lands the var; arch env registry (lines 213–230) omits it.
  Agent grepped for other occurrences: none (Config-management + Inherited-Defaults reference the
  `ANDROMEDA_PULSE_*` family generically), so no `dependent-of` proposal.

### security-plan — D-security-input (escalate)
- section: §Input Validation → CLI / env var inputs row
- change: add the var to the row's name list + validation column (bounded `u64`, trimmed, non-zero,
  `< WINDOW_DURATION_SECONDS`, never panics, not a path, no user data).
- rationale: report declares it production-consumed; the row is the doc's per-name registry and omits it.
- agent's single-site evidence: per-name occurrences exist only at line 138; lines 34/84 are `ANDROMEDA_PULSE_*`
  wildcards and line 72 is the path-env-var-only list, which a non-path var does not join.
- agent also reported **no drift** for D-security-auth (no identity/session/token/key surface; the
  `serde(skip)` field leaves the bincode shape and the keyring flow untouched) and **no drift** for
  D-security-deps (report: no dependency added or bumped; the `cargo audit` deferral matches §Dependency
  Security as written — session 26, interval next at 28, skip recorded, overlap re-observed).

### design-system — clean
`proposals: []`. All three of the report's new surfaces carry `tokens n/a` / `a11y n/a (no UI)`; Files are
all Rust; no `hardcoded✗` flag anywhere; design-system is deliberately absent from the report's expected
amendments.

### layout-templates — clean
`proposals: []`. All six touched files are Rust backend; report states "No new IPC method, endpoint, port,
socket, or broadcast topic"; the new config key is env-layer-only so it never reaches the §Settings modal
form-controls list; the report's "surfaces" are coverage-matrix boundaries, not user-facing regions.

### test-plan — D-tests-coverage ×3 (warning)
1. §2 pyramid Unit row — add `triage` to the per-crate enumeration.
2. §4 "What unit tests cover" — add a `**triage crate:**` bullet (resolver bounded-parse family +
   `BaselineState` window surface; clock injected by parameter). `dependent-of: D-tests-coverage`.
3. §4 Conventions → Test file location → Rust — add `triage` to the LIBRARY-crate parenthetical.
   `dependent-of: D-tests-coverage`.
- rationale: report Changes shows `triage` changed and gained 12 tests (1775 → 1790); the crate appears in
  NONE of the three enumerations. Orchestrator independently confirmed: `grep -c triage test-plan.md` = 0.
- agent also reported **no drift** for D-tests-framework (gates are exactly §3's standard set; webview gates
  correctly omitted; `pulse-app/tests/*.rs` placement matches the documented `[lib] test = false` exception)
  and **no drift** for D-tests-obs-harness (no harness/status/log-format delta; Spec claim 3 measures
  PRE-EXISTING behaviour §3 already documents, and §3 already sanctions the direct-binary variant).

### obs-plan — D-obs-pii ×2 (escalate)
1. §8 Default-deny posture — add the EXACT leaf `triage.baseline.bootstrap_window.override`
   (`resolved_seconds` / `default_seconds` / `reason`), no bare `triage` or `triage.baseline` prefix key.
2. §6 Log levels mapping (`warn` row) — register the same target in the once-per-boot WARN enumeration.
   `dependent-of: D-obs-pii`.
- rationale: the target is new this chunk and §8 listed no entry, so the doc claimed these fields redacted
  while the smoke measured them emitted un-redacted. §6 is the second registration site — every sibling
  once-per-boot WARN is stated in BOTH sections.
- agent also reported **no drift** for D-obs-stack (no dependency delta; `tracing` only, no OTel SDK) and
  **no drift** for D-obs-instrumentation (the boot-path symbols carry the WARN; the per-tick
  `bootstrap_state` signature change correctly adds no per-call emission per the §11 hot-path ban).

### a11y-plan — clean
`proposals: []`. No interactive UI element added (all three new surfaces marked `a11y n/a`); the env key is
env-layer-only so the settings modal gains no control and §3's "add an axe spec per new route/modal" standing
rule is not triggered. No a11y-schema ↔ obs-§6-envelope divergence: the new target adds an emit site + a §8
leaf, leaving the `timestamp`/`level`/`target`/`message`/`fields` envelope untouched.

## Validation (orchestrator)

- **Playbook:** arch + security matched the 2026-06-28 new-env-var rule (routine — the rule names
  security-plan §Input Validation explicitly and scopes D-security-input's escalate severity to
  ACTUALLY-unvalidated boundaries). test-plan ×3 matched the 2026-08-14 routine-APPLY rule (doc-only fix; the
  discriminator "is there an impl half?" answers NO). obs ×2 matched 2026-07-08 in substance but D-obs-pii had
  **no codified escalate-override** → escalated once.
- **Cross-contradiction:** none — four docs, no opposing edits.
- **Intent-consistency:** proposals match the report and the plan's Expected amendments exactly. Aligned.
- **Absence needs evidence:** every agent cited its own grep; the orchestrator independently re-verified all
  five edit sites (arch env registry, security row 138, test-plan 135 / §4 bullets / 306, obs 413 + 521)
  before applying.
- **Expected-amendments reconciliation:** all four plan entries covered by proposals; none under-ran.
- **Disproved-claims disposition:** 6 entries, all disposed — 2 via the scope.md correction already applied at
  /phase P3 plus the P7 master-desc rewrite; 4 routed to route-resolve per operator directive.

## Escalation (resolved WITH the operator)

D-obs-pii ×2 — presented with the recommendation "routine + codify a rule". Operator chose it. Both
amendments applied; a new playbook rule was appended making this class routine, with two load-bearing
conditions stated (the leaf must enumerate EVERY field the emit site emits; the guard must live where it
actually runs — `pulse-app/tests/`, never a `[lib] test = false` src module).

## Handoff note (NOT amended under this marker)

`obs-plan.md:26` claims "8 instrumentable modules (ingest, buffer, viz, ui-bridge, snapshot,
workspace-detector, plugins, mcp-server)". The workspace has 14 library crates and `triage` has emitted
`triage.*` targets since chunk #62, so the count and list are stale by six crates. **Pre-existing, not caused
by this chunk, and not a citation of any passage amended here** — recorded as a handoff note per the
over-reach discipline rather than amended under this marker. Recounting instrumentable modules is a targeted
doc touch-up of its own.
