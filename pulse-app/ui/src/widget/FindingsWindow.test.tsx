import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
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

const windowMock = vi.hoisted(() => ({
  dismissFindings: vi.fn(),
  resizeFindingsWindow: vi.fn(),
  openReportWindow: vi.fn(),
  onReportClosed: vi.fn(() => () => {}),
}));

vi.mock("../hooks/use-findings", () => ({
  useFindings: () => findingsMock.state,
}));

vi.mock("../hooks/use-findings-window", () => ({
  dismissFindings: windowMock.dismissFindings,
  resizeFindingsWindow: windowMock.resizeFindingsWindow,
  openReportWindow: windowMock.openReportWindow,
  onReportClosed: windowMock.onReportClosed,
}));

import { FindingsWindow } from "./FindingsWindow";

const ROWS: FindingsRow[] = [
  { id: 1, priorityTier: "autonomous", title: "payment-service errors", openedAtUnixNano: 0 },
  { id: 2, priorityTier: "suggested", title: "checkout latency", openedAtUnixNano: 0 },
];

function setRows(rows: FindingsRow[]) {
  findingsMock.state.rows = rows;
  findingsMock.state.count = rows.length;
}

beforeEach(() => {
  windowMock.dismissFindings.mockReset();
  windowMock.resizeFindingsWindow.mockReset();
  windowMock.openReportWindow.mockReset();
  windowMock.onReportClosed.mockClear();
  findingsMock.state.markAllRead.mockReset();
  findingsMock.state.lastAnnouncement = "";
  setRows(ROWS);
});

describe("FindingsWindow — render", () => {
  it("renders the unread rows as native buttons", () => {
    render(<FindingsWindow />);
    const rows = screen.getAllByTestId("findings-window-row");
    expect(rows).toHaveLength(2);
    expect(rows[0].tagName).toBe("BUTTON");
    expect(rows[0].getAttribute("aria-label")).toContain("Autonomous");
  });

  it("uses the opaque --color-raised-2 popover surface", () => {
    render(<FindingsWindow />);
    const panel = screen.getByTestId("findings-window");
    expect(panel.style.background).toContain("var(--color-raised-2)");
  });

  it("renders an empty state when there are no unread rows", () => {
    setRows([]);
    render(<FindingsWindow />);
    expect(screen.getByTestId("findings-window-empty")).toBeTruthy();
    expect(screen.queryByTestId("findings-window-row")).toBeNull();
  });

  it("severity dot is aria-hidden and the tier word is in the accessible name (SC 1.4.1)", () => {
    render(<FindingsWindow />);
    const dots = screen.getAllByTestId("findings-window-severity-dot");
    for (const dot of dots) {
      expect(dot.getAttribute("aria-hidden")).toBe("true");
    }
    const rows = screen.getAllByTestId("findings-window-row");
    expect(rows[0].getAttribute("aria-label")).toMatch(/autonomous|suggested|curious/i);
  });
});

describe("FindingsWindow — focus + live region", () => {
  it("moves focus into the panel (first row) on mount", () => {
    render(<FindingsWindow />);
    const rows = screen.getAllByTestId("findings-window-row");
    expect(document.activeElement).toBe(rows[0]);
  });

  it("renders a polite live region reflecting lastAnnouncement", () => {
    findingsMock.state.lastAnnouncement = "Findings: 2 unread";
    render(<FindingsWindow />);
    const region = screen.getByTestId("findings-window-live-region");
    expect(region.getAttribute("role")).toBe("status");
    expect(region.getAttribute("aria-live")).toBe("polite");
    expect(region.textContent).toBe("Findings: 2 unread");
  });
});

describe("FindingsWindow — dismiss lifecycle", () => {
  it("Escape dismisses the window", () => {
    render(<FindingsWindow />);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(windowMock.dismissFindings).toHaveBeenCalledTimes(1);
  });

  it("mark-all-read marks + dismisses", async () => {
    const user = userEvent.setup();
    render(<FindingsWindow />);
    await user.click(screen.getByTestId("findings-window-mark-all-read"));
    expect(findingsMock.state.markAllRead).toHaveBeenCalledTimes(1);
    expect(windowMock.dismissFindings).toHaveBeenCalledTimes(1);
  });

  it("row-select opens the Report in a separate window (does not dismiss this one)", async () => {
    const user = userEvent.setup();
    render(<FindingsWindow />);
    await user.click(screen.getAllByTestId("findings-window-row")[0]);
    expect(windowMock.openReportWindow).toHaveBeenCalledWith(1);
    expect(windowMock.dismissFindings).not.toHaveBeenCalled();
  });

  it("does NOT dismiss on Escape while a Report window is open", async () => {
    const user = userEvent.setup();
    render(<FindingsWindow />);
    await user.click(screen.getAllByTestId("findings-window-row")[0]);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(windowMock.dismissFindings).not.toHaveBeenCalled();
  });

  it("window blur dismisses when the document is no longer focused", () => {
    vi.spyOn(document, "hasFocus").mockReturnValue(false);
    render(<FindingsWindow />);
    fireEvent(window, new Event("blur"));
    expect(windowMock.dismissFindings).toHaveBeenCalledTimes(1);
    vi.restoreAllMocks();
  });
});

describe("FindingsWindow — content sizing", () => {
  it("sizes the window to the row content (compact, not a fixed tall panel)", () => {
    render(<FindingsWindow />);
    const call = windowMock.resizeFindingsWindow.mock.calls.at(-1);
    expect(call?.[0]).toBe(380); // LIST_WIDTH
    expect(call?.[1]).toBeLessThan(300); // 2 rows → compact height, not the tall default
  });
});
