// Full dashboard surface — chunk #33 shell. Wraps TanStack Router around the
// root layout (Titlebar / SkipToMain / TabNav / Outlet / FooterStatusBar /
// CommandPalette).
//
// Per layouts §Wireframe — Full dashboard, the per-service constellation map +
// trace table live in TracesRoute; the other 4 tab routes (#34/#35/#38) fill
// the remaining tabs.

import { useMemo } from "react";
import { RouterProvider } from "@tanstack/react-router";
import { createDashboardRouter } from "./router";
import { InvestigationModalForm } from "./InvestigationModalForm";
import {
  InvestigationProvider,
  useInvestigation,
} from "../hooks/use-investigation";

export function Dashboard() {
  const router = useMemo(() => createDashboardRouter(), []);
  return (
    <InvestigationProvider>
      <RouterProvider router={router} />
      <DashboardInvestigationModal />
    </InvestigationProvider>
  );
}

function DashboardInvestigationModal() {
  const { open, closeInvestigation, triggerRef } = useInvestigation();
  return (
    <InvestigationModalForm
      open={open}
      onClose={closeInvestigation}
      triggerRef={triggerRef}
    />
  );
}
