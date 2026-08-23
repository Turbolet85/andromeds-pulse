import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import type { MetricRow } from "../../../bindings";
import { aggregateIntoBuckets, MetricsChart } from "./MetricsChart";

vi.mock("../../../canvas/webgpu-adapter", () => ({
  requestWebGPUAdapter: vi.fn(),
}));
vi.mock("../../../canvas/frame-metrics", () => ({
  recordFrameMs: vi.fn().mockResolvedValue(null),
  normalizeWgpuBackend: vi.fn().mockReturnValue("dx12"),
  detectWebviewBackend: vi.fn().mockReturnValue("webview2"),
}));
vi.mock("../../../hooks/use-reduced-motion", () => ({
  useReducedMotion: vi.fn().mockReturnValue(false),
}));

import { requestWebGPUAdapter } from "../../../canvas/webgpu-adapter";
import { recordFrameMs } from "../../../canvas/frame-metrics";

beforeEach(() => {
  vi.clearAllMocks();
  // jsdom returns null for canvas.getContext by default; stub a minimal 2D
  // context so MetricsChart's render path runs through to recordFrameMs.
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockImplementation(
    () =>
      // `as never` satisfies all `getContext` overload returns (the union now
      // includes `GPUCanvasContext` since @types/web added WebGPU; a narrower
      // cast picks one overload and fails the others). `never` is the bottom
      // type, assignable to every overload's return.
      ({
        clearRect: vi.fn(),
        strokeStyle: "",
        lineWidth: 0,
        beginPath: vi.fn(),
        moveTo: vi.fn(),
        lineTo: vi.fn(),
        stroke: vi.fn(),
      }) as never,
  );
});
afterEach(() => {
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

const fakeDevice = {
  queue: { onSubmittedWorkDone: vi.fn().mockResolvedValue(undefined) },
};

function mockAdapter(kind: "available" | "unavailable"): void {
  vi.mocked(requestWebGPUAdapter).mockResolvedValue(
    kind === "available"
      ? {
          kind: "available",
          adapter: {} as never,
          device: fakeDevice as never,
          backendKind: "dx12",
        }
      : { kind: "unavailable", reason: "navigator.gpu undefined" },
  );
}

const sampleRow = (ts_ns: number, value: number): MetricRow => ({
  metric_name: "cpu",
  ts_unix_nano: ts_ns,
  resource_hash: "abc",
  value,
  data_point_kind: 0,
  labels: "",
});

describe("MetricsChart", () => {
  it("wraps canvas in <section aria-label='Metrics time-series chart'>", async () => {
    mockAdapter("available");
    render(<MetricsChart rows={[]} windowSeconds={60} />);
    const region = await screen.findByRole("region", {
      name: "Metrics time-series chart",
    });
    expect(region).toBeDefined();
  });

  it("renders fallback when WebGPU is unavailable", async () => {
    mockAdapter("unavailable");
    render(<MetricsChart rows={[]} windowSeconds={60} />);
    await waitFor(() => {
      expect(
        screen.getByText(/webgpu not supported in this browser/i),
      ).toBeDefined();
    });
  });

  it("emits frame metric on render frame when adapter available", async () => {
    mockAdapter("available");
    render(
      <MetricsChart
        rows={[sampleRow(1_700_000_000_000, 5), sampleRow(1_700_000_000_500, 9)]}
        windowSeconds={60}
      />,
    );
    await waitFor(() => {
      expect(recordFrameMs).toHaveBeenCalled();
    });
  });

  it("exposes bucket count via data-bucket-count", async () => {
    mockAdapter("available");
    render(
      <MetricsChart
        rows={[sampleRow(1_700_000_000_000, 1)]}
        windowSeconds={60}
        bucketCount={5}
      />,
    );
    const region = await screen.findByRole("region", {
      name: "Metrics time-series chart",
    });
    expect(region.getAttribute("data-bucket-count")).toBe("5");
  });

  it("renders the canvas element when adapter resolves available", async () => {
    mockAdapter("available");
    render(<MetricsChart rows={[]} windowSeconds={60} />);
    await waitFor(() => {
      expect(screen.getByTestId("metrics-chart-canvas")).toBeDefined();
    });
  });
});

describe("aggregateIntoBuckets", () => {
  it("returns empty array when rows is empty", () => {
    expect(aggregateIntoBuckets([], 60, 30)).toEqual([]);
  });

  it("returns empty array when bucketCount is 0", () => {
    expect(aggregateIntoBuckets([sampleRow(1_700_000_000_000, 1)], 60, 0)).toEqual([]);
  });

  it("sums values into the correct bucket index", () => {
    const buckets = aggregateIntoBuckets(
      [sampleRow(1_700_000_000_000, 3), sampleRow(1_700_000_000_000, 4)],
      60,
      4,
    );
    expect(buckets).toHaveLength(4);
    const totalValue = buckets.reduce((acc, b) => acc + b.aggregateValue, 0);
    expect(totalValue).toBe(7);
  });

  it("clamps very-old rows into the first bucket", () => {
    const buckets = aggregateIntoBuckets(
      [
        sampleRow(1_700_000_000_000, 1),
        sampleRow(1_500_000_000_000, 2),
      ],
      60,
      3,
    );
    expect(buckets[0].aggregateValue).toBe(2);
  });
});
