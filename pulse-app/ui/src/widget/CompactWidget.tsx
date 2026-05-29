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
import { useConnectionState } from "../hooks/use-connection-state";
import { throughputToActivityState } from "../halo/activity-state";
import { Report } from "../report/Report";
import { AggregatedBadgeCanvas } from "./AggregatedBadgeCanvas";
import { FindingsCounter } from "./FindingsCounter";
import { FindingsDropdown } from "./FindingsDropdown";
import type { WidgetMetrics } from "./widget-types";
import type { ConnectionState } from "../bindings/index";

interface CompactWidgetProps {
  metrics: WidgetMetrics;
}

// Neutral default while the connection state is still loading (jsdom /
// pre-init Tauri context, where use-connection-state returns null): a
// bound-but-idle receiver renders the halo at full color (no grayout).
const DEFAULT_CONNECTION_STATE: ConnectionState = { state: "Listening" };

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
  const connection = useConnectionState();
  const [findingsOpen, setFindingsOpen] = useState(false);
  const [reportIncidentId, setReportIncidentId] = useState<number | null>(null);
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
  const handleRowClick = (incidentId: number) => {
    setFindingsOpen(false);
    setReportIncidentId(incidentId);
  };
  const handleReportClose = () => {
    setReportIncidentId(null);
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
            errorRate={metrics.errorRate}
            connectionState={connection?.state ?? DEFAULT_CONNECTION_STATE}
            cumulativeSeverity={findings.severityMax}
            activityState={throughputToActivityState(metrics.throughputHz)}
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
            onRowClick={handleRowClick}
            triggerRef={findingsTriggerRef}
            nowUnixNano={Date.now() * 1_000_000}
          />
        </div>
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
      <Report
        isOpen={reportIncidentId !== null}
        onClose={handleReportClose}
        incidentId={reportIncidentId}
        triggerRef={findingsTriggerRef}
      />
    </>
  );
}
