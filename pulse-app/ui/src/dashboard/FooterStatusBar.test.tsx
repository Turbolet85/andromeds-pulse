import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { FooterStatusBar } from "./FooterStatusBar";

describe("FooterStatusBar", () => {
  it("renders a <footer> landmark", () => {
    render(<FooterStatusBar />);
    const footer = screen.getByRole("contentinfo");
    expect(footer.tagName).toBe("FOOTER");
  });

  it("uses the data-testid 'dashboard-footer' (chunks #34/#35 fill content)", () => {
    render(<FooterStatusBar />);
    expect(screen.getByTestId("dashboard-footer")).toBeDefined();
  });
});
