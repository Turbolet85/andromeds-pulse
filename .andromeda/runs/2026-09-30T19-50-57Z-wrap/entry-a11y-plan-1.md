
## 2026-09-30-perf-instruments-measure-their-budgets — suite-health probe names the a11y config
**Section:** §3 Harness wiring & conventions → Adding a surface
**Change:** Verify suite health with `npx playwright test --config=playwright-a11y.config.ts --list`. The bare `npx playwright test --list` reads the deliberately inert default `playwright.config.ts` and prints `Total: 0 tests in 0 files`, exit 1, whatever the suite state (measured: bare → exit 1, 0 tests; config-named → exit 0, 41 tests in 18 files). Was: the bare form.
**Why:** a probe aimed at the inert config can never pass, so a chunk plan copying it inherits a permanent red; test-plan §2 already states the config-named form.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
