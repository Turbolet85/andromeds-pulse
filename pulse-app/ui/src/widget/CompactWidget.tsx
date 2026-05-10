// Compact-widget surface: top-level component for the `compact-widget` Tauri
// window per layout-templates.md §Surface: desktop-webview / Wireframe —
// Compact widget. Three-band wireframe: titlebar (chunk #30) at top / canvas
// with Halo-overlaid aggregated badge (chunk #32 §Step 5) in the middle /
// footer band (chunk #32 §Step 6) at bottom. Chunk #42 adds an Investigate
// telescope icon button to the titlebar (per layout-templates.md §Wireframe —
// Compact widget IA notes); the titlebar gear + Investigate button are the
// two tab stops on this surface.

import { Titlebar } from "../components/Titlebar";
import { InvestigationModalForm } from "../dashboard/InvestigationModalForm";
import {
  InvestigationProvider,
  useInvestigation,
} from "../hooks/use-investigation";
import { AggregatedBadgeCanvas } from "./AggregatedBadgeCanvas";
import { FooterBand } from "./FooterBand";
import type { WidgetMetrics } from "./widget-types";

interface CompactWidgetProps {
  metrics: WidgetMetrics;
}

export function CompactWidget({ metrics }: CompactWidgetProps) {
  return (
    <InvestigationProvider>
      <CompactWidgetContents metrics={metrics} />
    </InvestigationProvider>
  );
}

function CompactWidgetContents({ metrics }: CompactWidgetProps) {
  const { open, openInvestigation, closeInvestigation, triggerRef } =
    useInvestigation();
  return (
    <>
      <Titlebar onInvestigateClick={openInvestigation} />
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
      <InvestigationModalForm
        open={open}
        onClose={closeInvestigation}
        triggerRef={triggerRef}
      />
    </>
  );
}
