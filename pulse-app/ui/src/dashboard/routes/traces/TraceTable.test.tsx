import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import type { TraceRow } from "../../../bindings";
import { StatusLiveRegionProvider } from "../../StatusLiveRegion";
import { TraceTable } from "./TraceTable";

afterEach(() => {
  vi.unstubAllGlobals();
  vi.clearAllMocks();
});

function row(overrides: Partial<TraceRow>): TraceRow {
  return {
    trace_id: "00000000",
    span_id: "00000000",
    ts_unix_nano: 0,
    service: "svc",
    duration_ms: 10,
    error_count: 0,
    ...overrides,
  };
}

function renderWithProvider(rows: TraceRow[], isLoading = false) {
  return render(
    <StatusLiveRegionProvider>
      <TraceTable rows={rows} isLoading={isLoading} />
    </StatusLiveRegionProvider>,
  );
}

describe("TraceTable", () => {
  it("renders semantic <table> with all 4 column headers", () => {
    renderWithProvider([row({})]);
    const table = screen.getByRole("table");
    expect(table.tagName).toBe("TABLE");
    expect(within(table).getAllByRole("columnheader")).toHaveLength(4);
    expect(screen.getByRole("columnheader", { name: /Trace ID/i })).toBeDefined();
    expect(screen.getByRole("columnheader", { name: /Service/i })).toBeDefined();
    expect(screen.getByRole("columnheader", { name: /Latency/i })).toBeDefined();
    expect(screen.getByRole("columnheader", { name: /Error/i })).toBeDefined();
  });

  it("renders empty state when rows is empty + not loading", () => {
    renderWithProvider([], false);
    expect(screen.getByTestId("trace-table-empty")).toBeDefined();
    expect(screen.getByText(/No traces yet/i)).toBeDefined();
  });

  it("renders loading state when isLoading + no rows", () => {
    renderWithProvider([], true);
    expect(screen.getByTestId("trace-table-loading")).toBeDefined();
  });

  it("aria-sort defaults to none on every header", () => {
    renderWithProvider([row({})]);
    const headers = screen.getAllByRole("columnheader");
    for (const h of headers) {
      expect(h.getAttribute("aria-sort")).toBe("none");
    }
  });

  it("clicking a header cycles aria-sort: none → asc → desc → none", async () => {
    const user = userEvent.setup();
    renderWithProvider([
      row({ trace_id: "a", duration_ms: 10 }),
      row({ trace_id: "b", duration_ms: 20 }),
    ]);
    const traceIdHeader = screen.getByRole("columnheader", { name: /Trace ID/i });
    expect(traceIdHeader.getAttribute("aria-sort")).toBe("none");

    await user.click(screen.getByTestId("sort-trace_id"));
    expect(traceIdHeader.getAttribute("aria-sort")).toBe("ascending");

    await user.click(screen.getByTestId("sort-trace_id"));
    expect(traceIdHeader.getAttribute("aria-sort")).toBe("descending");

    await user.click(screen.getByTestId("sort-trace_id"));
    expect(traceIdHeader.getAttribute("aria-sort")).toBe("none");
  });

  it("sort change announces via the polite live region", async () => {
    const user = userEvent.setup();
    renderWithProvider([row({ duration_ms: 10 })]);
    await user.click(screen.getByTestId("sort-duration_ms"));
    const status = screen.getByTestId("status-live-region");
    expect(status.textContent).toMatch(/Sorted by Latency, ascending/i);
  });

  it("error row pairs --color-accent border with text+icon (not color alone)", () => {
    renderWithProvider([
      row({ trace_id: "ee", service: "svc-x", duration_ms: 5, error_count: 3 }),
    ]);
    const errorCell = screen.getByTestId("trace-row-error");
    // Border-left uses the accent token (color present)
    expect(errorCell.getAttribute("style")).toContain("var(--color-accent)");
    // The error count text is displayed (NOT only encoded by color)
    const inner = within(errorCell).getByTestId("trace-error-cell");
    expect(inner.textContent).toContain("3");
    // And the ✗ icon glyph is present (not color alone)
    expect(inner.textContent).toContain("✗");
  });

  it("renders a row per input + truncates long trace_id hex", () => {
    renderWithProvider([
      row({ trace_id: "0123456789abcdef0123456789abcdef" }),
      row({ trace_id: "ffeeddccbbaa99887766554433221100", span_id: "01" }),
    ]);
    const rows = screen.getAllByTestId("trace-row");
    expect(rows).toHaveLength(2);
    // Truncated form contains an ellipsis.
    expect(rows[0].textContent).toMatch(/01234567…/);
  });
});
