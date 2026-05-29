// Halo State Pulse signature element. Owns its own <canvas> + WebGPU pipeline
// (separate from canvas/CanvasContainer.tsx substrate canvas — Halo lives
// "outside React's render tree" per design-system §Surface: desktop-webview
// Performance notes). Wraps in <region role="region" aria-label="..."> per
// a11y-plan §1 desktop-webview row + §7 Landmark roles.
//
// Data inputs (connectionState + cumulativeSeverity + activityState) flow as
// React props from the parent (chunk #90; sourced from use-connection-state +
// use-findings + the throughput→activity derivation). Refs hold latest prop
// values so the long-lived rAF loop reads fresh data without tearing down the
// pipeline on every prop update.
//
// Three orthogonal axes (per design-system §Motion Decisions Log 2026-05-29):
// cumulative incident severity drives hue (LCH) + blur; activity drives the
// breathing pace (4-5 s quiet → ~2 s active); connection state grays out the
// halo independently (P-004). Breathing is opacity + blur only, never scale.
//
// Reduced-motion behavior: the rAF loop is suppressed (static glow) but hue +
// blur + grayout continue to update on prop change via a secondary effect that
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
import { lchInterpolate } from "./lch";
import {
  activityStateToBreathingPeriodMs,
  connectionStateToDim,
  severityToBlurPx,
  severityToHueFraction,
} from "./severity-to-halo";
import type { ActivityState } from "./halo-types";
import type { ConnectionState, PriorityTier } from "../bindings/index";

interface HaloCanvasProps {
  ariaLabel: string;
  connectionState: ConnectionState;
  cumulativeSeverity: PriorityTier | null;
  activityState: ActivityState;
}

const PREFERRED_FORMAT: GPUTextureFormat = "bgra8unorm";
// 16 bytes (color: vec4<f32>) + 4 bytes (blur_target: f32) + 4 bytes
// (pulse_phase: f32) + 4 bytes (connection_dim: f32) + 4 bytes (_pad0: f32)
// = 32 bytes total, matching the HaloUniforms struct in halo.wgsl with WGSL
// std140-equivalent alignment.
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

export function HaloCanvas({
  ariaLabel,
  connectionState,
  cumulativeSeverity,
  activityState,
}: HaloCanvasProps) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [adapter, setAdapter] = useState<AdapterResult | null>(null);
  const [haloPipelineFailed, setHaloPipelineFailed] = useState(false);
  const reducedMotion = useReducedMotion() ?? false;

  const connectionStateRef = useRef(connectionState);
  const cumulativeSeverityRef = useRef(cumulativeSeverity);
  const activityStateRef = useRef(activityState);
  const restartLoopRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    connectionStateRef.current = connectionState;
    cumulativeSeverityRef.current = cumulativeSeverity;
    activityStateRef.current = activityState;
  }, [connectionState, cumulativeSeverity, activityState]);

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
      const severity = cumulativeSeverityRef.current;
      const color = lchInterpolate(severityToHueFraction(severity), primaryHex, accentHex);
      const blurTarget = severityToBlurPx(severity);
      const connectionDim = connectionStateToDim(connectionStateRef.current);
      const data = new Float32Array([
        color.r,
        color.g,
        color.b,
        color.a,
        blurTarget,
        pulsePhase,
        connectionDim,
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
        const periodMs = activityStateToBreathingPeriodMs(activityStateRef.current);
        const hz = 1000 / periodMs;
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
  }, [connectionState, cumulativeSeverity, activityState, reducedMotion]);

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
