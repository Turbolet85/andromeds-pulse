import { describe, expect, it } from "vitest";
import { ACTIVE_THROUGHPUT_HZ, throughputToActivityState } from "./activity-state";

describe("throughputToActivityState — telemetry-flow tier", () => {
  it("returns 'idle' for zero throughput", () => {
    expect(throughputToActivityState(0)).toBe("idle");
  });

  it("returns 'idle' for non-finite throughput (jsdom / pre-init)", () => {
    expect(throughputToActivityState(Number.NaN)).toBe("idle");
    expect(throughputToActivityState(Number.POSITIVE_INFINITY)).toBe("idle");
  });

  it("returns 'idle' for negative throughput (clock-skew guard)", () => {
    expect(throughputToActivityState(-3)).toBe("idle");
  });

  it("returns 'quiet' for a non-zero trickle below the active threshold", () => {
    expect(throughputToActivityState(1)).toBe("quiet");
    expect(throughputToActivityState(ACTIVE_THROUGHPUT_HZ - 0.5)).toBe("quiet");
  });

  it("returns 'active' at or above the active threshold", () => {
    expect(throughputToActivityState(ACTIVE_THROUGHPUT_HZ)).toBe("active");
    expect(throughputToActivityState(ACTIVE_THROUGHPUT_HZ + 100)).toBe("active");
  });
});
