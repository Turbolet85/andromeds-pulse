import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { SettingsRoute } from "./SettingsRoute";

describe("SettingsRoute", () => {
  it("renders <section> with id='tabpanel-settings' + aria-labelledby heading", () => {
    render(<SettingsRoute />);
    const section = screen.getByTestId("route-settings");
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("id")).toBe("tabpanel-settings");
    expect(section.getAttribute("aria-labelledby")).toBe("route-heading-settings");
  });

  it("renders empty-state placeholder pointing to chunk #38", () => {
    render(<SettingsRoute />);
    expect(screen.getByTestId("route-empty-state")).toBeDefined();
    expect(screen.getByText(/settings form lands in chunk #38/i)).toBeDefined();
  });

  it("includes a single h1 heading with route label", () => {
    render(<SettingsRoute />);
    expect(screen.getByRole("heading", { name: /settings/i, level: 1 })).toBeDefined();
  });
});
