
## 2026-10-09-ci-on-linux-alone — the `a11y` job runs on ubuntu-22.04 alone
**Section:** §3 → CI integration (the key file `registries/contracts/a11y-plan/ci-integration.md`, Pipeline integration) · §9 CI Integration → Pipeline integration
**Change:** Was: "their own `a11y` matrix job (Linux/macOS/Windows, blocking, parallel with the test jobs)" with artifacts "under per-OS names", and in §9 "its own `a11y` matrix job". Now: one `a11y` job on `ubuntu-22.04` alone, blocking and parallel with the test jobs; its artifacts are `a11y-violations-Linux` and `playwright-a11y-report-Linux`.
**Why:** Chunk 2026-10-09-ci-on-linux-alone removed the job's three-system matrix.
**Kept:** The key file's per-PR regression detection (the `a11y-violations-base` download has no producer in any workflow) and §9's `Lint | eslint-plugin-jsx-a11y` stage (no workflow runs an ESLint step): both found standing, neither amended here, their owner a route matter.
**Ref:** .andromeda/runs/2026-10-09T15-11-06Z-wrap/
