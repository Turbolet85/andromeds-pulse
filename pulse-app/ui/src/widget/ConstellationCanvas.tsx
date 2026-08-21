// Compact-widget service constellation (chunk #91) — replaces the chunk #32
// AggregatedBadgeCanvas. One soft Halo dot per service from the registry:
// hue = per-service incident severity (LCH Earth Blue → Alert Burgundy),
// brightness = lifecycle activity tier, position = stable hash(service_name)
// scatter. Only currently-live services (recent last_seen) shown; stale/
// archived hidden (P-067). The dots breathe (opacity-only
// envelope, never scale — P-026) when motion is allowed; reduced-motion
// renders a single static frame (hue + brightness still encode state).
//
// Mirrors halo/HaloCanvas.tsx + dashboard ConstellationCanvas: owns its own
// <canvas> + WebGPU pipeline outside React's render tree; the per-dot draw
// loop + frame-metrics bridge match the established constellation precedent.
//
// The <canvas> is opaque to screen readers (a11y-plan §1), so the wrapper
// <section> carries a complete-sentence accessible name summarizing the
// constellation (count + per-state breakdown + active-findings count) — the
// not-color-alone (SC 1.4.1) text equivalent for the color/brightness dots.

import { useEffect, useMemo, useRef, useState } from "react";
import { createFrameLoop } from "../canvas/frame-loop";
import { Fallback } from "../canvas/Fallback";
import {
  detectWebviewBackend,
  normalizeWgpuBackend,
  recordConstellationDiscoveryLatency,
  recordConstellationHueLatency,
  recordFrameMs,
  type HueSeverityTierKind,
} from "../canvas/frame-metrics";
import { requestWebGPUAdapter, type AdapterResult } from "../canvas/webgpu-adapter";
import { lchInterpolate } from "../halo/lch";
import { useReducedMotion } from "../hooks/use-reduced-motion";
import { createConstellationPipeline } from "./constellation-pipeline";
import {
  constellationSummary,
  visibleDots,
  type ConstellationDot,
} from "./constellation-types";
import type { PriorityTier, ServiceListItem } from "../bindings/index";

interface ConstellationCanvasProps {
  items: readonly ServiceListItem[];
}

const PREFERRED_FORMAT: GPUTextureFormat = "bgra8unorm";
// 16 (color) + 8 (center: vec2) + 4 (radius) + 4 (brightness) = 32 bytes,
// matching the DotUniforms struct in constellation.wgsl.
const UNIFORM_BUFFER_SIZE = 32;
// GPUBufferUsage.UNIFORM (0x40) | COPY_DST (0x08) inlined per jsdom test
// portability discipline (.claude/rules/testing.md 2026-05-09).
const UNIFORM_BUFFER_USAGE = 0x40 | 0x08;
// Soft-falloff dot radius in normalized [-1, 1] canvas units.
const DOT_RADIUS_NORM = 0.14;
// Gentle shared breathing period (ms); within the design-system quiet band.
const BREATHING_PERIOD_MS = 3500;

function readDesignToken(name: string, fallback: string): string {
  if (typeof document === "undefined") return fallback;
  const value = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  return value || fallback;
}

export function ConstellationCanvas({ items }: ConstellationCanvasProps) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [adapter, setAdapter] = useState<AdapterResult | null>(null);
  const [pipelineFailed, setPipelineFailed] = useState(false);
  const reducedMotion = useReducedMotion() ?? false;

  const nowUnixNano = Date.now() * 1_000_000;
  const dots = useMemo(() => visibleDots(items, nowUnixNano), [items, nowUnixNano]);
  const summary = useMemo(
    () => constellationSummary(items, nowUnixNano),
    [items, nowUnixNano],
  );
  const dotsRef = useRef<readonly ConstellationDot[]>(dots);
  const discoveredRef = useRef<Set<string>>(new Set());
  const hueTierRef = useRef<Map<string, PriorityTier | null>>(new Map());

  useEffect(() => {
    dotsRef.current = dots;
  }, [dots]);

  // P-027 discovery bound: a service's span arrival (`last_seen_unix_nano`,
  // already on the wire) → its dot's first appearance in the rendered set.
  // Emitted AGGREGATE-only — a count plus the slowest arrival in this batch —
  // because `service` is an OTLP resource attribute and may never be a label
  // (security-plan §Logging; obs-plan §5 cardinality).
  useEffect(() => {
    const fresh = dots.filter((dot) => !discoveredRef.current.has(dot.service));
    if (fresh.length === 0) {
      return;
    }
    const nowMs = Date.now();
    let slowestMs = 0;
    for (const dot of fresh) {
      discoveredRef.current.add(dot.service);
      const item = items.find((candidate) => candidate.service === dot.service);
      if (item === undefined) {
        continue;
      }
      const elapsedMs = nowMs - item.last_seen_unix_nano / 1_000_000;
      if (elapsedMs > slowestMs) {
        slowestMs = elapsedMs;
      }
    }
    void recordConstellationDiscoveryLatency({
      duration_ms: slowestMs,
      discovered_count: fresh.length,
    });
  }, [dots, items]);

  // P-025 hue bound: a service's span arrival → the severity-driven hue its dot
  // renders. This is the surface where `severityToHueFraction` actually runs
  // (`constellation-types.ts`); the Halo State Pulse canvas the design system
  // describes has no production render site, so timing it there would emit
  // nothing. Fires on a tier CHANGE, which is what "hue updated" means.
  useEffect(() => {
    const nowMs = Date.now();
    let slowestMs = 0;
    let slowestTier: HueSeverityTierKind = "none";
    let changed = 0;

    for (const dot of dots) {
      const tier = dot.priorityTier ?? null;
      const known = hueTierRef.current.has(dot.service);
      const previous = hueTierRef.current.get(dot.service) ?? null;
      hueTierRef.current.set(dot.service, tier);

      // A first sighting at the calm baseline is not a hue update — timing it
      // would report every discovery as a severity change.
      if (!known && tier === null) {
        continue;
      }
      if (known && previous === tier) {
        continue;
      }
      changed += 1;
      const item = items.find((candidate) => candidate.service === dot.service);
      if (item === undefined) {
        continue;
      }
      const elapsedMs = nowMs - item.last_seen_unix_nano / 1_000_000;
      if (elapsedMs > slowestMs) {
        slowestMs = elapsedMs;
        slowestTier = tier ?? "none";
      }
    }

    if (changed === 0) {
      return;
    }
    void recordConstellationHueLatency({
      duration_ms: slowestMs,
      severity_tier: slowestTier,
    });
  }, [dots, items]);

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

    const pipelineResult = createConstellationPipeline(device, PREFERRED_FORMAT);
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

    const writeUniformsForDot = (dot: ConstellationDot, envelope: number): void => {
      const color = lchInterpolate(dot.hueFraction, primaryHex, accentHex);
      const effectiveBrightness = dot.brightness * envelope;
      const data = new Float32Array([
        color.r,
        color.g,
        color.b,
        color.a,
        dot.x,
        dot.y,
        DOT_RADIUS_NORM,
        effectiveBrightness,
      ]);
      device.queue.writeBuffer(uniformBuffer, 0, data.buffer);
    };

    const clearFrame = (): void => {
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
    };

    const renderAllDots = (envelope: number): void => {
      const list = dotsRef.current;
      if (list.length === 0) {
        clearFrame();
        return;
      }
      for (let i = 0; i < list.length; i += 1) {
        writeUniformsForDot(list[i], envelope);
        const encoder = device.createCommandEncoder({ label: "constellation-encoder" });
        const pass = encoder.beginRenderPass({
          colorAttachments: [
            {
              view: renderContext.getCurrentTexture().createView(),
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
        const phase = ((timestamp % BREATHING_PERIOD_MS) / BREATHING_PERIOD_MS) * 2 * Math.PI;
        const envelope = 0.6 + 0.4 * (0.5 + 0.5 * Math.sin(phase));
        renderAllDots(envelope);
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
        // Static glow: full envelope, no rhythm; hue + brightness still encode state.
        renderAllDots(1);
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
      aria-label={summary}
      data-testid="service-constellation"
      data-service-count={dots.length}
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
