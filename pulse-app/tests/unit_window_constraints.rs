//! Unit coverage for the compact-widget aspect-ratio clamp (intent F2 / P-062).
//! Lives here rather than in `window.rs` `mod tests` because the `pulse-app`
//! lib sets `test = false` (the Windows WebView2 DLL-load workaround), so
//! src-level `#[cfg(test)]` blocks never run under nextest.

use pulse_app::window::{WIDGET_MAX_ASPECT, WIDGET_MIN_ASPECT, clamp_to_aspect_bounds};

#[test]
fn in_band_returns_none() {
    // 480×270 = 16:9 ≈ 1.778, inside [1.4, 2.1] → no correction.
    assert_eq!(
        clamp_to_aspect_bounds((480, 270), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT),
        None
    );
}

#[test]
fn min_size_floor_is_in_band() {
    // The tauri.conf.json compact-widget min-size (400×225) must itself sit
    // inside the band, else min-size + aspect would fight at the floor.
    assert_eq!(
        clamp_to_aspect_bounds((400, 225), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT),
        None
    );
}

#[test]
fn near_band_edges_return_none() {
    // Clearly inside the band near each edge (avoids float-exact-edge fragility).
    assert_eq!(
        clamp_to_aspect_bounds((410, 200), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT), // 2.05 < 2.1
        None
    );
    assert_eq!(
        clamp_to_aspect_bounds((290, 200), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT), // 1.45 > 1.4
        None
    );
}

#[test]
fn too_wide_reduces_width_into_band() {
    // 800×270 ≈ 2.96 > 2.1 → reduce width, keep height.
    let (w, h) = clamp_to_aspect_bounds((800, 270), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT)
        .expect("out-of-band (too wide) should clamp");
    assert_eq!(h, 270);
    assert!(w < 800);
    let ratio = w as f32 / h as f32;
    assert!(
        ratio <= WIDGET_MAX_ASPECT + f32::EPSILON,
        "ratio {ratio} still too wide"
    );
    assert!(
        ratio >= WIDGET_MIN_ASPECT,
        "ratio {ratio} overshot below band"
    );
}

#[test]
fn too_tall_reduces_height_into_band() {
    // 300×400 = 0.75 < 1.4 → reduce height, keep width.
    let (w, h) = clamp_to_aspect_bounds((300, 400), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT)
        .expect("out-of-band (too tall) should clamp");
    assert_eq!(w, 300);
    assert!(h < 400);
    let ratio = w as f32 / h as f32;
    assert!(
        ratio >= WIDGET_MIN_ASPECT - f32::EPSILON,
        "ratio {ratio} still too tall"
    );
    assert!(
        ratio <= WIDGET_MAX_ASPECT,
        "ratio {ratio} overshot above band"
    );
}

#[test]
fn degenerate_zero_returns_none() {
    assert_eq!(
        clamp_to_aspect_bounds((0, 270), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT),
        None
    );
    assert_eq!(
        clamp_to_aspect_bounds((480, 0), WIDGET_MIN_ASPECT, WIDGET_MAX_ASPECT),
        None
    );
}
