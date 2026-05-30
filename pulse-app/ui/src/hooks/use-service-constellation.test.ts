import { afterEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";
import type { ServiceListItem, ServiceListPayload } from "../bindings/index";
import { useServiceConstellation } from "./use-service-constellation";

const proxyMock = vi.hoisted(() => ({
  listWithStates: vi.fn(),
}));

vi.mock("../bindings/index", () => ({
  createTauRPCProxy: () => ({
    services: { list_with_states: proxyMock.listWithStates },
  }),
}));

const ITEMS: ServiceListItem[] = [
  {
    service: "checkout",
    state: "active",
    last_seen_unix_nano: 1_700_000_000_000,
    manual_override: null,
    priority_tier: "autonomous",
  },
  {
    service: "billing",
    state: "quiet",
    last_seen_unix_nano: 1_700_000_000_000,
    manual_override: null,
    priority_tier: null,
  },
];

const PAYLOAD: ServiceListPayload = { items: ITEMS, total: 2, next_cursor: null };

afterEach(() => {
  proxyMock.listWithStates.mockReset();
});

describe("useServiceConstellation", () => {
  it("polls services.list_with_states on mount and exposes the items", async () => {
    proxyMock.listWithStates.mockResolvedValue(PAYLOAD);
    const { result } = renderHook(() => useServiceConstellation());
    await waitFor(() => expect(proxyMock.listWithStates).toHaveBeenCalled());
    await waitFor(() => expect(result.current).toEqual(ITEMS));
  });

  it("re-polls on window focus", async () => {
    proxyMock.listWithStates.mockResolvedValue(PAYLOAD);
    renderHook(() => useServiceConstellation());
    await waitFor(() => expect(proxyMock.listWithStates).toHaveBeenCalled());
    const before = proxyMock.listWithStates.mock.calls.length;
    act(() => {
      window.dispatchEvent(new Event("focus"));
    });
    await waitFor(() =>
      expect(proxyMock.listWithStates.mock.calls.length).toBeGreaterThan(before),
    );
  });

  it("stays empty and does not throw when the proxy rejects (jsdom / pre-init)", async () => {
    proxyMock.listWithStates.mockRejectedValue(new Error("no tauri internals"));
    const { result } = renderHook(() => useServiceConstellation());
    await waitFor(() => expect(proxyMock.listWithStates).toHaveBeenCalled());
    expect(result.current).toEqual([]);
  });

  it("removes the focus listener on unmount (no further poll)", async () => {
    proxyMock.listWithStates.mockResolvedValue(PAYLOAD);
    const { unmount } = renderHook(() => useServiceConstellation());
    await waitFor(() => expect(proxyMock.listWithStates).toHaveBeenCalled());
    unmount();
    const afterUnmount = proxyMock.listWithStates.mock.calls.length;
    act(() => {
      window.dispatchEvent(new Event("focus"));
    });
    expect(proxyMock.listWithStates.mock.calls.length).toBe(afterUnmount);
  });
});
