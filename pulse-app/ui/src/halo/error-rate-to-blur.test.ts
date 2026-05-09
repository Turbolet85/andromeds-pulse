import { describe, expect, it } from "vitest";
import { errorRateToBlur } from "./error-rate-to-blur";

describe("errorRateToBlur — outer envelope mapping (4-16 px)", () => {
  it.each([
    [0.0, 4],
    [0.25, 7],
    [0.5, 10],
    [0.75, 13],
    [1.0, 16],
  ])("error_rate %f maps to %f px", (input, expected) => {
    expect(errorRateToBlur(input)).toBeCloseTo(expected, 6);
  });

  it("clamps negative error_rate to lower bound 4 px", () => {
    expect(errorRateToBlur(-1)).toBe(4);
    expect(errorRateToBlur(-100)).toBe(4);
  });

  it("clamps error_rate above 1.0 to upper bound 16 px", () => {
    expect(errorRateToBlur(1.5)).toBe(16);
    expect(errorRateToBlur(100)).toBe(16);
  });

  it("collapses NaN to lower bound (non-finite guard)", () => {
    expect(errorRateToBlur(Number.NaN)).toBe(4);
  });

  it("collapses positive Infinity to lower bound (non-finite guard precedes clamp)", () => {
    expect(errorRateToBlur(Number.POSITIVE_INFINITY)).toBe(4);
  });

  it("collapses negative Infinity to lower bound", () => {
    expect(errorRateToBlur(Number.NEGATIVE_INFINITY)).toBe(4);
  });
});
