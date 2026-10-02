
## 2026-09-29-p-025-hue-shift-observable-made-gradable — npm residuals: GHSA-ggr8 pruned, GHSA-7pqw accepted
**Section:** §Dependency Security → npm channel (`pulse-app/ui`) → current state
**Change:** Was residual roots GHSA-ggr8-5vv4-36mx (deepmerge-ts, "pinned <8 by the entire webdriverio 9 line") and GHSA-jmr9-qjv8-65gv (extract-zip). Now both advisory exceptions are extract-zip — GHSA-jmr9 and GHSA-7pqw-9j4j-h8q3 (range `*`, 2.0.1 the latest release, no fixed release anywhere; npm's remedy a rejected pa11y-ci downgrade; dev-only via the puppeteer chains) — and GHSA-7pqw is the one residual accepted at this chunk. GHSA-ggr8 was pruned: its closing condition fired (webdriverio 9.32.0 brings deepmerge-ts 8.0.2). The chunk's dev-only bumps are recorded: vitest 4.1.11 (GHSA-82fw), qs 6.16.0, undici 6.29.0, webdriverio 9.32.0. Exception counts unchanged (2 advisory + 2 license).
**Why:** the no-safe-upgrade class is exactly what the exception form exists for; the operator relay named GHSA-7pqw the one accepted residual.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
