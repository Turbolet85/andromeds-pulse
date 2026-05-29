// Maps the ingest throughput signal to a coarse ActivityState tier that
// drives the Halo breathing pace (chunk #90). Activity is the telemetry-flow
// axis — independent of connection health (a separate orthogonal axis per
// P-004) and of incident severity (the hue/blur axis). throughputHz arrives
// from use-widget-metrics (chunk #57; payload-rate proxy), so the tiering is
// coarse by construction; the breathing-pace mapping lives in severity-to-halo.

import type { ActivityState } from "./halo-types";

// Batches/sec at or above which flow counts as actively streaming. Below it
// (but non-zero) is a quiet trickle; zero / non-finite is idle.
export const ACTIVE_THROUGHPUT_HZ = 5;

export function throughputToActivityState(throughputHz: number): ActivityState {
  if (!Number.isFinite(throughputHz) || throughputHz <= 0) {
    return "idle";
  }
  if (throughputHz >= ACTIVE_THROUGHPUT_HZ) {
    return "active";
  }
  return "quiet";
}
