// Canvas wrapper component for chunk #28 + chunk #29. Establishes the
// substrate consumed by Epoch 5 surfaces: semantic <region> wrapper per a11y
// plan §7 Landmark roles, design-token chrome per layout-templates.md
// §Component — Halo State Pulse canvas Layout, reduced-motion gate via
// useReducedMotion + chunk #15 createFrameLoop, adapter-driven branch between
// <canvas> + <Fallback />.
//
// Chunk #29 extension: compute-pipeline init alongside the 3 render pipelines
// + per-frame timing capture invoking the TauRPC
// `telemetry.frontend.record_frame_ms` resolver. Compute pipeline failure
// surfaces <Fallback /> just like adapter unavailability — both block the
// canvas from rendering and direct the user to the substrate-not-supported
// branch.
//
// Substrate-only: no per-frame data binding (chunks #34/#35), no per-surface
// dimensions / aspect ratios (chunks #30/#33/#36). The `mirrorTable` slot is
// reserved for downstream chunks to inject a <table> DOM mirror per a11y plan
// §1 critical path P1 (chart text equivalents for screen-reader users).

import { useEffect, useRef, useState, type ReactNode } from "react";
import { createFrameLoop } from "./frame-loop";
import { Fallback } from "./Fallback";
import { createAggregationComputePipeline } from "./compute-pipeline";
import {
  detectWebviewBackend,
  normalizeWgpuBackend,
  recordFrameMs,
} from "./frame-metrics";
import {
  createFlamegraphPipeline,
  createMetricsChartPipeline,
  createTraceTimelinePipeline,
} from "./render-pipeline";
import { requestWebGPUAdapter, type AdapterResult } from "./webgpu-adapter";
import { useReducedMotion } from "../hooks/use-reduced-motion";

interface CanvasContainerProps {
  ariaLabel: string;
  mirrorTable?: ReactNode;
}

const PREFERRED_FORMAT: GPUTextureFormat = "bgra8unorm";

export function CanvasContainer({ ariaLabel, mirrorTable }: CanvasContainerProps) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [adapter, setAdapter] = useState<AdapterResult | null>(null);
  const [computePipelineFailed, setComputePipelineFailed] = useState(false);
  const reducedMotion = useReducedMotion() ?? false;

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
    const context = canvas.getContext("webgpu");
    if (context === null) {
      return;
    }
    const { device, backendKind } = adapter;
    context.configure({ device, format: PREFERRED_FORMAT, alphaMode: "premultiplied" });

    // Substrate: instantiate three render pipelines so chunks #34/#35 can
    // bind data without re-architecting init. Per arch §Standard Contracts
    // alignment with viz query routers (`traces.*` / `metrics.*` / `logs.*`).
    createTraceTimelinePipeline(device, PREFERRED_FORMAT);
    createFlamegraphPipeline(device, PREFERRED_FORMAT);
    createMetricsChartPipeline(device, PREFERRED_FORMAT);

    const computeResult = createAggregationComputePipeline(device);
    if (computeResult.kind === "failed") {
      setComputePipelineFailed(true);
      return;
    }
    setComputePipelineFailed(false);

    const wgpuBackend = normalizeWgpuBackend(backendKind);
    const webviewBackend = detectWebviewBackend();

    const loop = createFrameLoop({
      onFrame: () => {
        const start = performance.now();
        void device.queue.onSubmittedWorkDone().then(() => {
          const duration_ms = performance.now() - start;
          void recordFrameMs({
            duration_ms,
            wgpu_backend: wgpuBackend,
            webview_backend: webviewBackend,
            timing_method: "cpu",
          });
        });
      },
      prefersReducedMotion: reducedMotion,
      onReducedMotionFrame: () => {
        // Static-paint path emits ONE metric event so SLO aggregation continues
        // over a sparser stream (per obs cross-domain binding "stable shape
        // across motion modes"). Same 4-field shape as the rAF path.
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

  const showFallback =
    (adapter !== null && adapter.kind === "unavailable") || computePipelineFailed;

  return (
    <section
      aria-label={ariaLabel}
      style={{
        background: "var(--color-inset)",
        border: "1px solid rgba(74, 144, 226, 0.3)",
        borderRadius: "var(--radius-md)",
        padding: "var(--spacing-md)",
        position: "relative",
        width: "100%",
        height: "100%",
        boxSizing: "border-box",
      }}
    >
      {showFallback ? (
        <Fallback />
      ) : (
        <canvas
          ref={canvasRef}
          style={{
            position: "relative",
            width: "100%",
            height: "100%",
            display: "block",
          }}
        />
      )}
      {mirrorTable}
    </section>
  );
}
