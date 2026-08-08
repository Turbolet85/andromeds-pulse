import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { tabbable } from "tabbable";
import { CompactWidget } from "./CompactWidget";
import type { ServiceListItem } from "../bindings/index";
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

const servicesMock = vi.hoisted(() => ({
  items: [] as ServiceListItem[],
}));

vi.mock("../hooks/use-findings", () => ({
  useFindings: () => findingsMock.state,
}));

vi.mock("../hooks/use-service-constellation", () => ({
  useServiceConstellation: () => servicesMock.items,
}));

vi.mock("../components/Titlebar", () => ({
  Titlebar: () => <header data-testid="titlebar-stub">titlebar</header>,
}));

vi.mock("./ConstellationCanvas", () => ({
  ConstellationCanvas: ({ items }: { items: readonly ServiceListItem[] }) => (
    <div data-testid="constellation-stub" data-service-count={items.length} />
  ),
}));

function setFindingsState(state: Partial<typeof findingsMock.state>) {
  findingsMock.state.rows = state.rows ?? [];
  findingsMock.state.count = state.count ?? 0;
  findingsMock.state.severityMax = state.severityMax ?? null;
  findingsMock.state.lastAnnouncement = state.lastAnnouncement ?? "";
}

function setServices(items: ServiceListItem[]) {
  servicesMock.items = items;
}

describe("CompactWidget — three-band wireframe", () => {
  it("renders titlebar / main in DOM order (footer band removed)", () => {
    setFindingsState({ count: 0 });
    const { container } = render(<CompactWidget />);
    const top = container.firstElementChild as Element;
    expect(top.tagName).toBe("HEADER");
    expect(screen.getByRole("main").tagName).toBe("MAIN");
    expect(screen.queryByTestId("footer-band-stub")).toBeNull();
  });

  it("does not render the removed footer metrics (P-024 ambient invariant)", () => {
    setFindingsState({ count: 0 });
    render(<CompactWidget />);
    expect(screen.queryByText("Ingest")).toBeNull();
    expect(screen.queryByText("Error")).toBeNull();
    expect(screen.queryByText("Retention")).toBeNull();
  });

  it("the <main> element has id='main-content' for skip-link target", () => {
    render(<CompactWidget />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("id")).toBe("main-content");
  });

  it("the <main> element is focusable via tabIndex=-1 (focus restoration target)", () => {
    render(<CompactWidget />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("tabindex")).toBe("-1");
  });

  it("renders <main> with flex-column layout", () => {
    render(<CompactWidget />);
    const main = screen.getByRole("main");
    expect(main.style.display).toBe("flex");
    expect(main.style.flexDirection).toBe("column");
  });
});

describe("CompactWidget — constellation", () => {
  it("renders the service constellation fed by useServiceConstellation", () => {
    setFindingsState({ count: 0 });
    setServices([
      {
        service: "checkout",
        state: "active",
        last_seen_unix_nano: 1_000,
        manual_override: null,
        priority_tier: "autonomous",
      },
      {
        service: "billing",
        state: "quiet",
        last_seen_unix_nano: 1_000,
        manual_override: null,
        priority_tier: null,
      },
    ]);
    render(<CompactWidget />);
    const constellation = screen.getByTestId("constellation-stub");
    expect(constellation.getAttribute("data-service-count")).toBe("2");
  });

  it("renders the constellation with zero services without crashing", () => {
    setFindingsState({ count: 0 });
    setServices([]);
    render(<CompactWidget />);
    expect(screen.getByTestId("constellation-stub").getAttribute("data-service-count")).toBe("0");
  });
});

describe("CompactWidget — focus order", () => {
  it("introduces zero focusable elements when findings count is zero", () => {
    setFindingsState({ count: 0 });
    setServices([]);
    const { container } = render(<CompactWidget />);
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
    render(<CompactWidget />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.tagName).toBe("BUTTON");
    expect(counter.getAttribute("aria-label")).toContain("Findings: 3 unread");
  });
});

describe("CompactWidget — findings live region", () => {
  it("renders a polite aria-live region for findings announcements", () => {
    setFindingsState({ count: 0 });
    render(<CompactWidget />);
    const liveRegion = screen.getByTestId("findings-live-region");
    expect(liveRegion.getAttribute("role")).toBe("status");
    expect(liveRegion.getAttribute("aria-live")).toBe("polite");
  });

  it("reflects findings lastAnnouncement string in the live region", () => {
    setFindingsState({ count: 2, lastAnnouncement: "Findings: 2 unread" });
    render(<CompactWidget />);
    expect(screen.getByTestId("findings-live-region").textContent).toBe("Findings: 2 unread");
  });
});
