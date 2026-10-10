## Relevant amendment history

- **2026-05-02** — the plan's first generation: tier Standard, WCAG 2.1 AA with the SC 2.3.3 AAA escalation, and the
  first text of §3 Harness Contract, §9 CI and §11 Anti-Patterns. Why it matters: the §10 Standard+ invariant (zero new
  violations per PR, by artifact comparison), the §11 CI ban on a soft gate, the §11 SLO ban on skipping regression
  detection and the §11 Universal machine-verifiable-evidence rule the extract leans on all rest on this tier decision.
  As read in this sidecar, no later entry changes any of those four, nor the "Per-PR regression detection" label: a
  fork arm that removes the comparison would be the first amendment those sites ever take.
- **2026-06-10** — chunk #99's tag gate. It added the §3 "Harness wiring & conventions" subsection, regenerated the
  in-tree baseline `baselines/a11y-violations-summary.json` on 2026-06-09 from a clean full-chain pass (6
  informational lighthouse-score tuples, 0 violation tuples) and recorded "§9 gate restored". Why it matters: (1) this
  is the amendment behind the extract's second baseline source, the one with a producer inside the tree, and the only
  place the sidecar states what a clean baseline holds; the entry ties the restored regression gate to that in-tree
  file and names no downloaded baseline. (2) It is the direct precedent for this chunk's defect: the Playwright a11y
  suite had been silently dead since session 64 (a list-time import failure) "so the §9 violation-JSON regression gate
  guarded nothing", and the chunk repaired the reader, it did not remove the gate. (3) Its Why states which rule won:
  the §10 hard gates and the §11 machine-verifiable-evidence invariant. (4) It introduced the bare
  `npx playwright test --list` health check, corrected later (see 2026-09-30-perf-instruments-measure-their-budgets).
- **2026-08-23-a11y-verification** — among seven sites, it added to §3 Harness wiring → Adding a surface the
  populated-fixture requirement and the `__mockReject` / `__mockDelayMs` sentinels, and moved the surface set P1–P12 →
  P1–P14. Why it matters: this is the amendment behind the extract's pattern "a sweep over an empty fixture passes by
  comparing nothing"; the witness rule (a comparison reads something only when its input is shown non-empty on the
  run) extends it. Its standing lesson is a trap for P3: an earlier measurement "checked the wrong file and reported an
  absence for a role that was present", so a claimed absence (no workflow uploads the `-base` name, a result file is
  not written) is verified against the thing that owns it.
- **2026-08-30-npm-advisory-coverage** — pa11y 9.x → 10.x and Lighthouse 12.x → 13.x; one of its sites is the §9
  artifact table's E2E row. Why it matters: that row is among the sites the extract lists for an expected amendment
  (`a11y-plan.md:466-467`), and this entry is the last to touch it, for tool versions only, never for whether each
  named file is written or uploaded. It also records a full a11y chain run "with ZERO new violation tuples (regression
  0/0), so the baseline held without churn": the regression leg did read a baseline on that run. The entry does not say
  which of the two sources it read, nor that the run was a CI run. Its closing note is a caution for the pin: no
  detector invariant covers these crossings, they surface only at validate.
- **2026-09-29-ci-wall-time-and-round-trips** — the a11y assertions became their own `a11y` job with the step order
  `npm run build --prefix pulse-app/ui` → Playwright chromium → the PR-only `a11y-violations-base` download →
  `cargo xtask test:a11y`, artifacts uploaded `if: always()` under per-OS names (before: steps inside the shared
  lint/test/build job). Why it matters: this is the wording of §3 CI integration → Pipeline integration and §9
  Pipeline integration that the chunk's fork amends, and the first place this sidecar names the download. The entry
  moved the steps and states no producer for the name; the a11y ↔ tests binding was held unchanged, so test-plan §9's
  A11y suite row moves in step with these sites.
- **2026-09-30-perf-instruments-measure-their-budgets** — §3 Adding a surface: the suite-health probe became
  `npx playwright test --config=playwright-a11y.config.ts --list`; the bare form reads the inert default config and
  prints `Total: 0 tests in 0 files`, exit 1, whatever the suite's state (measured then: config-named → exit 0, 41
  tests in 18 files). Why it matters: it is the amendment behind the extract's cheap health probe, and a nearby case
  of the same family seen from the other side, a probe that reads nothing and so can never pass; a plan step that
  copies the bare form inherits a permanent red. The 41 / 18 count is a 2026-09-30 reading, not re-read here.
- **2026-10-09-ci-on-linux-alone** — the `a11y` job runs on `ubuntu-22.04` alone and its artifacts are named
  `a11y-violations-Linux` and `playwright-a11y-report-Linux` (was a three-system matrix with per-OS names); sites: the
  key file `registries/contracts/a11y-plan/ci-integration.md` (Pipeline integration) and §9 Pipeline integration. Why
  it matters: it fixes the two artifact names the pins in `pulse-app/tests/a11y_perf_workflow.rs` read, and its
  **Kept** line is this chunk's own ground: "the `a11y-violations-base` download has no producer in any workflow",
  found standing, not amended there, "their owner a route matter". This chunk is that owner. The same line leaves a
  second standing item, §9's `Lint | eslint-plugin-jsx-a11y` stage, which no workflow runs: a stated stage with no
  step, not a comparison or an upload, so outside P-128's text, but it sits in the same §9 table the chunk amends and
  is not to be restated as true in passing.
