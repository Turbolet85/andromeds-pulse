import { afterEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";
import type { ConnectionStatePayload } from "../bindings/index";
import { useConnectionState } from "./use-connection-state";

const proxyMock = vi.hoisted(() => ({
  currentState: vi.fn(),
}));

vi.mock("../bindings/index", () => ({
  createTauRPCProxy: () => ({
    connection: { current_state: proxyMock.currentState },
  }),
}));

const SAMPLE: ConnectionStatePayload = {
  state: { state: "Receiving" },
  last_span_ago_ms: 1500,
  severity: "info",
  message: null,
  reason: null,
};

afterEach(() => {
  proxyMock.currentState.mockReset();
});

describe("useConnectionState", () => {
  it("polls connection.current_state on mount and exposes the payload", async () => {
    proxyMock.currentState.mockResolvedValue(SAMPLE);
    const { result } = renderHook(() => useConnectionState());
    await waitFor(() => expect(proxyMock.currentState).toHaveBeenCalled());
    await waitFor(() => expect(result.current).toEqual(SAMPLE));
  });

  it("re-polls on window focus", async () => {
    proxyMock.currentState.mockResolvedValue(SAMPLE);
    renderHook(() => useConnectionState());
    await waitFor(() => expect(proxyMock.currentState).toHaveBeenCalled());
    const before = proxyMock.currentState.mock.calls.length;
    act(() => {
      window.dispatchEvent(new Event("focus"));
    });
    await waitFor(() =>
      expect(proxyMock.currentState.mock.calls.length).toBeGreaterThan(before),
    );
  });

  it("stays null and does not throw when the proxy rejects (jsdom / pre-init)", async () => {
    proxyMock.currentState.mockRejectedValue(new Error("no tauri internals"));
    const { result } = renderHook(() => useConnectionState());
    await waitFor(() => expect(proxyMock.currentState).toHaveBeenCalled());
    expect(result.current).toBeNull();
  });

  it("removes the focus listener on unmount (no further poll)", async () => {
    proxyMock.currentState.mockResolvedValue(SAMPLE);
    const { unmount } = renderHook(() => useConnectionState());
    await waitFor(() => expect(proxyMock.currentState).toHaveBeenCalled());
    unmount();
    const afterUnmount = proxyMock.currentState.mock.calls.length;
    act(() => {
      window.dispatchEvent(new Event("focus"));
    });
    expect(proxyMock.currentState.mock.calls.length).toBe(afterUnmount);
  });
});
