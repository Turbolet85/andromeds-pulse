// Webview shell lifecycle module (chunk #24). Boot-time platform detection
// spans + close→minimize-to-tray policy per arch §Cross-cutting Patterns
// "Tray icon policy" (closing the main window minimizes to tray rather than
// terminating the process). Tray icon glyph + menu land at chunk #32; this
// module supplies the structural plumbing only.
//
// Chunk #30 extends with apply_widget_settings: applies persisted Settings
// (widget_position + always_on_top) to the compact-widget window after show.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tauri::{LogicalSize, Manager, PhysicalPosition, PhysicalSize, WindowEvent};
use tauri_plugin_notification::NotificationExt;
use tracing::{info, warn};
use ui_bridge::contract::{Settings, WidgetPosition};

use crate::window_geometry::{GeometryStore, WindowGeometry};

const COMPACT_WIDGET_LABEL: &str = "compact-widget";
const MAIN_WINDOW_LABEL: &str = "main";
const FINDINGS_WINDOW_LABEL: &str = "findings";
const REPORT_WINDOW_LABEL: &str = "report";

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

// Calibrated default size for the compact glance widget — matches tauri.conf
// (480×270, 16:9, inside the P-062 min-size 400×225 + aspect band). Re-asserted
// at boot so the widget never sizes to content or inherits a stale geometry.
const WIDGET_DEFAULT_WIDTH: f64 = 480.0;
const WIDGET_DEFAULT_HEIGHT: f64 = 270.0;

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
        wgpu_backend = detect_wgpu_backend(),
        "compile-target default wgpu backend; no adapter probe runs here (the frame loop's adapter branch is the adapter evidence)",
    );
    info!(
        target: "app.boot.tray.init",
        tray_api = detect_tray_api(),
        "tray API selected at boot (icon registration deferred to chunk #32)",
    );
}

// Bounded enumeration of the 2 declared windows in tauri.conf.json. Anything
// else collapses to "unknown" so the obs allowlist sees a bounded label.
#[doc(hidden)]
pub fn sanitize_window_label(label: &str) -> &'static str {
    match label {
        MAIN_WINDOW_LABEL => MAIN_WINDOW_LABEL,
        COMPACT_WIDGET_LABEL => COMPACT_WIDGET_LABEL,
        FINDINGS_WINDOW_LABEL => FINDINGS_WINDOW_LABEL,
        REPORT_WINDOW_LABEL => REPORT_WINDOW_LABEL,
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

// The signpost fires EVERY time the app goes fully to the tray (the deliberate,
// infrequent widget-close — P-063 toast correction), gated only on the user's
// notification preference. No first-close latch: the trigger is rare, so
// every-time confirms the hide-to-tray without nagging. Pure seam.
pub fn should_show_close_signpost(notifications_enabled: bool) -> bool {
    notifications_enabled
}

// Primary/secondary close model (P-063 correction): the compact widget is the
// primary surface, the dashboard secondary. Closing the WIDGET sends the whole
// app to the tray (hide both windows + the first-close signpost); closing the
// DASHBOARD merely collapses back to the widget (no extra hide, no signpost).
// Pure seam for unit coverage.
pub fn close_sends_app_to_tray(label: &str) -> bool {
    label == COMPACT_WIDGET_LABEL
}

// Emit the "still running in the tray" signpost via the OS notification
// plugin (Rust-side — not gated by the webview capability ACL), honoring
// Settings.notifications_enabled per arch §OS-notification-policy. Fires on
// EVERY widget close (see should_show_close_signpost — no first-close latch).
// Best-effort: a failed dispatch never blocks close. The body is a static
// string and is never logged (security-plan §Logging NEVER-log); the record
// carries only the bounded window label so the verification harness can
// assert a field rather than bare record presence.
fn maybe_show_close_signpost<R: tauri::Runtime>(window: &tauri::Window<R>, data_dir: &Path) {
    let notifications_enabled = Settings::load_from_data_dir(data_dir).notifications_enabled;
    if !should_show_close_signpost(notifications_enabled) {
        return;
    }
    let _ = window
        .app_handle()
        .notification()
        .builder()
        .title("andromeda-pulse is still running")
        .body("Closed to the tray — right-click the tray icon to quit.")
        .show();
    info!(
        target: "tray.signpost.shown",
        window_label = sanitize_window_label(window.label()),
        "close-to-tray running-state signpost shown",
    );
}

pub fn on_window_event<R: tauri::Runtime>(
    window: &tauri::Window<R>,
    event: &WindowEvent,
    store: &Mutex<GeometryStore>,
    data_dir: &Path,
    resize_gen: &Arc<AtomicU64>,
) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            if let Ok(store) = store.lock() {
                store.flush(data_dir);
            }
            if close_sends_app_to_tray(window.label()) {
                // Widget (primary) closed → whole app to the tray: also hide the
                // dashboard (if open) so no window stays visible, then fire the
                // every-time "still running" signpost. The dashboard close
                // (secondary) just collapses to the widget — no extra hide, no
                // signpost.
                if let Some(main) = window.app_handle().get_webview_window(MAIN_WINDOW_LABEL) {
                    let _ = main.hide();
                }
                maybe_show_close_signpost(window, data_dir);
            }
            handle_close_to_tray(window);
        }
        // Only the dashboard's position is remembered; the compact widget always
        // boots to its fixed corner (P-061 correction — no widget remembered
        // geometry to restore or drift off-screen).
        WindowEvent::Moved(position)
            if sanitize_window_label(window.label()) == MAIN_WINDOW_LABEL =>
        {
            if let Ok(mut store) = store.lock() {
                store.record_move_throttled(MAIN_WINDOW_LABEL, position.x, position.y, data_dir);
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

// Every window label declared in tauri.conf.json. Bounded by construction —
// these ARE the sanitized set, so they can be emitted as labels directly.
const ALL_WINDOW_LABELS: [&str; 4] = [
    COMPACT_WIDGET_LABEL,
    MAIN_WINDOW_LABEL,
    FINDINGS_WINDOW_LABEL,
    REPORT_WINDOW_LABEL,
];

const BLANK_URL: &str = "about:blank";

// Long enough that an ordinary first navigation has completed, so a window
// still blank here lost it rather than being mid-flight.
const NAVIGATION_SETTLE: Duration = Duration::from_secs(5);

/// Record, once per boot, whether each declared window actually navigated.
///
/// A webview that loses its initial navigation stays on `about:blank` and
/// nothing re-navigates it — `show()` is not navigation — so it surfaces as a
/// blank window whenever it is first opened. That state is otherwise invisible:
/// the app boots, ticks, and logs normally, and no shipped smoke reads a
/// per-webview URL. This emits a positive record per window so a healthy boot is
/// evidence rather than mere absence of a complaint.
pub fn spawn_navigation_check<R: tauri::Runtime>(app: tauri::AppHandle<R>) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(NAVIGATION_SETTLE).await;
        for label in ALL_WINDOW_LABELS {
            let (navigated, reason) = match app.get_webview_window(label) {
                None => (false, "window_absent"),
                Some(window) => match window.url() {
                    Err(_) => (false, "url_unavailable"),
                    Ok(url) if url.as_str() == BLANK_URL => (false, "blank"),
                    Ok(_) => (true, "navigated"),
                },
            };
            if navigated {
                info!(
                    target: "app.boot.window.navigation",
                    window_label = label,
                    navigated = true,
                    reason = reason,
                    "webview navigated",
                );
            } else {
                warn!(
                    target: "app.boot.window.navigation",
                    window_label = label,
                    navigated = false,
                    reason = reason,
                    "webview never navigated; this window renders blank when shown",
                );
            }
        }
    });
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
#[doc(hidden)]
pub fn widget_position_label(position: WidgetPosition) -> &'static str {
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

// Inset from the monitor edge so the snapped widget never sits flush against a
// screen edge or the taskbar (P-061 correction — "a corner with a margin").
const WIDGET_EDGE_MARGIN: i32 = 24;

// Compute the per-display snap position. Inputs are absolute physical
// pixel coordinates; output is the (x, y) origin to pass to set_position
// such that the window snaps to the requested corner of the display
// containing it, inset by WIDGET_EDGE_MARGIN. "Per-display memory" is implicit:
// the widget always snaps relative to its current monitor's bounds, regardless
// of which display that is.
#[doc(hidden)]
pub fn compute_snap_position(
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
    let m = WIDGET_EDGE_MARGIN;
    let (x, y) = match position {
        WidgetPosition::TopLeft => (mon_x + m, mon_y + m),
        WidgetPosition::TopRight => (mon_x + mon_w - win_w - m, mon_y + m),
        WidgetPosition::BottomLeft => (mon_x + m, mon_y + mon_h - win_h - m),
        WidgetPosition::BottomRight => (mon_x + mon_w - win_w - m, mon_y + mon_h - win_h - m),
    };
    PhysicalPosition::new(x, y)
}

// Apply persisted Settings to the compact-widget window: re-assert the
// calibrated default size, snap to the configured corner (margin-inset) of the
// current monitor, and the always-on-top flag. The buggy remembered
// free-position restore was dropped (P-061 correction) — a stale off-screen x/y
// could spawn the widget partly or fully off-screen. Emits `ui.layout.transition`
// on completion. Best-effort: any sub-step failure logs a warn and continues —
// boot must not abort on settings-apply failure.
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

    // Re-assert the calibrated fixed size so the widget never sizes to content
    // or inherits a stale geometry (P-061 correction) — 480×270 logical, matching
    // tauri.conf (inside the P-062 min-size + aspect band, so no re-clamp).
    if let Err(e) = window.set_size(LogicalSize::new(
        WIDGET_DEFAULT_WIDTH,
        WIDGET_DEFAULT_HEIGHT,
    )) {
        warn!(
            target: "app.boot.window.show",
            label = COMPACT_WIDGET_LABEL,
            error_kind = "set_size_failed",
            error_msg = %e,
            "failed to apply default widget size",
        );
    }

    // Fixed on-screen position: snap to the configured corner (margin-inset) of
    // the current monitor — sized from the known default × scale so it is correct
    // regardless of any pending resize — or center when the monitor is
    // unavailable. No remembered free-position restore (P-061 correction).
    let layout_mode_to: &str = match window.current_monitor() {
        Ok(Some(monitor)) => {
            let monitor_pos = monitor.position();
            let monitor_size = monitor.size();
            let scale = monitor.scale_factor();
            let win_w = (WIDGET_DEFAULT_WIDTH * scale).round() as u32;
            let win_h = (WIDGET_DEFAULT_HEIGHT * scale).round() as u32;
            let pos = compute_snap_position(
                (monitor_pos.x, monitor_pos.y),
                (monitor_size.width, monitor_size.height),
                (win_w, win_h),
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

// Tests migrated to `pulse-app/tests/unit_window_shell.rs` — a src-level `mod tests`
// compiles but never runs under `[lib] test = false` (2026-05-20 precedent).
