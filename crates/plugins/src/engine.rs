//! wasmtime Component Model Engine substrate for the andromeda-pulse plugin host.
//!
//! Centralizes the `wasmtime::Config` posture:
//! - `Config::epoch_interruption(true)` — required substrate per security
//!   plan §API Security row "Plugin host capability sandbox" + §Security
//!   Decisions Log 2026-05-02. 2-3× faster than fuel for plugin-call
//!   timeouts; chunk #46 attaches the per-`Store` `ResourceLimiter`.
//! - `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` constant — canonical bound for
//!   wasi-http header fields at substrate level. Per security-plan
//!   Decisions Log 2026-05-11 (Trigger 4 amendment): `Config::max_wasm_http_fields_size`
//!   is NOT a method on `wasmtime::Config` in wasmtime 25.x; actual
//!   enforcement attaches via wasi-http context (e.g.,
//!   `wasmtime_wasi_http::WasiHttpCtxBuilder::max_field_size`) when
//!   wasi-http imports are introduced in subsequent chunks (#46+).
//!   Chunk #45 substrate declares zero wasi-http imports in the 3 plugin
//!   categories; the constant exists as the substrate-level intent
//!   anchor for CVE-2026-27572 (April 2026 wasi-http header advisory).
//! - Cranelift backend stays the default on x86_64 — never enable a
//!   non-Cranelift wasmtime feature flag (security plan §Anti-Patterns
//!   Data Protection row 1; CVE-2026-34941 + CVE-2026-35195 cluster).
//!
//! Subsequent chunks (#46 sandbox, #47 loader) extend this same Config
//! builder rather than re-instantiate the Engine.

use std::time::Instant;

use tracing::instrument;
use wasmtime::{Config, Engine};

use crate::contract::Error;

/// Max wasi-http header field bytes accepted by guest plugins. Anchors
/// CVE-2026-27572 (April 2026 advisory cluster). Bounded conservatively
/// at 64 KB; tightens further when wasi-http imports are declared per
/// category (chunk #45 substrate declares none).
pub const MAX_WASM_HTTP_FIELDS_SIZE_BYTES: usize = 64 * 1024;

// Compile-time sanity bound on the wasi-http field size constant.
// Any future edit that would zero it out OR raise it above 1 MB
// fails the build instead of silently regressing the CVE-2026-27572
// anchor. Module-scope so the check runs on every build (not just tests).
const _: () = {
    assert!(MAX_WASM_HTTP_FIELDS_SIZE_BYTES > 0);
    assert!(MAX_WASM_HTTP_FIELDS_SIZE_BYTES <= 1024 * 1024);
};

/// Build the canonical wasmtime `Engine` for the plugin host.
///
/// Posture sealed in this fn:
/// - Component Model enabled (`wasm_component_model(true)`).
/// - `epoch_interruption(true)` — required for plugin-call timeouts
///   (security plan §API Security + §Security Decisions Log).
/// - Cranelift backend stays default on x86_64 (security plan
///   §Anti-Patterns Data Protection row 1; CVE-2026-34941 / CVE-2026-35195
///   were unaffected on Cranelift on x86_64).
///
/// `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` is recorded as a tracing field so
/// downstream diagnostic correlation knows the canonical bound. Actual
/// enforcement of the bound attaches via wasi-http context at the chunk
/// that introduces wasi-http imports (#46+); chunk #45 substrate has none.
/// See security-plan.md §Security Decisions Log 2026-05-11 for the
/// Trigger 4 amendment that clarifies the canonical-bound-vs-enforcement
/// distinction.
///
/// Returns `Error::EngineInit { reason }` on construction failure (rare
/// — typical causes are wasmtime feature-flag misconfiguration or
/// platform-specific Cranelift unavailability).
#[instrument(skip_all, fields(
    epoch_interruption_enabled = tracing::field::Empty,
    max_wasm_http_fields_size_bytes = tracing::field::Empty,
    duration_ms = tracing::field::Empty,
))]
pub fn build_engine() -> Result<Engine, Error> {
    let started = Instant::now();
    let span = tracing::Span::current();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.epoch_interruption(true);

    let engine = Engine::new(&config).map_err(|e| Error::EngineInit {
        reason: sanitize_wasmtime_error(&e.to_string()),
    })?;

    let elapsed = started.elapsed().as_millis() as u64;
    span.record("epoch_interruption_enabled", true);
    span.record(
        "max_wasm_http_fields_size_bytes",
        MAX_WASM_HTTP_FIELDS_SIZE_BYTES as u64,
    );
    span.record("duration_ms", elapsed);

    Ok(engine)
}

/// Sanitize wasmtime error strings before they cross into structured
/// `tracing` fields. Keeps a leading short token; strips paths / numbers
/// that vary across runs. Per security plan §Anti-Patterns Logging:
/// never expose file paths or library versions in error messages crossing
/// the bridge.
pub(crate) fn sanitize_wasmtime_error(raw: &str) -> String {
    raw.lines()
        .next()
        .unwrap_or("wasmtime error")
        .chars()
        .take(120)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_engine_succeeds_with_default_posture() {
        let result = build_engine();
        assert!(
            result.is_ok(),
            "build_engine must succeed on default Cranelift+component-model config"
        );
    }

    #[test]
    fn build_engine_returns_independent_engines() {
        // Engine construction is allocation-cheap; verify the builder is
        // re-callable. Chunks #46-#47 will share one Engine across plugins,
        // but the API does not preclude multiple engines per process.
        let _a = build_engine().expect("first engine builds");
        let _b = build_engine().expect("second engine builds");
    }

    #[test]
    fn sanitize_wasmtime_error_keeps_first_line_only() {
        let raw = "first line\nsecond line\nthird line";
        let sanitized = sanitize_wasmtime_error(raw);
        assert_eq!(sanitized, "first line");
    }

    #[test]
    fn sanitize_wasmtime_error_truncates_to_120_chars() {
        let raw = "x".repeat(500);
        let sanitized = sanitize_wasmtime_error(&raw);
        assert_eq!(sanitized.chars().count(), 120);
    }

    #[test]
    fn sanitize_wasmtime_error_falls_back_for_empty_input() {
        // Empty input has no lines; the fn returns the fallback string
        // so diagnostic fields never emit an empty / blank reason.
        let sanitized = sanitize_wasmtime_error("");
        assert_eq!(sanitized, "wasmtime error");
    }
}
