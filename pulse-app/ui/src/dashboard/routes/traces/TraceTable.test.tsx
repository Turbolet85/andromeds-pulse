import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import type { TraceRow } from "../../../bindings";
import { InvestigationProvider } from "../../../hooks/use-investigation";
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

function treeFor(rows: TraceRow[], isLoading = false) {
  return (
    <StatusLiveRegionProvider>
      <InvestigationProvider>
        <TraceTable rows={rows} isLoading={isLoading} />
      </InvestigationProvider>
    </StatusLiveRegionProvider>
  );
}

function renderWithProvider(rows: TraceRow[], isLoading = false) {
  return render(treeFor(rows, isLoading));
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

  it("default view surfaces an erroring row first (anomaly-first, mixed dataset)", () => {
    renderWithProvider([
      row({ trace_id: "healthy1", error_count: 0 }),
      row({ trace_id: "boom", error_count: 2 }),
      row({ trace_id: "healthy2", error_count: 0 }),
    ]);
    const traceRows = screen.getAllByTestId("trace-row");
    // The erroring row is hoisted to the top (intent F8); the top row is not
    // all-healthy when an error exists.
    expect(within(traceRows[0]).queryByTestId("trace-row-error")).not.toBeNull();
    expect(traceRows[0].textContent).toContain("boom");
  });

  it("Errors only filter narrows to erroring rows via a real DOM event, then restores", async () => {
    const user = userEvent.setup();
    renderWithProvider([
      row({ trace_id: "healthy1", error_count: 0 }),
      row({ trace_id: "boom", error_count: 2 }),
      row({ trace_id: "healthy2", error_count: 0 }),
    ]);
    const toggle = screen.getByTestId("trace-errors-only-filter");
    expect(toggle.getAttribute("aria-pressed")).toBe("false");
    expect(screen.getAllByTestId("trace-row")).toHaveLength(3);

    await user.click(toggle);
    expect(toggle.getAttribute("aria-pressed")).toBe("true");
    const filtered = screen.getAllByTestId("trace-row");
    expect(filtered).toHaveLength(1);
    expect(filtered[0].textContent).toContain("boom");

    await user.click(toggle);
    expect(toggle.getAttribute("aria-pressed")).toBe("false");
    expect(screen.getAllByTestId("trace-row")).toHaveLength(3);
  });

  it("toggling the filter announces via the polite live region", async () => {
    const user = userEvent.setup();
    renderWithProvider([row({ error_count: 1 })]);
    await user.click(screen.getByTestId("trace-errors-only-filter"));
    const status = screen.getByTestId("status-live-region");
    expect(status.textContent).toMatch(/Showing errors only/i);
  });

  it("announces once (polite) when the table goes from empty to populated", () => {
    const { rerender } = render(treeFor([], false));
    const status = screen.getByTestId("status-live-region");
    expect(status.textContent).toBe("");

    rerender(treeFor([row({ trace_id: "aa" })], false));
    expect(status.textContent).toMatch(/Traces loaded/i);
  });

  it("does not announce when mounted already-populated, nor on later row changes", () => {
    const { rerender } = render(treeFor([row({ trace_id: "aa" })], false));
    const status = screen.getByTestId("status-live-region");
    expect(status.textContent).toBe("");

    rerender(treeFor([row({ trace_id: "aa" }), row({ trace_id: "bb" })], false));
    expect(status.textContent).toBe("");
  });

  it("preserves focus on the Errors-only button across a rows update", () => {
    const { rerender } = render(treeFor([row({ trace_id: "aa" })], false));
    const toggle = screen.getByTestId("trace-errors-only-filter");
    toggle.focus();
    expect(document.activeElement).toBe(toggle);

    rerender(treeFor([row({ trace_id: "aa" }), row({ trace_id: "bb" })], false));
    expect(document.activeElement).toBe(toggle);
  });

  it("keeps Errors-only + active sort across a rows update (no reset)", async () => {
    const user = userEvent.setup();
    const dataset = (): TraceRow[] => [
      row({ trace_id: "healthy", error_count: 0, duration_ms: 5 }),
      row({ trace_id: "boom", error_count: 2, duration_ms: 9 }),
    ];
    const { rerender } = render(treeFor(dataset()));
    const toggle = screen.getByTestId("trace-errors-only-filter");
    await user.click(toggle);
    await user.click(screen.getByTestId("sort-duration_ms"));
    const latencyHeader = screen.getByRole("columnheader", { name: /Latency/i });
    expect(toggle.getAttribute("aria-pressed")).toBe("true");
    expect(latencyHeader.getAttribute("aria-sort")).toBe("ascending");
    expect(screen.getAllByTestId("trace-row")).toHaveLength(1);

    rerender(treeFor(dataset()));
    expect(toggle.getAttribute("aria-pressed")).toBe("true");
    expect(latencyHeader.getAttribute("aria-sort")).toBe("ascending");
    expect(screen.getAllByTestId("trace-row")).toHaveLength(1);
  });
});
