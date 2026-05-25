// Compact-widget surface: top-level component for the `compact-widget` Tauri
// window per layout-templates.md §Surface: desktop-webview / Wireframe —
// Compact widget. Three-band wireframe: titlebar (chunk #30) at top / canvas
// with Halo-overlaid aggregated badge (chunk #32 §Step 5) in the middle /
// footer band (chunk #32 §Step 6) at bottom. Chunk #42 adds an Investigate
// telescope icon button to the titlebar (per layout-templates.md §Wireframe —
// Compact widget IA notes); the titlebar gear + Investigate button are the
// two tab stops on this surface.

import { useEffect, useRef, useState } from "react";
import { Titlebar } from "../components/Titlebar";
import { InvestigationModalForm } from "../dashboard/InvestigationModalForm";
import {
  InvestigationProvider,
  useInvestigation,
} from "../hooks/use-investigation";
import { useFindings } from "../hooks/use-findings";
import { AggregatedBadgeCanvas } from "./AggregatedBadgeCanvas";
import { FindingsCounter } from "./FindingsCounter";
import { FindingsDropdown } from "./FindingsDropdown";
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
  const findings = useFindings();
  const [findingsOpen, setFindingsOpen] = useState(false);
  const findingsTriggerRef = useRef<HTMLButtonElement | null>(null);
  useEffect(() => {
    if (findings.count === 0 && findingsOpen) {
      setFindingsOpen(false);
    }
  }, [findings.count, findingsOpen]);
  const handleMarkAllRead = () => {
    void findings.markAllRead();
    setFindingsOpen(false);
  };
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
        <div
          data-testid="findings-band"
          style={{
            position: "relative",
            display: "flex",
            justifyContent: "flex-end",
            padding: "0 var(--spacing-md) var(--spacing-xs)",
            flexShrink: 0,
            minHeight: findings.count > 0 ? undefined : 0,
          }}
        >
          <FindingsCounter
            count={findings.count}
            severity={findings.severityMax}
            isOpen={findingsOpen}
            onOpen={() => setFindingsOpen((prev) => !prev)}
            triggerRef={findingsTriggerRef}
          />
          <FindingsDropdown
            rows={findings.rows}
            isOpen={findingsOpen}
            onClose={() => setFindingsOpen(false)}
            onMarkAllRead={handleMarkAllRead}
            triggerRef={findingsTriggerRef}
            nowUnixNano={Date.now() * 1_000_000}
          />
        </div>
        <FooterBand
          throughputHz={metrics.throughputHz}
          errorRate={metrics.errorRate}
          retentionUsedSeconds={metrics.retentionUsedSeconds}
          retentionMaxSeconds={metrics.retentionMaxSeconds}
        />
      </main>
      <div
        role="status"
        aria-live="polite"
        data-testid="findings-live-region"
        style={{
          position: "absolute",
          width: 1,
          height: 1,
          margin: -1,
          padding: 0,
          overflow: "hidden",
          clip: "rect(0, 0, 0, 0)",
          whiteSpace: "nowrap",
          border: 0,
        }}
      >
        {findings.lastAnnouncement}
      </div>
      <InvestigationModalForm
        open={open}
        onClose={closeInvestigation}
        triggerRef={triggerRef}
      />
    </>
  );
}
