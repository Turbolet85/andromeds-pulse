// Webview shell lifecycle module (chunk #24). Boot-time platform detection
// spans + close→minimize-to-tray policy per arch §Cross-cutting Patterns
// "Tray icon policy" (closing the main window minimizes to tray rather than
// terminating the process). Tray icon glyph + menu land at chunk #32; this
// module supplies the structural plumbing only.
//
// Chunk #30 extends with apply_widget_settings: applies persisted Settings
// (widget_position + always_on_top) to the compact-widget window after show.

use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tauri::{Manager, PhysicalPosition, PhysicalSize, WindowEvent};
use tauri_plugin_notification::NotificationExt;
use tracing::{info, warn};
use ui_bridge::contract::{Settings, WidgetPosition};

use crate::window_geometry::{GeometryStore, WindowGeometry};

const COMPACT_WIDGET_LABEL: &str = "compact-widget";
const MAIN_WINDOW_LABEL: &str = "main";

// Aspect-ratio band for the compact glance widget (intent F2). The widget is
// 480×270 (16:9 ≈ 1.778) by default; the band keeps it "fixed-ish" without a
// hard pin. Tauri 2.11 / tao 0.35 expose no native aspect-ratio API, so the
// bound is held by a Resized-event clamp (clamp_to_aspect_bounds). The min-size
// floor in tauri.conf.json (400×225) is itself 16:9, hence inside this band.
#[doc(hidden)]
pub const WIDGET_MIN_ASPECT: f32 = 1.4;
#[doc(hidden)]
pub const WIDGET_MAX_ASPECT: f32 = 2.1;
const _: () = {
    assert!(WIDGET_MIN_ASPECT > 0.0);
    assert!(WIDGET_MIN_ASPECT < WIDGET_MAX_ASPECT);
};

// Let the resize SETTLE before clamping the aspect. Clamping on every Resized
// during a drag fights the cursor frame-by-frame and flickers badly; waiting
// for the events to stop means a single snap once the user lets go.
const ASPECT_DEBOUNCE: Duration = Duration::from_millis(150);

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

// The signpost shows on the FIRST close per session only, and only when the
// user has not disabled notifications — so close→hide-to-tray is predictable
// (intent F3) without nagging on every close. Pure seam for unit coverage.
pub fn should_show_close_signpost(notifications_enabled: bool, already_shown: bool) -> bool {
    notifications_enabled && !already_shown
}

// Emit the one-time "still running in the tray" signpost via the OS
// notification plugin (Rust-side — not gated by the webview capability ACL),
// honoring Settings.notifications_enabled per arch §OS-notification-policy.
// Best-effort: a failed dispatch never blocks close. The body is a static
// string and is never logged (security-plan §Logging NEVER-log).
fn maybe_show_close_signpost<R: tauri::Runtime>(
    window: &tauri::Window<R>,
    data_dir: &Path,
    signpost_shown: &AtomicBool,
) {
    let notifications_enabled = Settings::load_from_data_dir(data_dir).notifications_enabled;
    if !should_show_close_signpost(
        notifications_enabled,
        signpost_shown.load(Ordering::Acquire),
    ) {
        return;
    }
    signpost_shown.store(true, Ordering::Release);
    let _ = window
        .app_handle()
        .notification()
        .builder()
        .title("andromeda-pulse is still running")
        .body("Closed to the tray — right-click the tray icon to quit.")
        .show();
    info!(
        target: "tray.signpost.shown",
        "close-to-tray running-state signpost shown",
    );
}

pub fn on_window_event<R: tauri::Runtime>(
    window: &tauri::Window<R>,
    event: &WindowEvent,
    store: &Mutex<GeometryStore>,
    data_dir: &Path,
    signpost_shown: &AtomicBool,
    resize_gen: &Arc<AtomicU64>,
) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            if let Ok(store) = store.lock() {
                store.flush(data_dir);
            }
            maybe_show_close_signpost(window, data_dir, signpost_shown);
            handle_close_to_tray(window);
        }
        WindowEvent::Moved(position) => {
            let label = sanitize_window_label(window.label());
            if label != "unknown" {
                if let Ok(mut store) = store.lock() {
                    store.record_move_throttled(label, position.x, position.y, data_dir);
                }
            }
        }
        WindowEvent::Resized(size) => {
            // Aspect band applies to the glance widget only; the dashboard is
            // free-form (intent F2 names "the glance widget").
            if sanitize_window_label(window.label()) != COMPACT_WIDGET_LABEL {
                return;
            }
            // Debounce: bump the generation and clamp only once the resize
            // settles (no newer Resized within ASPECT_DEBOUNCE), so we snap
            // once on release instead of fighting the cursor every frame.
            let generation = resize_gen.fetch_add(1, Ordering::AcqRel) + 1;
            let (w0, h0) = (size.width, size.height);
            let window = window.clone();
            let resize_gen = Arc::clone(resize_gen);
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(ASPECT_DEBOUNCE).await;
                if resize_gen.load(Ordering::Acquire) != generation {
                    return;
                }
                if let Some((w, h)) =
                    clamp_to_aspect_bounds((w0, h0), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT)
                {
                    if let Err(e) = window.set_size(PhysicalSize::new(w, h)) {
                        warn!(
                            target: "app.boot.window.show",
                            label = COMPACT_WIDGET_LABEL,
                            error_kind = "set_size_failed",
                            error_msg = %e,
                            "failed to clamp compact-widget aspect ratio",
                        );
                    }
                }
            });
        }
        _ => {}
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

// Pure aspect-band clamp (intent F2). Given a physical (width, height) and the
// allowed aspect band, returns Some(corrected) when w/h falls outside
// [min_aspect, max_aspect] — reducing the offending dimension so the result
// sits inside the band — or None when already in-band (or degenerate). Reducing
// (never growing) keeps the window from being pushed off-screen; truncation
// keeps the corrected ratio within the band so the set_size echo does not
// re-clamp. Pure + unit-tested, mirroring compute_snap_position.
#[doc(hidden)]
pub fn clamp_to_aspect_bounds(
    size: (u32, u32),
    min_aspect: f32,
    max_aspect: f32,
) -> Option<(u32, u32)> {
    let (w, h) = size;
    if w == 0 || h == 0 {
        return None;
    }
    let aspect = w as f32 / h as f32;
    if aspect > max_aspect {
        let new_w = (h as f32 * max_aspect) as u32;
        Some((new_w.max(1), h))
    } else if aspect < min_aspect {
        let new_h = (w as f32 / min_aspect) as u32;
        Some((w, new_h.max(1)))
    } else {
        None
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
pub fn apply_widget_settings<R: tauri::Runtime, M: Manager<R>>(
    app: &M,
    settings: &Settings,
    geometry: &WindowGeometry,
) {
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

    // A remembered free position (from a prior drag) overrides the corner
    // snap; otherwise snap to the configured corner, and if the monitor is
    // unavailable center the window rather than leaving it at the OS
    // top-left default (intent F1 "pinned to the top-left corner").
    let layout_mode_to: &str = if let Some(pos) = geometry.position(COMPACT_WIDGET_LABEL) {
        if let Err(e) = window.set_position(PhysicalPosition::new(pos.x, pos.y)) {
            warn!(
                target: "app.boot.window.show",
                label = COMPACT_WIDGET_LABEL,
                error_kind = "set_position_failed",
                error_msg = %e,
                "failed to restore remembered widget position",
            );
        }
        "remembered"
    } else {
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
                widget_position_label(settings.widget_position)
            }
            _ => {
                if let Err(e) = window.center() {
                    warn!(
                        target: "app.boot.window.show",
                        label = COMPACT_WIDGET_LABEL,
                        error_kind = "center_failed",
                        error_msg = %e,
                        "failed to center widget on monitor-unavailable fallback",
                    );
                }
                "centered"
            }
        }
    };

    let duration_ms = start.elapsed().as_millis() as u64;
    info!(
        target: "ui.layout.transition",
        layout_mode_from = "boot_default",
        layout_mode_to,
        always_on_top = settings.always_on_top,
        duration_ms = duration_ms,
        "applied widget geometry",
    );
}

// Restore the remembered position of the main dashboard window. The window
// is created hidden + centered (tauri.conf `center: true`); a position
// remembered from a prior session overrides the centered default.
pub fn restore_main_window_position<R: tauri::Runtime, M: Manager<R>>(
    app: &M,
    geometry: &WindowGeometry,
) {
    let Some(pos) = geometry.position(MAIN_WINDOW_LABEL) else {
        return;
    };
    match app.get_webview_window(MAIN_WINDOW_LABEL) {
        Some(window) => {
            if let Err(e) = window.set_position(PhysicalPosition::new(pos.x, pos.y)) {
                warn!(
                    target: "app.boot.window.show",
                    label = MAIN_WINDOW_LABEL,
                    error_kind = "set_position_failed",
                    error_msg = %e,
                    "failed to restore main window position",
                );
            }
        }
        None => {
            warn!(
                target: "app.boot.window.show",
                label = MAIN_WINDOW_LABEL,
                error_kind = "not_found",
                "main window not registered; skipping position restore",
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
