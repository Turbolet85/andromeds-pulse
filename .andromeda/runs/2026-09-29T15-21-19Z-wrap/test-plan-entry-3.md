
## 2026-09-29-p-025-hue-shift-observable-made-gradable — CI rows: per-job DATA_DIR export, xvfb, a11y measured green
**Section:** §9 Pipeline structure → Boot smoke (harness) row · A11y suite row
**Change:**
- Boot smoke: was "sharing the workflow-level `ANDROMEDA_PULSE_DATA_DIR` (ci.yml:12)"; now the cycle runs inside one `xvfb-run` after the Linux system libraries install, and every job exports the variable to `$GITHUB_ENV` right after harden-runner (`runner.*` is unavailable in workflow/job `env:`), pinned by two workflow self-lint guards.
- A11y suite: "runs in CI today" is now measured green on run `ci#36574279289` (6/6, `464f2a3`), with the Playwright chromium browser installed before it.
**Why:** the workflow-level form never parsed; the chunk's CI rehabilitation produced the first real green run.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
