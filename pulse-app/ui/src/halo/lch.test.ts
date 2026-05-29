import { describe, expect, it } from "vitest";
import { lchInterpolate } from "./lch";

const EARTH_BLUE = "#4A90E2";
const ALERT_BURGUNDY = "#C7556A";

describe("lchInterpolate — LCH (not RGB) color interpolation", () => {
  it("fraction=0 returns approximately Earth Blue (sRGB float)", () => {
    const result = lchInterpolate(0, EARTH_BLUE, ALERT_BURGUNDY);
    expect(result.r).toBeCloseTo(74 / 255, 2);
    expect(result.g).toBeCloseTo(144 / 255, 2);
    expect(result.b).toBeCloseTo(226 / 255, 2);
    expect(result.a).toBe(1);
  });

  it("fraction=1 returns approximately Alert Burgundy (sRGB float)", () => {
    const result = lchInterpolate(1, EARTH_BLUE, ALERT_BURGUNDY);
    expect(result.r).toBeCloseTo(199 / 255, 2);
    expect(result.g).toBeCloseTo(85 / 255, 2);
    expect(result.b).toBeCloseTo(106 / 255, 2);
    expect(result.a).toBe(1);
  });

  it("fraction=0.5 midpoint MUST land on LCH geodesic, NOT naive RGB midpoint (~#896E96)", () => {
    const midpoint = lchInterpolate(0.5, EARTH_BLUE, ALERT_BURGUNDY);
    // RGB midpoint of Earth Blue (#4A90E2) and Alert Burgundy (#C7556A):
    //   r = (0.290 + 0.780) / 2 ≈ 0.535
    //   g = (0.565 + 0.333) / 2 ≈ 0.449
    //   b = (0.886 + 0.416) / 2 ≈ 0.651
    // LCH midpoint preserves perceptual lightness/chroma rather than naive
    // RGB averaging — the result MUST measurably differ from the RGB midpoint
    // in at least one channel by ≥ 0.05 distance.
    const rgbMidpoint = { r: 0.535, g: 0.449, b: 0.651 };
    const distanceR = Math.abs(midpoint.r - rgbMidpoint.r);
    const distanceG = Math.abs(midpoint.g - rgbMidpoint.g);
    const distanceB = Math.abs(midpoint.b - rgbMidpoint.b);
    const maxDistance = Math.max(distanceR, distanceG, distanceB);
    expect(maxDistance).toBeGreaterThan(0.05);
  });

  it("clamps fraction below 0 to lower bound (returns Earth Blue)", () => {
    const result = lchInterpolate(-1, EARTH_BLUE, ALERT_BURGUNDY);
    expect(result.r).toBeCloseTo(74 / 255, 2);
    expect(result.g).toBeCloseTo(144 / 255, 2);
    expect(result.b).toBeCloseTo(226 / 255, 2);
  });

  it("clamps fraction above 1 to upper bound (returns Alert Burgundy)", () => {
    const result = lchInterpolate(1.5, EARTH_BLUE, ALERT_BURGUNDY);
    expect(result.r).toBeCloseTo(199 / 255, 2);
    expect(result.g).toBeCloseTo(85 / 255, 2);
    expect(result.b).toBeCloseTo(106 / 255, 2);
  });

  it("collapses NaN fraction to lower bound (non-finite guard)", () => {
    const result = lchInterpolate(Number.NaN, EARTH_BLUE, ALERT_BURGUNDY);
    expect(result.r).toBeCloseTo(74 / 255, 2);
    expect(result.g).toBeCloseTo(144 / 255, 2);
    expect(result.b).toBeCloseTo(226 / 255, 2);
  });

  it("returns alpha = 1.0 (always opaque) regardless of fraction", () => {
    expect(lchInterpolate(0, EARTH_BLUE, ALERT_BURGUNDY).a).toBe(1);
    expect(lchInterpolate(0.25, EARTH_BLUE, ALERT_BURGUNDY).a).toBe(1);
    expect(lchInterpolate(0.5, EARTH_BLUE, ALERT_BURGUNDY).a).toBe(1);
    expect(lchInterpolate(0.75, EARTH_BLUE, ALERT_BURGUNDY).a).toBe(1);
    expect(lchInterpolate(1, EARTH_BLUE, ALERT_BURGUNDY).a).toBe(1);
  });

  it("clamps output channels to [0, 1] sRGB displayable range (gamut clipping)", () => {
    // LCH→sRGB conversion may produce out-of-gamut values for some interpolation
    // midpoints; clampUnit brings them back into displayable range so the
    // shader uniform never receives negative or >1 values.
    const samples = [0, 0.1, 0.25, 0.5, 0.75, 0.9, 1];
    for (const t of samples) {
      const result = lchInterpolate(t, EARTH_BLUE, ALERT_BURGUNDY);
      expect(result.r).toBeGreaterThanOrEqual(0);
      expect(result.r).toBeLessThanOrEqual(1);
      expect(result.g).toBeGreaterThanOrEqual(0);
      expect(result.g).toBeLessThanOrEqual(1);
      expect(result.b).toBeGreaterThanOrEqual(0);
      expect(result.b).toBeLessThanOrEqual(1);
    }
  });
});
