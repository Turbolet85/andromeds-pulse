// Global keyboard-shortcut handler for the full dashboard surface per
// layout-templates.md §IA notes "Keyboard shortcuts":
//   - Cmd+K / Ctrl+K   → toggle command palette
//   - Cmd+Shift+P / Ctrl+Shift+P → toggle compact widget ↔ full dashboard
//   - Escape           → forwarded to onEscape (DashboardShell closes palette
//                         if open; otherwise no-op)
//
// The compact↔full toggle invokes Tauri 2 webview API directly:
// `getAllWebviewWindows()` → resolve compact-widget + main → show inactive +
// hide active. No new TauRPC procedure (covered by core:default capability
// per pulse-app/capabilities/default.json).
//
// All listeners detached on unmount. Handler runs at window level so the
// shortcut fires even when focus is in tabpanel content or inside the
// command palette dialog.

import { useEffect } from "react";
import { getAllWebviewWindows } from "@tauri-apps/api/webviewWindow";

interface KeyboardShortcutsHookProps {
  onTogglePalette: () => void;
  onEscape: () => void;
}

export function useKeyboardShortcuts({
  onTogglePalette,
  onEscape,
}: KeyboardShortcutsHookProps): void {
  useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      const isMod = event.metaKey || event.ctrlKey;
      if (isMod && !event.shiftKey && (event.key === "k" || event.key === "K")) {
        event.preventDefault();
        onTogglePalette();
        return;
      }
      if (isMod && event.shiftKey && (event.key === "p" || event.key === "P")) {
        event.preventDefault();
        void toggleCompactDashboard();
        return;
      }
      if (event.key === "Escape") {
        onEscape();
      }
    };
    window.addEventListener("keydown", handler);
    return () => {
      window.removeEventListener("keydown", handler);
    };
  }, [onTogglePalette, onEscape]);
}

async function toggleCompactDashboard(): Promise<void> {
  let windows;
  try {
    windows = await getAllWebviewWindows();
  } catch {
    return;
  }
  const compact = windows.find((w) => w.label === "compact-widget");
  const main = windows.find((w) => w.label === "main");
  if (!compact || !main) return;
  let compactVisible = false;
  let mainVisible = false;
  try {
    compactVisible = await compact.isVisible();
  } catch {
    compactVisible = false;
  }
  try {
    mainVisible = await main.isVisible();
  } catch {
    mainVisible = false;
  }
  if (compactVisible) {
    await main.show();
    await main.setFocus();
    await compact.hide();
  } else if (mainVisible) {
    await compact.show();
    await compact.setFocus();
    await main.hide();
  } else {
    await main.show();
    await main.setFocus();
  }
}
