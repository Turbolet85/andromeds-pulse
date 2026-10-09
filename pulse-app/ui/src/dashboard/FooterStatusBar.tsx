// Full-dashboard footer status bar slot per layout-templates.md §Component —
// Footer "Full dashboard": flex row, space-sm height, 0 space-md padding,
// space-md gap, 1px subtle Earth-Blue top border separating from view content.
// Empty in chunk #33 — control population (time-range picker, service filter,
// read-only ingest/error indicators) lands in chunks #34/#35.

import { ConnectionStatusLine } from "./ConnectionStatusLine";

export function FooterStatusBar() {
  return (
    <footer
      className="dashboard-footer"
      data-testid="dashboard-footer"
      style={{
        display: "flex",
        alignItems: "center",
        gap: "var(--spacing-md)",
        height: "var(--spacing-lg)",
        padding: "0 var(--spacing-md)",
        background: "var(--color-base)",
        borderTop: "1px solid rgba(74, 144, 226, 0.1)",
        color: "var(--color-text-secondary)",
        fontFamily: "var(--font-body)",
        fontSize: "12px",
      }}
    >
      <ConnectionStatusLine />
    </footer>
  );
}
