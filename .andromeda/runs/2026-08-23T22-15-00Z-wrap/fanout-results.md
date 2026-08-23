# Fan-out results — 2026-08-23-a11y-verification

7 Explore doc-agents, one per spec source, run in one parallel batch against
`andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/report.md`.

**Note on raw twins:** `amendment-flow.md` asks for a per-doc `.raw-fanout-{doc}.md` where a return
needed stripping or carried proposals. All seven returns wrapped their YAML in evaluation prose, so
seven twins would have been warranted; they are CONSOLIDATED here instead — every proposal is
recorded verbatim in substance below, together with each clean verdict and its stated basis. Recorded
as a deviation rather than done silently.

## Verdicts

| doc | proposals | verdict |
|---|---|---|
| arch | 0 | clean — no new registry-class resource, no new dependency/runtime |
| security-plan | 1 | 1 primary (severity `escalate`, dispositioned routine by actual class) |
| design-system | 3 | 1 primary + 2 `dependent-of` |
| layout-templates | 4 | 2 primaries + 2 `dependent-of` |
| test-plan | 5 | 2 primaries + 3 `dependent-of` |
| obs-plan | 0 | clean — no new telemetry, no logging, no schema change |
| a11y-plan | 6 | 1 primary + 5 `dependent-of` |
| **total** | **19** | **all applied; 2 escalations resolved WITH the operator** |

## Clean verdicts (basis)

- **arch (D-arch-resources, D-arch-decisions).** Report Changes record no new public fn / IPC procedure /
  endpoint / export / port / socket / env var, and zero compiled-source delta. New artifacts are webview DOM
  handles and one Playwright spec — not arch-registry classes (§File naming defers webview conventions).
  Dependencies "none added, none bumped". The agent additionally swept arch for every claim shape this chunk
  retires (spec-set counts, the NOT-SHIPPED region/table wording, the 7-ID advisory count, the CI a11y stage)
  and found none — so no duplicate-occurrence proposal either.
- **obs-plan (D-obs-instrumentation, D-obs-stack, D-obs-pii).** No new operation lands on a §4 must-trace path;
  no `metric.*` observable joins §5's set; per-keypress emission would in fact be barred by §11. No logger or
  OTel setup introduced. No logging added at all; the one new accessible-tree surface is measured aggregate-only
  (no service name reaches it), matching §8's ban on `service` as a constellation label.

## Proposals (all applied)

### security-plan — 1
1. `D-security-deps` · §Dependency Security → CI integration (standing-deferral Probe clause) ·
   owned upgradeable advisory IDs **7 → 8**, enumerated. Raised at `escalate` severity; the finding is a stale
   measured COUNT, not a banned/unvetted dependency (Dependencies "none added, none bumped";
   `cargo deny check bans licenses sources` ok). Dispositioned **routine-APPLY by actual class**, and the
   severity/class mismatch was codified as a new playbook rule (below). Duplicate sweep: single occurrence in
   the body; other hits are append-only sidecar history.

### design-system — 3 (playbook 2026-08-14 routine-APPLY)
1. `D-design-tokens` (primary) · §Component Patterns → Input Fields Error state · error text `#C7556A` →
   `var(--color-text-primary)`, accent as border/icon only.
2. `D-design-tokens` (dependent) · §Color Palette → Accent usage · retire the "pending the error-UI chunk" deferral.
3. `D-design-tokens` (dependent) · §Color Palette → Semantic Colors, Error row Text cell `#C7556A` → `#E8EEF7`.
   The table cell mattered most: left alone it keeps a hardcoded sub-4.5:1 normal-text value alive after the prose is fixed.

### layout-templates — 4 (playbook 2026-07-08 routine-APPLY)
1. `D-layout-surface` (primary) · §Wireframe notes — Traces · hero recorded as a landmark region with the stable
   literal name + `aria-describedby` hidden aggregate summary; `role="status"` deliberately not applied.
2. `D-layout-surface` (dependent) · §Primary screens → Full dashboard (Traces view) bullet · same region inventory.
3. `D-layout-surface` (primary) · §Component — Trace data table · new Keyboard/focus block (roving tabindex,
   Up/Down/Home/End/Enter/Esc, `--border-focus` ring, in-row button `tabIndex={-1}`, focus survives re-poll).
4. `D-layout-surface` (dependent) · §Component — Investigation modal → Trigger · adds Enter-on-focused-row and
   notes the in-row button left the tab order.

### test-plan — 5
1. `D-tests-coverage` (primary) · §1 trigger `traces-surface-a11y-semantics-absent` → **LANDED**, premise retracted
   as measurement-disproved. routine-APPLY.
2. `D-tests-coverage` (dependent) · §6 Selector strategy · Traces roles removed from the phantom-name example list;
   measure-at-HEAD rule extended to claimed ABSENCES. routine-APPLY.
3. `D-tests-framework` (primary) · §2 runner inventory · browser-driven a11y tier recorded as ADOPTED; the
   "Playwright UNADOPTED" ruling scoped to `connectOverCDP`. **ESCALATED** — adoption pre-dates this chunk;
   operator ruled apply-all-three with the sidecar naming it a pre-existing gap surfaced, not this chunk's adoption.
4. `D-tests-framework` (dependent) · §9 CI pipeline table · new **A11y suite** stage (`ci.yml:125`). routine-APPLY.
5. `D-tests-framework` (dependent) · §6 Drivers per surface · a11y chain recorded as the second desktop-webview
   driver, plus the measured `launch`-stage flake. **ESCALATED** with #3; resolved together.

### a11y-plan — 6 (playbook 2026-08-14 routine-APPLY, except §11)
1. `D-a11y-surface` (primary) · §5 full-dashboard-traces focus order · cell-navigation premise retired; row-level
   contract stated.
2. `D-a11y-surface` (dependent) · §1 P1 row · NOT-SHIPPED claim retired on both halves; region re-located to the
   sibling canvas wrapper; focus-order cell restated.
3. `D-a11y-surface` (dependent) · §1 P4 row · in-step NOT-SHIPPED note retired; re-poll focus preservation (P-081) recorded.
4. `D-a11y-surface` (dependent) · §7 Landmark roles · region moves into the inventory with the stable name +
   describedby summary; `role="status"` omission recorded as deliberate and reversible.
5. `D-a11y-surface` (dependent) · §11 Anti-Patterns → Keyboard · ban narrowed from "nested focusables" to nested
   TAB STOPS with a grid-lite carve-out. **ESCALATED** — narrowing a NEVER-ban; operator approved.
6. `D-a11y-surface` (dependent) · §1 Surface set extension · P1–P12 → **P1–P14**.

**Orchestrator-raised (beyond the detector set):** §3 "Adding a surface" standing rule also restated the spec-set
count (`p8–p12`) — caught by the orchestrator's own residue grep after the primary apply, not by the detector
sweep. Applied with the same disposition; it is a 7th a11y-plan site.

## Validation summary

| check | outcome |
|---|---|
| 1 Playbook | 16 routine-APPLY · 2 escalate (resolved) · 1 severity-mismatch (codified) |
| 2 Cross-contradiction | none — no two proposals edit the same section in opposing directions |
| 3 Intent-consistency | aligned with the working-route entry + plan acceptance criteria |
| 4 Absence-needs-evidence | every absence claim cites its search (arch, security, obs all did) |
| 5 Expected-amendments floor | all 8 plan-listed entries covered; +6 beyond-plan finds; no under-run |
| 6 Disproved-claims disposition | all 9 disposed — 8 by proposal, `rules/a11y.md` by cascade |

## Escalations resolved WITH the operator

1. **a11y-plan §11 keyboard ban** → narrow to nested TAB STOPS + grid-lite carve-out. Rationale accepted: the flat
   ban prohibited the ARIA APG pattern the chunk shipped, which REDUCES tab stops (one row, vs one row + N buttons).
2. **test-plan §2/§6/§9 Playwright tier** → apply all three sites, sidecar naming it a pre-existing gap.
3. **New playbook rule** (proposed → approved → appended): an escalate-severity detector firing OUTSIDE its escalate
   class is routine when the report substantiates the actual class and the escalate condition is affirmatively absent.
   Generalizes the 2026-06-28 (D-security-input) and 2026-08-16 (D-obs-pii) narrow siblings — third instance, the
   recurrence bar this playbook uses.

## Cascade

- **Cross-master citation grep** (all 7 masters, for each retired wording): only this wrap's own retraction
  narratives matched — no master cites another's retired claim.
- **Preserve-verbatim homes** (CLAUDE.md `USER:session-learnings` · rules' `## Session Additions` ·
  `docs/session-learnings.md`): zero hits — nothing routed to curation.
- **Judgment bases** (`playbook.md` · `drift-base.md`): zero hits.
- **Leaves re-derived:** `.claude/rules/a11y.md` (3 sites) · `.claude/docs/a11y-summary.md` (3 sites) ·
  `.claude/rules/security.md` (1 site). `rules/testing.md`, `docs/tests-summary.md`, `rules/design-tokens.md`,
  `docs/design-summary.md`, `docs/security-summary.md` measured already-correct — no edit.
  CLAUDE.md `GENERATED:setup:warnings` carries no a11y-related line — no edit.
- **Closure:** every remaining match of a retired phrase is inside a deliberate retraction narrative or a
  historical-progression clause. **drift = 0.**
