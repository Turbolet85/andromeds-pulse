//! Acceptance for P-072 — Investigate actions functional (intent F12).
//!
//! Drives the public `investigate.run_action` resolver under the deterministic
//! L4 mode (P-073): each of the 4 actions yields a populated result,
//! reproducibly; an unknown id is a Validation error; a missing buffer is a
//! Storage error; runner + parse failures surface as `AppError` (never a silent
//! no-op); and the aggregate-only observability leaks no telemetry content.
//!
//! pulse-app's `[lib] test = false` (CLAUDE.md testing.md 2026-05-20) disables
//! source-level `mod tests`, and the resolver's private helpers are not
//! reachable cross-crate, so coverage runs entirely through the public API here.

use std::sync::{Arc, Mutex};

use duckdb::Connection;
use interpretation::contract::{
    InferenceError, InferenceFuture, LlmInferenceRunner, ModelIdentity, ModelStatus, ModelTier,
};
use pulse_app::deterministic_inference::DeterministicInferenceRunner;
use pulse_app::investigate_router::{InvestigateApi, InvestigateApiImpl};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};
use ui_bridge::contract::AppError;

const ACTIONS: &[&str] = &[
    "diagnose-latency-outlier",
    "find-error-correlation",
    "trace-failed-request",
    "summarize-service-health",
];

fn seeded_conn(service: &str) -> Arc<Mutex<Connection>> {
    let conn = Connection::open_in_memory().expect("in-memory DuckDB");
    buffer::create_schema(&conn).expect("schema create");
    let now_ns = chrono::Utc::now()
        .timestamp_nanos_opt()
        .expect("nanos in range");
    for i in 0..3u8 {
        conn.execute(
            "INSERT INTO spans (trace_id, span_id, ts, ts_unix_nano, service_name, end_time_unix_nano, status_code) VALUES (?, ?, '2026-06-28T00:00:00Z'::TIMESTAMPTZ, ?, ?, ?, ?)",
            duckdb::params![
                vec![1u8; 16],
                vec![i + 1; 8],
                now_ns - 1_000_000_000,
                service,
                now_ns - 999_000_000,
                0i32,
            ],
        )
        .expect("insert span");
    }
    Arc::new(Mutex::new(conn))
}

fn deterministic_impl(conn: Arc<Mutex<Connection>>) -> InvestigateApiImpl {
    InvestigateApiImpl::new(
        Some(conn),
        Arc::new(DeterministicInferenceRunner::new(ModelTier::Primary)),
    )
}

#[tokio::test]
async fn deterministic_mode_each_action_yields_a_populated_result() {
    let impl_ = deterministic_impl(seeded_conn("checkout"));
    for action in ACTIONS {
        let dto = impl_
            .clone()
            .run_action((*action).to_string())
            .await
            .unwrap_or_else(|e| panic!("action {action} must yield a result, got {e:?}"));
        assert_eq!(dto.action_id, *action);
        assert!(
            !dto.title.is_empty(),
            "action {action} must yield a non-empty result title (never a silent no-op)"
        );
    }
}

#[tokio::test]
async fn deterministic_mode_is_reproducible() {
    let impl_ = deterministic_impl(seeded_conn("checkout"));
    let a = impl_
        .clone()
        .run_action("diagnose-latency-outlier".to_string())
        .await
        .expect("first run ok");
    let b = impl_
        .clone()
        .run_action("diagnose-latency-outlier".to_string())
        .await
        .expect("second run ok");
    assert_eq!(
        a, b,
        "deterministic mode must produce a reproducible result"
    );
}

#[tokio::test]
async fn empty_buffer_still_yields_a_result() {
    let conn = Connection::open_in_memory().expect("in-memory DuckDB");
    buffer::create_schema(&conn).expect("schema create");
    let impl_ = deterministic_impl(Arc::new(Mutex::new(conn)));
    let dto = impl_
        .run_action("summarize-service-health".to_string())
        .await
        .expect("empty buffer is a valid run, not an error");
    assert!(!dto.title.is_empty());
}

#[tokio::test]
async fn unknown_action_id_is_validation_error() {
    let impl_ = deterministic_impl(seeded_conn("checkout"));
    match impl_.run_action("not-a-real-action".to_string()).await {
        Err(AppError::Validation { field, .. }) => assert_eq!(field, "action_id"),
        other => panic!("expected Validation(action_id), got {other:?}"),
    }
}

#[tokio::test]
async fn missing_buffer_is_storage_error() {
    let impl_ = InvestigateApiImpl::new(
        None,
        Arc::new(DeterministicInferenceRunner::new(ModelTier::Primary)),
    );
    match impl_
        .run_action("diagnose-latency-outlier".to_string())
        .await
    {
        Err(AppError::Storage { .. }) => {}
        other => panic!("expected Storage, got {other:?}"),
    }
}

struct FailingRunner;
impl LlmInferenceRunner for FailingRunner {
    fn current_status(&self) -> ModelStatus {
        ModelStatus::Error
    }
    fn identity(&self) -> Option<ModelIdentity> {
        None
    }
    fn tier(&self) -> ModelTier {
        ModelTier::Primary
    }
    fn generate_constrained<'a>(
        &'a self,
        _prompt: &'a str,
        _schema: &'a str,
    ) -> InferenceFuture<'a, String> {
        Box::pin(async move { Err(InferenceError::ModelNotConfigured) })
    }
}

#[tokio::test]
async fn runner_error_surfaces_never_silent() {
    let impl_ = InvestigateApiImpl::new(Some(seeded_conn("checkout")), Arc::new(FailingRunner));
    let res = impl_
        .run_action("diagnose-latency-outlier".to_string())
        .await;
    assert!(
        res.is_err(),
        "a runner error must surface, never a silent no-op"
    );
}

struct GarbageRunner;
impl LlmInferenceRunner for GarbageRunner {
    fn current_status(&self) -> ModelStatus {
        ModelStatus::Loaded
    }
    fn identity(&self) -> Option<ModelIdentity> {
        None
    }
    fn tier(&self) -> ModelTier {
        ModelTier::Primary
    }
    fn generate_constrained<'a>(
        &'a self,
        _prompt: &'a str,
        _schema: &'a str,
    ) -> InferenceFuture<'a, String> {
        Box::pin(async move { Ok("this is not valid L4 JSON".to_string()) })
    }
}

#[tokio::test]
async fn unparseable_output_surfaces_never_silent() {
    let impl_ = InvestigateApiImpl::new(Some(seeded_conn("checkout")), Arc::new(GarbageRunner));
    let res = impl_
        .run_action("diagnose-latency-outlier".to_string())
        .await;
    assert!(
        res.is_err(),
        "a parse failure must surface, never a silent no-op"
    );
}

// ---- PII negative-canary: no telemetry content in aggregate-only obs ----

struct CapturingSubscriber {
    events: Arc<Mutex<Vec<(String, String)>>>,
}

struct FieldCollector {
    sink: String,
}

impl Visit for FieldCollector {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={:?}", field.name(), value);
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={}", field.name(), value);
    }
    fn record_bool(&mut self, field: &Field, value: bool) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={}", field.name(), value);
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={}", field.name(), value);
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={}", field.name(), value);
    }
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
        let mut collector = FieldCollector {
            sink: String::new(),
        };
        event.record(&mut collector);
        self.events
            .lock()
            .expect("event lock not poisoned")
            .push((metadata.target().to_string(), collector.sink));
    }
    fn enter(&self, _: &Id) {}
    fn exit(&self, _: &Id) {}
}

#[tokio::test]
async fn investigate_does_not_log_telemetry_canary() {
    let canary = "secret-canary-service-name-42";
    let impl_ = deterministic_impl(seeded_conn(canary));

    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let subscriber = CapturingSubscriber {
        events: events.clone(),
    };
    // Global (not thread-local `set_default`) so the obs event emitted after the
    // resolver's internal `spawn_blocking` thread-hop is captured deterministically
    // — `set_default` is racy under in-process parallel libtest. This is the only
    // subscriber-setting test in the binary, so the once-per-process call is safe.
    tracing::subscriber::set_global_default(subscriber)
        .expect("global default settable once per test process");
    let _ = impl_
        .run_action("find-error-correlation".to_string())
        .await
        .expect("run ok");

    let captured = events.lock().expect("lock").clone();
    assert!(
        captured
            .iter()
            .any(|(t, _)| t == "investigate.run_action.request"),
        "expected the investigate.run_action.request obs event to be captured, got: {captured:?}"
    );
    for (target, fields) in captured.iter() {
        assert!(
            !target.contains(canary),
            "canary leaked into tracing target: {target}"
        );
        assert!(
            !fields.contains(canary),
            "canary leaked into tracing fields ({target}): {fields}"
        );
    }
}
