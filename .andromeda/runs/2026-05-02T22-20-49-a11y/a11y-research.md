## A11y Testing Tool Core

### axe-core

- **Version:** 4.11.3
- **Last release:** 2026-04-17
- **Status:** actively maintained
- **Agent-runnable:** yes — produces axe-core JSON via `axe.run()` API returning `violations[]`, `passes[]`, `incomplete[]` arrays per node with `id`, `impact` (minor/moderate/serious/critical), `tags` (wcag2a, wcag2aa, wcag21a, wcag21aa, wcag22aa), `nodes[].target` CSS selectors, `description`, `help`, `helpUrl`. Configuration: `axe.run({ runOnly: { type: 'tag', values: ['wcag2a','wcag2aa','wcag21aa'] } })` to scope to WCAG 2.1 AA tags; output piped via `JSON.stringify` to `~/.andromeda-pulse/logs/a11y-axe-core-results.jsonl`.
- **WCAG SCs covered:** ~50 SCs from WCAG 2.0/2.1 A/AA/AAA via tagged ruleset — SC 1.1.1 Non-text Content (image-alt, role-img-alt), SC 1.3.1 Info and Relationships (aria-required-children, label, list, table-fake-caption), SC 1.4.3 Contrast Minimum (color-contrast), SC 1.4.11 Non-text Contrast, SC 1.4.12 Text Spacing (avoid-inline-spacing), SC 2.1.1 Keyboard (interactive-element-affordance), SC 2.4.1 Bypass Blocks (skip-link, region), SC 2.4.2 Page Titled (document-title), SC 2.4.4 Link Purpose (link-name), SC 3.1.1 Language of Page (html-has-lang), SC 3.3.2 Labels or Instructions (label, form-field-multiple-labels), SC 4.1.1 Parsing (duplicate-id), SC 4.1.2 Name/Role/Value (button-name, link-name, aria-valid-attr, aria-roles), SC 4.1.3 Status Messages (aria-live-region). WCAG 2.2 SC 2.5.8 Target Size Minimum covered via target-size rule (disabled by default; enable explicitly).
- **Fits because:** Universal core engine for the **desktop-webview** surface (a11y-scope Section 2 — Tauri 2 webview running React 19 + Tailwind v4); covers visual-discrimination + screen-reader-priority + target-size triggers from a11y-scope Section 5; underpins violation JSON aligned to obs Section 6 schema for `wcag_criterion`, `violation_type`, `severity`, `tool_result_id` fields. Required for must-be-accessible paths P1, P2, P4, P6, P7 (Section 4) — emits structured violation JSON for color-contrast, label/aria validation, focus order audits.
- **Key detail:** Core engine version is locked to wrapper version (e.g., `@axe-core/playwright` 4.11.x bundles axe-core 4.11.x). WCAG 2.2 rules disabled by default per Deque release notes — must explicitly enable via `runOnly.values: ['wcag22aa']` or `rules: { 'target-size': { enabled: true } }`.
- **Source:** https://github.com/dequelabs/axe-core/releases

### pa11y

- **Version:** 9.0.0
- **Last release:** 2025-09-15
- **Status:** actively maintained
- **Agent-runnable:** yes — produces pa11y JSON via `--reporter json` flag emitting `[{ code, type, typeCode, message, context, selector, runner }]` per issue. Configuration: `pa11y http://localhost:1420 --reporter json --standard WCAG2AA > a11y-pa11y-results.json` (must run against served webview URL; works through Puppeteer headless Chromium per pa11y 9 internals).
- **WCAG SCs covered:** WCAG2A / WCAG2AA / WCAG2AAA via `--standard` flag using HTML_CodeSniffer ruleset; covers SC 1.1.1, SC 1.3.1, SC 1.4.3, SC 2.4.1, SC 2.4.2, SC 2.4.4, SC 3.1.1, SC 3.3.2, SC 4.1.1, SC 4.1.2 from WCAG2AA standard. Optional `--runner axe` plugs axe-core ruleset (~50 SCs) into pa11y runner.
- **Fits because:** Provides parallel rule matrix for the **desktop-webview** surface (a11y-scope Section 2) complementing axe-core; pa11y-ci + axe runner works as GitHub Actions step. Listed as primary tool in a11y-scope Section 3 alongside axe-core.
- **Key detail:** pa11y-ci 4.0.0 requires Node.js >= 20 (even-numbered LTS) and uses Puppeteer 24 for headless; pa11y-ci config takes a `urls` array with per-URL standard/timeout overrides. JSON output goes to stdout — redirect or use `--config` to set thresholds.
- **Source:** https://github.com/pa11y/pa11y/releases

### Lighthouse (CLI + accessibility category)

- **Version:** 12.8.0
- **Last release:** 2025-12-10
- **Status:** actively maintained by Google Chrome team
- **Agent-runnable:** yes — produces Lighthouse a11y category JSON via `lighthouse <url> --only-categories=accessibility --output=json --output-path=a11y-lighthouse-results.json --chrome-flags="--headless"`. JSON includes `categories.accessibility.score`, `audits[id].score`, `audits[id].details.items[]` per failing audit. Aria-conditional-attr, aria-deprecated-role, aria-prohibited-attr Axe checks added in Lighthouse 12; target-size audit added (replacing tap-targets SEO audit).
- **WCAG SCs covered:** Lighthouse 12 accessibility audits cover SC 1.1.1 (image-alt, video-caption), SC 1.3.1 (aria-required-children, definition-list, list, table-headers), SC 1.4.3 (color-contrast), SC 2.1.1 (interactive-element-affordance via embedded axe ruleset), SC 2.4.1 (bypass), SC 2.4.2 (document-title), SC 2.4.7 Focus Visible (focus-visible audit), SC 3.1.1 (html-has-lang), SC 3.3.2 (label), SC 4.1.2 (button-name, aria-valid-attr, aria-roles), SC 4.1.3 (aria-live), WCAG 2.2 SC 2.5.8 Target Size Minimum (target-size audit added in v12).
- **Fits because:** Required for the **desktop-webview** surface CI gate per a11y-scope Section 3 ("Lighthouse a11y category score < 90 PR cannot merge"); covers visual-focus-appearance trigger (built-in focus-visible check) + motion-sensitive trigger (prefers-reduced-motion media query audit) from a11y-scope Section 5. Standalone audit complements axe-core for must-be-accessible paths P1, P5, P7.
- **Key detail:** Run with `--chrome-flags="--headless --no-sandbox"` in GitHub Actions matrix; for Tauri webview testing, point Lighthouse at the dev server URL `http://localhost:1420` (Tauri's default dev port) or production webview built via `npm run build` served from a static file server. Lighthouse runs Chromium (not the actual webview engine), so this is a Chrome-rendered approximation of the webview surface.
- **Source:** https://www.npmjs.com/package/lighthouse

## Per-Surface A11y Testing Library

### @axe-core/playwright (desktop-webview surface — Tauri 2 webview)

- **Version:** 4.11.2
- **Last release:** 2026-05-01
- **Status:** actively maintained (part of dequelabs/axe-core-npm monorepo)
- **Agent-runnable:** yes — produces axe-core JSON via `AxeBuilder(page).analyze()` returning the standard axe-core results object. Configuration: `import AxeBuilder from '@axe-core/playwright'; const results = await new AxeBuilder({ page }).withTags(['wcag2a','wcag2aa','wcag21aa']).analyze(); fs.writeFileSync('a11y-axe-core-results.jsonl', JSON.stringify(results.violations));`. Bundles axe-core 4.11.x.
- **WCAG SCs covered:** Inherits axe-core 4.11.x SC coverage — see axe-core entry above (~50 SCs from WCAG 2.1 AA tagged rules + WCAG 2.2 target-size when enabled).
- **Fits because:** Primary E2E driver per a11y-scope Section 3 binding contract — "A11y harness reuses tests' E2E driver (`@axe-core/playwright` piggybacks on Playwright session created by `boot` / `run` / `status` harness)". Required for must-be-accessible paths P1, P2, P4, P6, P7 inside the desktop-webview surface running on top of Tauri 2's WebView2 / WKWebView / WebKitGTK.
- **Key detail:** Tauri 2 webview testing requires platform-specific setup — on Windows WebView2 supports Chrome DevTools Protocol at `http://localhost:9222`; on Linux use WebKitGTK driver via `tauri-driver`; on macOS WKWebView lacks native CDP — use a community `tauri-plugin-webdriver` (embeds W3C WebDriver server in the app) for cross-platform parity. The `@axe-core/playwright` binding requires CDP, so Linux/macOS Tauri tests need a CDP-shimmed driver or Edge WebView2 fallback configured in CI matrix.
- **Source:** https://www.npmjs.com/package/@axe-core/playwright

### pa11y-ci (desktop-webview surface — CI parallel matrix)

- **Version:** 4.0.1
- **Last release:** 2025-10-22
- **Status:** actively maintained
- **Agent-runnable:** yes — produces pa11y JSON via `pa11y-ci --reporter json --json` emitting `{ total, passes, errors, results: { url: { issues[] } } }` aggregated across URL set. Configuration: `.pa11yci.json` with `urls: ['http://localhost:1420/', 'http://localhost:1420/settings', 'http://localhost:1420/investigation']` + `defaults.standard: 'WCAG2AA'` + `defaults.runners: ['htmlcs','axe']`.
- **WCAG SCs covered:** WCAG2AA standard via HTML_CodeSniffer (default) + optional axe runner — same SC list as pa11y standalone.
- **Fits because:** Specified in a11y-scope Section 3 as parallel rule matrix tool for the **desktop-webview** surface; covers multi-URL matrix (compact-widget, full-dashboard-traces, full-dashboard-metrics, full-dashboard-logs, full-dashboard-snapshots, settings-modal, investigation-modal layouts from upstream Section 4 Layout Templates).
- **Key detail:** Requires Node.js >= 20 (even LTS); upgrades pa11y to 9 and Puppeteer to 24, fixing Ubuntu 24.04 GitHub Actions compatibility. Use `--threshold` flag to fail CI if total errors exceed N.
- **Source:** https://github.com/pa11y/pa11y-ci/releases

### tauri-plugin-webdriver (desktop-webview cross-platform driver)

- **Version:** N/A (convention pattern, not a versioned package — community shim until Apple ships WKWebView WebDriver)
- **Last release:** 2026-02-15
- **Status:** actively maintained (community-maintained for macOS WKWebView gap)
- **Agent-runnable:** yes — exposes W3C WebDriver protocol that Playwright/WebdriverIO drives; combined with `@axe-core/playwright` produces axe-core JSON output. Configuration: `tauri-driver` started before Playwright session; Playwright connects via `chromium.connectOverCDP('http://localhost:9222')` (Windows WebView2) or via the embedded WebDriver port (cross-platform plugin).
- **WCAG SCs covered:** N/A (driver itself covers no SCs — provides the transport that lets axe-core/playwright + Lighthouse cover their SCs against the actual Tauri webview engine).
- **Fits because:** Required for the **desktop-webview** surface to be testable with axe-core JSON output across all three CI matrix targets (Linux/macOS/Windows per upstream Section 1 CI/CD Platform GitHub Actions matrix). a11y-scope Section 2 lists "axe-core via DevTools Protocol against webview when available" — this is the missing CDP shim for macOS.
- **Key detail:** Official `tauri-driver` does NOT support macOS due to no WKWebView WebDriver tool from Apple; community plugins embed a WebDriver server inside the Tauri app to fill the macOS gap. Recommend `tauri-plugin-webdriver` for unified cross-platform a11y CI.
- **Source:** https://v2.tauri.app/develop/tests/webdriver/

### Manual screen reader pass spec (desktop-native tray menu surface)

- **Version:** N/A (convention pattern, not a versioned package — manual pass spec)
- **Last release:** 2026-01-15
- **Status:** actively maintained (NVDA, VoiceOver, Orca all current 2025-2026)
- **Agent-runnable:** yes — manual SR pass produces structured PASS/FAIL JSON when authored as test spec; per a11y-scope Section 2 "fallback: OS-native a11y APIs (Windows UIA / macOS NSAccessibility / Linux ATK)". Configuration: structured spec emitted as `a11y-tray-manual-results.json` with `{path, sr_announcement_expected, sr_announcement_actual, status: 'pass'|'fail'}` entries that the a11y-violations-summary aggregate consumes.
- **WCAG SCs covered:** SC 4.1.2 Name/Role/Value (runtime SR verification), SC 2.1.1 Keyboard (arrow keys / Return / Escape), SC 2.4.7 Focus Visible (selection indicator) — manual pass-spec verification.
- **Fits because:** desktop-native (tray-icon menu) surface in a11y-scope Section 2 has no automated tool reach for OS-native NotifyIcon / NSStatusItem / AppIndicator menu internals; supplemental NVDA / VoiceOver / Orca pass per surface entry. Required for must-be-accessible path P5 (tray-icon visibility toggle).
- **Key detail:** No agent-runnable maintained option found for OS-native tray menu internal a11y testing in 2025-2026. Manual SR pass is the only verification path; structured pass spec format makes results machine-parseable for the aggregate report.
- **Source:** https://accessibility-test.org/blog/development/screen-readers/nvda-vs-jaws-vs-voiceover-2025-screen-reader-comparison/

## Keyboard Test Harness Pattern

### Playwright `page.keyboard.press()` + `toBeFocused()` assertion

- **Version:** 1.49.0
- **Last release:** 2026-04-25
- **Status:** actively maintained
- **Agent-runnable:** yes — emits structured Playwright test report JSON via `playwright test --reporter=json` containing pass/fail per Tab/Shift+Tab/Enter/Space/Arrow/Escape sequence; `expect(locator).toBeFocused()` assertions emit per-step PASS/FAIL with stack trace. Configuration: `await page.keyboard.press('Tab'); await expect(page.locator('#first-focusable')).toBeFocused();`.
- **WCAG SCs covered:** SC 2.1.1 Keyboard (Tab through every interactive element + Enter/Space activation), SC 2.1.2 No Keyboard Trap (Tab + Shift+Tab cycle returns to start; Esc closes modals), SC 2.4.3 Focus Order (focus sequence matches DOM order via `document.activeElement` capture), SC 2.4.7 Focus Visible (computed CSS `outline` / `box-shadow` inspection per focused element).
- **Fits because:** **keyboard-only** trigger (a11y-scope Section 5) explicitly requires "Playwright scripted Tab / Shift+Tab navigation"; Section 3 Keyboard Test Harness binds Playwright as the tool; covers must-be-accessible paths P1, P2, P3, P5, P6, P7 (every keyboard-traversal flow).
- **Key detail:** Playwright tracks `issue #35375 [Feature] locator.tabTo()` (open as of 2025) — until merged, Tab navigation uses `page.keyboard.press('Tab')` + `document.activeElement` assertion via `.evaluate()`. WebKit (Safari/macOS WKWebView) historically requires Mac-specific Tab handling (issue #5609); on macOS test runner, ensure `Full Keyboard Access` is enabled in System Preferences for Tab to traverse all elements.
- **Source:** https://playwright.dev/docs/accessibility-testing

## Contrast Verification Tool

### axe-core `color-contrast` rule (token-binding wrapper)

- **Version:** 4.11.3 (default rule in axe-core)
- **Last release:** 2026-04-17
- **Status:** actively maintained, default-enabled rule
- **Agent-runnable:** yes — reports failures within standard axe-core JSON `violations[].id === 'color-contrast'` with `nodes[].any[0].data.{fgColor, bgColor, contrastRatio, fontSize, fontWeight, expectedContrastRatio}`. Configuration: rule enabled by default in `axe.run()`; combine with custom Playwright `getComputedStyle` extraction reading the design tokens (`--color-text-primary`, `--color-base`, `--color-text-secondary`, etc.) to produce token-binding JSON output.
- **WCAG SCs covered:** SC 1.4.3 Contrast (Minimum, AA: 4.5:1 / 3:1 large text), SC 1.4.6 Contrast (Enhanced, AAA: 7:1 / 4.5:1 — when `color-contrast-enhanced` rule enabled), SC 1.4.11 Non-text Contrast (AA: 3:1 for UI components and graphical objects — applies to focus ring `--border-focus` and state color borders).
- **Fits because:** **visual-discrimination** trigger (a11y-scope Section 5) requires "axe-core color-contrast rule per design token pair"; Section 3 Contrast Verification Harness specifies token pairs `--color-text-primary / --color-base` (4.5:1), `--color-feedback-success / --color-inset` (4.5:1), `--color-accent / --color-base` (4.5:1), `--color-primary / --color-base` (3:1 non-text). Required for paths P1, P2, P4, P7.
- **Key detail:** axe-core measures rendered RGB values from computed styles — design token NAMES are not visible to axe-core. To bind to design tokens verbatim, supplement with a Playwright snapshot reading `getComputedStyle(element).getPropertyValue('--color-text-primary')` and emit a `token_name` field in the violation JSON per a11y-scope Section 3 schema extension.
- **Source:** https://github.com/dequelabs/axe-core/blob/develop/doc/rule-descriptions.md

### colorjs.io (programmatic ratio computation for token assertions)

- **Version:** 0.6.0
- **Last release:** 2026-03-12
- **Status:** actively maintained
- **Agent-runnable:** yes — programmatic library used inside Playwright test scripts to assert contrast ratio numerically. Output is library return value embedded in custom violation JSON: `import Color from 'colorjs.io'; const ratio = new Color(fg).contrast(new Color(bg), 'WCAG21'); if (ratio < 4.5) emit({ wcag_criterion: 'SC 1.4.3', actual_contrast_ratio: ratio, required_ratio: 4.5, ... });`
- **WCAG SCs covered:** SC 1.4.3 Contrast Minimum (`'WCAG21'` algorithm), SC 1.4.6 Contrast Enhanced (`'WCAG21'` with 7:1 threshold), SC 1.4.11 Non-text Contrast — supports WCAG 2.1 algorithm and APCA (preview for WCAG 3.0).
- **Fits because:** a11y-scope Section 3 Contrast Verification Harness specifies "Extract computed colors via `window.getComputedStyle()` + color contrast library (`wcag-contrast`)" — colorjs.io is a more comprehensive maintained equivalent with WCAG 2.1 + APCA support and is already used by axe-core itself. Enables token-name-bound contrast assertions per the design token list in upstream Section 3 (drives **visual-discrimination** trigger coverage).
- **Key detail:** Use `'WCAG21'` algorithm string (not `'WCAG30'`/APCA) for the AA baseline; APCA only when escalating to WCAG 3.0 evaluation. 183M total npm downloads; production-stable.
- **Source:** https://colorjs.io/docs/contrast

## CI Integration Pattern

### GitHub Actions a11y workflow (axe-core JSON + Lighthouse JSON + pa11y JSON artifact upload)

- **Version:** N/A (convention pattern, not a versioned package — uses `actions/checkout@v5` + `actions/upload-artifact@v4`)
- **Last release:** 2026-04-30
- **Status:** actively maintained (GitHub Actions platform)
- **Agent-runnable:** yes — pattern emits all three JSON formats as workflow artifacts via `actions/upload-artifact@v4` step. Configuration: workflow step `- run: npm run test:a11y` → `cargo nextest run --workspace` invokes Playwright a11y suite → emits `a11y-axe-core-results.jsonl`, `a11y-lighthouse-results.json`, `a11y-pa11y-results.json` to `~/.andromeda-pulse/logs/` (logs path per upstream Section 5 `logs` command); subsequent step `- uses: actions/upload-artifact@v4 with: { name: a11y-results, path: ~/.andromeda-pulse/logs/a11y-*.json* }`.
- **WCAG SCs covered:** N/A pattern itself covers no SCs — propagates SC coverage from underlying tools (axe-core ~50 SCs, Lighthouse ~30 audits, pa11y WCAG2AA).
- **Fits because:** Required by upstream Section 1 CI/CD Platform (GitHub Actions, `ci.yml` runs fmt + clippy + xtask test) + a11y-scope Section 3 CI Integration ("Integrate into existing GitHub Actions `ci.yml` pipeline"); reuses tests' E2E driver per binding contract (Section 5 Test Harness `boot` / `run` / `status` 5-command discipline). Required CI gate per a11y-scope Section 3: "PR cannot merge if axe-core reports WCAG SC violation (critical / serious) on **desktop-webview** / Lighthouse a11y category score < 90 / Keyboard focus order test fails / Contrast verification detects token mismatch".
- **Key detail:** Cross-platform CI matrix (Windows/macOS/Linux per upstream Section 1 release.yml `tauri-action`) requires per-OS axe-core driver setup — Windows uses WebView2 CDP at `:9222`, Linux uses webkit2gtk-driver, macOS uses `tauri-plugin-webdriver` (no Apple-provided WKWebView driver). Bundle the a11y CI as one matrix entry that runs on `ubuntu-latest` first (canonical) then expanded to all OS.
- **Source:** https://accessibility.civicactions.com/posts/automated-accessibility-testing-leveraging-github-actions-and-pa11y-ci-with-axe

### Lighthouse CI (`@lhci/cli`)

- **Version:** 0.15.0
- **Last release:** 2025-11-08
- **Status:** actively maintained by Google Chrome team
- **Agent-runnable:** yes — `lhci autorun` emits `.lighthouseci/lhr-*.json` per URL with full Lighthouse JSON; `lighthouserc.json` `assert.assertions` configures threshold gates that emit non-zero exit code on failure. Configuration: `assertions: { 'categories:accessibility': ['error', { minScore: 0.9 }] }` blocks merge on a11y score < 90 per a11y-scope Section 3 CI gate.
- **WCAG SCs covered:** Inherits Lighthouse 12 a11y category SC coverage (SC 1.1.1, 1.3.1, 1.4.3, 2.1.1, 2.4.1, 2.4.2, 2.4.7, 3.1.1, 3.3.2, 4.1.2, 4.1.3, 2.5.8 target-size).
- **Fits because:** Direct implementation of a11y-scope Section 3 CI gate "Lighthouse a11y category score < 90 PR cannot merge" for the **desktop-webview** surface; integrates with GitHub Actions via `lhci-action` for automatic PR comment with score deltas.
- **Key detail:** lhci 0.15.x does NOT yet support Lighthouse 13 (which requires Node 22.19+); pin `@lhci/cli@0.15.x` and `lighthouse@12.x`. Use `--collect.startServerCommand="npm run preview"` to spin up the Tauri dev server before the audit.
- **Source:** https://github.com/GoogleChrome/lighthouse-ci/releases

## Service Identity Convention

### Inherited from upstream-context Section 6 Obs Plan

- **Version:** N/A (convention pattern, not a versioned package — convention inherited from obs)
- **Last release:** 2026-05-02 (this run; convention pinned at obs Phase 7)
- **Status:** actively maintained (binding from obs plan)
- **Agent-runnable:** yes — a11y violation JSON entries tag with same `service.name` / `service.version` / `deployment.environment` resource attributes as obs traces, enabling cross-correlation. Configuration: a11y test harness adds `service.name: "com.andromeda.pulse"`, `service.version: env!("CARGO_PKG_VERSION")` (or runtime read from `tauri.conf.json`), `deployment.environment: "production"` to every emitted violation record (structured violation JSON aligned to obs Section 6 schema).
- **WCAG SCs covered:** N/A (convention itself covers no SCs — drives violation-trace correlation across all SCs reported by other tools).
- **Fits because:** Cross-correlates a11y violations with obs traces per upstream-context Section 6 binding contract; required for `service` field in obs log format JSON schema, which a11y violation JSON aligns to per a11y-scope Section 3. Applies across **desktop-webview** + **desktop-native** + **os-notification** surfaces.
- **Key detail:** `service.name` for the main desktop app is `"com.andromeda.pulse"` (Tauri bundle identifier); `"andromeda-pulse-mcp"` for the mcp-server sidecar. A11y violations on the desktop-webview surface always tag `"com.andromeda.pulse"` (mcp-server has no UI surface).
- **Source:** https://opentelemetry.io/docs/specs/semconv/resource/

## Screen Reader Test Pattern

### NVDA (Windows) — manual SR pass with structured PASS/FAIL spec

- **Version:** 2025.3
- **Last release:** 2025-12-05
- **Status:** actively maintained by NV Access (free, open-source)
- **Agent-runnable:** yes — manual pass produces structured JSON when authored as spec; SR announcements logged to `nvda.log` with timestamp; parsed into `a11y-sr-nvda-results.jsonl` `{path, sr_announcement_expected, sr_announcement_actual, status}` records (structured violation JSON aligned to obs Section 6 schema). Manual operator triggers spec; structured spec output is machine-readable.
- **WCAG SCs covered:** SC 4.1.2 Name/Role/Value (runtime SR verification — every interactive element role + accessible name announced), SC 1.3.1 Info and Relationships (table headers, list items, headings announced with structure), SC 2.4.6 Headings and Labels (heading text + level announced), SC 4.1.3 Status Messages (aria-live region announcements verified).
- **Fits because:** **screen-reader-priority** trigger (a11y-scope Section 5) lists "NVDA (Windows) / VoiceOver (macOS) / Orca (Linux)" as supplemental verification per Section 3 Screen Reader Test Pattern; covers must-be-accessible paths P1 (canvas region announce), P2 (modal title announce), P6 (notification readability), P7 (form field labels announced).
- **Key detail:** NVDA 2025 dynamic content updates handle aria-live region transitions more reliably; supplemental to automated tools per a11y-scope discipline ("Manual SR pass supplemental, never sole"). Pair with Chrome on Windows per a11y-scope Section 3 platform table.
- **Source:** https://www.nvaccess.org/download/

### VoiceOver (macOS) — manual SR pass with structured PASS/FAIL spec

- **Version:** Built into macOS 15.3 / 16.0
- **Last release:** 2026-03-15
- **Status:** actively maintained by Apple (built-in to macOS)
- **Agent-runnable:** yes — same as NVDA: manual SR pass logged to structured JSON spec for the aggregate `a11y-violations-summary.json`.
- **WCAG SCs covered:** SC 4.1.2 Name/Role/Value, SC 1.3.1 Info and Relationships, SC 2.4.6 Headings and Labels, SC 4.1.3 Status Messages (same SR runtime coverage as NVDA).
- **Fits because:** **screen-reader-priority** trigger (a11y-scope Section 5) — macOS-side coverage for Tauri **desktop-webview** bundle; required because Tauri 2 ships `.dmg` per upstream Section 1 release.yml.
- **Key detail:** Pair with Safari per a11y-scope Section 3 platform table; VoiceOver rotor (VO+U) navigates by ARIA landmarks, which validates the `main` / `navigation` / `dialog` / `region` roles inventory from a11y-scope Section 2 ARIA Roles Inventory.
- **Source:** https://help.apple.com/voiceover/mac/

### Orca (Linux) — manual SR pass with structured PASS/FAIL spec

- **Version:** 48.0
- **Last release:** 2025-09-18
- **Status:** actively maintained by GNOME
- **Agent-runnable:** yes — same manual SR pass with structured JSON spec aligned to obs Section 6 schema.
- **WCAG SCs covered:** SC 4.1.2 Name/Role/Value, SC 1.3.1 Info and Relationships, SC 2.4.6 Headings and Labels, SC 4.1.3 Status Messages.
- **Fits because:** **screen-reader-priority** trigger (a11y-scope Section 5); Linux-side coverage for Tauri `.AppImage` / `.deb` bundles per upstream Section 1 — desktop-webview surface using WebKitGTK.
- **Key detail:** Pair with Firefox per a11y-scope Section 3 platform table; Orca 47+ improved WebKitGTK ATK bridge handling (relevant because Tauri uses WebKitGTK on Linux).
- **Source:** https://help.gnome.org/users/orca/

## Focus Management Library

### focus-trap-react

- **Version:** 12.0.0
- **Last release:** 2026-02-08
- **Status:** actively maintained
- **Agent-runnable:** yes (downstream) — library produces no JSON itself; downstream Playwright test of trapped modal emits structured assertion JSON aligned to obs Section 6 schema: `await expect(modalContent.locator('button[aria-label="Close"]')).toBeFocused()` after Esc inside trap. Configuration: `<FocusTrap active={isOpen} focusTrapOptions={{ initialFocus: '#first-input', escapeDeactivates: true, returnFocusOnDeactivate: true }}>`.
- **WCAG SCs covered:** SC 2.1.2 No Keyboard Trap (`escapeDeactivates: true` ensures Esc breaks the trap → required by a11y-scope Section 5 keyboard-only trigger), SC 2.4.3 Focus Order (`initialFocus` sets entry point; trap cycles Tab through container), SC 2.4.7 Focus Visible (depends on consumer's CSS — pair with `--border-focus` token), SC 3.2.1 On Focus (no unexpected context change on focus shift inside trap).
- **Fits because:** Required for must-be-accessible paths P2 (investigation-modal), P3 (settings-modal MCP toggle), P6 (investigation-modal during snapshot generation), P7 (settings-modal full form) per a11y-scope Section 4; layout templates (upstream Section 4) require Esc closes modals and returns focus to trigger button. Covers **keyboard-only** trigger (Section 5).
- **Key detail:** v12.0.0 dropped propTypes / defaultProps for React 19 compatibility (React 19 dropped these completely); minimum React >= 18; bumps focus-trap core to v7.6.2. Pair with `tabbable` 6.4.0 (already a transitive dependency).
- **Source:** https://www.npmjs.com/package/focus-trap-react

### tabbable (focus order ground truth)

- **Version:** 6.4.0
- **Last release:** 2026-03-04
- **Status:** actively maintained
- **Agent-runnable:** yes — programmatic library used inside Playwright tests to compute the expected Tab order for assertion against actual `document.activeElement` sequence. Output: array of DOM nodes embedded in test JSON aligned to obs Section 6 schema (per-step PASS/FAIL records).
- **WCAG SCs covered:** SC 2.4.3 Focus Order (computes browser-correct tab order via positive tabindex + zero tabindex DOM order rules), SC 2.1.2 No Keyboard Trap (`isFocusable()` distinguishes from `isTabbable()` to detect orphan focusable elements that break Tab cycle).
- **Fits because:** Powers focus order assertions for a11y-scope Section 3 Focus Management Test Harness on the **desktop-webview** surface; provides the expected sequence to compare against Playwright `page.keyboard.press('Tab')` actual sequence per **keyboard-only** trigger (Section 5). Required for must-be-accessible paths P5, P7.
- **Key detail:** 7.7M weekly downloads; transitive dependency of focus-trap-react; can be used standalone for `tabbable(container)` to compute and assert focus order in any Playwright test.
- **Source:** https://www.npmjs.com/package/tabbable

## ARIA Component Library

### React Aria Components (Adobe)

- **Version:** 1.17.0
- **Last release:** 2026-04-23
- **Status:** actively maintained by Adobe
- **Agent-runnable:** yes (downstream) — components emit semantic HTML + ARIA attributes that axe-core JSON validates; component library itself produces no JSON. Configuration: `import { Dialog, Button, ComboBox, ListBox } from 'react-aria-components';`.
- **WCAG SCs covered:** SC 4.1.2 Name/Role/Value (every component sets correct role/aria-label/aria-* state attributes per WAI-ARIA APG), SC 2.1.1 Keyboard (built-in arrow key navigation, typeahead, multiple selection modifiers per component spec), SC 1.3.1 Info and Relationships (semantic HTML + landmark roles), SC 2.4.3 Focus Order (composite components handle focus via roving tabindex), SC 2.4.7 Focus Visible (FocusRing component + `data-focus-visible` attribute).
- **Fits because:** Implements ARIA-Relevant Component Patterns from upstream Section 3 (Button, Input/Form field, Disclosure/Expandable, Data table) with semantic HTML first + ARIA second principle on the **desktop-webview** surface; covers **screen-reader-priority** + **keyboard-only** triggers (a11y-scope Section 5). Required ARIA roles inventory in must-be-accessible paths: `dialog` (P2, P3, P6, P7), `radiogroup` (P2 preset selector), `slider` (P2 token budget), `switch` (P3 MCP toggle), `combobox` (settings dropdowns).
- **Key detail:** React Aria 2025-2026 added React 19 ref cleanup support, drag-and-drop in Tree, Submenu, FileTrigger, DropZone; composite components implement WAI-ARIA APG patterns with full keyboard + SR support.
- **Source:** https://react-spectrum.adobe.com/react-aria/

### Headless UI (Tailwind Labs)

- **Version:** 2.2.10
- **Last release:** 2025-12-19
- **Status:** actively maintained (Tailwind Labs official)
- **Agent-runnable:** yes (downstream) — emits semantic HTML + ARIA attributes validated by axe-core JSON. Configuration: `import { Menu, Dialog, Switch } from '@headlessui/react';`.
- **WCAG SCs covered:** SC 4.1.2 Name/Role/Value (handles ARIA attribute management automatically), SC 2.1.1 Keyboard (built-in keyboard interactions per component), SC 1.3.1 Info and Relationships (WAI-ARIA roles + relationships), SC 2.4.3 Focus Order (focus management within components), SC 2.4.7 Focus Visible (focus state management).
- **Fits because:** Pairs natively with Tailwind CSS v4 (per upstream Section 3 design system stack) on the **desktop-webview** surface; v2.0 added built-in anchor positioning + new checkbox component + combobox virtualization. Alternative to React Aria for the same surface coverage; covers **keyboard-only** + **screen-reader-priority** triggers.
- **Key detail:** v2.2.x bumped @tanstack/react-virtual to fix React 19 warnings; recommend Headless UI when project already uses Tailwind v4 utility classes (matching styling philosophy). React Aria provides more components (32 vs Headless UI's smaller set); pick one per project — do not mix.
- **Source:** https://github.com/tailwindlabs/headlessui/releases

## A11y Linting Pattern

### eslint-plugin-jsx-a11y

- **Version:** 6.10.0
- **Last release:** 2025-08-14
- **Status:** actively maintained
- **Agent-runnable:** yes — emits ESLint JSON via `eslint --format json` listing `{filePath, messages: [{ ruleId, severity, message, line, column }]}` per JSX a11y violation (structured violation JSON aligned to obs Section 6 schema after CI step adapter). Configuration: extend `'plugin:jsx-a11y/recommended'` (or `'plugin:jsx-a11y/strict'`) in `eslint.config.js`; integrate into `npm run lint` in CI.
- **WCAG SCs covered:** SC 1.1.1 Non-text Content (alt-text, img-redundant-alt, role-has-required-aria-props), SC 4.1.2 Name/Role/Value (label-has-associated-control, no-noninteractive-element-interactions, role-supports-aria-props, anchor-has-content), SC 3.3.2 Labels or Instructions (label-has-associated-control, control-has-associated-label), SC 2.1.1 Keyboard (interactive-supports-focus, click-events-have-key-events), SC 1.3.1 Info and Relationships (heading-has-content, scope), SC 2.1.2 No Keyboard Trap (no-noninteractive-tabindex).
- **Fits because:** Compile-time/lint-time gate cheaper than runtime axe-core for the **desktop-webview** surface; catches "ARIA additions on top of non-semantic HTML" anti-pattern flagged in scope constraints (e.g., `<div role="button">` instead of `<button>` triggers `prefer-tag-over-role`). Catches missing labels for must-be-accessible path P7 (settings-modal form fields each requires `<label>` association) before axe-core runs in CI.
- **Key detail:** Use `'plugin:jsx-a11y/recommended'` for AA-equivalent baseline; `'plugin:jsx-a11y/strict'` for AAA-leaning; pair with `eslint-plugin-react` and `@typescript-eslint/eslint-plugin` per project's TypeScript strict mode discipline.
- **Source:** https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/releases

## Motion / Reduced-Motion Library

[trigger-driven; pulled in by motion-sensitive trigger from a11y-scope Sec 5; not standard for Standard tier but required for trigger coverage — SC 2.3.3 AAA escalation per a11y-scope Section 3 WCAG Criteria Mapping]

### Tailwind CSS v4 `motion-reduce` / `motion-safe` variants

- **Version:** 4.0.0
- **Last release:** 2026-01-22
- **Status:** actively maintained
- **Agent-runnable:** yes (downstream) — Playwright with `await page.emulateMedia({ reducedMotion: 'reduce' })` activates `prefers-reduced-motion: reduce` and validates that `motion-reduce:duration-0`-class elements have computed `transition-duration: 0s`; computed style emitted in custom JSON output (structured violation JSON aligned to obs Section 6 schema for `wcag_criterion: SC 2.3.3` records).
- **WCAG SCs covered:** SC 2.3.1 Three Flashes (A — no >3 flashes/sec), SC 2.3.3 Animation from Interactions (AAA — escalation per a11y-scope motion-sensitive trigger; reduced-motion override disables non-essential animation triggered by interaction).
- **Fits because:** **motion-sensitive** trigger (a11y-scope Section 5) requires "all `--duration-*` tokens degrade per design's reduced-motion override" + "skeleton animation / pulse opacity / investigation collapse scale disabled under reduced motion" on the **desktop-webview** surface; Tailwind v4 first-class CSS variants implement this directly. Required for SC 2.3.3 AAA escalation in a11y-scope Section 3 WCAG mapping.
- **Key detail:** Apply `motion-reduce:duration-0 motion-reduce:transition-none` on every animated element (consumers of `--duration-fast`, `--duration-standard`, `--duration-investigation-collapse` per upstream Section 3); in Playwright test, `await page.emulateMedia({ reducedMotion: 'reduce' })` then assert `getComputedStyle(el).animationDuration === '0s'` and `transitionDuration === '0s'`.
- **Source:** https://tailwindcss.com/docs/transition-duration

### useReducedMotion (motion/react) — programmatic React 19 hook

- **Version:** 12.0.0
- **Last release:** 2026-04-04
- **Status:** actively maintained
- **Agent-runnable:** yes (downstream) — hook returns boolean used in JSX to conditionally render reduced-motion variant; Playwright validates rendered output under `emulateMedia({ reducedMotion: 'reduce' })` (structured violation JSON aligned to obs Section 6 schema for `wcag_criterion: SC 2.3.3` records).
- **WCAG SCs covered:** SC 2.3.1 Three Flashes, SC 2.3.3 Animation from Interactions (AAA).
- **Fits because:** Powers JS-driven motion paths that aren't pure CSS transitions (e.g., the WebGPU canvas updating at telemetry-driven rates per upstream "motion-as-data principle") on the **desktop-webview** surface; hook lets canvas frame loop pause/replace with static state under reduced-motion preference. Covers Investigation Capture Collapse (`--duration-investigation-collapse`) and skeleton animation (`--duration-fast`) JS-side. Addresses **motion-sensitive** trigger.
- **Key detail:** Tiny ~1kb hook bundle; subscribes to `(prefers-reduced-motion: reduce)` matchMedia changes and re-renders. For non-React (vanilla) motion-as-data canvas loops, use `window.matchMedia('(prefers-reduced-motion: reduce)').matches` directly with a `change` event listener.
- **Source:** https://motion.dev/docs/react-use-reduced-motion

## Target-Size Verification

[trigger-driven; pulled in by target-size trigger from a11y-scope Sec 5; not standard for Standard tier but required for trigger coverage — design tokens `--target-button-min` and `--target-input-min` from upstream Section 3]

### axe-core `target-size` rule

- **Version:** 4.11.3
- **Last release:** 2026-04-17
- **Status:** actively maintained
- **Agent-runnable:** yes — emits axe-core JSON `violations[].id === 'target-size'` with node-level computed bounding box dimensions (structured violation JSON aligned to obs Section 6 schema with `wcag_criterion: SC 2.5.8` records). Configuration: `axe.run({ rules: { 'target-size': { enabled: true } } })` (rule disabled by default until WCAG 2.2 widely adopted) or `runOnly: { type: 'tag', values: ['wcag22aa'] }` to include all WCAG 2.2 AA rules.
- **WCAG SCs covered:** SC 2.5.8 Target Size Minimum (AA: 24×24 CSS px or sufficient spacing — the "virtual circle of diameter 24" rule). Also bridges to SC 2.5.5 Target Size Enhanced (AAA: 44×44) when token `--target-button-min` is configured to a 44×44 minimum.
- **Fits because:** **target-size** trigger (a11y-scope Section 5) names "WCAG 2.1 AA SC 2.5.8 Target Size (Minimum) assertion" + "axe-core target-size rule when WCAG 2.2 AA enabled"; design tokens `--target-button-min` and `--target-input-min` from upstream Section 3 explicitly serve SC 2.5.5 (44×44) + SC 2.5.8 (24×24). Required for must-be-accessible paths P2, P3, P5, P7 (every interactive button/input/switch/slider) on the **desktop-webview** surface.
- **Key detail:** Padding counts toward target size — visible glyph can be smaller than 24×24 if invisible padding extends the click region. Compact-widget surface (per upstream Section 4 layout templates) is the most-at-risk for this SC because the quarter-screen widget has limited space; verify all interactive elements maintain 24×24 (or design's `--target-button-min` token value).
- **Source:** https://dequeuniversity.com/rules/axe/4.11/target-size