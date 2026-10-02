## 3. Design System Excerpt

### Surfaces

- **desktop-webview** (React 19 + Tailwind CSS v4 webview) — Windows/macOS/Linux WebView2/WKWebView hosting a React dashboard with compact widget and full dashboard expansion, using WebGPU canvas for real-time telemetry visualization.
- **desktop-native** (Tauri 2 native integration) — Windows/macOS/Linux tray icon and menu providing always-visible access to snapshot generation, MCP server toggle, and settings without requiring the webview window.

### Layout Categories

- **Compact widget** — used in: desktop-webview (primary surface, quarter-screen size with titlebar and canvas).
- **Full dashboard** — used in: desktop-webview (expanded window with sidebar/tab navigation to Traces / Metrics / Logs / Snapshots / Settings views).
- **Tray icon and menu** — used in: desktop-native (always-visible status icon with context menu for actions and app control).

### Brand Identity Anchors

- **Earth Blue (#4A90E2)** (primary color, 30% opacity for default borders, 60% opacity for active states, solid for focus rings) — drives selector for: active/focused states, healthy service status, navigation focus, control emphasis.
- **Alert Burgundy (#8B2E3B)** (accent color for error states) — drives selector for: error/alert states, anomaly indicators, error text and borders.
- **Status White-Blue (#E8EEF7)** (primary text color, ~8.5:1 contrast) — drives selector for: critical display text and primary labels distinguishable from secondary/tertiary text hierarchy.
