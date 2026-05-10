import { afterEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import type { LogRow } from "../../../bindings";
import { LogTable } from "./LogTable";

vi.mock("../../StatusLiveRegion", () => ({
  useStatusAnnouncer: () => vi.fn(),
}));

afterEach(() => {
  vi.clearAllMocks();
});

const row = (severity: number, severityText: string, body: string, traceId = ""): LogRow => ({
  ts_unix_nano: 1_700_000_000_000,
  resource_hash: "abc",
  severity_number: severity,
  body,
  severity_text: severityText,
  trace_id: traceId,
  span_id: "",
});

describe("LogTable", () => {
  it("renders semantic <table> with three sortable column headers", () => {
    render(<LogTable rows={[row(9, "INFO", "x")]} isLoading={false} />);
    const table = screen.getByTestId("log-table");
    expect(table.tagName).toBe("TABLE");
    expect(screen.getByTestId("sort-ts_unix_nano")).toBeDefined();
    expect(screen.getByTestId("sort-severity_number")).toBeDefined();
    expect(screen.getByTestId("sort-body")).toBeDefined();
  });

  it("each column header has scope='col' + aria-sort attribute", () => {
    render(<LogTable rows={[]} isLoading={false} />);
    const headers = screen.getAllByRole("columnheader");
    expect(headers).toHaveLength(3);
    for (const header of headers) {
      expect(header.getAttribute("scope")).toBe("col");
      expect(header.getAttribute("aria-sort")).toBe("none");
    }
  });

  it("clicking a header cycles aria-sort none → asc → desc → none", () => {
    render(<LogTable rows={[row(9, "INFO", "x")]} isLoading={false} />);
    const tsHeader = screen.getAllByRole("columnheader")[0];
    expect(tsHeader.getAttribute("aria-sort")).toBe("none");
    fireEvent.click(screen.getByTestId("sort-ts_unix_nano"));
    expect(tsHeader.getAttribute("aria-sort")).toBe("ascending");
    fireEvent.click(screen.getByTestId("sort-ts_unix_nano"));
    expect(tsHeader.getAttribute("aria-sort")).toBe("descending");
    fireEvent.click(screen.getByTestId("sort-ts_unix_nano"));
    expect(tsHeader.getAttribute("aria-sort")).toBe("none");
  });

  it("renders log stream tbody with role='log' aria-live='polite'", () => {
    render(<LogTable rows={[row(9, "INFO", "x")]} isLoading={false} />);
    const stream = screen.getByTestId("log-stream");
    expect(stream.getAttribute("role")).toBe("log");
    expect(stream.getAttribute("aria-live")).toBe("polite");
    expect(stream.getAttribute("aria-relevant")).toBe("additions");
  });

  it("each severity tier renders tri-channel signal (border + icon + text)", () => {
    render(
      <LogTable
        rows={[
          row(17, "ERROR", "boom"),
          row(13, "WARN", "warn"),
          row(9, "INFO", "info"),
          row(5, "DEBUG", "debug"),
        ]}
        isLoading={false}
      />,
    );
    const rows = screen.getAllByTestId("log-row");
    expect(rows[0].getAttribute("data-severity")).toBe("error");
    expect(rows[1].getAttribute("data-severity")).toBe("warn");
    expect(rows[2].getAttribute("data-severity")).toBe("info");
    expect(rows[3].getAttribute("data-severity")).toBe("debug");
    // Severity text labels visible
    expect(screen.getByText("ERROR")).toBeDefined();
    expect(screen.getByText("WARN")).toBeDefined();
    expect(screen.getByText("INFO")).toBeDefined();
    expect(screen.getByText("DEBUG")).toBeDefined();
  });

  it("shows loading row when rows empty + isLoading=true", () => {
    render(<LogTable rows={[]} isLoading={true} />);
    expect(screen.getByTestId("log-table-loading")).toBeDefined();
  });

  it("shows empty state when rows empty + isLoading=false", () => {
    render(<LogTable rows={[]} isLoading={false} />);
    expect(screen.getByTestId("log-table-empty")).toBeDefined();
  });

  it("renders trace_id correlation marker when present", () => {
    render(
      <LogTable
        rows={[row(17, "ERROR", "boom", "0102030405060708090a0b0c0d0e0f10")]}
        isLoading={false}
      />,
    );
    expect(screen.getByTestId("log-trace-correlation")).toBeDefined();
  });
});
