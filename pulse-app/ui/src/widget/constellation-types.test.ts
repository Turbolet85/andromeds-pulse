import { describe, expect, it } from "vitest";
import type { ServiceLifecycleState, ServiceListItem } from "../bindings/index";
import {
  constellationSummary,
  hashServiceName,
  lifecycleToBrightness,
  MAX_CONSTELLATION_DOTS,
  scatterPosition,
  visibleDots,
} from "./constellation-types";

function item(
  service: string,
  state: ServiceLifecycleState,
  priorityTier: ServiceListItem["priority_tier"] = null,
): ServiceListItem {
  return {
    service,
    state,
    last_seen_unix_nano: 1_000,
    manual_override: null,
    priority_tier: priorityTier,
  };
}

describe("scatterPosition — deterministic seeded scatter", () => {
  it("is stable across calls (cross-session consistency)", () => {
    const a = scatterPosition("checkout-service");
    const b = scatterPosition("checkout-service");
    expect(a).toEqual(b);
  });

  it("produces distinct positions for distinct names", () => {
    const names = ["alpha", "bravo", "charlie", "delta", "echo"];
    const keys = names.map((n) => {
      const { x, y } = scatterPosition(n);
      return `${x.toFixed(6)},${y.toFixed(6)}`;
    });
    expect(new Set(keys).size).toBe(names.length);
  });

  it("stays within the [-0.85, 0.85] scatter extent", () => {
    for (const n of ["a", "service-with-a-long-name", "x", ""]) {
      const { x, y } = scatterPosition(n);
      expect(Math.abs(x)).toBeLessThanOrEqual(0.85);
      expect(Math.abs(y)).toBeLessThanOrEqual(0.85);
    }
  });

  it("hashServiceName is a finite 32-bit unsigned integer", () => {
    const h = hashServiceName("svc");
    expect(Number.isFinite(h)).toBe(true);
    expect(h).toBeGreaterThanOrEqual(0);
    expect(h).toBeLessThanOrEqual(0xffffffff);
  });
});

describe("lifecycleToBrightness — activity tier", () => {
  it.each<[ServiceLifecycleState, number]>([
    ["active", 1],
    ["bootstrapping", 0.85],
    ["quiet", 0.6],
    ["silent", 0.4],
    ["dormant", 0.25],
    ["unknown", 0.5],
    ["archived", 0],
  ])("maps %s → brightness %f", (state, expected) => {
    expect(lifecycleToBrightness(state)).toBe(expected);
  });

  it("active is brighter than quiet which is brighter than dormant", () => {
    expect(lifecycleToBrightness("active")).toBeGreaterThan(lifecycleToBrightness("quiet"));
    expect(lifecycleToBrightness("quiet")).toBeGreaterThan(lifecycleToBrightness("dormant"));
  });
});

describe("visibleDots — filter + map", () => {
  it("hides Archived services", () => {
    const dots = visibleDots([item("a", "active"), item("b", "archived"), item("c", "quiet")]);
    expect(dots.map((d) => d.service)).toEqual(["a", "c"]);
  });

  it("keeps Dormant services (dimmed, not hidden)", () => {
    const dots = visibleDots([item("a", "dormant")]);
    expect(dots).toHaveLength(1);
    expect(dots[0].brightness).toBe(0.25);
  });

  it("derives hueFraction from priority_tier (severity hue)", () => {
    const dots = visibleDots([
      item("calm", "active", null),
      item("hot", "active", "autonomous"),
    ]);
    const calm = dots.find((d) => d.service === "calm")!;
    const hot = dots.find((d) => d.service === "hot")!;
    expect(calm.hueFraction).toBe(0);
    expect(hot.hueFraction).toBe(1);
    expect(hot.hueFraction).toBeGreaterThan(calm.hueFraction);
  });

  it("normalizes undefined priority_tier to null (calm baseline)", () => {
    const raw = { service: "x", state: "active", last_seen_unix_nano: 0, manual_override: null };
    const dots = visibleDots([raw as ServiceListItem]);
    expect(dots[0].priorityTier).toBeNull();
    expect(dots[0].hueFraction).toBe(0);
  });

  it("caps at MAX_CONSTELLATION_DOTS", () => {
    const many = Array.from({ length: MAX_CONSTELLATION_DOTS + 8 }, (_, i) =>
      item(`svc-${String(i).padStart(2, "0")}`, "active"),
    );
    expect(visibleDots(many)).toHaveLength(MAX_CONSTELLATION_DOTS);
  });

  it("returns empty for empty input", () => {
    expect(visibleDots([])).toEqual([]);
  });
});

describe("constellationSummary — off-canvas accessible name", () => {
  it("reports no active services when empty", () => {
    expect(constellationSummary([])).toBe("Service constellation: no active services.");
    expect(constellationSummary([item("a", "archived")])).toBe(
      "Service constellation: no active services.",
    );
  });

  it("counts per-state and active findings in words (not color-alone)", () => {
    const summary = constellationSummary([
      item("a", "active", "autonomous"),
      item("b", "active"),
      item("c", "quiet"),
      item("d", "dormant"),
      item("z", "archived"),
    ]);
    expect(summary).toContain("4 services");
    expect(summary).toContain("2 active");
    expect(summary).toContain("1 quiet");
    expect(summary).toContain("1 dormant");
    expect(summary).toContain("1 with active findings");
  });

  it("omits the findings clause when no service has an active finding", () => {
    const summary = constellationSummary([item("a", "active")]);
    expect(summary).toContain("1 service");
    expect(summary).not.toContain("active findings");
  });
});
