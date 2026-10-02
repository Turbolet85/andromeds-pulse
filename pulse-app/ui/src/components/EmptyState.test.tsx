import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { EmptyState } from "./EmptyState";

describe("EmptyState", () => {
  it("renders the message", () => {
    render(<EmptyState message="No data yet" />);
    expect(screen.getByText("No data yet")).toBeDefined();
  });

  it("renders the hint when provided, including the port literals", () => {
    render(
      <EmptyState
        message="No metrics received yet"
        hint={
          <>
            Point an OTLP metrics exporter at <code>:4318</code> / <code>:4317</code>
          </>
        }
        testId="metrics-empty-state"
      />,
    );
    const hint = screen.getByTestId("metrics-empty-state-hint");
    expect(hint.textContent).toContain(":4318");
    expect(hint.textContent).toContain(":4317");
  });

  it("omits the hint node when no hint is passed", () => {
    render(<EmptyState message="No snapshots yet" testId="snap" />);
    expect(screen.queryByTestId("snap-hint")).toBeNull();
  });

  it("renders the glyph as a decorative aria-hidden svg (not role=img)", () => {
    const { container } = render(<EmptyState message="x" />);
    const svg = container.querySelector("svg");
    expect(svg).not.toBeNull();
    expect(svg?.getAttribute("aria-hidden")).toBe("true");
    expect(svg?.getAttribute("role")).toBeNull();
  });

  it("uses --color-text-secondary (>=4.5:1 body text), not tertiary/muted", () => {
    render(<EmptyState message="x" testId="es" />);
    expect(screen.getByTestId("es").style.color).toBe("var(--color-text-secondary)");
  });

  it("is a non-interactive plain-text panel — no live region, no focusable element", () => {
    const { container } = render(<EmptyState message="x" testId="es" />);
    const root = screen.getByTestId("es");
    expect(root.getAttribute("role")).toBeNull();
    expect(root.getAttribute("aria-live")).toBeNull();
    expect(container.querySelector("button, a[href], input, [tabindex]")).toBeNull();
  });

  it("honors a custom glyph and testId", () => {
    render(<EmptyState message="x" glyph="aperture" testId="custom" />);
    expect(screen.getByTestId("custom")).toBeDefined();
  });
});
