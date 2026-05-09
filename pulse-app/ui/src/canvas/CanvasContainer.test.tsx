import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";

vi.mock("../hooks/use-reduced-motion", () => ({
  useReducedMotion: vi.fn().mockReturnValue(false),
}));

vi.mock("./render-pipeline", () => ({
  createTraceTimelinePipeline: vi.fn().mockReturnValue({}),
  createFlamegraphPipeline: vi.fn().mockReturnValue({}),
  createMetricsChartPipeline: vi.fn().mockReturnValue({}),
}));

vi.mock("./compute-pipeline", () => ({
  createAggregationComputePipeline: vi
    .fn()
    .mockReturnValue({ kind: "created", pipeline: {} as GPUComputePipeline }),
}));

vi.mock("./frame-metrics", () => ({
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
const renderPipelineModule = await import("./render-pipeline");
const computePipelineModule = await import("./compute-pipeline");
const frameMetricsModule = await import("./frame-metrics");
const { CanvasContainer } = await import("./CanvasContainer");

function stubGpuAvailable() {
  const fakeDevice = {
    queue: {
      onSubmittedWorkDone: vi.fn().mockResolvedValue(undefined),
    },
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
  }) as unknown as typeof HTMLCanvasElement.prototype.getContext;
}

function stubGpuUnavailable() {
  vi.stubGlobal("navigator", { gpu: undefined });
}

describe("CanvasContainer — semantic wrapper + adapter branch + reduced-motion gate", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.mocked(useReducedMotion).mockReturnValue(false);
    vi.mocked(renderPipelineModule.createTraceTimelinePipeline).mockClear();
    vi.mocked(renderPipelineModule.createFlamegraphPipeline).mockClear();
    vi.mocked(renderPipelineModule.createMetricsChartPipeline).mockClear();
    vi.mocked(computePipelineModule.createAggregationComputePipeline).mockClear();
    vi.mocked(computePipelineModule.createAggregationComputePipeline).mockReturnValue({
      kind: "created",
      pipeline: {} as GPUComputePipeline,
    });
    vi.mocked(frameMetricsModule.recordFrameMs).mockClear();
  });

  it("renders <section role='region'> wrapper with the provided aria-label", async () => {
    stubGpuAvailable();
    render(<CanvasContainer ariaLabel="Telemetry traces chart" />);
    const region = await screen.findByRole("region", { name: "Telemetry traces chart" });
    expect(region.tagName).toBe("SECTION");
  });

  it("applies design-token chrome on the wrapper (color-inset bg, radius-md, space-md padding)", async () => {
    stubGpuAvailable();
    render(<CanvasContainer ariaLabel="Telemetry visualization canvas" />);
    const region = await screen.findByRole("region");
    expect(region.style.background).toBe("var(--color-inset)");
    expect(region.style.borderRadius).toBe("var(--radius-md)");
    expect(region.style.padding).toBe("var(--spacing-md)");
  });

  it("renders <canvas> when WebGPU adapter is available + initializes 3 render pipelines + compute pipeline", async () => {
    stubGpuAvailable();
    const { container } = render(<CanvasContainer ariaLabel="Telemetry chart" />);
    await waitFor(() => {
      expect(renderPipelineModule.createTraceTimelinePipeline).toHaveBeenCalled();
    });
    expect(renderPipelineModule.createFlamegraphPipeline).toHaveBeenCalled();
    expect(renderPipelineModule.createMetricsChartPipeline).toHaveBeenCalled();
    expect(computePipelineModule.createAggregationComputePipeline).toHaveBeenCalled();
    expect(container.querySelector("canvas")).not.toBeNull();
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("renders <Fallback role='alert'> when navigator.gpu is undefined", async () => {
    stubGpuUnavailable();
    const { container } = render(<CanvasContainer ariaLabel="Telemetry chart" />);
    await waitFor(() => {
      expect(screen.getByRole("alert")).toBeDefined();
    });
    expect(screen.getByText("WebGPU not supported in this browser")).toBeDefined();
    expect(container.querySelector("canvas")).toBeNull();
  });

  it("does NOT initialize render pipelines when adapter is unavailable", async () => {
    stubGpuUnavailable();
    render(<CanvasContainer ariaLabel="Telemetry chart" />);
    await waitFor(() => {
      expect(screen.getByRole("alert")).toBeDefined();
    });
    expect(renderPipelineModule.createTraceTimelinePipeline).not.toHaveBeenCalled();
    expect(renderPipelineModule.createFlamegraphPipeline).not.toHaveBeenCalled();
    expect(renderPipelineModule.createMetricsChartPipeline).not.toHaveBeenCalled();
    expect(computePipelineModule.createAggregationComputePipeline).not.toHaveBeenCalled();
  });

  it("renders <Fallback> when the compute pipeline fails (chunk #29 substrate failure)", async () => {
    stubGpuAvailable();
    vi.mocked(computePipelineModule.createAggregationComputePipeline).mockReturnValue({
      kind: "failed",
      reason: "compute pipeline creation failed",
    });
    const { container } = render(<CanvasContainer ariaLabel="Telemetry chart" />);
    await waitFor(() => {
      expect(screen.getByRole("alert")).toBeDefined();
    });
    expect(container.querySelector("canvas")).toBeNull();
  });

  it("respects prefers-reduced-motion: reduce by suppressing the rAF loop (onReducedMotionFrame branch)", async () => {
    vi.mocked(useReducedMotion).mockReturnValue(true);
    stubGpuAvailable();
    const rafSpy = vi.spyOn(globalThis, "requestAnimationFrame");
    render(<CanvasContainer ariaLabel="Telemetry chart" />);
    await waitFor(() => {
      expect(renderPipelineModule.createTraceTimelinePipeline).toHaveBeenCalled();
    });
    expect(rafSpy).not.toHaveBeenCalled();
    rafSpy.mockRestore();
  });

  it("invokes recordFrameMs from the reduced-motion static-paint branch (stable 4-field shape)", async () => {
    vi.mocked(useReducedMotion).mockReturnValue(true);
    stubGpuAvailable();
    render(<CanvasContainer ariaLabel="Telemetry chart" />);
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
  });

  it("renders the optional mirrorTable slot when provided (chart-text DOM mirror per a11y plan)", async () => {
    stubGpuAvailable();
    render(
      <CanvasContainer
        ariaLabel="Telemetry chart"
        mirrorTable={<table data-testid="trace-table" />}
      />,
    );
    expect(await screen.findByTestId("trace-table")).toBeDefined();
  });
});
