import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { WindowControls } from "./WindowControls";

vi.mock("../hooks/use-window-controls", () => ({
  useWindowControls: () => ({
    minimize: vi.fn().mockResolvedValue(undefined),
    maximize: vi.fn().mockResolvedValue(undefined),
    close: vi.fn().mockResolvedValue(undefined),
  }),
}));

describe("WindowControls — platform-conditional rendering", () => {
  it("renders nothing on macOS (OS provides traffic-light buttons)", () => {
    const { container } = render(<WindowControls platform="macos" />);
    expect(container.firstChild).toBeNull();
  });

  it("renders three native <button> elements on Windows", () => {
    render(<WindowControls platform="windows" />);
    const buttons = screen.getAllByRole("button");
    expect(buttons).toHaveLength(3);
  });

  it("renders three native <button> elements on Linux", () => {
    render(<WindowControls platform="linux" />);
    const buttons = screen.getAllByRole("button");
    expect(buttons).toHaveLength(3);
  });

  it("each Windows/Linux button has aria-label per a11y plan", () => {
    render(<WindowControls platform="windows" />);
    expect(screen.getByRole("button", { name: "Minimize" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Maximize" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Close to tray" })).toBeDefined();
  });

  it("each button is a native <button>, not a styled div", () => {
    render(<WindowControls platform="windows" />);
    for (const btn of screen.getAllByRole("button")) {
      expect(btn.tagName).toBe("BUTTON");
    }
  });

  it("decorative span inside each button is aria-hidden", () => {
    render(<WindowControls platform="windows" />);
    for (const btn of screen.getAllByRole("button")) {
      const span = btn.querySelector("span");
      expect(span?.getAttribute("aria-hidden")).toBe("true");
    }
  });

  it("buttons are type=button (no implicit submit)", () => {
    render(<WindowControls platform="windows" />);
    for (const btn of screen.getAllByRole("button")) {
      expect(btn.getAttribute("type")).toBe("button");
    }
  });
});
