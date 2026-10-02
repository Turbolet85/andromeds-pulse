import { afterEach, describe, expect, it, vi } from "vitest";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createTauRPCProxy } from "../bindings/index";
import { outcomeForReason, reportAdapterOutcome } from "./adapter-state";

vi.mock("@tauri-apps/api/webviewWindow", () => ({
  getCurrentWebviewWindow: vi.fn(),
}));

vi.mock("../bindings/index", () => ({
  createTauRPCProxy: vi.fn(),
}));

function proxyWith(recordWebgpuAdapter: (...args: unknown[]) => Promise<void>) {
  vi.mocked(createTauRPCProxy).mockResolvedValue({
    telemetry: { frontend: { record_webgpu_adapter: recordWebgpuAdapter } },
  } as never);
}

function labelIs(label: string) {
  vi.mocked(getCurrentWebviewWindow).mockReturnValue({ label } as ReturnType<
    typeof getCurrentWebviewWindow
  >);
}

describe("outcomeForReason", () => {
  it.each([
    ["navigator.gpu undefined", "no_navigator_gpu"],
    ["requestAdapter returned null", "adapter_null"],
    ["requestAdapter rejected", "adapter_request_rejected"],
    ["requestDevice failed", "device_request_failed"],
  ] as const)("maps %s to %s", (reason, outcome) => {
    expect(outcomeForReason(reason)).toBe(outcome);
  });
});

describe("reportAdapterOutcome", () => {
  afterEach(() => {
    vi.mocked(createTauRPCProxy).mockReset();
    vi.mocked(getCurrentWebviewWindow).mockReset();
  });

  it("sends exactly the closed outcome and the sanitized window label", async () => {
    const record = vi.fn(async () => {});
    proxyWith(record);
    labelIs("compact-widget");
    await reportAdapterOutcome("adapter_null");
    expect(record).toHaveBeenCalledTimes(1);
    expect(record).toHaveBeenCalledWith({ outcome: "adapter_null", window_label: "compact-widget" });
  });

  it("collapses an out-of-set window label to `unknown`", async () => {
    const record = vi.fn(async () => {});
    proxyWith(record);
    labelIs("devtools-injected");
    await reportAdapterOutcome("obtained");
    expect(record).toHaveBeenCalledWith({ outcome: "obtained", window_label: "unknown" });
  });

  it("swallows a rejected report instead of throwing into the caller", async () => {
    proxyWith(vi.fn(async () => Promise.reject(new Error("Command not allowed by ACL"))));
    labelIs("main");
    await expect(reportAdapterOutcome("device_request_failed")).resolves.toBeUndefined();
  });

  it("swallows a proxy that cannot be created", async () => {
    vi.mocked(createTauRPCProxy).mockRejectedValue(new Error("no __TAURI_INTERNALS__"));
    await expect(reportAdapterOutcome("no_navigator_gpu")).resolves.toBeUndefined();
  });

  it("carries no raw error text — only the two bounded fields cross", async () => {
    const record = vi.fn(async () => {});
    proxyWith(record);
    labelIs("report");
    await reportAdapterOutcome("adapter_request_rejected");
    const [payload] = record.mock.calls[0] as unknown as [Record<string, unknown>];
    expect(Object.keys(payload).sort()).toEqual(["outcome", "window_label"]);
  });
});
