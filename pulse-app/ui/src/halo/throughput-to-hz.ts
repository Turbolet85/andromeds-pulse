// Halo State Pulse frequency mapper. Frontend owns the clamp before pulse-cycle
// emission per design-system §Brand Identity Signature element + §Downstream
// Readiness obs-handoff (frontend clamps; obs does NOT re-clamp).

const HZ_MIN = 0.8;
const HZ_MAX = 2.4;
const HZ_DIVISOR = 1000;

export function throughputToHz(throughputHz: number): number {
  if (!Number.isFinite(throughputHz)) {
    return HZ_MIN;
  }
  const raw = throughputHz / HZ_DIVISOR;
  if (raw < HZ_MIN) return HZ_MIN;
  if (raw > HZ_MAX) return HZ_MAX;
  return raw;
}
