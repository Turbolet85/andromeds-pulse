// Halo State Pulse blur radius envelope. Returns the OUTER ENVELOPE (current
// target blur magnitude in CSS px). The mid-cycle sinusoidal sweep around the
// envelope happens in the WGSL fragment shader (per design-system §Motion
// High-impact moment #1 Halo State Pulse breathing).

const BLUR_MIN_PX = 4;
const BLUR_MAX_PX = 16;

export function errorRateToBlur(errorRate: number): number {
  if (!Number.isFinite(errorRate)) {
    return BLUR_MIN_PX;
  }
  const clamped = Math.max(0, Math.min(1, errorRate));
  return BLUR_MIN_PX + (BLUR_MAX_PX - BLUR_MIN_PX) * clamped;
}
