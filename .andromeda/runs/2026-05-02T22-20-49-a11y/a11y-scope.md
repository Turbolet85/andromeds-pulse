## 1. A11y Scope

| Entity | Source | Assertability | Reason if not fully assertable |
|--------|--------|--------------|-----|
| **desktop-webview (React 19 + Tailwind CSS + WebGPU canvas)** | Arch Excerpt Stack + Surfaces + Design Excerpt Surfaces | Assertable | Primary web UI surface: compact-widget, full-dashboard, settings-modal, investigation-modal layouts |
| **desktop-native (Tauri 2 tray icon)** | Arch Excerpt Stack + Surfaces + Design Excerpt Surfaces | Assertable | System tray menu surface (Windows NotifyIcon / macOS NSStatusItem / Linux AppIndicator) — keyboard navigation via arrow keys / Return; OS-native a11y APIs apply |
| **WebGPU canvas visualization** | Arch Excerpt Stack | Assertable | GPU-accelerated chart rendering within webview; depends on semantic labels, interactive controls, and ARIA markup from design layer — no built-in accessibility; relies on wrapper element semantics |
| **Tauri IPC bridge (TauRPC)** | Arch Excerpt Stack + A11y-Relevant Conventions | Not-assertable | No UI surface; IPC mechanics affect error serialization and focus state during async operations but not a11y-testable surface itself |
| **OTLP/gRPC receiver (tonic)** | Arch Excerpt Stack | Not-assertable | Backend service surface; non-UI, no a11y testing scope |
| **OTLP/HTTP receiver (axum + hyper + tower)** | Arch Excerpt Stack | Not-assertable | Backend service surface; non-UI, no a11y testing scope |
| **DuckDB columnar storage** | Arch Excerpt Stack | Not-assertable | Backend-only; no UI surface, no a11y testing scope |
| **WASM plugin runtime (wasmtime)** | Arch Excerpt Stack | Boundary-only | Plugins can extend UI surfaces if granted WIT imports; a11y testing of plugin-provided UI is plugin-author responsibility; boundary assertion covers capability gating only |
| **Tray icon status display** | Design Excerpt Surfaces + Layout Templates | Assertable | Traffic-light status indicator within tray-icon surface; keyboard-navigable via OS menu system |
| **Notification (OS toast)** | Creator Brief Excerpt must-work scenarios | Assertable | "Snapshot ready ({N} tokens)" notification; content must be screen-reader accessible via OS notification API; OS-native a11y |
| **Settings modal form** | Layout Templates + Creator Brief | Assertable | Theme selector, widget snap position, retention input, MCP toggle, snapshot preset, plugin manager — form controls require label association and keyboard navigation |
| **Investigation modal** | Layout Templates | Assertable | Modal with close button (Esc escape path); focus management required |
| **File picker (native)** | Layout Templates | Boundary-only | OS-native file dialog; a11y boundary is Tauri modal wrapper, not picker internals |

---

## 2. A11y Surfaces & Assistive Tech Reach

| Surface | Automated Tool Reach | Manual Verification (supplemental) | ARIA Roles Inventory | Service Identity Tagging | Notes |
|---------|---------------------|--------------------------------|---------------------|------------------------|-------|
| **desktop-webview (web-spa)** | **axe-core** (`@axe-core/playwright` for E2E + `axe-core` standalone for rendered DOM) + **Lighthouse a11y category** (Chrome DevTools / `lighthouse` CLI) + **pa11y** (`pa11y` / `pa11y-ci`) | **NVDA** (Windows) / **JAWS** (Windows) / **VoiceOver** (macOS) / **Orca** (Linux) — supplemental to automated assertions, never sole | **Landmark roles:** `main` / `navigation` / `banner` / `contentinfo` (layout templates define regions); **Interactive roles:** `button` / `tab` / `dialog` (modal patterns) / `alert` / `status` (toast / error messages); **Live regions:** `aria-live="polite"` for toasts, `aria-live="assertive"` for error focus shifts | `service.name`: `"com.andromeda.pulse"` (compile-time constant, Tauri bundle identifier); `deployment.environment`: `"production"` | Tauri webview a11y tree exposed via DevTools Protocol; full axe-core reach for React 19 rendered DOM. Canvas (`<canvas>` + WebGPU) has no built-in ARIA; semantics enforced via label elements and ARIA attributes on wrapper/control elements. |
| **desktop-native (tray-icon menu)** | **axe-core via DevTools Protocol** (`@axe-core/playwright` / `@axe-core/puppeteer` against webview when available); **fallback: OS-native a11y APIs** (Windows UIA / macOS NSAccessibility / Linux ATK — no automated tool reach for native menu internals) | **VoiceOver** (macOS) / **NVDA** (Windows) / **Orca** (Linux) — manual SR pass required for menu navigation and item selection; OS keyboard discipline (arrow keys / Return) verified manually | **Interactive roles:** `menu` / `menuitem` (tray action menu); **State roles:** button state via native menu item attributes | `service.name`: `"com.andromeda.pulse"` | OS-native tray surfaces (NotifyIcon / NSStatusItem / AppIndicator) expose limited a11y tree to automated tools; manual keyboard + screen reader testing required. No axe-core reach for tray menu internals. |
| **OS notification (toast)** | No automated tool reach (OS-level API) | **VoiceOver** (macOS) / **NVDA** (Windows) / **Orca** (Linux) / **TalkBack** (if web-accessible fallback provided) — manual verification of notification readability and timing | No ARIA; OS notification content exposed via native a11y APIs (`NSAccessibilityNotificationKey` / UIA notification event) | `service.name`: `"com.andromeda.pulse"` | Content authored in code ("Snapshot ready ({N} tokens). Paste in {AI tool} to investigate.") must be screen-reader readable; OS-level a11y handled by Tauri / system framework. |

---

## 3. A11y Assertion Harness Specification

### A11y Testing Tool Pick
- **Primary tool (desktop-webview):** **axe-core** (`@axe-core/playwright` 4.x for E2E + `axe-core` standalone for rendered DOM validation) + **Lighthouse a11y category** (Chrome DevTools / `lighthouse` 11.x CLI for CI gate) + **pa11y** (`pa11y` / `pa11y-ci` 7.x for parallel rule matrix)
- **Secondary tool (desktop-native / OS surfaces):** **No automated tool reach** for tray menu / native file picker / OS notifications. Manual screen reader testing supplemental. Focus management + keyboard discipline verified via Playwright against webview surface; tray menu keyboard tested manually or via native API mocking.

**From upstream-context Section 5 Test Harness Contract:** A11y harness reuses tests' E2E driver (`@axe-core/playwright` piggybacks on Playwright session created by `boot` / `run` / `status` harness). CI integration occurs via existing `run` command (cargo nextest); a11y assertions emit JSON (axe-core JSON + Lighthouse JSON + structured violation JSON) to logs dir alongside test results.

### WCAG Criteria Mapping

**Tier:** **Standard (1)** — mapped to **WCAG 2.1 AA** full (~50 SCs) as baseline.

**Trigger-driven AAA escalation:**
- **motion-sensitive trigger** (from Creator Brief Excerpt, rigor hints: "respects `prefers-reduced-motion`; motion-as-data principle; motion reflects telemetry character") → **ADD WCAG 2.1 AAA SC 2.3.3 Animation from Interactions** alongside Standard AA list. All motion tokens (`--duration-fast`, `--duration-standard`, `--duration-investigation-collapse`, `--easing-out`, `--easing-in-out`) must emit reduced-motion override assertions.
- **visual-discrimination trigger** (from Creator Brief Excerpt rigor hints: "Status colors not-color-alone (paired with iconography per WCAG)"; from Design Excerpt error/warning/success/info state color tokens) → **ADD WCAG 2.1 AA SC 1.4.1 Use of Color** (already part of Standard AA; no AAA escalation). Paired iconography supplement verified per token (--color-accent + icon/label, --color-feedback-success + icon/label, etc.).

**WCAG 2.1 AA core criteria list (50 SCs):**
- **Perceivable:** SC 1.1.1 Non-text Content / SC 1.2.1 Audio-only and Video-only / SC 1.2.2 Captions / SC 1.2.3 Audio Description / SC 1.3.1 Info and Relationships / SC 1.3.2 Meaningful Sequence / SC 1.3.3 Sensory Characteristics / SC 1.4.1 Use of Color / SC 1.4.2 Audio Control / SC 1.4.3 Contrast (Minimum) / SC 1.4.4 Resize Text / SC 1.4.5 Images of Text / SC 1.4.10 Reflow / SC 1.4.11 Non-text Contrast / SC 1.4.12 Text Spacing / SC 1.4.13 Content on Hover or Focus
- **Operable:** SC 2.1.1 Keyboard / SC 2.1.2 No Keyboard Trap / SC 2.1.4 Character Key Shortcuts / SC 2.2.1 Timing Adjustable / SC 2.2.2 Pause, Stop, Hide / SC 2.3.1 Three Flashes / SC 2.4.1 Bypass Blocks / SC 2.4.2 Page Titled / SC 2.4.3 Focus Order / SC 2.4.4 Link Purpose / SC 2.4.7 Focus Visible
- **Understandable:** SC 3.1.1 Language of Page / SC 3.2.1 On Focus / SC 3.2.2 On Input / SC 3.3.1 Error Identification / SC 3.3.2 Labels or Instructions / SC 3.3.3 Error Suggestion / SC 3.3.4 Error Prevention / SC 4.1.2 Name, Role, Value / SC 4.1.3 Status Messages
- **Robust:** SC 4.1.1 Parsing (deprecated in WCAG 2.2; retain for AA baseline)

**Trigger-pulled AAA SCs (WCAG 2.1 AAA only):**
- **SC 2.3.3 Animation from Interactions** (motion-sensitive trigger from Creator Brief) — machine-verifiable via reduced-motion media query assertion + Lighthouse a11y audit

### Structured Violation JSON Schema

**Binding contract from upstream-context Section 6 Obs Plan Excerpt Log Format JSON Schema:**

```json
{
  "timestamp": "2026-05-02T16:18:34.567Z",
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
    "remediation": "Increase foreground darkness to meet 4.5:1 minimum per SC 1.4.3 (AA); use --color-text-primary token instead",
    "tool": "axe-core",
    "tool_result_id": "color-contrast-rule-12345"
  }
}
```

**A11y violation emissions align to obs Section 6 schema** (binding: obs fixed the contract; a11y narrows). Required fields:
- `wcag_criterion` — SC ID (e.g., "SC 1.4.3")
- `violation_type` — axe-core rule category (e.g., "color-contrast", "button-name", "aria-required-attr")
- `severity` — "critical" / "serious" / "moderate" / "minor" per axe-core impact classification
- `surface` — "desktop-webview" / "desktop-native" / "os-notification"
- `selector` — DOM/element path for web surfaces; "N/A" for native
- `remediation` — plain-language fix with token reference if design-system-driven
- `tool` — "axe-core" / "lighthouse" / "pa11y" / "manual"
- `tool_result_id` — rule instance ID for correlation

**Extensions (optional, per trigger):**
- `token_name` — design token binding (e.g., "--color-text-primary" for contrast pair)
- `measured_value` / `required_value` — numeric comparison (e.g., measured contrast 3.2 vs required 4.5)
- `affected_component` — React component or layout type (e.g., "SettingsModal", "full-dashboard-traces")

### Focus Management Test Harness

**Tool:** **Playwright** (`@axe-core/playwright`) focus tracking pattern + scripted Tab navigation via `page.keyboard.press('Tab')` / `page.keyboard.press('Shift+Tab')`.

**Contract from upstream-context Section 5 Test Harness:**
- Reuses E2E driver: `boot` command starts Tauri app → `run` invokes `cargo nextest` (tests spawn app) → a11y focus harness taps same test session via Playwright.
- Initial focus anchor per layout template (Section 4 Focus Management Anchors):
  - **compact-widget:** Esc key minimizes to tray; focus returns to tray icon on restoration
  - **full-dashboard:** Tab navigates through tabs/sidebar items; Enter / Space activates
  - **settings-modal:** Tab cycles through form controls; focus ring displays `--border-focus` token
  - **investigation-modal:** close button (✕ glyph) in top-right; Esc closes (explicit escape path)
- Focus order assertion: verify Tab sequence matches visual left-to-right / top-to-bottom order
- Focus-visible assertion: verify `--border-focus` outline composition applied to all `:focus-visible` elements (Lighthouse a11y check + manual CSS inspection)
- Focus trap: verify Esc closes modals (settings, investigation) and returns focus to trigger button

**Focus-relevant span coverage (obs excerpt):** None explicitly named in upstream-context; Phase 3 will recommend focus tracing spans (`focus.shift` / `focus.trap.enter` / `focus.trap.exit` / `focus.restore`) if obs plan adds later.

### Keyboard Test Harness

**Tool:** **Playwright** scripted key sequences per surface.

**Keyboard coverage per surface:**
- **desktop-webview (React UI):**
  - **Tab / Shift+Tab:** Forward/backward focus navigation through all interactive elements (buttons, inputs, tabs, links, disclosure toggles)
  - **Enter / Space:** Activate buttons, submit forms, toggle disclosures
  - **Arrow keys:** Navigate tab panels (left/right), data table cells (up/down/left/right), combobox / menu items (up/down)
  - **Escape:** Close modals (settings, investigation); minimize compact-widget to tray
  - **Scripting:** `page.keyboard.press('Tab')` → capture `document.activeElement` → assert focus order matches visual DOM tree
- **desktop-native (tray menu):**
  - **Arrow keys:** Navigate menu items (up/down)
  - **Return / Space:** Select menu item
  - **Escape:** Close menu
  - **Manual verification:** Tray menu is OS-native; no automated keyboard simulation available; manual pass required
- **CLI / TUI:** Not present in project; N/A

**Test pattern:** Playwright E2E test per layout type — tab through each layout (compact-widget, full-dashboard-traces, settings-modal, investigation-modal) and assert focus sequence matches DOM order via `.evaluate()` checking `document.activeElement.id` / `document.activeElement.className` after each Tab press.

### Screen Reader Test Pattern

**Manual SR pass spec** (supplemental to automated assertions, never sole).

**Browsers + SRs per platform:**
- **Windows:** Chrome + NVDA (free, open-source)
- **macOS:** Safari + VoiceOver (native OS)
- **Linux:** Firefox + Orca (free, open-source)

**Test spec (per must-be-accessible path):**
1. Launch app via `boot` command
2. Activate screen reader (NVDA command, VoiceOver rotor, Orca accessible menu)
3. Navigate via SR commands (NVDA: browse mode arrow keys; VoiceOver: VO+arrow; Orca: arrow keys)
4. Assert label/heading/role announcement for each element in focus order
5. Verify error messages announced with ARIA markup (aria-describedby, aria-invalid=true)
6. Verify toast notifications announced via aria-live=polite (non-blocking status updates)
7. Verify modal title and close button announced on dialog open
8. Verify interactive element roles and states announced (button pressed, input required, etc.)
9. Verify WebGPU canvas region announced with semantic label (not just `<canvas>` tag)

**Defects recorded:** SR announcements missing / out of order / conflicting with visual state.

### Contrast Verification Harness

**Tool:** **axe-core auto-check** (color-contrast rule) + **custom assertion reading design tokens** from upstream-context Section 3 Design Excerpt A11y-Relevant Design Tokens.

**Token pairs validated:**
- `--color-text-primary / --color-base` → 4.5:1 (SC 1.4.3 minimum)
- `--color-text-secondary / --color-base` → 3:1 (SC 1.4.3 large text minimum)
- `--color-text-tertiary / --color-base` → 3:1 (SC 1.4.3 large text minimum for large text only)
- `--color-primary / --color-base` → focus indicator / active state accent — verify ≥ 3:1 non-text contrast (SC 1.4.11)
- `--color-feedback-success / --color-inset` → measure and assert (SC 1.4.3 minimum 4.5:1 for text; 3:1 for non-text backgrounds per SC 1.4.11)
- `--color-accent / --color-base` → measure and assert (error state text / alert border; SC 1.4.3 minimum per context)

**Harness implementation:**
- Playwright snapshot of rendered dashboard with all state colors (default, hover, focus, error, success, loading)
- Extract computed colors via `window.getComputedStyle()` + color contrast library (`wcag-contrast`)
- Assert ratio ≥ 4.5:1 for text (or 3:1 for large text) per SC 1.4.3 (AA)
- If motion-sensitive trigger escalates to SC 2.3.3 AAA, verify reduced-motion override does not break contrast (e.g., color is still distinguishable if motion removed)

**Reduced-motion assertion (motion-sensitive trigger):**
- Test `prefers-reduced-motion: reduce` media query override
- Assert all `--duration-*` tokens degrade per design's reduced-motion override
- Verify skeleton animation / pulse opacity / investigation collapse scale disabled under reduced motion
- Assert state change still conveyed via color / text / focus indicator (not motion alone)

### CI Integration

**CI command:** Integrate into existing GitHub Actions `ci.yml` pipeline (upstream-context Section 1 CI/CD Platform: GitHub Actions).

**Integration point:** A11y assertions run as part of `run` command (`cargo nextest run --workspace`) via new test suite `tests::a11y_*` or separate npm script invoked from Tauri build step.

**Typical CI command (Phase 3 will finalize):**
```bash
npm run test:a11y
# OR
cargo xtask test:a11y
```

**Output:** JSON files emitted to `~/.andromeda-pulse/logs/` in obs log format:
- `a11y-axe-core-results.jsonl` — axe-core violations (desktop-webview)
- `a11y-lighthouse-results.json` — Lighthouse a11y category audit
- `a11y-pa11y-results.json` — pa11y matrix results
- `a11y-keyboard-focus-results.jsonl` — Playwright focus order test results
- `a11y-violations-summary.json` — aggregated summary per surface + WCAG criterion

**CI gate:** PR cannot merge if:
- axe-core reports WCAG SC violation (critical / serious) on desktop-webview
- Lighthouse a11y category score < 90
- Keyboard focus order test fails
- Contrast verification detects token mismatch

---

## 4. Critical Paths (must-be-accessible)

| Path | Surfaces Involved | Required ARIA Roles | Required Focus Order | Required WCAG SC Coverage per Tier | Source |
|------|------------------|---------------------|---------------------|----------------------------------|--------|
| **P1: Receive OTLP telemetry (gRPC), visualize in WebGPU dashboard** | desktop-webview (full-dashboard-traces layout); backend OTLP receivers are not-assertable and omitted | `main` / `region[aria-label="Telemetry traces chart"]` / `button` (interaction controls on canvas) / `table` (trace list below chart) | Initial focus: first trace table; Tab navigates table cells; Enter/arrow keys drill into trace detail | SC 1.3.1 Info and Relationships (canvas region semantics) / SC 2.1.1 Keyboard (Tab through chart controls + table) / SC 2.4.3 Focus Order (visual left-to-right table) / SC 1.4.3 Contrast (Minimum) (trace value colors) / SC 4.1.2 Name, Role, Value (canvas label + table structure) | Tests excerpt Critical Path P1 + Creator Brief must-work "Glance-readable from 2 meters" |
| **P2: Generate token-efficient curated snapshot (not raw dump)** | desktop-webview (full-dashboard-traces layout + investigation-modal); backend snapshot service is not-assertable and omitted | `main` / `dialog[aria-label="Investigation Snapshot"]` / `button` (Generate button → aria-busy during generation) / `slider[aria-label="Token budget"]` (token budget control) / `radiogroup` (preset selector) / `status[aria-live="polite"]` (progress message) | Initial focus: Generate button or modal title; Tab through budget slider / preset / Generate button; after submission, focus moves to progress message (aria-live polite non-blocking) | SC 2.1.1 Keyboard (Tab through modal controls) / SC 2.4.3 Focus Order (budget slider → preset → Generate button sequence) / SC 3.3.1 Error Identification (token budget exceeded message) / SC 3.3.2 Labels or Instructions (budget input label + hint) / SC 4.1.2 Name, Role, Value (button purpose + slider min/max) / SC 4.1.3 Status Messages (progress aria-live) | Tests excerpt Critical Path P2 + Creator Brief "Generates token-efficient curated snapshot" + Design excerpt async status pattern |
| **P3: MCP server toggle in settings → notification on state change** | desktop-webview (settings-modal layout) + os-notification surface (assertable via OS a11y APIs); MCP server backend is not-assertable and omitted | `dialog[aria-label="Settings"]` / `switch[aria-checked]` (MCP toggle) / OS notification: title + body (accessible via OS a11y API) | Settings toggle: Tab to MCP switch, Space toggles state, focus stays on switch; OS notification fires asynchronously (does not steal focus) | SC 2.1.1 Keyboard (Tab/Space on switch) / SC 4.1.2 Name, Role, Value (switch role + checked state) / SC 4.1.3 Status Messages (OS notification carries success/failure) | Tests excerpt Critical Path P3 + Creator Brief "MCP server toggle" setting + Obs excerpt Service Identity (mcp-server sidecar identity) |
| **P4: Real-time push of spans/metrics/logs via Tauri IPC Channel** | desktop-webview (full-dashboard-traces / metrics / logs layouts); IPC channel itself is not-assertable and omitted | `region[aria-live="polite"]` (live trace list) / `status` (throughput / error-rate counter) / `table` (incremental row additions); `aria-busy=false` once the channel is ready | Live updates do NOT steal focus from current keyboard target; user-initiated row select via arrow keys / Enter follows focus order rules | SC 2.1.1 Keyboard (current focus preserved through live updates) / SC 2.4.3 Focus Order (focus stable across re-renders) / SC 4.1.3 Status Messages (aria-live announces incremental events without interrupting) / SC 1.4.3 Contrast (live counter colors) | Tests excerpt Critical Path P6 + Creator Brief "Live infographics — real-time visualizations" |
| **P5: Widget compact mode ↔ dashboard expansion ↔ tray icon visibility toggle** | desktop-webview (compact-widget layout) + desktop-native (tray-icon menu); webview IPC plumbing is not-assertable and omitted | `main` (webview) / `button[aria-label="Expand to dashboard"]` (expand button in widget) / `button[aria-label="Minimize to tray"]` (Esc or button) / tray menu item: `menuitem` (toggle visibility) | Compact-widget: Esc minimizes to tray; focus returns to tray icon on widget restoration via Tauri IPC. Tray menu: arrow keys navigate items (open/hide/quit). Full dashboard: Tab navigates sidebar tabs + main content. On expand, focus moves to first tab in full dashboard. On collapse, focus returns to compact widget. | SC 2.1.1 Keyboard (Esc escape path from widget; arrow keys in tray menu) / SC 2.1.2 No Keyboard Trap (Esc works from any modal) / SC 2.4.3 Focus Order (Tab sequence within each layout) / SC 2.4.7 Focus Visible (focus ring on widget buttons + tray selection indicator) / SC 4.1.2 Name, Role, Value (window/tray role indicators) | Tests excerpt Critical Path P5 + Layout Templates focus management anchors (compact-widget Esc, tray-menu arrow keys) + Creator Brief "Click to expand" |
| **P6: Snapshot generation completion → notification + "Investigate" button activation** | desktop-webview (full-dashboard layout + investigation-modal) + os-notification surface | `main` / `dialog[aria-label="Investigation Snapshot"]` / `button` (Generate button → aria-busy during generation) / `status[aria-live="assertive"]` (completion message in modal) / OS notification: title + body (accessible via OS a11y API) | Focus during snapshot generation: stays in modal (aria-busy=true on Generate button). On completion: modal updates with result token count (aria-live="assertive" announces completion). OS notification emitted (async, may not trap focus). | SC 2.1.1 Keyboard (Tab through modal during/after generation) / SC 4.1.2 Name, Role, Value (button busy state) / SC 4.1.3 Status Messages (aria-live polite/assertive on completion) / SC 2.2.1 Timing Adjustable (notification dismiss time not forced if button provided for manual close) | Creator Brief "Notification — Snapshot ready ({N} tokens)" + Design excerpt async status pattern (aria-busy, aria-live) |
| **P7: Settings modal form submission (theme, widget position, retention, MCP toggle, snapshot preset)** | desktop-webview (settings-modal layout) | `dialog[aria-label="Settings"]` / `form` / `label` (for each input) / `button` (Save / Cancel primary/secondary) / `radiogroup` (theme / widget position) / `switch` (MCP toggle) | Initial focus: first form control (theme selector) or modal title. Tab: theme radio → widget position radio → retention input → MCP toggle switch → snapshot preset → Save button → Cancel button. Esc closes modal. Form invalid: focus moves to first invalid input with aria-invalid=true; error message aria-describedby linked to input. | SC 2.1.1 Keyboard (Tab through all controls; Esc to close) / SC 2.1.2 No Keyboard Trap (Esc works from any focused input) / SC 2.4.3 Focus Order (visual top-to-bottom form sequence) / SC 2.4.7 Focus Visible (focus ring on each input per `--border-focus` token) / SC 3.3.1 Error Identification (error message per invalid field) / SC 3.3.2 Labels or Instructions (label element or aria-label for each input) / SC 4.1.2 Name, Role, Value (input role + state + required status) | Layout Templates focus management anchors (settings-modal Tab cycle) + Creator Brief "Settings flows..." + Design excerpt form error/validation pattern |

---

## 5. A11y Triggers

| Trigger Type | Source | Required Assertion |
|-------------|--------|-------------------|
| **visual-discrimination** | Creator Brief rigor hints: "Status colors not-color-alone (paired with iconography per WCAG)"; Design excerpt error/warning/success/info state color tokens (--color-accent, --color-feedback-success, --color-primary, --color-secondary) | **axe-core color-contrast rule** per design token pair (--color-text-primary / --color-base → 4.5:1; --color-feedback-success / --color-inset → 4.5:1; --color-accent / --color-base → 4.5:1). **Custom assertion:** verify icon/label supplement present for every state color (error has icon + text "Invalid"; success has checkmark + "Validation successful"). Assertion type: DOM inspect (icon element present + aria-label on icon if decorative) + axe-core impact classification. **WCAG 2.1 AA SC 1.4.1 Use of Color** + **SC 1.4.3 Contrast (Minimum)** + **SC 1.4.11 Non-text Contrast**. |
| **motion-sensitive** | Creator Brief rigor hints: "respects `prefers-reduced-motion` for accessibility (degrades to instant)"; "motion-as-data principle (motion reflects telemetry character, not decoration)"; design-system motion tokens (--duration-fast, --duration-standard, --duration-investigation-collapse, --easing-out, --easing-in-out). Also creator anti-pattern: "motion-as-data principle (motion reflects telemetry character, not decoration)" (decorative-motion anti-pattern). | **Lighthouse a11y check for prefers-reduced-motion media query override** (built-in audit). **Custom assertion:** Playwright test with `prefers-reduced-motion: reduce` emulated; verify all transitions degrade per design's reduced-motion override; skeleton animation disabled; investigation collapse scale disabled. Assert state change still conveyed via color + text + focus ring (not motion alone). **WCAG 2.1 AAA SC 2.3.3 Animation from Interactions** — escalation added to mapping. **WCAG 2.1 AA SC 2.3.1 Three Flashes** also covered (no flashing > 3 times per second). |
| **keyboard-only** | Layout Templates focus management anchors explicit for all surfaces (compact-widget Esc, tray-menu arrow keys, full-dashboard Tab/Enter/Esc, settings-modal Tab cycle). Creator Brief does not name motor-impaired users explicitly but "always-on-top toggle" + "tray icon" signals power-user workflow compatible with keyboard-only. | **Playwright scripted Tab / Shift+Tab navigation** covering all interactive elements per layout. **Focus order assertion:** Tab sequence matches visual DOM left-to-right, top-to-bottom. **Escape path assertion:** Esc closes all modals (settings, investigation) and returns focus to trigger. **Arrow key assertion (tray menu only):** arrow keys navigate menu items; Return/Space selects. **WCAG 2.1 AA SC 2.1.1 Keyboard** + **SC 2.1.2 No Keyboard Trap** + **SC 2.4.7 Focus Visible** — verify focus ring visible on all `:focus-visible` elements per `--border-focus` token. |
| **screen-reader-priority** | Creator brief does not name blind/low-vision users explicitly; however, "Glance-readable from 2 meters" + "high-contrast palette" + "motion-as-data" (visual + kinetic redundancy) are accessibility-conscious design signals. Semantic HTML first (React `<label>`, `<button>`, `<input>`, `<table>`, `<dialog>` native elements) + ARIA roles inventory (main, navigation, dialog, button, table, alert, status, live regions). | **Manual SR test pattern** per Critical Paths P1-P7 using NVDA (Windows) / VoiceOver (macOS) / Orca (Linux). **Assertion:** ARIA roles announced correctly (button, dialog, table roles); text labels and headings announced; error messages announced with aria-describedby linkage; status messages announced via aria-live=polite (non-blocking) or aria-live=assertive (interrupting); focus order matches visual order when announced by SR. **WCAG 2.1 AA SC 4.1.2 Name, Role, Value** — verify all interactive elements have accessible name (label / aria-label / aria-labelledby). **Custom assertion:** no redundant ARIA (e.g., `role="button"` on `<button>` is redundant; use native button). |
| **visual-focus-appearance** | Layout Templates focus management anchors require focused inputs display `--border-focus` token (focus ring opacity and composition defined in design). Design excerpt focus ring token: `--border-focus` (outline composition and offset specified in design). Creator Brief does not name this trigger; however, compact-widget visual prominence ("always-on-top", "glanceable surface") + "Glance-readable from 2 meters" imply focus ring visibility is critical for keyboard users. | **Lighthouse a11y audit** check for visible focus indicator on interactive elements (built-in SC 2.4.7 Focus Visible check). **Custom Playwright assertion:** after Tab to each focusable element, inspect computed `outline` / `box-shadow` CSS property to verify `--border-focus` token applied. **Token binding:** `--border-focus` outline must meet ≥ 3:1 non-text contrast against background (SC 1.4.11). **WCAG 2.1 AA SC 2.4.7 Focus Visible** + **SC 1.4.11 Non-text Contrast** — verify outline visible on all interactive elements (buttons, inputs, links, disclosure toggles, tabs). |
| **target-size** | Creator Brief does not name touch surfaces explicitly (desktop-only app: Windows / macOS / Linux via Tauri 2). Design excerpt target size tokens: `--target-button-min` (serves SC 2.5.5 44×44 minimum) + `--target-input-min` (serves SC 2.5.5 44×44 minimum and SC 2.5.8 24×24 minimum; platform-native minimums defined in design). Compact-widget design implies small form factors. | **WCAG 2.1 AA SC 2.5.8 Target Size (Minimum)** assertion: measure button / input interactive area ≥ 24×24 CSS px (axe-core target-size rule when WCAG 2.2 AA enabled, or custom Playwright assertion measuring computed width/height of `[role="button"], button, input, a` elements). **Design token assertion:** verify `--target-button-min` and `--target-input-min` tokens applied to all interactive elements. **Conditional escalation to SC 2.5.5 AAA (44×44):** if project adds mobile web or touch mode, escalate. |

**No multi-language, regulated-compliance, or cognitive-accessibility triggers detected** in upstream-context. Creator Brief targets "developers staring at telemetry for hours" (professional users, no explicit cognitive disability accommodation). No internationalization signals in Design Excerpt. Security tier = Minimal (not Hardened). Phase 3 will apply default a11y discipline (plain language, consistent labeling) where it costs nothing extra.

---

## 6. A11y Tier

**Tier:** **Standard (1)**

**Justification:**

Andromeda Pulse is a **cross-platform desktop application** (Windows / macOS / Linux via Tauri 2.x) with **2 primary UI surfaces** (desktop-webview React SPA + desktop-native tray menu) plus an OS notification surface, spanning **8 assertable entities** with **7 must-be-accessible flows** (Critical Paths P1-P7). The **security tier is Minimal** (local-first, zero infrastructure, no user accounts, loopback-only OTLP), but **tests tier = Standard** (upstream-context Section 5 — cross-platform, multi-surface coordination, 7 critical paths, persistent in-memory data) and **obs tier = Standard** (8 instrumentable modules, 8 major surfaces, real-time push, security-sensitive logging vectors). **No WCAG compliance triggers** in security plan (Minimal tier — no Section 508 / ADA / EAA / EN 301 549 mandate). **Creator brief explicit rigor signals** include WCAG color discipline ("Status colors not-color-alone (paired with iconography per WCAG)"), reduced-motion respect ("respects `prefers-reduced-motion`"), typography legibility ("Glance-readable from 2 meters"), and motion-as-data principle (motion conveys state, not decoration). Creator brief names no blind / low-vision / cognitive-disability target users explicitly, but the accessibility-conscious design tokens (high-contrast palette, focus ring, motion budget, state color supplements) and agent-driven development style ("machine-runnable verification" with axe-core JSON / Lighthouse JSON / Playwright focus + keyboard) warrant **Standard WCAG 2.1 AA** with a **motion-sensitive AAA escalation** (SC 2.3.3 Animation from Interactions).

**Surface count (3 surfaces, 8 assertable entities) + must-be-accessible path count (7) + tests/obs tier alignment (both Standard) + 6 a11y triggers (visual-discrimination, motion-sensitive, keyboard-only, screen-reader-priority, visual-focus-appearance, target-size) + agent-driven discipline (axe-core JSON + Lighthouse JSON + pa11y JSON + Playwright focus/keyboard) justify Standard tier as the baseline.** No Comprehensive escalation: no regulated-compliance trigger, no cognitive-disability signals, security tier = Minimal, no multi-language target.