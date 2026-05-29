import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { tabbable } from "tabbable";
import { CompactWidget } from "./CompactWidget";
import type { WidgetMetrics } from "./widget-types";
import type { FindingsRow } from "./findings-types";

const findingsMock = vi.hoisted(() => ({
  state: {
    rows: [] as FindingsRow[],
    count: 0,
    severityMax: null as null | "autonomous" | "suggested" | "curious",
    markAllRead: vi.fn(),
    refetch: vi.fn(),
    lastAnnouncement: "",
  },
}));

vi.mock("../hooks/use-findings", () => ({
  useFindings: () => findingsMock.state,
}));

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

function setFindingsState(state: Partial<typeof findingsMock.state>) {
  findingsMock.state.rows = state.rows ?? [];
  findingsMock.state.count = state.count ?? 0;
  findingsMock.state.severityMax = state.severityMax ?? null;
  findingsMock.state.lastAnnouncement = state.lastAnnouncement ?? "";
}

const metrics: WidgetMetrics = {
  serviceCount: 24,
  throughputHz: 1234,
  errorRate: 0.012,
  retentionUsedSeconds: 480,
  retentionMaxSeconds: 600,
};

describe("CompactWidget — three-band wireframe", () => {
  it("renders titlebar / main in DOM order (footer band removed)", () => {
    setFindingsState({ count: 0 });
    const { container } = render(<CompactWidget metrics={metrics} />);
    const top = container.firstElementChild as Element;
    expect(top.tagName).toBe("HEADER");
    expect(screen.getByRole("main").tagName).toBe("MAIN");
    expect(screen.queryByTestId("footer-band-stub")).toBeNull();
  });

  it("does not render the removed footer metrics (P-024 ambient invariant)", () => {
    setFindingsState({ count: 0 });
    render(<CompactWidget metrics={metrics} />);
    expect(screen.queryByText("Ingest")).toBeNull();
    expect(screen.queryByText("Error")).toBeNull();
    expect(screen.queryByText("Retention")).toBeNull();
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
});

describe("CompactWidget — focus order", () => {
  it("introduces zero focusable elements when findings count is zero", () => {
    setFindingsState({ count: 0 });
    const { container } = render(<CompactWidget metrics={metrics} />);
    const focusables = tabbable(container);
    expect(focusables).toEqual([]);
  });

  it("renders the findings counter button when count > 0", () => {
    setFindingsState({
      count: 3,
      severityMax: "autonomous",
      rows: [
        { id: 1, priorityTier: "autonomous", title: "x", openedAtUnixNano: 0 },
        { id: 2, priorityTier: "autonomous", title: "y", openedAtUnixNano: 0 },
        { id: 3, priorityTier: "suggested", title: "z", openedAtUnixNano: 0 },
      ],
    });
    render(<CompactWidget metrics={metrics} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.tagName).toBe("BUTTON");
    expect(counter.getAttribute("aria-label")).toContain("Findings: 3 unread");
  });
});

describe("CompactWidget — findings live region", () => {
  it("renders а polite aria-live region for findings announcements", () => {
    setFindingsState({ count: 0 });
    render(<CompactWidget metrics={metrics} />);
    const liveRegion = screen.getByTestId("findings-live-region");
    expect(liveRegion.getAttribute("role")).toBe("status");
    expect(liveRegion.getAttribute("aria-live")).toBe("polite");
  });

  it("reflects findings lastAnnouncement string в the live region", () => {
    setFindingsState({ count: 2, lastAnnouncement: "Findings: 2 unread" });
    render(<CompactWidget metrics={metrics} />);
    expect(screen.getByTestId("findings-live-region").textContent).toBe("Findings: 2 unread");
  });
});
