// Diagnostic Report hook (chunk #88). Encapsulates the TauRPC fetch +
// clipboard write + copy-state lifecycle so Report.tsx + ReportRenderer.tsx
// stay focused on rendering. The hook fetches when `incidentId` flips
// non-null (single fetch per open cycle); webview consumers close the
// Report and reopen it to re-fetch.
//
// Copy state machine: idle → copying → copied | error. The `copied`
// state auto-resets to idle after AUTO_RESET_MS so the live region
// announcement clears (per a11y plan §7 Live regions + CLAUDE.md
// §Critical Warnings clipboard hygiene rule — visible event accompanies
// every clipboard write).

import { useCallback, useEffect, useRef, useState } from "react";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { createTauRPCProxy } from "../bindings/index";
import type { CopyState, ReportPayload } from "./report-types";

export interface UseReportResult {
  report: ReportPayload | null;
  loading: boolean;
  error: string | null;
  copyState: CopyState;
  copyMarkdown: () => Promise<void>;
}

export const AUTO_RESET_MS = 2_000;

export function useReport(incidentId: number | null): UseReportResult {
  const [report, setReport] = useState<ReportPayload | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [copyState, setCopyState] = useState<CopyState>("idle");
  const cancelRef = useRef<boolean>(false);

  useEffect(() => {
    if (incidentId === null) {
      setReport(null);
      setLoading(false);
      setError(null);
      setCopyState("idle");
      return;
    }
    cancelRef.current = false;
    setReport(null);
    setLoading(true);
    setError(null);
    setCopyState("idle");
    (async () => {
      try {
        const proxy = await createTauRPCProxy();
        const result = await proxy.incidents.get_report(incidentId);
        if (cancelRef.current) {
          return;
        }
        setReport(result);
        setLoading(false);
      } catch (err) {
        if (cancelRef.current) {
          return;
        }
        const message =
          err instanceof Error ? err.message : "Failed to load report";
        setError(message);
        setLoading(false);
      }
    })();
    return () => {
      cancelRef.current = true;
    };
  }, [incidentId]);

  useEffect(() => {
    if (copyState !== "copied" && copyState !== "error") {
      return;
    }
    const handle = window.setTimeout(() => {
      setCopyState("idle");
    }, AUTO_RESET_MS);
    return () => {
      window.clearTimeout(handle);
    };
  }, [copyState]);

  const copyMarkdown = useCallback(async () => {
    if (report === null) {
      setCopyState("error");
      return;
    }
    setCopyState("copying");
    try {
      await writeText(report.markdown);
      setCopyState("copied");
    } catch {
      setCopyState("error");
    }
  }, [report]);

  return {
    report,
    loading,
    error,
    copyState,
    copyMarkdown,
  };
}
