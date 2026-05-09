// Synthetic data simulator for the compact-widget aggregated badge + footer
// band. Drives `WidgetMetrics` from a `setInterval` at slow cadence (1000ms)
// so SR users do not thrash on sub-second numeric churn — combined.md (a11y)
// live-region debouncing constraint. Real binding via
// `streams.subscribe_metrics` Arrow IPC bytes parser is deferred to chunks
// #34/#35 alongside other broadcast subscribers; this simulator IS the
// placeholder, replaceable when the backend emitters land.
//
// Cadence rationale: HaloCanvas needs frequent (~250ms) updates for smooth
// pulse rhythm + LCH hue interpolation; footer numbers need slower updates
// (~1000ms) for SR sanity. Two simulators (this hook + use-synthetic-halo-input)
// stay separate so each surface gets the cadence it needs.
//
// retentionUsedSeconds linearly ramps 0 → 600 over 30 seconds then resets;
// retentionMaxSeconds stays at 600 (10 minutes — matches the configured
// retention default per arch §Inherited Defaults). This visualizes the
// "Retention: Nm / 10m" footer line crossing meaningful values during a
// dev-loop session without binding to real `metric.buffer.memory_bytes`.

import { useEffect, useState } from "react";
import type { WidgetMetrics } from "../widget/widget-types";

const TICK_INTERVAL_MS = 1000;
const SERVICE_COUNT_CYCLE = [3, 5, 8, 12, 18, 24, 30, 24, 18, 12];
const THROUGHPUT_BASE_HZ = 1000;
const THROUGHPUT_AMPLITUDE_HZ = 800;
const ERROR_RATE_AMPLITUDE = 0.05;
const ERROR_RATE_CENTER = 0.05;
const RETENTION_MAX_SECONDS = 600;
const RETENTION_RAMP_TICKS = 30;

const INITIAL_METRICS: WidgetMetrics = {
  serviceCount: SERVICE_COUNT_CYCLE[0]!,
  throughputHz: THROUGHPUT_BASE_HZ,
  errorRate: ERROR_RATE_CENTER,
  retentionUsedSeconds: 0,
  retentionMaxSeconds: RETENTION_MAX_SECONDS,
};

export function useSyntheticWidgetMetrics(): WidgetMetrics {
  const [metrics, setMetrics] = useState<WidgetMetrics>(INITIAL_METRICS);

  useEffect(() => {
    let tick = 0;
    const interval = setInterval(() => {
      tick += 1;
      const phase = (tick / 12) * 2 * Math.PI;
      const throughputHz = THROUGHPUT_BASE_HZ + THROUGHPUT_AMPLITUDE_HZ * Math.sin(phase);
      const errorRate = ERROR_RATE_CENTER + ERROR_RATE_AMPLITUDE * Math.sin(phase * 0.7);
      const serviceCount = SERVICE_COUNT_CYCLE[tick % SERVICE_COUNT_CYCLE.length]!;
      const retentionUsedSeconds =
        ((tick % RETENTION_RAMP_TICKS) / RETENTION_RAMP_TICKS) * RETENTION_MAX_SECONDS;
      setMetrics({
        serviceCount,
        throughputHz,
        errorRate,
        retentionUsedSeconds,
        retentionMaxSeconds: RETENTION_MAX_SECONDS,
      });
    }, TICK_INTERVAL_MS);
    return () => clearInterval(interval);
  }, []);

  return metrics;
}
