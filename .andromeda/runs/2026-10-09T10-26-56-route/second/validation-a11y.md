# A11y validation — route draft

## No suggestions
Draft covers the a11y domain by retirement: every surface, path and harness in a11y-plan (§1 through §11, read whole, plus the §3 key file "Bootstrap phases (derive for route / setup-project)") belongs to the 0.3.0 window, tray or their OS toast, and P-083 removes that subject this version, so no a11y chunk is owed and none should be added.

- **Bootstrap items present:** none as install/wire chunks, and none should be inserted — all are already installed from earlier versions and all serve only the retired desktop-webview surface. The plan's key file lists nine items, two more than the focus guide's seven (a11y-linting-install, motion-tokens-respect-install); all nine leave in Epoch 2:
  - a11y-ci-gate-wire, violation-json-emission-wire → chunk "Window's gates retired" (epoch 2), which names the a11y CI job (a11y-plan §9; today the `a11y` job in `.github/workflows/ci.yml`).
  - contrast-verification-harness-setup, screen-reader-test-spec-setup → chunk "Window's gates retired" (epoch 2), as part of the webview suite (a11y-plan §1 harness specification).
  - focus-management-library-install, aria-component-library-install, motion-tokens-respect-install → chunk "Window retired" (epoch 2), with the webview interface they are compiled into.
  - a11y-tooling-install, a11y-linting-install → chunk "Desktop distribution retired" (epoch 2), with the npm tree (the packages sit in `pulse-app/ui/package.json`).
- **Sequencing deps satisfied:**
  - Gate before surface → confirmed: "Window's gates retired" precedes "Window retired", so the a11y-plan §9 job never audits a surface that is already gone and the §10 zero-violation gate is retired as stated work, not left red or skipped (§11 CI anti-patterns).
  - Gate before its tooling → confirmed: "Window's gates retired" precedes "Desktop distribution retired", so the job leaves before the npm packages it invokes.
  - Gate still standing through Foundation → confirmed: "CI on Linux alone" (epoch 1) trims only the a11y matrix's Windows and macOS legs (P-113); the Linux leg keeps a11y-plan §10 enforced on the window until Epoch 2 removes both.
  - a11y-tooling-install before any chunk using it; focus + aria installs before UI feature chunks; contrast harness before the full gate; a11y-ci-gate reachability in Foundation → not applicable: Epochs 1 and 3–9 build a headless engine with no chunk that renders markup, takes focus or uses a11y tooling.
- **Coverage:**
  - Must-be-accessible paths P1–P14 (a11y-plan §1 critical paths and surface-set extension, §5 Keyboard Navigation) → all live on the window, the tray, or the separate findings and report windows; retired with the surface under P-083, so no feature chunk and no Polish verification chunk is owed; "Version close on Linux" correctly carries no a11y gate and no SC list.
  - Per-surface a11y verification → no surface with a11y tooling reach remains after Epoch 2 (a11y-plan §1 reach table).
  - Master state → "Window retired" has the a11y master state no interface this version, matching P-083's wording; "Capability record re-based" (epoch 1) records the window's a11y capabilities as retired with their surface rather than regressed.
  - Successors of two a11y-plan §1 assertable entities (tray icon status display; OS notification) → "State in one line for a desktop panel" and "System notification on a change of state" (epoch 8, P-099) carry the state as text by their own wording, so the §1 visual-discrimination trigger holds by construction; a11y-plan §1 records no automated tool reach for OS notifications and §2 forbids a manual-only gate, so no verification chunk is suggested for either.
