import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { LogsRoute } from "./LogsRoute";

describe("LogsRoute", () => {
  it("renders <section> with id='tabpanel-logs' + aria-labelledby heading", () => {
    render(<LogsRoute />);
    const section = screen.getByTestId("route-logs");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-logs");
    expect(section.getAttribute("aria-labelledby")).toBe("route-heading-logs");
  });

  it("renders empty-state placeholder pointing to chunk #35", () => {
    render(<LogsRoute />);
    expect(screen.getByTestId("route-empty-state")).toBeDefined();
    expect(
      screen.getByText(/no logs yet — chunk #35 fills this view/i),
    ).toBeDefined();
  });

  it("includes a single h1 heading with route label", () => {
    render(<LogsRoute />);
    expect(screen.getByRole("heading", { name: /logs/i, level: 1 })).toBeDefined();
  });
});
