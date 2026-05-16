// Real-data widget metrics hook. Subscribes to the existing
// `streams.subscribe_metrics` TauRPC procedure (chunk #23 substrate at
// `pulse-app/src/streams.rs:17`) and exposes a stable `WidgetMetrics`
// shape for the compact-widget surface. First webview consumer of any
// `streams.*` Channel API procedure.
//
// Aggregation posture (chunk #57 minimum scope; refactored in chunk #81
// per route §2 Epoch 9 entry "Halo retains errorRate/throughputHz shape
// (refactored in chunk #81)"):
// - throughputHz: rolling count of Channel payloads received per 1-second
//   window. Each payload is one Arrow IPC batch from `buffer::BroadcastSenders`;
//   counting batches as a proxy for ingest rate is honest at chunk #57
//   level (precise per-point throughput requires Arrow IPC decode, deferred
//   to chunk #81's real-error-metric pipeline).
// - errorRate: 0 pre-chunk-#81. Real error-rate derivation requires
//   decoding the metric stream payload + locating an error-marker metric;
//   that pipeline is the chunk #81 refactor scope.
// - serviceCount: 0 pre-chunk-#81. Distinct service.name aggregation needs
//   Arrow IPC decode of resource attributes.
// - retentionMaxSeconds: from the configured `retention_seconds` user
//   setting (via `get_settings()` TauRPC on mount); falls back to 600
//   when the setting fetch fails (jsdom or first-boot before settings
//   persisted).
// - retentionUsedSeconds: 0 pre-chunk-#81. No TauRPC surface exposes
//   buffer used-bytes / earliest-ts; chunk #81 will introduce the source.
//
// Per obs-plan §11 Logs anti-pattern row 4 + Metrics anti-pattern row 2,
// the hook emits ZERO `tracing`/`console.*` per Arrow IPC payload — only
// ref-increment + 1s rollup. The backend producer at `pulse-app/src/streams.rs`
// already emits `tauri.channel.emit` per payload at INFO; webview-side
// duplication would cardinality-bomb `agent-latest.jsonl`.
//
// jsdom posture: `createTauRPCProxy()` itself does not throw (returns a
// Proxy); the invoke calls inside `proxy.get_settings()` /
// `proxy.streams.subscribe_metrics(...)` reject when `__TAURI_INTERNALS__`
// is absent. `.catch(() => {})` keeps the hook silent in test environments
// + production-time TauRPC failure (the visible effect is throughputHz
// staying at 0; no console noise).

import { useEffect, useRef, useState } from "react";
import { createTauRPCProxy } from "../bindings/index";
import type { WidgetMetrics } from "../widget/widget-types";

const RETENTION_MAX_SECONDS_DEFAULT = 600;
const RECOMPUTE_INTERVAL_MS = 1000;

const INITIAL_METRICS: WidgetMetrics = {
  serviceCount: 0,
  throughputHz: 0,
  errorRate: 0,
  retentionUsedSeconds: 0,
  retentionMaxSeconds: RETENTION_MAX_SECONDS_DEFAULT,
};

export function useWidgetMetrics(): WidgetMetrics {
  const [metrics, setMetrics] = useState<WidgetMetrics>(INITIAL_METRICS);
  const payloadCountRef = useRef(0);
  const retentionMaxRef = useRef(RETENTION_MAX_SECONDS_DEFAULT);

  useEffect(() => {
    const proxy = createTauRPCProxy();

    proxy
      .get_settings()
      .then((settings) => {
        if (typeof settings.retention_seconds === "number") {
          retentionMaxRef.current = settings.retention_seconds;
        }
      })
      .catch(() => {
        // Silent: jsdom / pre-init Tauri context. Retention max stays at default.
      });

    proxy.streams
      .subscribe_metrics(() => {
        payloadCountRef.current += 1;
      })
      .catch(() => {
        // Silent: jsdom / pre-init Tauri context. throughputHz stays at 0.
      });

    const interval = window.setInterval(() => {
      const count = payloadCountRef.current;
      payloadCountRef.current = 0;
      setMetrics({
        serviceCount: 0,
        throughputHz: count * (1000 / RECOMPUTE_INTERVAL_MS),
        errorRate: 0,
        retentionUsedSeconds: 0,
        retentionMaxSeconds: retentionMaxRef.current,
      });
    }, RECOMPUTE_INTERVAL_MS);

    return () => {
      window.clearInterval(interval);
    };
  }, []);

  return metrics;
}
