// Platform-detection hook for the Tauri 2 webview shell. Returns the
// host OS so the custom titlebar can place window controls per OS chrome
// convention (macOS LEFT traffic-light, Windows/Linux RIGHT three-button).
//
// Detection uses navigator.userAgent substring match — Tauri 2's webview
// always exposes UA. Detection runs ONCE at hook init (no subscription to
// platform changes; OS doesn't change at runtime).

export type Platform = "macos" | "windows" | "linux";

export function detectPlatform(userAgent: string): Platform {
  const ua = userAgent.toLowerCase();
  if (ua.includes("mac os") || ua.includes("macintosh")) {
    return "macos";
  }
  if (ua.includes("windows") || ua.includes("win64") || ua.includes("win32")) {
    return "windows";
  }
  return "linux";
}

export function usePlatform(): Platform {
  if (typeof navigator === "undefined") {
    return "linux";
  }
  return detectPlatform(navigator.userAgent);
}
