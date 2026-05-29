import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";

vi.mock("../hooks/use-reduced-motion", () => ({
  useReducedMotion: vi.fn().mockReturnValue(false),
}));

vi.mock("./halo-pipeline", () => ({
  createHaloPipeline: vi.fn().mockReturnValue({
    kind: "created",
    pipeline: {
      getBindGroupLayout: vi.fn().mockReturnValue({}),
    } as unknown as GPURenderPipeline,
  }),
}));

vi.mock("../canvas/frame-metrics", () => ({
  recordFrameMs: vi.fn().mockResolvedValue(undefined),
  detectWebviewBackend: vi.fn().mockReturnValue("webview2"),
  normalizeWgpuBackend: vi.fn().mockImplementation((value: string) => {
    if (value === "metal" || value === "dx12") {
      return value;
    }
    return "vulkan";
  }),
}));

const { useReducedMotion } = await import("../hooks/use-reduced-motion");
const haloPipelineModule = await import("./halo-pipeline");
const frameMetricsModule = await import("../canvas/frame-metrics");
const { HaloCanvas } = await import("./HaloCanvas");

function stubGpuAvailable(): void {
  const fakeDevice = {
    queue: {
      onSubmittedWorkDone: vi.fn().mockResolvedValue(undefined),
      writeBuffer: vi.fn(),
      submit: vi.fn(),
    },
    createBuffer: vi.fn().mockReturnValue({}),
    createBindGroup: vi.fn().mockReturnValue({}),
    createCommandEncoder: vi.fn().mockReturnValue({
      beginRenderPass: vi.fn().mockReturnValue({
        setPipeline: vi.fn(),
        setBindGroup: vi.fn(),
        draw: vi.fn(),
        end: vi.fn(),
      }),
      finish: vi.fn().mockReturnValue({}),
    }),
  } as unknown as GPUDevice;
  const fakeAdapter = {
    requestDevice: vi.fn().mockResolvedValue(fakeDevice),
    info: { backend: "vulkan" },
  } as unknown as GPUAdapter;
  vi.stubGlobal("navigator", {
    gpu: {
      requestAdapter: vi.fn().mockResolvedValue(fakeAdapter),
    },
    userAgent: "Mozilla/5.0 ... Edg/120.0.0.0",
  });

  HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue({
    configure: vi.fn(),
    getCurrentTexture: vi.fn().mockReturnValue({
      createView: vi.fn().mockReturnValue({}),
    }),
  }) as unknown as typeof HTMLCanvasElement.prototype.getContext;
}

function stubGpuUnavailable(): void {
  vi.stubGlobal("navigator", { gpu: undefined });
}

describe("HaloCanvas — semantic wrapper + adapter branch + reduced-motion gate", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.mocked(useReducedMotion).mockReturnValue(false);
    vi.mocked(haloPipelineModule.createHaloPipeline).mockReturnValue({
      kind: "created",
      pipeline: {
        getBindGroupLayout: vi.fn().mockReturnValue({}),
      } as unknown as GPURenderPipeline,
    });
    vi.mocked(haloPipelineModule.createHaloPipeline).mockClear();
    vi.mocked(frameMetricsModule.recordFrameMs).mockClear();
  });

  it("renders <section role='region'> wrapper with the provided aria-label", async () => {
    stubGpuAvailable();
    render(<HaloCanvas ariaLabel="Application status indicator" connectionState={{ state: "Receiving" }} cumulativeSeverity={null} activityState="active" />);
    const region = await screen.findByRole("region", { name: "Application status indicator" });
    expect(region.tagName).toBe("SECTION");
  });

  it("applies design-token chrome (color-inset bg, radius-md, spacing-md padding)", async () => {
    stubGpuAvailable();
    render(<HaloCanvas ariaLabel="Application status indicator" connectionState={{ state: "Receiving" }} cumulativeSeverity={null} activityState="active" />);
    const region = await screen.findByRole("region");
    expect(region.style.background).toBe("var(--color-inset)");
    expect(region.style.borderRadius).toBe("var(--radius-md)");
    expect(region.style.padding).toBe("var(--spacing-md)");
  });

  it("renders <canvas> + creates halo pipeline when WebGPU adapter is available", async () => {
    stubGpuAvailable();
    const { container } = render(
      <HaloCanvas ariaLabel="Halo" connectionState={{ state: "Receiving" }} cumulativeSeverity={null} activityState="active" />,
    );
    await waitFor(() => {
      expect(haloPipelineModule.createHaloPipeline).toHaveBeenCalled();
    });
    expect(container.querySelector("canvas")).not.toBeNull();
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("renders <Fallback role='alert'> when navigator.gpu is undefined", async () => {
    stubGpuUnavailable();
    const { container } = render(
      <HaloCanvas ariaLabel="Halo" connectionState={{ state: "Receiving" }} cumulativeSeverity={null} activityState="active" />,
    );
    await waitFor(() => {
      expect(screen.getByRole("alert")).toBeDefined();
    });
    expect(screen.getByText("WebGPU not supported in this browser")).toBeDefined();
    expect(container.querySelector("canvas")).toBeNull();
  });

  it("renders <Fallback> when createHaloPipeline returns failed", async () => {
    stubGpuAvailable();
    vi.mocked(haloPipelineModule.createHaloPipeline).mockReturnValue({
      kind: "failed",
      reason: "halo pipeline creation failed",
    });
    const { container } = render(
      <HaloCanvas ariaLabel="Halo" connectionState={{ state: "Receiving" }} cumulativeSeverity={null} activityState="active" />,
    );
    await waitFor(() => {
      expect(screen.getByRole("alert")).toBeDefined();
    });
    expect(container.querySelector("canvas")).toBeNull();
  });

  it("respects prefers-reduced-motion: reduce by suppressing the rAF loop", async () => {
    vi.mocked(useReducedMotion).mockReturnValue(true);
    stubGpuAvailable();
    const rafSpy = vi.spyOn(globalThis, "requestAnimationFrame");
    render(<HaloCanvas ariaLabel="Halo" connectionState={{ state: "Receiving" }} cumulativeSeverity={null} activityState="active" />);
    await waitFor(() => {
      expect(haloPipelineModule.createHaloPipeline).toHaveBeenCalled();
    });
    expect(rafSpy).not.toHaveBeenCalled();
    rafSpy.mockRestore();
  });

  it("invokes recordFrameMs from the reduced-motion static-paint branch (stable 4-field shape)", async () => {
    vi.mocked(useReducedMotion).mockReturnValue(true);
    stubGpuAvailable();
    render(<HaloCanvas ariaLabel="Halo" connectionState={{ state: "Receiving" }} cumulativeSeverity={null} activityState="active" />);
    await waitFor(() => {
      expect(frameMetricsModule.recordFrameMs).toHaveBeenCalled();
    });
    const call = vi.mocked(frameMetricsModule.recordFrameMs).mock.calls[0][0];
    expect(Object.keys(call).sort()).toEqual([
      "duration_ms",
      "timing_method",
      "webview_backend",
      "wgpu_backend",
    ]);
    expect(call.duration_ms).toBe(0);
    expect(call.timing_method).toBe("cpu");
  });

  it("re-renders one static-glow frame on cumulativeSeverity prop change under reduced-motion", async () => {
    vi.mocked(useReducedMotion).mockReturnValue(true);
    stubGpuAvailable();
    const { rerender } = render(
      <HaloCanvas
        ariaLabel="Halo"
        connectionState={{ state: "Receiving" }}
        cumulativeSeverity={null}
        activityState="active"
      />,
    );
    await waitFor(() => {
      expect(frameMetricsModule.recordFrameMs).toHaveBeenCalled();
    });
    const callsBeforeRerender = vi.mocked(frameMetricsModule.recordFrameMs).mock.calls.length;
    rerender(
      <HaloCanvas
        ariaLabel="Halo"
        connectionState={{ state: "Receiving" }}
        cumulativeSeverity="autonomous"
        activityState="active"
      />,
    );
    await waitFor(() => {
      expect(vi.mocked(frameMetricsModule.recordFrameMs).mock.calls.length).toBeGreaterThan(
        callsBeforeRerender,
      );
    });
  });
});
