// Compact-widget shared types + pure-function formatters for the
// service-constellation aggregated badge + ingest/error/retention footer
// band. Formatters preserve tabular alignment by emitting fixed-width
// numeric strings where possible (`tabular-nums` on the consuming `<span>`
// handles glyph alignment; formatters handle scale + decimal precision).

export interface WidgetMetrics {
  serviceCount: number;
  throughputHz: number;
  errorRate: number;
  retentionUsedSeconds: number;
  retentionMaxSeconds: number;
}

// Threshold above which the footer error-rate field switches to
// `--color-accent` border + paired text/icon affordance per design
// §Decisions Log 2026-05-03 accent-lift entry + a11y SC 1.4.1 not-color-alone.
export const ERROR_RATE_ACCENT_THRESHOLD = 0.05;

// "—" placeholder for non-finite inputs so footer never displays "NaN" or
// "Infinity" to the user; accepted as a quiet fallback per glance-readable
// discipline.
const PLACEHOLDER = "—";

export function formatSpansPerSec(throughputHz: number): string {
  if (!Number.isFinite(throughputHz) || throughputHz < 0) {
    return PLACEHOLDER;
  }
  if (throughputHz >= 1000) {
    const k = throughputHz / 1000;
    return `${k.toFixed(1)}k/s`;
  }
  return `${Math.round(throughputHz)}/s`;
}

export function formatErrorRatePercent(errorRate: number): string {
  if (!Number.isFinite(errorRate) || errorRate < 0) {
    return PLACEHOLDER;
  }
  const pct = errorRate * 100;
  return `${pct.toFixed(1)}%`;
}

export function formatRetentionMinutes(usedSeconds: number, maxSeconds: number): string {
  if (
    !Number.isFinite(usedSeconds) ||
    !Number.isFinite(maxSeconds) ||
    usedSeconds < 0 ||
    maxSeconds <= 0
  ) {
    return PLACEHOLDER;
  }
  const usedMin = Math.floor(usedSeconds / 60);
  const maxMin = Math.floor(maxSeconds / 60);
  return `${usedMin}m / ${maxMin}m`;
}

export function formatServiceCount(count: number): string {
  if (!Number.isFinite(count) || count < 0) {
    return PLACEHOLDER;
  }
  return `${Math.floor(count)}`;
}

export function formatBadgeAriaLabel(serviceCount: number, errorRate: number): string {
  const countStr = formatServiceCount(serviceCount);
  const errStr = formatErrorRatePercent(errorRate);
  return `${countStr} services, ${errStr} average error rate`;
}
