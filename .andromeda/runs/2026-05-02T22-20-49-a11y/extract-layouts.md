## 4. Layout Templates Excerpt

### Layout Types per Surface

- **desktop-webview:** compact-widget, full-dashboard-traces, full-dashboard-metrics, full-dashboard-logs, full-dashboard-snapshots, settings-modal, investigation-modal
- **desktop-native:** tray-icon, tray-menu, notifications, file-picker

### Error Boundary Placement

(No explicit error boundary placement in layouts — Phase 3 will recommend defaults per layout category, typically section-level for dashboards / page-root for forms with focus-on-error pattern.)

### Focus Management Anchors

- **compact-widget:** Esc key minimizes to tray (focus returns to tray icon on restoration)
- **full-dashboard:** Tab navigates through tabs/sidebar items; Enter / Space activates tab; Esc closes modal (Settings or Investigation)
- **settings-modal:** Tab cycles through form controls (theme selector, widget snap position, retention input, MCP toggle); primary Save button and secondary Cancel button are keyboard-accessible; focused inputs display --border-focus token (focus ring opacity and composition defined in design)
- **investigation-modal:** close button (✕ glyph) in top-right corner; Esc closes modal (explicit escape path for keyboard users)
- **tray-menu:** arrow keys navigate menu items, Return / Space selects, Escape closes menu

### Heading Hierarchy Anchors

(No explicit heading hierarchy in layouts — Phase 3 will recommend WCAG SC 1.3.1 + SC 2.4.6 defaults: single h1 per page, sequential h2 / h3 nesting, main + navigation + contentinfo landmarks.)