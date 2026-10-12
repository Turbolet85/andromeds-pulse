# A11y validation — route draft

## No suggestions
Draft covers the a11y domain by retirement: a11y-plan describes the 0.3.0 window, and requirement P-083 (intent R1, ruled) removes that window, its audits and the a11y master's subject this version, so no a11y chunk is owed.

- **Bootstrap items present:** none, and none should be inserted. All nine items in a11y-plan §3 contract "Bootstrap phases (derive for route / setup-project)" serve only the desktop-webview surface, which P-083 retires: a11y-tooling-install, focus-management-library-install, aria-component-library-install, contrast-verification-harness-setup, screen-reader-test-spec-setup, a11y-linting-install, motion-tokens-respect-install, a11y-ci-gate-wire, violation-json-emission-wire. They were installed in earlier versions and leave in Epoch 2 via "Window's gates retired" (the a11y CI job, i.e. a11y-ci-gate-wire and violation-json-emission-wire, per a11y-plan §9) and "Window retired" (the remaining tooling, libraries and specs, with the webview interface).
- **Sequencing deps satisfied:**
  - Gate before surface → confirmed: "Window's gates retired" precedes "Window retired" in Epoch 2, so the a11y-plan §9 a11y matrix job never runs against a surface that is already gone, and the §10 zero-violation gate is never left red or silently skipped.
  - a11y-ci-gate reachability in Foundation → not applicable: Epoch 1 builds a headless engine with no surface for a11y tooling to reach, and P-083 states that no gate audits a window.
  - Focus and ARIA library installs before UI feature chunks → not applicable: no chunk in Epochs 3–8 builds a UI surface.
- **Coverage:**
  - Must-be-accessible paths P1–P14 (a11y-plan §1 critical paths and surface-set extension, §5 Keyboard Navigation) → all on the window, the tray or the separate findings and report windows; retired with the surface under P-083, so no feature chunk or Polish verification chunk is owed and none appears.
  - Per-surface a11y verification → no surface with a11y tooling remains; "Version close on Linux" correctly carries no a11y gate or SC list.
  - Master state → "Window retired" already has the a11y master state no interface this version, which matches P-083's own wording; no rewrite needed.
  - Successors of two a11y-plan §1 assertable entities (tray icon status display; OS notification) → "State in one line for a desktop panel" and "System notification on a change of state" (Epoch 7, P-099) both carry the state as text by their own wording, so the §1 visual-discrimination trigger (state not by colour alone) and the status-message intent of the P3/P6 rows hold by construction. P-099 rules this surface as the founder's own desktop only, and a11y-plan §1 records no automated tool reach for OS notifications, so no verification chunk is suggested.
