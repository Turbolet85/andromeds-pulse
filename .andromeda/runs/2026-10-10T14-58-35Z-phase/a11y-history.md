## Relevant amendment history

- **2026-05-02** — the initial plan: tier Standard, axe-core chosen because it "piggybacks on the tests' Playwright
  5-command E2E harness". This is the origin of the a11y ↔ tests harness binding the chunk re-aims. No later entry in
  the sidecar names §1 Focus Management Test Harness, §1 Screen Reader Test Pattern, §3 Focus management test harness
  (Contract binding) or §3 CI integration (Command, the `logs/` emission) in its Section line, so the sentences that
  say `boot` starts the Tauri app and that a11y result JSON lands in the product's `logs/` stand as first written,
  with no amendment that ever re-measured them against the scripts. A wrap amendment of them is the first; whether
  the a11y chain calls any of the five verbs or writes into `logs/` today is not answered by any entry here. (The
  entry's Lighthouse 12.x / pa11y 9.x pins are superseded by 2026-08-30-npm-advisory-coverage; not this chunk's.)
- **2026-06-10** — chunk #99: created §3 "Harness wiring & conventions" (the mock layer `mock-tauri.ts`, the static
  server `static-server.mjs` with SPA fallback, the standing rule of one axe spec per new route or modal, the
  `--list` health check) and regenerated the regression baseline, restoring the §9 gate. Why it matters: it is the
  source of the extract's "statically served SPA with a mocked IPC layer, not a live engine" pattern, so the a11y
  chain was built not to need a booted program. Its trap is the precedent for this chunk's "a check that can fail"
  item: the suite had been dead since session 64 behind a list-time import failure, so the gate "guarded nothing"
  while reading green. Its bare `npx playwright test --list` form is corrected by
  2026-09-30-perf-instruments-measure-their-budgets.
- **2026-08-23-a11y-verification** — extended §3 Harness wiring → Adding a surface (surface set P1–P12 → P1–P14, the
  `__mockReject` / `__mockDelayMs` sentinels, the populated-fixture requirement) and retracted a NOT-SHIPPED claim
  that had been measured in the wrong file. Bears on this chunk in two narrow ways: it is the current form of the
  standing rule the extract cites (no spec is owed while the chunk adds no route or modal), and its lesson — verify a
  claimed absence in the file of the component that owns the surface — applies to research's two open questions
  (does the a11y job invoke a verb; do a11y files land where an engine check reads). Its `Supersedes` names an entry
  absent from this sidecar (the one UNRESOLVED `Supersedes` the handoff carries): do not cite the superseded marker.
- **2026-08-23-headful-leg-extension** — §1 P1 row: the headful leg's selectors re-pointed to the region's accessible
  name, the native `table` and `.trace-row`; the same fact is discharged in test-plan §6. Listed because that leg
  runs against a live window app, the program the five verbs start today; the entry does not say how the leg reaches
  the app, so whether a re-aimed `boot` touches it is research's to read, not a finding of this history.
- **2026-09-29-ci-wall-time-and-round-trips** — moved the a11y assertions out of the shared lint/test/build job into
  their own blocking `a11y` job, parallel with the test jobs (build → Playwright chromium → `cargo xtask test:a11y`,
  uploads `if: always()`), and recorded "the harness contract shared with tests is unchanged". It is why a `ci.yml`
  edit for the engine's checks can leave the job whole, and the precedent for restructuring the workflow while the
  a11y ↔ tests binding holds. Its matrix and its `a11y-violations-base` download are both gone since (the two entries
  below).
- **2026-09-30-perf-instruments-measure-their-budgets** — the suite-health probe is
  `npx playwright test --config=playwright-a11y.config.ts --list` (measured then: exit 0, 41 tests in 18 files); the
  bare form reads the deliberately inert default config and prints `Total: 0 tests in 0 files`, exit 1, whatever the
  suite's state. If the plan lists the probe after an edit to a shared harness script, it lists the config-named
  form; the 41 / 18 count is that chunk's measurement, not a current one.
- **2026-10-09-ci-on-linux-alone** — the `a11y` job runs on `ubuntu-22.04` alone, blocking; its artifacts are
  `a11y-violations-Linux` and `playwright-a11y-report-Linux`. These are the two fixed names the extract's first
  acceptance line holds, and the set a new engine-check artifact must stay outside. Kept there and still unowned by
  any chunk: §9's `Lint | eslint-plugin-jsx-a11y` stage, which no workflow runs.
- **2026-10-10-no-ci-step-reads-nothing** — the regression baseline is the file committed in the tree
  (`pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json`), nothing is downloaded, an absent baseline is
  exit 1, and the job's two uploads fail their step when they find no file; pinned by
  `a11y_regression_baseline_is_committed_in_the_tree` and `a11y_regression_detector_fails_when_its_baseline_is_absent`.
  Why it matters: it is the state of the job the chunk must leave as it is, and the reason no path under
  `pulse-app/ui/tests-a11y/` may enter the diff. Three limits carried from it: the detector writes
  `tests-a11y/regression-set.json` on every run (ignored by git, so a local a11y run leaves it on the host without
  entering the diff); the absent-baseline arm is held by the pin on the dev host and was read by no runner; the a11y
  upload has three path members and fails only when none matches (a `CARRY:` on `Window's gates retired`, not this
  entry). It is also the nearest precedent in this plan for a gate that read nothing and was made to fail.
