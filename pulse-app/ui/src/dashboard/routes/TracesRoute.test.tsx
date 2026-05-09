import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { TracesRoute } from "./TracesRoute";
import { HaloInputProvider } from "../halo-input-context";

vi.mock("../../canvas/CanvasContainer", () => ({
  CanvasContainer: ({ ariaLabel }: { ariaLabel: string }) => (
    <section aria-label={ariaLabel} data-testid="canvas-container-stub" />
  ),
}));

vi.mock("../../halo/HaloCanvas", () => ({
  HaloCanvas: ({ ariaLabel, throughputHz, errorRate }: { ariaLabel: string; throughputHz: number; errorRate: number }) => (
    <section
      aria-label={ariaLabel}
      data-testid="halo-canvas-stub"
      data-throughput={throughputHz}
      data-error-rate={errorRate}
    />
  ),
}));

const haloInput = { throughputHz: 1234, errorRate: 0.05 };

describe("TracesRoute", () => {
  it("renders <section> with id='tabpanel-traces' (Outlet target)", () => {
    render(
      <HaloInputProvider value={haloInput}>
        <TracesRoute />
      </HaloInputProvider>,
    );
    const section = screen.getByTestId("route-traces");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-traces");
  });

  it("includes a single h1 heading with route label", () => {
    render(
      <HaloInputProvider value={haloInput}>
        <TracesRoute />
      </HaloInputProvider>,
    );
    const heading = screen.getByRole("heading", { name: /traces/i, level: 1 });
    expect(heading).toBeDefined();
  });

  it("mounts CanvasContainer with telemetry visualization aria-label", () => {
    render(
      <HaloInputProvider value={haloInput}>
        <TracesRoute />
      </HaloInputProvider>,
    );
    expect(
      screen.getByRole("region", { name: "Telemetry visualization canvas" }),
    ).toBeDefined();
  });

  it("mounts HaloCanvas with status-indicator aria-label + haloInput propagated", () => {
    render(
      <HaloInputProvider value={haloInput}>
        <TracesRoute />
      </HaloInputProvider>,
    );
    const halo = screen.getByTestId("halo-canvas-stub");
    expect(halo.getAttribute("data-throughput")).toBe("1234");
    expect(halo.getAttribute("data-error-rate")).toBe("0.05");
  });
});
