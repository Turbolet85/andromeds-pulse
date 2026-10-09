### CI integration

- **Pipeline integration:** GitHub Actions (upstream-context Section 1). A11y assertions run in same `ci.yml` pipeline as tests E2E per upstream-context Section 5 Test Harness Contract binding — since chunk 2026-09-29-ci-wall-time-and-round-trips as their own `a11y` matrix job (Linux/macOS/Windows, blocking, parallel with the test jobs): `npm run build --prefix pulse-app/ui` → Playwright chromium → the PR-only `a11y-violations-base` download → `cargo xtask test:a11y`, its artifacts uploaded `if: always()` under per-OS names. Specifically: tests' `npm test` (or equivalent E2E command) invokes Playwright test harness which runs axe-core assertions inline; OR separate `npm run test:a11y` command reuses tests' build / start / status / cleanup harness.

- **Command:** 
  ```bash
  npm run test:a11y
  ```
  Invokes Playwright E2E suite with `@axe-core/playwright` assertions + focus/keyboard test harness + contrast verification harness; emits JSON artifacts to `~/.andromeda-pulse/logs/`.

- **Artifact:** Structured violation JSON (per surface per WCAG SC); uploaded as CI artifact via GitHub Actions `actions/upload-artifact@v4` step.
- **Per-PR regression detection:** Download base-branch `a11y-violations-summary.json` via `actions/download-artifact@v4` (name `a11y-violations-base`); compare current PR violations vs baseline using a `jq`-style filter on `{surface, wcag_criterion, selector, severity}` tuples; fail PR if the current set contains any tuple NOT present in baseline (regression = new violation on the same surface / selector / WCAG SC). Tooling: GitHub Actions step running `jq` (or `node` with the same filter) emits a `regression-set.json` artifact for audit alongside the per-run results.

- **CI gate:** PR cannot merge if:
  - axe-core reports WCAG SC violation (critical / serious severity) on desktop-webview
  - Lighthouse a11y category score < 90
  - Keyboard focus order test fails
  - Contrast verification detects token mismatch (actual ratio < required ratio)
