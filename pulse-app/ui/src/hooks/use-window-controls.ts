// Window control bridge to the Tauri 2 JS API. Webview-side calls
// `getCurrentWindow().minimize()` / `.toggleMaximize()` / `.close()`; the
// Rust side intercepts CloseRequested in `pulse-app/src/window.rs` and
// hides the window instead of terminating the process per arch §Cross-cutting
// Patterns Tray icon policy.

import { getCurrentWindow } from "@tauri-apps/api/window";

export interface WindowControls {
  minimize: () => Promise<void>;
  maximize: () => Promise<void>;
  close: () => Promise<void>;
}

export function useWindowControls(): WindowControls {
  return {
    minimize: () => getCurrentWindow().minimize(),
    maximize: () => getCurrentWindow().toggleMaximize(),
    close: () => getCurrentWindow().close(),
  };
}
