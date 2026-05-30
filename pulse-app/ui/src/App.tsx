import { useEffect } from "react";
import { useWindowLabel } from "./hooks/use-window-label";
import { useSyntheticHaloInput } from "./hooks/use-synthetic-halo-input";
import { CompactWidget } from "./widget/CompactWidget";
import { Dashboard } from "./dashboard/Dashboard";

// Window-label router (chunk #32 §Step 9 extension): branches to the
// `compact-widget` surface (chunk #32 §Step 7) for the small always-on-top
// window, otherwise falls through to the full `<Dashboard>` (extracted from
// chunk #31 App.tsx body — main window content). The compact widget sources
// its own per-service constellation data via `useServiceConstellation`
// (chunk #91, replacing the chunk #57 aggregated-metrics binding); full
// dashboard Halo input remains a synthetic simulator pending its own
// real-data binding (separate v0.2.0 chunk).

export function App() {
  const windowLabel = useWindowLabel();
  const haloInput = useSyntheticHaloInput();

  useEffect(() => {
    document.title = "andromeda-pulse";
  }, []);

  if (windowLabel === "compact-widget") {
    return <CompactWidget />;
  }
  return <Dashboard haloInput={haloInput} />;
}
