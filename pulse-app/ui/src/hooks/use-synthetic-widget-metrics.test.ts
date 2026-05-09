import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook } from "@testing-library/react";
import { useSyntheticWidgetMetrics } from "./use-synthetic-widget-metrics";

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

describe("useSyntheticWidgetMetrics", () => {
  it("returns initial metrics matching WidgetMetrics shape", () => {
    const { result } = renderHook(() => useSyntheticWidgetMetrics());
    expect(result.current).toEqual(
      expect.objectContaining({
        serviceCount: expect.any(Number),
        throughputHz: expect.any(Number),
        errorRate: expect.any(Number),
        retentionUsedSeconds: expect.any(Number),
        retentionMaxSeconds: 600,
      }),
    );
  });

  it("advances metrics on tick (throughputHz changes after 1s)", () => {
    const { result } = renderHook(() => useSyntheticWidgetMetrics());
    const initial = result.current.throughputHz;
    act(() => {
      vi.advanceTimersByTime(1000);
    });
    expect(result.current.throughputHz).not.toBe(initial);
  });

  it("keeps throughputHz within documented bounds [200, 1800]", () => {
    const { result } = renderHook(() => useSyntheticWidgetMetrics());
    for (let i = 0; i < 30; i += 1) {
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(result.current.throughputHz).toBeGreaterThanOrEqual(200);
      expect(result.current.throughputHz).toBeLessThanOrEqual(1800);
    }
  });

  it("keeps errorRate within documented bounds [0, 0.10]", () => {
    const { result } = renderHook(() => useSyntheticWidgetMetrics());
    for (let i = 0; i < 30; i += 1) {
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(result.current.errorRate).toBeGreaterThanOrEqual(0);
      expect(result.current.errorRate).toBeLessThanOrEqual(0.10);
    }
  });

  it("retentionUsedSeconds ramps 0 → maxSeconds then resets", () => {
    const { result } = renderHook(() => useSyntheticWidgetMetrics());
    let maxSeen = 0;
    for (let i = 0; i < 60; i += 1) {
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      maxSeen = Math.max(maxSeen, result.current.retentionUsedSeconds);
      expect(result.current.retentionUsedSeconds).toBeGreaterThanOrEqual(0);
      expect(result.current.retentionUsedSeconds).toBeLessThan(601);
    }
    expect(maxSeen).toBeGreaterThan(500);
  });

  it("clears interval on unmount", () => {
    const clearIntervalSpy = vi.spyOn(globalThis, "clearInterval");
    const { unmount } = renderHook(() => useSyntheticWidgetMetrics());
    unmount();
    expect(clearIntervalSpy).toHaveBeenCalled();
    clearIntervalSpy.mockRestore();
  });

  it("serviceCount stays within the documented cycle set", () => {
    const allowed = new Set([3, 5, 8, 12, 18, 24, 30]);
    const { result } = renderHook(() => useSyntheticWidgetMetrics());
    for (let i = 0; i < 30; i += 1) {
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(allowed.has(result.current.serviceCount)).toBe(true);
    }
  });
});
