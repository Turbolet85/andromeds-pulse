// Main-window content extracted from chunk #31 App.tsx body. The full
// dashboard surface keeps the chunks #28/#29 CanvasContainer substrate +
// chunk #31 HaloCanvas peer composition; chunk #33 (Full dashboard shell +
// tab nav) layers TanStack Router + tabs on top.
//
// Visually-hidden shell-status announcer (`role="status" aria-live="polite"`)
// stays here because the compact-widget surface uses FooterBand as its
// richer status surface — App.tsx is now a clean window-label router with
// no status surfaces of its own.

import { CanvasContainer } from "../canvas/CanvasContainer";
import { HaloCanvas } from "../halo/HaloCanvas";
import { Titlebar } from "../components/Titlebar";
import type { HaloInput } from "../halo/halo-types";

interface DashboardProps {
  haloInput: HaloInput;
}

export function Dashboard({ haloInput }: DashboardProps) {
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
