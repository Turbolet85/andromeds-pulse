import { afterEach, describe, expect, it, vi } from "vitest";
import { requestWebGPUAdapter } from "./webgpu-adapter";

describe("requestWebGPUAdapter — adapter detection + sanitized fallback", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("returns unavailable with `navigator.gpu undefined` when navigator.gpu is missing", async () => {
    vi.stubGlobal("navigator", { gpu: undefined });
    const result = await requestWebGPUAdapter();
    expect(result.kind).toBe("unavailable");
    if (result.kind === "unavailable") {
      expect(result.reason).toBe("navigator.gpu undefined");
    }
  });

  it("returns unavailable with `requestAdapter returned null` when adapter resolution is null", async () => {
    vi.stubGlobal("navigator", {
      gpu: {
        requestAdapter: vi.fn().mockResolvedValue(null),
      },
    });
    const result = await requestWebGPUAdapter();
    expect(result.kind).toBe("unavailable");
    if (result.kind === "unavailable") {
      expect(result.reason).toBe("requestAdapter returned null");
    }
  });

  it("returns available with backendKind when adapter + device acquire successfully", async () => {
    const fakeDevice = {} as GPUDevice;
    const fakeAdapter = {
      requestDevice: vi.fn().mockResolvedValue(fakeDevice),
      info: { backend: "vulkan" },
    } as unknown as GPUAdapter;
    vi.stubGlobal("navigator", {
      gpu: {
        requestAdapter: vi.fn().mockResolvedValue(fakeAdapter),
      },
    });
    const result = await requestWebGPUAdapter();
    expect(result.kind).toBe("available");
    if (result.kind === "available") {
      expect(result.backendKind).toBe("vulkan");
      expect(result.device).toBe(fakeDevice);
    }
  });

  it("maps unrecognized adapter backend to `unknown`", async () => {
    const fakeDevice = {} as GPUDevice;
    const fakeAdapter = {
      requestDevice: vi.fn().mockResolvedValue(fakeDevice),
      info: { backend: "opengl" },
    } as unknown as GPUAdapter;
    vi.stubGlobal("navigator", {
      gpu: {
        requestAdapter: vi.fn().mockResolvedValue(fakeAdapter),
      },
    });
    const result = await requestWebGPUAdapter();
    expect(result.kind).toBe("available");
    if (result.kind === "available") {
      expect(result.backendKind).toBe("unknown");
    }
  });

  it("returns unavailable with sanitized `requestDevice failed` reason on device throw", async () => {
    const fakeAdapter = {
      requestDevice: vi.fn().mockRejectedValue(
        new Error("OperationError: Device descriptor invalid at line 142 src/webgpu/device.cc"),
      ),
      info: { backend: "metal" },
    } as unknown as GPUAdapter;
    vi.stubGlobal("navigator", {
      gpu: {
        requestAdapter: vi.fn().mockResolvedValue(fakeAdapter),
      },
    });
    const result = await requestWebGPUAdapter();
    expect(result.kind).toBe("unavailable");
    if (result.kind === "unavailable") {
      // The reason MUST be the sanitized allowlist-style discriminator,
      // NOT the raw browser error message (no stack / file path / library
      // version leaks per security plan §Anti-Patterns Logging row 4).
      expect(result.reason).toBe("requestDevice failed");
      expect(result.reason).not.toContain("OperationError");
      expect(result.reason).not.toContain("src/webgpu");
    }
  });
});
