// Pure-function core for the compact-widget service constellation (replaces
// the aggregated badge). Deterministic, side-effect-free, fully unit-tested;
// the WebGPU rendering in ConstellationCanvas consumes these outputs.
//
// Encoding (per design-system §Motion Decisions Log 2026-05-29 + chunk #90
// Halo model): dot HUE = per-service incident severity (Earth Blue → Alert
// Burgundy via severity-to-halo); dot BRIGHTNESS = lifecycle activity tier;
// dot POSITION = stable hash(service_name) scatter for cross-session
// consistency. Dormant dimmed; Archived hidden.

import type { PriorityTier, ServiceLifecycleState, ServiceListItem } from "../bindings/index";
import { severityToHueFraction } from "../halo/severity-to-halo";

// Up to ~20 services rendered before glance-readability degrades at the fixed
// quarter-screen widget size (layout-templates.md §Responsive Compact widget).
export const MAX_CONSTELLATION_DOTS = 20;

// Scatter half-extent in normalized [-1, 1] canvas space; the margin keeps
// dots off the canvas edge.
const SCATTER_EXTENT = 0.85;

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

// Build the renderable dots: drop Archived (hidden), stable-sort by name,
// cap to MAX_CONSTELLATION_DOTS.
export function visibleDots(items: readonly ServiceListItem[]): ConstellationDot[] {
  return items
    .filter((item) => item.state !== "archived")
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
export function constellationSummary(items: readonly ServiceListItem[]): string {
  const visible = items.filter((item) => item.state !== "archived");
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
