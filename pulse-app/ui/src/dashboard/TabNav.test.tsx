import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { TabNav } from "./TabNav";

describe("TabNav", () => {
  it("renders <nav> with aria-label='Dashboard sections' wrapping role='tablist'", () => {
    render(<TabNav activeTabId="traces" onSelect={vi.fn()} />);
    const nav = screen.getByRole("navigation", { name: /dashboard sections/i });
    expect(nav).toBeDefined();
    const tablist = screen.getByRole("tablist");
    expect(nav.contains(tablist)).toBe(true);
  });

  it("renders exactly 5 tabs in the canonical order", () => {
    render(<TabNav activeTabId="traces" onSelect={vi.fn()} />);
    const tabs = screen.getAllByRole("tab");
    expect(tabs).toHaveLength(5);
    expect(tabs.map((t) => t.getAttribute("data-testid"))).toEqual([
      "tab-traces",
      "tab-metrics",
      "tab-logs",
      "tab-snapshots",
      "tab-settings",
    ]);
  });

  it("active tab has aria-selected='true' + tabindex=0; others tabindex=-1", () => {
    render(<TabNav activeTabId="metrics" onSelect={vi.fn()} />);
    const metricsTab = screen.getByTestId("tab-metrics");
    expect(metricsTab.getAttribute("aria-selected")).toBe("true");
    expect(metricsTab.getAttribute("tabindex")).toBe("0");
    const tracesTab = screen.getByTestId("tab-traces");
    expect(tracesTab.getAttribute("aria-selected")).toBe("false");
    expect(tracesTab.getAttribute("tabindex")).toBe("-1");
  });

  it("only the selected tab carries aria-controls (inactive panels are unmounted)", () => {
    // Chunk #99 re-audit: a static aria-controls on every tab dangles to
    // missing ids (aria-valid-attr-value@critical) because only the active
    // route's tabpanel is mounted. ARIA APG marks aria-controls optional.
    render(<TabNav activeTabId="traces" onSelect={vi.fn()} />);
    expect(screen.getByTestId("tab-traces").getAttribute("aria-controls")).toBe(
      "tabpanel-traces",
    );
    expect(screen.getByTestId("tab-settings").getAttribute("aria-controls")).toBeNull();
  });

  it("non-tab routes mark no tab selected and emit no aria-controls", () => {
    render(<TabNav activeTabId="traces" activeIsCurrentRoute={false} onSelect={vi.fn()} />);
    const traces = screen.getByTestId("tab-traces");
    expect(traces.getAttribute("aria-selected")).toBe("false");
    expect(traces.getAttribute("aria-controls")).toBeNull();
    // Roving entry point preserved: the fallback tab keeps tabindex 0.
    expect(traces.getAttribute("tabindex")).toBe("0");
  });

  it("clicking a tab calls onSelect with that tab id", async () => {
    const onSelect = vi.fn();
    const user = userEvent.setup();
    render(<TabNav activeTabId="traces" onSelect={onSelect} />);
    await user.click(screen.getByTestId("tab-logs"));
    expect(onSelect).toHaveBeenCalledWith("logs");
  });

  it("ArrowRight cycles to the next tab via onSelect", () => {
    const onSelect = vi.fn();
    render(<TabNav activeTabId="traces" onSelect={onSelect} />);
    const tablist = screen.getByRole("tablist");
    fireEvent.keyDown(tablist, { key: "ArrowRight" });
    expect(onSelect).toHaveBeenCalledWith("metrics");
  });

  it("ArrowLeft from first tab wraps to last tab via onSelect", () => {
    const onSelect = vi.fn();
    render(<TabNav activeTabId="traces" onSelect={onSelect} />);
    const tablist = screen.getByRole("tablist");
    fireEvent.keyDown(tablist, { key: "ArrowLeft" });
    expect(onSelect).toHaveBeenCalledWith("settings");
  });

  it("Home jumps to the first tab; End jumps to the last", () => {
    const onSelect = vi.fn();
    render(<TabNav activeTabId="logs" onSelect={onSelect} />);
    const tablist = screen.getByRole("tablist");
    fireEvent.keyDown(tablist, { key: "Home" });
    expect(onSelect).toHaveBeenCalledWith("traces");
    fireEvent.keyDown(tablist, { key: "End" });
    expect(onSelect).toHaveBeenCalledWith("settings");
  });

  it("active state uses font-weight 600 (non-color cue per SC 1.4.1)", () => {
    render(<TabNav activeTabId="traces" onSelect={vi.fn()} />);
    const tracesTab = screen.getByTestId("tab-traces");
    expect(tracesTab.style.fontWeight).toBe("600");
    const metricsTab = screen.getByTestId("tab-metrics");
    expect(metricsTab.style.fontWeight).toBe("500");
  });
});
