## 3. Design System Excerpt

### Surfaces

- **desktop-webview** (Windows/macOS/Linux WebView2 + WKWebView with React 19 + Tailwind v4) — Service constellation dashboard and compact widget in browser environment; instrumentation via browser OTel SDK + web-vitals for TTI/LCP metrics on Halo State Pulse canvas updates and panel transitions.

- **desktop-native** (Windows/macOS/Linux tray via NotifyIcon/NSStatusItem/AppIndicator) — Always-visible tray icon with unified Halo State Pulse glow encoding service health; native telemetry via OS notification system (no frontend SDK — emit state changes via app logs or custom telemetry endpoint).

### Loading / Error / Empty State Patterns

- **Skeleton pulse** — visibility: background overlay on card/panel (opacity 50–100% discrete pulse at 1.2s cycle) — telemetry hook: observe skeleton duration as span within component-render lifecycle; metric: time from mount to content visibility (LCP proxy).

- **Empty state** — visibility: centered text + optional icon within container (color #7D8697 Tertiary) — telemetry hook: counter for empty-state occurrence per view (Traces / Metrics / Logs / Snapshots); error attribute if triggered by data-fetch failure.

- **Error state** — visibility: inline text below input field or modal body (color #8B2E3B Alert Burgundy, 1px border rgba(139, 46, 59, 0.5)) — telemetry hook: capture error message and error class; span attribute for user-facing error surface; duration of error state visibility.

### User-Facing Error Surfaces

- **Input field error** (location: inline below input) — appears for: form validation failure, API response error on field submission; recovery affordance: retry (form re-submit) or contact (support link if provided in error text) — feedback widget candidate: no (field-level scope too narrow for Sentry widget).

- **Modal error state** (location: modal body with Alert Burgundy border and text) — appears for: operation failure (e.g., snapshot generation failure, trace query failure); recovery affordance: retry button or dismiss + fallback action — feedback widget candidate: yes (modal contains sufficient context and user attention for feedback collection).

- **Notification (OS-native)** (location: system notification center) — appears for: snapshot generation completion status (success or failure), MCP server status change, update available; recovery affordance: action button ("View" snapshot, "Copy to Clipboard", or implicit dismiss) — feedback widget candidate: no (native notification scope outside web instrumentation; design defers to OS feedback mechanisms).
