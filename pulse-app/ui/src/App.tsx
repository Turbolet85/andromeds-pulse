import { useEffect, useState } from "react";
import { CanvasContainer } from "./canvas/CanvasContainer";
import { HaloCanvas } from "./halo/HaloCanvas";
import type { HaloInput } from "./halo/halo-types";
import { Titlebar } from "./components/Titlebar";

// Desktop-webview shell composition (chunk #24 + chunk #31 extension). Top-level
// layout: custom titlebar (frameless, drag region, platform-conditional window
// controls) + <main> landmark with two side-by-side WebGPU regions —
// <CanvasContainer> substrate (chunks #28/#29) + <HaloCanvas> signature
// element (chunk #31) — + <div role="status" aria-live="polite"> announcer for
// state-transition notifications.
//
// Per a11y plan §7 Landmark roles + SC 1.3.1; per design plan §Surface:
// desktop-webview Component Patterns "Navigation / App Shell".
//
// Synthetic Halo input simulator (chunk #31 placeholder): drives throughputHz
// + errorRate via two sine waves at slow cadence so the data-driven motion
// (pulse rhythm + LCH hue interpolation) is observable end-to-end without the
// real metric stream wired. Real binding via `streams.subscribe_metrics` Arrow
// IPC parser is deferred to chunks #34/#35 when the backend
// `metric.ingest.throughput_events_per_sec` +
// `metric.trace.error_rate_percent` broadcast emitters are landed. To remove
// the simulator: search for the data-testid="halo-input-simulator" anchor and
// replace with the broadcast subscription.

const SYNTHETIC_INPUT_INTERVAL_MS = 250;
const THROUGHPUT_BASE_HZ = 1000;
const THROUGHPUT_AMPLITUDE_HZ = 800;
const THROUGHPUT_FREQ = 0.0003;
const ERROR_RATE_FREQ = 0.0005;

export function App() {
  const [haloInput, setHaloInput] = useState<HaloInput>({
    throughputHz: THROUGHPUT_BASE_HZ,
    errorRate: 0,
  });

  useEffect(() => {
    document.title = "andromeda-pulse";
  }, []);

  useEffect(() => {
    const startedAt = performance.now();
    const interval = setInterval(() => {
      const elapsed = performance.now() - startedAt;
      const throughputHz = THROUGHPUT_BASE_HZ + THROUGHPUT_AMPLITUDE_HZ * Math.sin(elapsed * THROUGHPUT_FREQ);
      const errorRate = 0.5 + 0.5 * Math.sin(elapsed * ERROR_RATE_FREQ);
      setHaloInput({ throughputHz, errorRate });
    }, SYNTHETIC_INPUT_INTERVAL_MS);
    return () => clearInterval(interval);
  }, []);

  return (
    <>
      <Titlebar />
      <main
        id="main-content"
        tabIndex={-1}
        style={{
          background: "var(--color-base)",
          color: "var(--color-text-primary)",
          fontFamily: "var(--font-body)",
          minHeight: "calc(100vh - 32px)",
          display: "flex",
          gap: "var(--spacing-md)",
        }}
      >
        <div style={{ flex: 1, minWidth: 0 }}>
          <CanvasContainer ariaLabel="Telemetry visualization canvas" />
        </div>
        <div
          style={{ flex: "0 0 240px", minWidth: 0 }}
          data-testid="halo-input-simulator"
        >
          <HaloCanvas
            ariaLabel="Application status indicator"
            throughputHz={haloInput.throughputHz}
            errorRate={haloInput.errorRate}
          />
        </div>
      </main>
      <div
        role="status"
        aria-live="polite"
        id="shell-status"
        style={{
          position: "absolute",
          width: 1,
          height: 1,
          margin: -1,
          padding: 0,
          overflow: "hidden",
          clip: "rect(0, 0, 0, 0)",
          whiteSpace: "nowrap",
          border: 0,
        }}
      />
    </>
  );
}
