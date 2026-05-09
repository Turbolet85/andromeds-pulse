import { useMemo } from "react";
import type { TraceRow } from "../../../bindings";

// Per-service aggregate driving the constellation map. throughputHz +
// errorRate per service feed the existing chunk #31 Halo signature element
// (one dot per service) at clamped 0.8–2.4 Hz + LCH Earth Blue ↔ Alert
// Burgundy hue per design-system.md §Brand Identity Signature element.
export interface ServiceAggregate {
  serviceName: string;
  throughputHz: number;
  errorRate: number;
  position: { x: number; y: number };
}

export interface UseConstellationOptions {
  windowSeconds: number;
}

export function useConstellationData(
  rows: readonly TraceRow[],
  options: UseConstellationOptions,
): ServiceAggregate[] {
  const window = Math.max(1, options.windowSeconds);
  return useMemo(() => aggregateByService(rows, window), [rows, window]);
}

export function aggregateByService(
  rows: readonly TraceRow[],
  windowSeconds: number,
): ServiceAggregate[] {
  const buckets = new Map<string, { count: number; errors: number }>();
  for (const row of rows) {
    const key = row.service === "" ? "(unknown)" : row.service;
    const bucket = buckets.get(key) ?? { count: 0, errors: 0 };
    bucket.count += 1;
    bucket.errors += row.error_count;
    buckets.set(key, bucket);
  }

  const services = Array.from(buckets.keys()).sort();
  const total = services.length;
  const safeWindow = Math.max(1, windowSeconds);
  return services.map((serviceName, index) => {
    const bucket = buckets.get(serviceName)!;
    const throughputHz = bucket.count / safeWindow;
    const errorRate = bucket.count === 0 ? 0 : bucket.errors / bucket.count;
    return {
      serviceName,
      throughputHz,
      errorRate,
      position: radialPosition(index, total),
    };
  });
}

// Deterministic radial layout in normalized [-1, 1] canvas space. Single dot
// at origin; multi-service dots evenly distributed around the unit circle.
function radialPosition(index: number, total: number): { x: number; y: number } {
  if (total <= 1) {
    return { x: 0, y: 0 };
  }
  const angle = (2 * Math.PI * index) / total;
  return { x: Math.cos(angle), y: Math.sin(angle) };
}
