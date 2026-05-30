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

function item(
  service: string,
  state: ServiceLifecycleState,
  priorityTier: ServiceListItem["priority_tier"] = null,
): ServiceListItem {
  return {
    service,
    state,
    last_seen_unix_nano: 1_000,
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
    render(
      <ConstellationCanvas items={[item("svc-a", "active"), item("svc-z", "archived")]} />,
    );
    expect(screen.getByTestId("service-constellation").getAttribute("data-service-count")).toBe("1");
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

  it("handles empty items without crashing", async () => {
    render(<ConstellationCanvas items={[]} />);
    const wrapper = await screen.findByRole("region", { name: /no active services/ });
    expect(wrapper.getAttribute("data-service-count")).toBe("0");
  });
});
