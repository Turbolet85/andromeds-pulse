import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";
import type { PaginatedResponse, TraceRow } from "../../../bindings";
import { __setProxyForTest, useTraces } from "./use-traces";

const sampleRows: TraceRow[] = [
  {
    trace_id: "deadbeef",
    span_id: "00000001",
    ts_unix_nano: 1_700_000_000_000_000_000,
    service: "svc-a",
    duration_ms: 12,
    error_count: 0,
  },
];

const sampleResponse: PaginatedResponse<TraceRow> = {
  items: sampleRows,
  total: 1,
  next_cursor: null,
};

let queryFn: ReturnType<typeof vi.fn>;

beforeEach(() => {
  queryFn = vi.fn().mockResolvedValue(sampleResponse);
  __setProxyForTest({ traces: { query: queryFn } } as never);
});

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
});

describe("useTraces", () => {
  it("starts in loading state", () => {
    const { result } = renderHook(() =>
      useTraces({ timeWindowSeconds: 60, limit: 100 }),
    );
    expect(result.current.isLoading).toBe(true);
    expect(result.current.rows).toEqual([]);
  });

  it("resolves with rows + total + clears loading on success", async () => {
    const { result } = renderHook(() =>
      useTraces({ timeWindowSeconds: 60, limit: 100 }),
    );
    await waitFor(() => expect(result.current.isLoading).toBe(false));
    expect(result.current.rows).toEqual(sampleRows);
    expect(result.current.total).toBe(1);
    expect(result.current.error).toBeNull();
    expect(queryFn).toHaveBeenCalledWith({
      time_window_seconds: 60,
      limit: 100,
      cursor: null,
    });
  });

  it("captures AppError on rejection without throwing", async () => {
    const err = { kind: "storage", message: "viz query failed" } as const;
    queryFn.mockRejectedValueOnce(err);
    const { result } = renderHook(() =>
      useTraces({ timeWindowSeconds: 30, limit: 50 }),
    );
    await waitFor(() => expect(result.current.isLoading).toBe(false));
    expect(result.current.error).toEqual(err);
    expect(result.current.rows).toEqual([]);
  });

  it("falls back to internal AppError shape on unknown rejection", async () => {
    queryFn.mockRejectedValueOnce(new Error("boom"));
    const { result } = renderHook(() =>
      useTraces({ timeWindowSeconds: 60, limit: 100 }),
    );
    await waitFor(() => expect(result.current.isLoading).toBe(false));
    expect(result.current.error).toEqual({
      kind: "internal",
      message: "traces.query failed",
    });
  });

  it("re-polls on the interval and reflects newly-arrived rows", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    try {
      const empty: PaginatedResponse<TraceRow> = {
        items: [],
        total: 0,
        next_cursor: null,
      };
      queryFn.mockResolvedValue(empty);
      const { result } = renderHook(() =>
        useTraces({ timeWindowSeconds: 60, limit: 100 }),
      );
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(result.current.rows).toEqual([]);
      const callsAfterMount = queryFn.mock.calls.length;

      queryFn.mockResolvedValue(sampleResponse);
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });

      expect(queryFn.mock.calls.length).toBeGreaterThan(callsAfterMount);
      expect(result.current.rows).toEqual(sampleRows);
      expect(result.current.total).toBe(1);
      // Every poll stays on page 1 (cursor: null) — pagination is untouched
      // so the latent viz next_cursor keying (query.rs) is not exercised.
      for (const call of queryFn.mock.calls) {
        expect(call[0]).toEqual({
          time_window_seconds: 60,
          limit: 100,
          cursor: null,
        });
      }
    } finally {
      vi.useRealTimers();
    }
  });

  it("clears the interval on unmount (no further polls)", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    try {
      queryFn.mockResolvedValue(sampleResponse);
      const { unmount } = renderHook(() =>
        useTraces({ timeWindowSeconds: 60, limit: 100 }),
      );
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      unmount();
      const callsAtUnmount = queryFn.mock.calls.length;

      await act(async () => {
        await vi.advanceTimersByTimeAsync(3000);
      });

      expect(queryFn.mock.calls.length).toBe(callsAtUnmount);
    } finally {
      vi.useRealTimers();
    }
  });

  it("keeps last-good rows when a re-poll fails after data has loaded", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    try {
      queryFn.mockResolvedValue(sampleResponse);
      const { result } = renderHook(() =>
        useTraces({ timeWindowSeconds: 60, limit: 100 }),
      );
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(result.current.rows).toEqual(sampleRows);
      expect(result.current.error).toBeNull();

      queryFn.mockRejectedValue({ kind: "storage", message: "transient" });
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });

      expect(result.current.rows).toEqual(sampleRows);
      expect(result.current.error).toBeNull();
    } finally {
      vi.useRealTimers();
    }
  });
});
