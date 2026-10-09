### Bootstrap phases (derive for route / setup-project)

The downstream skills derive the following bootstrap phases from the contract above:

- **a11y-tooling-install:** `npm install --save-dev @axe-core/playwright@4.11.x lighthouse@13.x pa11y@10.x pa11y-ci@4.x` (lighthouse 12→13 + pa11y 9→10 as of chunk `2026-08-30-npm-advisory-coverage`, advisory-driven majors per the operator ruling) + configure axe with `runOnly: { type: 'tag', values: ['wcag2a', 'wcag2aa', 'wcag21aa', 'wcag22aa'] }` for Standard tier WCAG 2.1 AA + WCAG 2.2 AA target-size support (SC 2.5.8)
- **focus-management-library-install:** `npm install focus-trap-react@12.x tabbable@6.4.x` (focus-trap-react for modals; tabbable for focus order ground-truth)
- **aria-component-library-install:** `npm install react-aria-components@1.17.x @headlessui/react@2.2.x` (semantic HTML + ARIA; recommend React Aria for 32 components; Headless UI for Tailwind v4 integration)
- **contrast-verification-harness-setup:** `npm install --save-dev colorjs.io@0.6.x` + scaffold Playwright test reading design tokens via `getComputedStyle` + colorjs.io contrast() algorithm
- **screen-reader-test-spec-setup:** scaffold per-surface SR test spec files (a11y-sr-nvda.md / a11y-sr-voiceover.md / a11y-sr-orca.md) per Screen reader test pattern; structured JSON output per manual pass
- **a11y-linting-install:** `npm install --save-dev eslint-plugin-jsx-a11y@6.10.x` + extend ESLint config with `'plugin:jsx-a11y/recommended'` (compile-time gate)
- **motion-tokens-respect-install:** `npm install motion@12.x` + wire `useReducedMotion` hook from `motion/react` into canvas frame loop and animation triggers; apply Tailwind v4 `motion-reduce:` variants to CSS transitions
- **a11y-ci-gate-wire:** integrate `npm run test:a11y` into GitHub Actions `ci.yml` (reuse existing `run` step with E2E driver)
- **violation-json-emission-wire:** emit structured violation JSON per a11y-scope Section 3 schema; upload as CI artifact

---
