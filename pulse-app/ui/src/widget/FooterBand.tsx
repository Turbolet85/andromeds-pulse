// Compact-widget footer band — read-only, glance-readable status display
// for the 3 live metrics (ingest spans/sec, error %, retention used m / max m)
// per layout-templates.md §Component — Footer / Compact widget.
//
// Live-region semantics: `<footer role="status" aria-live="polite">` so
// SR users receive non-blocking announcements without focus theft. Parent
// throttles state updates (synthetic simulator ticks at 1000ms; real
// broadcast subscriber in chunks #34/#35 will throttle to ≥5s) so the
// announcer does not thrash. Footer itself stays naive — props in, DOM out.
//
// Not-color-alone (SC 1.4.1): when error rate ≥ 5% threshold, the value
// renders with `--color-accent` border AND keeps `--color-text-primary` text
// color; the percentage number itself carries the meaning, the border is
// a paired affordance. Pure background-color encoding would fail SC 1.4.1.
//
// No entrance animations / opacity fades on data updates per design
// §Motion 0.35 expression budget.

import { Icon } from "../components/icons";
import {
  ERROR_RATE_ACCENT_THRESHOLD,
  formatErrorRatePercent,
  formatRetentionMinutes,
  formatSpansPerSec,
} from "./widget-types";

interface FooterBandProps {
  throughputHz: number;
  errorRate: number;
  retentionUsedSeconds: number;
  retentionMaxSeconds: number;
}

const LABEL_STYLE: React.CSSProperties = {
  fontFamily: "var(--font-body)",
  fontSize: "12px",
  fontWeight: 500,
  color: "var(--color-text-secondary)",
  margin: 0,
};

const NUMERIC_STYLE: React.CSSProperties = {
  fontFamily: "var(--font-code)",
  fontVariantNumeric: "tabular-nums",
  fontSize: "12px",
  color: "var(--color-text-primary)",
  margin: 0,
};

export function FooterBand({
  throughputHz,
  errorRate,
  retentionUsedSeconds,
  retentionMaxSeconds,
}: FooterBandProps) {
  const errorBorderColor =
    errorRate >= ERROR_RATE_ACCENT_THRESHOLD ? "var(--color-accent)" : "transparent";

  return (
    <footer
      role="status"
      aria-live="polite"
      aria-label="Live telemetry summary"
      data-testid="footer-band"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-xs)",
        padding: "var(--spacing-sm) var(--spacing-md)",
        borderTop: "1px solid rgba(74, 144, 226, 0.3)",
        background: "transparent",
      }}
    >
      <dl
        style={{
          display: "flex",
          justifyContent: "space-between",
          gap: "var(--spacing-md)",
          margin: 0,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "var(--spacing-xs)",
          }}
        >
          <dt style={LABEL_STYLE}>Ingest</dt>
          <dd
            style={{
              display: "flex",
              alignItems: "center",
              gap: "var(--spacing-xs)",
              margin: 0,
            }}
          >
            <Icon glyph="circular-pulse" size={16} />
            <span style={NUMERIC_STYLE}>{formatSpansPerSec(throughputHz)}</span>
          </dd>
        </div>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "var(--spacing-xs)",
          }}
        >
          <dt style={LABEL_STYLE}>Error</dt>
          <dd style={{ margin: 0 }}>
            <span
              data-testid="footer-error-value"
              style={{
                ...NUMERIC_STYLE,
                border: `1px solid ${errorBorderColor}`,
                borderRadius: "var(--radius-sm)",
                padding: "0 var(--spacing-xs)",
              }}
            >
              {formatErrorRatePercent(errorRate)}
            </span>
          </dd>
        </div>
      </dl>
      <dl
        style={{
          display: "flex",
          gap: "var(--spacing-xs)",
          margin: 0,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "var(--spacing-xs)",
          }}
        >
          <dt style={LABEL_STYLE}>Retention</dt>
          <dd
            style={{
              display: "flex",
              alignItems: "center",
              gap: "var(--spacing-xs)",
              margin: 0,
            }}
          >
            <span style={NUMERIC_STYLE}>
              {formatRetentionMinutes(retentionUsedSeconds, retentionMaxSeconds)}
            </span>
            <span
              aria-hidden="true"
              style={{
                ...LABEL_STYLE,
                color: "var(--color-text-secondary)",
              }}
            >
              used
            </span>
          </dd>
        </div>
      </dl>
    </footer>
  );
}
