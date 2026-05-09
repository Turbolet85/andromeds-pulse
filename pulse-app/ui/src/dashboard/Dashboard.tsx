// Full dashboard surface — chunk #33 shell. Wraps TanStack Router around the
// root layout (Titlebar / SkipToMain / TabNav / Outlet / FooterStatusBar /
// CommandPalette) and surfaces the synthetic HaloInput stream from App.tsx
// to TracesRoute via React context.
//
// Per layouts §Wireframe — Full dashboard, the existing CanvasContainer +
// HaloCanvas peer composition (chunks #28/#29 substrate + #31 Halo signature)
// relocates into TracesRoute; subsequent chunks (#34/#35/#38) fill the
// other 4 tab routes.

import { useMemo } from "react";
import { RouterProvider } from "@tanstack/react-router";
import { createDashboardRouter } from "./router";
import { HaloInputProvider } from "./halo-input-context";
import type { HaloInput } from "../halo/halo-types";

interface DashboardProps {
  haloInput: HaloInput;
}

export function Dashboard({ haloInput }: DashboardProps) {
  const router = useMemo(() => createDashboardRouter(), []);
  return (
    <HaloInputProvider value={haloInput}>
      <RouterProvider router={router} />
    </HaloInputProvider>
  );
}
