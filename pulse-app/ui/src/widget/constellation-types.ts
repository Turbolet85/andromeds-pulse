// Pure-function core for the compact-widget service constellation (replaces
// the aggregated badge). Deterministic, side-effect-free, fully unit-tested;
// the WebGPU rendering in ConstellationCanvas consumes these outputs.
//
// Encoding (per design-system §Motion Decisions Log 2026-05-29 + chunk #90
// Halo model): dot HUE = per-service incident severity (Earth Blue → Alert
// Burgundy via severity-to-halo); dot BRIGHTNESS = lifecycle activity tier;
// dot POSITION = stable hash(service_name) scatter for cross-session
// consistency. Only currently-live services (last span within
// LIVE_RECENCY_WINDOW_NANOS) are shown; stale/archived hidden (P-067).

import type { PriorityTier, ServiceLifecycleState, ServiceListItem } from "../bindings/index";
import { severityToHueFraction } from "../halo/severity-to-halo";

// Up to ~20 services rendered before glance-readability degrades at the fixed
// quarter-screen widget size (layout-templates.md §Responsive Compact widget).
export const MAX_CONSTELLATION_DOTS = 20;

// Scatter half-extent in normalized [-1, 1] canvas space; the margin keeps
// dots off the canvas edge.
const SCATTER_EXTENT = 0.85;

// A service counts as live only while its last span is within this window.
// Mirrors the Rust FSM `ACTIVE_TO_QUIET_THRESHOLD_SECONDS` (60s), so a
// corpus-restored or long-quiet service (old last_seen) is not surfaced as a
// live dot (P-067 — intent F7).
export const LIVE_RECENCY_WINDOW_NANOS = 60 * 1_000_000_000;

export function isServiceLive(item: ServiceListItem, nowUnixNano: number): boolean {
  return nowUnixNano - item.last_seen_unix_nano <= LIVE_RECENCY_WINDOW_NANOS;
}

export interface ConstellationDot {
  service: string;
  // Normalized [-SCATTER_EXTENT, SCATTER_EXTENT] position.
  x: number;
  y: number;
  // [0, 1] activity brightness (active brightest → dormant dim).
  brightness: number;
  // [0, 1] LCH severity hue fraction (0 = calm Earth Blue, 1 = Alert Burgundy).
  hueFraction: number;
  state: ServiceLifecycleState;
  priorityTier: PriorityTier | null;
}

// FNV-1a 32-bit hash → deterministic per-name seed so a service occupies the
// same scatter position across sessions.
export function hashServiceName(serviceName: string): number {
  let hash = 0x811c9dc5;
  for (let i = 0; i < serviceName.length; i += 1) {
    hash ^= serviceName.charCodeAt(i);
    hash = Math.imul(hash, 0x01000193);
  }
  return hash >>> 0;
}

export function scatterPosition(serviceName: string): { x: number; y: number } {
  const hash = hashServiceName(serviceName);
  const xUnit = (hash & 0xffff) / 0xffff;
  const yUnit = ((hash >>> 16) & 0xffff) / 0xffff;
  return {
    x: (xUnit * 2 - 1) * SCATTER_EXTENT,
    y: (yUnit * 2 - 1) * SCATTER_EXTENT,
  };
}

const BRIGHTNESS_BY_STATE: Record<ServiceLifecycleState, number> = {
  active: 1,
  bootstrapping: 0.85,
  quiet: 0.6,
  silent: 0.4,
  dormant: 0.25,
  unknown: 0.5,
  archived: 0,
};

export function lifecycleToBrightness(state: ServiceLifecycleState): number {
  return BRIGHTNESS_BY_STATE[state];
}

const LABEL_BY_STATE: Record<ServiceLifecycleState, string> = {
  active: "active",
  bootstrapping: "starting up",
  quiet: "quiet",
  silent: "silent",
  dormant: "dormant",
  unknown: "unknown",
  archived: "archived",
};

export function lifecycleLabel(state: ServiceLifecycleState): string {
  return LABEL_BY_STATE[state];
}

// Non-color severity token (SC 1.4.1): pairs with the dot hue so per-service
// health is not conveyed by color alone. null = no active incident = "healthy".
const TOKEN_BY_SEVERITY: Record<PriorityTier, string> = {
  autonomous: "autonomous",
  suggested: "suggested",
  curious: "curious",
};

export function severityToken(tier: PriorityTier | null): string {
  return tier === null ? "healthy" : TOKEN_BY_SEVERITY[tier];
}

export interface DotLabelPosition {
  leftPct: number;
  topPct: number;
}

// Map a dot's normalized clip-space coord (x,y in [-1,1], y-up) to CSS percent
// offsets for the DOM label overlay positioned over the canvas box (y-down):
// x=-1 -> 0% left, x=1 -> 100% left; y=1 (top) -> 0% top, y=-1 (bottom) -> 100% top.
export function dotLabelPosition(dot: ConstellationDot): DotLabelPosition {
  return {
    leftPct: ((dot.x + 1) / 2) * 100,
    topPct: ((1 - dot.y) / 2) * 100,
  };
}

export interface DotLabelPlacement extends DotLabelPosition {
  service: string;
}

// Approximate label-chip footprint as a fraction of the hero box, for overlap
// detection. Position-based heuristic (true chip widths vary with name length +
// need layout) — resolves the common cluster case; the operator visual verify
// is the geometry check.
const LABEL_VERTICAL_BAND_PCT = 11;
const LABEL_HORIZONTAL_BAND_PCT = 24;

// De-stagger overlapping label chips: place dots top-to-bottom (stable by name)
// and push each candidate down past any already-placed chip whose footprint it
// intersects. Deterministic; every dot keeps a label (P-069 always-on).
export function resolveLabelPositions(
  dots: readonly ConstellationDot[],
): DotLabelPlacement[] {
  const ordered = dots
    .map((dot) => ({ service: dot.service, ...dotLabelPosition(dot) }))
    .sort((a, b) => a.topPct - b.topPct || a.service.localeCompare(b.service));
  const placed: DotLabelPlacement[] = [];
  for (const candidate of ordered) {
    let topPct = candidate.topPct;
    let collided = true;
    while (collided) {
      collided = false;
      for (const p of placed) {
        if (
          Math.abs(p.leftPct - candidate.leftPct) < LABEL_HORIZONTAL_BAND_PCT &&
          Math.abs(p.topPct - topPct) < LABEL_VERTICAL_BAND_PCT
        ) {
          topPct = p.topPct + LABEL_VERTICAL_BAND_PCT;
          collided = true;
        }
      }
    }
    placed.push({ service: candidate.service, leftPct: candidate.leftPct, topPct });
  }
  return placed;
}

// Build the renderable dots: drop Archived (hidden), stable-sort by name,
// cap to MAX_CONSTELLATION_DOTS.
export function visibleDots(
  items: readonly ServiceListItem[],
  nowUnixNano: number,
): ConstellationDot[] {
  return items
    .filter((item) => item.state !== "archived" && isServiceLive(item, nowUnixNano))
    .slice()
    .sort((a, b) => a.service.localeCompare(b.service))
    .slice(0, MAX_CONSTELLATION_DOTS)
    .map((item) => {
      const tier = item.priority_tier ?? null;
      const { x, y } = scatterPosition(item.service);
      return {
        service: item.service,
        x,
        y,
        brightness: lifecycleToBrightness(item.state),
        hueFraction: severityToHueFraction(tier),
        state: item.state,
        priorityTier: tier,
      };
    });
}

// Off-canvas accessible summary (the `<canvas>` is opaque to screen readers
// per a11y-plan §1); conveys count + per-state breakdown + active-findings
// count in words so dot state is not color/brightness-only (SC 1.4.1).
export function constellationSummary(
  items: readonly ServiceListItem[],
  nowUnixNano: number,
): string {
  const visible = items.filter(
    (item) => item.state !== "archived" && isServiceLive(item, nowUnixNano),
  );
  if (visible.length === 0) {
    return "Service constellation: no active services.";
  }
  const counts = new Map<ServiceLifecycleState, number>();
  let withFindings = 0;
  for (const item of visible) {
    counts.set(item.state, (counts.get(item.state) ?? 0) + 1);
    if ((item.priority_tier ?? null) !== null) {
      withFindings += 1;
    }
  }
  const order: ServiceLifecycleState[] = [
    "active",
    "quiet",
    "silent",
    "dormant",
    "bootstrapping",
    "unknown",
  ];
  const stateParts: string[] = [];
  for (const state of order) {
    const count = counts.get(state);
    if (count !== undefined && count > 0) {
      stateParts.push(`${count} ${lifecycleLabel(state)}`);
    }
  }
  const plural = visible.length === 1 ? "service" : "services";
  const findingsClause = withFindings > 0 ? ` ${withFindings} with active findings.` : "";
  return `Service constellation: ${visible.length} ${plural} — ${stateParts.join(", ")}.${findingsClause}`;
}

export interface HueShiftSample {
  duration_ms: number;
  severity_tier: PriorityTier | "none";
}

export interface HueShiftResult {
  samples: HueShiftSample[];
  next: Map<string, PriorityTier | null>;
}

// P-025: one sample per dot whose tier changed, measuring paint instant −
// the instant that tier became true on the backend (`tier_effective_at`).
// Only changes this canvas WITNESSED count (effective ≥ mount): a tier
// restored at boot is not a hue-update latency.
export function hueShiftSamples(
  previous: ReadonlyMap<string, PriorityTier | null>,
  dots: readonly ConstellationDot[],
  items: readonly ServiceListItem[],
  mountedAtMs: number,
  paintNowMs: number,
): HueShiftResult {
  const next = new Map(previous);
  const samples: HueShiftSample[] = [];
  for (const dot of dots) {
    const tier = dot.priorityTier ?? null;
    const known = previous.has(dot.service);
    const before = previous.get(dot.service) ?? null;
    next.set(dot.service, tier);
    if ((!known && tier === null) || (known && before === tier)) {
      continue;
    }
    const item = items.find((candidate) => candidate.service === dot.service);
    const effectiveNanos = item?.tier_effective_at_unix_nano ?? null;
    if (effectiveNanos === null) {
      continue;
    }
    const effectiveMs = effectiveNanos / 1_000_000;
    if (effectiveMs < mountedAtMs) {
      continue;
    }
    samples.push({
      duration_ms: Math.max(0, paintNowMs - effectiveMs),
      severity_tier: tier ?? "none",
    });
  }
  return { samples, next };
}
