import { describe, expect, it } from "vitest";
import type { PriorityTier, ServiceLifecycleState, ServiceListItem } from "../bindings/index";
import {
  constellationSummary,
  type ConstellationDot,
  dotLabelPosition,
  hashServiceName,
  isServiceLive,
  LIVE_RECENCY_WINDOW_NANOS,
  lifecycleToBrightness,
  MAX_CONSTELLATION_DOTS,
  resolveLabelPositions,
  scatterPosition,
  severityToken,
  visibleDots,
} from "./constellation-types";

// Fixed reference "now" (nanoseconds, realistic scale); fixtures are dated
// relative to it so liveness is deterministic (the pure functions read no
// wall clock — `now` is injected).
const NOW = 1_700_000_000_000_000_000;
const RECENT = NOW - 10 * 1_000_000_000; // 10s ago → live
const STALE = NOW - 120 * 1_000_000_000; // 2min ago → not live (> 60s window)

function item(
  service: string,
  state: ServiceLifecycleState,
  priorityTier: ServiceListItem["priority_tier"] = null,
  lastSeenUnixNano: number = RECENT,
): ServiceListItem {
  return {
    service,
    state,
    last_seen_unix_nano: lastSeenUnixNano,
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

describe("isServiceLive — recency gate (P-067)", () => {
  it("is live when the last span is within the 60s window", () => {
    expect(isServiceLive(item("a", "active"), NOW)).toBe(true);
    expect(isServiceLive(item("a", "active", null, NOW - 59 * 1_000_000_000), NOW)).toBe(true);
  });

  it("is not live once the last span is older than the window", () => {
    expect(isServiceLive(item("a", "active", null, STALE), NOW)).toBe(false);
    expect(isServiceLive(item("a", "active", null, NOW - 61 * 1_000_000_000), NOW)).toBe(false);
  });

  it("treats a corpus-restored ancient last_seen as not live", () => {
    expect(isServiceLive(item("a", "active", null, 1_000), NOW)).toBe(false);
  });

  it("exposes the 60s window as nanoseconds", () => {
    expect(LIVE_RECENCY_WINDOW_NANOS).toBe(60 * 1_000_000_000);
  });
});

describe("visibleDots — recency filter + map", () => {
  it("hides Archived services", () => {
    const dots = visibleDots([item("a", "active"), item("b", "archived"), item("c", "quiet")], NOW);
    expect(dots.map((d) => d.service)).toEqual(["a", "c"]);
  });

  it("hides services whose last span is older than the live window", () => {
    const dots = visibleDots([item("live", "active"), item("stale", "active", null, STALE)], NOW);
    expect(dots.map((d) => d.service)).toEqual(["live"]);
  });

  it("hides corpus-restored/stale services with an ancient last_seen", () => {
    // A restored service keeps its prior-session last_seen (P-067); recency
    // hides it regardless of the persisted state label.
    expect(visibleDots([item("d", "dormant", null, 1_000)], NOW)).toEqual([]);
    expect(visibleDots([item("q", "quiet", null, STALE)], NOW)).toEqual([]);
  });

  it("keeps brightness-by-state for surviving live dots", () => {
    const dots = visibleDots([item("q", "quiet")], NOW);
    expect(dots).toHaveLength(1);
    expect(dots[0].brightness).toBe(0.6);
  });

  it("derives hueFraction from priority_tier (severity hue)", () => {
    const dots = visibleDots([item("calm", "active", null), item("hot", "active", "autonomous")], NOW);
    const calm = dots.find((d) => d.service === "calm")!;
    const hot = dots.find((d) => d.service === "hot")!;
    expect(calm.hueFraction).toBe(0);
    expect(hot.hueFraction).toBe(1);
    expect(hot.hueFraction).toBeGreaterThan(calm.hueFraction);
  });

  it("normalizes undefined priority_tier to null (calm baseline)", () => {
    const raw = { service: "x", state: "active", last_seen_unix_nano: RECENT, manual_override: null };
    const dots = visibleDots([raw as ServiceListItem], NOW);
    expect(dots[0].priorityTier).toBeNull();
    expect(dots[0].hueFraction).toBe(0);
  });

  it("caps at MAX_CONSTELLATION_DOTS", () => {
    const many = Array.from({ length: MAX_CONSTELLATION_DOTS + 8 }, (_, i) =>
      item(`svc-${String(i).padStart(2, "0")}`, "active"),
    );
    expect(visibleDots(many, NOW)).toHaveLength(MAX_CONSTELLATION_DOTS);
  });

  it("returns empty for empty input", () => {
    expect(visibleDots([], NOW)).toEqual([]);
  });

  it("returns empty when all services are stale (zero live telemetry)", () => {
    const stale = [
      item("a", "active", null, STALE),
      item("b", "quiet", null, STALE),
      item("c", "active", null, 1_000),
    ];
    expect(visibleDots(stale, NOW)).toEqual([]);
  });
});

describe("constellationSummary — off-canvas accessible name", () => {
  it("reports no active services when empty", () => {
    expect(constellationSummary([], NOW)).toBe("Service constellation: no active services.");
    expect(constellationSummary([item("a", "archived")], NOW)).toBe(
      "Service constellation: no active services.",
    );
  });

  it("reports no active services when all services are stale (zero live telemetry)", () => {
    const stale = [item("a", "active", null, STALE), item("b", "quiet", null, 1_000)];
    expect(constellationSummary(stale, NOW)).toBe("Service constellation: no active services.");
  });

  it("counts per-state and active findings in words (not color-alone)", () => {
    const summary = constellationSummary(
      [
        item("a", "active", "autonomous"),
        item("b", "active"),
        item("c", "quiet"),
        item("z", "archived"),
      ],
      NOW,
    );
    expect(summary).toContain("3 services");
    expect(summary).toContain("2 active");
    expect(summary).toContain("1 quiet");
    expect(summary).toContain("1 with active findings");
  });

  it("omits the findings clause when no service has an active finding", () => {
    const summary = constellationSummary([item("a", "active")], NOW);
    expect(summary).toContain("1 service");
    expect(summary).not.toContain("active findings");
  });
});

function dotAt(x: number, y: number): ConstellationDot {
  return { service: "s", x, y, brightness: 1, hueFraction: 0, state: "active", priorityTier: null };
}

describe("severityToken — non-color severity cue (SC 1.4.1)", () => {
  it.each<[PriorityTier | null, string]>([
    [null, "healthy"],
    ["curious", "curious"],
    ["suggested", "suggested"],
    ["autonomous", "autonomous"],
  ])("maps %s → %s", (tier, expected) => {
    expect(severityToken(tier)).toBe(expected);
  });

  it("produces a distinct token per tier (severity is not color-alone)", () => {
    const tiers: (PriorityTier | null)[] = [null, "curious", "suggested", "autonomous"];
    expect(new Set(tiers.map(severityToken)).size).toBe(4);
  });
});

describe("dotLabelPosition — normalized coord → CSS percent (y-flipped)", () => {
  it("maps centre (0,0) to 50%/50%", () => {
    expect(dotLabelPosition(dotAt(0, 0))).toEqual({ leftPct: 50, topPct: 50 });
  });

  it("flips y: top (y=1) → 0% top, bottom (y=-1) → 100% top", () => {
    expect(dotLabelPosition(dotAt(0, 1)).topPct).toBe(0);
    expect(dotLabelPosition(dotAt(0, -1)).topPct).toBe(100);
  });

  it("maps x: left (x=-1) → 0% left, right (x=1) → 100% left", () => {
    expect(dotLabelPosition(dotAt(-1, 0)).leftPct).toBe(0);
    expect(dotLabelPosition(dotAt(1, 0)).leftPct).toBe(100);
  });

  it("keeps scatter-extent dots within [0,100]%", () => {
    for (const [x, y] of [
      [0.85, 0.85],
      [-0.85, -0.85],
    ] as const) {
      const { leftPct, topPct } = dotLabelPosition(dotAt(x, y));
      expect(leftPct).toBeGreaterThanOrEqual(0);
      expect(leftPct).toBeLessThanOrEqual(100);
      expect(topPct).toBeGreaterThanOrEqual(0);
      expect(topPct).toBeLessThanOrEqual(100);
    }
  });
});

describe("resolveLabelPositions — label collision-avoidance (P-069 CARRY)", () => {
  function dotNamed(service: string, x: number, y: number): ConstellationDot {
    return { service, x, y, brightness: 1, hueFraction: 0, state: "active", priorityTier: null };
  }

  it("leaves well-separated dots at their base label positions", () => {
    const dots = [dotNamed("a", -0.8, 0.8), dotNamed("b", 0.8, -0.8)];
    const placed = resolveLabelPositions(dots);
    for (const dot of dots) {
      const base = dotLabelPosition(dot);
      const p = placed.find((q) => q.service === dot.service)!;
      expect(p.leftPct).toBeCloseTo(base.leftPct);
      expect(p.topPct).toBeCloseTo(base.topPct);
    }
  });

  it("separates overlapping labels when dots cluster at the same position", () => {
    const dots = [dotNamed("a", 0, 0), dotNamed("b", 0, 0), dotNamed("c", 0, 0)];
    const placed = resolveLabelPositions(dots);
    expect(placed).toHaveLength(3);
    const tops = placed.map((p) => p.topPct).sort((m, n) => m - n);
    // Each successive label clears the vertical band of the one above it.
    for (let i = 1; i < tops.length; i += 1) {
      expect(tops[i] - tops[i - 1]).toBeGreaterThanOrEqual(11);
    }
  });

  it("preserves every dot (P-069 always-on label) and is deterministic", () => {
    const dots = [dotNamed("a", 0.1, 0.1), dotNamed("b", 0.1, 0.1), dotNamed("c", -0.5, 0.3)];
    const first = resolveLabelPositions(dots);
    const second = resolveLabelPositions(dots);
    expect(first.map((p) => p.service).sort()).toEqual(["a", "b", "c"]);
    expect(first).toEqual(second);
  });
});
