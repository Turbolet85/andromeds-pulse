import { beforeEach, describe, expect, it, vi } from "vitest";
import { act, render, screen } from "@testing-library/react";
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
  onReportOpen: vi.fn(),
  closeReportWindow: vi.fn(),
  reportOpenHandler: null as null | ((id: number) => void),
}));

vi.mock("../hooks/use-findings", () => ({
  useFindings: () => findingsMock.state,
}));

vi.mock("../hooks/use-findings-window", () => ({
  onReportOpen: (h: (id: number) => void) => {
    windowMock.reportOpenHandler = h;
    windowMock.onReportOpen();
    return () => {};
  },
  closeReportWindow: windowMock.closeReportWindow,
}));

vi.mock("../report/Report", () => ({
  Report: ({
    isOpen,
    incidentId,
    onClose,
  }: {
    isOpen: boolean;
    incidentId: number | null;
    onClose: () => void;
  }) =>
    isOpen ? (
      <button data-testid="report-stub" data-incident-id={incidentId} onClick={onClose}>
        report
      </button>
    ) : null,
}));

import { ReportWindow } from "./ReportWindow";

const ROWS: FindingsRow[] = [
  { id: 5, priorityTier: "autonomous", title: "a", openedAtUnixNano: 0 },
];

beforeEach(() => {
  windowMock.onReportOpen.mockClear();
  windowMock.closeReportWindow.mockReset();
  windowMock.reportOpenHandler = null;
  findingsMock.state.rows = [];
});

describe("ReportWindow", () => {
  it("renders nothing when there is no incident (no event, no active incidents)", () => {
    render(<ReportWindow />);
    expect(screen.queryByTestId("report-stub")).toBeNull();
  });

  it("falls back to the first active incident when opened without an event id", () => {
    findingsMock.state.rows = ROWS;
    render(<ReportWindow />);
    expect(screen.getByTestId("report-stub").getAttribute("data-incident-id")).toBe("5");
  });

  it("renders the report for the incident id delivered by the report-open event", () => {
    render(<ReportWindow />);
    act(() => {
      windowMock.reportOpenHandler?.(42);
    });
    expect(screen.getByTestId("report-stub").getAttribute("data-incident-id")).toBe("42");
  });

  it("closing the report calls closeReportWindow", async () => {
    findingsMock.state.rows = ROWS;
    const user = userEvent.setup();
    render(<ReportWindow />);
    await user.click(screen.getByTestId("report-stub"));
    expect(windowMock.closeReportWindow).toHaveBeenCalled();
  });
});
