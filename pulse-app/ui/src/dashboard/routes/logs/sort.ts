import type { LogRow } from "../../../bindings";

export type SortColumn = "ts_unix_nano" | "severity_number" | "body";
export type SortDirection = "asc" | "desc" | "none";

export const SORT_COLUMNS: readonly SortColumn[] = [
  "ts_unix_nano",
  "severity_number",
  "body",
];

export interface SortState {
  column: SortColumn;
  direction: SortDirection;
}

export const SORT_STATE_NONE: SortState = {
  column: "ts_unix_nano",
  direction: "none",
};

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

export function sortRows(rows: readonly LogRow[], state: SortState): LogRow[] {
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

function compareByColumn(a: LogRow, b: LogRow, column: SortColumn): number {
  switch (column) {
    case "ts_unix_nano":
      return a.ts_unix_nano - b.ts_unix_nano;
    case "severity_number":
      return a.severity_number - b.severity_number;
    case "body":
      return a.body.localeCompare(b.body);
  }
}

export const COLUMN_LABEL: Record<SortColumn, string> = {
  ts_unix_nano: "Timestamp",
  severity_number: "Severity",
  body: "Body",
};

export const DIRECTION_LABEL: Record<Exclude<SortDirection, "none">, string> = {
  asc: "ascending",
  desc: "descending",
};
