// Tauri 2 webview window-label detection hook for App.tsx routing between
// `compact-widget` and `main` window content. Mirrors the Rust-side bounded
// set in `pulse-app/src/window.rs::sanitize_window_label` — anything outside
// the 2 declared labels collapses to `"unknown"` so the obs allowlist (and
// caller routing logic) sees a bounded enum.
//
// Detection runs ONCE at hook init via lazy initializer — Tauri assigns the
// window label during webview boot and it does not change at runtime. In
// jsdom test environments without Tauri context, `getCurrentWebviewWindow()`
// throws (no `window.__TAURI_INTERNALS__`); the try/catch falls back to
// `"unknown"` so App.tsx routes to `<Dashboard>` (safer default than crashing).

import { useState } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export type WindowLabel = "compact-widget" | "main" | "findings" | "report" | "unknown";

export function sanitizeWindowLabel(label: string): WindowLabel {
  if (
    label === "compact-widget" ||
    label === "main" ||
    label === "findings" ||
    label === "report"
  ) {
    return label;
  }
  return "unknown";
}

export function useWindowLabel(): WindowLabel {
  const [label] = useState<WindowLabel>(() => {
    try {
      const window = getCurrentWebviewWindow();
      return sanitizeWindowLabel(window.label);
    } catch {
      return "unknown";
    }
  });
  return label;
}
