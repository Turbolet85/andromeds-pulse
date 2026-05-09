// Traces route — primary screen for the full dashboard. Hosts the existing
// CanvasContainer (chunks #28/#29 WebGPU substrate) + HaloCanvas (chunk #31)
// peer composition relocated from chunk #31's Dashboard root per layout-
// templates.md §Wireframe — Full dashboard which places the hero canvas
// region inside each tab's content area.
//
// haloInput flows from App.tsx → HaloInputProvider → useHaloInput here. Real
// telemetry binding via streams.subscribe_metrics deferred to chunks #34/#35.

import { CanvasContainer } from "../../canvas/CanvasContainer";
import { HaloCanvas } from "../../halo/HaloCanvas";
import { useHaloInput } from "../halo-input-context";

export function TracesRoute() {
  const haloInput = useHaloInput();
  return (
    <section
      id="tabpanel-traces"
      aria-labelledby="route-heading-traces"
      data-testid="route-traces"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-md)",
        padding: "var(--spacing-md)",
      }}
    >
      <h1
        id="route-heading-traces"
        style={{
          fontFamily: "var(--font-display)",
          fontSize: "20px",
          fontWeight: 600,
          color: "var(--color-text-primary)",
          margin: 0,
        }}
      >
        Traces
      </h1>
      <div
        style={{
          display: "flex",
          gap: "var(--spacing-md)",
          minHeight: 0,
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
      </div>
    </section>
  );
}
