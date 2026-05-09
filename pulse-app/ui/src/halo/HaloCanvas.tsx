// Halo State Pulse signature element. Owns its own <canvas> + WebGPU pipeline
// (separate from canvas/CanvasContainer.tsx substrate canvas — Halo lives
// "outside React's render tree" per design-system §Surface: desktop-webview
// Performance notes). Wraps in <region role="region" aria-label="..."> per
// a11y-plan §1 desktop-webview row + §7 Landmark roles.
//
// Data inputs (throughputHz + errorRate) flow as React props from the parent;
// chunk #31 ships with synthetic input from App.tsx (sine-wave simulator)
// while real `streams.subscribe_metrics` Arrow IPC binding is deferred to
// chunks #34/#35. Refs hold latest prop values so the long-lived rAF loop
// reads fresh data without tearing down the pipeline on every prop update.
//
// Reduced-motion behavior: the rAF loop is suppressed (static glow) but hue
// continues to update on errorRate prop change via a secondary effect that
// re-invokes one static-glow render per change. Per design-system §Motion
// Accessibility + canvas/types.ts comment ("consumers may re-call start() to
// repaint after data updates").

import { useEffect, useRef, useState } from "react";
import { useReducedMotion } from "../hooks/use-reduced-motion";
import { createFrameLoop } from "../canvas/frame-loop";
import { Fallback } from "../canvas/Fallback";
import {
  detectWebviewBackend,
  normalizeWgpuBackend,
  recordFrameMs,
} from "../canvas/frame-metrics";
import { requestWebGPUAdapter, type AdapterResult } from "../canvas/webgpu-adapter";
import { createHaloPipeline } from "./halo-pipeline";
import { throughputToHz } from "./throughput-to-hz";
import { errorRateToBlur } from "./error-rate-to-blur";
import { lchInterpolate } from "./lch";

interface HaloCanvasProps {
  ariaLabel: string;
  throughputHz: number;
  errorRate: number;
}

const PREFERRED_FORMAT: GPUTextureFormat = "bgra8unorm";
// 16 bytes (color: vec4<f32>) + 4 bytes (blur_target: f32) + 4 bytes
// (pulse_phase: f32) + 8 bytes (_pad0/_pad1: f32) = 32 bytes total, matching
// the HaloUniforms struct in halo.wgsl with WGSL std140-equivalent alignment.
const UNIFORM_BUFFER_SIZE = 32;
// Inlined per WebGPU spec (GPUBufferUsage.UNIFORM = 0x40, COPY_DST = 0x08).
// jsdom test environment does not expose the runtime GPUBufferUsage global
// (only @webgpu/types compile-time interface); inlining keeps tests portable
// without per-test stubbing.
const UNIFORM_BUFFER_USAGE = 0x40 | 0x08;

function readDesignToken(name: string, fallback: string): string {
  if (typeof document === "undefined") return fallback;
  const value = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  return value || fallback;
}

export function HaloCanvas({ ariaLabel, throughputHz, errorRate }: HaloCanvasProps) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [adapter, setAdapter] = useState<AdapterResult | null>(null);
  const [haloPipelineFailed, setHaloPipelineFailed] = useState(false);
  const reducedMotion = useReducedMotion() ?? false;

  const throughputHzRef = useRef(throughputHz);
  const errorRateRef = useRef(errorRate);
  const restartLoopRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    throughputHzRef.current = throughputHz;
    errorRateRef.current = errorRate;
  }, [throughputHz, errorRate]);

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
      setHaloPipelineFailed(true);
      return;
    }
    setHaloPipelineFailed(false);
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

    const writeUniforms = (pulsePhase: number): void => {
      const currentErrorRate = errorRateRef.current;
      const color = lchInterpolate(currentErrorRate, primaryHex, accentHex);
      const blurTarget = errorRateToBlur(currentErrorRate);
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

    const encodeAndSubmit = (): void => {
      const encoder = device.createCommandEncoder({ label: "halo-encoder" });
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
      pass.setPipeline(pipeline);
      pass.setBindGroup(0, bindGroup);
      pass.draw(6);
      pass.end();
      device.queue.submit([encoder.finish()]);
    };

    const loop = createFrameLoop({
      onFrame: (timestamp) => {
        const start = performance.now();
        const hz = throughputToHz(throughputHzRef.current);
        const phase = ((timestamp / 1000) * 2 * Math.PI * hz) % (2 * Math.PI);
        writeUniforms(phase);
        encodeAndSubmit();
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
        writeUniforms(0);
        encodeAndSubmit();
        void recordFrameMs({
          duration_ms: 0,
          wgpu_backend: wgpuBackend,
          webview_backend: webviewBackend,
          timing_method: "cpu",
        });
      },
    });
    loop.start();

    restartLoopRef.current = () => {
      loop.stop();
      loop.start();
    };

    return () => {
      loop.stop();
      restartLoopRef.current = null;
    };
  }, [adapter, reducedMotion]);

  useEffect(() => {
    if (reducedMotion && restartLoopRef.current !== null) {
      restartLoopRef.current();
    }
  }, [throughputHz, errorRate, reducedMotion]);

  const showFallback =
    (adapter !== null && adapter.kind === "unavailable") || haloPipelineFailed;

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
    </section>
  );
}
