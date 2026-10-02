// Global keyboard-shortcut handler for the full dashboard surface per
// layout-templates.md §IA notes "Keyboard shortcuts":
//   - Cmd+K / Ctrl+K   → toggle command palette
//   - Escape           → forwarded to onEscape (DashboardShell closes palette
//                         if open; otherwise no-op)
//
// The compact-widget ↔ dashboard toggle (Cmd+Shift+P) lives in
// `useDashboardToggleShortcut` (hooks/use-toggle-dashboard) so the in-widget
// button and the shortcut share ONE handler and the shortcut is mounted in
// BOTH windows (widget + dashboard).
//
// All listeners detached on unmount. Handler runs at window level so the
// shortcut fires even when focus is in tabpanel content or inside the
// command palette dialog.

import { useEffect } from "react";

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
