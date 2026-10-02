# A11y Screen Reader Manual-Pass Spec — Orca + Firefox (Linux)

**Pairing:** Orca 48.x + Firefox on Linux (GNOME 48+, KDE Plasma, or other
desktop env with AT-SPI accessibility bus running)
**Navigation mode:** caret browsing mode arrow keys (F7 in Firefox toggles
caret browsing; arrow keys cycle reading positions; H jumps headings; D
jumps landmarks; B between buttons; F between form fields; T between
tables; KP_Insert+H opens Orca commands; Insert+Space toggles flat review)
**Surface kind:** `webview` for desktop-webview targets;
`os-native` (AppIndicator / StatusNotifier API) for desktop-native targets
**Reduced-motion preference:** GNOME Settings → Accessibility → Reduce
animations ON (or `gsettings set org.gnome.desktop.interface enable-animations false`)
**`screen_reader`** field value in JSON output: `"Orca"`
**`navigation_mode`** field value: `"caret_mode"`

This fixture is SUPPLEMENTAL evidence per `a11y-plan.md` §11 Universal:
automated axe-core / Lighthouse / pa11y CI gates remain the primary a11y
verification path. See `README.md` for the supplemental-not-sole-gate
discipline.

> **Surface placeholder note:** Surfaces ship at chunks #25 (webview shell)
> / #28 (compact widget) / #33 (full dashboard) / #36 (tray icon) / #37+#38
> (modals). Fixture spec authored at chunk #14 to set the manual-pass
> discipline now; per-path runs block on those chunks landing. Use
> `status: "blocked"` in JSON output when running against a not-yet-shipped
> surface.

> **Linux tray-icon note:** desktop env determines the tray API surface
> (StatusNotifierItem on KDE / Wayland-friendly envs;
> `libayatana-appindicator` on legacy GNOME extensions). Tauri 2 abstracts
> the API at the binding layer; Orca exercises whichever AT-SPI bridge the
> running desktop publishes. Record the active backend in
> `fields.metadata.tray_backend` if known (e.g., `"appindicator"` or
> `"status-notifier-item"`).

## Manual pass spec — canonical 9-step sequence (per a11y-plan §3)

For each Critical Path below, perform the following sequence and record
observations in JSON output (see "Output" section). Steps that don't
apply to a path are marked N/A.

1. **Boot:** start `pulse-app` per `xtask harness:status`; wait for
   `subsystems.{otlp_grpc_receiver,otlp_http_receiver,buffer,ingest_channel}.status == "initialized"|"ready"`.
2. **Activate SR:** start Orca 48.x (`orca` from terminal, or
   Super+Alt+S system shortcut depending on desktop env); confirm
   announcement chime.
3. **Navigate via SR commands:** in Firefox, F7 enables caret browsing
   mode; arrow keys cycle reading positions; H/D/B/F/T quick nav between
   headings/landmarks/buttons/forms/tables; Tab/Shift+Tab cycle focusable
   elements; KP_Insert+Space toggles flat review for raw screen content.
4. **Assert label / heading / role announcement:** for each interactive
   element under test, verify Orca announces the accessible name + role +
   relevant state.
5. **Verify error message ARIA:** for forms with validation, trigger an
   invalid state; verify Orca announces the error via `aria-describedby`
   association on focus.
6. **Verify toast aria-live:** polite `aria-live="polite"`
   announcements appear without stealing focus; assertive
   `aria-live="assertive"` interrupts current speech.
7. **Verify modal title + close button:** dialogs announce title at open
   + Tab cycles within the focus trap + Esc closes + focus returns to
   triggering element.
8. **Verify interactive role + state:** for switches/sliders/comboboxes,
   verify Orca announces both role and current state.
9. **Verify WebGPU canvas region label:** for the Halo State Pulse +
   chart canvas regions, verify Orca announces the wrapping
   `<region role="region" aria-label="...">` semantic label, NOT the raw
   `<canvas>` element (canvas itself MUST be `aria-hidden="true"`).

## Critical Paths P1–P7

### P1 — Full dashboard Traces

**Surface:** `desktop-webview/full-dashboard/traces`
**WCAG SCs:** SC 1.3.1, SC 4.1.2, SC 2.4.3, SC 1.4.3
**ARIA pattern (per a11y-plan §4):** landmark roles + table
**Steps applied:** 1, 2, 3, 4, 9

Boot the app, navigate to the Traces tab. Orca pass:

- D (next landmark) → "Telemetry traces chart, region" (verifies the
  `<region role="region" aria-label="Telemetry traces chart">` wrapper
  around the WebGPU canvas; the `<canvas>` element MUST NOT be
  announced).
- T (next table) → "Trace ID, Service, Duration, Status, table with N
  rows" (verifies sortable trace data table column headers).
- Arrow Down within table → row-by-row reading; column headers re-announce
  on column boundary cross.
- Tab → cycles between sortable column headers + filter inputs; natural
  DOM order, no `tabindex > 0` warpings.
- Halo State Pulse pulse rhythm is decorative; Orca does NOT describe
  canvas pixel motion (canvas itself is `aria-hidden`).

Sample JSON record:

```json
{
  "timestamp": "2026-05-15T10:23:14.572Z",
  "level": "info",
  "target": "a11y.screen_reader.pass.full_dashboard_traces",
  "message": "P1 traces canvas region semantic label verified",
  "fields": {
    "surface": "desktop-webview/full-dashboard/traces",
    "surface_kind": "webview",
    "screen_reader": "Orca",
    "navigation_mode": "caret_mode",
    "wcag_sc": "SC 1.3.1",
    "critical_path": "P1",
    "status": "blocked",
    "tool": "manual",
    "sr_announcement_expected": "Telemetry traces chart, region",
    "sr_announcement_actual": "(blocked - surface ships at chunk #33 full dashboard shell)"
  }
}
```

### P2 — Investigation modal

**Surface:** `desktop-webview/full-dashboard/traces` (modal layered on top)
**WCAG SCs:** SC 1.3.1, SC 4.1.2, SC 2.4.3, SC 4.1.3, SC 2.1.2 (focus trap)
**ARIA patterns (per a11y-plan §4):** dialog + slider + status (aria-live)
**Steps applied:** 1, 2, 3, 4, 6, 7, 8

From P1's Traces table, trigger Investigation. Orca pass:

- On modal open: Orca announces "Investigation Snapshot, dialog" (verifies
  `<dialog aria-label="Investigation Snapshot">`).
- Tab cycle within modal: token budget slider → Generate button → Cancel
  button → close `✕` button → loops back to slider.
- Slider focused: Orca announces "Token budget, slider, 25000, minimum
  10000, maximum 50000" + arrow up/down adjusts value.
- Generate button activated: Orca announces "Generating, busy" within
  100ms (verifies `aria-busy="true"`).
- Progress: `<status aria-live="polite">` announces stages.
- On completion: `<status aria-live="assertive">` announces "Snapshot
  ready, 23145 tokens".
- Esc → modal closes; focus returns to triggering button.

Sample JSON record:

```json
{
  "timestamp": "2026-05-15T10:25:02.108Z",
  "level": "info",
  "target": "a11y.screen_reader.pass.investigation_modal",
  "message": "P2 dialog title + slider role/state + aria-live progress verified",
  "fields": {
    "surface": "desktop-webview/full-dashboard/traces",
    "surface_kind": "webview",
    "screen_reader": "Orca",
    "navigation_mode": "caret_mode",
    "wcag_sc": "SC 4.1.2",
    "critical_path": "P2",
    "status": "blocked",
    "tool": "manual",
    "sr_announcement_expected": "Investigation Snapshot, dialog",
    "sr_announcement_actual": "(blocked - Investigation modal ships at chunk #37)"
  }
}
```

### P3 — Settings modal MCP toggle

**Surface:** `desktop-webview/full-dashboard/settings` (modal)
**WCAG SCs:** SC 4.1.2 (switch role+state), SC 2.1.1 (keyboard
activation), SC 4.1.3 (status messages)
**ARIA patterns (per a11y-plan §4):** switch + status-alert
**Steps applied:** 1, 2, 3, 4, 6, 8

Open Settings (likely Cmd+, or settings gear in titlebar). Navigate to
MCP toggle. Orca pass:

- Switch focused: Orca announces "MCP server, switch, off" (verifies
  `<switch aria-checked="false">`).
- Space activates: state changes; Orca announces "MCP server, switch, on"
  (focus stays on switch during state change per a11y-plan P3).
- Linux desktop notification fires for the toggle: verified separately
  via the `desktop-native/notifications` fixture (libnotify /
  freedesktop notification spec).

Sample JSON record:

```json
{
  "timestamp": "2026-05-15T10:27:18.443Z",
  "level": "info",
  "target": "a11y.screen_reader.pass.settings_mcp_toggle",
  "message": "P3 switch role+state + post-toggle announcement verified",
  "fields": {
    "surface": "desktop-webview/full-dashboard/settings",
    "surface_kind": "webview",
    "screen_reader": "Orca",
    "navigation_mode": "caret_mode",
    "wcag_sc": "SC 4.1.2",
    "critical_path": "P3",
    "status": "blocked",
    "tool": "manual",
    "sr_announcement_expected": "MCP server, switch, off -> on",
    "sr_announcement_actual": "(blocked - Settings modal ships at chunk #38)"
  }
}
```

### P4 — Live trace list

**Surface:** `desktop-webview/full-dashboard/traces` (live region under table)
**WCAG SCs:** SC 4.1.3 (status messages), SC 2.1.1 (keyboard reach)
**ARIA pattern (per a11y-plan §4):** status-alert (aria-live polite)
**Steps applied:** 1, 2, 3, 6

Load Traces tab; ingest synthetic OTLP traffic via OTLP client to `:4317`
(synthetic only per `security-plan.md` §Logging fixture hygiene). Orca
pass:

- Live region announces "1247 new traces ingested" without interrupting
  current reading position (verifies `<region aria-live="polite">`).
- Live region rate is throttled (no announcement floods).
- Tab focus does NOT shift to the live region on update (verifies live
  updates do NOT steal focus per a11y-plan P4).

Sample JSON record:

```json
{
  "timestamp": "2026-05-15T10:30:55.901Z",
  "level": "info",
  "target": "a11y.screen_reader.pass.live_trace_list",
  "message": "P4 live region polite + no focus steal verified under sustained ingest",
  "fields": {
    "surface": "desktop-webview/full-dashboard/traces",
    "surface_kind": "webview",
    "screen_reader": "Orca",
    "navigation_mode": "caret_mode",
    "wcag_sc": "SC 4.1.3",
    "critical_path": "P4",
    "status": "blocked",
    "tool": "manual",
    "sr_announcement_expected": "1247 new traces ingested",
    "sr_announcement_actual": "(blocked - live trace stream ships at chunk #33)"
  }
}
```

### P5 — Compact widget Esc → tray restore

**Surface:** `desktop-webview/compact-widget` (+ `desktop-native/tray-icon`
on restore)
**WCAG SCs:** SC 2.1.1 (keyboard), SC 2.4.3 (focus order), SC 4.1.2 (button labels)
**ARIA pattern (per a11y-plan §4):** button (with explicit aria-label)
**Steps applied:** 1, 2, 3, 4, 8

Boot app in compact widget mode (default startup). Orca pass:

- Tab cycles through compact widget chrome:
  1. Custom titlebar drag region — decorative, not focusable
  2. Settings gear button → "Settings, button"
  3. Dashboard-toggle button → "Toggle dashboard, button" (verifies
     `<button aria-label="Toggle dashboard">`)
  4. Minimize button → "Minimize to tray, button" (verifies
     `<button aria-label="Minimize to tray">`)
  5. Window controls (minimize / maximize / close) right side per
     Linux convention
- Activating Minimize (Enter or Space): widget minimizes to tray; focus
  returns to the tray icon (verified separately via the
  `desktop-native/tray-icon` fixture using AppIndicator/StatusNotifier API).
- Esc on widget triggers the same minimize → tray flow per a11y-plan P5.

Sample JSON record:

```json
{
  "timestamp": "2026-05-15T10:33:42.234Z",
  "level": "info",
  "target": "a11y.screen_reader.pass.compact_widget_esc_tray",
  "message": "P5 widget minimize button label + Esc -> tray restore verified",
  "fields": {
    "surface": "desktop-webview/compact-widget",
    "surface_kind": "webview",
    "screen_reader": "Orca",
    "navigation_mode": "caret_mode",
    "wcag_sc": "SC 4.1.2",
    "critical_path": "P5",
    "status": "blocked",
    "tool": "manual",
    "sr_announcement_expected": "Minimize to tray, button",
    "sr_announcement_actual": "(blocked - compact widget shell ships at chunk #28)"
  }
}
```

### P6 — Snapshot completion

**Surface:** `desktop-webview/full-dashboard/snapshots` (modal/inline) +
`desktop-native/notifications`
**WCAG SCs:** SC 4.1.3 (status), SC 2.4.3 (focus order on completion)
**ARIA patterns (per a11y-plan §4):** status-alert (aria-live assertive)
**Steps applied:** 1, 2, 3, 6

Trigger snapshot generation from P2 Investigation flow OR Snapshots tab.
On completion:

- `<status aria-live="assertive">` announces "Snapshot ready, 23145
  tokens" (verifies assertive aria-live announcement on final completion
  per a11y-plan P6).
- Linux desktop notification fires; Orca announces via libnotify /
  freedesktop notification API (separate fixture under
  `desktop-native/notifications`).
- Focus does NOT shift on completion announcement (assertive interrupts
  speech, not focus).

Sample JSON record:

```json
{
  "timestamp": "2026-05-15T10:36:11.778Z",
  "level": "info",
  "target": "a11y.screen_reader.pass.snapshot_completion",
  "message": "P6 assertive completion announcement + OS notification verified",
  "fields": {
    "surface": "desktop-webview/full-dashboard/snapshots",
    "surface_kind": "webview",
    "screen_reader": "Orca",
    "navigation_mode": "caret_mode",
    "wcag_sc": "SC 4.1.3",
    "critical_path": "P6",
    "status": "blocked",
    "tool": "manual",
    "sr_announcement_expected": "Snapshot ready, 23145 tokens",
    "sr_announcement_actual": "(blocked - snapshot pipeline ships at chunks #39-#43)"
  }
}
```

### P7 — Settings form

**Surface:** `desktop-webview/full-dashboard/settings`
**WCAG SCs:** SC 1.3.1 (form structure), SC 3.3.1 + SC 3.3.2 (form labels +
error association), SC 2.4.3 (focus order), SC 2.1.2 (no focus trap outside
modal)
**ARIA patterns (per a11y-plan §4):** form input + radiogroup + switch +
dialog (Settings is itself modal)
**Steps applied:** 1, 2, 3, 4, 5, 7, 8

Open Settings modal. Orca pass:

- Dialog announces: "Settings, dialog".
- Tab cycle (per a11y-plan P7): theme radiogroup → widget position
  radiogroup → retention input → MCP server switch → snapshot preset
  combobox → Save button → Cancel button → loops to theme.
- Each input has visible `<label>` + Orca reads the label on focus
  (e.g., "Theme, dark, radio button, 1 of 3 selected").
- Invalid retention input (e.g., "abc"): on blur, Orca announces
  "Invalid value: must be number 60 to 3600" via `aria-describedby`
  association + focus shifts to the invalid field on Save attempt.
- Esc closes Settings → focus returns to triggering element (settings
  gear or Cmd+, source).

Sample JSON record:

```json
{
  "timestamp": "2026-05-15T10:39:24.012Z",
  "level": "info",
  "target": "a11y.screen_reader.pass.settings_form",
  "message": "P7 form labels + invalid focus shift verified",
  "fields": {
    "surface": "desktop-webview/full-dashboard/settings",
    "surface_kind": "webview",
    "screen_reader": "Orca",
    "navigation_mode": "caret_mode",
    "wcag_sc": "SC 3.3.1",
    "critical_path": "P7",
    "status": "blocked",
    "tool": "manual",
    "sr_announcement_expected": "Theme, dark, radio button, 1 of 3 selected",
    "sr_announcement_actual": "(blocked - Settings modal ships at chunk #38)"
  }
}
```

## Defects-recorded checklist (per a11y-plan §3)

For each Critical Path above, record any defects observed using these
categories:

- [ ] SR announcements **missing** (element exists but Orca silent)
- [ ] SR announcements **out of order** (announcement order differs from
  visual reading order; e.g., button announced before its label)
- [ ] SR announcements **conflicting with visual state** (e.g., toggle
  visually shows ON but Orca announces "off, switch")

When ANY checkbox is checked, the corresponding JSON record's `status`
field MUST be `"fail"` (severe mismatch) or `"partial"` (minor deviation).
Pass only when all three checkboxes remain unchecked for that Critical Path.

## Custom Icon glyph announcements (per design-system §Iconography)

When Orca encounters icons in the surface chrome, the `aria-label` of the
wrapping element MUST use the custom Icon registry domain glyph name, NOT
a generic library name. Verified glyphs:

- `aperture` → expected: "Aperture" or domain context like
  "Snapshot capture"
- `telescope` → expected: "Telescope" or domain context
- `constellation-grid` → expected: "Constellation grid" or "Service map"
- `star` → expected: "Star" or "Favorite"
- `circular-pulse` → expected: "Circular pulse" or "Live status"

Generic library names are an anti-pattern per `design-system.md`
§Anti-Patterns Rejected Defaults item 5; record as a `fail` if Orca
announces the generic name instead of the domain glyph name.

## Reduced-motion variant (per design-system §Motion + a11y-plan §SC 2.3.3)

Every motion-bearing surface (P1 Halo Pulse on Traces canvas; P2
Investigation Capture Collapse; P5 widget minimize-to-tray transition; P6
snapshot completion announcement timing) has a reduced-motion variant
under Linux reduced-motion preference (GNOME Settings → Accessibility →
Reduce animations ON, or `gsettings set org.gnome.desktop.interface
enable-animations false`).

Run each Critical Path twice: once with motion ON (default), once with
reduced-motion ON. Record both runs as separate JSON records with a
distinguishing field in `fields.metadata`:

```json
{
  "fields": {
    "metadata": {
      "reduced_motion": true,
      "run_id": "550e8400-e29b-41d4-a716-446655440000"
    }
  }
}
```

Halo State Pulse under reduced-motion: announce as static state (e.g.,
"Live status, healthy" — hue/state still surfaced via aria-live; rhythm
dropped per design-system §Motion exemption).

## Output

Write JSON output to:

```
pulse-app/ui/test-results/a11y-sr/orca-<surface_slug>-<ISO-date>.jsonl
```

Where `<surface_slug>` is the surface ID with `/` replaced by `_`
(e.g., `desktop-webview_full-dashboard_traces`) and `<ISO-date>` is the
local date of the pass run (e.g., `2026-05-15`). One JSON record per
assertion. Each record validates against `schema.json` (draft 2020-12).
