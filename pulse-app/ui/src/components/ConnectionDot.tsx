// Ambient connection-state dot for the compact-widget titlebar (chunk #89).
// Replaces the removed dashboard-mindset FooterBand metrics with a single
// glance-readable signal: a 8px dot colored per the chunk #59 connection
// FSM state, with a hover tooltip showing the state + last-span-ago.
//
// Non-interactive by design: `role="img"` + an always-present `aria-label`
// carrying the state word satisfies SC 1.4.1 (not-color-alone) without an
// interactive control, so the dot adds no titlebar tab stop and SC 1.4.13
// (content on hover/focus of a UI component) does not apply. The visual
// tooltip is `aria-hidden` — assistive tech already has the full state via
// the accessible name. State + last-span-ago are bounded, sanitized values
// from `connection.current_state` (per its bindings docstring); no raw OTLP
// attribute content is rendered (per security plan §Logging & Monitoring).

import { useState } from "react";
import { useConnectionState } from "../hooks/use-connection-state";
import type {
  ConnectionState,
  ConnectionStatePayload,
  ReceiverFailureReason,
} from "../bindings/index";

const DOT_SIZE_PX = 8;

// Earth Blue (healthy) for the three nominal states; Alert Burgundy for the
// two degraded states (Stalled = warning, ReceiverFailed = critical) per
// design-system §Color Palette + §Semantic Colors. Exhaustive switch over
// the 5-state FSM discriminator (chunk #59) — a future 6th state becomes a
// compile error, not a silent color gap.
export function connectionColorToken(state: ConnectionState): string {
  switch (state.state) {
    case "Listening":
    case "Receiving":
    case "Idle":
      return "var(--color-primary)";
    case "Stalled":
    case "ReceiverFailed":
      return "var(--color-accent)";
  }
}

const FAILURE_REASON_LABEL: Record<ReceiverFailureReason, string> = {
  bind_failed: "bind failed",
  stale_heartbeat: "stale heartbeat",
  receiver_panicked: "receiver panicked",
};

export function connectionStateLabel(payload: ConnectionStatePayload): string {
  switch (payload.state.state) {
    case "Listening":
      return "Listening";
    case "Receiving":
      return "Receiving";
    case "Idle":
      return "Idle";
    case "Stalled":
      return "Stalled";
    case "ReceiverFailed": {
      const reason = payload.reason
        ? ` (${FAILURE_REASON_LABEL[payload.reason]})`
        : "";
      return `Receiver failed${reason}`;
    }
  }
}

export function formatLastSpanAgo(ms: number): string {
  if (ms < 1000) {
    return "just now";
  }
  const seconds = Math.floor(ms / 1000);
  if (seconds < 60) {
    return `${seconds}s ago`;
  }
  const minutes = Math.floor(seconds / 60);
  return `${minutes}m ago`;
}

export function ConnectionDot() {
  const payload = useConnectionState();
  const [tooltipVisible, setTooltipVisible] = useState(false);

  const label = payload ? connectionStateLabel(payload) : "Connecting…";
  const lastSpan = payload ? formatLastSpanAgo(payload.last_span_ago_ms) : null;
  const colorToken = payload
    ? connectionColorToken(payload.state)
    : "var(--color-text-muted)";
  const ariaLabel = lastSpan
    ? `Connection: ${label} — last span ${lastSpan}`
    : `Connection: ${label}`;

  return (
    <span
      className="titlebar__connection"
      style={{
        position: "relative",
        display: "inline-flex",
        alignItems: "center",
      }}
      onMouseEnter={() => setTooltipVisible(true)}
      onMouseLeave={() => setTooltipVisible(false)}
    >
      <span
        role="img"
        aria-label={ariaLabel}
        data-testid="connection-dot"
        data-connection-state={payload ? payload.state.state : "Unknown"}
        style={{
          width: `${DOT_SIZE_PX}px`,
          height: `${DOT_SIZE_PX}px`,
          borderRadius: "var(--radius-full)",
          background: colorToken,
        }}
      />
      {tooltipVisible ? (
        <span
          aria-hidden="true"
          data-testid="connection-tooltip"
          style={{
            position: "absolute",
            top: "calc(100% + var(--spacing-xs))",
            left: 0,
            zIndex: 1,
            whiteSpace: "nowrap",
            background: "var(--color-raised-2)",
            border: "1px solid rgba(74, 144, 226, 0.1)",
            borderRadius: "var(--radius-md)",
            padding: "var(--spacing-xs) var(--spacing-sm)",
            fontFamily: "var(--font-body)",
            fontSize: "12px",
            color: "var(--color-text-primary)",
          }}
        >
          <span>{label}</span>
          {lastSpan ? (
            <span
              style={{
                fontFamily: "var(--font-code)",
                fontVariantNumeric: "tabular-nums",
                color: "var(--color-text-secondary)",
                marginLeft: "var(--spacing-xs)",
              }}
            >
              last span {lastSpan}
            </span>
          ) : null}
        </span>
      ) : null}
    </span>
  );
}
