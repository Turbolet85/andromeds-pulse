import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { SkipToMain } from "./SkipToMain";

describe("SkipToMain", () => {
  it("renders an anchor pointing to #main-content", () => {
    render(<SkipToMain />);
    const link = screen.getByTestId("skip-to-main");
    expect(link.tagName).toBe("A");
    expect(link.getAttribute("href")).toBe("#main-content");
  });

  it("provides 'Skip to main content' accessible name", () => {
    render(<SkipToMain />);
    const link = screen.getByRole("link", { name: /skip to main content/i });
    expect(link).toBeDefined();
  });
});
