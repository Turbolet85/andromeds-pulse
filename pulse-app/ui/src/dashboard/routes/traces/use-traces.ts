import { useEffect, useState } from "react";
import {
  createTauRPCProxy,
  type AppError,
  type PaginatedResponse,
  type TraceRow,
  type TracesQueryArgs,
} from "../../../bindings";

interface UseTracesState {
  rows: TraceRow[];
  total: number;
  isLoading: boolean;
  error: AppError | null;
}

const INITIAL_STATE: UseTracesState = {
  rows: [],
  total: 0,
  isLoading: true,
  error: null,
};

let cachedClient: ReturnType<typeof createTauRPCProxy> | null = null;

function getClient(): ReturnType<typeof createTauRPCProxy> {
  if (cachedClient === null) {
    cachedClient = createTauRPCProxy();
  }
  return cachedClient;
}

// Test-only seam mirroring canvas/frame-metrics.ts __setProxyForTest pattern.
export function __setProxyForTest(
  proxy: ReturnType<typeof createTauRPCProxy> | null,
): void {
  cachedClient = proxy;
}

export interface UseTracesOptions {
  timeWindowSeconds: number;
  limit: number;
}

const POLL_INTERVAL_MS = 1000;

export function useTraces(options: UseTracesOptions): UseTracesState {
  const [state, setState] = useState<UseTracesState>(INITIAL_STATE);

  useEffect(() => {
    let cancelled = false;
    setState((prev) => ({ ...prev, isLoading: true, error: null }));

    const args: TracesQueryArgs = {
      time_window_seconds: options.timeWindowSeconds,
      limit: options.limit,
      cursor: null,
    };

    const poll = (): void => {
      void getClient()
        .traces.query(args)
        .then((response: PaginatedResponse<TraceRow>) => {
          if (cancelled) return;
          setState({
            rows: response.items,
            total: response.total,
            isLoading: false,
            error: null,
          });
        })
        .catch((err: unknown) => {
          if (cancelled) return;
          // First fetch (still loading) surfaces the error + empty; a later
          // re-poll hiccup after data has shown keeps last-good rows (silent
          // background refresh, mirroring use-service-constellation.ts).
          setState((prev) =>
            prev.isLoading
              ? { rows: [], total: 0, isLoading: false, error: toAppError(err) }
              : prev,
          );
        });
    };

    poll();
    const interval = window.setInterval(poll, POLL_INTERVAL_MS);
    const handleFocus = (): void => {
      poll();
    };
    window.addEventListener("focus", handleFocus);

    return () => {
      cancelled = true;
      window.clearInterval(interval);
      window.removeEventListener("focus", handleFocus);
    };
  }, [options.timeWindowSeconds, options.limit]);

  return state;
}

function toAppError(err: unknown): AppError {
  if (
    err !== null &&
    typeof err === "object" &&
    "kind" in err &&
    typeof (err as { kind: unknown }).kind === "string"
  ) {
    return err as AppError;
  }
  return { kind: "internal", message: "traces.query failed" };
}
