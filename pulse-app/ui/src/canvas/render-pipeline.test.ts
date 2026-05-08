import { describe, expect, it, vi } from "vitest";
import {
  createFlamegraphPipeline,
  createMetricsChartPipeline,
  createTraceTimelinePipeline,
} from "./render-pipeline";

function makeDeviceMock() {
  const shaderModule = {} as GPUShaderModule;
  const pipeline = {} as GPURenderPipeline;
  return {
    device: {
      createShaderModule: vi.fn().mockReturnValue(shaderModule),
      createRenderPipeline: vi.fn().mockReturnValue(pipeline),
    } as unknown as GPUDevice,
    pipeline,
  };
}

describe("render-pipeline factories — substrate for chunks #34/#35", () => {
  it("createTraceTimelinePipeline invokes createShaderModule once with non-empty WGSL source + returns pipeline", () => {
    const { device, pipeline } = makeDeviceMock();
    const result = createTraceTimelinePipeline(device, "bgra8unorm");
    expect(device.createShaderModule).toHaveBeenCalledTimes(1);
    const arg = (device.createShaderModule as ReturnType<typeof vi.fn>).mock.calls[0][0] as {
      code: string;
      label?: string;
    };
    expect(arg.code).toBeTruthy();
    expect(arg.code.length).toBeGreaterThan(0);
    expect(arg.label).toBe("trace-timeline");
    expect(result).toBe(pipeline);
  });

  it("createFlamegraphPipeline produces pipeline labelled `flamegraph`", () => {
    const { device, pipeline } = makeDeviceMock();
    const result = createFlamegraphPipeline(device, "bgra8unorm");
    const arg = (device.createShaderModule as ReturnType<typeof vi.fn>).mock.calls[0][0] as {
      code: string;
      label?: string;
    };
    expect(arg.label).toBe("flamegraph");
    expect(result).toBe(pipeline);
  });

  it("createMetricsChartPipeline produces pipeline labelled `metrics-chart`", () => {
    const { device, pipeline } = makeDeviceMock();
    const result = createMetricsChartPipeline(device, "bgra8unorm");
    const arg = (device.createShaderModule as ReturnType<typeof vi.fn>).mock.calls[0][0] as {
      code: string;
      label?: string;
    };
    expect(arg.label).toBe("metrics-chart");
    expect(result).toBe(pipeline);
  });

  it("rethrows sanitized one-liner when shader compile fails (no stack / file path leak)", () => {
    const device = {
      createShaderModule: vi.fn().mockImplementation(() => {
        throw new Error(
          "GPUValidationError: parsing 'src/shaders/trace.wgsl' line 14: expected ';' at compiler.cc:892",
        );
      }),
      createRenderPipeline: vi.fn(),
    } as unknown as GPUDevice;
    expect(() => createTraceTimelinePipeline(device, "bgra8unorm")).toThrow(
      /shader compile failed: trace-timeline/,
    );
    try {
      createTraceTimelinePipeline(device, "bgra8unorm");
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      expect(message).not.toContain("GPUValidationError");
      expect(message).not.toContain("compiler.cc");
      expect(message).not.toContain("src/shaders/trace.wgsl");
    }
  });

  it("uses primitive topology `triangle-list` (substrate full-screen-quad render)", () => {
    const { device } = makeDeviceMock();
    createTraceTimelinePipeline(device, "bgra8unorm");
    const arg = (device.createRenderPipeline as ReturnType<typeof vi.fn>).mock.calls[0][0] as {
      primitive: { topology: string };
    };
    expect(arg.primitive.topology).toBe("triangle-list");
  });
});
