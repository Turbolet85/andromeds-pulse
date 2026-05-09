import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { Dashboard } from "./Dashboard";

vi.mock("../components/Titlebar", () => ({
  Titlebar: () => <header data-testid="titlebar-stub">titlebar</header>,
}));

vi.mock("../canvas/CanvasContainer", () => ({
  CanvasContainer: ({ ariaLabel }: { ariaLabel: string }) => (
    <section aria-label={ariaLabel} />
  ),
}));

vi.mock("../halo/HaloCanvas", () => ({
  HaloCanvas: ({ ariaLabel }: { ariaLabel: string; throughputHz: number; errorRate: number }) => (
    <section aria-label={ariaLabel} />
  ),
}));

const haloInput = { throughputHz: 1000, errorRate: 0.5 };

describe("Dashboard — shell composition", () => {
  it("renders <header> (titlebar) and <main> landmarks", () => {
    render(<Dashboard haloInput={haloInput} />);
    expect(screen.getByRole("banner").tagName).toBe("HEADER");
    expect(screen.getByRole("main").tagName).toBe("MAIN");
  });

  it("the <main> element has id='main-content' and tabindex=-1", () => {
    render(<Dashboard haloInput={haloInput} />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("id")).toBe("main-content");
    expect(main.getAttribute("tabindex")).toBe("-1");
  });

  it("renders both CanvasContainer and HaloCanvas regions inside <main>", () => {
    render(<Dashboard haloInput={haloInput} />);
    expect(
      screen.getByRole("region", { name: "Telemetry visualization canvas" }),
    ).toBeDefined();
    expect(
      screen.getByRole("region", { name: "Application status indicator" }),
    ).toBeDefined();
  });

  it("renders a polite live-region status announcer with id='shell-status'", () => {
    render(<Dashboard haloInput={haloInput} />);
    const status = screen.getByRole("status");
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.getAttribute("id")).toBe("shell-status");
  });
});
