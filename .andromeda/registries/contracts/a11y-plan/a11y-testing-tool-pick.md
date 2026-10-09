### A11y testing tool pick

- **Primary tool per surface:** **@axe-core/playwright** 4.11.x for desktop-webview E2E + **Lighthouse** 13.x CLI for CI gate + **pa11y** 10.x for parallel rule matrix
- **Configuration:** 
  - axe-core: `runOnly: { type: 'tag', values: ['wcag2a', 'wcag2aa', 'wcag21aa'] }` for Standard tier baseline
  - axe-core: add `'wcag22aa'` to include WCAG 2.2 AA target-size rule (SC 2.5.8)
  - Lighthouse 13.x: built-in a11y category includes focus-visible + prefers-reduced-motion audits (SC 2.4.7 + SC 2.3.3 AAA trigger coverage)
  - pa11y 10.x: `--runner axe` for full axe ruleset coverage parallel to standalone axe-core
  - eslint-plugin-jsx-a11y 6.10.x: extend `'plugin:jsx-a11y/recommended'` in ESLint config (compile-time gate for ARIA on non-semantic HTML + missing labels)
