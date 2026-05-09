// Compact-widget aggregated service-count badge wrapped by the chunk #31
// Halo State Pulse signature element. The HaloCanvas fills the container;
// the badge overlay sits in the bottom-right quadrant per layout-templates.md
// §Surface: desktop-webview / Signature placement so the unified halo glow
// geometrically encloses it.
//
// Composition over API extension: HaloCanvas's `{ ariaLabel, throughputHz,
// errorRate }` contract stays untouched (chunk #31 acceptance). The badge
// overlay rides above HaloCanvas as a sibling with `pointer-events: none`
// so the canvas continues to own its visual space.
//
// Per a11y plan §1 P5 + SC 4.1.3 Status Messages: badge wraps in
// `role="status"` with a complete-sentence aria-label
// ("N services, P% average error rate") for SR users; the visible text is
// abbreviated. Icon is decorative (`aria-hidden="true"`) per chunk #11
// flip pattern — the adjacent `<span>services</span>` decoration carries
// only context, no meaning.
//
// Per design plan §Self-Validation Protocol Signature Test #3: this surface
// is one of the 3 mandatory Halo locations. Replacing HaloCanvas here with
// a generic spinner / static swatch fails the test.

import { HaloCanvas } from "../halo/HaloCanvas";
import { Icon } from "../components/icons";
import { formatBadgeAriaLabel, formatServiceCount } from "./widget-types";

interface AggregatedBadgeCanvasProps {
  serviceCount: number;
  throughputHz: number;
  errorRate: number;
}

export function AggregatedBadgeCanvas({
  serviceCount,
  throughputHz,
  errorRate,
}: AggregatedBadgeCanvasProps) {
  const ariaLabel = formatBadgeAriaLabel(serviceCount, errorRate);
  return (
    <div
      style={{ position: "relative", width: "100%", height: "100%" }}
      data-testid="aggregated-badge-canvas"
    >
      <HaloCanvas
        ariaLabel="Service constellation halo"
        throughputHz={throughputHz}
        errorRate={errorRate}
      />
      <div
        role="status"
        aria-label={ariaLabel}
        style={{
          position: "absolute",
          bottom: "var(--spacing-md)",
          right: "var(--spacing-md)",
          display: "flex",
          alignItems: "center",
          gap: "var(--spacing-xs)",
          color: "var(--color-text-primary)",
          fontFamily: "var(--font-body)",
          fontSize: "12px",
          fontWeight: 500,
          pointerEvents: "none",
        }}
      >
        <Icon glyph="constellation-grid" size={16} />
        <span
          style={{
            fontFamily: "var(--font-code)",
            fontVariantNumeric: "tabular-nums",
          }}
        >
          {formatServiceCount(serviceCount)}
        </span>
        <span aria-hidden="true">services</span>
      </div>
    </div>
  );
}
