import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { MetricsRoute } from "./MetricsRoute";

describe("MetricsRoute", () => {
  it("renders <section> with id='tabpanel-metrics' + aria-labelledby heading", () => {
    render(<MetricsRoute />);
    const section = screen.getByTestId("route-metrics");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-metrics");
    expect(section.getAttribute("aria-labelledby")).toBe("route-heading-metrics");
  });

  it("renders empty-state placeholder pointing to chunk #35", () => {
    render(<MetricsRoute />);
    expect(screen.getByTestId("route-empty-state")).toBeDefined();
    expect(
      screen.getByText(/no metrics yet — chunk #35 fills this view/i),
    ).toBeDefined();
  });

  it("includes a single h1 heading with route label", () => {
    render(<MetricsRoute />);
    expect(screen.getByRole("heading", { name: /metrics/i, level: 1 })).toBeDefined();
  });
});
