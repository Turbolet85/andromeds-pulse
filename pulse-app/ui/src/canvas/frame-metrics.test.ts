import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  __setProxyForTest,
  clampDurationMs,
  detectWebviewBackend,
  normalizeWgpuBackend,
  recordFrameMs,
} from "./frame-metrics";

interface MockProxy {
  telemetry: {
    frontend: {
      record_frame_ms: ReturnType<typeof vi.fn>;
    };
  };
}

function makeStubProxy(): MockProxy {
  return {
    telemetry: {
      frontend: {
        record_frame_ms: vi.fn().mockResolvedValue(undefined),
      },
    },
  };
}

describe("clampDurationMs — defense-in-depth client-side validation", () => {
  it("returns the value unchanged when within [0, 60_000]", () => {
    expect(clampDurationMs(16.7)).toBe(16.7);
    expect(clampDurationMs(0)).toBe(0);
    expect(clampDurationMs(60_000)).toBe(60_000);
  });

  it("clamps negative values to 0 (matches backend `out of range` rejection threshold)", () => {
    expect(clampDurationMs(-5)).toBe(0);
  });

  it("clamps above-max values to 60_000", () => {
    expect(clampDurationMs(120_000)).toBe(60_000);
  });

  it("collapses NaN to 0 (avoid backend `non-finite` reject in the happy path)", () => {
    expect(clampDurationMs(Number.NaN)).toBe(0);
  });

  it("collapses Infinity to 0", () => {
    expect(clampDurationMs(Number.POSITIVE_INFINITY)).toBe(0);
    expect(clampDurationMs(Number.NEGATIVE_INFINITY)).toBe(0);
  });
});

describe("normalizeWgpuBackend — bounded to 3-enum allowlist", () => {
  it("passes through metal / dx12 unchanged", () => {
    expect(normalizeWgpuBackend("metal")).toBe("metal");
    expect(normalizeWgpuBackend("dx12")).toBe("dx12");
  });

  it("maps unknown / opengl / empty to vulkan (defensive fallback)", () => {
    expect(normalizeWgpuBackend("opengl")).toBe("vulkan");
    expect(normalizeWgpuBackend("unknown")).toBe("vulkan");
    expect(normalizeWgpuBackend("")).toBe("vulkan");
  });
});

describe("detectWebviewBackend — UA discrimination", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("returns webview2 when user-agent contains Edg/ marker", () => {
    vi.stubGlobal("navigator", { userAgent: "Mozilla/5.0 ... Edg/120.0.0.0" });
    expect(detectWebviewBackend()).toBe("webview2");
  });

  it("returns wkwebview for WebKit UA without Chrome", () => {
    vi.stubGlobal("navigator", {
      userAgent:
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    });
    expect(detectWebviewBackend()).toBe("wkwebview");
  });

  it("returns gtkwebkit on Linux/X11 UA", () => {
    vi.stubGlobal("navigator", {
      userAgent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/605.1.15",
    });
    expect(detectWebviewBackend()).toBe("gtkwebkit");
  });
});

describe("recordFrameMs — TauRPC bridge invocation", () => {
  let proxy: MockProxy;

  beforeEach(() => {
    proxy = makeStubProxy();
    __setProxyForTest(proxy as unknown as ReturnType<typeof import("../bindings").createTauRPCProxy>);
  });

  afterEach(() => {
    __setProxyForTest(null);
    vi.unstubAllGlobals();
  });

  it("invokes telemetry.frontend.record_frame_ms with the clamped, bounded fields", async () => {
    await recordFrameMs({
      duration_ms: 16.7,
      wgpu_backend: "vulkan",
      webview_backend: "webview2",
      timing_method: "cpu",
    });
    expect(proxy.telemetry.frontend.record_frame_ms).toHaveBeenCalledTimes(1);
    expect(proxy.telemetry.frontend.record_frame_ms).toHaveBeenCalledWith({
      duration_ms: 16.7,
      wgpu_backend: "vulkan",
      webview_backend: "webview2",
      timing_method: "cpu",
    });
  });

  it("clamps duration_ms before invocation (out-of-range pre-validates)", async () => {
    await recordFrameMs({
      duration_ms: 999_999,
      wgpu_backend: "metal",
      webview_backend: "wkwebview",
      timing_method: "cpu",
    });
    const arg = proxy.telemetry.frontend.record_frame_ms.mock.calls[0][0];
    expect(arg.duration_ms).toBe(60_000);
  });

  it("does NOT throw when the resolver rejects (frame loop must keep firing)", async () => {
    proxy.telemetry.frontend.record_frame_ms.mockRejectedValue(new Error("ipc closed"));
    await expect(
      recordFrameMs({
        duration_ms: 16.7,
        wgpu_backend: "vulkan",
        webview_backend: "webview2",
        timing_method: "cpu",
      }),
    ).resolves.toBeUndefined();
  });

  it("emits the same 4-field shape regardless of motion mode (obs cross-domain binding)", async () => {
    // Reduced-motion-active path: the resolver MUST receive the same shape so
    // p99 jq aggregation works against a sparser stream.
    await recordFrameMs({
      duration_ms: 32.0,
      wgpu_backend: "dx12",
      webview_backend: "webview2",
      timing_method: "gpu",
    });
    const arg = proxy.telemetry.frontend.record_frame_ms.mock.calls[0][0];
    expect(Object.keys(arg).sort()).toEqual([
      "duration_ms",
      "timing_method",
      "webview_backend",
      "wgpu_backend",
    ]);
  });
});
