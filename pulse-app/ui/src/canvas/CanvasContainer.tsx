// Canvas wrapper component for chunk #28. Establishes the substrate consumed
// by Epoch 5 surfaces: semantic <region> wrapper per a11y plan §7 Landmark
// roles, design-token chrome per layout-templates.md §Component — Halo State
// Pulse canvas Layout, reduced-motion gate via useReducedMotion + chunk #15
// createFrameLoop, adapter-driven branch between <canvas> + <Fallback />.
//
// Substrate-only: no per-frame data binding (chunks #34/#35), no per-surface
// dimensions / aspect ratios (chunks #30/#33/#36), no TauRPC frame metric
// emission (chunk #29 wires telemetry.frontend.record_frame_ms). The
// `mirrorTable` slot is reserved for downstream chunks к inject а
// <table> DOM mirror per a11y plan §1 critical path P1 (chart text
// equivalents for screen-reader users).

import { useEffect, useRef, useState, type ReactNode } from "react";
import { createFrameLoop } from "./frame-loop";
import { Fallback } from "./Fallback";
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
    const { device } = adapter;
    context.configure({ device, format: PREFERRED_FORMAT, alphaMode: "premultiplied" });

    // Substrate: instantiate three render pipelines so chunks #34/#35 can
    // bind data without re-architecting init. Per arch §Standard Contracts
    // alignment с viz query routers (`traces.*` / `metrics.*` / `logs.*`).
    createTraceTimelinePipeline(device, PREFERRED_FORMAT);
    createFlamegraphPipeline(device, PREFERRED_FORMAT);
    createMetricsChartPipeline(device, PREFERRED_FORMAT);

    const loop = createFrameLoop({
      onFrame: () => {
        // Frame-timing measurement point (chunk #29 wires the actual TauRPC
        // telemetry.frontend.record_frame_ms call per the existing TODO в
        // frame-loop.ts). Per obs plan §11 Logs anti-pattern: NEVER log per
        // frame at info level — chunk #29 emits the metric event.
      },
      prefersReducedMotion: reducedMotion,
      onReducedMotionFrame: () => {
        // Static-paint branch: chunk #28 substrate only confirms the gate
        // fires; data-driven hue updates land с the consuming chunks.
      },
    });
    loop.start();
    return () => {
      loop.stop();
    };
  }, [adapter, reducedMotion]);

  return (
    <section
      role="region"
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
      {adapter !== null && adapter.kind === "unavailable" ? (
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
