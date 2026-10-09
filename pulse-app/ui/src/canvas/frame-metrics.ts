// Frame-duration metric bridge for chunk #29 — invokes the TauRPC
// `telemetry.frontend.record_frame_ms` resolver after a webview frame
// completes. Per obs-plan §11 Frontend bridge: webview measures via
// `performance.now()` + `device.queue.onSubmittedWorkDone()`, hands the
// duration to the backend via TauRPC, backend emits `tracing::info!(target:
// "metric.webgpu.frame_duration_ms", ...)` against the chunk #28 pre-staged
// AllowList entry.
//
// Defense-in-depth: pre-clamp the duration client-side so an out-of-range
// invocation returns AppError::Validation cleanly without crashing the frame
// loop. The resolver also validates — clamping here just keeps the warn log
// out of the happy path while the RAF loop is healthy.

import { createTauRPCProxy } from "../bindings";

export type WgpuBackendKind = "vulkan" | "metal" | "dx12";
export type WebviewBackendKind = "webview2" | "wkwebview" | "gtkwebkit";
export type TimingMethodKind = "cpu" | "gpu";

export interface FrameMetricInput {
  duration_ms: number;
  wgpu_backend: WgpuBackendKind;
  webview_backend: WebviewBackendKind;
  timing_method: TimingMethodKind;
}

const DURATION_MS_MIN = 0;
const DURATION_MS_MAX = 60_000;

export function clampDurationMs(value: number): number {
  if (!Number.isFinite(value)) {
    return 0;
  }
  if (value < DURATION_MS_MIN) {
    return DURATION_MS_MIN;
  }
  if (value > DURATION_MS_MAX) {
    return DURATION_MS_MAX;
  }
  return value;
}

// Map the four enum strings shared on the WebGPUBackendKind tagged-union from
// `webgpu-adapter.ts` into the bounded enum here. "unknown" never reaches the
// backend (the smart enum on the Rust side rejects any non-allowlist value);
// we surface "vulkan" as a defensive fallback so the resolver still records
// the frame rather than silently dropping it. The fallback is documented in
// the obs-plan §11 cardinality discipline as the trade-off vs adding a 4th
// "unknown" enum value (which would explode label cardinality forever).
export function normalizeWgpuBackend(input: string): WgpuBackendKind {
  if (input === "metal" || input === "dx12") {
    return input;
  }
  return "vulkan";
}

export function detectWebviewBackend(): WebviewBackendKind {
  // Per arch §Stack Visualization surface row: WebView2 (Windows) /
  // WKWebView (macOS) / GTKWebKit (Linux). Discriminator order matters —
  // Linux WebKit UAs contain BOTH "X11" and "WebKit" / "AppleWebKit", so the
  // Linux branch must precede the generic WebKit branch to avoid Linux being
  // misclassified as macOS.
  if (typeof navigator === "undefined") {
    return "webview2";
  }
  const ua = navigator.userAgent ?? "";
  if (ua.includes("Edg/")) {
    return "webview2";
  }
  if (ua.includes("X11") || ua.includes("Linux")) {
    return "gtkwebkit";
  }
  if (ua.includes("WebKit") && !ua.includes("Chrome")) {
    return "wkwebview";
  }
  return "webview2";
}

let cachedClient: ReturnType<typeof createTauRPCProxy> | null = null;

function getClient(): ReturnType<typeof createTauRPCProxy> {
  if (cachedClient === null) {
    cachedClient = createTauRPCProxy();
  }
  return cachedClient;
}

export async function recordFrameMs(input: FrameMetricInput): Promise<void> {
  const sanitized: FrameMetricInput = {
    ...input,
    duration_ms: clampDurationMs(input.duration_ms),
  };
  try {
    const client = getClient();
    // The dotted namespace `telemetry.frontend.record_frame_ms` resolves via
    // the TauRPC proxy's nested router shape. Specta emits the type alongside
    // the merged Router type so the call site is typechecked at build time.
    const proxyRoot = client as unknown as {
      telemetry?: {
        frontend?: {
          record_frame_ms?: (input: FrameMetricInput) => Promise<void>;
        };
      };
    };
    const resolver = proxyRoot.telemetry?.frontend?.record_frame_ms;
    if (typeof resolver !== "function") {
      // Bindings out-of-date or feature-disabled — skip emission; downstream
      // observers tail agent-latest.jsonl and notice the absence rather than
      // see noisy console errors.
      return;
    }
    await resolver(sanitized);
  } catch {
    // Do NOT throw — frame loop continues on next frame regardless of
    // emission success. The obs gate fails CI if emissions are absent under
    // load, so the failure stays visible without crashing the UI.
  }
}

// Test-only seam — lets unit tests inject a stub client without touching
// global Tauri state.
export function __setProxyForTest(proxy: ReturnType<typeof createTauRPCProxy> | null): void {
  cachedClient = proxy;
}

// Delegated timing observables (P-025 / P-027 / P-045). Each bound ends at a
// paint inside this webview, so the backend cannot observe it; obs-plan §1/§4
// makes `telemetry.frontend.*` the only sanctioned route to the log. Same
// posture as recordFrameMs: clamp client-side, never throw into a render path.

export type HueSeverityTierKind = "none" | "curious" | "suggested" | "autonomous";

export interface ConstellationHueLatencyInputJs {
  duration_ms: number;
  severity_tier: HueSeverityTierKind;
}

export interface ConstellationDiscoveryInputJs {
  duration_ms: number;
  discovered_count: number;
}

export interface FindingsCounterRefreshInputJs {
  duration_ms: number;
}

const DISCOVERED_COUNT_MAX = 10_000;

export function clampDiscoveredCount(value: number): number {
  if (!Number.isFinite(value) || value < 0) {
    return 0;
  }
  return Math.min(Math.floor(value), DISCOVERED_COUNT_MAX);
}

async function invokeTelemetry<T>(method: string, payload: T): Promise<void> {
  try {
    const proxyRoot = getClient() as unknown as {
      telemetry?: {
        frontend?: Record<string, ((input: T) => Promise<void>) | undefined>;
      };
    };
    const resolver = proxyRoot.telemetry?.frontend?.[method];
    if (typeof resolver !== "function") {
      return;
    }
    await resolver(payload);
  } catch {
    // Never throw into a render/poll path — an absent emission surfaces to the
    // obs gate as a missing metric stream, not as a broken surface.
  }
}

export async function recordConstellationHueLatency(input: ConstellationHueLatencyInputJs): Promise<void> {
  await invokeTelemetry("record_constellation_hue_latency", {
    ...input,
    duration_ms: clampDurationMs(input.duration_ms),
  });
}

export async function recordConstellationDiscoveryLatency(
  input: ConstellationDiscoveryInputJs,
): Promise<void> {
  await invokeTelemetry("record_constellation_discovery_latency", {
    duration_ms: clampDurationMs(input.duration_ms),
    discovered_count: clampDiscoveredCount(input.discovered_count),
  });
}

export async function recordFindingsCounterRefresh(
  input: FindingsCounterRefreshInputJs,
): Promise<void> {
  await invokeTelemetry("record_findings_counter_refresh", {
    duration_ms: clampDurationMs(input.duration_ms),
  });
}
