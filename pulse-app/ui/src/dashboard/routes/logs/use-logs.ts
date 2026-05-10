import { useEffect, useState } from "react";
import {
  createTauRPCProxy,
  type AppError,
  type LogRow,
  type LogsQueryArgs,
  type PaginatedResponse,
} from "../../../bindings";

interface UseLogsState {
  rows: LogRow[];
  total: number;
  isLoading: boolean;
  error: AppError | null;
}

const INITIAL_STATE: UseLogsState = {
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

export interface UseLogsOptions {
  timeWindowSeconds: number;
  limit: number;
}

export function useLogs(options: UseLogsOptions): UseLogsState {
  const [state, setState] = useState<UseLogsState>(INITIAL_STATE);

  useEffect(() => {
    let cancelled = false;
    setState((prev) => ({ ...prev, isLoading: true, error: null }));

    const args: LogsQueryArgs = {
      time_window_seconds: options.timeWindowSeconds,
      limit: options.limit,
      cursor: null,
    };

    void getClient()
      .logs.query(args)
      .then((response: PaginatedResponse<LogRow>) => {
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
  return { kind: "internal", message: "logs.query failed" };
}
