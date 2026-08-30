// Migrated 2026-08-30 from `pulse-app/src/tray.rs::tests` — that crate sets
// `[lib] test = false` (the WebView2 workaround), so a src-level `mod tests`
// compiles, passes clippy, and NEVER RUNS. Internals reach here via
// `pub` + `#[doc(hidden)]` per test-plan §2/§4.

use pulse_app::tray::{
    MENU_ID_OPEN, MENU_ID_OPEN_SETTINGS, MENU_ID_QUIT, MENU_ID_SNAPSHOT, MENU_ID_SUMMARY,
    TRAY_GLYPH_SIZE, TrayError, build_glyph_image, build_glyph_pixels, sanitize_menu_id,
};

#[cfg(feature = "mcp-server")]
use pulse_app::tray::MENU_ID_MCP_TOGGLE;

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
    // The obs allowlist at pulse-app/src/observability.rs pre-allocates the
    // `menu_item` field name; field VALUES are these bounded constants.
    // `summary` is excluded from the value set (read-only menu item never
    // fires interaction). `open_settings` extends the canonical set per
    // design plan §Surface: desktop-native menu structure.
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
