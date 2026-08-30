// Migrated 2026-08-30 from `pulse-app/src/window.rs::tests` — that crate
// sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS. Internals reach here
// via `pub` + `#[doc(hidden)]` per test-plan §2/§4.

use pulse_app::window::{
    compute_snap_position, detect_tray_api, detect_webview_backend, detect_wgpu_backend,
    sanitize_window_label, widget_position_label,
};
use ui_bridge::contract::WidgetPosition;

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
    assert_eq!(sanitize_window_label("findings"), "findings");
    assert_eq!(sanitize_window_label("report"), "report");
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
fn compute_snap_position_top_left_insets_by_margin_from_origin() {
    let pos = compute_snap_position((0, 0), (1920, 1080), (480, 270), WidgetPosition::TopLeft);
    assert_eq!(pos.x, 24);
    assert_eq!(pos.y, 24);
}

#[test]
fn compute_snap_position_top_right_insets_from_right_edge() {
    let pos = compute_snap_position((0, 0), (1920, 1080), (480, 270), WidgetPosition::TopRight);
    assert_eq!(pos.x, 1920 - 480 - 24);
    assert_eq!(pos.y, 24);
}

#[test]
fn compute_snap_position_bottom_left_insets_from_bottom_edge() {
    let pos = compute_snap_position((0, 0), (1920, 1080), (480, 270), WidgetPosition::BottomLeft);
    assert_eq!(pos.x, 24);
    assert_eq!(pos.y, 1080 - 270 - 24);
}

#[test]
fn compute_snap_position_bottom_right_insets_from_bottom_right_corner() {
    let pos = compute_snap_position(
        (0, 0),
        (1920, 1080),
        (480, 270),
        WidgetPosition::BottomRight,
    );
    assert_eq!(pos.x, 1920 - 480 - 24);
    assert_eq!(pos.y, 1080 - 270 - 24);
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
    assert_eq!(pos.x, 1920 + 1920 - 480 - 24);
    assert_eq!(pos.y, 24);
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
    assert_eq!(pos.x, -1920 + 24);
    assert_eq!(pos.y, 1080 - 270 - 24);
}
