// Webview shell lifecycle module (chunk #24). Boot-time platform detection
// spans + close→minimize-to-tray policy per arch §Cross-cutting Patterns
// "Tray icon policy" (closing the main window minimizes to tray rather than
// terminating the process). Tray icon glyph + menu land at chunk #32; this
// module supplies the structural plumbing only.

use tauri::{Manager, WindowEvent};
use tracing::{info, warn};

const COMPACT_WIDGET_LABEL: &str = "compact-widget";
const MAIN_WINDOW_LABEL: &str = "main";

pub fn detect_webview_backend() -> &'static str {
    if cfg!(target_os = "windows") {
        "WebView2"
    } else if cfg!(target_os = "macos") {
        "WKWebView"
    } else if cfg!(target_os = "linux") {
        "GTKWebKit"
    } else {
        "Unknown"
    }
}

pub fn detect_tray_api() -> &'static str {
    if cfg!(target_os = "windows") {
        "NotifyIcon"
    } else if cfg!(target_os = "macos") {
        "NSStatusItem"
    } else if cfg!(target_os = "linux") {
        "AppIndicator"
    } else {
        "Unknown"
    }
}

pub fn detect_wgpu_backend() -> &'static str {
    if cfg!(target_os = "windows") {
        "dx12"
    } else if cfg!(target_os = "macos") {
        "metal"
    } else {
        "vulkan"
    }
}

pub fn emit_boot_spans() {
    info!(
        target: "app.boot.webview.init",
        webview_backend = detect_webview_backend(),
        "webview backend detected at boot",
    );
    info!(
        target: "app.boot.gpu.check",
        gpu_available = false,
        wgpu_backend = detect_wgpu_backend(),
        "GPU adapter check (boot-time pre-render; runtime adapter check at chunk #28)",
    );
    info!(
        target: "app.boot.tray.init",
        tray_api = detect_tray_api(),
        "tray API selected at boot (icon registration deferred to chunk #32)",
    );
}

// Bounded enumeration of the 2 declared windows in tauri.conf.json. Anything
// else collapses to "unknown" so the obs allowlist sees a bounded label.
fn sanitize_window_label(label: &str) -> &'static str {
    match label {
        MAIN_WINDOW_LABEL => MAIN_WINDOW_LABEL,
        COMPACT_WIDGET_LABEL => COMPACT_WIDGET_LABEL,
        _ => "unknown",
    }
}

// Hide the window (instead of terminating the process) and emit the
// must-trace P5 transition span per obs-plan §1 P5 row.
pub fn handle_close_to_tray<R: tauri::Runtime>(window: &tauri::Window<R>) {
    let layout_mode_from = sanitize_window_label(window.label());
    let hide_ok = window.hide().is_ok();
    info!(
        target: "ui.layout.transition",
        layout_mode_from,
        layout_mode_to = "hidden",
        tray_visible = hide_ok,
        "window close→minimize-to-tray",
    );
}

pub fn on_window_event<R: tauri::Runtime>(window: &tauri::Window<R>, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        handle_close_to_tray(window);
    }
}

// Show the compact widget after Tauri setup finishes. Failure is non-fatal
// (boot continues) but emits a warn so the harness can detect.
pub fn show_compact_widget<R: tauri::Runtime, M: Manager<R>>(app: &M) {
    match app.get_webview_window(COMPACT_WIDGET_LABEL) {
        Some(w) => {
            if let Err(e) = w.show() {
                warn!(
                    target: "app.boot.window.show",
                    label = COMPACT_WIDGET_LABEL,
                    error_kind = "show_failed",
                    error_msg = %e,
                    "failed to show compact-widget window",
                );
            }
        }
        None => {
            warn!(
                target: "app.boot.window.show",
                label = COMPACT_WIDGET_LABEL,
                error_kind = "not_found",
                "compact-widget window not registered in tauri config",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_webview_backend_returns_enumerated_value() {
        let v = detect_webview_backend();
        assert!(matches!(v, "WebView2" | "WKWebView" | "GTKWebKit" | "Unknown"));
    }

    #[test]
    fn detect_tray_api_returns_enumerated_value() {
        let v = detect_tray_api();
        assert!(matches!(
            v,
            "NotifyIcon" | "NSStatusItem" | "AppIndicator" | "Unknown"
        ));
    }

    #[test]
    fn detect_wgpu_backend_returns_enumerated_value() {
        let v = detect_wgpu_backend();
        assert!(matches!(v, "dx12" | "metal" | "vulkan"));
    }

    #[test]
    fn detect_webview_backend_matches_target_os() {
        let v = detect_webview_backend();
        if cfg!(target_os = "windows") {
            assert_eq!(v, "WebView2");
        } else if cfg!(target_os = "macos") {
            assert_eq!(v, "WKWebView");
        } else if cfg!(target_os = "linux") {
            assert_eq!(v, "GTKWebKit");
        }
    }

    #[test]
    fn detect_tray_api_matches_target_os() {
        let v = detect_tray_api();
        if cfg!(target_os = "windows") {
            assert_eq!(v, "NotifyIcon");
        } else if cfg!(target_os = "macos") {
            assert_eq!(v, "NSStatusItem");
        } else if cfg!(target_os = "linux") {
            assert_eq!(v, "AppIndicator");
        }
    }

    #[test]
    fn sanitize_window_label_collapses_unknown_to_constant() {
        assert_eq!(sanitize_window_label("main"), "main");
        assert_eq!(sanitize_window_label("compact-widget"), "compact-widget");
        assert_eq!(sanitize_window_label("evil-injection-attempt"), "unknown");
        assert_eq!(sanitize_window_label(""), "unknown");
    }
}
