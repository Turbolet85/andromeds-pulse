import type { TraceRow } from "../../../bindings";

// Bounded enums per security plan §Input Validation: sort UI flows through
// typed values, never free-form strings. Order matches table column order.
export type SortColumn = "trace_id" | "service" | "duration_ms" | "error_count";
export type SortDirection = "asc" | "desc" | "none";

export const SORT_COLUMNS: readonly SortColumn[] = [
  "trace_id",
  "service",
  "duration_ms",
  "error_count",
];

export interface SortState {
  column: SortColumn;
  direction: SortDirection;
}

export const SORT_STATE_NONE: SortState = { column: "trace_id", direction: "none" };

// Click cycle per layouts §Component — Trace data table Sorting:
// none → asc → desc → none. New column resets to asc.
export function nextSortState(current: SortState, clicked: SortColumn): SortState {
  if (current.column !== clicked) {
    return { column: clicked, direction: "asc" };
  }
  switch (current.direction) {
    case "none":
      return { column: clicked, direction: "asc" };
    case "asc":
      return { column: clicked, direction: "desc" };
    case "desc":
      return { column: clicked, direction: "none" };
  }
}

export function sortRows(rows: readonly TraceRow[], state: SortState): TraceRow[] {
  if (state.direction === "none") {
    return rows.slice();
  }
  const copy = rows.slice();
  copy.sort((a, b) => compareByColumn(a, b, state.column));
  if (state.direction === "desc") {
    copy.reverse();
  }
  return copy;
}

function compareByColumn(a: TraceRow, b: TraceRow, column: SortColumn): number {
  switch (column) {
    case "trace_id":
      return a.trace_id.localeCompare(b.trace_id);
    case "service":
      return a.service.localeCompare(b.service);
    case "duration_ms":
      return a.duration_ms - b.duration_ms;
    case "error_count":
      return a.error_count - b.error_count;
  }
}

// Human-readable column labels for the announcement region (a11y SC 4.1.3).
export const COLUMN_LABEL: Record<SortColumn, string> = {
  trace_id: "Trace ID",
  service: "Service",
  duration_ms: "Latency",
  error_count: "Error",
};

// Human-readable direction labels for the announcement region.
export const DIRECTION_LABEL: Record<Exclude<SortDirection, "none">, string> = {
  asc: "ascending",
  desc: "descending",
};
