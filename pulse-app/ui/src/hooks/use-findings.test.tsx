import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";

const mocks = vi.hoisted(() => ({
  listActiveFn: vi.fn(),
  markAllReadFn: vi.fn(),
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
    mocks.listActiveFn.mockResolvedValue({
      items: [],
      total: 0,
      next_cursor: null,
    });
    renderHook(() => useFindings());
    await waitFor(() => expect(mocks.listActiveFn).toHaveBeenCalledTimes(1));

    await act(async () => {
      window.dispatchEvent(new Event("focus"));
    });

    await waitFor(() => expect(mocks.listActiveFn).toHaveBeenCalledTimes(2));
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
