import { useEffect, useState } from "react";
import {
  createTauRPCProxy,
  type AppError,
  type MetricRow,
  type MetricsQueryArgs,
  type PaginatedResponse,
} from "../../../bindings";

interface UseMetricsState {
  rows: MetricRow[];
  total: number;
  isLoading: boolean;
  error: AppError | null;
}

const INITIAL_STATE: UseMetricsState = {
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

export function __setProxyForTest(
  proxy: ReturnType<typeof createTauRPCProxy> | null,
): void {
  cachedClient = proxy;
}

export interface UseMetricsOptions {
  timeWindowSeconds: number;
  limit: number;
}

export function useMetrics(options: UseMetricsOptions): UseMetricsState {
  const [state, setState] = useState<UseMetricsState>(INITIAL_STATE);

  useEffect(() => {
    let cancelled = false;
    setState((prev) => ({ ...prev, isLoading: true, error: null }));

    const args: MetricsQueryArgs = {
      time_window_seconds: options.timeWindowSeconds,
      limit: options.limit,
      cursor: null,
    };

    void getClient()
      .metrics.query(args)
      .then((response: PaginatedResponse<MetricRow>) => {
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
        setState({
          rows: [],
          total: 0,
          isLoading: false,
          error: toAppError(err),
        });
      });

    return () => {
      cancelled = true;
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
  return { kind: "internal", message: "metrics.query failed" };
}
