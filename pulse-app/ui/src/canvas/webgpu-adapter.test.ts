import { afterEach, describe, expect, it, vi } from "vitest";
import { requestWebGPUAdapter } from "./webgpu-adapter";

const { reportAdapterOutcome } = vi.hoisted(() => ({
  reportAdapterOutcome: vi.fn(async () => {}),
}));

vi.mock("./adapter-state", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./adapter-state")>();
  return { ...actual, reportAdapterOutcome };
});

describe("requestWebGPUAdapter — adapter detection + sanitized fallback", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    reportAdapterOutcome.mockClear();
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
  it("returns unavailable with `requestAdapter rejected` when the adapter request rejects", async () => {
    vi.stubGlobal("navigator", {
      gpu: {
        requestAdapter: vi.fn().mockRejectedValue(new Error("GPU process crashed at gpu_main.cc:88")),
      },
    });
    const result = await requestWebGPUAdapter();
    expect(result).toEqual({ kind: "unavailable", reason: "requestAdapter rejected" });
  });
});

describe("requestWebGPUAdapter — reports each outcome once", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    reportAdapterOutcome.mockClear();
  });

  const adapterWith = (requestDevice: () => Promise<unknown>) =>
    ({ requestDevice: vi.fn(requestDevice), info: { backend: "dx12" } }) as unknown as GPUAdapter;

  it.each([
    ["no_navigator_gpu", () => ({ gpu: undefined })],
    ["adapter_null", () => ({ gpu: { requestAdapter: vi.fn().mockResolvedValue(null) } })],
    [
      "adapter_request_rejected",
      () => ({ gpu: { requestAdapter: vi.fn().mockRejectedValue(new Error("lost")) } }),
    ],
    [
      "device_request_failed",
      () => ({
        gpu: {
          requestAdapter: vi
            .fn()
            .mockResolvedValue(adapterWith(() => Promise.reject(new Error("bad descriptor")))),
        },
      }),
    ],
    [
      "obtained",
      () => ({
        gpu: {
          requestAdapter: vi.fn().mockResolvedValue(adapterWith(() => Promise.resolve({}))),
        },
      }),
    ],
  ])("reports %s exactly once", async (outcome, navigatorStub) => {
    vi.stubGlobal("navigator", navigatorStub());
    await requestWebGPUAdapter();
    expect(reportAdapterOutcome).toHaveBeenCalledTimes(1);
    expect(reportAdapterOutcome).toHaveBeenCalledWith(outcome);
  });
});
