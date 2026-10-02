import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";

const mocks = vi.hoisted(() => ({
  listActiveFn: vi.fn(),
  markAllReadFn: vi.fn(),
  recordFindingsCounterRefreshFn: vi.fn(),
}));

vi.mock("../canvas/frame-metrics", () => ({
  recordFindingsCounterRefresh: mocks.recordFindingsCounterRefreshFn,
}));

vi.mock("../bindings/index", () => ({
  createTauRPCProxy: () => ({
    incidents: {
      list_active: mocks.listActiveFn,
      mark_all_read: mocks.markAllReadFn,
    },
  }),
}));

import { useFindings } from "./use-findings";

const NOW_NANO = 1_700_000_000_000;
const SECOND = 1_000_000_000;
const MINUTE = 60 * SECOND;

function makeRecord(overrides: Partial<Record<string, unknown>> = {}) {
  return {
    id: 1,
    workspace: "ws-a",
    kind: "error_rate_spike",
    scope: "service",
    status: "active",
    severity: "warn",
    priority_tier: "suggested",
    title: "sample",
    detail: "[redacted]",
    opened_at_unix_nano: NOW_NANO - 5 * MINUTE,
    updated_at_unix_nano: NOW_NANO - 5 * MINUTE,
    acknowledged_at_unix_nano: null,
    resolved_at_unix_nano: null,
    read_at_unix_nano: null,
    evidence_count: 0,
    ...overrides,
  };
}

beforeEach(() => {
  mocks.listActiveFn.mockReset();
  mocks.markAllReadFn.mockReset();
  // vitest 4's restoreAllMocks no longer clears vi.fn() call history, so an
  // absence assertion on the recorder would read earlier tests' calls.
  mocks.recordFindingsCounterRefreshFn.mockReset();
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe("useFindings — initial fetch", () => {
  it("returns empty state before resolve", () => {
    mocks.listActiveFn.mockReturnValue(new Promise(() => {}));
    const { result } = renderHook(() => useFindings());
    expect(result.current.rows).toEqual([]);
    expect(result.current.count).toBe(0);
    expect(result.current.severityMax).toBeNull();
  });

  it("populates rows after list_active resolves", async () => {
    mocks.listActiveFn.mockResolvedValue({
      items: [
        makeRecord({ id: 1, priority_tier: "autonomous" }),
        makeRecord({ id: 2, priority_tier: "suggested" }),
      ],
      total: 2,
      next_cursor: null,
    });
    const { result } = renderHook(() => useFindings());
    await waitFor(() => expect(result.current.count).toBe(2));
    expect(result.current.severityMax).toBe("autonomous");
    expect(result.current.rows.map((r) => r.id)).toEqual([1, 2]);
  });

  it("silently catches IPC errors (jsdom-style)", async () => {
    mocks.listActiveFn.mockRejectedValue(new Error("no TAURI_INTERNALS"));
    const { result } = renderHook(() => useFindings());
    await waitFor(() => expect(mocks.listActiveFn).toHaveBeenCalled());
    expect(result.current.rows).toEqual([]);
    expect(result.current.count).toBe(0);
  });
});

describe("useFindings — markAllRead", () => {
  it("invokes IPC mark_all_read + clears local unread state", async () => {
    mocks.listActiveFn.mockResolvedValue({
      items: [makeRecord({ id: 1 }), makeRecord({ id: 2 })],
      total: 2,
      next_cursor: null,
    });
    mocks.markAllReadFn.mockResolvedValue({ affected_count: 2, marked_at_unix_nano: NOW_NANO });
    const { result } = renderHook(() => useFindings());
    await waitFor(() => expect(result.current.count).toBe(2));

    await act(async () => {
      await result.current.markAllRead();
    });

    expect(mocks.markAllReadFn).toHaveBeenCalledTimes(1);
    await waitFor(() => expect(result.current.count).toBe(0));
    expect(result.current.lastAnnouncement).toBe("All findings marked as read");
  });

  it("silently catches IPC errors on markAllRead", async () => {
    mocks.listActiveFn.mockResolvedValue({
      items: [makeRecord({ id: 1 })],
      total: 1,
      next_cursor: null,
    });
    mocks.markAllReadFn.mockRejectedValue(new Error("no TAURI_INTERNALS"));
    const { result } = renderHook(() => useFindings());
    await waitFor(() => expect(result.current.count).toBe(1));

    await act(async () => {
      await result.current.markAllRead();
    });

    expect(result.current.count).toBe(1);
  });
});

describe("useFindings — window focus refetch", () => {
  it("refetches when window receives focus event", async () => {
    vi.useFakeTimers();
    try {
      mocks.listActiveFn.mockResolvedValue({ items: [], total: 0, next_cursor: null });
      renderHook(() => useFindings());
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(mocks.listActiveFn).toHaveBeenCalledTimes(1);
      await act(async () => {
        window.dispatchEvent(new Event("focus"));
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(mocks.listActiveFn).toHaveBeenCalledTimes(2);
    } finally {
      vi.useRealTimers();
    }
  });
});

describe("useFindings — background re-poll (2026-07-10 CARRY)", () => {
  it("re-polls list_active on the ~1s background interval", async () => {
    vi.useFakeTimers();
    try {
      mocks.listActiveFn.mockResolvedValue({ items: [], total: 0, next_cursor: null });
      renderHook(() => useFindings());
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(mocks.listActiveFn).toHaveBeenCalledTimes(1);
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });
      expect(mocks.listActiveFn).toHaveBeenCalledTimes(2);
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });
      expect(mocks.listActiveFn).toHaveBeenCalledTimes(3);
    } finally {
      vi.useRealTimers();
    }
  });

  it("keeps last-good rows when a re-poll fails after data has loaded", async () => {
    vi.useFakeTimers();
    try {
      mocks.listActiveFn.mockResolvedValueOnce({
        items: [makeRecord({ id: 1 })],
        total: 1,
        next_cursor: null,
      });
      const { result } = renderHook(() => useFindings());
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(result.current.count).toBe(1);
      mocks.listActiveFn.mockRejectedValue(new Error("transient"));
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });
      expect(result.current.count).toBe(1);
    } finally {
      vi.useRealTimers();
    }
  });

  it("announces once on the 0→N edge, never on a subsequent count change", async () => {
    vi.useFakeTimers();
    try {
      mocks.listActiveFn.mockResolvedValueOnce({
        items: [makeRecord({ id: 1 })],
        total: 1,
        next_cursor: null,
      });
      const { result } = renderHook(() => useFindings());
      await act(async () => {
        await vi.advanceTimersByTimeAsync(0);
      });
      expect(result.current.lastAnnouncement).toBe("Findings: 1 unread");
      mocks.listActiveFn.mockResolvedValue({
        items: [makeRecord({ id: 1 }), makeRecord({ id: 2 })],
        total: 2,
        next_cursor: null,
      });
      await act(async () => {
        await vi.advanceTimersByTimeAsync(1000);
      });
      expect(result.current.count).toBe(2);
      expect(result.current.lastAnnouncement).toBe("Findings: 1 unread");
    } finally {
      vi.useRealTimers();
    }
  });
});

describe("useFindings — announcement state", () => {
  it("announces unread count on non-zero mount", async () => {
    mocks.listActiveFn.mockResolvedValue({
      items: [makeRecord({ id: 1 }), makeRecord({ id: 2 }), makeRecord({ id: 3 })],
      total: 3,
      next_cursor: null,
    });
    const { result } = renderHook(() => useFindings());
    await waitFor(() => expect(result.current.lastAnnouncement).toBe("Findings: 3 unread"));
  });

  it("does NOT update announcement on transient zero state", async () => {
    mocks.listActiveFn.mockResolvedValue({
      items: [],
      total: 0,
      next_cursor: null,
    });
    const { result } = renderHook(() => useFindings());
    await waitFor(() => expect(mocks.listActiveFn).toHaveBeenCalled());
    expect(result.current.lastAnnouncement).toBe("");
  });
});

describe("P-045 counter-refresh observable", () => {
  it("emits a refresh latency on a successful poll (never forward-inert)", async () => {
    mocks.listActiveFn.mockResolvedValue({ items: [makeRecord()], total: 1, next_cursor: null });
    const { result } = renderHook(() => useFindings());

    await waitFor(() => {
      expect(result.current.count).toBe(1);
    });
    await waitFor(() => {
      expect(mocks.recordFindingsCounterRefreshFn).toHaveBeenCalled();
    });
    const arg = mocks.recordFindingsCounterRefreshFn.mock.calls[0][0];
    expect(Object.keys(arg)).toEqual(["duration_ms"]);
    expect(Number.isFinite(arg.duration_ms)).toBe(true);
    expect(arg.duration_ms).toBeGreaterThanOrEqual(0);
  });

  it("does NOT emit when the poll rejects (the mark measures a committed refresh)", async () => {
    mocks.listActiveFn.mockRejectedValue(new Error("ipc closed"));
    const { result } = renderHook(() => useFindings());

    // Wait for the rejected poll to settle through the hook's catch branch —
    // this is what makes the absence assertion below non-vacuous rather than
    // merely racing the first render.
    await waitFor(() => {
      expect(mocks.listActiveFn).toHaveBeenCalled();
    });
    expect(result.current.count).toBe(0);
    expect(mocks.recordFindingsCounterRefreshFn).not.toHaveBeenCalled();
  });
});
