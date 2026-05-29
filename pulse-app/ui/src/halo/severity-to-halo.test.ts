import { describe, expect, it } from "vitest";
import type { ConnectionState } from "../bindings/index";
import {
  BLUR_AUTONOMOUS_PX,
  BLUR_NONE_PX,
  BREATHING_ACTIVE_MS,
  BREATHING_IDLE_MS,
  DIM_FAILED,
  DIM_FULL,
  DIM_STALLED,
  HUE_AUTONOMOUS,
  HUE_NONE,
  activityStateToBreathingPeriodMs,
  connectionStateToDim,
  severityToBlurPx,
  severityToHueFraction,
} from "./severity-to-halo";

describe("severityToBlurPx — cumulative severity → blur envelope", () => {
  it("maps null (no active incidents) to the calm baseline", () => {
    expect(severityToBlurPx(null)).toBe(BLUR_NONE_PX);
  });

  it("scales monotonically curious < suggested < autonomous", () => {
    expect(severityToBlurPx("curious")).toBeLessThan(severityToBlurPx("suggested"));
    expect(severityToBlurPx("suggested")).toBeLessThan(severityToBlurPx("autonomous"));
  });

  it("maps autonomous to the maximum blur", () => {
    expect(severityToBlurPx("autonomous")).toBe(BLUR_AUTONOMOUS_PX);
  });
});

describe("severityToHueFraction — cumulative severity → LCH fraction", () => {
  it("maps null to the Earth-Blue end (0)", () => {
    expect(severityToHueFraction(null)).toBe(HUE_NONE);
  });

  it("maps autonomous to the Alert-Burgundy end (1)", () => {
    expect(severityToHueFraction("autonomous")).toBe(HUE_AUTONOMOUS);
  });

  it("scales monotonically curious < suggested < autonomous within [0,1]", () => {
    const c = severityToHueFraction("curious");
    const s = severityToHueFraction("suggested");
    const a = severityToHueFraction("autonomous");
    expect(c).toBeGreaterThanOrEqual(0);
    expect(c).toBeLessThan(s);
    expect(s).toBeLessThan(a);
    expect(a).toBeLessThanOrEqual(1);
  });
});

describe("activityStateToBreathingPeriodMs — activity → breathing pace", () => {
  it("breathes fastest when active and slowest when idle", () => {
    expect(activityStateToBreathingPeriodMs("active")).toBe(BREATHING_ACTIVE_MS);
    expect(activityStateToBreathingPeriodMs("idle")).toBe(BREATHING_IDLE_MS);
    expect(activityStateToBreathingPeriodMs("active")).toBeLessThan(
      activityStateToBreathingPeriodMs("quiet"),
    );
    expect(activityStateToBreathingPeriodMs("quiet")).toBeLessThan(
      activityStateToBreathingPeriodMs("idle"),
    );
  });

  it("keeps the period within the 2000–5000 ms band (≈0.2–0.5 Hz, < 3 Hz seizure threshold)", () => {
    for (const state of ["active", "quiet", "idle"] as const) {
      const periodMs = activityStateToBreathingPeriodMs(state);
      expect(periodMs).toBeGreaterThanOrEqual(2000);
      expect(periodMs).toBeLessThanOrEqual(5000);
    }
  });
});

describe("connectionStateToDim — connection health → grayout axis", () => {
  const cases: { state: ConnectionState; expected: number }[] = [
    { state: { state: "Receiving" }, expected: DIM_FULL },
    { state: { state: "Listening" }, expected: DIM_FULL },
    { state: { state: "Idle" }, expected: DIM_FULL },
    { state: { state: "Stalled" }, expected: DIM_STALLED },
    { state: { state: "ReceiverFailed" }, expected: DIM_FAILED },
  ];

  for (const { state, expected } of cases) {
    it(`maps ${state.state} to dim ${expected}`, () => {
      expect(connectionStateToDim(state)).toBe(expected);
    });
  }

  it("grays out (dim < full) only for degraded/failed receiver states", () => {
    expect(connectionStateToDim({ state: "Stalled" })).toBeLessThan(DIM_FULL);
    expect(connectionStateToDim({ state: "ReceiverFailed" })).toBeLessThan(
      connectionStateToDim({ state: "Stalled" }),
    );
  });
});
