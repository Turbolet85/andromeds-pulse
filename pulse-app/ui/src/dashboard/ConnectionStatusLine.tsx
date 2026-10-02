// Plain-language connection-status line (P-070) for the full-dashboard footer.
// States, in words, how many sources are connected, whether telemetry is
// arriving (spans/s), and the buffer fill — the worded complement to the
// ConnectionDot + constellation (a11y-plan §1 not-color-alone).
//
// Read-only: no interactive control, so it adds no tab stop. The connected
// count reuses the P-067 isServiceLive gate so it equals the constellation's
// live-dot count by construction. Numeric segments render in the JetBrains
// Mono Data role; words in IBM Plex Sans (design-system §Typography).

import { useEffect, useRef, type CSSProperties, type ReactNode } from "react";
import { useConnectionState } from "../hooks/use-connection-state";
import { useIngestStats } from "../hooks/use-ingest-stats";
import { useServiceConstellation } from "../hooks/use-service-constellation";
import { isServiceLive } from "../widget/constellation-types";
import { useStatusAnnouncer } from "./StatusLiveRegion";

function nowUnixNano(): number {
  return Date.now() * 1_000_000;
}

function humanizeRate(rate: number): string {
  return rate >= 1000 ? `${(rate / 1000).toFixed(1)}k` : `${Math.round(rate)}`;
}

const numStyle: CSSProperties = {
  fontFamily: "var(--font-code)",
  fontVariantNumeric: "tabular-nums",
};

function Num({ children }: { children: ReactNode }) {
  return (
    <span data-num="" style={numStyle}>
      {children}
    </span>
  );
}

export function ConnectionStatusLine() {
  const services = useServiceConstellation();
  const connection = useConnectionState();
  const stats = useIngestStats();
  const announce = useStatusAnnouncer();

  const connectedCount = services.filter((s) => isServiceLive(s, nowUnixNano())).length;
  const state = connection?.state.state ?? null;
  const degraded = state === "Stalled" || state === "ReceiverFailed";
  const hasTelemetry = connectedCount > 0 || (stats?.hasData ?? false);

  // SC 4.1.3: announce the connected <-> no-telemetry transition ONCE, politely,
  // via the shared region — never a new live element, never per rate-tick.
  const prevHasTelemetry = useRef<boolean | null>(null);
  useEffect(() => {
    if (prevHasTelemetry.current !== null && prevHasTelemetry.current !== hasTelemetry) {
      announce(
        hasTelemetry
          ? `Receiving telemetry from ${connectedCount} ${connectedCount === 1 ? "service" : "services"}.`
          : "No telemetry — listening for OTLP.",
      );
    }
    prevHasTelemetry.current = hasTelemetry;
  }, [hasTelemetry, connectedCount, announce]);

  let content: ReactNode;
  if (degraded) {
    const word = state === "ReceiverFailed" ? "Receiver unavailable" : "Stalled";
    content = (
      <span data-status-kind="degraded">
        {word}
        {connection?.message ? ` — ${connection.message}` : ""}
      </span>
    );
  } else if (hasTelemetry) {
    const serviceWord = connectedCount === 1 ? "service" : "services";
    content = (
      <span data-status-kind="live">
        {connectedCount > 0 ? (
          <>
            Receiving from <Num>{connectedCount}</Num> {serviceWord}
          </>
        ) : (
          "Telemetry flowing, no live services"
        )}
        {stats && stats.spansPerSec !== null ? (
          <>
            {" · ~"}
            <Num>{humanizeRate(stats.spansPerSec)}</Num> spans/s
          </>
        ) : null}
        {stats ? (
          <>
            {" · buffer "}
            <Num>{Math.floor(stats.bufferUsedSeconds / 60)}</Num> min /{" "}
            <Num>{Math.floor(stats.retentionSeconds / 60)}</Num> min
          </>
        ) : null}
      </span>
    );
  } else {
    content = (
      <span data-status-kind="empty">
        No telemetry yet — listening on <Num>:4317</Num>/<Num>:4318</Num>
      </span>
    );
  }

  return (
    <div
      className="connection-status-line"
      data-testid="connection-status-line"
      style={{
        fontFamily: "var(--font-body)",
        fontSize: "12px",
        color: "var(--color-text-primary)",
        whiteSpace: "nowrap",
        overflow: "hidden",
        textOverflow: "ellipsis",
      }}
    >
      {content}
    </div>
  );
}
