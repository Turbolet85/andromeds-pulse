import { useEffect } from "react";
import { useWindowLabel } from "./hooks/use-window-label";
import { useSyntheticHaloInput } from "./hooks/use-synthetic-halo-input";
import { useWidgetMetrics } from "./hooks/use-widget-metrics";
import { CompactWidget } from "./widget/CompactWidget";
import { Dashboard } from "./dashboard/Dashboard";

// Window-label router (chunk #32 §Step 9 extension): branches to the
// `compact-widget` surface (chunk #32 §Step 7) for the small always-on-top
// window, otherwise falls through to the full `<Dashboard>` (extracted from
// chunk #31 App.tsx body — main window content). Compact widget consumes
// real `streams.subscribe_metrics` data via `useWidgetMetrics` (chunk #57);
// full dashboard Halo input remains a synthetic simulator pending its own
// real-data binding (separate v0.2.0 chunk; chunk #81 will refactor the
// Halo data path).

export function App() {
  const windowLabel = useWindowLabel();
  const haloInput = useSyntheticHaloInput();
  const widgetMetrics = useWidgetMetrics();

  useEffect(() => {
    document.title = "andromeda-pulse";
  }, []);

  if (windowLabel === "compact-widget") {
    return <CompactWidget metrics={widgetMetrics} />;
  }
  return <Dashboard haloInput={haloInput} />;
}
