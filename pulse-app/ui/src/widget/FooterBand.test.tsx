import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { FooterBand } from "./FooterBand";

const baseProps = {
  throughputHz: 1234,
  errorRate: 0.012,
  retentionUsedSeconds: 480,
  retentionMaxSeconds: 600,
};

describe("FooterBand — live-region semantics", () => {
  it("wraps in <footer role=status> with aria-live=polite", () => {
    render(<FooterBand {...baseProps} />);
    const footer = screen.getByRole("status", { name: /Live telemetry summary/ });
    expect(footer.tagName).toBe("FOOTER");
    expect(footer.getAttribute("aria-live")).toBe("polite");
  });

  it("aria-label is 'Live telemetry summary' (canonical phrasing)", () => {
    render(<FooterBand {...baseProps} />);
    const footer = screen.getByRole("status");
    expect(footer.getAttribute("aria-label")).toBe("Live telemetry summary");
  });
});

describe("FooterBand — 3 metrics rendered", () => {
  it("renders Ingest label + value", () => {
    render(<FooterBand {...baseProps} />);
    expect(screen.getByText("Ingest")).toBeDefined();
    expect(screen.getByText("1.2k/s")).toBeDefined();
  });

  it("renders Error label + percent value", () => {
    render(<FooterBand {...baseProps} />);
    expect(screen.getByText("Error")).toBeDefined();
    expect(screen.getByText("1.2%")).toBeDefined();
  });

  it("renders Retention label + ratio value", () => {
    render(<FooterBand {...baseProps} />);
    expect(screen.getByText("Retention")).toBeDefined();
    expect(screen.getByText("8m / 10m")).toBeDefined();
  });

  it("uses dt/dd semantic structure for each label/value pair", () => {
    const { container } = render(<FooterBand {...baseProps} />);
    expect(container.querySelectorAll("dt").length).toBe(3);
    expect(container.querySelectorAll("dd").length).toBe(3);
  });
});

describe("FooterBand — error-rate accent threshold", () => {
  it("error-rate < 5% uses transparent border (no accent)", () => {
    render(<FooterBand {...baseProps} errorRate={0.02} />);
    const value = screen.getByTestId("footer-error-value");
    expect(value.style.border).toContain("transparent");
  });

  it("error-rate ≥ 5% pairs --color-accent border with --color-text-primary text", () => {
    render(<FooterBand {...baseProps} errorRate={0.07} />);
    const value = screen.getByTestId("footer-error-value");
    expect(value.style.border).toContain("--color-accent");
    expect(value.style.color).toContain("--color-text-primary");
  });

  it("error-rate at exactly 5% threshold triggers accent (>= comparison)", () => {
    render(<FooterBand {...baseProps} errorRate={0.05} />);
    const value = screen.getByTestId("footer-error-value");
    expect(value.style.border).toContain("--color-accent");
  });
});

describe("FooterBand — typography tokens", () => {
  it("numeric values render in --font-code with tabular-nums", () => {
    render(<FooterBand {...baseProps} />);
    const ingest = screen.getByText("1.2k/s");
    expect(ingest.style.fontFamily).toContain("--font-code");
    expect(ingest.style.fontVariantNumeric).toBe("tabular-nums");
  });

  it("labels render in --font-body Label role (12px / 500)", () => {
    render(<FooterBand {...baseProps} />);
    const label = screen.getByText("Ingest");
    expect(label.style.fontFamily).toContain("--font-body");
    expect(label.style.fontSize).toBe("12px");
    expect(label.style.fontWeight).toBe("500");
  });
});

describe("FooterBand — chrome", () => {
  it("border-top uses Subtle Earth Blue 0.3 opacity", () => {
    render(<FooterBand {...baseProps} />);
    const footer = screen.getByRole("status");
    expect(footer.style.borderTop).toContain("rgba(74, 144, 226, 0.3)");
  });

  it("padding follows spacing-sm (vertical) + spacing-md (horizontal) tokens", () => {
    render(<FooterBand {...baseProps} />);
    const footer = screen.getByRole("status");
    expect(footer.style.padding).toContain("--spacing-sm");
    expect(footer.style.padding).toContain("--spacing-md");
  });

  it("flex-column layout for the 2-line wireframe", () => {
    render(<FooterBand {...baseProps} />);
    const footer = screen.getByRole("status");
    expect(footer.style.display).toBe("flex");
    expect(footer.style.flexDirection).toBe("column");
  });
});
