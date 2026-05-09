import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { ConstellationCanvas } from "./ConstellationCanvas";
import type { ServiceAggregate } from "./use-constellation-data";

const adapterMock = vi.hoisted(() => ({
  requestWebGPUAdapter: vi.fn(),
}));

vi.mock("../../../canvas/webgpu-adapter", () => ({
  requestWebGPUAdapter: adapterMock.requestWebGPUAdapter,
}));

vi.mock("../../../canvas/frame-metrics", () => ({
  recordFrameMs: vi.fn().mockResolvedValue(undefined),
  normalizeWgpuBackend: () => "vulkan" as const,
  detectWebviewBackend: () => "webview2" as const,
}));

beforeEach(() => {
  adapterMock.requestWebGPUAdapter.mockReset();
});

afterEach(() => {
  vi.unstubAllGlobals();
  vi.clearAllMocks();
});

const services: ServiceAggregate[] = [
  { serviceName: "svc-a", throughputHz: 10, errorRate: 0, position: { x: 1, y: 0 } },
  { serviceName: "svc-b", throughputHz: 20, errorRate: 0.5, position: { x: -1, y: 0 } },
];

describe("ConstellationCanvas", () => {
  it("renders <section aria-label='Service constellation'> wrapper without explicit role", async () => {
    adapterMock.requestWebGPUAdapter.mockResolvedValue({
      kind: "unavailable",
      reason: "no-navigator-gpu",
    });
    render(<ConstellationCanvas services={services} />);
    const wrapper = await screen.findByRole("region", { name: "Service constellation" });
    expect(wrapper.tagName).toBe("SECTION");
    // Implicit role only — explicit role attribute is forbidden by jsx-a11y.
    expect(wrapper.hasAttribute("role")).toBe(false);
  });

  it("exposes service count via data-service-count for downstream tests", () => {
    adapterMock.requestWebGPUAdapter.mockResolvedValue({
      kind: "unavailable",
      reason: "no-navigator-gpu",
    });
    render(<ConstellationCanvas services={services} />);
    const wrapper = screen.getByTestId("constellation-canvas");
    expect(wrapper.getAttribute("data-service-count")).toBe("2");
  });

  it("renders Fallback when WebGPU adapter is unavailable", async () => {
    adapterMock.requestWebGPUAdapter.mockResolvedValue({
      kind: "unavailable",
      reason: "no-navigator-gpu",
    });
    render(<ConstellationCanvas services={services} />);
    // Wait for the adapter promise to resolve + fallback to render.
    const wrapper = screen.getByTestId("constellation-canvas");
    await vi.waitFor(() => {
      expect(wrapper.querySelector("canvas")).toBeNull();
    });
  });

  it("requests the WebGPU adapter on mount", () => {
    adapterMock.requestWebGPUAdapter.mockResolvedValue({
      kind: "unavailable",
      reason: "no-navigator-gpu",
    });
    render(<ConstellationCanvas services={services} />);
    expect(adapterMock.requestWebGPUAdapter).toHaveBeenCalled();
  });

  it("handles empty services array without crashing", async () => {
    adapterMock.requestWebGPUAdapter.mockResolvedValue({
      kind: "unavailable",
      reason: "no-navigator-gpu",
    });
    render(<ConstellationCanvas services={[]} />);
    const wrapper = await screen.findByRole("region", { name: "Service constellation" });
    expect(wrapper.getAttribute("data-service-count")).toBe("0");
  });
});
