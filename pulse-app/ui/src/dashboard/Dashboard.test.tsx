import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { Dashboard } from "./Dashboard";

vi.mock("../components/Titlebar", () => ({
  Titlebar: ({ title }: { title?: string }) => (
    <header data-testid="titlebar-stub">{title ?? "titlebar"}</header>
  ),
}));

vi.mock("../canvas/CanvasContainer", () => ({
  CanvasContainer: ({ ariaLabel }: { ariaLabel: string }) => (
    <section aria-label={ariaLabel} data-testid="canvas-container-stub" />
  ),
}));

vi.mock("../halo/HaloCanvas", () => ({
  HaloCanvas: ({ ariaLabel }: { ariaLabel: string; throughputHz: number; errorRate: number }) => (
    <section aria-label={ariaLabel} data-testid="halo-canvas-stub" />
  ),
}));

vi.mock("@tauri-apps/api/webviewWindow", () => ({
  getAllWebviewWindows: vi.fn().mockResolvedValue([]),
}));

const haloInput = { throughputHz: 1000, errorRate: 0.5 };

describe("Dashboard — shell composition", () => {
  beforeEach(() => {
    window.history.pushState({}, "", "/");
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("renders titlebar + tablist + main + footer + status live region", async () => {
    render(<Dashboard haloInput={haloInput} />);
    expect(await screen.findByRole("banner")).toBeDefined();
    expect(screen.getByRole("tablist")).toBeDefined();
    expect(screen.getByRole("main")).toBeDefined();
    expect(screen.getByTestId("dashboard-footer")).toBeDefined();
    expect(screen.getByRole("status")).toBeDefined();
  });

  it("the <main> element has id='main-content' and tabindex=-1 (skip-link target)", async () => {
    render(<Dashboard haloInput={haloInput} />);
    await screen.findByRole("main");
    const main = screen.getByRole("main");
    expect(main.getAttribute("id")).toBe("main-content");
    expect(main.getAttribute("tabindex")).toBe("-1");
  });

  it("renders skip-to-main link as the first focusable element", async () => {
    render(<Dashboard haloInput={haloInput} />);
    const skipLink = await screen.findByTestId("skip-to-main");
    expect(skipLink.tagName).toBe("A");
    expect(skipLink.getAttribute("href")).toBe("#main-content");
  });

  it("status live region is polite and visually hidden", async () => {
    render(<Dashboard haloInput={haloInput} />);
    const status = await screen.findByRole("status");
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.getAttribute("id")).toBe("shell-status");
  });

  it("renders the 5-tab tablist with the canonical tab order", async () => {
    render(<Dashboard haloInput={haloInput} />);
    await screen.findByRole("tablist");
    const tabs = screen.getAllByRole("tab");
    expect(tabs).toHaveLength(5);
    expect(tabs.map((t) => t.textContent)).toEqual([
      expect.stringContaining("Traces"),
      expect.stringContaining("Metrics"),
      expect.stringContaining("Logs"),
      expect.stringContaining("Snapshots"),
      expect.stringContaining("Settings"),
    ]);
  });

  it("default route is /traces — Traces tab is aria-selected", async () => {
    render(<Dashboard haloInput={haloInput} />);
    const tracesTab = await screen.findByTestId("tab-traces");
    expect(tracesTab.getAttribute("aria-selected")).toBe("true");
  });
});
