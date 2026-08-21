import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { ConstellationCanvas } from "./ConstellationCanvas";
import type { ServiceLifecycleState, ServiceListItem } from "../bindings/index";

const adapterMock = vi.hoisted(() => ({
  requestWebGPUAdapter: vi.fn(),
}));

vi.mock("../canvas/webgpu-adapter", () => ({
  requestWebGPUAdapter: adapterMock.requestWebGPUAdapter,
}));

vi.mock("../canvas/frame-metrics", () => ({
  recordFrameMs: vi.fn().mockResolvedValue(undefined),
  recordConstellationDiscoveryLatency: vi.fn().mockResolvedValue(undefined),
  recordConstellationHueLatency: vi.fn().mockResolvedValue(undefined),
  normalizeWgpuBackend: () => "vulkan" as const,
  detectWebviewBackend: () => "webview2" as const,
}));

beforeEach(() => {
  adapterMock.requestWebGPUAdapter.mockReset();
  adapterMock.requestWebGPUAdapter.mockResolvedValue({
    kind: "unavailable",
    reason: "navigator.gpu undefined",
  });
});

afterEach(() => {
  vi.unstubAllGlobals();
  vi.clearAllMocks();
});

// The component reads Date.now() at render for the recency gate (P-067), so
// fixtures are dated relative to it: 20s ago is comfortably live (< 60s
// window, with slack for the test-file duration), 2min ago is stale.
const LIVE_OFFSET_NANOS = 20 * 1_000_000_000;
const STALE_OFFSET_NANOS = 120 * 1_000_000_000;

function nowNano(): number {
  return Date.now() * 1_000_000;
}

function item(
  service: string,
  state: ServiceLifecycleState,
  priorityTier: ServiceListItem["priority_tier"] = null,
  lastSeenUnixNano: number = nowNano() - LIVE_OFFSET_NANOS,
): ServiceListItem {
  return {
    service,
    state,
    last_seen_unix_nano: lastSeenUnixNano,
    manual_override: null,
    priority_tier: priorityTier,
  };
}

const ITEMS: ServiceListItem[] = [item("svc-a", "active", "autonomous"), item("svc-b", "quiet")];

describe("ConstellationCanvas", () => {
  it("renders a <section> region (implicit role) with the summary as accessible name", async () => {
    render(<ConstellationCanvas items={ITEMS} />);
    const wrapper = await screen.findByRole("region", {
      name: /Service constellation: 2 services/,
    });
    expect(wrapper.tagName).toBe("SECTION");
    expect(wrapper.hasAttribute("role")).toBe(false);
  });

  it("conveys per-state counts + active findings in the accessible name (not color-alone)", () => {
    render(<ConstellationCanvas items={ITEMS} />);
    const wrapper = screen.getByTestId("service-constellation");
    const label = wrapper.getAttribute("aria-label") ?? "";
    expect(label).toContain("1 active");
    expect(label).toContain("1 quiet");
    expect(label).toContain("1 with active findings");
  });

  it("exposes the visible-dot count via data-service-count", () => {
    render(<ConstellationCanvas items={ITEMS} />);
    expect(screen.getByTestId("service-constellation").getAttribute("data-service-count")).toBe("2");
  });

  it("hides Archived services from the dot count", () => {
    render(<ConstellationCanvas items={[item("svc-a", "active"), item("svc-z", "archived")]} />);
    expect(screen.getByTestId("service-constellation").getAttribute("data-service-count")).toBe("1");
  });

  it("shows no live services when every service is stale (zero live telemetry — P-067)", async () => {
    const stale = [
      item("svc-a", "active", "autonomous", nowNano() - STALE_OFFSET_NANOS),
      item("svc-b", "quiet", null, nowNano() - STALE_OFFSET_NANOS),
    ];
    render(<ConstellationCanvas items={stale} />);
    const wrapper = await screen.findByRole("region", { name: /no active services/ });
    expect(wrapper.getAttribute("data-service-count")).toBe("0");
  });

  it("ages out a service that goes quiet past the live window (now recomputed each render)", () => {
    vi.useFakeTimers({ toFake: ["Date"] });
    try {
      // 55s ago → live (within the 60s window).
      const items = [item("svc-a", "active", null, nowNano() - 55 * 1_000_000_000)];
      const { rerender } = render(<ConstellationCanvas items={items} />);
      const wrapper = screen.getByTestId("service-constellation");
      expect(wrapper.getAttribute("data-service-count")).toBe("1");

      // Advance 15s → the same service is now 70s stale (past the window). A
      // re-render must recompute Date.now() (not cache it at mount) to drop it.
      vi.setSystemTime(Date.now() + 15_000);
      rerender(<ConstellationCanvas items={items} />);

      expect(wrapper.getAttribute("data-service-count")).toBe("0");
      expect(wrapper.getAttribute("aria-label")).toContain("no active services");
    } finally {
      vi.useRealTimers();
    }
  });

  it("renders Fallback (no canvas) when the WebGPU adapter is unavailable", async () => {
    render(<ConstellationCanvas items={ITEMS} />);
    const wrapper = screen.getByTestId("service-constellation");
    await vi.waitFor(() => {
      expect(wrapper.querySelector("canvas")).toBeNull();
    });
  });

  it("requests the WebGPU adapter on mount", () => {
    render(<ConstellationCanvas items={ITEMS} />);
    expect(adapterMock.requestWebGPUAdapter).toHaveBeenCalled();
  });

  it("emits the delegated timing observables on a live render (never forward-inert)", async () => {
    const metrics = await import("../canvas/frame-metrics");
    render(<ConstellationCanvas items={ITEMS} />);

    // P-027: both services are newly discovered on this first render.
    await vi.waitFor(() => {
      expect(metrics.recordConstellationDiscoveryLatency).toHaveBeenCalled();
    });
    const discovery = vi.mocked(metrics.recordConstellationDiscoveryLatency).mock.calls[0][0];
    expect(discovery.discovered_count).toBe(2);
    expect(Number.isFinite(discovery.duration_ms)).toBe(true);

    // P-025: svc-a carries a non-null tier, so its dot hue is a real update.
    // svc-b is calm on first sighting and must NOT be counted as a hue change.
    await vi.waitFor(() => {
      expect(metrics.recordConstellationHueLatency).toHaveBeenCalled();
    });
    const hue = vi.mocked(metrics.recordConstellationHueLatency).mock.calls[0][0];
    expect(hue.severity_tier).toBe("autonomous");
    expect(Number.isFinite(hue.duration_ms)).toBe(true);
  });

  it("does not report a hue update when every live service is at the calm baseline", async () => {
    const metrics = await import("../canvas/frame-metrics");
    render(<ConstellationCanvas items={[item("svc-calm", "active")]} />);

    await vi.waitFor(() => {
      expect(metrics.recordConstellationDiscoveryLatency).toHaveBeenCalled();
    });
    // Discovery fired, so the effect definitely ran — which is what makes this
    // absence assertion mean something rather than pass vacuously.
    expect(metrics.recordConstellationHueLatency).not.toHaveBeenCalled();
  });

  it("handles empty items without crashing", async () => {
    render(<ConstellationCanvas items={[]} />);
    const wrapper = await screen.findByRole("region", { name: /no active services/ });
    expect(wrapper.getAttribute("data-service-count")).toBe("0");
  });
});
