// Compact-widget surface: top-level component for the `compact-widget` Tauri
// window per layout-templates.md §Surface: desktop-webview / Wireframe —
// Compact widget. Three-band wireframe: titlebar (chunk #30) at top / canvas
// with Halo-overlaid aggregated badge (chunk #32 §Step 5) in the middle /
// footer band (chunk #32 §Step 6) at bottom. Read-only — no new focusable
// interactive elements; the titlebar gear remains the only tab stop.

import { Titlebar } from "../components/Titlebar";
import { AggregatedBadgeCanvas } from "./AggregatedBadgeCanvas";
import { FooterBand } from "./FooterBand";
import type { WidgetMetrics } from "./widget-types";

interface CompactWidgetProps {
  metrics: WidgetMetrics;
}

export function CompactWidget({ metrics }: CompactWidgetProps) {
  return (
    <>
      <Titlebar />
      <main
        id="main-content"
        tabIndex={-1}
        style={{
          display: "flex",
          flexDirection: "column",
          minHeight: "calc(100vh - 32px)",
          background: "var(--color-base)",
          color: "var(--color-text-primary)",
          fontFamily: "var(--font-body)",
        }}
      >
        <div style={{ flex: 1, minHeight: 0, padding: "var(--spacing-md)" }}>
          <AggregatedBadgeCanvas
            serviceCount={metrics.serviceCount}
            throughputHz={metrics.throughputHz}
            errorRate={metrics.errorRate}
          />
        </div>
        <FooterBand
          throughputHz={metrics.throughputHz}
          errorRate={metrics.errorRate}
          retentionUsedSeconds={metrics.retentionUsedSeconds}
          retentionMaxSeconds={metrics.retentionMaxSeconds}
        />
      </main>
    </>
  );
}
