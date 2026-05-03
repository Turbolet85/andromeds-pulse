import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import type { FC } from "react";
import {
  Aperture,
  CircularPulse,
  ConstellationGrid,
  Icon,
  Star,
  Telescope,
} from "./index";
import type { GlyphName, IconProps } from "./types";

const glyphs: Array<[GlyphName, FC<IconProps>]> = [
  ["aperture", Aperture],
  ["telescope", Telescope],
  ["constellation-grid", ConstellationGrid],
  ["star", Star],
  ["circular-pulse", CircularPulse],
];

describe("Icon registry — per-glyph render contract", () => {
  for (const [name, Component] of glyphs) {
    describe(name, () => {
      it("renders an <svg> with viewBox 0 0 24 24 and focusable=false", () => {
        const { container } = render(<Component />);
        const svg = container.querySelector("svg");
        expect(svg).not.toBeNull();
        expect(svg!.getAttribute("viewBox")).toBe("0 0 24 24");
        expect(svg!.getAttribute("focusable")).toBe("false");
      });

      it("defaults to aria-hidden=true (decorative)", () => {
        const { container } = render(<Component />);
        const svg = container.querySelector("svg")!;
        expect(svg.getAttribute("aria-hidden")).toBe("true");
        expect(svg.getAttribute("role")).toBeNull();
        expect(svg.getAttribute("aria-label")).toBeNull();
      });

      it("flips to role=img + aria-label when name provided (meaningful)", () => {
        const { container } = render(
          <Component aria-label="Open settings" />,
        );
        const svg = container.querySelector("svg")!;
        expect(svg.getAttribute("aria-hidden")).toBeNull();
        expect(svg.getAttribute("role")).toBe("img");
        expect(svg.getAttribute("aria-label")).toBe("Open settings");
      });

      it("renders at default size 24 when no size prop", () => {
        const { container } = render(<Component />);
        const svg = container.querySelector("svg")!;
        expect(svg.getAttribute("width")).toBe("24");
        expect(svg.getAttribute("height")).toBe("24");
      });

      it("respects size prop (16, 20, 24)", () => {
        for (const size of [16, 20, 24] as const) {
          const { container, unmount } = render(<Component size={size} />);
          const svg = container.querySelector("svg")!;
          expect(svg.getAttribute("width")).toBe(String(size));
          expect(svg.getAttribute("height")).toBe(String(size));
          unmount();
        }
      });

      it("passes through className prop", () => {
        const { container } = render(
          <Component className="text-text-secondary" />,
        );
        const svg = container.querySelector("svg")!;
        expect(svg.getAttribute("class")).toBe("text-text-secondary");
      });

      it("uses currentColor for stroke (no hardcoded hex)", () => {
        const { container } = render(<Component />);
        const svg = container.querySelector("svg")!;
        expect(svg.getAttribute("stroke")).toBe("currentColor");
        const elementsWithStroke = svg.querySelectorAll("[stroke]");
        for (const el of elementsWithStroke) {
          const stroke = el.getAttribute("stroke");
          expect(stroke).toMatch(/^(currentColor|none)$/);
        }
      });

      it("uses currentColor or none for fill (no hardcoded hex)", () => {
        const { container } = render(<Component />);
        const svg = container.querySelector("svg")!;
        expect(svg.getAttribute("fill")).toBe("none");
        const elementsWithFill = svg.querySelectorAll("[fill]");
        for (const el of elementsWithFill) {
          const fill = el.getAttribute("fill");
          expect(fill).toMatch(/^(currentColor|none)$/);
        }
      });

      it("contains no inline animation tags", () => {
        const { container } = render(<Component />);
        const svg = container.querySelector("svg")!;
        expect(svg.querySelector("animate")).toBeNull();
        expect(svg.querySelector("animateTransform")).toBeNull();
        expect(svg.querySelector("animateMotion")).toBeNull();
        expect(svg.querySelector("set")).toBeNull();
      });

      it("renders <title> child when title prop provided", () => {
        const { container } = render(<Component title="Aperture glyph" />);
        const titleEl = container.querySelector("svg > title");
        expect(titleEl).not.toBeNull();
        expect(titleEl!.textContent).toBe("Aperture glyph");
      });
    });
  }
});

describe("Icon dispatcher — glyph prop maps to per-glyph component", () => {
  for (const [name, Component] of glyphs) {
    it(`dispatches glyph='${name}' to the matching component`, () => {
      const dispatched = render(<Icon glyph={name} />);
      const direct = render(<Component />);
      expect(dispatched.container.innerHTML).toBe(direct.container.innerHTML);
    });
  }

  it("forwards aria-label through dispatcher (decorative→meaningful flip)", () => {
    const { container } = render(
      <Icon glyph="aperture" aria-label="View settings" />,
    );
    const svg = container.querySelector("svg")!;
    expect(svg.getAttribute("role")).toBe("img");
    expect(svg.getAttribute("aria-label")).toBe("View settings");
    expect(svg.getAttribute("aria-hidden")).toBeNull();
  });

  it("forwards size through dispatcher", () => {
    const { container } = render(<Icon glyph="telescope" size={16} />);
    const svg = container.querySelector("svg")!;
    expect(svg.getAttribute("width")).toBe("16");
    expect(svg.getAttribute("height")).toBe("16");
  });
});
