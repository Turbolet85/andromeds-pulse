import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { tabbable } from "tabbable";
import { CompactWidget } from "./CompactWidget";
import type { WidgetMetrics } from "./widget-types";

vi.mock("../components/Titlebar", () => ({
  Titlebar: () => <header data-testid="titlebar-stub">titlebar</header>,
}));

vi.mock("./AggregatedBadgeCanvas", () => ({
  AggregatedBadgeCanvas: ({
    serviceCount,
    throughputHz,
    errorRate,
  }: {
    serviceCount: number;
    throughputHz: number;
    errorRate: number;
  }) => (
    <div
      data-testid="aggregated-badge-stub"
      data-service-count={serviceCount}
      data-throughput-hz={throughputHz}
      data-error-rate={errorRate}
    />
  ),
}));

vi.mock("./FooterBand", () => ({
  FooterBand: ({
    throughputHz,
    errorRate,
    retentionUsedSeconds,
    retentionMaxSeconds,
  }: {
    throughputHz: number;
    errorRate: number;
    retentionUsedSeconds: number;
    retentionMaxSeconds: number;
  }) => (
    <footer
      data-testid="footer-band-stub"
      data-throughput-hz={throughputHz}
      data-error-rate={errorRate}
      data-retention-used={retentionUsedSeconds}
      data-retention-max={retentionMaxSeconds}
    />
  ),
}));

const metrics: WidgetMetrics = {
  serviceCount: 24,
  throughputHz: 1234,
  errorRate: 0.012,
  retentionUsedSeconds: 480,
  retentionMaxSeconds: 600,
};

describe("CompactWidget — three-band wireframe", () => {
  it("renders titlebar / main / footer in DOM order", () => {
    const { container } = render(<CompactWidget metrics={metrics} />);
    const top = container.firstElementChild as Element;
    expect(top.tagName).toBe("HEADER");
    expect(screen.getByRole("main").tagName).toBe("MAIN");
    expect(screen.getByTestId("footer-band-stub")).toBeDefined();
  });

  it("the <main> element has id='main-content' for skip-link target", () => {
    render(<CompactWidget metrics={metrics} />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("id")).toBe("main-content");
  });

  it("the <main> element is focusable via tabIndex=-1 (focus restoration target)", () => {
    render(<CompactWidget metrics={metrics} />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("tabindex")).toBe("-1");
  });

  it("renders <main> with flex-column layout", () => {
    render(<CompactWidget metrics={metrics} />);
    const main = screen.getByRole("main");
    expect(main.style.display).toBe("flex");
    expect(main.style.flexDirection).toBe("column");
  });
});

describe("CompactWidget — props flow", () => {
  it("forwards serviceCount + throughputHz + errorRate to AggregatedBadgeCanvas", () => {
    render(<CompactWidget metrics={metrics} />);
    const badge = screen.getByTestId("aggregated-badge-stub");
    expect(badge.dataset.serviceCount).toBe("24");
    expect(badge.dataset.throughputHz).toBe("1234");
    expect(badge.dataset.errorRate).toBe("0.012");
  });

  it("forwards 4 metric fields to FooterBand", () => {
    render(<CompactWidget metrics={metrics} />);
    const footer = screen.getByTestId("footer-band-stub");
    expect(footer.dataset.throughputHz).toBe("1234");
    expect(footer.dataset.errorRate).toBe("0.012");
    expect(footer.dataset.retentionUsed).toBe("480");
    expect(footer.dataset.retentionMax).toBe("600");
  });
});

describe("CompactWidget — focus order", () => {
  it("introduces zero new focusable interactive elements on the surface", () => {
    const { container } = render(<CompactWidget metrics={metrics} />);
    const focusables = tabbable(container);
    expect(focusables).toEqual([]);
  });
});
