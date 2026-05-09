import { describe, expect, it } from "vitest";
import {
  ERROR_RATE_ACCENT_THRESHOLD,
  formatBadgeAriaLabel,
  formatErrorRatePercent,
  formatRetentionMinutes,
  formatServiceCount,
  formatSpansPerSec,
} from "./widget-types";

describe("ERROR_RATE_ACCENT_THRESHOLD", () => {
  it("is 5% so footer renders accent border at and above this rate", () => {
    expect(ERROR_RATE_ACCENT_THRESHOLD).toBe(0.05);
  });
});

describe("formatSpansPerSec", () => {
  it.each([
    [0, "0/s"],
    [45, "45/s"],
    [200, "200/s"],
    [999, "999/s"],
    [1000, "1.0k/s"],
    [1234, "1.2k/s"],
    [1800, "1.8k/s"],
    [9999, "10.0k/s"],
  ])("formats %d as %s", (input, expected) => {
    expect(formatSpansPerSec(input)).toBe(expected);
  });

  it.each([Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, -1, -100])(
    "returns placeholder for non-finite or negative input %s",
    (input) => {
      expect(formatSpansPerSec(input)).toBe("—");
    },
  );
});

describe("formatErrorRatePercent", () => {
  it.each([
    [0, "0.0%"],
    [0.012, "1.2%"],
    [0.05, "5.0%"],
    [0.07, "7.0%"],
    [0.1, "10.0%"],
    [1, "100.0%"],
  ])("formats %d as %s", (input, expected) => {
    expect(formatErrorRatePercent(input)).toBe(expected);
  });

  it.each([Number.NaN, Number.POSITIVE_INFINITY, -0.01])(
    "returns placeholder for non-finite or negative input %s",
    (input) => {
      expect(formatErrorRatePercent(input)).toBe("—");
    },
  );
});

describe("formatRetentionMinutes", () => {
  it.each([
    [0, 600, "0m / 10m"],
    [60, 600, "1m / 10m"],
    [480, 600, "8m / 10m"],
    [600, 600, "10m / 10m"],
  ])("formats (%d, %d) as %s", (used, max, expected) => {
    expect(formatRetentionMinutes(used, max)).toBe(expected);
  });

  it("returns placeholder when max is zero or negative", () => {
    expect(formatRetentionMinutes(60, 0)).toBe("—");
    expect(formatRetentionMinutes(60, -60)).toBe("—");
  });

  it("returns placeholder for non-finite inputs", () => {
    expect(formatRetentionMinutes(Number.NaN, 600)).toBe("—");
    expect(formatRetentionMinutes(60, Number.POSITIVE_INFINITY)).toBe("—");
  });

  it("returns placeholder for negative used", () => {
    expect(formatRetentionMinutes(-60, 600)).toBe("—");
  });
});

describe("formatServiceCount", () => {
  it.each([
    [0, "0"],
    [1, "1"],
    [24, "24"],
    [9999, "9999"],
  ])("formats %d as %s", (input, expected) => {
    expect(formatServiceCount(input)).toBe(expected);
  });

  it("floors fractional inputs", () => {
    expect(formatServiceCount(24.7)).toBe("24");
  });

  it("returns placeholder for non-finite or negative input", () => {
    expect(formatServiceCount(Number.NaN)).toBe("—");
    expect(formatServiceCount(-1)).toBe("—");
  });
});

describe("formatBadgeAriaLabel", () => {
  it("composes the canonical sentence shape", () => {
    expect(formatBadgeAriaLabel(24, 0.012)).toBe("24 services, 1.2% average error rate");
  });

  it("composes with zero error rate", () => {
    expect(formatBadgeAriaLabel(8, 0)).toBe("8 services, 0.0% average error rate");
  });

  it("composes placeholder for non-finite inputs", () => {
    expect(formatBadgeAriaLabel(Number.NaN, 0.012)).toBe("— services, 1.2% average error rate");
    expect(formatBadgeAriaLabel(24, Number.NaN)).toBe("24 services, — average error rate");
  });
});
