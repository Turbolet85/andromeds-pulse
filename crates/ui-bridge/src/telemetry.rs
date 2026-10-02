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

// Cumulative-incident-severity tier driving the rendered dot hue. Mirrors the
// `triage::contract::PriorityTier` label set plus an explicit `None` for "no
// active incidents", rather than depending on the triage crate — the same
// local-bounded-enum shape the backend enums above use, keeping ui-bridge free
// of a sibling dep per architecture.md §Cross-cutting Module dependency
// direction.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HueSeverityTier {
    None,
    Curious,
    Suggested,
    Autonomous,
}

impl HueSeverityTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Curious => "curious",
            Self::Suggested => "suggested",
            Self::Autonomous => "autonomous",
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ConstellationHueLatencyInput {
    pub duration_ms: f64,
    pub severity_tier: HueSeverityTier,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ConstellationDiscoveryInput {
    pub duration_ms: f64,
    pub discovered_count: u32,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FindingsCounterRefreshInput {
    pub duration_ms: f64,
}

// Upper bound on services reported newly-discovered in one constellation
// render. The lifecycle registry is itself capped, so a count beyond this is a
// malformed webview payload rather than a large deployment.
const DISCOVERED_COUNT_MAX: u32 = 10_000;

pub fn validate_discovered_count(discovered_count: u32) -> Result<(), AppError> {
    if discovered_count > DISCOVERED_COUNT_MAX {
        return Err(AppError::Validation {
            field: "discovered_count".to_string(),
            reason: "out of range".to_string(),
        });
    }
    Ok(())
}

// Capability-rejected webview IPC record (security-plan §Logging & Monitoring
// "What to log"). The category is classified webview-side from the invoke
// rejection value — Tauri 2.11 rejects a denied command back to the webview
// with a message containing "not allowed" (release: `Command {cmd} not
// allowed by ACL`, src/webview/mod.rs; debug: resolve_access_message,
// src/ipc/authority.rs) — and a non-matching rejection degrades to `Other`
// rather than misclassifying, so a Tauri rewording can never silently drop
// the record class.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpcRejectionCategory {
    AclRejected,
    Other,
}

impl IpcRejectionCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AclRejected => "acl_rejected",
            Self::Other => "other",
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IpcRejectionInput {
    pub error_category: IpcRejectionCategory,
    pub window_label: String,
    pub payload_bytes: u32,
}

// Upper bound on the reported payload byte count. A rejected clipboard write
// carries at most a few hundred KB of markdown; anything past this is a
// malformed webview payload, not a large report.
const PAYLOAD_BYTES_MAX: u32 = 100_000_000;

pub fn validate_payload_bytes(payload_bytes: u32) -> Result<(), AppError> {
    if payload_bytes > PAYLOAD_BYTES_MAX {
        return Err(AppError::Validation {
            field: "payload_bytes".to_string(),
            reason: "out of range".to_string(),
        });
    }
    Ok(())
}

// Third copy of the bounded window-label set, mirroring
// `pulse-app/src/window.rs::sanitize_window_label` (unreachable from this
// crate — the dependency DAG roots at pulse-app) and the webview's
// `use-window-label.ts::sanitizeWindowLabel`. Coerces rather than rejects:
// a mislabeled window must still land its rejection record, as `unknown`.
pub fn coerce_window_label(label: &str) -> &'static str {
    match label {
        "main" => "main",
        "compact-widget" => "compact-widget",
        "findings" => "findings",
        "report" => "report",
        _ => "unknown",
    }
}

// The outcome of one webview WebGPU adapter request, classified webview-side
// (`canvas/adapter-state.ts`). A frame-less log reads its cause from these
// records (`xtask::perf_budget::frame_cause`), so the set stays closed: no raw
// error text crosses.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WebgpuAdapterOutcome {
    Obtained,
    NoNavigatorGpu,
    AdapterNull,
    AdapterRequestRejected,
    DeviceRequestFailed,
}

impl WebgpuAdapterOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Obtained => "obtained",
            Self::NoNavigatorGpu => "no_navigator_gpu",
            Self::AdapterNull => "adapter_null",
            Self::AdapterRequestRejected => "adapter_request_rejected",
            Self::DeviceRequestFailed => "device_request_failed",
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebgpuAdapterInput {
    pub outcome: WebgpuAdapterOutcome,
    pub window_label: String,
}

#[cfg(feature = "taurpc-runtime")]
mod runtime {
    use super::*;

    // record_constellation_hue_latency / record_constellation_discovery_latency /
    // record_findings_counter_refresh carry the three delegated timing bounds
    // (P-025 / P-027 / P-045) from their rendered surfaces to the self-observation
    // log, so an external harness can grade bounds the backend cannot see.
    // record_webgpu_adapter carries each webview adapter request's closed outcome
    // plus its coerced window label, once per request (never per frame), so a
    // frame-less run can name why it drew nothing.
    // Doc prose lives here rather than as `///` inside the trait: the
    // taurpc::procedures macro rejects multi-line doc attributes on its methods.
    #[taurpc::procedures(path = "telemetry.frontend")]
    pub trait TelemetryApi {
        async fn record_frame_ms(input: FrameDurationInput) -> Result<(), AppError>;
        async fn record_constellation_hue_latency(
            input: ConstellationHueLatencyInput,
        ) -> Result<(), AppError>;
        async fn record_constellation_discovery_latency(
            input: ConstellationDiscoveryInput,
        ) -> Result<(), AppError>;
        async fn record_findings_counter_refresh(
            input: FindingsCounterRefreshInput,
        ) -> Result<(), AppError>;
        async fn record_ipc_rejection(input: IpcRejectionInput) -> Result<(), AppError>;
        async fn record_webgpu_adapter(input: WebgpuAdapterInput) -> Result<(), AppError>;
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

        async fn record_constellation_hue_latency(
            self,
            input: ConstellationHueLatencyInput,
        ) -> Result<(), AppError> {
            validate_duration_ms(input.duration_ms)?;
            if tracing::enabled!(tracing::Level::INFO) {
                tracing::info!(
                    target: "metric.constellation.hue_update_ms",
                    duration_ms = input.duration_ms,
                    severity_tier = input.severity_tier.as_str(),
                    "constellation hue update latency recorded",
                );
            }
            Ok(())
        }

        async fn record_constellation_discovery_latency(
            self,
            input: ConstellationDiscoveryInput,
        ) -> Result<(), AppError> {
            validate_duration_ms(input.duration_ms)?;
            validate_discovered_count(input.discovered_count)?;
            if tracing::enabled!(tracing::Level::INFO) {
                tracing::info!(
                    target: "metric.constellation.discovery_ms",
                    duration_ms = input.duration_ms,
                    discovered_count = input.discovered_count,
                    "constellation discovery latency recorded",
                );
            }
            Ok(())
        }

        async fn record_findings_counter_refresh(
            self,
            input: FindingsCounterRefreshInput,
        ) -> Result<(), AppError> {
            validate_duration_ms(input.duration_ms)?;
            if tracing::enabled!(tracing::Level::INFO) {
                tracing::info!(
                    target: "metric.findings.counter_refresh_ms",
                    duration_ms = input.duration_ms,
                    "findings counter refresh latency recorded",
                );
            }
            Ok(())
        }

        async fn record_ipc_rejection(self, input: IpcRejectionInput) -> Result<(), AppError> {
            validate_payload_bytes(input.payload_bytes)?;
            let window_label = coerce_window_label(&input.window_label);
            // WARN, once per rejection event — rejections are rare by
            // construction and never on a hot path (obs-plan §6), so no
            // level-gating like the INFO metric emits above.
            tracing::warn!(
                target: "ui.ipc.rejection",
                error_category = input.error_category.as_str(),
                window_label,
                payload_bytes = input.payload_bytes,
                "capability-rejected webview IPC recorded",
            );
            Ok(())
        }

        async fn record_webgpu_adapter(self, input: WebgpuAdapterInput) -> Result<(), AppError> {
            let window_label = coerce_window_label(&input.window_label);
            let outcome = input.outcome.as_str();
            // Once per canvas mount, off any hot path (obs-plan §6 warn row): an
            // obtained adapter is INFO, every other outcome a WARN.
            if input.outcome == WebgpuAdapterOutcome::Obtained {
                tracing::info!(
                    target: "ui.webgpu.adapter",
                    outcome,
                    window_label,
                    "webgpu adapter request recorded",
                );
            } else {
                tracing::warn!(
                    target: "ui.webgpu.adapter",
                    outcome,
                    window_label,
                    "webgpu adapter request recorded",
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

    // Field-materializing sink for the delegated-timing pins: the tuple-only
    // CapturingSubscriber above cannot assert a label's VALUE, and a
    // target-only pin cannot tell a correct severity label from a wrong one.
    struct FieldCollector {
        sink: String,
    }

    impl tracing::field::Visit for FieldCollector {
        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.sink
                .push_str(&format!("{}={:?};", field.name(), value));
        }
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.sink.push_str(&format!("{}={};", field.name(), value));
        }
        fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
            self.sink.push_str(&format!("{}={};", field.name(), value));
        }
        fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
            self.sink.push_str(&format!("{}={};", field.name(), value));
        }
    }

    struct FieldCapturingSubscriber {
        events: Arc<Mutex<Vec<(String, String)>>>,
    }

    impl Subscriber for FieldCapturingSubscriber {
        fn enabled(&self, _: &Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &Attributes<'_>) -> Id {
            Id::from_u64(1)
        }
        fn record(&self, _: &Id, _: &Record<'_>) {}
        fn record_follows_from(&self, _: &Id, _: &Id) {}
        fn event(&self, event: &Event<'_>) {
            let mut collector = FieldCollector {
                sink: String::new(),
            };
            event.record(&mut collector);
            self.events
                .lock()
                .expect("event lock not poisoned")
                .push((event.metadata().target().to_string(), collector.sink));
        }
        fn enter(&self, _: &Id) {}
        fn exit(&self, _: &Id) {}
    }

    // Async-safe capture: `set_default` scopes the subscriber to this thread
    // across the `.await`, per the with_default-takes-a-sync-closure constraint
    // (a `block_on` inside a #[tokio::test] would panic instead).
    #[cfg(feature = "taurpc-runtime")]
    type FieldEvents = Arc<Mutex<Vec<(String, String)>>>;

    #[cfg(feature = "taurpc-runtime")]
    fn field_sink() -> (FieldEvents, FieldCapturingSubscriber) {
        let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = FieldCapturingSubscriber {
            events: events.clone(),
        };
        (events, subscriber)
    }

    #[test]
    fn hue_severity_tier_serializes_lowercase_bounded_labels() {
        for (tier, expected) in [
            (HueSeverityTier::None, "\"none\""),
            (HueSeverityTier::Curious, "\"curious\""),
            (HueSeverityTier::Suggested, "\"suggested\""),
            (HueSeverityTier::Autonomous, "\"autonomous\""),
        ] {
            let json = serde_json::to_string(&tier).expect("serializes");
            assert_eq!(json, expected);
        }
    }

    #[test]
    fn constellation_hue_latency_input_rejects_unknown_severity_tier() {
        let json = r#"{"duration_ms":12.0,"severity_tier":"catastrophic"}"#;
        let result: Result<ConstellationHueLatencyInput, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "closed enum must reject an unknown severity_tier (no free-form String escape)",
        );
    }

    #[test]
    fn validate_discovered_count_accepts_zero_and_max_boundary() {
        assert!(validate_discovered_count(0).is_ok());
        assert!(validate_discovered_count(DISCOVERED_COUNT_MAX).is_ok());
    }

    #[test]
    fn validate_discovered_count_rejects_above_max() {
        let err =
            validate_discovered_count(DISCOVERED_COUNT_MAX + 1).expect_err("above max must reject");
        match err {
            AppError::Validation { field, reason } => {
                assert_eq!(field, "discovered_count");
                assert_eq!(reason, "out of range");
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_constellation_hue_latency_emits_exact_target_with_severity_label() {
        let api = TelemetryApiImpl::new();
        let (events, subscriber) = field_sink();
        let guard = tracing::subscriber::set_default(subscriber);
        api.record_constellation_hue_latency(ConstellationHueLatencyInput {
            duration_ms: 42.5,
            severity_tier: HueSeverityTier::Suggested,
        })
        .await
        .expect("ok");
        drop(guard);
        let captured = events.lock().expect("lock").clone();

        let hit = captured
            .iter()
            .find(|(target, _)| target == "metric.constellation.hue_update_ms")
            .unwrap_or_else(|| {
                panic!("expected metric.constellation.hue_update_ms; got {captured:?}")
            });
        assert!(
            hit.1.contains("severity_tier=suggested"),
            "expected the bounded severity label verbatim; fields were: {}",
            hit.1
        );
        assert!(
            hit.1.contains("duration_ms=42.5"),
            "expected the measured duration verbatim; fields were: {}",
            hit.1
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_constellation_discovery_latency_emits_exact_target_with_aggregate_count() {
        let api = TelemetryApiImpl::new();
        let (events, subscriber) = field_sink();
        let guard = tracing::subscriber::set_default(subscriber);
        api.record_constellation_discovery_latency(ConstellationDiscoveryInput {
            duration_ms: 1200.0,
            discovered_count: 3,
        })
        .await
        .expect("ok");
        drop(guard);
        let captured = events.lock().expect("lock").clone();

        let hit = captured
            .iter()
            .find(|(target, _)| target == "metric.constellation.discovery_ms")
            .unwrap_or_else(|| {
                panic!("expected metric.constellation.discovery_ms; got {captured:?}")
            });
        assert!(
            hit.1.contains("discovered_count=3"),
            "expected the aggregate count verbatim; fields were: {}",
            hit.1
        );
        assert!(
            !hit.1.contains("service"),
            "aggregate-only: no per-service identifier may appear; fields were: {}",
            hit.1
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_findings_counter_refresh_emits_exact_target() {
        let api = TelemetryApiImpl::new();
        let (events, subscriber) = field_sink();
        let guard = tracing::subscriber::set_default(subscriber);
        api.record_findings_counter_refresh(FindingsCounterRefreshInput { duration_ms: 850.0 })
            .await
            .expect("ok");
        drop(guard);
        let captured = events.lock().expect("lock").clone();

        let hit = captured
            .iter()
            .find(|(target, _)| target == "metric.findings.counter_refresh_ms")
            .unwrap_or_else(|| {
                panic!("expected metric.findings.counter_refresh_ms; got {captured:?}")
            });
        assert!(
            hit.1.contains("duration_ms=850"),
            "expected the measured duration verbatim; fields were: {}",
            hit.1
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn delegated_timing_resolvers_reject_non_finite_duration() {
        let api = TelemetryApiImpl::new();

        let halo = api
            .clone()
            .record_constellation_hue_latency(ConstellationHueLatencyInput {
                duration_ms: f64::NAN,
                severity_tier: HueSeverityTier::None,
            })
            .await;
        assert!(matches!(
            halo,
            Err(AppError::Validation { ref field, ref reason })
                if field == "duration_ms" && reason == "non-finite"
        ));

        let constellation = api
            .clone()
            .record_constellation_discovery_latency(ConstellationDiscoveryInput {
                duration_ms: f64::INFINITY,
                discovered_count: 1,
            })
            .await;
        assert!(matches!(
            constellation,
            Err(AppError::Validation { ref field, ref reason })
                if field == "duration_ms" && reason == "non-finite"
        ));

        let findings = api
            .record_findings_counter_refresh(FindingsCounterRefreshInput { duration_ms: -1.0 })
            .await;
        assert!(matches!(
            findings,
            Err(AppError::Validation { ref field, ref reason })
                if field == "duration_ms" && reason == "out of range"
        ));
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_constellation_discovery_latency_rejects_implausible_count() {
        let api = TelemetryApiImpl::new();
        let result = api
            .record_constellation_discovery_latency(ConstellationDiscoveryInput {
                duration_ms: 10.0,
                discovered_count: DISCOVERED_COUNT_MAX + 1,
            })
            .await;
        match result {
            Err(AppError::Validation { field, reason }) => {
                assert_eq!(field, "discovered_count");
                assert_eq!(reason, "out of range");
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

    #[test]
    fn ipc_rejection_category_serializes_snake_case_bounded_labels() {
        for (category, expected) in [
            (IpcRejectionCategory::AclRejected, "\"acl_rejected\""),
            (IpcRejectionCategory::Other, "\"other\""),
        ] {
            let json = serde_json::to_string(&category).expect("serializes");
            assert_eq!(json, expected);
        }
    }

    #[test]
    fn ipc_rejection_input_rejects_unknown_category() {
        let json = r#"{"error_category":"panic","window_label":"report","payload_bytes":10}"#;
        let result: Result<IpcRejectionInput, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "closed enum must reject an unknown error_category (no free-form String escape)",
        );
    }

    #[test]
    fn coerce_window_label_collapses_unknown_to_constant() {
        assert_eq!(coerce_window_label("main"), "main");
        assert_eq!(coerce_window_label("compact-widget"), "compact-widget");
        assert_eq!(coerce_window_label("findings"), "findings");
        assert_eq!(coerce_window_label("report"), "report");
        assert_eq!(coerce_window_label("evil-injection-attempt"), "unknown");
        assert_eq!(coerce_window_label(""), "unknown");
    }

    #[test]
    fn validate_payload_bytes_accepts_zero_and_max_boundary() {
        assert!(validate_payload_bytes(0).is_ok());
        assert!(validate_payload_bytes(PAYLOAD_BYTES_MAX).is_ok());
    }

    #[test]
    fn validate_payload_bytes_rejects_above_max() {
        let err = validate_payload_bytes(PAYLOAD_BYTES_MAX + 1).expect_err("above max must reject");
        match err {
            AppError::Validation { field, reason } => {
                assert_eq!(field, "payload_bytes");
                assert_eq!(reason, "out of range");
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_ipc_rejection_emits_exact_target_with_coerced_label_and_category() {
        let api = TelemetryApiImpl::new();
        let (events, subscriber) = field_sink();
        let guard = tracing::subscriber::set_default(subscriber);
        api.record_ipc_rejection(IpcRejectionInput {
            error_category: IpcRejectionCategory::AclRejected,
            window_label: "evil-injection-attempt".to_string(),
            payload_bytes: 1234,
        })
        .await
        .expect("ok");
        drop(guard);
        let captured = events.lock().expect("lock").clone();

        let hit = captured
            .iter()
            .find(|(target, _)| target == "ui.ipc.rejection")
            .unwrap_or_else(|| panic!("expected ui.ipc.rejection; got {captured:?}"));
        assert!(
            hit.1.contains("error_category=acl_rejected"),
            "expected the bounded category verbatim; fields were: {}",
            hit.1
        );
        assert!(
            hit.1.contains("window_label=unknown"),
            "an out-of-set label must land COERCED, never verbatim; fields were: {}",
            hit.1
        );
        assert!(
            !hit.1.contains("evil-injection-attempt"),
            "the raw webview-supplied label must never reach the record; fields were: {}",
            hit.1
        );
        assert!(
            hit.1.contains("payload_bytes=1234"),
            "expected the byte count verbatim; fields were: {}",
            hit.1
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_ipc_rejection_emits_at_warn_level() {
        let events: Arc<Mutex<Vec<(String, tracing::Level)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        let guard = tracing::subscriber::set_default(subscriber);
        let api = TelemetryApiImpl::new();
        api.record_ipc_rejection(IpcRejectionInput {
            error_category: IpcRejectionCategory::Other,
            window_label: "report".to_string(),
            payload_bytes: 0,
        })
        .await
        .expect("ok");
        drop(guard);

        let captured = events.lock().expect("lock").clone();
        assert!(
            captured
                .iter()
                .any(|(t, l)| t == "ui.ipc.rejection" && *l == tracing::Level::WARN),
            "the rejection record is a WARN-class security event; captured: {captured:?}"
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_ipc_rejection_rejects_implausible_payload_bytes_without_emitting() {
        let api = TelemetryApiImpl::new();
        let (events, subscriber) = field_sink();
        let guard = tracing::subscriber::set_default(subscriber);
        let result = api
            .record_ipc_rejection(IpcRejectionInput {
                error_category: IpcRejectionCategory::AclRejected,
                window_label: "report".to_string(),
                payload_bytes: PAYLOAD_BYTES_MAX + 1,
            })
            .await;
        drop(guard);

        match result {
            Err(AppError::Validation { field, reason }) => {
                assert_eq!(field, "payload_bytes");
                assert_eq!(reason, "out of range");
            }
            other => panic!("expected Validation error, got {other:?}"),
        }
        let captured = events.lock().expect("lock").clone();
        assert!(
            !captured.iter().any(|(t, _)| t == "ui.ipc.rejection"),
            "a rejected input must not emit a record — the emit rides valid input only; got {captured:?}"
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    const NON_OBTAINED: [WebgpuAdapterOutcome; 4] = [
        WebgpuAdapterOutcome::NoNavigatorGpu,
        WebgpuAdapterOutcome::AdapterNull,
        WebgpuAdapterOutcome::AdapterRequestRejected,
        WebgpuAdapterOutcome::DeviceRequestFailed,
    ];

    #[test]
    fn webgpu_adapter_outcome_serializes_snake_case_bounded_labels() {
        for (outcome, expected) in [
            (WebgpuAdapterOutcome::Obtained, "obtained"),
            (WebgpuAdapterOutcome::NoNavigatorGpu, "no_navigator_gpu"),
            (WebgpuAdapterOutcome::AdapterNull, "adapter_null"),
            (
                WebgpuAdapterOutcome::AdapterRequestRejected,
                "adapter_request_rejected",
            ),
            (
                WebgpuAdapterOutcome::DeviceRequestFailed,
                "device_request_failed",
            ),
        ] {
            let json = serde_json::to_string(&outcome).expect("serializes");
            assert_eq!(json, format!("\"{expected}\""));
            assert_eq!(outcome.as_str(), expected);
        }
    }

    #[test]
    fn webgpu_adapter_input_rejects_unknown_outcome() {
        let json = r#"{"outcome":"GPUAdapter lost: driver reset","window_label":"main"}"#;
        let result: Result<WebgpuAdapterInput, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "closed enum must reject an unknown outcome (no raw error text crosses)",
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    async fn adapter_levels(outcome: WebgpuAdapterOutcome) -> Vec<(String, tracing::Level)> {
        let events: Arc<Mutex<Vec<(String, tracing::Level)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        let guard = tracing::subscriber::set_default(subscriber);
        TelemetryApiImpl::new()
            .record_webgpu_adapter(WebgpuAdapterInput {
                outcome,
                window_label: "compact-widget".to_string(),
            })
            .await
            .expect("ok");
        drop(guard);
        let captured = events.lock().expect("lock").clone();
        captured
            .into_iter()
            .filter(|(t, _)| t == "ui.webgpu.adapter")
            .collect()
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_webgpu_adapter_emits_one_info_record_on_obtained() {
        let hits = adapter_levels(WebgpuAdapterOutcome::Obtained).await;
        assert_eq!(
            hits,
            vec![("ui.webgpu.adapter".to_string(), tracing::Level::INFO)]
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_webgpu_adapter_emits_one_warn_record_on_every_other_outcome() {
        for outcome in NON_OBTAINED {
            let hits = adapter_levels(outcome).await;
            assert_eq!(
                hits,
                vec![("ui.webgpu.adapter".to_string(), tracing::Level::WARN)],
                "outcome {outcome:?}"
            );
        }
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn record_webgpu_adapter_carries_the_outcome_and_a_coerced_label_only() {
        let api = TelemetryApiImpl::new();
        let (events, subscriber) = field_sink();
        let guard = tracing::subscriber::set_default(subscriber);
        api.record_webgpu_adapter(WebgpuAdapterInput {
            outcome: WebgpuAdapterOutcome::AdapterNull,
            window_label: "evil-injection-attempt".to_string(),
        })
        .await
        .expect("ok");
        drop(guard);
        let captured = events.lock().expect("lock").clone();

        let hit = captured
            .iter()
            .find(|(target, _)| target == "ui.webgpu.adapter")
            .unwrap_or_else(|| panic!("expected ui.webgpu.adapter; got {captured:?}"));
        assert!(hit.1.contains("outcome=adapter_null"), "fields: {}", hit.1);
        assert!(hit.1.contains("window_label=unknown"), "fields: {}", hit.1);
        assert!(
            !hit.1.contains("evil-injection-attempt"),
            "fields: {}",
            hit.1
        );
    }
}
