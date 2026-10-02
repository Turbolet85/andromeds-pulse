import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import type { LogRow } from "../../bindings";
import { __setProxyForTest } from "./logs/use-logs";
import { LogsRoute } from "./LogsRoute";

vi.mock("./logs/LogFilter", () => ({
  LogFilter: ({ state }: { state: { searchQuery: string } }) => (
    <div data-testid="log-filter-stub" data-search={state.searchQuery} />
  ),
}));

vi.mock("./logs/LogTable", () => ({
  LogTable: ({ rows }: { rows: ReadonlyArray<LogRow> }) => (
    <div data-testid="log-table-stub" data-row-count={rows.length} />
  ),
}));

const sampleRows: LogRow[] = [
  {
    ts_unix_nano: 1_700_000_000_000,
    resource_hash: "abc",
    severity_number: 9,
    body: "info row",
    severity_text: "INFO",
    trace_id: "",
    span_id: "",
  },
  {
    ts_unix_nano: 1_700_000_000_500,
    resource_hash: "def",
    severity_number: 17,
    body: "error row",
    severity_text: "ERROR",
    trace_id: "0102030405060708090a0b0c0d0e0f10",
    span_id: "0102030405060708",
  },
];

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
});

function setupProxy(rows: LogRow[]) {
  const queryFn = vi.fn().mockResolvedValue({
    items: rows,
    total: rows.length,
    next_cursor: null,
  });
  __setProxyForTest({ logs: { query: queryFn } } as never);
  return queryFn;
}

function setupRejectProxy() {
  const queryFn = vi.fn().mockRejectedValue({ kind: "internal", message: "boom" });
  __setProxyForTest({ logs: { query: queryFn } } as never);
  return queryFn;
}

describe("LogsRoute", () => {
  it("renders <section> with id='tabpanel-logs' + aria-labelledby heading", async () => {
    setupProxy([]);
    render(<LogsRoute />);
    const section = screen.getByTestId("route-logs");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-logs");
    expect(section.getAttribute("aria-labelledby")).toBe("route-heading-logs");
  });

  it("includes a single h1 heading with route label", async () => {
    setupProxy([]);
    render(<LogsRoute />);
    expect(screen.getByRole("heading", { name: /logs/i, level: 1 })).toBeDefined();
  });

  it("renders LogFilter ABOVE LogTable in DOM order", async () => {
    setupProxy(sampleRows);
    render(<LogsRoute />);
    await waitFor(() =>
      expect(screen.getByTestId("log-table-stub").getAttribute("data-row-count")).toBe("2"),
    );
    const filter = screen.getByTestId("log-filter-stub");
    const table = screen.getByTestId("log-table-stub");
    expect(filter.compareDocumentPosition(table)).toBe(
      Node.DOCUMENT_POSITION_FOLLOWING,
    );
  });

  it("forwards rows through filter to LogTable", async () => {
    setupProxy(sampleRows);
    render(<LogsRoute />);
    await waitFor(() => {
      expect(screen.getByTestId("log-table-stub").getAttribute("data-row-count")).toBe("2");
    });
  });

  it("shows the self-explaining empty state on settled no-data, naming the ports", async () => {
    setupProxy([]);
    render(<LogsRoute />);
    const empty = await screen.findByTestId("logs-empty-state");
    expect(empty.textContent).toContain("No logs received yet");
    const hint = screen.getByTestId("logs-empty-state-hint");
    expect(hint.textContent).toContain(":4318");
    expect(hint.textContent).toContain(":4317");
    expect(screen.queryByTestId("log-table-stub")).toBeNull();
    expect(screen.queryByTestId("log-filter-stub")).toBeNull();
  });

  it("does not flash the empty state while loading (filter+table until settled)", async () => {
    setupProxy([]);
    render(<LogsRoute />);
    expect(screen.queryByTestId("logs-empty-state")).toBeNull();
    expect(screen.getByTestId("log-table-stub")).toBeDefined();
    await screen.findByTestId("logs-empty-state");
  });

  it("keeps filter+table (not the empty state) when data exists", async () => {
    setupProxy(sampleRows);
    render(<LogsRoute />);
    await waitFor(() =>
      expect(screen.getByTestId("log-table-stub").getAttribute("data-row-count")).toBe("2"),
    );
    expect(screen.queryByTestId("logs-empty-state")).toBeNull();
  });

  it("shows a distinct error state without the exporter hint when the query fails", async () => {
    setupRejectProxy();
    render(<LogsRoute />);
    const err = await screen.findByTestId("logs-error-state");
    expect(err.textContent).toContain("Couldn't load logs");
    expect(err.textContent).not.toContain(":4318");
    expect(screen.queryByTestId("logs-empty-state")).toBeNull();
  });
});
