// Real-data ingest-stats hook for the plain-language connection-status line
// (P-070). Polls the existing top-level `ready()` TauRPC on mount, on a 1s
// interval, and on window focus; derives spans/s from the cumulative
// `rows_ingested` delta and surfaces the buffer fill + retention window.
//
// PULL-only + silent-background-refresh posture mirrors use-connection-state.ts
// / use-traces.ts: keep the last-good value on a post-load re-poll error
// (jsdom / pre-init Tauri context) so the line never blanks mid-poll.

import { useEffect, useRef, useState } from "react";
import { createTauRPCProxy, type ReadyEnvelope } from "../bindings/index";

const POLL_INTERVAL_MS = 1000;

// Test seam mirroring use-traces.ts: an injected fake proxy for jsdom tests.
type ReadyProxy = { ready: () => Promise<ReadyEnvelope> };
let proxyOverride: ReadyProxy | null = null;
export function __setProxyForTest(proxy: ReadyProxy | null): void {
  proxyOverride = proxy;
}

export interface IngestStats {
  // null until a second sample exists (no fabricated 0-rate on the first poll).
  spansPerSec: number | null;
  bufferUsedSeconds: number;
  retentionSeconds: number;
  hasData: boolean;
}

export function useIngestStats(): IngestStats | null {
  const [stats, setStats] = useState<IngestStats | null>(null);
  const prevRef = useRef<{ rows: number; atMs: number } | null>(null);

  useEffect(() => {
    const proxy: ReadyProxy = proxyOverride ?? createTauRPCProxy();

    const poll = () => {
      proxy
        .ready()
        .then((env) => {
          const checks = env.checks;
          const nowMs = Date.now();
          let spansPerSec: number | null = null;
          const prev = prevRef.current;
          if (prev !== null) {
            const elapsedSec = (nowMs - prev.atMs) / 1000;
            if (elapsedSec > 0) {
              const delta = checks.rows_ingested - prev.rows;
              spansPerSec = delta > 0 ? delta / elapsedSec : 0;
            }
          }
          prevRef.current = { rows: checks.rows_ingested, atMs: nowMs };
          setStats({
            spansPerSec,
            bufferUsedSeconds: checks.buffer_used_seconds,
            retentionSeconds: checks.retention_seconds,
            hasData: checks.rows_ingested > 0,
          });
        })
        .catch(() => {
          // Silent background refresh: keep last-good on a post-load re-poll
          // failure (jsdom / pre-init Tauri context) — never blank the line.
        });
    };

    poll();
    const interval = window.setInterval(poll, POLL_INTERVAL_MS);
    const handleFocus = () => {
      poll();
    };
    window.addEventListener("focus", handleFocus);

    return () => {
      window.clearInterval(interval);
      window.removeEventListener("focus", handleFocus);
    };
  }, []);

  return stats;
}
