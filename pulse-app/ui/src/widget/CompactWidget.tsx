// Compact-widget surface: top-level component for the `compact-widget` Tauri
// window per layout-templates.md §Surface: desktop-webview / Wireframe —
// Compact widget. Titlebar (chunk #30, + connection dot chunk #89) at top /
// service constellation canvas (chunk #91, replacing the chunk #32 aggregated
// badge) in the middle / findings band (chunk #87) below.

import { useEffect, useRef, useState } from "react";
import { Titlebar } from "../components/Titlebar";
import { InvestigationModalForm } from "../dashboard/InvestigationModalForm";
import {
  InvestigationProvider,
  useInvestigation,
} from "../hooks/use-investigation";
import { useFindings } from "../hooks/use-findings";
import { useServiceConstellation } from "../hooks/use-service-constellation";
import {
  useToggleDashboard,
  useDashboardToggleShortcut,
} from "../hooks/use-toggle-dashboard";
import {
  openFindingsWindow,
  hideFindingsWindow,
  onFindingsDismissed,
} from "../hooks/use-findings-window";
import { ConstellationCanvas } from "./ConstellationCanvas";
import { FindingsCounter } from "./FindingsCounter";

export function CompactWidget() {
  return (
    <InvestigationProvider>
      <CompactWidgetContents />
    </InvestigationProvider>
  );
}

function CompactWidgetContents() {
  const { open, openInvestigation, closeInvestigation, triggerRef } =
    useInvestigation();
  const findings = useFindings();
  const services = useServiceConstellation();
  const onToggleDashboard = useToggleDashboard();
  useDashboardToggleShortcut();
  const { refetch: refetchFindings } = findings;
  const [findingsWindowOpen, setFindingsWindowOpen] = useState(false);
  const findingsTriggerRef = useRef<HTMLButtonElement | null>(null);
  useEffect(() => {
    if (findings.count === 0 && findingsWindowOpen) {
      void hideFindingsWindow();
      setFindingsWindowOpen(false);
    }
  }, [findings.count, findingsWindowOpen]);
  // The findings window signals dismissal (Esc / blur / mark-all-read) → flip
  // the badge shut, restore focus to it, and refetch the count (a11y-plan §5).
  useEffect(() => {
    return onFindingsDismissed(() => {
      setFindingsWindowOpen(false);
      findingsTriggerRef.current?.focus();
      void refetchFindings();
    });
  }, [refetchFindings]);
  const handleFindingsToggle = () => {
    setFindingsWindowOpen((prev) => {
      if (prev) {
        void hideFindingsWindow();
        return false;
      }
      void openFindingsWindow();
      return true;
    });
  };
  return (
    <>
      <Titlebar
        onInvestigateClick={openInvestigation}
        onToggleDashboardClick={onToggleDashboard}
      />
      <main
        id="main-content"
        tabIndex={-1}
        style={{
          display: "flex",
          flexDirection: "column",
          height: "calc(100vh - 32px)",
          background: "var(--color-base)",
          color: "var(--color-text-primary)",
          fontFamily: "var(--font-body)",
        }}
      >
        <div style={{ flex: 1, minHeight: 0, padding: "var(--spacing-md)" }}>
          <ConstellationCanvas items={services} />
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
            isOpen={findingsWindowOpen}
            onOpen={handleFindingsToggle}
            triggerRef={findingsTriggerRef}
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
    </>
  );
}
