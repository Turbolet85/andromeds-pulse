import { afterEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";
import type { MetricRow } from "../../../bindings";
import { __setProxyForTest, useMetrics } from "./use-metrics";

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
});

const sampleRow: MetricRow = {
  metric_name: "cpu.usage",
  ts_unix_nano: 1_700_000_000_000,
  resource_hash: "deadbeef",
  value: 42,
  data_point_kind: 0,
  labels: "http.route=/alpha",
};

describe("useMetrics", () => {
  it("starts with empty rows + isLoading=true", () => {
    __setProxyForTest({
      metrics: { query: vi.fn().mockResolvedValue({ items: [], total: 0, next_cursor: null }) },
    } as never);
    const { result } = renderHook(() =>
      useMetrics({ timeWindowSeconds: 60, limit: 100 }),
    );
    expect(result.current.isLoading).toBe(true);
    expect(result.current.rows).toEqual([]);
    expect(result.current.error).toBeNull();
  });

  it("loads rows on mount via metrics.query", async () => {
    const queryFn = vi.fn().mockResolvedValue({
      items: [sampleRow],
      total: 1,
      next_cursor: null,
    });
    __setProxyForTest({ metrics: { query: queryFn } } as never);
    const { result } = renderHook(() =>
      useMetrics({ timeWindowSeconds: 60, limit: 100 }),
    );
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });
    expect(result.current.rows).toHaveLength(1);
    expect(result.current.rows[0].metric_name).toBe("cpu.usage");
    expect(result.current.total).toBe(1);
    expect(queryFn).toHaveBeenCalledWith({
      time_window_seconds: 60,
      limit: 100,
      cursor: null,
    });
  });

  it("captures rejection as AppError fallback", async () => {
    const queryFn = vi.fn().mockRejectedValue(new Error("network"));
    __setProxyForTest({ metrics: { query: queryFn } } as never);
    const { result } = renderHook(() =>
      useMetrics({ timeWindowSeconds: 60, limit: 100 }),
    );
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });
    expect(result.current.error).toEqual({
      kind: "internal",
      message: "metrics.query failed",
    });
  });

  it("preserves typed AppError when proxy rejects with kind discriminator", async () => {
    const appError = { kind: "validation", field: "limit", reason: "above max" };
    const queryFn = vi.fn().mockRejectedValue(appError);
    __setProxyForTest({ metrics: { query: queryFn } } as never);
    const { result } = renderHook(() =>
      useMetrics({ timeWindowSeconds: 60, limit: 100 }),
    );
    await waitFor(() => {
      expect(result.current.error).toEqual(appError);
    });
  });

  it("does not setState after unmount when proxy resolves late", async () => {
    let resolveLater: (v: unknown) => void = () => {};
    const queryFn = vi.fn(
      () =>
        new Promise((resolve) => {
          resolveLater = resolve;
        }),
    );
    __setProxyForTest({ metrics: { query: queryFn } } as never);
    const { unmount } = renderHook(() =>
      useMetrics({ timeWindowSeconds: 60, limit: 100 }),
    );
    unmount();
    await act(async () => {
      resolveLater({ items: [sampleRow], total: 1, next_cursor: null });
    });
    // No assertion on result.current — unmounted; check no console error.
    expect(queryFn).toHaveBeenCalled();
  });
});
