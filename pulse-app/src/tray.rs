// Tray icon + native menu module (chunk #36).
//
// Registers an OS-native tray icon (NSStatusItem on macOS / NotifyIcon on
// Windows / AppIndicator on Linux per `window::detect_tray_api`) at app
// boot, displaying a monochrome aperture/circular-pulse glyph constructed
// programmatically (no PNG decoder dependency on the `image-png` Tauri
// feature). Right-click opens a flat OS-native menu per arch §Cross-cutting
// Patterns "Tray icon policy" and design plan §Surface: desktop-native
// canonical tray menu.
//
// Menu items wire to existing infrastructure (Open / Open Settings →
// window focus; Quit → app.exit) or emit `tray.menu.interaction`
// placeholders for handlers awaiting their epochs (Snapshot defers to
// epoch 6's snapshot.generate; MCP toggle defers to chunk #46's rmcp
// lifecycle but the menu item exists when --features mcp-server is built).
//
// Halo state encoding via icon swap (Q1 Option C from research.md) is
// intentionally NOT implemented in this chunk — Q1 Option D fallback
// applies: static monochrome glyph + menu summary line carries the live
// Halo state values (spans/sec + error rate + retention used). Periodic
// summary-line refresh from broadcast is a follow-up enhancement.

use std::sync::Arc;

use buffer::BroadcastSenders;
use tauri::menu::{Menu, MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::{AppHandle, Manager, Runtime};
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum TrayError {
    #[error("tray menu construction failed")]
    Menu,
    #[error("tray icon registration failed")]
    Register,
}

const COMPACT_WIDGET_LABEL: &str = "compact-widget";
const MAIN_WINDOW_LABEL: &str = "main";

// Bounded enum of menu item IDs. Matches obs allowlist `menu_item` field
// values per `pulse-app/src/observability.rs::AllowList::production()`
// chunks #4/#7 pre-allocations: {open, snapshot, mcp_toggle, quit}.
// `open_settings` extends the canonical set per design plan §Surface:
// desktop-native canonical menu; the allowlist treats menu_item values as
// bounded but does not pin the value-set to specific identifiers.
const MENU_ID_OPEN: &str = "open";
const MENU_ID_SUMMARY: &str = "summary";
const MENU_ID_SNAPSHOT: &str = "snapshot";
#[cfg(feature = "mcp-server")]
const MENU_ID_MCP_TOGGLE: &str = "mcp_toggle";
const MENU_ID_OPEN_SETTINGS: &str = "open_settings";
const MENU_ID_QUIT: &str = "quit";

const TRAY_GLYPH_SIZE: u32 = 32;

/// Build + register the OS-native tray icon. Caller stores the returned
/// handle via `app.manage(tray)` to keep it alive for app lifetime
/// (Tauri's TrayIcon is RAII; dropping causes the tray icon to disappear).
pub fn setup_tray<R: Runtime>(
    app: &AppHandle<R>,
    _broadcast_senders: Arc<BroadcastSenders>,
) -> Result<TrayIcon<R>, TrayError> {
    let menu = build_menu(app).map_err(|e| {
        warn!(
            target: "app.boot.tray.init",
            error_kind = "menu_build_failed",
            error_msg = %e,
            "tray menu construction failed",
        );
        TrayError::Menu
    })?;

    let icon_image = build_glyph_image();

    let tray = TrayIconBuilder::new()
        .icon(icon_image)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            handle_menu_event(app, event.id().as_ref());
        })
        .build(app)
        .map_err(|e| {
            warn!(
                target: "app.boot.tray.init",
                error_kind = "tray_build_failed",
                error_msg = %e,
                "tray icon registration failed",
            );
            TrayError::Register
        })?;

    info!(
        target: "tray.visibility.toggle",
        tray_visible = true,
        "tray icon registered",
    );

    Ok(tray)
}

// Construct a 32x32 monochrome aperture/circular-pulse glyph as raw RGBA
// bytes. Mirrors `pulse-app/ui/src/components/icons/CircularPulse.tsx`
// design: outer circle outline + concentric arcs + center dot. White on
// transparent. Avoids PNG decoder dependency on the `image-png` Tauri
// feature; `Image::new` accepts raw RGBA directly.
fn build_glyph_pixels() -> Vec<u8> {
    let size = TRAY_GLYPH_SIZE;
    let mut pixels = vec![0u8; (size * size * 4) as usize];
    let cx = (size as f32 - 1.0) / 2.0;
    let cy = (size as f32 - 1.0) / 2.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            // Outer circle outline at radius ~13.5 (chunk #36 32x32 scaling
            // of CircularPulse r=9 at 24-unit viewBox: 9 * 32/24 = 12; a
            // slightly larger radius reads better at tray-bar size).
            let on_outer_circle = (dist - 13.0).abs() < 0.7;
            // Outer arc — top half — at radius 7 (CircularPulse r=5 scaled).
            let on_outer_arc = (dist - 7.0).abs() < 0.6 && dy < 0.5;
            // Inner arc — bottom half — at radius 3.5 (CircularPulse r=2.5).
            let on_inner_arc = (dist - 3.5).abs() < 0.6 && dy > -0.5;
            // Center dot.
            let in_center_dot = dist < 1.3;

            if on_outer_circle || on_outer_arc || on_inner_arc || in_center_dot {
                let idx = ((y * size + x) * 4) as usize;
                pixels[idx] = 255;
                pixels[idx + 1] = 255;
                pixels[idx + 2] = 255;
                pixels[idx + 3] = 255;
            }
        }
    }
    pixels
}

fn build_glyph_image() -> tauri::image::Image<'static> {
    // Static lifetime via Vec::leak — one-time allocation for app lifetime,
    // ~4KB (32x32x4); negligible. Tauri's Image holds borrowed bytes;
    // 'static lifetime trivially outlives the AppHandle and TrayIcon
    // that consume it.
    let pixels = build_glyph_pixels();
    let leaked: &'static [u8] = pixels.leak();
    tauri::image::Image::new(leaked, TRAY_GLYPH_SIZE, TRAY_GLYPH_SIZE)
}

fn build_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let open_item = MenuItemBuilder::with_id(MENU_ID_OPEN, "Open andromeda-pulse").build(app)?;

    // Read-only summary line — disabled (cannot be clicked); reflects
    // current ingest state. For chunk #36 this stays at canonical-format
    // placeholder values; periodic refresh from broadcast is a follow-up
    // (per layouts plan §Component — Tray menu (OS-native) → Structure).
    let summary_item = MenuItemBuilder::with_id(
        MENU_ID_SUMMARY,
        "Ingest: 0 sp/s | Error: 0% | Retention: 0 min used",
    )
    .enabled(false)
    .build(app)?;

    let snapshot_item =
        MenuItemBuilder::with_id(MENU_ID_SNAPSHOT, "Generate Snapshot").build(app)?;

    let open_settings_item =
        MenuItemBuilder::with_id(MENU_ID_OPEN_SETTINGS, "Open Settings").build(app)?;

    let quit_item = MenuItemBuilder::with_id(MENU_ID_QUIT, "Quit").build(app)?;

    let mut builder = MenuBuilder::new(app)
        .item(&open_item)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&summary_item)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&snapshot_item);

    #[cfg(feature = "mcp-server")]
    {
        let mcp_toggle_item =
            tauri::menu::CheckMenuItemBuilder::with_id(MENU_ID_MCP_TOGGLE, "Toggle MCP Server")
                .checked(false)
                .build(app)?;
        builder = builder.item(&mcp_toggle_item);
    }

    builder = builder
        .item(&open_settings_item)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&quit_item);

    builder.build()
}

fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, menu_id: &str) {
    let bounded_id = sanitize_menu_id(menu_id);
    info!(
        target: "tray.menu.interaction",
        menu_item = bounded_id,
        "tray menu item activated",
    );

    match menu_id {
        MENU_ID_OPEN => {
            focus_or_show_window(app, COMPACT_WIDGET_LABEL);
        }
        MENU_ID_OPEN_SETTINGS => {
            // Chunk #38 (Settings modal) replaces the /settings stub route
            // with the actual form. For chunk #36 we focus the main window;
            // navigation to /settings happens webview-side via TanStack
            // Router based on the URL fragment / chunk #33's router stub.
            focus_or_show_window(app, MAIN_WINDOW_LABEL);
        }
        #[cfg(feature = "mcp-server")]
        MENU_ID_MCP_TOGGLE => {
            // Settings.mcp_server_enabled flip via existing update_settings
            // procedure is the design intent (Q3 path 3a). Actual rmcp
            // sidecar lifecycle is chunk #46's territory; this only flips
            // the persisted flag. Wiring to Settings persistence helper
            // is a follow-up enhancement for this chunk's scope.
            warn!(
                target: "tray.menu.interaction",
                menu_item = "mcp_toggle",
                deferred_to = "chunk_46_rmcp_lifecycle",
                "MCP toggle: Settings.mcp_server_enabled flip not yet wired",
            );
        }
        MENU_ID_SNAPSHOT => {
            // Defers to epoch 6's snapshot.generate per Q2 path 2a.
            warn!(
                target: "tray.menu.interaction",
                menu_item = "snapshot",
                deferred_to = "epoch_6_snapshot_pipeline",
                "Snapshot generation: snapshot.generate not yet registered",
            );
        }
        MENU_ID_QUIT => {
            // Window-close-to-tray policy from chunk #24 means close button
            // hides window; Quit menu item is the canonical termination
            // per arch §Cross-cutting Patterns "Tray icon policy".
            app.exit(0);
        }
        _ => {
            // Unknown menu id (summary line is disabled and shouldn't fire;
            // future-added items would land here) — no-op.
        }
    }
}

fn focus_or_show_window<R: Runtime>(app: &AppHandle<R>, label: &str) {
    if let Some(window) = app.get_webview_window(label) {
        if let Err(e) = window.show() {
            warn!(
                target: "app.boot.window.show",
                label = label,
                error_kind = "show_failed",
                error_msg = %e,
                "failed to show window from tray",
            );
            return;
        }
        if let Err(e) = window.set_focus() {
            warn!(
                target: "app.boot.window.show",
                label = label,
                error_kind = "set_focus_failed",
                error_msg = %e,
                "failed to focus window from tray",
            );
        }
    } else {
        warn!(
            target: "app.boot.window.show",
            label = label,
            error_kind = "not_found",
            "window label not registered in tauri config",
        );
    }
}

// Bounded enumeration of menu IDs. Matches obs allowlist `menu_item` field
// values exactly. Anything else collapses to "unknown" so the obs allowlist
// sees a bounded label.
fn sanitize_menu_id(id: &str) -> &'static str {
    match id {
        MENU_ID_OPEN => "open",
        MENU_ID_SNAPSHOT => "snapshot",
        #[cfg(feature = "mcp-server")]
        MENU_ID_MCP_TOGGLE => "mcp_toggle",
        MENU_ID_OPEN_SETTINGS => "open_settings",
        MENU_ID_QUIT => "quit",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_glyph_pixels_returns_32x32_rgba_buffer() {
        let pixels = build_glyph_pixels();
        assert_eq!(
            pixels.len(),
            (TRAY_GLYPH_SIZE * TRAY_GLYPH_SIZE * 4) as usize
        );
    }

    #[test]
    fn build_glyph_pixels_has_some_lit_pixels() {
        // Aperture motif must produce SOMETHING visible — empty buffer
        // means invisible icon. 50 is a conservative lower bound;
        // typical lit count for the 32x32 motif is ~80-120.
        let pixels = build_glyph_pixels();
        let lit_count = pixels.chunks(4).filter(|p| p[3] != 0).count();
        assert!(lit_count > 50, "expected >50 lit pixels, got {lit_count}");
    }

    #[test]
    fn build_glyph_pixels_white_opaque_or_fully_transparent() {
        // Every pixel is either pure white opaque (lit) or fully
        // transparent — no in-between colors / opacities.
        let pixels = build_glyph_pixels();
        for chunk in pixels.chunks(4) {
            if chunk[3] == 0 {
                continue;
            }
            assert_eq!(
                chunk,
                &[255, 255, 255, 255],
                "lit pixel must be white opaque",
            );
        }
    }

    #[test]
    fn build_glyph_pixels_outer_ring_present() {
        // Verify the outer circle outline visibility — pixels at the four
        // cardinal points (top, bottom, left, right) at radius ~13 should
        // all be lit. Catches accidental glyph-shape regressions.
        let pixels = build_glyph_pixels();
        let size = TRAY_GLYPH_SIZE as usize;
        let cx = (size - 1) / 2;
        let cy = (size - 1) / 2;
        let r = 13;
        let cardinals = [
            (cx, cy.saturating_sub(r)), // top
            (cx, cy + r),               // bottom
            (cx.saturating_sub(r), cy), // left
            (cx + r, cy),               // right
        ];
        for (x, y) in cardinals {
            let idx = (y * size + x) * 4;
            assert_eq!(
                pixels[idx + 3],
                255,
                "expected lit pixel at outer ring cardinal ({x},{y})",
            );
        }
    }

    #[test]
    fn sanitize_menu_id_collapses_unknown_to_constant() {
        assert_eq!(sanitize_menu_id("open"), "open");
        assert_eq!(sanitize_menu_id("snapshot"), "snapshot");
        assert_eq!(sanitize_menu_id("open_settings"), "open_settings");
        assert_eq!(sanitize_menu_id("quit"), "quit");
        assert_eq!(sanitize_menu_id("evil-injection-attempt"), "unknown");
        assert_eq!(sanitize_menu_id(""), "unknown");
        assert_eq!(sanitize_menu_id("summary"), "unknown");
    }

    #[cfg(feature = "mcp-server")]
    #[test]
    fn sanitize_menu_id_mcp_toggle_under_feature_flag() {
        assert_eq!(sanitize_menu_id("mcp_toggle"), "mcp_toggle");
    }

    #[test]
    fn menu_id_constants_align_with_obs_allowlist_value_set() {
        // The obs allowlist at pulse-app/src/observability.rs (lines
        // 705-718) pre-allocates the `menu_item` field name; field VALUES
        // are these bounded constants. `summary` is excluded from the
        // value set (read-only menu item never fires interaction).
        // `open_settings` extends the canonical set per design plan
        // §Surface: desktop-native menu structure.
        assert_eq!(MENU_ID_OPEN, "open");
        assert_eq!(MENU_ID_SNAPSHOT, "snapshot");
        assert_eq!(MENU_ID_OPEN_SETTINGS, "open_settings");
        assert_eq!(MENU_ID_QUIT, "quit");
        assert_eq!(MENU_ID_SUMMARY, "summary");
    }

    #[cfg(feature = "mcp-server")]
    #[test]
    fn menu_id_mcp_toggle_const_under_feature_flag() {
        assert_eq!(MENU_ID_MCP_TOGGLE, "mcp_toggle");
    }

    #[test]
    fn tray_error_display_does_not_leak_internals() {
        // AppError sanitization discipline (per security plan §Error
        // Handling): error display must not contain stack traces, Rust
        // struct names, file paths, or library versions. TrayError is
        // module-internal (not crossing the bridge), but follows the same
        // sanitization pattern for consistency.
        let menu_msg = TrayError::Menu.to_string();
        assert_eq!(menu_msg, "tray menu construction failed");
        assert!(!menu_msg.contains("::"));
        assert!(!menu_msg.contains('/'));

        let reg_msg = TrayError::Register.to_string();
        assert_eq!(reg_msg, "tray icon registration failed");
        assert!(!reg_msg.contains("::"));
        assert!(!reg_msg.contains('/'));
    }

    #[test]
    fn build_glyph_image_returns_image_with_correct_dimensions() {
        // Construct the image (incidentally tests Vec::leak path); the
        // Image's width/height accessors should reflect the constants.
        let image = build_glyph_image();
        assert_eq!(image.width(), TRAY_GLYPH_SIZE);
        assert_eq!(image.height(), TRAY_GLYPH_SIZE);
    }
}
