import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { App } from "./App";
import { useWindowLabel } from "./hooks/use-window-label";

vi.mock("./hooks/use-window-label", () => ({
  useWindowLabel: vi.fn().mockReturnValue("main"),
}));

vi.mock("./widget/CompactWidget", () => ({
  CompactWidget: () => <div data-testid="compact-widget-stub" />,
}));

vi.mock("./dashboard/Dashboard", () => ({
  Dashboard: () => <div data-testid="dashboard-stub" />,
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

describe("App — document title", () => {
  it("sets document.title to 'andromeda-pulse' on mount (SC 2.4.2)", () => {
    document.title = "previous-stub";
    render(<App />);
    expect(document.title).toBe("andromeda-pulse");
  });
});
