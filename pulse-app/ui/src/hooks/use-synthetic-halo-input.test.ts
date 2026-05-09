import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook } from "@testing-library/react";
import { useSyntheticHaloInput } from "./use-synthetic-halo-input";

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

describe("useSyntheticHaloInput", () => {
  it("returns initial HaloInput shape", () => {
    const { result } = renderHook(() => useSyntheticHaloInput());
    expect(result.current).toEqual(
      expect.objectContaining({
        throughputHz: expect.any(Number),
        errorRate: expect.any(Number),
      }),
    );
  });

  it("advances HaloInput on each interval tick", () => {
    const { result } = renderHook(() => useSyntheticHaloInput());
    const initial = result.current.throughputHz;
    act(() => {
      vi.advanceTimersByTime(250);
    });
    expect(result.current.throughputHz).not.toBe(initial);
  });

  it("keeps errorRate in [0, 1]", () => {
    const { result } = renderHook(() => useSyntheticHaloInput());
    for (let i = 0; i < 200; i += 1) {
      act(() => {
        vi.advanceTimersByTime(250);
      });
      expect(result.current.errorRate).toBeGreaterThanOrEqual(0);
      expect(result.current.errorRate).toBeLessThanOrEqual(1);
    }
  });

  it("clears interval on unmount", () => {
    const clearIntervalSpy = vi.spyOn(globalThis, "clearInterval");
    const { unmount } = renderHook(() => useSyntheticHaloInput());
    unmount();
    expect(clearIntervalSpy).toHaveBeenCalled();
    clearIntervalSpy.mockRestore();
  });
});
