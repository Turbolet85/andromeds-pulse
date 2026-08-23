# A11y Summary

_Distilled from `.andromeda/a11y-plan.md` by `/setup-project`. Read on demand. **Tier: Standard (1)** — WCAG 2.1 AA full + AAA escalation **SC 2.3.3 Animation from Interactions** (motion-sensitive trigger)._

## Why Standard tier
Cross-platform desktop app (Windows / macOS / Linux via Tauri 2.x) with 2 primary UI surfaces (desktop-webview React SPA + desktop-native tray menu) plus an OS notification surface. 8 assertable entities, 7 must-be-accessible flows. Creator brief explicit rigor signals: WCAG color discipline ("not-color-alone"), reduced-motion respect, "glance-readable from 2 meters" typography legibility, motion-as-data principle.

## A11y testing tool pick (binding)
| Surface | Primary tool | Secondary / fallback |
|---|---|---|
| **desktop-webview (React UI)** | `@axe-core/playwright` 4.11.x for E2E + Lighthouse 12.x CLI for CI gate + pa11y 9.x for parallel rule matrix | Manual SR pass (NVDA / VoiceOver / Orca) supplemental |
| **desktop-native (tray-icon menu)** | No automated tool reach; OS-native a11y APIs (Windows UIA / macOS NSAccessibility / Linux ATK) | Manual SR + keyboard pass required |
| **OS notification (toast)** | No automated tool reach (OS-level API) | Manual SR pass via NVDA / VoiceOver / Orca |

## WCAG criteria (Standard tier)
- **Baseline:** WCAG 2.1 AA full (~50 SCs). axe-core config: `runOnly: { type: 'tag', values: ['wcag2a', 'wcag2aa', 'wcag21aa', 'wcag22aa'] }` (adds SC 2.5.8 target-size from WCAG 2.2 AA).
- **AAA escalation (motion-sensitive trigger):** **SC 2.3.3 Animation from Interactions** — Lighthouse 12.x prefers-reduced-motion audit + custom Playwright `page.emulateMedia({ reducedMotion: 'reduce' })` assertion.
- **Explicitly N/A SCs (12 with documented reasons):** SC 1.2.1/1.2.2/1.2.3/1.4.2 (no audio), 1.4.4/1.4.5 (no text-as-image), 1.4.10 (desktop-only viewport), 1.4.13 (re-evaluated chunk #99: the header connection-dot tooltip is a non-interactive `role="img"` summary with an `aria-hidden` decorative tooltip — no hover/focus-triggered ADDITIONAL content in the SC sense; still N/A), 2.2.1/2.2.2 (no time-dependent content), 2.4.4 (verified via 4.1.2), 3.1.1 (single-language), 3.2.1/3.2.2 (covered in P7), 3.3.3/3.3.4 (covered via P2/P7), 4.1.1 (parsing — deprecated WCAG 2.2; React 19 + TS + ESLint enforce).

## Critical paths (P1–P12 must-be-accessible)

_Extended from P1–P7 to **P1–P12** at chunk #99: **p8** findings dropdown · **p9** diagnostic report modal · **p10** diagnostics view · **p11** constellation semantics · **p12** export preview — each with its own axe spec plus `keyboard-focus/widget-and-modals.spec.ts`. The universal minimums (SC 2.1.1 / 2.4.3 / 4.1.2) and the §10 SLO coverage apply across all twelve. The P1–P7 rows below are the original set; p8–p12 follow the same role/focus/SC discipline._

| Path | Required ARIA roles | Required focus order | Required WCAG SC |
|---|---|---|---|
| **P1** Receive OTLP + visualize | `main` / `button` ship. `region[aria-label="Telemetry traces chart"]` + `table` are **required but NOT SHIPPED** (measured absent at HEAD 2026-08-23; `TraceTable.tsx` is `data-testid`-only) — owner: the A11y verification entry | Initial focus main content region via the `data-testid` trace list; cell-level Tab/arrow navigation presumes the unshipped `table` role and is not assertable yet | 1.3.1 / 2.1.1 / 2.4.3 / 1.4.3 / 4.1.2 |
| **P2** Generate snapshot | `main` / `dialog[aria-label="Investigation Snapshot"]` / `button[aria-busy]` / `slider` / `radiogroup` / `status[aria-live="polite"]` | Generate → budget slider → preset → Generate; on submit → progress message | 2.1.1 / 2.4.3 / 3.3.1 / 3.3.2 / 4.1.2 / 4.1.3 |
| **P3** MCP toggle in settings | `dialog[aria-label="Settings"]` / `switch[aria-checked]` + OS notification | Tab to MCP switch → Space toggles → focus stays on switch | 2.1.1 / 2.4.3 / 4.1.2 / 4.1.3 |
| **P4** Real-time push | `region[aria-live="polite"]` / `status` / `table` (incremental rows) | Live updates do NOT steal focus | 2.1.1 / 2.4.3 / 4.1.2 / 4.1.3 / 1.4.3 |
| **P5** Widget ↔ dashboard ↔ tray | `main` / `button[aria-label="Toggle dashboard"]` (opens/hides dashboard, widget stays; also Cmd/Ctrl+Shift+P) / `button[aria-label="Minimize"]` / `button[aria-label="Close to tray"]` (titlebar ✕ → hides widget to tray; Esc also triggers) / tray `menuitem` | Esc minimizes widget; arrow keys nav tray menu; Tab nav dashboard sidebar | 2.1.1 / 2.1.2 / 2.4.3 / 2.4.7 / 4.1.2 |
| **P6** Snapshot completion | `dialog` / `button[aria-busy]` / `status[aria-live="assertive"]` + OS notification | Focus stays in modal during async; completion announced | 2.1.1 / 2.4.3 / 4.1.2 / 4.1.3 |
| **P7** Settings form | `dialog` / `form` / `label` / `radiogroup` / `switch` / `button` | Tab theme→position→retention→MCP→preset→Save→Cancel; Esc closes | 2.1.1 / 2.1.2 / 2.4.3 / 2.4.7 / 3.3.1 / 3.3.2 / 4.1.2 |

## Triggers (escalations + custom assertions)
| Trigger | Source | Required assertion |
|---|---|---|
| **visual-discrimination** | Creator brief: "Status colors not-color-alone (paired with iconography per WCAG)" | axe-core color-contrast rule per token pair + custom assertion verifying icon/label supplement (SC 1.4.1 + 1.4.3 + 1.4.11) |
| **motion-sensitive** | Creator brief: "respects `prefers-reduced-motion`"; motion-as-data principle | Lighthouse prefers-reduced-motion audit + Playwright `emulateMedia({ reducedMotion: 'reduce' })`; **escalates to AAA SC 2.3.3** |
| **keyboard-only** | Layout templates focus management anchors | Playwright Tab/Shift+Tab/Esc/Arrow per layout; focus trap entry/exit; SC 2.1.1 + 2.1.2 + 2.4.7 |
| **screen-reader-priority** | "Glance-readable from 2 meters" + accessibility-conscious design signals | Manual SR pass (NVDA/VoiceOver/Orca) per critical path; SC 4.1.2 |
| **visual-focus-appearance** | Layout templates require `--border-focus` token outline | Lighthouse focus-visible audit + Playwright outline computed-style check; SC 2.4.7 + 1.4.11 |
| **target-size** | Compact widget design implies small form factors | axe-core target-size rule + design token `--target-button-min` / `--target-input-min` ≥24×24 (SC 2.5.8 AA) |

## Bootstrap phases (a11y-plan §3.5)
1. **a11y-tooling-install:** `@axe-core/playwright@4.11` + `lighthouse@12` + `pa11y@9` + `pa11y-ci@4`
2. **focus-management-library-install:** `focus-trap-react@12` + `tabbable@6.4`
3. **aria-component-library-install:** `react-aria-components@1.17` (preferred) OR `@headlessui/react@2.2` — pick ONE; mixing causes conflicts
4. **contrast-verification-harness-setup:** `colorjs.io@0.6` + Playwright test reading design tokens via `getComputedStyle`
5. **screen-reader-test-spec-setup:** scaffold `a11y-sr-{nvda|voiceover|orca}.md` per Screen reader test pattern
6. **a11y-linting-install:** `eslint-plugin-jsx-a11y@6.10` + extend `'plugin:jsx-a11y/recommended'`
7. **motion-tokens-respect-install:** `motion@12` + wire `useReducedMotion` hook into canvas frame loop + Tailwind v4 `motion-reduce:` variants
8. **a11y-ci-gate-wire:** integrate `npm run test:a11y` into GitHub Actions `ci.yml` (reuse existing E2E driver)
9. **violation-json-emission-wire:** emit structured violation JSON per a11y-plan §3 schema; upload as CI artifact

## Structured violation JSON (binding to obs §6)
```json
{
  "timestamp": "…",
  "level": "ERROR",
  "target": "a11y::assertion",
  "message": "WCAG SC 1.4.3 Contrast violation detected",
  "fields": {
    "wcag_criterion": "SC 1.4.3",
    "violation_type": "color-contrast",
    "severity": "critical",
    "surface": "desktop-webview",
    "selector": ".settings-modal form input[type='text']",
    "actual_contrast_ratio": 3.2,
    "required_ratio": 4.5,
    "remediation": "Increase foreground darkness to meet 4.5:1; use --color-text-primary token",
    "tool": "axe-core",
    "tool_result_id": "color-contrast-rule-12345"
  }
}
```
Emitted to `~/.andromeda-pulse/logs/a11y-{tool}-results.jsonl`; uploaded as CI artifact.

## SLO invariants
- **Zero WCAG AA violations** on must-be-accessible paths.
- **Zero new violations per PR** — per-PR diff detection vs base branch (`actions/download-artifact@v4` → `jq` filter on `{surface, wcag_criterion, selector, severity}` tuples).
- **Performance budget per a11y CI run:** axe-core <30s per surface; Lighthouse a11y <15s per URL; total a11y CI + tests E2E <10 min.

## CI gate (PR cannot merge if)
- axe-core reports critical/serious WCAG violation on desktop-webview
- Lighthouse a11y category score <90 on any URL
- Keyboard focus order test fails
- Contrast verification detects token mismatch (actual <required)
- Per-PR regression — any new tuple `{surface, wcag_criterion, selector, severity}` not in base branch

## v0.2.0 re-audit (chunk #99, per a11y-plan §12 2026-06-10)
- **Harness repaired** — the Playwright a11y suite was silently dead since session 64 (4 infra bugs: stale root-level `../helpers/` import; IPC mock matched `plugin:taurpc|` while taurpc 0.7 invokes `TauRPC__<router.path>`; window-label global never read — production reads `__TAURI_INTERNALS__.metadata.currentWebview.label`; `/#/route` URLs never resolved — TanStack Router uses browser history). Repairs: `helpers/mock-tauri.ts` rewrite + `helpers/v02-fixtures.ts` + `helpers/static-server.mjs` SPA fallback + `pa11y/run-pa11y.mjs` + real route paths in pa11y/Lighthouse configs.
- **Spec coverage extended** — axe specs p1–p7 → **p1–p12** (findings dropdown / diagnostic report modal / diagnostics view / constellation semantics / export preview) + `keyboard-focus/widget-and-modals.spec.ts`; p5 + reduced-motion updated to the redesigned widget. New-surface discipline: every new route/modal chunk adds an axe spec + `v02-fixtures.ts` payloads in the same chunk.
- **~10 violations remediated in-chunk** — SC 2.5.8 target-size (WindowControls 24px), SC 1.4.3/1.4.11 contrast (tertiary→secondary text; severity text→primary + severity borders), svg-img-alt, aria-valid-attr-value (TabNav dangling `aria-controls`), SkipToMain clip pattern.
- **Baseline re-established 2026-06-09** — `baselines/a11y-violations-summary.json` from a clean full-chain pass (6 informational lighthouse-score tuples, 0 violation tuples); suite health checked cheaply via `npx playwright test --list`.

## Top anti-patterns (a11y-plan §11)
- NEVER use ARIA on non-semantic HTML (`role="button"` on `<div>` instead of `<button>`).
- NEVER overuse `aria-label` (visible `<label>` first).
- NEVER set `tabindex > 0` (creates non-natural focus order).
- NEVER `outline: none` without alternative `:focus-visible`.
- NEVER hide important content from assistive tech (`aria-hidden="true"` on essential content).
- NEVER convey information by color alone — pair `--color-accent`/`--color-feedback-success`/`--color-secondary` with icon + text label.
- NEVER autoplay motion without `prefers-reduced-motion: reduce` respect.
- NEVER flash content >3 times per second.
- NEVER claim WCAG conformance without machine-verifiable evidence.
- NEVER mix React Aria Components + Headless UI in same app.
- NEVER skip a11y on CI failure (FAIL-fast, not warning).

Full plan: `.andromeda/a11y-plan.md`.
