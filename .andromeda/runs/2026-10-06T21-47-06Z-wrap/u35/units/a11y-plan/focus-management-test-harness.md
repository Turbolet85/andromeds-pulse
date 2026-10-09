### Focus management test harness

- **Driver:** **Playwright** 1.49.x with **@axe-core/playwright** 4.11.x + **tabbable** 6.4.x focus order ground-truth computation
- **Pattern:** 
  - Scripted Tab / Shift+Tab traversal covering all interactive elements per layout
  - Focus trap entry / exit verification: Esc closes modal; focus restores to triggering element
  - Focus restoration on modal close: trigger button receives focus
  - Focus order assertion: `tabbable(container)` computes expected sequence; `page.keyboard.press('Tab')` actual sequence; assert match
  - **Contract binding:** reuses tests' E2E driver per upstream-context Section 5 Test Harness (5-command discipline: `boot`, `run`, `status`, `cleanup`, `logs`)
