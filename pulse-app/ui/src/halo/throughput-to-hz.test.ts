import { describe, expect, it } from "vitest";
import { throughputToHz } from "./throughput-to-hz";

describe("throughputToHz — pulse frequency clamp (frontend owns clamp)", () => {
  it.each([
    [0, 0.8],
    [100, 0.8],
    [799, 0.8],
    [800, 0.8],
    [1000, 1.0],
    [2000, 2.0],
    [2400, 2.4],
    [5000, 2.4],
    [10000, 2.4],
    [99999, 2.4],
  ])("throughput %d Hz maps to %d Hz pulse", (input, expected) => {
    expect(throughputToHz(input)).toBeCloseTo(expected, 6);
  });

  it("collapses NaN to lower bound (avoid producing non-finite shader uniform)", () => {
    expect(throughputToHz(Number.NaN)).toBe(0.8);
  });

  it("collapses positive Infinity to lower bound (non-finite guard precedes clamp)", () => {
    expect(throughputToHz(Number.POSITIVE_INFINITY)).toBe(0.8);
  });

  it("collapses negative Infinity to lower bound", () => {
    expect(throughputToHz(Number.NEGATIVE_INFINITY)).toBe(0.8);
  });

  it("clamps negative throughput to lower bound", () => {
    expect(throughputToHz(-1)).toBe(0.8);
    expect(throughputToHz(-100000)).toBe(0.8);
  });

  it("upper bound MUST stay ≤ 2.4 Hz to satisfy WCAG SC 2.3.1 three-flashes safety (< 3 Hz)", () => {
    // Boundary fuzz across the danger threshold — every input in this range
    // MUST clamp at or below the 2.4 Hz design-spec maximum so that no code
    // path can ever produce ≥ 3 Hz pulse rate (a11y-plan §6 Motion tokens).
    const fuzzInputs = [3000, 3001, 5000, 9999, 10000, 100000, 1_000_000];
    for (const input of fuzzInputs) {
      expect(throughputToHz(input)).toBeLessThan(3);
      expect(throughputToHz(input)).toBeLessThanOrEqual(2.4);
    }
  });
});
