// Real-data service-registry hook for the compact-widget constellation
// (chunk #91). Polls the existing `services.list_with_states()` TauRPC
// (chunk #67 substrate at `pulse-app/src/services_router.rs`) on mount, on a
// periodic interval, and on window focus, exposing the latest
// `ServiceListItem[]` for ConstellationCanvas. First webview consumer of the
// services surface.
//
// PULL-only posture mirrors `use-connection-state.ts` (chunk #89) +
// `use-findings.ts` (chunk #87): the `pulse://stream/service-lifecycle`
// broadcast is emit-only with no webview Channel subscriber wired yet, so a
// future chunk can add push-driven invalidation without changing this hook's
// public shape. A 1s poll keeps the constellation fresh enough for an
// ambient glance surface.
//
// jsdom posture mirrors `use-widget-metrics.ts` (chunk #57): every proxy
// invocation is wrapped in `.catch(() => {})` so jsdom test environments +
// pre-init Tauri context fall back to the last value (or empty) silently.

import { useEffect, useState } from "react";
import { createTauRPCProxy, type ServiceListItem } from "../bindings/index";

const POLL_INTERVAL_MS = 1000;

const EMPTY: ServiceListItem[] = [];

export function useServiceConstellation(): ServiceListItem[] {
  const [items, setItems] = useState<ServiceListItem[]>(EMPTY);

  useEffect(() => {
    const proxy = createTauRPCProxy();

    const poll = () => {
      proxy.services
        .list_with_states()
        .then((payload) => {
          setItems(payload.items);
        })
        .catch(() => {
          // Silent: jsdom / pre-init Tauri context. Items stay at last value.
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

  return items;
}
