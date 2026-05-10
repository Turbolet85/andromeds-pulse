import { useMemo, useState } from "react";
import type { LogRow } from "../../../bindings";

// 4-tier severity binning per design extract:
//   ERROR/FATAL: severity_number >= 17
//   WARN:        severity_number 13..=16
//   INFO:        severity_number 9..=12
//   DEBUG/TRACE: severity_number <= 8
export type SeverityTier = "error" | "warn" | "info" | "debug";

export const ALL_SEVERITY_TIERS: readonly SeverityTier[] = [
  "error",
  "warn",
  "info",
  "debug",
];

export function severityNumberToTier(n: number): SeverityTier {
  if (n >= 17) return "error";
  if (n >= 13) return "warn";
  if (n >= 9) return "info";
  return "debug";
}

export interface LogFilterState {
  searchQuery: string;
  enabledTiers: ReadonlySet<SeverityTier>;
}

export const INITIAL_FILTER_STATE: LogFilterState = {
  searchQuery: "",
  enabledTiers: new Set(ALL_SEVERITY_TIERS),
};

export interface UseLogFilterResult {
  state: LogFilterState;
  filteredRows: LogRow[];
  setSearchQuery: (query: string) => void;
  toggleTier: (tier: SeverityTier) => void;
  clear: () => void;
}

export function useLogFilter(rows: readonly LogRow[]): UseLogFilterResult {
  const [state, setState] = useState<LogFilterState>(INITIAL_FILTER_STATE);

  const filteredRows = useMemo(
    () => filterRows(rows, state),
    [rows, state],
  );

  return {
    state,
    filteredRows,
    setSearchQuery: (query: string) =>
      setState((prev) => ({ ...prev, searchQuery: query })),
    toggleTier: (tier: SeverityTier) =>
      setState((prev) => {
        const next = new Set(prev.enabledTiers);
        if (next.has(tier)) {
          next.delete(tier);
        } else {
          next.add(tier);
        }
        return { ...prev, enabledTiers: next };
      }),
    clear: () => setState(INITIAL_FILTER_STATE),
  };
}

export function filterRows(
  rows: readonly LogRow[],
  state: LogFilterState,
): LogRow[] {
  const needle = state.searchQuery.toLowerCase();
  return rows.filter((row) => {
    const tier = severityNumberToTier(row.severity_number);
    if (!state.enabledTiers.has(tier)) return false;
    if (needle === "") return true;
    return row.body.toLowerCase().includes(needle);
  });
}
