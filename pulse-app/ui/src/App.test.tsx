import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { App } from "./App";
import { useWindowLabel } from "./hooks/use-window-label";

vi.mock("./hooks/use-window-label", () => ({
  useWindowLabel: vi.fn().mockReturnValue("main"),
}));

vi.mock("./hooks/use-synthetic-halo-input", () => ({
  useSyntheticHaloInput: () => ({ throughputHz: 1000, errorRate: 0.5 }),
}));

vi.mock("./widget/CompactWidget", () => ({
  CompactWidget: () => <div data-testid="compact-widget-stub" />,
}));

vi.mock("./dashboard/Dashboard", () => ({
  Dashboard: ({
    haloInput,
  }: {
    haloInput: { throughputHz: number; errorRate: number };
  }) => (
    <div
      data-testid="dashboard-stub"
      data-throughput-hz={haloInput.throughputHz}
      data-error-rate={haloInput.errorRate}
    />
  ),
}));

afterEach(() => {
  vi.mocked(useWindowLabel).mockReset();
  vi.mocked(useWindowLabel).mockReturnValue("main");
});

describe("App — window-label routing", () => {
  it("renders <CompactWidget> when window-label is 'compact-widget'", () => {
    vi.mocked(useWindowLabel).mockReturnValue("compact-widget");
    render(<App />);
    expect(screen.getByTestId("compact-widget-stub")).toBeDefined();
    expect(screen.queryByTestId("dashboard-stub")).toBeNull();
  });

  it("renders <Dashboard> when window-label is 'main'", () => {
    vi.mocked(useWindowLabel).mockReturnValue("main");
    render(<App />);
    expect(screen.getByTestId("dashboard-stub")).toBeDefined();
    expect(screen.queryByTestId("compact-widget-stub")).toBeNull();
  });

  it("renders <Dashboard> when window-label is 'unknown' (jsdom fallback)", () => {
    vi.mocked(useWindowLabel).mockReturnValue("unknown");
    render(<App />);
    expect(screen.getByTestId("dashboard-stub")).toBeDefined();
    expect(screen.queryByTestId("compact-widget-stub")).toBeNull();
  });
});

describe("App — props flow to children", () => {
  it("forwards HaloInput to <Dashboard>", () => {
    vi.mocked(useWindowLabel).mockReturnValue("main");
    render(<App />);
    const dashboard = screen.getByTestId("dashboard-stub");
    expect(dashboard.dataset.throughputHz).toBe("1000");
    expect(dashboard.dataset.errorRate).toBe("0.5");
  });
});

describe("App — document title", () => {
  it("sets document.title to 'andromeda-pulse' on mount (SC 2.4.2)", () => {
    document.title = "previous-stub";
    render(<App />);
    expect(document.title).toBe("andromeda-pulse");
  });
});
