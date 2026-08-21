// Findings hook for the compact-widget Findings counter + dropdown
// (chunk #87). Fetches incident records via the existing chunk #78
// `incidents.list_active()` TauRPC procedure on mount and on window
// focus events, filters to unread non-Resolved rows per chunk #87
// project-doc §86 derivation ("counter derived via corpus query on
// incident events and widget focus, no separate state file"), and
// exposes the bulk `incidents.mark_all_read()` callback.
//
// Subscription posture: PULL with a ~1s background re-poll (2026-07-10 CARRY,
// frontend.md 2026-07-06/07) so the badge reflects incidents as they arrive —
// previously fetch-once-at-mount + on window-focus left the badge hidden until
// a resize forced a focus refetch. The re-poll is a SILENT background refresh:
// `refetch` keeps the last-good records on error (never empties, no loading
// flag to re-flip). A future chunk wiring `streams.subscribe_incidents` can
// swap the poll for push-driven invalidation without changing the public API.
//
// jsdom posture mirrors `use-widget-metrics.ts` (chunk #57): all proxy
// invocations are wrapped in `.catch(() => {})` so jsdom test
// environments + pre-init Tauri context fall back to empty rows
// silently without console noise.

import { useCallback, useEffect, useRef, useState } from "react";
import { recordFindingsCounterRefresh } from "../canvas/frame-metrics";
import { createTauRPCProxy, type IncidentRecord } from "../bindings/index";
import type { FindingsRow } from "../widget/findings-types";
import { maxPriorityTier, selectUnreadRows } from "../widget/findings-types";
import type { PriorityTier } from "../bindings/index";

export interface UseFindingsResult {
  rows: FindingsRow[];
  count: number;
  severityMax: PriorityTier | null;
  markAllRead: () => Promise<void>;
  refetch: () => Promise<void>;
  lastAnnouncement: string;
}

const EMPTY_RECORDS: IncidentRecord[] = [];

// Background re-poll cadence (2026-07-10 CARRY) — mirrors the constellation's
// ~1s live-refresh precedent.
const FINDINGS_REPOLL_MS = 1000;

export function useFindings(): UseFindingsResult {
  const [records, setRecords] = useState<IncidentRecord[]>(EMPTY_RECORDS);
  const [lastAnnouncement, setLastAnnouncement] = useState<string>("");
  const prevCountRef = useRef(0);

  const refetch = useCallback(async (): Promise<void> => {
    const proxy = createTauRPCProxy();
    const startedAt = performance.now();
    try {
      const payload = await proxy.incidents.list_active();
      setRecords(payload.items);
      // P-045 counter-refresh bound: request → committed counter state. The
      // measure is webview-local by construction, so no backend reference is
      // needed. Fire-and-forget so a telemetry failure never stalls the poll.
      void recordFindingsCounterRefresh({ duration_ms: performance.now() - startedAt });
    } catch {
      // jsdom / pre-init Tauri context. Empty rows; no console noise.
    }
  }, []);

  const markAllRead = useCallback(async (): Promise<void> => {
    const proxy = createTauRPCProxy();
    try {
      await proxy.incidents.mark_all_read();
      setRecords((prev) =>
        prev.map((r) =>
          r.read_at_unix_nano === null && r.status !== "resolved"
            ? { ...r, read_at_unix_nano: Date.now() * 1_000_000 }
            : r,
        ),
      );
      setLastAnnouncement("All findings marked as read");
    } catch {
      // jsdom / pre-init Tauri context. State unchanged; no console noise.
    }
  }, []);

  useEffect(() => {
    void refetch();
    const handleFocus = () => {
      void refetch();
    };
    window.addEventListener("focus", handleFocus);
    const interval = setInterval(() => {
      void refetch();
    }, FINDINGS_REPOLL_MS);
    return () => {
      window.removeEventListener("focus", handleFocus);
      clearInterval(interval);
    };
  }, [refetch]);

  const rows = selectUnreadRows(records);
  const count = rows.length;
  const severityMax = maxPriorityTier(rows);

  useEffect(() => {
    const prev = prevCountRef.current;
    prevCountRef.current = count;
    // Announce ONCE on the 0→N edge (SC 4.1.3, a11y.md 2026-07-07) — never per
    // re-poll tick, never on a subsequent count change (2→3), which would spam
    // the live region now that the hook re-polls.
    if (count > 0 && prev === 0) {
      setLastAnnouncement(`Findings: ${count} unread`);
    }
  }, [count]);

  return {
    rows,
    count,
    severityMax,
    markAllRead,
    refetch,
    lastAnnouncement,
  };
}
