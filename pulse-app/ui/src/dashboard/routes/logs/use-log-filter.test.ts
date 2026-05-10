import { describe, expect, it } from "vitest";
import { act, renderHook } from "@testing-library/react";
import type { LogRow } from "../../../bindings";
import {
  filterRows,
  INITIAL_FILTER_STATE,
  severityNumberToTier,
  useLogFilter,
} from "./use-log-filter";

const row = (severity: number, body: string): LogRow => ({
  ts_unix_nano: 0,
  resource_hash: "",
  severity_number: severity,
  body,
  severity_text: "",
  trace_id: "",
  span_id: "",
});

describe("severityNumberToTier", () => {
  it("maps OTLP severity_number ranges to 4 tiers", () => {
    expect(severityNumberToTier(1)).toBe("debug"); // TRACE
    expect(severityNumberToTier(5)).toBe("debug"); // DEBUG
    expect(severityNumberToTier(9)).toBe("info");
    expect(severityNumberToTier(12)).toBe("info");
    expect(severityNumberToTier(13)).toBe("warn");
    expect(severityNumberToTier(16)).toBe("warn");
    expect(severityNumberToTier(17)).toBe("error");
    expect(severityNumberToTier(24)).toBe("error"); // FATAL
  });
});

describe("filterRows", () => {
  it("returns all rows when no search and all tiers enabled", () => {
    const rows = [row(9, "hello"), row(17, "boom")];
    expect(filterRows(rows, INITIAL_FILTER_STATE)).toEqual(rows);
  });

  it("filters by case-insensitive substring match on body", () => {
    const rows = [row(9, "Hello world"), row(9, "goodbye")];
    expect(
      filterRows(rows, { ...INITIAL_FILTER_STATE, searchQuery: "hello" }),
    ).toHaveLength(1);
  });

  it("filters out rows whose tier is not enabled", () => {
    const rows = [row(9, "info-row"), row(17, "error-row")];
    const state = {
      searchQuery: "",
      enabledTiers: new Set(["info" as const]),
    };
    expect(filterRows(rows, state)).toHaveLength(1);
    expect(filterRows(rows, state)[0].body).toBe("info-row");
  });
});

describe("useLogFilter", () => {
  it("starts with all tiers enabled and empty search", () => {
    const rows = [row(9, "x")];
    const { result } = renderHook(() => useLogFilter(rows));
    expect(result.current.state.searchQuery).toBe("");
    expect(result.current.state.enabledTiers.size).toBe(4);
    expect(result.current.filteredRows).toHaveLength(1);
  });

  it("setSearchQuery filters rows on subsequent render", () => {
    const rows = [row(9, "boom"), row(9, "ok")];
    const { result } = renderHook(() => useLogFilter(rows));
    act(() => {
      result.current.setSearchQuery("boom");
    });
    expect(result.current.filteredRows).toHaveLength(1);
    expect(result.current.filteredRows[0].body).toBe("boom");
  });

  it("toggleTier removes tier on first call and re-adds on second", () => {
    const rows = [row(9, "info"), row(17, "err")];
    const { result } = renderHook(() => useLogFilter(rows));
    act(() => result.current.toggleTier("info"));
    expect(result.current.filteredRows).toHaveLength(1);
    expect(result.current.filteredRows[0].body).toBe("err");
    act(() => result.current.toggleTier("info"));
    expect(result.current.filteredRows).toHaveLength(2);
  });

  it("clear() restores INITIAL_FILTER_STATE", () => {
    const rows = [row(9, "x")];
    const { result } = renderHook(() => useLogFilter(rows));
    act(() => result.current.setSearchQuery("zzz"));
    expect(result.current.filteredRows).toHaveLength(0);
    act(() => result.current.clear());
    expect(result.current.state.searchQuery).toBe("");
    expect(result.current.filteredRows).toHaveLength(1);
  });
});
