// Report window surface (2026-07-10) — the render root for the SEPARATE
// borderless `report` window that shows an incident's Diagnostic Report,
// positioned relative to the findings dropdown so opening it never stretches or
// moves the compact findings window. The incident id arrives via the
// cross-window REPORT_OPEN event; closing (✕ / Esc through the Report modal)
// hides the window and returns focus to the findings dropdown.

import { useEffect, useRef, useState } from "react";
import { Report } from "../report/Report";
import { onReportOpen, closeReportWindow } from "../hooks/use-findings-window";
import { useFindings } from "../hooks/use-findings";

export function ReportWindow() {
  const [incidentId, setIncidentId] = useState<number | null>(null);
  const findings = useFindings();
  const triggerRef = useRef<HTMLElement | null>(null);

  useEffect(() => onReportOpen((id) => setIncidentId(id)), []);

  // Normal path: the id is delivered by REPORT_OPEN when the window is opened.
  // Fallback (window shown without an id — e.g. an audit harness): the first
  // active incident, so the surface is never blank.
  const effectiveId = incidentId ?? findings.rows[0]?.id ?? null;

  const handleClose = () => {
    setIncidentId(null);
    void closeReportWindow();
  };

  return (
    <Report
      isOpen={effectiveId !== null}
      onClose={handleClose}
      incidentId={effectiveId}
      triggerRef={triggerRef}
      variant="fill"
    />
  );
}
