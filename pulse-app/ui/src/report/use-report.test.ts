// Copy state machine (`useReport().copyMarkdown`). The report window's Copy
// control was a dead affordance until 2026-08-27: the webview clipboard write
// was ACL-rejected and the bare `catch` turned it into `copyState: "error"`,
// so the press resolved either way. These pins hold the rejection path
// distinguishable from the success path at the unit tier, independently of
// whichever window labels the capability happens to grant.

import { afterEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";

const mocks = vi.hoisted(() => ({
  writeText: vi.fn(),
  proxy: {} as Record<string, unknown>,
}));

vi.mock("@tauri-apps/plugin-clipboard-manager", () => ({
  writeText: mocks.writeText,
}));

vi.mock("../bindings/index", () => ({
  createTauRPCProxy: () => mocks.proxy,
}));

import { AUTO_RESET_MS, useReport } from "./use-report";

const REPORT = { title: "payment-service latency", markdown: "## Symptom\nslow" };

function stubReport(payload: unknown = REPORT) {
  mocks.proxy = {
    incidents: { get_report: () => Promise.resolve(payload) },
  };
}

afterEach(() => {
  mocks.writeText.mockReset();
  mocks.proxy = {};
  vi.useRealTimers();
});

describe("useReport copy state machine", () => {
  it("reaches `copied` when the clipboard write resolves", async () => {
    stubReport();
    mocks.writeText.mockResolvedValue(undefined);
    const { result } = renderHook(() => useReport(7));

    await waitFor(() => expect(result.current.report).not.toBeNull());
    await act(async () => {
      await result.current.copyMarkdown();
    });

    expect(result.current.copyState).toBe("copied");
    expect(mocks.writeText).toHaveBeenCalledWith(REPORT.markdown);
  });

  it("reaches `error` when the clipboard write rejects", async () => {
    stubReport();
    mocks.writeText.mockRejectedValue(new Error("forbidden"));
    const { result } = renderHook(() => useReport(7));

    await waitFor(() => expect(result.current.report).not.toBeNull());
    await act(async () => {
      await result.current.copyMarkdown();
    });

    expect(result.current.copyState).toBe("error");
    expect(mocks.writeText).toHaveBeenCalledTimes(1);
  });

  it("short-circuits to `error` without attempting a write when no report loaded", async () => {
    stubReport();
    const { result } = renderHook(() => useReport(null));

    await act(async () => {
      await result.current.copyMarkdown();
    });

    expect(result.current.copyState).toBe("error");
    expect(mocks.writeText).not.toHaveBeenCalled();
  });

  it("decays a terminal state back to `idle` after AUTO_RESET_MS", async () => {
    stubReport();
    mocks.writeText.mockResolvedValue(undefined);
    const { result } = renderHook(() => useReport(7));

    await waitFor(() => expect(result.current.report).not.toBeNull());

    // Fake ONLY the timeout pair, and only after the load settles: the reset
    // effect must register its timer against the fake clock (installing later
    // leaves a real pending timer that advanceTimersByTime cannot reach), while
    // waitFor's polling above stays on real scheduling. The copy itself is
    // promise-only, so awaiting it under faked timeouts is safe.
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    await act(async () => {
      await result.current.copyMarkdown();
    });
    expect(result.current.copyState).toBe("copied");

    act(() => {
      vi.advanceTimersByTime(AUTO_RESET_MS + 1);
    });

    expect(result.current.copyState).toBe("idle");
  });
});
