//! Linux launch render posture.
//!
//! On an NVIDIA GPU under a native Wayland compositor, NVIDIA's EGL
//! explicit-sync path and WebKitGTK 2.52 disagree: the compositor closes the
//! connection with a protocol error and GTK's Wayland backend calls
//! `_exit(1)` about two seconds into boot. `__NV_DISABLE_EXPLICIT_SYNC=1`
//! removes that path. Only NVIDIA's driver reads the variable, so setting it
//! on every Linux host scopes the default to NVIDIA by construction.
//!
//! A value already present in the launch environment (any value, empty
//! included) is honoured and never parsed: process env outranks built-in
//! defaults (architecture, Cross-cutting Patterns -> Config management).

use tracing::{info, warn};

pub const LEVER_ENV: &str = "__NV_DISABLE_EXPLICIT_SYNC";
pub const LEVER_VALUE: &str = "1";

const TARGET: &str = "app.boot.render.posture";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderPosture {
    Applied,
    PresetHonoured,
    NotApplicable,
}

impl RenderPosture {
    pub fn label(self) -> &'static str {
        match self {
            RenderPosture::Applied => "applied",
            RenderPosture::PresetHonoured => "preset_honoured",
            RenderPosture::NotApplicable => "not_applicable",
        }
    }
}

pub fn decide(is_linux: bool, preset_present: bool) -> RenderPosture {
    match (is_linux, preset_present) {
        (false, _) => RenderPosture::NotApplicable,
        (true, true) => RenderPosture::PresetHonoured,
        (true, false) => RenderPosture::Applied,
    }
}

/// Applies the Linux default when the lever is unset.
///
/// Must be called ONLY as the first statement of `main()`, before the tokio
/// runtime is built; no other call site is sound.
pub fn apply_linux_default() -> RenderPosture {
    let posture = decide(
        cfg!(target_os = "linux"),
        std::env::var_os(LEVER_ENV).is_some(),
    );
    if posture == RenderPosture::Applied {
        // SAFETY: called as the first statement of `main()`, before
        // `tokio::runtime::Builder::new_multi_thread().build()` spawns any
        // thread, so no other thread reads or writes the environment.
        unsafe { std::env::set_var(LEVER_ENV, LEVER_VALUE) };
    }
    posture
}

/// Emits the boot record for a posture decided before the sink existed.
pub fn emit_posture(posture: RenderPosture) {
    match posture {
        RenderPosture::PresetHonoured => warn!(
            target: TARGET,
            posture = posture.label(),
            lever = LEVER_ENV,
            "launch render posture overridden by a preset value",
        ),
        RenderPosture::Applied | RenderPosture::NotApplicable => info!(
            target: TARGET,
            posture = posture.label(),
            lever = LEVER_ENV,
            "launch render posture",
        ),
    }
}
