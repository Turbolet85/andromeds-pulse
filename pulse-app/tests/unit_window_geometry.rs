// Remembered-window-geometry persistence (chunk 2026-06-29-window-geometry-
// movable-shell, P-061). The lib's own `mod tests` do not run under
// `[lib] test = false`, so these run as an integration test crate.

use pulse_app::window_geometry::{GeometryStore, Position, WindowGeometry};
use tempfile::TempDir;

#[test]
fn record_then_position_roundtrips_in_memory() {
    let mut geom = WindowGeometry::default();
    assert_eq!(geom.position("main"), None);
    geom.record("main", 100, 200);
    geom.record("compact-widget", -5, 42);
    assert_eq!(geom.position("main"), Some(Position { x: 100, y: 200 }));
    assert_eq!(
        geom.position("compact-widget"),
        Some(Position { x: -5, y: 42 })
    );
    assert_eq!(geom.position("unknown"), None);
}

#[test]
fn save_then_load_roundtrips_through_disk() {
    let dir = TempDir::new().expect("tempdir");
    let mut geom = WindowGeometry::default();
    geom.record("main", 640, 360);
    geom.save_to_data_dir(dir.path()).expect("save");
    let loaded = WindowGeometry::load_from_data_dir(dir.path());
    assert_eq!(loaded.position("main"), Some(Position { x: 640, y: 360 }));
}

#[test]
fn load_missing_file_returns_default() {
    let dir = TempDir::new().expect("tempdir");
    let loaded = WindowGeometry::load_from_data_dir(dir.path());
    assert_eq!(loaded, WindowGeometry::default());
    assert_eq!(loaded.position("main"), None);
}

#[test]
fn load_corrupt_file_returns_default_without_panic() {
    let dir = TempDir::new().expect("tempdir");
    std::fs::write(
        dir.path().join("window-geometry.json"),
        b"{ not valid json ]",
    )
    .expect("write");
    let loaded = WindowGeometry::load_from_data_dir(dir.path());
    assert_eq!(loaded, WindowGeometry::default());
}

#[test]
fn geometry_store_records_in_memory_and_flush_persists() {
    let dir = TempDir::new().expect("tempdir");
    let mut store = GeometryStore::load(dir.path());
    // The first move may be inside the throttle window; the return value is a
    // timing detail (not asserted). What matters: the move is held in memory
    // and a flush persists the settled position for next boot.
    let _ = store.record_move_throttled("compact-widget", 11, 22, dir.path());
    assert_eq!(
        store.snapshot().position("compact-widget"),
        Some(Position { x: 11, y: 22 })
    );
    store.flush(dir.path());
    let reloaded = WindowGeometry::load_from_data_dir(dir.path());
    assert_eq!(
        reloaded.position("compact-widget"),
        Some(Position { x: 11, y: 22 })
    );
}
