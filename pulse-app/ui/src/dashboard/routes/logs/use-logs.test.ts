import { afterEach, describe, expect, it, vi } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import type { LogRow } from "../../../bindings";
import { __setProxyForTest, useLogs } from "./use-logs";

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
});

const sampleRow: LogRow = {
  ts_unix_nano: 1_700_000_000_000,
  resource_hash: "abc",
  severity_number: 9,
  body: "hello",
  severity_text: "INFO",
  trace_id: "",
  span_id: "",
};

describe("useLogs", () => {
  it("starts with empty rows + isLoading=true", () => {
    __setProxyForTest({
      logs: { query: vi.fn().mockResolvedValue({ items: [], total: 0, next_cursor: null }) },
    } as never);
    const { result } = renderHook(() => useLogs({ timeWindowSeconds: 60, limit: 100 }));
    expect(result.current.isLoading).toBe(true);
    expect(result.current.rows).toEqual([]);
  });

  it("loads rows via logs.query on mount", async () => {
    const queryFn = vi.fn().mockResolvedValue({
      items: [sampleRow],
      total: 1,
      next_cursor: null,
    });
    __setProxyForTest({ logs: { query: queryFn } } as never);
    const { result } = renderHook(() => useLogs({ timeWindowSeconds: 60, limit: 100 }));
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });
    expect(result.current.rows).toHaveLength(1);
    expect(result.current.rows[0].body).toBe("hello");
    expect(queryFn).toHaveBeenCalledWith({
      time_window_seconds: 60,
      limit: 100,
      cursor: null,
    });
  });

  it("captures rejection as AppError fallback", async () => {
    __setProxyForTest({
      logs: { query: vi.fn().mockRejectedValue(new Error("boom")) },
    } as never);
    const { result } = renderHook(() => useLogs({ timeWindowSeconds: 60, limit: 100 }));
    await waitFor(() => {
      expect(result.current.error).toEqual({
        kind: "internal",
        message: "logs.query failed",
      });
    });
  });

  it("preserves typed AppError when proxy rejects with kind discriminator", async () => {
    const appError = { kind: "validation", field: "limit", reason: "above max" };
    __setProxyForTest({ logs: { query: vi.fn().mockRejectedValue(appError) } } as never);
    const { result } = renderHook(() => useLogs({ timeWindowSeconds: 60, limit: 100 }));
    await waitFor(() => {
      expect(result.current.error).toEqual(appError);
    });
  });
});
