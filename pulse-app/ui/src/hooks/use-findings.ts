// Findings hook for the compact-widget Findings counter + dropdown
// (chunk #87). Fetches incident records via the existing chunk #78
// `incidents.list_active()` TauRPC procedure on mount and on window
// focus events, filters to unread non-Resolved rows per chunk #87
// project-doc §86 derivation ("counter derived via corpus query on
// incident events and widget focus, no separate state file"), and
// exposes the bulk `incidents.mark_all_read()` callback.
//
// Subscription posture: this chunk is PULL-only. No `pulse://stream/
// incidents` channel subscriber lives here because the chunk #78
// broadcast is emit-only per the upstream broadcast.rs docstring
// ("Webview subscription wiring lives in а future v0.2.0 chunk"). Live
// updates currently arrive только on the next widget-focus refetch.
// A future chunk wiring `streams.subscribe_incidents` can extend this
// hook with push-driven invalidation без changing the public API.
//
// jsdom posture mirrors `use-widget-metrics.ts` (chunk #57): all proxy
// invocations are wrapped в `.catch(() => {})` so jsdom test
// environments + pre-init Tauri context fall back to empty rows
// silently без console noise.

import { useCallback, useEffect, useState } from "react";
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

export function useFindings(): UseFindingsResult {
  const [records, setRecords] = useState<IncidentRecord[]>(EMPTY_RECORDS);
  const [lastAnnouncement, setLastAnnouncement] = useState<string>("");

  const refetch = useCallback(async (): Promise<void> => {
    const proxy = createTauRPCProxy();
    try {
      const payload = await proxy.incidents.list_active();
      setRecords(payload.items);
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
    return () => {
      window.removeEventListener("focus", handleFocus);
    };
  }, [refetch]);

  const rows = selectUnreadRows(records);
  const count = rows.length;
  const severityMax = maxPriorityTier(rows);

  useEffect(() => {
    if (count === 0) {
      // Suppress announcement on transient zero state — count=0 hides the
      // counter; subsequent re-mounts will re-announce when non-zero.
      return;
    }
    setLastAnnouncement(`Findings: ${count} unread`);
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
