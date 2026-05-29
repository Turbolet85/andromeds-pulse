// LCH color-space interpolation between Halo State Pulse hue endpoints. LCH
// (CIELCh / OKLCH-equivalent) preserves perceptual lightness + chroma across
// the geodesic, avoiding the muddy gray midpoint produced by naive RGB lerp
// (~#896E96 for Earth Blue ↔ Alert Burgundy). Per design-system §Brand Identity
// Signature element + §Decisions Log 2026-05-03 LCH-chroma anchor.
//
// Output is sRGB float quadruple (0..1) ready for vec4<f32> WebGPU uniform
// upload. Per-channel clampUnit() defends against out-of-gamut intermediate
// hues that LCH→sRGB conversion can produce for some interpolation midpoints.

import Color from "colorjs.io";

export interface RgbaFloat {
  r: number;
  g: number;
  b: number;
  a: number;
}

export function lchInterpolate(
  severityFraction: number,
  primaryHex: string,
  accentHex: string,
): RgbaFloat {
  const t = !Number.isFinite(severityFraction) ? 0 : Math.max(0, Math.min(1, severityFraction));
  const primary = new Color(primaryHex);
  const accent = new Color(accentHex);
  const interp = primary.range(accent, { space: "lch" })(t);
  const srgb = interp.to("srgb");
  const [r, g, b] = srgb.coords;
  return {
    r: clampUnit(r),
    g: clampUnit(g),
    b: clampUnit(b),
    a: 1,
  };
}

function clampUnit(value: number | null): number {
  if (value === null || !Number.isFinite(value)) return 0;
  if (value < 0) return 0;
  if (value > 1) return 1;
  return value;
}
