import { describe, expect, it } from "vitest";
import type { LogRow } from "../../../bindings";
import {
  COLUMN_LABEL,
  DIRECTION_LABEL,
  SORT_STATE_NONE,
  nextSortState,
  sortRows,
  type SortState,
} from "./sort";

const row = (ts_ns: number, severity: number, body: string): LogRow => ({
  ts_unix_nano: ts_ns,
  resource_hash: "",
  severity_number: severity,
  body,
  severity_text: "",
  trace_id: "",
  span_id: "",
});

describe("nextSortState", () => {
  it("clicking different column resets to asc", () => {
    const next = nextSortState(SORT_STATE_NONE, "severity_number");
    expect(next).toEqual({ column: "severity_number", direction: "asc" });
  });

  it("none → asc on same column", () => {
    expect(nextSortState({ column: "ts_unix_nano", direction: "none" }, "ts_unix_nano"))
      .toEqual({ column: "ts_unix_nano", direction: "asc" });
  });

  it("asc → desc on same column", () => {
    expect(nextSortState({ column: "ts_unix_nano", direction: "asc" }, "ts_unix_nano"))
      .toEqual({ column: "ts_unix_nano", direction: "desc" });
  });

  it("desc → none on same column", () => {
    expect(nextSortState({ column: "ts_unix_nano", direction: "desc" }, "ts_unix_nano"))
      .toEqual({ column: "ts_unix_nano", direction: "none" });
  });
});

describe("sortRows", () => {
  const rows = [
    row(2, 17, "zebra"),
    row(1, 9, "alpha"),
    row(3, 13, "mango"),
  ];

  it("returns shallow copy when direction is none (preserves order)", () => {
    const out = sortRows(rows, { column: "ts_unix_nano", direction: "none" });
    expect(out).toEqual(rows);
    expect(out).not.toBe(rows);
  });

  it("sorts ascending by ts_unix_nano", () => {
    const state: SortState = { column: "ts_unix_nano", direction: "asc" };
    const out = sortRows(rows, state);
    expect(out.map((r) => r.ts_unix_nano)).toEqual([1, 2, 3]);
  });

  it("sorts descending by ts_unix_nano", () => {
    const state: SortState = { column: "ts_unix_nano", direction: "desc" };
    const out = sortRows(rows, state);
    expect(out.map((r) => r.ts_unix_nano)).toEqual([3, 2, 1]);
  });

  it("sorts by severity_number numerically", () => {
    const state: SortState = { column: "severity_number", direction: "asc" };
    const out = sortRows(rows, state);
    expect(out.map((r) => r.severity_number)).toEqual([9, 13, 17]);
  });

  it("sorts by body using locale compare", () => {
    const state: SortState = { column: "body", direction: "asc" };
    const out = sortRows(rows, state);
    expect(out.map((r) => r.body)).toEqual(["alpha", "mango", "zebra"]);
  });
});

describe("labels", () => {
  it("exposes human-readable column labels", () => {
    expect(COLUMN_LABEL.ts_unix_nano).toBe("Timestamp");
    expect(COLUMN_LABEL.severity_number).toBe("Severity");
    expect(COLUMN_LABEL.body).toBe("Body");
  });

  it("exposes direction labels", () => {
    expect(DIRECTION_LABEL.asc).toBe("ascending");
    expect(DIRECTION_LABEL.desc).toBe("descending");
  });
});
