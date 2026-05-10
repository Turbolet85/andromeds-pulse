import { useMemo, useState } from "react";
import type { LogRow } from "../../../bindings";
import { useStatusAnnouncer } from "../../StatusLiveRegion";
import {
  COLUMN_LABEL,
  DIRECTION_LABEL,
  SORT_STATE_NONE,
  type SortColumn,
  type SortState,
  nextSortState,
  sortRows,
} from "./sort";
import { severityNumberToTier, type SeverityTier } from "./use-log-filter";

interface LogTableProps {
  rows: readonly LogRow[];
  isLoading: boolean;
}

const COLUMNS: readonly { id: SortColumn; label: string }[] = [
  { id: "ts_unix_nano", label: "Timestamp" },
  { id: "severity_number", label: "Severity" },
  { id: "body", label: "Body" },
];

const TIER_TEXT_LABEL: Record<SeverityTier, string> = {
  error: "ERROR",
  warn: "WARN",
  info: "INFO",
  debug: "DEBUG",
};

const TIER_GLYPH: Record<SeverityTier, string> = {
  error: "✗",
  warn: "!",
  info: "i",
  debug: "·",
};

export function LogTable({ rows, isLoading }: LogTableProps) {
  const [sortState, setSortState] = useState<SortState>(SORT_STATE_NONE);
  const announce = useStatusAnnouncer();

  const sorted = useMemo(() => sortRows(rows, sortState), [rows, sortState]);

  const handleSort = (column: SortColumn): void => {
    setSortState((prev) => {
      const next = nextSortState(prev, column);
      if (next.direction === "none") {
        announce(`Cleared sort on ${COLUMN_LABEL[column]}`);
      } else {
        announce(`Sorted by ${COLUMN_LABEL[column]}, ${DIRECTION_LABEL[next.direction]}`);
      }
      return next;
    });
  };

  return (
    <div
      style={{
        background: "var(--color-raised-1)",
        border: "1px solid rgba(74, 144, 226, 0.3)",
        borderRadius: "var(--radius-md)",
        padding: "var(--spacing-md)",
      }}
      data-testid="log-table-card"
    >
      <table
        data-testid="log-table"
        style={{
          width: "100%",
          borderCollapse: "collapse",
          fontFamily: "var(--font-body)",
          fontSize: "12px",
        }}
      >
        <thead style={{ background: "var(--color-base)" }}>
          <tr>
            {COLUMNS.map((col) => (
              <th
                key={col.id}
                scope="col"
                aria-sort={ariaSortFor(sortState, col.id)}
                style={{
                  textAlign: "left",
                  padding: "var(--spacing-sm)",
                  color: "var(--color-text-primary)",
                  fontWeight: 600,
                  borderBottom: "1px solid rgba(74, 144, 226, 0.3)",
                }}
              >
                <button
                  type="button"
                  onClick={() => handleSort(col.id)}
                  style={{
                    background: "transparent",
                    border: 0,
                    color: "inherit",
                    font: "inherit",
                    fontWeight: "inherit",
                    cursor: "pointer",
                    padding: 0,
                  }}
                  data-testid={`sort-${col.id}`}
                >
                  {col.label}
                  <SortIndicator state={sortState} column={col.id} />
                </button>
              </th>
            ))}
          </tr>
        </thead>
        <tbody
          role="log"
          aria-live="polite"
          aria-relevant="additions"
          data-testid="log-stream"
        >
          {isLoading && rows.length === 0 ? (
            <tr>
              <td
                colSpan={COLUMNS.length}
                style={{
                  padding: "var(--spacing-md)",
                  color: "var(--color-text-tertiary)",
                  textAlign: "center",
                }}
                data-testid="log-table-loading"
              >
                Loading…
              </td>
            </tr>
          ) : sorted.length === 0 ? (
            <tr>
              <td
                colSpan={COLUMNS.length}
                style={{
                  padding: "var(--spacing-md)",
                  color: "var(--color-text-tertiary)",
                  textAlign: "center",
                }}
                data-testid="log-table-empty"
              >
                No logs match filter
              </td>
            </tr>
          ) : (
            sorted.map((row, idx) => (
              <LogRowView
                key={`${row.ts_unix_nano}-${row.severity_number}-${idx}`}
                row={row}
              />
            ))
          )}
        </tbody>
      </table>
    </div>
  );
}

function ariaSortFor(state: SortState, column: SortColumn): "ascending" | "descending" | "none" {
  if (state.column !== column || state.direction === "none") return "none";
  return state.direction === "asc" ? "ascending" : "descending";
}

function SortIndicator({ state, column }: { state: SortState; column: SortColumn }) {
  if (state.column !== column || state.direction === "none") {
    return null;
  }
  return (
    <span
      aria-hidden="true"
      style={{ marginLeft: 4, color: "var(--color-primary)" }}
      data-testid={`sort-indicator-${column}`}
    >
      {state.direction === "asc" ? "▲" : "▼"}
    </span>
  );
}

function LogRowView({ row }: { row: LogRow }) {
  const tier = severityNumberToTier(row.severity_number);
  const severityLabel = row.severity_text || TIER_TEXT_LABEL[tier];
  return (
    <tr
      data-testid="log-row"
      data-severity={tier}
      style={{
        borderBottom: "1px solid rgba(74, 144, 226, 0.1)",
      }}
    >
      <td
        style={{
          padding: "var(--spacing-sm)",
          fontFamily: "var(--font-code)",
          fontVariantNumeric: "tabular-nums",
          color: "var(--color-text-primary)",
        }}
      >
        {formatNs(row.ts_unix_nano)}
      </td>
      <td
        style={{
          padding: "var(--spacing-sm)",
          color: "var(--color-text-primary)",
          borderLeft: tierBorderLeft(tier),
        }}
        data-testid="log-severity-cell"
      >
        <span aria-hidden="true" style={{ marginRight: 4, color: tierForeground(tier) }}>
          {TIER_GLYPH[tier]}
        </span>
        <span style={{ color: tierForeground(tier) }}>{severityLabel}</span>
      </td>
      <td
        style={{
          padding: "var(--spacing-sm)",
          color: "var(--color-text-primary)",
          fontFamily: "var(--font-code)",
        }}
      >
        {row.body || <span style={{ color: "var(--color-text-tertiary)" }}>(empty)</span>}
        {row.trace_id && (
          <span
            data-testid="log-trace-correlation"
            title={`trace_id: ${row.trace_id}`}
            style={{
              marginLeft: 8,
              fontSize: "11px",
              color: "var(--color-text-tertiary)",
            }}
          >
            [{row.trace_id.slice(0, 8)}…]
          </span>
        )}
      </td>
    </tr>
  );
}

function tierBorderLeft(tier: SeverityTier): string {
  switch (tier) {
    case "error":
    case "warn":
      return "3px solid var(--color-accent)";
    case "info":
      return "3px solid var(--color-primary)";
    case "debug":
      return "3px solid var(--color-text-tertiary)";
  }
}

function tierForeground(tier: SeverityTier): string {
  switch (tier) {
    case "error":
      return "var(--color-accent)";
    case "warn":
      return "var(--color-text-primary)";
    case "info":
      return "var(--color-primary)";
    case "debug":
      return "var(--color-text-tertiary)";
  }
}

function formatNs(ts_unix_nano: number): string {
  // Truncate to ms-resolution display; full nanosecond precision retained on
  // backend. Format: HH:MM:SS.mmm
  const ms = Math.floor(ts_unix_nano / 1_000_000);
  const date = new Date(ms);
  const h = String(date.getUTCHours()).padStart(2, "0");
  const m = String(date.getUTCMinutes()).padStart(2, "0");
  const s = String(date.getUTCSeconds()).padStart(2, "0");
  const millis = String(date.getUTCMilliseconds()).padStart(3, "0");
  return `${h}:${m}:${s}.${millis}`;
}
