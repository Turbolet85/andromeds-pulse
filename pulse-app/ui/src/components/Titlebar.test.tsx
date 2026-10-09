import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
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

vi.mock("../hooks/use-connection-state", () => ({
  useConnectionState: () => ({
    state: { state: "Receiving" },
    last_span_ago_ms: 1200,
    severity: "info",
    message: null,
    reason: null,
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

  it("does NOT render the Investigate button when onInvestigateClick is undefined", () => {
    render(<Titlebar />);
    expect(screen.queryByTestId("titlebar-investigate")).toBeNull();
  });

  it("renders the Investigate button with aria-label when onInvestigateClick is provided", () => {
    render(<Titlebar onInvestigateClick={() => {}} />);
    const btn = screen.getByTestId("titlebar-investigate");
    expect(btn.tagName).toBe("BUTTON");
    expect(btn.getAttribute("aria-label")).toBe("Investigate");
  });

  it("clicking the Investigate button forwards the trigger element to onInvestigateClick", async () => {
    const onInvestigateClick = vi.fn();
    const user = userEvent.setup();
    render(<Titlebar onInvestigateClick={onInvestigateClick} />);
    const btn = screen.getByTestId("titlebar-investigate");
    await user.click(btn);
    expect(onInvestigateClick).toHaveBeenCalledTimes(1);
    expect(onInvestigateClick).toHaveBeenCalledWith(btn);
  });

  it("does NOT render the dashboard-toggle button when onToggleDashboardClick is undefined", () => {
    render(<Titlebar />);
    expect(screen.queryByTestId("titlebar-toggle-dashboard")).toBeNull();
  });

  it("renders the dashboard-toggle button with aria-label when onToggleDashboardClick is provided", () => {
    render(<Titlebar onToggleDashboardClick={() => {}} />);
    const btn = screen.getByRole("button", { name: "Toggle dashboard" });
    expect(btn.tagName).toBe("BUTTON");
    expect(btn.getAttribute("type")).toBe("button");
    expect(screen.getByTestId("titlebar-toggle-dashboard")).toBe(btn);
  });

  it("clicking the dashboard-toggle button forwards onToggleDashboardClick", async () => {
    const onToggleDashboardClick = vi.fn();
    const user = userEvent.setup();
    render(<Titlebar onToggleDashboardClick={onToggleDashboardClick} />);
    await user.click(screen.getByTestId("titlebar-toggle-dashboard"));
    expect(onToggleDashboardClick).toHaveBeenCalledTimes(1);
  });

  it("renders the connection-state dot with role=img and a Connection aria-label", () => {
    render(<Titlebar />);
    const dot = screen.getByTestId("connection-dot");
    expect(dot.getAttribute("role")).toBe("img");
    expect(dot.getAttribute("aria-label")).toContain("Connection: ");
  });
});
