// Pure render-parameter mappers for the Halo State Pulse (chunk #90),
// replacing the legacy error-rate-to-blur / throughput-to-hz helpers. Three
// orthogonal axes (per design-system §Motion + Decisions Log 2026-05-29):
//   - cumulative incident severity (PriorityTier | null) → hue fraction + blur
//   - activity state → breathing period
//   - connection state → grayout/desaturation dim
// PriorityTier is the cumulative-attention signal exposed by use-findings
// (severityMax); null means no active incidents (calmest baseline).

import type { ConnectionState, PriorityTier } from "../bindings/index";
import type { ActivityState } from "./halo-types";

// Outer blur-envelope magnitude in CSS px, per the chunk #90 severity table
// (None=4, Curious=8, Suggested=12, Autonomous=16). The mid-cycle sinusoidal
// sweep around this envelope happens in the WGSL fragment shader.
export const BLUR_NONE_PX = 4;
export const BLUR_CURIOUS_PX = 8;
export const BLUR_SUGGESTED_PX = 12;
export const BLUR_AUTONOMOUS_PX = 16;

export function severityToBlurPx(tier: PriorityTier | null): number {
  if (tier === null) return BLUR_NONE_PX;
  switch (tier) {
    case "autonomous":
      return BLUR_AUTONOMOUS_PX;
    case "suggested":
      return BLUR_SUGGESTED_PX;
    case "curious":
      return BLUR_CURIOUS_PX;
  }
}

// LCH interpolation fraction (0 = Earth Blue, 1 = Alert Burgundy) for the hue
// axis. None/Curious sit near the calm Earth-Blue end; Autonomous reaches the
// full Alert-Burgundy end.
export const HUE_NONE = 0;
export const HUE_CURIOUS = 0.33;
export const HUE_SUGGESTED = 0.67;
export const HUE_AUTONOMOUS = 1;

export function severityToHueFraction(tier: PriorityTier | null): number {
  if (tier === null) return HUE_NONE;
  switch (tier) {
    case "autonomous":
      return HUE_AUTONOMOUS;
    case "suggested":
      return HUE_SUGGESTED;
    case "curious":
      return HUE_CURIOUS;
  }
}

// Breathing period in ms per activity tier (the 4–5 s quiet → ~2 s active
// band per design-system Decisions Log 2026-05-29; HaloCanvas converts
// period → pulse-phase rate).
export const BREATHING_ACTIVE_MS = 2000;
export const BREATHING_QUIET_MS = 3500;
export const BREATHING_IDLE_MS = 4500;

export function activityStateToBreathingPeriodMs(state: ActivityState): number {
  switch (state) {
    case "active":
      return BREATHING_ACTIVE_MS;
    case "quiet":
      return BREATHING_QUIET_MS;
    case "idle":
      return BREATHING_IDLE_MS;
  }
}

// Connection-health grayout factor [0,1] for the orthogonal saturation axis
// (P-004): 1 = full color (receiver healthy), lower = desaturated toward gray
// as the receiver degrades. Independent of severity hue + activity rhythm.
export const DIM_FULL = 1;
export const DIM_STALLED = 0.5;
export const DIM_FAILED = 0.25;

export function connectionStateToDim(state: ConnectionState): number {
  switch (state.state) {
    case "Receiving":
    case "Listening":
    case "Idle":
      return DIM_FULL;
    case "Stalled":
      return DIM_STALLED;
    case "ReceiverFailed":
      return DIM_FAILED;
  }
}
