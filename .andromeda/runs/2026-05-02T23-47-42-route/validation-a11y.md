# A11y validation — route draft

## Insert

- Between `Design tokens bundle` and `A11y dev stack install` (Epoch 1 — Foundation): **"Contrast verification harness — design tokens + colorjs.io + per-pair JSON emission"** (epoch: `Foundation`)
  Reason: per a11y plan §3.5 Bootstrap item `contrast-verification-harness-setup`, contrast harness must be wired as distinct phase before any UI development that depends on token verification.

- Between `A11y dev stack install` and `OTLP gRPC receiver` (Epoch 1 → Epoch 2): **"A11y screen reader test spec scaffold — NVDA/VoiceOver/Orca fixtures + Guidepup integration"** (epoch: `Foundation`)
  Reason: per a11y plan §3.5 Bootstrap item `screen-reader-test-spec-setup`, SR test spec must be scaffolded before feature epochs so manual verification patterns are available from project start.

- Between `A11y dev stack install` and `OTLP gRPC receiver` (Epoch 1 → Epoch 2): **"Motion tokens library install — @rive-app/react or motion/react + useReducedMotion hook + Tailwind motion-reduce: variants"** (epoch: `Foundation`)
  Reason: per a11y plan §3.5 Bootstrap item `motion-tokens-respect-install`, motion token wiring must land in Foundation alongside other design token consumption libraries to support prefers-reduced-motion override across all features.

- Between `Smoke tests + tauri-driver matrix` and `Release pipeline + signing automation` (Epoch 8 — Polish & ship): **"A11y CI gate — npm test:a11y commands matrix + cross-platform keyboard testing via Playwright"** (epoch: `Polish & ship`)
  Reason: per a11y plan §3.5 Bootstrap item `a11y-ci-gate-wire`, CI gate commands must be wired explicitly before signing automation so a11y gates can block PRs per Polish epoch sequencing (a11y-ci-gate-wire precedes release pipeline per a11y plan §3 Sequencing dependencies).

- Between `A11y CI gate` and `Release pipeline + signing automation` (Epoch 8 — Polish & ship): **"A11y violation JSON emission wire — structured violation JSON per service identity + jq tagging + artifact upload"** (epoch: `Polish & ship`)
  Reason: per a11y plan §3.5 Bootstrap item `violation-json-emission-wire`, JSON emission must be wired as separate phase after CI gate (harness consumes JSON from gate output) to align with obs plan Section 6 log format binding and regression detection per a11y plan §3 CI Integration.

## Rewrite

- `A11y dev stack install`: change "axe-core/playwright + Lighthouse + pa11y + react-aria-components + focus-trap-react + tabbable + colorjs.io + eslint-jsx-a11y" → "axe-core/playwright 4.11.x + Lighthouse 12.x + pa11y 9.x + react-aria-components 1.17.x + focus-trap-react 12.x + tabbable 6.4.x + eslint-plugin-jsx-a11y 6.10.x"
  Reason: a11y plan §3 Testing Tool Pick specifies exact versions for Standard tier WCAG 2.1 AA compliance; version pinning required for audit trail and regression detection.

- `A11y audit + perf SLO + violation-JSON gates`: change "WCAG 2.1 AA (axe/Lighthouse/pa11y) + SC 2.3.3 AAA reduced-motion + 10k spans/sec ≥30 fps + regression detection" → "WCAG 2.1 AA (axe-core + Lighthouse + pa11y) + SC 2.3.3 AAA reduced-motion respect (prefers-reduced-motion: reduce emulation) + 10k spans/sec ≥30 fps SLO + per-surface violation JSON regression detection"
  Reason: a11y plan §3 CI Integration specifies explicit regression detection mechanism (`compare current PR violations vs baseline using jq` on `{surface, wcag_criterion, selector, severity}` tuples) and motion-sensitive trigger requires explicit `prefers-reduced-motion` emulation harness, not just token presence.
