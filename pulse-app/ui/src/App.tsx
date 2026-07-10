import { useEffect } from "react";
import { useWindowLabel } from "./hooks/use-window-label";
import { useSuppressBrowserChrome } from "./hooks/use-suppress-browser-chrome";
import { CompactWidget } from "./widget/CompactWidget";
import { FindingsWindow } from "./widget/FindingsWindow";
import { ReportWindow } from "./widget/ReportWindow";
import { Dashboard } from "./dashboard/Dashboard";

// Window-label router (chunk #32 §Step 9 extension): branches to the
// `compact-widget` surface (chunk #32 §Step 7) for the small always-on-top
// window, otherwise falls through to the full `<Dashboard>` (extracted from
// chunk #31 App.tsx body — main window content). Both surfaces source their
// own per-service constellation data via `useServiceConstellation` (compact
// widget chunk #91, full dashboard chunk #93).

export function App() {
  const windowLabel = useWindowLabel();

  useSuppressBrowserChrome();

  useEffect(() => {
    document.title = "andromeda-pulse";
  }, []);

  if (windowLabel === "compact-widget") {
    return <CompactWidget />;
  }
  if (windowLabel === "findings") {
    return <FindingsWindow />;
  }
  if (windowLabel === "report") {
    return <ReportWindow />;
  }
  return <Dashboard />;
}
