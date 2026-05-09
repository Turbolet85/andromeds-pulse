import { describe, expect, it, vi } from "vitest";
import { createHaloPipeline } from "./halo-pipeline";

function stubDevice(overrides: Partial<GPUDevice> = {}): GPUDevice {
  return {
    createShaderModule: vi.fn().mockReturnValue({}),
    createRenderPipeline: vi.fn().mockReturnValue({}),
    ...overrides,
  } as unknown as GPUDevice;
}

describe("createHaloPipeline — render pipeline factory with tagged-union failure", () => {
  it("returns kind=created on success", () => {
    const device = stubDevice();
    const result = createHaloPipeline(device, "bgra8unorm");
    expect(result.kind).toBe("created");
    if (result.kind === "created") {
      expect(result.pipeline).toBeDefined();
    }
  });

  it("returns kind=failed with generic discriminator on shader-module compile error", () => {
    const device = stubDevice({
      createShaderModule: vi.fn().mockImplementation(() => {
        throw new Error("OPAQUE BROWSER ERROR with internal details");
      }),
    });
    const result = createHaloPipeline(device, "bgra8unorm");
    expect(result.kind).toBe("failed");
    if (result.kind === "failed") {
      expect(result.reason).toBe("halo pipeline creation failed");
      // Sanitization invariant: reason MUST NOT expose raw error.message contents
      expect(result.reason).not.toContain("OPAQUE");
      expect(result.reason).not.toContain("internal details");
    }
  });

  it("returns kind=failed with 'unsupported feature' on NotSupportedError from pipeline creation", () => {
    const err = new Error("Not supported");
    Object.defineProperty(err, "name", { value: "NotSupportedError" });
    const device = stubDevice({
      createRenderPipeline: vi.fn().mockImplementation(() => {
        throw err;
      }),
    });
    const result = createHaloPipeline(device, "bgra8unorm");
    expect(result.kind).toBe("failed");
    if (result.kind === "failed") {
      expect(result.reason).toBe("unsupported feature");
    }
  });

  it("collapses unknown errors to generic discriminator (defense-in-depth)", () => {
    const device = stubDevice({
      createRenderPipeline: vi.fn().mockImplementation(() => {
        throw new Error("Some other error");
      }),
    });
    const result = createHaloPipeline(device, "bgra8unorm");
    expect(result.kind).toBe("failed");
    if (result.kind === "failed") {
      expect(result.reason).toBe("halo pipeline creation failed");
    }
  });

  it("does not leak raw GPUError.message contents into reason on validation failure", () => {
    const device = stubDevice({
      createRenderPipeline: vi.fn().mockImplementation(() => {
        const err = new Error("WGSL: validation failed at line 42 column 17 — 'foo' undeclared");
        Object.defineProperty(err, "name", { value: "GPUValidationError" });
        throw err;
      }),
    });
    const result = createHaloPipeline(device, "bgra8unorm");
    expect(result.kind).toBe("failed");
    if (result.kind === "failed") {
      // Reason is the allowlist discriminator string only; never the raw shader
      // compile detail per security plan §Anti-Patterns Logging row 4.
      expect(result.reason).toBe("halo pipeline creation failed");
      expect(result.reason).not.toContain("WGSL");
      expect(result.reason).not.toContain("line 42");
      expect(result.reason).not.toContain("foo");
    }
  });
});
