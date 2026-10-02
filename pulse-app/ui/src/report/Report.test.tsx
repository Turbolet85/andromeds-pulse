// ErrorState DOM-shape pins (CARRY: the accent-as-error-text class the
// 2026-08-23 a11y pass swept at three other sites and missed here). Renders
// the real <Report> composition — Modal supplies the tabbable close button
// the focus trap requires — with the hooks stubbed to the load-error state.

import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { createRef } from "react";

const mocks = vi.hoisted(() => ({
  useReport: vi.fn(),
  useMcpDelivery: vi.fn(),
}));

vi.mock("./use-report", () => ({ useReport: mocks.useReport }));
vi.mock("./use-mcp-delivery", () => ({ useMcpDelivery: mocks.useMcpDelivery }));

import { Report } from "./Report";

afterEach(() => {
  mocks.useReport.mockReset();
  mocks.useMcpDelivery.mockReset();
});

function renderLoadError() {
  mocks.useReport.mockReturnValue({
    report: null,
    loading: false,
    error: "Failed to load report",
    copyState: "idle",
    copyMarkdown: vi.fn(),
  });
  mocks.useMcpDelivery.mockReturnValue({
    available: false,
    sendState: "idle",
    send: vi.fn(),
  });
  const triggerRef = createRef<HTMLElement>();
  return render(
    <Report isOpen onClose={() => {}} incidentId={3} triggerRef={triggerRef} />,
  );
}

describe("Report load-error state", () => {
  it("renders body text in the >=4.5:1 text token, never accent", () => {
    renderLoadError();
    const error = screen.getByTestId("report-error");
    expect(error.style.color).toBe("var(--color-text-primary)");
  });

  it("keeps the accent on the border only", () => {
    renderLoadError();
    const error = screen.getByTestId("report-error");
    expect(error.style.border).toBe("1px solid var(--color-accent)");
    expect(error.style.color).not.toBe("var(--color-accent)");
  });

  it("conveys the failure with a text label and role=alert, never color alone", () => {
    renderLoadError();
    const error = screen.getByTestId("report-error");
    expect(error.getAttribute("role")).toBe("alert");
    expect(error.textContent).toMatch(/could not load diagnostic report/i);
  });
});
