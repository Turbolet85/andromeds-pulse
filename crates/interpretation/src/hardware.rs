//! Concrete `HardwareProfileDetector` implementing the chunk #80
//! `HardwareProfileSource` trait (declared in `triage::contract` +
//! re-exported via `interpretation::contract`).
//!
//! Detection signals (chunk #82 cold-start; refinements deferred):
//! - GPU presence: platform-specific (Metal on macOS, CUDA on Linux/Windows
//!   via library probe).
//! - CPU core count: `std::thread::available_parallelism()`.
//! - Memory size: deferred to chunk #83+ (requires sysinfo dep or
//!   platform-specific syscalls); current detector uses conservative
//!   defaults + env var override.
//!
//! Override via `ANDROMEDA_PULSE_HARDWARE_PROFILE` env var (accepts
//! `gpu-primary` / `gpu-fallback` / `cpu-primary` / `cpu-fallback`).
//! Useful for tests + degraded-environment validation.

use std::num::NonZeroUsize;

use tracing::{info, warn};
use triage::contract::{HardwareProfile, HardwareProfileSource};

/// Env var override for hardware profile classification. Accepts the
/// snake_case label of any `HardwareProfile` variant (e.g., "cpu_primary"
/// or "cpu-primary" — both kebab and snake case accepted).
pub const ENV_HARDWARE_PROFILE_OVERRIDE: &str = "ANDROMEDA_PULSE_HARDWARE_PROFILE";

/// Default core-count threshold separating cpu-primary from cpu-fallback.
/// Per pulse-distillation-architecture.md L4 §Hardware Profile Matrix:
/// systems with ≥4 cores can run primary-tier CPU inference within latency
/// budget; fewer cores route to fallback-tier (smaller models, less
/// constrained output).
pub const CPU_PRIMARY_CORE_THRESHOLD: usize = 4;

/// Concrete `HardwareProfileSource` impl. Caches detection result on first
/// call — idempotent + cheap (boot-time classification, subsequent reads
/// return cached value).
#[derive(Debug, Clone)]
pub struct HardwareProfileDetector {
    profile: HardwareProfile,
}

impl HardwareProfileDetector {
    /// Constructs the detector by probing host hardware. Reads env var
    /// override first (so tests + degraded-environment validation can
    /// force a specific profile); falls back to real detection.
    pub fn new() -> Self {
        let profile = match std::env::var(ENV_HARDWARE_PROFILE_OVERRIDE) {
            Ok(raw) => match parse_profile_override(&raw) {
                Some(p) => {
                    info!(
                        target: "interpretation.hardware.detect",
                        profile = profile_label(p),
                        override_source = "env",
                        "hardware profile overridden via env var"
                    );
                    p
                }
                None => {
                    warn!(
                        target: "interpretation.hardware.detect",
                        override_source = "env",
                        "invalid ANDROMEDA_PULSE_HARDWARE_PROFILE value; falling back to real detection"
                    );
                    detect_real()
                }
            },
            Err(_) => detect_real(),
        };

        Self { profile }
    }

    /// Constructs a detector with the supplied profile, bypassing detection
    /// entirely. Test-only constructor (mirrors `FixedProfile` test fixture
    /// at `crates/triage/src/cadence/coordinator.rs`).
    #[cfg(any(test, feature = "test-utils"))]
    pub fn with_profile(profile: HardwareProfile) -> Self {
        Self { profile }
    }
}

impl Default for HardwareProfileDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareProfileSource for HardwareProfileDetector {
    fn current_profile(&self) -> HardwareProfile {
        self.profile
    }
}

/// Real hardware detection probing GPU presence + CPU core count.
///
/// Classification rules (chunk #82 cold-start; refinements deferred):
/// - `GpuPrimary`: GPU available (Metal on macOS OR CUDA on Linux/Windows)
/// - `CpuPrimary`: no GPU + cores ≥ [`CPU_PRIMARY_CORE_THRESHOLD`]
/// - `CpuFallback`: no GPU + cores < [`CPU_PRIMARY_CORE_THRESHOLD`]
/// - `GpuFallback`: NOT detected by real probe in this chunk (VRAM
///   detection deferred to chunk #83+); only reachable via env var override
fn detect_real() -> HardwareProfile {
    let gpu_available = detect_gpu_present();
    let core_count = available_cpu_cores();

    let profile = if gpu_available {
        HardwareProfile::GpuPrimary
    } else if core_count >= CPU_PRIMARY_CORE_THRESHOLD {
        HardwareProfile::CpuPrimary
    } else {
        HardwareProfile::CpuFallback
    };

    info!(
        target: "interpretation.hardware.detect",
        profile = profile_label(profile),
        gpu_available,
        cpu_core_count = core_count,
        profile_detection_decision_recorded = matches!(profile, HardwareProfile::CpuPrimary),
        "hardware profile detected"
    );

    profile
}

/// Cross-platform GPU presence probe. Returns `true` if a GPU-accelerated
/// inference backend is likely available; `false` otherwise.
///
/// Per security extract anti-pattern "NEVER hardcode model paths or URLs":
/// no model files OR external URLs probed here; only system-level signals.
fn detect_gpu_present() -> bool {
    #[cfg(target_os = "macos")]
    {
        // macOS: assume Metal available (every Mac since 2012 supports Metal;
        // Apple Silicon Macs have unified memory architecture suitable for
        // LLM inference). Refinement (Metal feature-set verification)
        // deferred to chunk #83+.
        true
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: check for libcuda.so existence in standard library paths.
        // Common locations: /usr/lib/x86_64-linux-gnu/libcuda.so,
        // /usr/local/cuda/lib64/libcuda.so. Heuristic — false negatives OK
        // (will reclassify to cpu-primary which is safe default).
        [
            "/usr/lib/x86_64-linux-gnu/libcuda.so",
            "/usr/local/cuda/lib64/libcuda.so",
        ]
        .iter()
        .any(|p| std::path::Path::new(p).exists())
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: check for nvcuda.dll in System32 (CUDA Runtime DLL).
        // Heuristic — false negatives OK; reclassifies to cpu-primary which
        // is safe default.
        std::path::Path::new(r"C:\Windows\System32\nvcuda.dll").exists()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        false
    }
}

/// Returns available CPU core count via `std::thread::available_parallelism()`.
/// Returns 1 (single-core safe-fallback) if the underlying call fails (e.g.,
/// unusual containerized environment).
fn available_cpu_cores() -> usize {
    std::thread::available_parallelism()
        .map(NonZeroUsize::get)
        .unwrap_or(1)
}

/// Parses a user-supplied profile string into a bounded `HardwareProfile`.
/// Accepts both snake_case + kebab-case variants for the four known
/// profiles. Returns `None` on unrecognized input.
fn parse_profile_override(raw: &str) -> Option<HardwareProfile> {
    match raw.trim().to_lowercase().as_str() {
        "gpu_primary" | "gpu-primary" => Some(HardwareProfile::GpuPrimary),
        "gpu_fallback" | "gpu-fallback" => Some(HardwareProfile::GpuFallback),
        "cpu_primary" | "cpu-primary" => Some(HardwareProfile::CpuPrimary),
        "cpu_fallback" | "cpu-fallback" => Some(HardwareProfile::CpuFallback),
        _ => None,
    }
}

/// Static bounded label per obs §5 cardinality discipline. Used as tracing
/// field value + cross-broadcast comparison.
pub fn profile_label(profile: HardwareProfile) -> &'static str {
    match profile {
        HardwareProfile::Unknown => "unknown",
        HardwareProfile::GpuPrimary => "gpu_primary",
        HardwareProfile::GpuFallback => "gpu_fallback",
        HardwareProfile::CpuPrimary => "cpu_primary",
        HardwareProfile::CpuFallback => "cpu_fallback",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_profile_override_accepts_snake_case() {
        assert_eq!(
            parse_profile_override("cpu_primary"),
            Some(HardwareProfile::CpuPrimary)
        );
        assert_eq!(
            parse_profile_override("gpu_fallback"),
            Some(HardwareProfile::GpuFallback)
        );
    }

    #[test]
    fn parse_profile_override_accepts_kebab_case() {
        assert_eq!(
            parse_profile_override("cpu-primary"),
            Some(HardwareProfile::CpuPrimary)
        );
        assert_eq!(
            parse_profile_override("gpu-primary"),
            Some(HardwareProfile::GpuPrimary)
        );
    }

    #[test]
    fn parse_profile_override_trims_whitespace_and_case() {
        assert_eq!(
            parse_profile_override("  CPU-PRIMARY  "),
            Some(HardwareProfile::CpuPrimary)
        );
    }

    #[test]
    fn parse_profile_override_rejects_unknown() {
        assert_eq!(parse_profile_override("unknown"), None);
        assert_eq!(parse_profile_override("gpu"), None);
        assert_eq!(parse_profile_override(""), None);
    }

    #[test]
    fn profile_label_is_bounded_for_all_variants() {
        assert_eq!(profile_label(HardwareProfile::Unknown), "unknown");
        assert_eq!(profile_label(HardwareProfile::GpuPrimary), "gpu_primary");
        assert_eq!(profile_label(HardwareProfile::GpuFallback), "gpu_fallback");
        assert_eq!(profile_label(HardwareProfile::CpuPrimary), "cpu_primary");
        assert_eq!(profile_label(HardwareProfile::CpuFallback), "cpu_fallback");
    }

    #[test]
    fn detector_with_profile_returns_supplied_profile() {
        let d = HardwareProfileDetector::with_profile(HardwareProfile::GpuPrimary);
        assert_eq!(d.current_profile(), HardwareProfile::GpuPrimary);
    }

    #[test]
    fn detector_with_profile_returns_cpu_primary_classification() {
        let d = HardwareProfileDetector::with_profile(HardwareProfile::CpuPrimary);
        assert_eq!(d.current_profile(), HardwareProfile::CpuPrimary);
    }

    #[test]
    fn detector_with_profile_returns_cpu_fallback_classification() {
        let d = HardwareProfileDetector::with_profile(HardwareProfile::CpuFallback);
        assert_eq!(d.current_profile(), HardwareProfile::CpuFallback);
    }

    #[test]
    fn detector_with_profile_returns_gpu_fallback_classification() {
        let d = HardwareProfileDetector::with_profile(HardwareProfile::GpuFallback);
        assert_eq!(d.current_profile(), HardwareProfile::GpuFallback);
    }

    #[test]
    fn available_cpu_cores_returns_nonzero() {
        assert!(available_cpu_cores() >= 1);
    }
}
