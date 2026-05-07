import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { Titlebar } from "./Titlebar";

vi.mock("../hooks/use-platform", () => ({
  usePlatform: () => "windows" as const,
}));

vi.mock("../hooks/use-window-controls", () => ({
  useWindowControls: () => ({
    minimize: vi.fn().mockResolvedValue(undefined),
    maximize: vi.fn().mockResolvedValue(undefined),
    close: vi.fn().mockResolvedValue(undefined),
  }),
}));

describe("Titlebar — semantic HTML + drag region + ARIA", () => {
  it("wraps the titlebar in a <header> landmark", () => {
    render(<Titlebar />);
    const header = screen.getByRole("banner");
    expect(header.tagName).toBe("HEADER");
  });

  it("the titlebar container carries data-tauri-drag-region", () => {
    render(<Titlebar />);
    const titlebar = screen.getByTestId("titlebar");
    expect(titlebar.hasAttribute("data-tauri-drag-region")).toBe(true);
  });

  it("titlebar root has no tabindex (drag region is not focusable)", () => {
    render(<Titlebar />);
    const titlebar = screen.getByTestId("titlebar");
    expect(titlebar.hasAttribute("tabindex")).toBe(false);
  });

  it("renders the app title 'andromeda-pulse' by default", () => {
    render(<Titlebar />);
    expect(screen.getByText("andromeda-pulse")).toBeDefined();
  });

  it("accepts a title override prop", () => {
    render(<Titlebar title="andromeda-pulse — Traces" />);
    expect(screen.getByText("andromeda-pulse — Traces")).toBeDefined();
  });

  it("renders an Open settings button containing the aperture glyph", () => {
    render(<Titlebar />);
    const settings = screen.getByRole("button", { name: "Open settings" });
    expect(settings.tagName).toBe("BUTTON");
    expect(settings.getAttribute("type")).toBe("button");
    const svg = settings.querySelector("svg");
    expect(svg).not.toBeNull();
    expect(svg!.getAttribute("width")).toBe("20");
  });

  it("settings button forwards onSettingsClick", () => {
    const handler = vi.fn();
    render(<Titlebar onSettingsClick={handler} />);
    screen.getByRole("button", { name: "Open settings" }).click();
    expect(handler).toHaveBeenCalledTimes(1);
  });

  it("renders three window-control buttons on Windows (Minimize / Maximize / Close to tray)", () => {
    render(<Titlebar />);
    expect(screen.getByRole("button", { name: "Minimize" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Maximize" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Close to tray" })).toBeDefined();
  });

  it("decorative app-icon span carries aria-hidden", () => {
    const { container } = render(<Titlebar />);
    const appIcon = container.querySelector(".titlebar__app-icon");
    expect(appIcon?.getAttribute("aria-hidden")).toBe("true");
  });
});
