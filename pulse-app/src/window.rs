// Webview shell lifecycle module (chunk #24). Boot-time platform detection
// spans + close→minimize-to-tray policy per arch §Cross-cutting Patterns
// "Tray icon policy" (closing the main window minimizes to tray rather than
// terminating the process). Tray icon glyph + menu land at chunk #32; this
// module supplies the structural plumbing only.
//
// Chunk #30 extends with apply_widget_settings: applies persisted Settings
// (widget_position + always_on_top) to the compact-widget window after show.

use std::time::Instant;

use tauri::{Manager, PhysicalPosition, WindowEvent};
use tracing::{info, warn};
use ui_bridge::contract::{Settings, WidgetPosition};

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

// Bounded enumeration label per WidgetPosition variant. Returned strings
// match the serde kebab-case wire form so obs allowlist `layout_mode_to`
// values stay aligned with frontend `widget_position` settings vocabulary.
pub(crate) fn widget_position_label(position: WidgetPosition) -> &'static str {
    match position {
        WidgetPosition::TopLeft => "top-left",
        WidgetPosition::TopRight => "top-right",
        WidgetPosition::BottomLeft => "bottom-left",
        WidgetPosition::BottomRight => "bottom-right",
    }
}

// Compute the per-display snap position. Inputs are absolute physical
// pixel coordinates; output is the (x, y) origin to pass to set_position
// such that the window snaps to the requested corner of the display
// containing it. "Per-display memory" is implicit: the widget always
// snaps relative to its current monitor's bounds, regardless of which
// display that is.
pub(crate) fn compute_snap_position(
    monitor_pos: (i32, i32),
    monitor_size: (u32, u32),
    window_size: (u32, u32),
    position: WidgetPosition,
) -> PhysicalPosition<i32> {
    let (mon_x, mon_y) = monitor_pos;
    let mon_w = monitor_size.0 as i32;
    let mon_h = monitor_size.1 as i32;
    let win_w = window_size.0 as i32;
    let win_h = window_size.1 as i32;
    let (x, y) = match position {
        WidgetPosition::TopLeft => (mon_x, mon_y),
        WidgetPosition::TopRight => (mon_x + mon_w - win_w, mon_y),
        WidgetPosition::BottomLeft => (mon_x, mon_y + mon_h - win_h),
        WidgetPosition::BottomRight => (mon_x + mon_w - win_w, mon_y + mon_h - win_h),
    };
    PhysicalPosition::new(x, y)
}

// Apply persisted Settings to the compact-widget window: snap position
// per WidgetPosition + current monitor, and always-on-top flag. Emits
// `ui.layout.transition` span on completion (or a warn target on
// failure paths). Best-effort: any sub-step failure logs a warn and
// continues — boot must not abort on settings-apply failure.
pub fn apply_widget_settings<R: tauri::Runtime, M: Manager<R>>(app: &M, settings: &Settings) {
    let start = Instant::now();
    let window = match app.get_webview_window(COMPACT_WIDGET_LABEL) {
        Some(w) => w,
        None => {
            warn!(
                target: "app.boot.window.show",
                label = COMPACT_WIDGET_LABEL,
                error_kind = "not_found",
                "compact-widget window not registered; skipping apply_widget_settings",
            );
            return;
        }
    };

    if let Err(e) = window.set_always_on_top(settings.always_on_top) {
        warn!(
            target: "app.boot.window.show",
            label = COMPACT_WIDGET_LABEL,
            error_kind = "set_always_on_top_failed",
            error_msg = %e,
            "failed to apply always-on-top setting",
        );
    }

    match (window.current_monitor(), window.outer_size()) {
        (Ok(Some(monitor)), Ok(outer_size)) => {
            let monitor_pos = monitor.position();
            let monitor_size = monitor.size();
            let pos = compute_snap_position(
                (monitor_pos.x, monitor_pos.y),
                (monitor_size.width, monitor_size.height),
                (outer_size.width, outer_size.height),
                settings.widget_position,
            );
            if let Err(e) = window.set_position(pos) {
                warn!(
                    target: "app.boot.window.show",
                    label = COMPACT_WIDGET_LABEL,
                    error_kind = "set_position_failed",
                    error_msg = %e,
                    "failed to apply snap position",
                );
            }
        }
        _ => {
            warn!(
                target: "app.boot.window.show",
                label = COMPACT_WIDGET_LABEL,
                error_kind = "monitor_unavailable",
                "current_monitor or outer_size unavailable; skipping snap position",
            );
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;
    info!(
        target: "ui.layout.transition",
        layout_mode_from = "boot_default",
        layout_mode_to = widget_position_label(settings.widget_position),
        always_on_top = settings.always_on_top,
        duration_ms = duration_ms,
        "applied persisted widget settings",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_webview_backend_returns_enumerated_value() {
        let v = detect_webview_backend();
        assert!(matches!(
            v,
            "WebView2" | "WKWebView" | "GTKWebKit" | "Unknown"
        ));
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

    #[test]
    fn widget_position_label_collapses_to_bounded_enum() {
        assert_eq!(widget_position_label(WidgetPosition::TopLeft), "top-left");
        assert_eq!(widget_position_label(WidgetPosition::TopRight), "top-right");
        assert_eq!(
            widget_position_label(WidgetPosition::BottomLeft),
            "bottom-left"
        );
        assert_eq!(
            widget_position_label(WidgetPosition::BottomRight),
            "bottom-right"
        );
    }

    #[test]
    fn compute_snap_position_top_left_returns_monitor_origin() {
        let pos = compute_snap_position((0, 0), (1920, 1080), (480, 270), WidgetPosition::TopLeft);
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn compute_snap_position_top_right_aligns_to_monitor_right_edge() {
        let pos = compute_snap_position((0, 0), (1920, 1080), (480, 270), WidgetPosition::TopRight);
        assert_eq!(pos.x, 1920 - 480);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn compute_snap_position_bottom_left_aligns_to_monitor_bottom_edge() {
        let pos =
            compute_snap_position((0, 0), (1920, 1080), (480, 270), WidgetPosition::BottomLeft);
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 1080 - 270);
    }

    #[test]
    fn compute_snap_position_bottom_right_aligns_to_monitor_bottom_right_corner() {
        let pos = compute_snap_position(
            (0, 0),
            (1920, 1080),
            (480, 270),
            WidgetPosition::BottomRight,
        );
        assert_eq!(pos.x, 1920 - 480);
        assert_eq!(pos.y, 1080 - 270);
    }

    #[test]
    fn compute_snap_position_respects_non_zero_monitor_origin() {
        // Secondary monitor at (1920, 0) — verifies "per-display" semantics:
        // snap math is relative to the display origin, not the global (0,0).
        let pos = compute_snap_position(
            (1920, 0),
            (1920, 1080),
            (480, 270),
            WidgetPosition::TopRight,
        );
        assert_eq!(pos.x, 1920 + 1920 - 480);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn compute_snap_position_handles_negative_monitor_origin() {
        // Monitor positioned to the LEFT of the primary (negative x).
        let pos = compute_snap_position(
            (-1920, 0),
            (1920, 1080),
            (480, 270),
            WidgetPosition::BottomLeft,
        );
        assert_eq!(pos.x, -1920);
        assert_eq!(pos.y, 1080 - 270);
    }
}
