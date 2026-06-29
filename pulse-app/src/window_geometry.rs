// Remembered window position — Rust-owned persisted state, deliberately
// decoupled from the webview `Settings` contract so an `update_settings`
// from the Settings form can never clobber it (the form does not edit
// geometry). Positions are captured on `WindowEvent::Moved` (throttled) +
// flushed on close-to-tray, and restored at boot. Coordinates are never
// logged (security-plan §Logging — no raw coordinate tuples).

use std::collections::BTreeMap;
use std::path::Path;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

pub const GEOMETRY_BASENAME: &str = "window-geometry.json";
const SAVE_THROTTLE: Duration = Duration::from_millis(750);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WindowGeometry {
    positions: BTreeMap<String, Position>,
}

impl WindowGeometry {
    pub fn load_from_data_dir(data_dir: &Path) -> Self {
        match std::fs::read(data_dir.join(GEOMETRY_BASENAME)) {
            Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn position(&self, label: &str) -> Option<Position> {
        self.positions.get(label).copied()
    }

    pub fn record(&mut self, label: &str, x: i32, y: i32) {
        self.positions.insert(label.to_string(), Position { x, y });
    }

    pub fn save_to_data_dir(&self, data_dir: &Path) -> std::io::Result<()> {
        let path = data_dir.join(GEOMETRY_BASENAME);
        let tmp = data_dir.join(format!("{GEOMETRY_BASENAME}.tmp"));
        let bytes = serde_json::to_vec_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, &path)
    }
}

// Boot-to-runtime holder: the remembered geometry plus a throttle clock so
// the per-pixel `Moved` stream during a drag does not hammer the file.
pub struct GeometryStore {
    geometry: WindowGeometry,
    last_save: Instant,
}

impl GeometryStore {
    pub fn load(data_dir: &Path) -> Self {
        Self {
            geometry: WindowGeometry::load_from_data_dir(data_dir),
            last_save: Instant::now(),
        }
    }

    pub fn snapshot(&self) -> WindowGeometry {
        self.geometry.clone()
    }

    // Record a move; persist only once the throttle window has elapsed.
    // Returns whether a save was attempted (best-effort; errors are ignored).
    pub fn record_move_throttled(&mut self, label: &str, x: i32, y: i32, data_dir: &Path) -> bool {
        self.geometry.record(label, x, y);
        if self.last_save.elapsed() >= SAVE_THROTTLE {
            let _ = self.geometry.save_to_data_dir(data_dir);
            self.last_save = Instant::now();
            true
        } else {
            false
        }
    }

    // Force a final persist (close-to-tray) so the settled position survives.
    pub fn flush(&self, data_dir: &Path) {
        let _ = self.geometry.save_to_data_dir(data_dir);
    }
}
