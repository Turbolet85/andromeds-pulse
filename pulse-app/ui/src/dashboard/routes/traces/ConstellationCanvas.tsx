import { useEffect, useRef, useState } from "react";
import { createFrameLoop } from "../../../canvas/frame-loop";
import { Fallback } from "../../../canvas/Fallback";
import {
  detectWebviewBackend,
  normalizeWgpuBackend,
  recordFrameMs,
} from "../../../canvas/frame-metrics";
import {
  requestWebGPUAdapter,
  type AdapterResult,
} from "../../../canvas/webgpu-adapter";
import { createHaloPipeline } from "../../../halo/halo-pipeline";
import { lchInterpolate } from "../../../halo/lch";
import { errorRateToBlur } from "../../../halo/error-rate-to-blur";
import { throughputToHz } from "../../../halo/throughput-to-hz";
import { useReducedMotion } from "../../../hooks/use-reduced-motion";
import type { ServiceAggregate } from "./use-constellation-data";

interface ConstellationCanvasProps {
  services: readonly ServiceAggregate[];
}

const PREFERRED_FORMAT: GPUTextureFormat = "bgra8unorm";
// Same uniform layout as halo/HaloCanvas: 16 (color) + 4 (blur) + 4 (phase)
// + 8 (padding) = 32 bytes. Per-service dots reuse the same struct via
// per-service draw calls + per-call uniform writes.
const UNIFORM_BUFFER_SIZE = 32;
// GPUBufferUsage.UNIFORM (0x40) | GPUBufferUsage.COPY_DST (0x08) inlined per
// jsdom test portability discipline (.claude/rules/testing.md 2026-05-09).
const UNIFORM_BUFFER_USAGE = 0x40 | 0x08;

function readDesignToken(name: string, fallback: string): string {
  if (typeof document === "undefined") return fallback;
  const value = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  return value || fallback;
}

export function ConstellationCanvas({ services }: ConstellationCanvasProps) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [adapter, setAdapter] = useState<AdapterResult | null>(null);
  const [pipelineFailed, setPipelineFailed] = useState(false);
  const reducedMotion = useReducedMotion() ?? false;
  const servicesRef = useRef<readonly ServiceAggregate[]>(services);

  useEffect(() => {
    servicesRef.current = services;
  }, [services]);

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

    const pipelineResult = createHaloPipeline(device, PREFERRED_FORMAT);
    if (pipelineResult.kind === "failed") {
      setPipelineFailed(true);
      return;
    }
    setPipelineFailed(false);
    const pipeline = pipelineResult.pipeline;

    const primaryHex = readDesignToken("--color-primary", "#4A90E2");
    const accentHex = readDesignToken("--color-accent", "#C7556A");

    const uniformBuffer = device.createBuffer({
      size: UNIFORM_BUFFER_SIZE,
      usage: UNIFORM_BUFFER_USAGE,
    });
    const bindGroup = device.createBindGroup({
      layout: pipeline.getBindGroupLayout(0),
      entries: [{ binding: 0, resource: { buffer: uniformBuffer } }],
    });

    const wgpuBackend = normalizeWgpuBackend(backendKind);
    const webviewBackend = detectWebviewBackend();
    const renderContext = context;

    const writeUniformsForService = (service: ServiceAggregate, pulsePhase: number): void => {
      const color = lchInterpolate(service.errorRate, primaryHex, accentHex);
      const blurTarget = errorRateToBlur(service.errorRate);
      const data = new Float32Array([
        color.r,
        color.g,
        color.b,
        color.a,
        blurTarget,
        pulsePhase,
        0,
        0,
      ]);
      device.queue.writeBuffer(uniformBuffer, 0, data.buffer);
    };

    const renderAllServices = (timestamp: DOMHighResTimeStamp): void => {
      const list = servicesRef.current;
      if (list.length === 0) {
        // Render a single clear frame so the canvas isn't a stale paint.
        const encoder = device.createCommandEncoder({ label: "constellation-clear" });
        const pass = encoder.beginRenderPass({
          colorAttachments: [
            {
              view: renderContext.getCurrentTexture().createView(),
              loadOp: "clear",
              storeOp: "store",
              clearValue: { r: 0, g: 0, b: 0, a: 0 },
            },
          ],
        });
        pass.end();
        device.queue.submit([encoder.finish()]);
        return;
      }
      for (let i = 0; i < list.length; i += 1) {
        const service = list[i];
        const hz = throughputToHz(service.throughputHz);
        const phase = ((timestamp / 1000) * 2 * Math.PI * hz) % (2 * Math.PI);
        writeUniformsForService(service, phase);
        const encoder = device.createCommandEncoder({ label: "constellation-encoder" });
        const pass = encoder.beginRenderPass({
          colorAttachments: [
            {
              view: renderContext.getCurrentTexture().createView(),
              // First service clears; subsequent services draw on top.
              loadOp: i === 0 ? "clear" : "load",
              storeOp: "store",
              clearValue: { r: 0, g: 0, b: 0, a: 0 },
            },
          ],
        });
        pass.setPipeline(pipeline);
        pass.setBindGroup(0, bindGroup);
        pass.draw(6);
        pass.end();
        device.queue.submit([encoder.finish()]);
      }
    };

    const loop = createFrameLoop({
      onFrame: (timestamp) => {
        const start = performance.now();
        renderAllServices(timestamp);
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
        renderAllServices(0);
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
    (adapter !== null && adapter.kind === "unavailable") || pipelineFailed;

  return (
    <section
      aria-label="Service constellation"
      data-testid="constellation-canvas"
      data-service-count={services.length}
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
