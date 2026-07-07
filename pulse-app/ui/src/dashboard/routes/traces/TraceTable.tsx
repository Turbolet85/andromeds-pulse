import { useEffect, useMemo, useRef, useState } from "react";
import type { TraceRow } from "../../../bindings";
import { InvestigateButton } from "../../../components/InvestigateButton";
import { useInvestigation } from "../../../hooks/use-investigation";
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

interface TraceTableProps {
  rows: readonly TraceRow[];
  isLoading: boolean;
}

const COLUMNS: readonly { id: SortColumn; label: string }[] = [
  { id: "trace_id", label: "Trace ID" },
  { id: "service", label: "Service" },
  { id: "duration_ms", label: "Latency" },
  { id: "error_count", label: "Error" },
];

export function TraceTable({ rows, isLoading }: TraceTableProps) {
  const [sortState, setSortState] = useState<SortState>(SORT_STATE_NONE);
  const [errorsOnly, setErrorsOnly] = useState(false);
  const announce = useStatusAnnouncer();
  const { openInvestigation } = useInvestigation();
  const prevRowCount = useRef(rows.length);

  useEffect(() => {
    if (prevRowCount.current === 0 && rows.length > 0) {
      announce("Traces loaded");
    }
    prevRowCount.current = rows.length;
  }, [rows.length, announce]);

  const sorted = useMemo(() => {
    const filtered = errorsOnly ? rows.filter((r) => r.error_count > 0) : rows;
    return sortRows(filtered, sortState);
  }, [rows, sortState, errorsOnly]);

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

  const handleToggleErrorsOnly = (): void => {
    const next = !errorsOnly;
    announce(next ? "Showing errors only" : "Showing all traces");
    setErrorsOnly(next);
  };

  return (
    <div
      style={{
        background: "var(--color-raised-1)",
        border: "1px solid rgba(74, 144, 226, 0.3)",
        borderRadius: "var(--radius-md)",
        padding: "var(--spacing-md)",
      }}
      data-testid="trace-table-card"
    >
      <div
        style={{
          display: "flex",
          justifyContent: "flex-end",
          marginBottom: "var(--spacing-sm)",
        }}
        data-testid="trace-table-toolbar"
      >
        <button
          type="button"
          aria-pressed={errorsOnly}
          onClick={handleToggleErrorsOnly}
          data-testid="trace-errors-only-filter"
          style={{
            background: errorsOnly ? "var(--color-raised-2)" : "var(--color-inset)",
            border: errorsOnly ? "1px solid #4A90E2" : "1px solid rgba(74, 144, 226, 0.3)",
            borderRadius: "var(--radius-sm)",
            padding: "var(--spacing-xs) var(--spacing-sm)",
            color: "var(--color-text-primary)",
            fontFamily: "var(--font-body)",
            fontSize: "12px",
            cursor: "pointer",
            opacity: errorsOnly ? 1 : 0.6,
          }}
        >
          Errors only
        </button>
      </div>
      <table
        data-testid="trace-table"
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
                  textAlign: col.id === "duration_ms" ? "right" : "left",
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
        <tbody>
          {isLoading && rows.length === 0 ? (
            <tr>
              <td
                colSpan={COLUMNS.length}
                style={{
                  padding: "var(--spacing-md)",
                  // text-secondary, not tertiary: 12px body text needs 4.5:1
                  // (tertiary is large-text-only; chunk #99 axe finding 3.92:1
                  // on raised-1)
                  color: "var(--color-text-secondary)",
                  textAlign: "center",
                }}
                data-testid="trace-table-loading"
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
                  // text-secondary, not tertiary (12px body text; see loading
                  // cell note)
                  color: "var(--color-text-secondary)",
                  textAlign: "center",
                }}
                data-testid="trace-table-empty"
              >
                No traces yet
              </td>
            </tr>
          ) : (
            sorted.map((row) => (
              <TraceRowView
                key={`${row.trace_id}-${row.span_id}`}
                row={row}
                onInvestigate={openInvestigation}
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

function TraceRowView({
  row,
  onInvestigate,
}: {
  row: TraceRow;
  onInvestigate: (trigger: HTMLElement | null) => void;
}) {
  const isError = row.error_count > 0;
  const traceLabel = truncateHex(row.trace_id);
  return (
    <tr
      data-testid="trace-row"
      onContextMenu={(event) => {
        event.preventDefault();
        onInvestigate(event.currentTarget);
      }}
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
        {traceLabel}
      </td>
      <td
        style={{
          padding: "var(--spacing-sm)",
          color: "var(--color-text-primary)",
        }}
      >
        {row.service === "" ? "(unknown)" : row.service}
      </td>
      <td
        style={{
          padding: "var(--spacing-sm)",
          fontFamily: "var(--font-code)",
          fontVariantNumeric: "tabular-nums",
          color: "var(--color-text-primary)",
          textAlign: "right",
        }}
      >
        {row.duration_ms} ms
      </td>
      <td
        style={{
          padding: "var(--spacing-sm)",
          color: "var(--color-text-primary)",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: "var(--spacing-sm)",
          ...(isError
            ? {
                borderLeft: "3px solid var(--color-accent)",
              }
            : {}),
        }}
        data-testid={isError ? "trace-row-error" : "trace-row-ok"}
      >
        {isError ? (
          <span data-testid="trace-error-cell">
            <span aria-hidden="true" style={{ marginRight: 4 }}>
              ✗
            </span>
            {row.error_count}
          </span>
        ) : (
          <span style={{ color: "var(--color-text-tertiary)" }}>0</span>
        )}
        <InvestigateButton
          variant="trace-row-inline"
          aria-label={`Investigate trace ${traceLabel}`}
          onClick={(event) => {
            event.stopPropagation();
            onInvestigate(event.currentTarget);
          }}
          data-testid="trace-row-investigate"
        />
      </td>
    </tr>
  );
}

function truncateHex(hex: string): string {
  if (hex.length <= 16) return hex;
  return `${hex.slice(0, 8)}…${hex.slice(-4)}`;
}
