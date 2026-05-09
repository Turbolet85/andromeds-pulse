use serde::{Deserialize, Serialize};

use crate::contract::AppError;

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WgpuBackend {
    Vulkan,
    Metal,
    Dx12,
}

impl WgpuBackend {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vulkan => "vulkan",
            Self::Metal => "metal",
            Self::Dx12 => "dx12",
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WebviewBackend {
    Webview2,
    Wkwebview,
    Gtkwebkit,
}

impl WebviewBackend {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Webview2 => "webview2",
            Self::Wkwebview => "wkwebview",
            Self::Gtkwebkit => "gtkwebkit",
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TimingMethod {
    Cpu,
    Gpu,
}

impl TimingMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Gpu => "gpu",
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FrameDurationInput {
    pub duration_ms: f64,
    pub wgpu_backend: WgpuBackend,
    pub webview_backend: WebviewBackend,
    pub timing_method: TimingMethod,
}

// Range bounds at the IPC trust boundary per security plan §Input Validation
// row 3: webview JS numerics are untrusted; validate before recording into the
// AllowList. Upper bound 60_000ms (60s) admits even a stalled GPU queue without
// pretending un-realistic frames are valid. NaN/Inf rejected separately so the
// reason discriminator stays informative.
const DURATION_MS_MIN: f64 = 0.0;
const DURATION_MS_MAX: f64 = 60_000.0;

pub fn validate_duration_ms(duration_ms: f64) -> Result<(), AppError> {
    if !duration_ms.is_finite() {
        return Err(AppError::Validation {
            field: "duration_ms".to_string(),
            reason: "non-finite".to_string(),
        });
    }
    if !(DURATION_MS_MIN..=DURATION_MS_MAX).contains(&duration_ms) {
        return Err(AppError::Validation {
            field: "duration_ms".to_string(),
            reason: "out of range".to_string(),
        });
    }
    Ok(())
}

#[cfg(feature = "taurpc-runtime")]
mod runtime {
    use super::*;

    #[taurpc::procedures(path = "telemetry.frontend")]
    pub trait TelemetryApi {
        async fn record_frame_ms(input: FrameDurationInput) -> Result<(), AppError>;
    }

    #[derive(Clone, Default)]
    pub struct TelemetryApiImpl;

    impl TelemetryApiImpl {
        pub fn new() -> Self {
            Self
        }
    }

    #[taurpc::resolvers]
    impl TelemetryApi for TelemetryApiImpl {
        async fn record_frame_ms(self, input: FrameDurationInput) -> Result<(), AppError> {
            validate_duration_ms(input.duration_ms)?;
            // Level-gate per obs-plan §11 Metrics row 2: avoid formatting the
            // payload when no subscriber consumes INFO. Frame events fire at
            // up to 60 Hz so the saving is non-trivial under load.
            if tracing::enabled!(tracing::Level::INFO) {
                tracing::info!(
                    target: "metric.webgpu.frame_duration_ms",
                    duration_ms = input.duration_ms,
                    wgpu_backend = input.wgpu_backend.as_str(),
                    webview_backend = input.webview_backend.as_str(),
                    timing_method = input.timing_method.as_str(),
                    "frame duration recorded",
                );
            }
            Ok(())
        }
    }
}

#[cfg(feature = "taurpc-runtime")]
pub use runtime::{TelemetryApi, TelemetryApiImpl};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tracing::span::{Attributes, Id, Record};
    use tracing::{Event, Metadata, Subscriber};

    // Mirror of the in-process CapturingSubscriber from
    // `crates/ui-bridge/src/contract.rs:359-391` per testing.md Session
    // Additions 2026-05-07. Captures (target, level, has_field) tuples so
    // each test can assert event emission without pulling in `tracing-test`.
    struct CapturingSubscriber {
        events: Arc<Mutex<Vec<(String, tracing::Level)>>>,
    }

    impl Subscriber for CapturingSubscriber {
        fn enabled(&self, _: &Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &Attributes<'_>) -> Id {
            Id::from_u64(1)
        }
        fn record(&self, _: &Id, _: &Record<'_>) {}
        fn record_follows_from(&self, _: &Id, _: &Id) {}
        fn event(&self, event: &Event<'_>) {
            let metadata = event.metadata();
            self.events
                .lock()
                .expect("event lock not poisoned")
                .push((metadata.target().to_string(), *metadata.level()));
        }
        fn enter(&self, _: &Id) {}
        fn exit(&self, _: &Id) {}
    }

    fn capture<F: FnOnce()>(f: F) -> Vec<(String, tracing::Level)> {
        let events: Arc<Mutex<Vec<(String, tracing::Level)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        tracing::subscriber::with_default(subscriber, f);
        events.lock().expect("event lock not poisoned").clone()
    }

    #[test]
    fn wgpu_backend_serializes_lowercase_per_obs_cardinality_discipline() {
        let json = serde_json::to_string(&WgpuBackend::Vulkan).expect("serializes");
        assert_eq!(json, "\"vulkan\"");
        let json = serde_json::to_string(&WgpuBackend::Metal).expect("serializes");
        assert_eq!(json, "\"metal\"");
        let json = serde_json::to_string(&WgpuBackend::Dx12).expect("serializes");
        assert_eq!(json, "\"dx12\"");
    }

    #[test]
    fn webview_backend_serializes_lowercase() {
        let json = serde_json::to_string(&WebviewBackend::Webview2).expect("serializes");
        assert_eq!(json, "\"webview2\"");
        let json = serde_json::to_string(&WebviewBackend::Wkwebview).expect("serializes");
        assert_eq!(json, "\"wkwebview\"");
        let json = serde_json::to_string(&WebviewBackend::Gtkwebkit).expect("serializes");
        assert_eq!(json, "\"gtkwebkit\"");
    }

    #[test]
    fn timing_method_serializes_lowercase() {
        let json = serde_json::to_string(&TimingMethod::Cpu).expect("serializes");
        assert_eq!(json, "\"cpu\"");
        let json = serde_json::to_string(&TimingMethod::Gpu).expect("serializes");
        assert_eq!(json, "\"gpu\"");
    }

    #[test]
    fn frame_duration_input_round_trips_through_serde() {
        let input = FrameDurationInput {
            duration_ms: 16.7,
            wgpu_backend: WgpuBackend::Metal,
            webview_backend: WebviewBackend::Wkwebview,
            timing_method: TimingMethod::Cpu,
        };
        let json = serde_json::to_string(&input).expect("serializes");
        let back: FrameDurationInput = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(back, input);
    }

    #[test]
    fn frame_duration_input_rejects_unknown_wgpu_backend() {
        let json = r#"{"duration_ms":16.7,"wgpu_backend":"opengl","webview_backend":"webview2","timing_method":"cpu"}"#;
        let result: Result<FrameDurationInput, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "smart enum must reject unknown wgpu_backend strings (no free-form String escape)",
        );
    }

    #[test]
    fn validate_duration_ms_accepts_zero() {
        assert!(validate_duration_ms(0.0).is_ok());
    }

    #[test]
    fn validate_duration_ms_accepts_typical_60fps_frame() {
        assert!(validate_duration_ms(16.7).is_ok());
    }

    #[test]
    fn validate_duration_ms_accepts_max_boundary() {
        assert!(validate_duration_ms(DURATION_MS_MAX).is_ok());
    }

    #[test]
    fn validate_duration_ms_rejects_negative() {
        let err = validate_duration_ms(-1.0).expect_err("negative must reject");
        match err {
            AppError::Validation { field, reason } => {
                assert_eq!(field, "duration_ms");
                assert_eq!(reason, "out of range");
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[test]
    fn validate_duration_ms_rejects_above_max() {
        let err = validate_duration_ms(DURATION_MS_MAX + 1.0).expect_err("above max must reject");
        assert!(matches!(
            err,
            AppError::Validation { ref field, ref reason }
                if field == "duration_ms" && reason == "out of range"
        ));
    }

    #[test]
    fn validate_duration_ms_rejects_nan() {
        let err = validate_duration_ms(f64::NAN).expect_err("NaN must reject");
        match err {
            AppError::Validation { field, reason } => {
                assert_eq!(field, "duration_ms");
                assert_eq!(reason, "non-finite");
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[test]
    fn validate_duration_ms_rejects_positive_infinity() {
        let err = validate_duration_ms(f64::INFINITY).expect_err("+inf must reject");
        assert!(matches!(
            err,
            AppError::Validation { ref reason, .. } if reason == "non-finite"
        ));
    }

    #[test]
    fn validate_duration_ms_rejects_negative_infinity() {
        let err = validate_duration_ms(f64::NEG_INFINITY).expect_err("-inf must reject");
        assert!(matches!(
            err,
            AppError::Validation { ref reason, .. } if reason == "non-finite"
        ));
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_frame_ms_emits_tracing_info_at_metric_target_on_success() {
        let events: Arc<Mutex<Vec<(String, tracing::Level)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        let guard = tracing::subscriber::set_default(subscriber);

        let api = TelemetryApiImpl::new();
        let input = FrameDurationInput {
            duration_ms: 16.7,
            wgpu_backend: WgpuBackend::Vulkan,
            webview_backend: WebviewBackend::Webview2,
            timing_method: TimingMethod::Cpu,
        };
        api.record_frame_ms(input).await.expect("ok");
        drop(guard);

        let captured = events.lock().expect("lock").clone();
        assert!(
            captured.iter().any(
                |(target, level)| target == "metric.webgpu.frame_duration_ms"
                    && *level == tracing::Level::INFO
            ),
            "expected metric.webgpu.frame_duration_ms INFO event; captured: {captured:?}"
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_frame_ms_returns_validation_error_on_negative_duration() {
        let api = TelemetryApiImpl::new();
        let input = FrameDurationInput {
            duration_ms: -5.0,
            wgpu_backend: WgpuBackend::Vulkan,
            webview_backend: WebviewBackend::Webview2,
            timing_method: TimingMethod::Cpu,
        };
        let result = api.record_frame_ms(input).await;
        match result {
            Err(AppError::Validation { field, reason }) => {
                assert_eq!(field, "duration_ms");
                assert_eq!(reason, "out of range");
            }
            other => panic!("expected Validation error, got {other:?}"),
        }
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_frame_ms_returns_validation_error_on_nan() {
        let api = TelemetryApiImpl::new();
        let input = FrameDurationInput {
            duration_ms: f64::NAN,
            wgpu_backend: WgpuBackend::Metal,
            webview_backend: WebviewBackend::Wkwebview,
            timing_method: TimingMethod::Gpu,
        };
        let result = api.record_frame_ms(input).await;
        match result {
            Err(AppError::Validation { field, reason }) => {
                assert_eq!(field, "duration_ms");
                assert_eq!(reason, "non-finite");
            }
            other => panic!("expected Validation error, got {other:?}"),
        }
    }

    #[test]
    fn capture_helper_records_synchronous_emission() {
        // Sanity check that the capture helper itself works for sync events.
        let events = capture(|| {
            tracing::info!(target: "metric.webgpu.frame_duration_ms", "sync emit");
        });
        assert!(
            events
                .iter()
                .any(|(t, l)| t == "metric.webgpu.frame_duration_ms" && *l == tracing::Level::INFO),
            "capture() helper must record sync tracing emissions; got: {events:?}"
        );
    }
}
