import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { Fallback } from "./Fallback";

describe("Fallback — WebGPU-unavailable semantic announcer", () => {
  it("renders an element with role='alert'", () => {
    render(<Fallback />);
    expect(screen.getByRole("alert")).toBeDefined();
  });

  it("the alert element carries aria-live='assertive' (blocking failure announcement)", () => {
    render(<Fallback />);
    const alert = screen.getByRole("alert");
    expect(alert.getAttribute("aria-live")).toBe("assertive");
  });

  it("displays the canonical text 'WebGPU not supported in this browser'", () => {
    render(<Fallback />);
    expect(screen.getByText("WebGPU not supported in this browser")).toBeDefined();
  });

  it("uses --color-text-tertiary text color (no color-alone state signaling)", () => {
    render(<Fallback />);
    const alert = screen.getByRole("alert");
    expect(alert.style.color).toBe("var(--color-text-tertiary)");
  });

  it("uses --font-body font family", () => {
    render(<Fallback />);
    const alert = screen.getByRole("alert");
    expect(alert.style.fontFamily).toBe("var(--font-body)");
  });
});
