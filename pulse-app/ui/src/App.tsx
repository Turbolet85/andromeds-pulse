import { useEffect } from "react";
import { useWindowLabel } from "./hooks/use-window-label";
import { useSyntheticHaloInput } from "./hooks/use-synthetic-halo-input";
import { useSyntheticWidgetMetrics } from "./hooks/use-synthetic-widget-metrics";
import { CompactWidget } from "./widget/CompactWidget";
import { Dashboard } from "./dashboard/Dashboard";

// Window-label router (chunk #32 §Step 9 extension): branches to the
// `compact-widget` surface (chunk #32 §Step 7) for the small always-on-top
// window, otherwise falls through to the full `<Dashboard>` (extracted from
// chunk #31 App.tsx body — main window content). Synthetic data simulators
// run as hooks; both surfaces consume their respective shapes. Real binding
// via `streams.subscribe_metrics` Arrow IPC parser deferred to chunks
// #34/#35 (search for `data-testid="halo-input-simulator"` to find the
// replacement anchor).

export function App() {
  const windowLabel = useWindowLabel();
  const haloInput = useSyntheticHaloInput();
  const widgetMetrics = useSyntheticWidgetMetrics();

  useEffect(() => {
    document.title = "andromeda-pulse";
  }, []);

  if (windowLabel === "compact-widget") {
    return <CompactWidget metrics={widgetMetrics} />;
  }
  return <Dashboard haloInput={haloInput} />;
}
