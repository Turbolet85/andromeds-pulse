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
  },
  {
    metric_name: "mem",
    ts_unix_nano: 1_700_000_000_500,
    resource_hash: "def",
    value: 8,
    data_point_kind: 1,
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
});
