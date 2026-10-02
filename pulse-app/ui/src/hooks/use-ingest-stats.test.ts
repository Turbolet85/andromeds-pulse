import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook } from "@testing-library/react";
import type { ReadyEnvelope } from "../bindings/index";
import { __setProxyForTest, useIngestStats } from "./use-ingest-stats";

function readyEnv(rows: number, used = 120, retention = 600): ReadyEnvelope {
  return {
    ready: true,
    checked_at: "2026-07-07T19:00:00Z",
    checks: {
      duckdb_connection: "ok",
      ingest_mpsc_capacity_pct: 0,
      broadcast_subscribers: 0,
      plugins_loaded: 0,
      mcp_server_enabled: false,
      rows_ingested: rows,
      buffer_used_seconds: used,
      retention_seconds: retention,
    },
  };
}

let readyFn: ReturnType<typeof vi.fn>;

beforeEach(() => {
  readyFn = vi.fn().mockResolvedValue(readyEnv(0));
  __setProxyForTest({ ready: readyFn } as never);
});

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
});

describe("useIngestStats", () => {
  it("exposes buffer + retention + hasData from ready() checks", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval", "Date"] });
    try {
      readyFn.mockResolvedValue(readyEnv(500, 180, 600));
      const { result } = renderHook(() => useIngestStats());
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(result.current?.bufferUsedSeconds).toBe(180);
      expect(result.current?.retentionSeconds).toBe(600);
      expect(result.current?.hasData).toBe(true);
      // No prior sample yet → no fabricated rate.
      expect(result.current?.spansPerSec).toBeNull();
    } finally {
      vi.useRealTimers();
    }
  });

  it("derives spans/s from the rows_ingested delta across polls", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval", "Date"] });
    try {
      readyFn.mockResolvedValueOnce(readyEnv(1000)).mockResolvedValue(readyEnv(1540));
      const { result } = renderHook(() => useIngestStats());
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(result.current?.spansPerSec).toBeNull();
      // +540 rows over 1s → ~540 spans/s.
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });
      expect(result.current?.spansPerSec).toBeCloseTo(540, 0);
    } finally {
      vi.useRealTimers();
    }
  });

  it("clears the interval on unmount (no further polls)", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval", "Date"] });
    try {
      const { unmount } = renderHook(() => useIngestStats());
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      unmount();
      const callsAtUnmount = readyFn.mock.calls.length;
      await act(async () => {
        await vi.advanceTimersByTimeAsync(3000);
      });
      expect(readyFn.mock.calls.length).toBe(callsAtUnmount);
    } finally {
      vi.useRealTimers();
    }
  });

  it("keeps last-good stats when a re-poll fails after data has loaded", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval", "Date"] });
    try {
      readyFn.mockResolvedValue(readyEnv(800, 120, 600));
      const { result } = renderHook(() => useIngestStats());
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(result.current?.bufferUsedSeconds).toBe(120);

      readyFn.mockRejectedValue({ kind: "storage", message: "transient" });
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });

      expect(result.current?.bufferUsedSeconds).toBe(120);
      expect(result.current?.hasData).toBe(true);
    } finally {
      vi.useRealTimers();
    }
  });
});
