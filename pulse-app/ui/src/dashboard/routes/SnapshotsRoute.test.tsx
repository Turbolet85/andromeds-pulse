import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { SnapshotsRoute } from "./SnapshotsRoute";

describe("SnapshotsRoute", () => {
  it("renders <section> with id='tabpanel-snapshots' + aria-labelledby heading", () => {
    render(<SnapshotsRoute />);
    const section = screen.getByTestId("route-snapshots");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-snapshots");
    expect(section.getAttribute("aria-labelledby")).toBe("route-heading-snapshots");
  });

  it("renders empty-state placeholder pointing to Investigate flow", () => {
    render(<SnapshotsRoute />);
    expect(screen.getByTestId("route-empty-state")).toBeDefined();
    expect(
      screen.getByText(/no snapshots yet — investigate flow lands in chunks/i),
    ).toBeDefined();
  });

  it("includes a single h1 heading with route label", () => {
    render(<SnapshotsRoute />);
    expect(screen.getByRole("heading", { name: /snapshots/i, level: 1 })).toBeDefined();
  });
});
