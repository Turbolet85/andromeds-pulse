// P-004 orthogonality assert (chunk #100 capability-audit gap-fill): the
// three Halo input axes are independent per design-system Decisions Log
// 2026-05-29 — severity drives hue fraction + blur envelope, connection
// drives the grayout dim, activity drives the breathing period. Composing
// the full shader-input tuple across the complete state matrix, each
// derived input must be a function of its own axis ONLY.

import { describe, expect, it } from "vitest";
import type { ConnectionState, PriorityTier } from "../bindings/index";
import type { ActivityState } from "./halo-types";
import {
  BLUR_AUTONOMOUS_PX,
  BLUR_CURIOUS_PX,
  BLUR_NONE_PX,
  BLUR_SUGGESTED_PX,
  DIM_FAILED,
  DIM_FULL,
  DIM_STALLED,
  HUE_AUTONOMOUS,
  HUE_CURIOUS,
  HUE_NONE,
  HUE_SUGGESTED,
  activityStateToBreathingPeriodMs,
  connectionStateToDim,
  severityToBlurPx,
  severityToHueFraction,
} from "./severity-to-halo";

const SEVERITIES: (PriorityTier | null)[] = [
  null,
  "curious",
  "suggested",
  "autonomous",
];
const CONNECTIONS: ConnectionState[] = [
  { state: "Listening" },
  { state: "Receiving" },
  { state: "Idle" },
  { state: "Stalled" },
  { state: "ReceiverFailed" },
];
const ACTIVITIES: ActivityState[] = ["active", "quiet", "idle"];

interface HaloInputs {
  hue: number;
  blur: number;
  dim: number;
  periodMs: number;
}

function composeInputs(
  tier: PriorityTier | null,
  conn: ConnectionState,
  activity: ActivityState,
): HaloInputs {
  return {
    hue: severityToHueFraction(tier),
    blur: severityToBlurPx(tier),
    dim: connectionStateToDim(conn),
    periodMs: activityStateToBreathingPeriodMs(activity),
  };
}

describe("P-004 halo axis orthogonality (severity ⊥ connection ⊥ activity)", () => {
  it("hue and blur depend only on severity across the full state matrix", () => {
    for (const tier of SEVERITIES) {
      const reference = composeInputs(tier, CONNECTIONS[0], ACTIVITIES[0]);
      for (const conn of CONNECTIONS) {
        for (const activity of ACTIVITIES) {
          const inputs = composeInputs(tier, conn, activity);
          expect(inputs.hue).toBe(reference.hue);
          expect(inputs.blur).toBe(reference.blur);
        }
      }
    }
  });

  it("grayout dim depends only on connection state across the full state matrix", () => {
    for (const conn of CONNECTIONS) {
      const reference = composeInputs(SEVERITIES[0], conn, ACTIVITIES[0]);
      for (const tier of SEVERITIES) {
        for (const activity of ACTIVITIES) {
          const inputs = composeInputs(tier, conn, activity);
          expect(inputs.dim).toBe(reference.dim);
        }
      }
    }
  });

  it("breathing period depends only on activity state across the full state matrix", () => {
    for (const activity of ACTIVITIES) {
      const reference = composeInputs(SEVERITIES[0], CONNECTIONS[0], activity);
      for (const tier of SEVERITIES) {
        for (const conn of CONNECTIONS) {
          const inputs = composeInputs(tier, conn, activity);
          expect(inputs.periodMs).toBe(reference.periodMs);
        }
      }
    }
  });

  it("severity axis values trace to the Decisions Log 2026-05-29 table", () => {
    expect(severityToHueFraction(null)).toBe(HUE_NONE);
    expect(severityToHueFraction("curious")).toBe(HUE_CURIOUS);
    expect(severityToHueFraction("suggested")).toBe(HUE_SUGGESTED);
    expect(severityToHueFraction("autonomous")).toBe(HUE_AUTONOMOUS);
    expect(severityToBlurPx(null)).toBe(BLUR_NONE_PX);
    expect(severityToBlurPx("curious")).toBe(BLUR_CURIOUS_PX);
    expect(severityToBlurPx("suggested")).toBe(BLUR_SUGGESTED_PX);
    expect(severityToBlurPx("autonomous")).toBe(BLUR_AUTONOMOUS_PX);
  });

  it("connection axis values cover healthy, stalled, and failed dim levels", () => {
    expect(connectionStateToDim({ state: "Receiving" })).toBe(DIM_FULL);
    expect(connectionStateToDim({ state: "Listening" })).toBe(DIM_FULL);
    expect(connectionStateToDim({ state: "Idle" })).toBe(DIM_FULL);
    expect(connectionStateToDim({ state: "Stalled" })).toBe(DIM_STALLED);
    expect(connectionStateToDim({ state: "ReceiverFailed" })).toBe(DIM_FAILED);
  });
});
