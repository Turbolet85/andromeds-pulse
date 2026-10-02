import { describe, expect, it, vi } from "vitest";
import { createAggregationComputePipeline } from "./compute-pipeline";

function makeDeviceMock() {
  const shaderModule = {} as GPUShaderModule;
  const pipeline = {} as GPUComputePipeline;
  return {
    device: {
      createShaderModule: vi.fn().mockReturnValue(shaderModule),
      createComputePipeline: vi.fn().mockReturnValue(pipeline),
    } as unknown as GPUDevice,
    pipeline,
  };
}

describe("createAggregationComputePipeline — substrate for chunks #34/#35", () => {
  it("returns kind=created with the GPUComputePipeline on success", () => {
    const { device, pipeline } = makeDeviceMock();
    const result = createAggregationComputePipeline(device);
    expect(result.kind).toBe("created");
    if (result.kind === "created") {
      expect(result.pipeline).toBe(pipeline);
    }
  });

  it("calls createShaderModule with non-empty WGSL source labelled `aggregation`", () => {
    const { device } = makeDeviceMock();
    createAggregationComputePipeline(device);
    expect(device.createShaderModule).toHaveBeenCalledTimes(1);
    const arg = (device.createShaderModule as ReturnType<typeof vi.fn>).mock.calls[0][0] as {
      code: string;
      label?: string;
    };
    expect(arg.code).toBeTruthy();
    expect(arg.code.length).toBeGreaterThan(0);
    expect(arg.label).toBe("aggregation");
  });

  it("uses entryPoint `main` matching the shader source", () => {
    const { device } = makeDeviceMock();
    createAggregationComputePipeline(device);
    const arg = (device.createComputePipeline as ReturnType<typeof vi.fn>).mock.calls[0][0] as {
      compute: { entryPoint: string };
    };
    expect(arg.compute.entryPoint).toBe("main");
  });

  it("returns sanitized 'compute pipeline creation failed' on generic shader-module exception", () => {
    const device = {
      createShaderModule: vi.fn().mockImplementation(() => {
        throw new Error(
          "GPUValidationError: parsing 'src/shaders/aggregation.wgsl' line 14 at compiler.cc:892",
        );
      }),
      createComputePipeline: vi.fn(),
    } as unknown as GPUDevice;
    const result = createAggregationComputePipeline(device);
    expect(result.kind).toBe("failed");
    if (result.kind === "failed") {
      expect(result.reason).toBe("compute pipeline creation failed");
    }
  });

  it("classifies NotSupportedError on createComputePipeline as 'unsupported feature'", () => {
    const device = {
      createShaderModule: vi.fn().mockReturnValue({} as GPUShaderModule),
      createComputePipeline: vi.fn().mockImplementation(() => {
        const err = new Error("feature missing");
        (err as Error & { name: string }).name = "NotSupportedError";
        throw err;
      }),
    } as unknown as GPUDevice;
    const result = createAggregationComputePipeline(device);
    expect(result.kind).toBe("failed");
    if (result.kind === "failed") {
      expect(result.reason).toBe("unsupported feature");
    }
  });

  it("classifies OperationError on createComputePipeline as 'unsupported limit'", () => {
    const device = {
      createShaderModule: vi.fn().mockReturnValue({} as GPUShaderModule),
      createComputePipeline: vi.fn().mockImplementation(() => {
        const err = new Error("limit exceeded");
        (err as Error & { name: string }).name = "OperationError";
        throw err;
      }),
    } as unknown as GPUDevice;
    const result = createAggregationComputePipeline(device);
    expect(result.kind).toBe("failed");
    if (result.kind === "failed") {
      expect(result.reason).toBe("unsupported limit");
    }
  });

  it("never propagates GPUError details / file paths / compiler internals in the failure reason", () => {
    const device = {
      createShaderModule: vi.fn().mockImplementation(() => {
        throw new Error(
          "GPUValidationError: src/shaders/aggregation.wgsl:14: expected ';' at compiler.cc:892",
        );
      }),
      createComputePipeline: vi.fn(),
    } as unknown as GPUDevice;
    const result = createAggregationComputePipeline(device);
    expect(result.kind).toBe("failed");
    if (result.kind === "failed") {
      expect(result.reason).not.toContain("GPUValidationError");
      expect(result.reason).not.toContain("compiler.cc");
      expect(result.reason).not.toContain("aggregation.wgsl");
    }
  });
});
