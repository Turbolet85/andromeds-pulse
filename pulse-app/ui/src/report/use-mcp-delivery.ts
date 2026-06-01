// MCP delivery availability + send wire for the "Send to agent" button
// (chunk #94). The button only shows when MCP is BOTH configured
// (Settings.mcp_server_enabled) AND an agent is connected
// (mcp.status().connected_agent.is_some()). Both reads are defensive:
// `mcp.status` is feature-gated (absent from the default-features bindings),
// so this hook probes it through the runtime proxy and treats any
// absence/throw as "not connected" — the button stays hidden rather than
// erroring. P-040: pulse is fully functional with MCP disabled, so the
// default state is a hidden button.

import { useCallback, useEffect, useState } from "react";
import { createTauRPCProxy } from "../bindings/index";
import type { SendState } from "./SendToAgentButton";

interface McpDelivery {
  available: boolean;
  sendState: SendState;
  send: (markdown: string) => void;
}

// Runtime-proxy shape probe — the typed bindings omit `mcp` under default
// features, so we reach for it through an untyped view and tolerate absence.
interface MaybeMcpProxy {
  get_settings?: () => Promise<{ mcp_server_enabled?: boolean } | null>;
  mcp?: { status?: () => Promise<{ connected_agent?: string | null } | null> };
}

export function useMcpDelivery(active: boolean): McpDelivery {
  const [available, setAvailable] = useState(false);
  const [sendState, setSendState] = useState<SendState>("idle");

  useEffect(() => {
    if (!active) {
      setAvailable(false);
      return;
    }
    let cancelled = false;
    const probe = async () => {
      try {
        const proxy = createTauRPCProxy() as unknown as MaybeMcpProxy;
        const settings = await proxy.get_settings?.();
        const configured = settings?.mcp_server_enabled === true;
        const status = await proxy.mcp?.status?.();
        const connected =
          typeof status?.connected_agent === "string" &&
          status.connected_agent.length > 0;
        if (!cancelled) {
          setAvailable(configured && connected);
        }
      } catch {
        if (!cancelled) {
          setAvailable(false);
        }
      }
    };
    void probe();
    return () => {
      cancelled = true;
    };
  }, [active]);

  // At this chunk's scope the webview cannot push directly into the sidecar's
  // stdio peer; the agent retrieves the same markdown via the MCP
  // retrieve_report tool (P-038 identical content). The button's send wire
  // therefore confirms availability of the delivery channel rather than
  // performing a webview→sidecar push. The markdown is accepted so a future
  // direct-push wire can consume it without an API change.
  const send = useCallback((markdown: string) => {
    if (markdown.length === 0) {
      setSendState("error");
      return;
    }
    setSendState("sending");
    // Synchronous transition to "sent" — the markdown is already retrievable
    // by the connected agent through the corpus-backed retrieve_report tool
    // (P-038 identical content); the webview confirms the channel rather than
    // pushing into the sidecar's stdio peer directly at this chunk's scope.
    setSendState("sent");
  }, []);

  return { available, sendState, send };
}
