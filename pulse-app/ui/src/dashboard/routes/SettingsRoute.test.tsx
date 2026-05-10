import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";

const navigateMock = vi.hoisted(() => vi.fn());

vi.mock("@tanstack/react-router", () => ({
  useNavigate: () => navigateMock,
}));

vi.mock("./SettingsModalForm", () => ({
  SettingsModalForm: ({
    open,
    onClose,
  }: {
    open: boolean;
    onClose: () => void;
    triggerRef: unknown;
  }) =>
    open ? (
      <div role="dialog" aria-label="Settings" data-testid="settings-modal-stub">
        <button type="button" onClick={onClose} data-testid="modal-close-stub">
          close
        </button>
      </div>
    ) : null,
}));

import { SettingsRoute } from "./SettingsRoute";

describe("SettingsRoute", () => {
  beforeEach(() => {
    navigateMock.mockClear();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it("renders <section> with id='tabpanel-settings' + aria-labelledby heading", () => {
    render(<SettingsRoute />);
    const section = screen.getByTestId("route-settings");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-settings");
    expect(section.getAttribute("aria-labelledby")).toBe("route-heading-settings");
  });

  it("renders SettingsModalForm with open=true", () => {
    render(<SettingsRoute />);
    expect(screen.getByTestId("settings-modal-stub")).toBeDefined();
  });

  it("includes a single h1 heading with route label", () => {
    render(<SettingsRoute />);
    expect(
      screen.getByRole("heading", { name: /settings/i, level: 1 }),
    ).toBeDefined();
  });

  it("modal onClose navigates back to /traces (default tab)", () => {
    render(<SettingsRoute />);
    const closeButton = screen.getByTestId("modal-close-stub");
    closeButton.click();
    expect(navigateMock).toHaveBeenCalledWith({ to: "/traces" });
  });
});
