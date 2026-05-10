import { useEffect, useMemo, useRef, useState } from "react";
import type { MetricRow } from "../../../bindings";
import { Fallback } from "../../../canvas/Fallback";
import { createFrameLoop } from "../../../canvas/frame-loop";
import {
  detectWebviewBackend,
  normalizeWgpuBackend,
  recordFrameMs,
} from "../../../canvas/frame-metrics";
import {
  requestWebGPUAdapter,
  type AdapterResult,
} from "../../../canvas/webgpu-adapter";
import { useReducedMotion } from "../../../hooks/use-reduced-motion";

interface MetricsChartProps {
  rows: readonly MetricRow[];
  windowSeconds: number;
  bucketCount?: number;
}

const DEFAULT_BUCKET_COUNT = 30;

export function MetricsChart({
  rows,
  windowSeconds,
  bucketCount = DEFAULT_BUCKET_COUNT,
}: MetricsChartProps) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [adapter, setAdapter] = useState<AdapterResult | null>(null);
  const reducedMotion = useReducedMotion() ?? false;

  const buckets = useMemo(
    () => aggregateIntoBuckets(rows, windowSeconds, bucketCount),
    [rows, windowSeconds, bucketCount],
  );
  const bucketsRef = useRef(buckets);
  useEffect(() => {
    bucketsRef.current = buckets;
  }, [buckets]);

  useEffect(() => {
    let cancelled = false;
    void requestWebGPUAdapter().then((result) => {
      if (!cancelled) {
        setAdapter(result);
      }
    });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (adapter === null || adapter.kind !== "available") {
      return;
    }
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }
    const context = canvas.getContext("2d");
    if (context === null) {
      return;
    }
    const { backendKind } = adapter;
    const wgpuBackend = normalizeWgpuBackend(backendKind);
    const webviewBackend = detectWebviewBackend();

    const renderChart = (): void => {
      const list = bucketsRef.current;
      const cssWidth = canvas.clientWidth || canvas.width;
      const cssHeight = canvas.clientHeight || canvas.height;
      if (canvas.width !== cssWidth || canvas.height !== cssHeight) {
        canvas.width = cssWidth;
        canvas.height = cssHeight;
      }
      const w = canvas.width;
      const h = canvas.height;
      context.clearRect(0, 0, w, h);
      if (list.length === 0) {
        return;
      }
      const max = Math.max(1, ...list.map((b) => b.aggregateValue));
      const primary = readDesignToken("--color-primary", "#4A90E2");
      context.strokeStyle = primary;
      context.lineWidth = 1.5;
      context.beginPath();
      list.forEach((b, i) => {
        const x = (i / Math.max(1, list.length - 1)) * w;
        const y = h - (b.aggregateValue / max) * (h - 8) - 4;
        if (i === 0) {
          context.moveTo(x, y);
        } else {
          context.lineTo(x, y);
        }
      });
      context.stroke();
    };

    const loop = createFrameLoop({
      onFrame: () => {
        const start = performance.now();
        renderChart();
        const duration_ms = performance.now() - start;
        void recordFrameMs({
          duration_ms,
          wgpu_backend: wgpuBackend,
          webview_backend: webviewBackend,
          timing_method: "cpu",
        });
      },
      prefersReducedMotion: reducedMotion,
      onReducedMotionFrame: () => {
        renderChart();
        void recordFrameMs({
          duration_ms: 0,
          wgpu_backend: wgpuBackend,
          webview_backend: webviewBackend,
          timing_method: "cpu",
        });
      },
    });
    loop.start();
    return () => {
      loop.stop();
    };
  }, [adapter, reducedMotion]);

  const showFallback = adapter !== null && adapter.kind === "unavailable";

  return (
    <section
      aria-label="Metrics time-series chart"
      data-testid="metrics-chart"
      data-bucket-count={buckets.length}
      style={{
        background: "var(--color-inset)",
        border: "1px solid rgba(74, 144, 226, 0.3)",
        borderRadius: "var(--radius-md)",
        padding: "var(--spacing-md)",
        position: "relative",
        width: "100%",
        height: "240px",
        boxSizing: "border-box",
      }}
    >
      {showFallback ? (
        <Fallback />
      ) : (
        <canvas
          ref={canvasRef}
          data-testid="metrics-chart-canvas"
          style={{
            position: "relative",
            width: "100%",
            height: "100%",
            display: "block",
          }}
        />
      )}
    </section>
  );
}

interface Bucket {
  index: number;
  aggregateValue: number;
}

export function aggregateIntoBuckets(
  rows: readonly MetricRow[],
  windowSeconds: number,
  bucketCount: number,
): Bucket[] {
  if (bucketCount <= 0 || rows.length === 0) {
    return [];
  }
  const safeBuckets = Math.max(1, bucketCount);
  const tsValues = rows.map((r) => r.ts_unix_nano);
  const maxTs = Math.max(...tsValues);
  const safeWindow = Math.max(1, windowSeconds);
  const minTs = maxTs - safeWindow * 1_000_000_000;
  const span = Math.max(1, maxTs - minTs);
  const out: Bucket[] = Array.from({ length: safeBuckets }, (_, index) => ({
    index,
    aggregateValue: 0,
  }));
  for (const row of rows) {
    const offset = row.ts_unix_nano - minTs;
    const idx = Math.min(
      safeBuckets - 1,
      Math.max(0, Math.floor((offset / span) * safeBuckets)),
    );
    out[idx].aggregateValue += row.value;
  }
  return out;
}

function readDesignToken(name: string, fallback: string): string {
  if (typeof document === "undefined") return fallback;
  const value = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  return value || fallback;
}
