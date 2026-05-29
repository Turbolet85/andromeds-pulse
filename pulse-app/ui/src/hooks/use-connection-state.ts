// Real-data connection-state hook for the compact-widget titlebar dot
// (chunk #89). Polls the existing `connection.current_state()` TauRPC
// (chunk #59 substrate at `pulse-app/src/connection_router.rs`) on mount,
// on a periodic interval, and on window focus, exposing the latest
// `ConnectionStatePayload` for the ConnectionDot. First webview consumer
// of the connection-state surface.
//
// PULL-only posture mirrors `use-findings.ts` (chunk #87): the
// `pulse://stream/connection-state` broadcast is emit-only with no webview
// Channel subscriber wired yet, so a future chunk can add push-driven
// invalidation without changing this hook's public shape. A 1s poll keeps
// the state + last-span-ago lag fresh enough for an ambient indicator.
//
// jsdom posture mirrors `use-widget-metrics.ts` (chunk #57): every proxy
// invocation is wrapped in `.catch(() => {})` so jsdom test environments +
// pre-init Tauri context fall back to the last value (or null) silently —
// the dot renders a neutral default, no console noise.

import { useEffect, useState } from "react";
import { createTauRPCProxy, type ConnectionStatePayload } from "../bindings/index";

const POLL_INTERVAL_MS = 1000;

export function useConnectionState(): ConnectionStatePayload | null {
  const [state, setState] = useState<ConnectionStatePayload | null>(null);

  useEffect(() => {
    const proxy = createTauRPCProxy();

    const poll = () => {
      proxy.connection
        .current_state()
        .then((payload) => {
          setState(payload);
        })
        .catch(() => {
          // Silent: jsdom / pre-init Tauri context. State stays at last value.
        });
    };

    poll();
    const interval = window.setInterval(poll, POLL_INTERVAL_MS);
    const handleFocus = () => {
      poll();
    };
    window.addEventListener("focus", handleFocus);

    return () => {
      window.clearInterval(interval);
      window.removeEventListener("focus", handleFocus);
    };
  }, []);

  return state;
}
