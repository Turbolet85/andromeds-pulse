import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { App } from "./App";

vi.mock("./hooks/use-platform", () => ({
  usePlatform: () => "windows" as const,
}));

vi.mock("./hooks/use-window-controls", () => ({
  useWindowControls: () => ({
    minimize: vi.fn().mockResolvedValue(undefined),
    maximize: vi.fn().mockResolvedValue(undefined),
    close: vi.fn().mockResolvedValue(undefined),
  }),
}));

// Stub the canvas substrate — App shell tests assert layout landmarks; the
// WebGPU adapter resolution is exercised in canvas/CanvasContainer.test.tsx.
vi.mock("./canvas/CanvasContainer", () => ({
  CanvasContainer: ({ ariaLabel }: { ariaLabel: string }) => (
    <section aria-label={ariaLabel} />
  ),
}));

// Stub HaloCanvas similarly — App shell tests assert composition; the WebGPU
// adapter + halo pipeline + reduced-motion gate are exercised in
// halo/HaloCanvas.test.tsx.
vi.mock("./halo/HaloCanvas", () => ({
  HaloCanvas: ({ ariaLabel }: { ariaLabel: string; throughputHz: number; errorRate: number }) => (
    <section aria-label={ariaLabel} />
  ),
}));

describe("App — shell composition", () => {
  it("renders both <header> (titlebar) and <main> landmarks", () => {
    render(<App />);
    expect(screen.getByRole("banner").tagName).toBe("HEADER");
    expect(screen.getByRole("main").tagName).toBe("MAIN");
  });

  it("the <main> element has id='main-content' for skip-link target (a11y plan §5)", () => {
    render(<App />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("id")).toBe("main-content");
  });

  it("the <main> element is focusable via tabIndex=-1 (focus restoration target)", () => {
    render(<App />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("tabindex")).toBe("-1");
  });

  it("renders a polite live-region status announcer", () => {
    render(<App />);
    const status = screen.getByRole("status");
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.getAttribute("id")).toBe("shell-status");
  });

  it("sets document.title to 'andromeda-pulse' on mount (SC 2.4.2)", () => {
    document.title = "previous-stub";
    render(<App />);
    expect(document.title).toBe("andromeda-pulse");
  });

  it("renders both CanvasContainer and HaloCanvas regions inside <main>", () => {
    render(<App />);
    expect(
      screen.getByRole("region", { name: "Telemetry visualization canvas" }),
    ).toBeDefined();
    expect(
      screen.getByRole("region", { name: "Application status indicator" }),
    ).toBeDefined();
  });

  it("starts a synthetic Halo input simulator interval on mount + clears on unmount", () => {
    const setIntervalSpy = vi.spyOn(globalThis, "setInterval");
    const clearIntervalSpy = vi.spyOn(globalThis, "clearInterval");
    const { unmount } = render(<App />);
    expect(setIntervalSpy).toHaveBeenCalled();
    unmount();
    expect(clearIntervalSpy).toHaveBeenCalled();
    setIntervalSpy.mockRestore();
    clearIntervalSpy.mockRestore();
  });
});
