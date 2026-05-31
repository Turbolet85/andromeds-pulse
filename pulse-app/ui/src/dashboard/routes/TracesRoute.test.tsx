import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import type { TraceRow } from "../../bindings";
import { InvestigationProvider } from "../../hooks/use-investigation";
import { __setProxyForTest } from "./traces/use-traces";
import { TracesRoute } from "./TracesRoute";

vi.mock("../../hooks/use-service-constellation", () => ({
  useServiceConstellation: () => [
    {
      service: "svc-a",
      state: "active",
      last_seen_unix_nano: 1_000,
      manual_override: null,
      priority_tier: null,
    },
    {
      service: "svc-b",
      state: "quiet",
      last_seen_unix_nano: 1_000,
      manual_override: null,
      priority_tier: null,
    },
  ],
}));

vi.mock("./traces/ConstellationCanvas", () => ({
  ConstellationCanvas: ({
    items,
  }: {
    items: ReadonlyArray<{ service: string }>;
  }) => (
    <section
      aria-label="Service constellation"
      data-testid="constellation-canvas-stub"
      data-service-count={items.length}
    />
  ),
}));

vi.mock("./traces/TraceTable", () => ({
  TraceTable: ({ rows }: { rows: ReadonlyArray<TraceRow> }) => (
    <div data-testid="trace-table-stub" data-row-count={rows.length} />
  ),
}));

const sampleRows: TraceRow[] = [
  {
    trace_id: "deadbeef",
    span_id: "00000001",
    ts_unix_nano: 1_700_000_000_000,
    service: "svc-a",
    duration_ms: 5,
    error_count: 0,
  },
  {
    trace_id: "feedface",
    span_id: "00000002",
    ts_unix_nano: 1_700_000_000_001,
    service: "svc-b",
    duration_ms: 12,
    error_count: 1,
  },
];

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
});

function setupProxyWithRows(rows: TraceRow[]) {
  const queryFn = vi.fn().mockResolvedValue({
    items: rows,
    total: rows.length,
    next_cursor: null,
  });
  __setProxyForTest({ traces: { query: queryFn } } as never);
  return queryFn;
}

describe("TracesRoute", () => {
  it("renders <section> with id='tabpanel-traces' (Outlet target)", async () => {
    setupProxyWithRows([]);
    render(
      <InvestigationProvider>
        <TracesRoute />
      </InvestigationProvider>,
    );
    const section = screen.getByTestId("route-traces");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-traces");
  });

  it("includes a single h1 heading with route label", async () => {
    setupProxyWithRows([]);
    render(
      <InvestigationProvider>
        <TracesRoute />
      </InvestigationProvider>,
    );
    const heading = screen.getByRole("heading", { name: /traces/i, level: 1 });
    expect(heading).toBeDefined();
  });

  it("renders ConstellationCanvas hero ABOVE TraceTable in DOM order", async () => {
    setupProxyWithRows(sampleRows);
    render(
      <InvestigationProvider>
        <TracesRoute />
      </InvestigationProvider>,
    );
    await waitFor(() =>
      expect(screen.getByTestId("trace-table-stub").getAttribute("data-row-count")).toBe("2"),
    );
    const constellation = screen.getByTestId("constellation-canvas-stub");
    const table = screen.getByTestId("trace-table-stub");
    // Bitmask: DOCUMENT_POSITION_FOLLOWING = 4 → constellation is followed by table.
    expect(constellation.compareDocumentPosition(table)).toBe(
      Node.DOCUMENT_POSITION_FOLLOWING,
    );
  });

  it("forwards service-registry items to ConstellationCanvas", async () => {
    setupProxyWithRows(sampleRows);
    render(
      <InvestigationProvider>
        <TracesRoute />
      </InvestigationProvider>,
    );
    await waitFor(() => {
      expect(
        screen.getByTestId("constellation-canvas-stub").getAttribute("data-service-count"),
      ).toBe("2");
    });
  });

  it("forwards rows to TraceTable", async () => {
    setupProxyWithRows(sampleRows);
    render(
      <InvestigationProvider>
        <TracesRoute />
      </InvestigationProvider>,
    );
    await waitFor(() => {
      expect(screen.getByTestId("trace-table-stub").getAttribute("data-row-count")).toBe("2");
    });
  });
});
