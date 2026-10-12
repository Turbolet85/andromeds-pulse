
## 2026-10-10-no-ci-step-reads-nothing — the regression baseline is the tree's; an absent baseline fails
**Section:** §3 → CI integration (key file: Pipeline integration, Per-PR regression detection, CI gate) · §3 → Harness wiring & conventions (key file: Regression baseline) · §1 CI Integration (the CI gate list) · §9 CI Integration (the E2E row) · §10 (Zero new violations per PR) · §11 (the per-PR diff ban)
**Change:**
- Per-PR regression detection: was "download base-branch `a11y-violations-summary.json` via `actions/download-artifact` (name `a11y-violations-base`)" and a `jq`-style filter; now the baseline is the file committed in the tree, `pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json`, nothing is downloaded, and `tests-a11y/regression-detector.mjs` (the last leg of `npm run test:a11y`) compares by the tuple key `{surface, wcag_criterion, selector, severity, violation_type}` and writes `tests-a11y/regression-set.json`.
- An absent baseline: was compared as an empty set; now exit 1 with `regression-detector: baseline not found at {path}`, pinned by `a11y_regression_baseline_is_committed_in_the_tree` and `a11y_regression_detector_fails_when_its_baseline_is_absent`.
- Pipeline integration: the `a11y` job's order loses the PR-only `a11y-violations-base` download; its two uploads fail their step when they find no file.
- Both CI gate lists gain one condition: the regression leg finds a tuple the committed baseline lacks, or the baseline file is absent.
- §9 E2E row: was "PR comment with new violations vs base branch"; now a new tuple fails the job and no step posts a pull-request comment.
- §10: was "vs base branch. Per-PR diff detection via artifact comparison"; now vs the tree baseline, no artifact compared.
- §11: the ban stands; "vs base branch" is now "vs the baseline committed in the tree", with "and on an absent baseline".
**Why:** The download never had a producer in any workflow and the comparison read the tree file all along. The operator ruled that a baseline committed in the tree at the commit under test has a producer, the tree, and asked that its absence fail (the operator, the pc overseer, at the P4 dialog, 2026-10-10).
**Kept:** The absent-baseline arm is held by the pin on the dev host; no runner read it. The a11y upload has three path members and fails only when none matches. Whether the version still wants a pull-request comment with the new violations was put to the operator at this wrap's route-resolve card.
**Ref:** .andromeda/runs/2026-10-10T08-04-06Z-wrap/
