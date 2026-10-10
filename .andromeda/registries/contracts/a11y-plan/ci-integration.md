### CI integration

- **Pipeline integration:** GitHub Actions (upstream-context Section 1). A11y assertions run in same `ci.yml` pipeline as tests E2E per upstream-context Section 5 Test Harness Contract binding — since chunk 2026-09-29-ci-wall-time-and-round-trips as their own `a11y` job (on `ubuntu-22.04` alone since chunk 2026-10-09-ci-on-linux-alone; blocking, parallel with the test jobs): `npm run build --prefix pulse-app/ui` → Playwright chromium → `cargo xtask test:a11y`, its artifacts uploaded `if: always()` as `a11y-violations-Linux` and `playwright-a11y-report-Linux`, each upload failing its step when it finds no file (since chunk 2026-10-10-no-ci-step-reads-nothing; the job downloads nothing — its PR-only `a11y-violations-base` download had no producer in any workflow and left at that chunk). Specifically: tests' `npm test` (or equivalent E2E command) invokes Playwright test harness which runs axe-core assertions inline; OR separate `npm run test:a11y` command reuses tests' build / start / status / cleanup harness.

- **Command:** 
  ```bash
  npm run test:a11y
  ```
  Invokes Playwright E2E suite with `@axe-core/playwright` assertions + focus/keyboard test harness + contrast verification harness; emits JSON artifacts to `~/.andromeda-pulse/logs/`.

- **Artifact:** Structured violation JSON (per surface per WCAG SC); uploaded as CI artifact via GitHub Actions `actions/upload-artifact@v4` step.
- **Per-PR regression detection:** the baseline is the file committed in the tree, `pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json` (§3 → Harness wiring & conventions, Regression baseline) — no artifact is downloaded, on a pull request or otherwise. `tests-a11y/regression-detector.mjs`, the last leg of `npm run test:a11y`, compares the current run's violations with it by a `{surface, wcag_criterion, selector, severity, violation_type}` tuple key and fails the job when the current set holds a tuple the baseline lacks (regression = new violation on the same surface / selector / WCAG SC), or when the baseline file is absent (exit 1, `regression-detector: baseline not found at {path}`; since chunk 2026-10-10-no-ci-step-reads-nothing, where the earlier base-branch download — `a11y-violations-base`, a name no workflow uploaded — left). It writes `tests-a11y/regression-set.json`, uploaded with the per-run results for audit.

- **CI gate:** PR cannot merge if:
  - axe-core reports WCAG SC violation (critical / serious severity) on desktop-webview
  - Lighthouse a11y category score < 90
  - Keyboard focus order test fails
  - Contrast verification detects token mismatch (actual ratio < required ratio)
  - The regression leg finds a violation tuple the committed baseline lacks, or the baseline file is absent
