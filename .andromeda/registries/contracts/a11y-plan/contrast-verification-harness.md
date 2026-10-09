### Contrast verification harness

- **Source-of-truth tokens:** Verbatim from upstream-context Section 3 A11y-Relevant Design Tokens:
  - `--color-text-primary / --color-base` → 4.5:1 (SC 1.4.3 AA)
  - `--color-text-secondary / --color-base` → 3:1 (SC 1.4.3 AA large text)
  - `--color-text-tertiary / --color-base` → 3:1 (SC 1.4.3 AA large text only)
  - `--color-primary / --color-base` → focus indicator; ≥ 3:1 non-text (SC 1.4.11)
  - `--color-feedback-success / --color-inset` → 4.5:1 for text (SC 1.4.3) or 3:1 for non-text (SC 1.4.11)
  - `--color-accent / --color-base` → error state; 4.5:1 for text (SC 1.4.3) or 3:1 for non-text (SC 1.4.11)
  - `--border-focus` → focus ring; ≥ 3:1 contrast against background (SC 1.4.11)

- **Verification tool:** 
  - **axe-core color-contrast rule** (default, covers SC 1.4.3 / 1.4.6 / 1.4.11)
  - **colorjs.io** 0.6.x custom token-based checker reading design tokens directly via `window.getComputedStyle().getPropertyValue('--color-token-name')` + `Color.contrast(fg, bg, 'WCAG21')` algorithm
  - Emits machine-readable PASS/FAIL per token pair with WCAG SC reference

- **WCAG SC mapping:** SC 1.4.3 Contrast Minimum (AA: 4.5:1 normal text / 3:1 large text) + SC 1.4.6 Contrast Enhanced (AAA: 7:1 / 4.5:1 — not required Standard tier) + SC 1.4.11 Non-text Contrast (AA: 3:1 for UI components and graphical objects)
