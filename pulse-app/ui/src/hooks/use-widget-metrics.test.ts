import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook } from "@testing-library/react";

const mocks = vi.hoisted(() => {
  const captured: { onMessage?: (payload: number[]) => void } = {};
  return {
    captured,
    subscribeMetrics: vi.fn(async (cb: (payload: number[]) => void) => {
      captured.onMessage = cb;
      return null;
    }),
    getSettings: vi.fn().mockResolvedValue({ retention_seconds: 300 }),
  };
});

vi.mock("../bindings/index", () => ({
  createTauRPCProxy: () => ({
    get_settings: mocks.getSettings,
    streams: { subscribe_metrics: mocks.subscribeMetrics },
  }),
}));

beforeEach(() => {
  vi.useFakeTimers();
  mocks.captured.onMessage = undefined;
  mocks.subscribeMetrics.mockClear();
  mocks.getSettings.mockClear();
  mocks.getSettings.mockResolvedValue({ retention_seconds: 300 });
});

afterEach(() => {
  vi.useRealTimers();
});

import { useWidgetMetrics } from "./use-widget-metrics";

describe("useWidgetMetrics — initial state", () => {
  it("returns canonical WidgetMetrics shape with zero-defaults on first mount", () => {
    const { result } = renderHook(() => useWidgetMetrics());
    expect(result.current).toEqual({
      serviceCount: 0,
      throughputHz: 0,
      errorRate: 0,
      retentionUsedSeconds: 0,
      retentionMaxSeconds: 600,
    });
  });
});

describe("useWidgetMetrics — subscription wiring", () => {
  it("calls streams.subscribe_metrics exactly once on mount", () => {
    renderHook(() => useWidgetMetrics());
    expect(mocks.subscribeMetrics).toHaveBeenCalledTimes(1);
    expect(mocks.captured.onMessage).toBeInstanceOf(Function);
  });

  it("calls get_settings once on mount to fetch retention_seconds", () => {
    renderHook(() => useWidgetMetrics());
    expect(mocks.getSettings).toHaveBeenCalledTimes(1);
  });
});

describe("useWidgetMetrics — throughput aggregation", () => {
  it("throughputHz reflects rolling 1-second payload count after recompute interval", async () => {
    const { result } = renderHook(() => useWidgetMetrics());
    await vi.waitFor(() => expect(mocks.captured.onMessage).toBeInstanceOf(Function));
    act(() => {
      mocks.captured.onMessage!([1, 2, 3]);
      mocks.captured.onMessage!([4, 5, 6]);
      mocks.captured.onMessage!([7, 8, 9]);
    });
    act(() => {
      vi.advanceTimersByTime(1000);
    });
    expect(result.current.throughputHz).toBe(3);
  });

  it("throughputHz resets к 0 if no payloads arrive в next window", async () => {
    const { result } = renderHook(() => useWidgetMetrics());
    await vi.waitFor(() => expect(mocks.captured.onMessage).toBeInstanceOf(Function));
    act(() => {
      mocks.captured.onMessage!([1]);
      vi.advanceTimersByTime(1000);
    });
    expect(result.current.throughputHz).toBe(1);
    act(() => {
      vi.advanceTimersByTime(1000);
    });
    expect(result.current.throughputHz).toBe(0);
  });
});

describe("useWidgetMetrics — retention max from settings", () => {
  it("retentionMaxSeconds reflects fetched setting after first recompute tick", async () => {
    const { result } = renderHook(() => useWidgetMetrics());
    await act(async () => {
      await Promise.resolve();
      await Promise.resolve();
      vi.advanceTimersByTime(1000);
    });
    expect(result.current.retentionMaxSeconds).toBe(300);
  });

  it("retentionMaxSeconds falls back к default 600 if settings fetch rejects", async () => {
    mocks.getSettings.mockRejectedValueOnce(new Error("no Tauri internals"));
    const { result } = renderHook(() => useWidgetMetrics());
    await act(async () => {
      await Promise.resolve();
      await Promise.resolve();
      vi.advanceTimersByTime(1000);
    });
    expect(result.current.retentionMaxSeconds).toBe(600);
  });
});

describe("useWidgetMetrics — cleanup", () => {
  it("clears the recompute interval on unmount", () => {
    const clearIntervalSpy = vi.spyOn(globalThis, "clearInterval");
    const { unmount } = renderHook(() => useWidgetMetrics());
    unmount();
    expect(clearIntervalSpy).toHaveBeenCalled();
    clearIntervalSpy.mockRestore();
  });
});

describe("useWidgetMetrics — deferred к chunk #81 fields", () => {
  it("errorRate / serviceCount / retentionUsedSeconds stay at 0 across recompute ticks (deferred к chunk #81 refactor)", async () => {
    const { result } = renderHook(() => useWidgetMetrics());
    await vi.waitFor(() => expect(mocks.captured.onMessage).toBeInstanceOf(Function));
    act(() => {
      for (let i = 0; i < 100; i += 1) {
        mocks.captured.onMessage!([i]);
      }
      vi.advanceTimersByTime(1000);
    });
    expect(result.current.errorRate).toBe(0);
    expect(result.current.serviceCount).toBe(0);
    expect(result.current.retentionUsedSeconds).toBe(0);
  });
});
