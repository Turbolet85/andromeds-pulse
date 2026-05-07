import { useEffect } from "react";
import { Titlebar } from "./components/Titlebar";

// Desktop-webview shell composition (chunk #24). Top-level layout: custom
// titlebar (frameless, drag region, platform-conditional window controls) +
// <main> landmark + <div role="status" aria-live="polite"> announcer for
// state-transition notifications (e.g., "Minimized to tray").
//
// Per a11y plan §7 Landmark roles + SC 1.3.1; per design plan §Surface:
// desktop-webview Component Patterns "Navigation / App Shell".
export function App() {
  useEffect(() => {
    document.title = "andromeda-pulse";
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
        }}
      >
        {/* Canvas + downstream surfaces land in chunks #28+ */}
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
