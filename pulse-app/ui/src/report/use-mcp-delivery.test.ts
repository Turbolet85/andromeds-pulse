// P-039 visibility-gate unit test (chunk #100 capability-audit gap-fill):
// `useMcpDelivery` exposes the "Send to agent" availability ONLY when MCP
// is configured (Settings.mcp_server_enabled) AND an agent is connected
// (mcp.status().connected_agent non-empty). Every other probe outcome —
// either gate false, the feature-gated `mcp` namespace absent from the
// runtime proxy, or the probe throwing — keeps the button hidden (P-040
// default-quiet posture).

import { afterEach, describe, expect, it, vi } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";

const mocks = vi.hoisted(() => ({
  proxy: {} as Record<string, unknown>,
}));

vi.mock("../bindings/index", () => ({
  createTauRPCProxy: () => mocks.proxy,
}));

import { useMcpDelivery } from "./use-mcp-delivery";

function stubProxy(options: {
  enabled?: boolean | undefined;
  connectedAgent?: string | null;
  settingsThrows?: boolean;
  omitMcpNamespace?: boolean;
}) {
  mocks.proxy = {
    get_settings: options.settingsThrows
      ? () => Promise.reject(new Error("ipc rejected"))
      : () => Promise.resolve({ mcp_server_enabled: options.enabled }),
    ...(options.omitMcpNamespace
      ? {}
      : {
          mcp: {
            status: () =>
              Promise.resolve({ connected_agent: options.connectedAgent ?? null }),
          },
        }),
  };
}

afterEach(() => {
  mocks.proxy = {};
});

describe("useMcpDelivery visibility gate (P-039)", () => {
  it("is available when configured AND connected", async () => {
    stubProxy({ enabled: true, connectedAgent: "claude-agent" });
    const { result } = renderHook(() => useMcpDelivery(true));
    await waitFor(() => expect(result.current.available).toBe(true));
  });

  it("stays hidden when configured but no agent is connected", async () => {
    stubProxy({ enabled: true, connectedAgent: null });
    const { result } = renderHook(() => useMcpDelivery(true));
    await waitFor(() => expect(result.current.available).toBe(false));
  });

  it("stays hidden when an agent is connected but MCP is not configured", async () => {
    stubProxy({ enabled: false, connectedAgent: "claude-agent" });
    const { result } = renderHook(() => useMcpDelivery(true));
    await waitFor(() => expect(result.current.available).toBe(false));
  });

  it("stays hidden when the feature-gated mcp namespace is absent from the proxy", async () => {
    stubProxy({ enabled: true, connectedAgent: "claude-agent", omitMcpNamespace: true });
    const { result } = renderHook(() => useMcpDelivery(true));
    await waitFor(() => expect(result.current.available).toBe(false));
  });

  it("treats a throwing probe as not connected (defensive default)", async () => {
    stubProxy({ settingsThrows: true });
    const { result } = renderHook(() => useMcpDelivery(true));
    await waitFor(() => expect(result.current.available).toBe(false));
  });

  it("stays unavailable while the report surface is inactive", async () => {
    stubProxy({ enabled: true, connectedAgent: "claude-agent" });
    const { result } = renderHook(() => useMcpDelivery(false));
    await waitFor(() => expect(result.current.available).toBe(false));
  });
});
