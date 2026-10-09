
## 2026-09-29-ci-wall-time-and-round-trips — the a11y gate as its own CI job
**Section:** §3 CI integration → Pipeline integration · §9 Pipeline integration
**Change:** the a11y assertions run as their own `a11y` matrix job (Linux/macOS/Windows, blocking, parallel with the test jobs): `npm run build --prefix pulse-app/ui` → Playwright chromium → the PR-only `a11y-violations-base` download → `cargo xtask test:a11y`, artifacts uploaded `if: always()` under per-OS names (was: steps inside the shared lint/test/build job); the harness contract shared with tests is unchanged.
**Why:** ci.yml split into seven jobs to cut round wall time; the a11y ↔ tests binding holds.
**Ref:** .andromeda/runs/2026-09-29T21-44-34Z-wrap/
