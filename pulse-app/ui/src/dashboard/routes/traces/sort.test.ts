import { describe, expect, it } from "vitest";
import type { TraceRow } from "../../../bindings";
import {
  COLUMN_LABEL,
  DIRECTION_LABEL,
  SORT_STATE_NONE,
  nextSortState,
  sortRows,
} from "./sort";

function row(overrides: Partial<TraceRow>): TraceRow {
  return {
    trace_id: "00000000",
    span_id: "00000000",
    ts_unix_nano: 0,
    service: "default",
    duration_ms: 0,
    error_count: 0,
    ...overrides,
  };
}

describe("nextSortState", () => {
  it("starts a new column at ascending", () => {
    const next = nextSortState(SORT_STATE_NONE, "service");
    expect(next).toEqual({ column: "service", direction: "asc" });
  });

  it("cycles asc → desc → none → asc on the same column", () => {
    let state = nextSortState(SORT_STATE_NONE, "duration_ms");
    expect(state).toEqual({ column: "duration_ms", direction: "asc" });
    state = nextSortState(state, "duration_ms");
    expect(state).toEqual({ column: "duration_ms", direction: "desc" });
    state = nextSortState(state, "duration_ms");
    expect(state).toEqual({ column: "duration_ms", direction: "none" });
    state = nextSortState(state, "duration_ms");
    expect(state).toEqual({ column: "duration_ms", direction: "asc" });
  });

  it("switching column resets to ascending", () => {
    const desc = { column: "trace_id", direction: "desc" } as const;
    const next = nextSortState(desc, "service");
    expect(next).toEqual({ column: "service", direction: "asc" });
  });
});

describe("sortRows", () => {
  const rows: TraceRow[] = [
    row({ trace_id: "c", service: "z", duration_ms: 50, error_count: 0 }),
    row({ trace_id: "a", service: "y", duration_ms: 200, error_count: 1 }),
    row({ trace_id: "b", service: "x", duration_ms: 100, error_count: 0 }),
  ];

  it("hoists erroring rows first when direction = none (anomaly-first baseline)", () => {
    const sorted = sortRows(rows, SORT_STATE_NONE);
    // 'a' is the only erroring row (error_count 1) → surfaced first; the healthy
    // rows keep query order (c before b). Intent F8 / P-068.
    expect(sorted.map((r) => r.trace_id)).toEqual(["a", "c", "b"]);
    // Returns a copy, not a mutation.
    expect(sorted).not.toBe(rows);
  });

  it("preserves query order within the erroring and healthy groups (stable)", () => {
    const mixed: TraceRow[] = [
      row({ trace_id: "h1", error_count: 0 }),
      row({ trace_id: "e1", error_count: 1 }),
      row({ trace_id: "h2", error_count: 0 }),
      row({ trace_id: "e2", error_count: 2 }),
    ];
    const sorted = sortRows(mixed, SORT_STATE_NONE);
    // Erroring group first (e1 before e2), then healthy group (h1 before h2).
    expect(sorted.map((r) => r.trace_id)).toEqual(["e1", "e2", "h1", "h2"]);
  });

  it("sorts asc by trace_id", () => {
    const sorted = sortRows(rows, { column: "trace_id", direction: "asc" });
    expect(sorted.map((r) => r.trace_id)).toEqual(["a", "b", "c"]);
  });

  it("sorts desc by service", () => {
    const sorted = sortRows(rows, { column: "service", direction: "desc" });
    expect(sorted.map((r) => r.service)).toEqual(["z", "y", "x"]);
  });

  it("sorts asc by duration_ms numerically (not lexicographically)", () => {
    const sorted = sortRows(rows, { column: "duration_ms", direction: "asc" });
    expect(sorted.map((r) => r.duration_ms)).toEqual([50, 100, 200]);
  });

  it("sorts asc by error_count (zeros first)", () => {
    const sorted = sortRows(rows, { column: "error_count", direction: "asc" });
    expect(sorted.map((r) => r.error_count)).toEqual([0, 0, 1]);
  });
});

describe("label tables", () => {
  it("COLUMN_LABEL has every column", () => {
    expect(COLUMN_LABEL.trace_id).toBe("Trace ID");
    expect(COLUMN_LABEL.service).toBe("Service");
    expect(COLUMN_LABEL.duration_ms).toBe("Latency");
    expect(COLUMN_LABEL.error_count).toBe("Error");
  });

  it("DIRECTION_LABEL covers asc + desc", () => {
    expect(DIRECTION_LABEL.asc).toBe("ascending");
    expect(DIRECTION_LABEL.desc).toBe("descending");
  });
});
