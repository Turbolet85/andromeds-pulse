import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import type { MetricRow } from "../../bindings";
import { __setProxyForTest } from "./metrics/use-metrics";
import { MetricsRoute } from "./MetricsRoute";

vi.mock("./metrics/MetricsChart", () => ({
  MetricsChart: ({ rows }: { rows: ReadonlyArray<MetricRow> }) => (
    <div data-testid="metrics-chart-stub" data-row-count={rows.length} />
  ),
}));

const sampleRows: MetricRow[] = [
  {
    metric_name: "cpu",
    ts_unix_nano: 1_700_000_000_000,
    resource_hash: "abc",
    value: 42,
    data_point_kind: 0,
    labels: "",
  },
  {
    metric_name: "mem",
    ts_unix_nano: 1_700_000_000_500,
    resource_hash: "def",
    value: 8,
    data_point_kind: 1,
    labels: "",
  },
];

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
});

function setupProxy(rows: MetricRow[]) {
  const queryFn = vi.fn().mockResolvedValue({
    items: rows,
    total: rows.length,
    next_cursor: null,
  });
  __setProxyForTest({ metrics: { query: queryFn } } as never);
  return queryFn;
}

function setupRejectProxy() {
  const queryFn = vi.fn().mockRejectedValue({ kind: "internal", message: "boom" });
  __setProxyForTest({ metrics: { query: queryFn } } as never);
  return queryFn;
}

describe("MetricsRoute", () => {
  it("renders <section> with id='tabpanel-metrics' + aria-labelledby heading", async () => {
    setupProxy([]);
    render(<MetricsRoute />);
    const section = screen.getByTestId("route-metrics");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-metrics");
    expect(section.getAttribute("aria-labelledby")).toBe("route-heading-metrics");
  });

  it("includes a single h1 heading with route label", async () => {
    setupProxy([]);
    render(<MetricsRoute />);
    expect(screen.getByRole("heading", { name: /metrics/i, level: 1 })).toBeDefined();
  });

  it("renders MetricsChart with rows fetched via metrics.query", async () => {
    setupProxy(sampleRows);
    render(<MetricsRoute />);
    await waitFor(() => {
      expect(
        screen.getByTestId("metrics-chart-stub").getAttribute("data-row-count"),
      ).toBe("2");
    });
  });

  it("shows the self-explaining empty state on settled no-data, naming the ports", async () => {
    setupProxy([]);
    render(<MetricsRoute />);
    const empty = await screen.findByTestId("metrics-empty-state");
    expect(empty.textContent).toContain("No metrics received yet");
    const hint = screen.getByTestId("metrics-empty-state-hint");
    expect(hint.textContent).toContain(":4318");
    expect(hint.textContent).toContain(":4317");
    expect(screen.queryByTestId("metrics-chart-stub")).toBeNull();
  });

  it("does not flash the empty state while loading (chart until settled)", async () => {
    setupProxy([]);
    render(<MetricsRoute />);
    expect(screen.queryByTestId("metrics-empty-state")).toBeNull();
    expect(screen.getByTestId("metrics-chart-stub")).toBeDefined();
    await screen.findByTestId("metrics-empty-state");
  });

  it("hides the empty state when populated", async () => {
    setupProxy(sampleRows);
    render(<MetricsRoute />);
    await waitFor(() =>
      expect(
        screen.getByTestId("metrics-chart-stub").getAttribute("data-row-count"),
      ).toBe("2"),
    );
    expect(screen.queryByTestId("metrics-empty-state")).toBeNull();
  });

  it("shows a distinct error state without the exporter hint when the query fails", async () => {
    setupRejectProxy();
    render(<MetricsRoute />);
    const err = await screen.findByTestId("metrics-error-state");
    expect(err.textContent).toContain("Couldn't load metrics");
    expect(err.textContent).not.toContain(":4318");
    expect(screen.queryByTestId("metrics-empty-state")).toBeNull();
  });
});
