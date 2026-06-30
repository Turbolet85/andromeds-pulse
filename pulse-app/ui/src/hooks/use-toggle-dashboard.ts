// Toggle the full dashboard window from the glance widget: open + focus it when
// hidden, hide it when visible. The glance widget itself always stays put. The
// show/hide/focus ops require core:window:allow-show / allow-set-focus /
// allow-hide in pulse-app/capabilities/default.json (negative-default — without
// the grant Tauri silently rejects them; the P-061/P-063 trap).

import { useEffect } from "react";
import { getAllWebviewWindows } from "@tauri-apps/api/webviewWindow";

export async function toggleDashboard(): Promise<void> {
  let windows;
  try {
    windows = await getAllWebviewWindows();
  } catch {
    return;
  }
  const main = windows.find((w) => w.label === "main");
  if (!main) return;
  let visible = false;
  try {
    visible = await main.isVisible();
  } catch {
    visible = false;
  }
  if (visible) {
    await main.hide();
  } else {
    await main.show();
    await main.setFocus();
  }
}

function handleToggleDashboard(): void {
  void toggleDashboard();
}

export function useToggleDashboard(): () => void {
  return handleToggleDashboard;
}

// Cmd/Ctrl+Shift+P toggles the dashboard via the SAME toggleDashboard() handler
// as the in-widget button. Mount in BOTH windows (widget + dashboard) so the
// shortcut works from either and never falls through to the WebView (the
// unmounted-on-widget case leaked to the browser print dialog).
export function useDashboardToggleShortcut(): void {
  useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      const isMod = event.metaKey || event.ctrlKey;
      if (isMod && event.shiftKey && (event.key === "p" || event.key === "P")) {
        event.preventDefault();
        void toggleDashboard();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, []);
}
