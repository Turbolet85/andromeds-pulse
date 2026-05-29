import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { AggregatedBadgeCanvas } from "./AggregatedBadgeCanvas";
import { HaloCanvas } from "../halo/HaloCanvas";
import type { ConnectionState, PriorityTier } from "../bindings/index";
import type { ActivityState } from "../halo/halo-types";

vi.mock("../halo/HaloCanvas", () => ({
  HaloCanvas: vi.fn(({ ariaLabel }: { ariaLabel: string }) => (
    <section aria-label={ariaLabel} data-testid="halo-canvas-stub" />
  )),
}));

afterEach(() => {
  vi.mocked(HaloCanvas).mockClear();
  vi.unstubAllGlobals();
});

const RECEIVING: ConnectionState = { state: "Receiving" };
const AUTONOMOUS: PriorityTier = "autonomous";
const ACTIVE: ActivityState = "active";

describe("AggregatedBadgeCanvas — composition", () => {
  it("forwards the three Halo axes (connectionState + cumulativeSeverity + activityState)", () => {
    render(
      <AggregatedBadgeCanvas
        serviceCount={24}
        errorRate={0.012}
        connectionState={{ state: "Stalled" }}
        cumulativeSeverity="suggested"
        activityState="quiet"
      />,
    );
    expect(vi.mocked(HaloCanvas)).toHaveBeenCalled();
    const lastCallProps = vi.mocked(HaloCanvas).mock.calls.at(-1)?.[0];
    expect(lastCallProps).toMatchObject({
      ariaLabel: "Service constellation halo",
      connectionState: { state: "Stalled" },
      cumulativeSeverity: "suggested",
      activityState: "quiet",
    });
  });

  it("renders the HaloCanvas stub inside the badge container", () => {
    render(
      <AggregatedBadgeCanvas
        serviceCount={24}
        errorRate={0.05}
        connectionState={RECEIVING}
        cumulativeSeverity={AUTONOMOUS}
        activityState={ACTIVE}
      />,
    );
    expect(screen.getByTestId("aggregated-badge-canvas")).toBeDefined();
    expect(screen.getByTestId("halo-canvas-stub")).toBeDefined();
  });

  it("badge wraps in role=status with complete-sentence aria-label", () => {
    render(
      <AggregatedBadgeCanvas
        serviceCount={24}
        errorRate={0.012}
        connectionState={RECEIVING}
        cumulativeSeverity={null}
        activityState={ACTIVE}
      />,
    );
    const status = screen.getByRole("status", {
      name: /\d+ services, \d+\.\d+% average error rate/,
    });
    expect(status.getAttribute("aria-label")).toBe("24 services, 1.2% average error rate");
  });

  it("decorative 'services' label is aria-hidden", () => {
    render(
      <AggregatedBadgeCanvas
        serviceCount={24}
        errorRate={0.012}
        connectionState={RECEIVING}
        cumulativeSeverity={null}
        activityState={ACTIVE}
      />,
    );
    const decoration = screen.getByText("services");
    expect(decoration.getAttribute("aria-hidden")).toBe("true");
  });

  it("visible service-count digits render in font-code with tabular-nums", () => {
    render(
      <AggregatedBadgeCanvas
        serviceCount={24}
        errorRate={0.012}
        connectionState={RECEIVING}
        cumulativeSeverity={null}
        activityState={ACTIVE}
      />,
    );
    const digit = screen.getByText("24");
    expect(digit.style.fontFamily).toContain("--font-code");
    expect(digit.style.fontVariantNumeric).toBe("tabular-nums");
  });

  it("badge overlay positions in bottom-right quadrant via absolute positioning", () => {
    render(
      <AggregatedBadgeCanvas
        serviceCount={24}
        errorRate={0.012}
        connectionState={RECEIVING}
        cumulativeSeverity={null}
        activityState={ACTIVE}
      />,
    );
    const status = screen.getByRole("status");
    expect(status.style.position).toBe("absolute");
    expect(status.style.bottom).toContain("--spacing-md");
    expect(status.style.right).toContain("--spacing-md");
  });

  it("placeholder service-count surfaces in aria-label on non-finite input", () => {
    render(
      <AggregatedBadgeCanvas
        serviceCount={Number.NaN}
        errorRate={0.012}
        connectionState={RECEIVING}
        cumulativeSeverity={null}
        activityState={ACTIVE}
      />,
    );
    const status = screen.getByRole("status");
    expect(status.getAttribute("aria-label")).toBe("— services, 1.2% average error rate");
  });
});
