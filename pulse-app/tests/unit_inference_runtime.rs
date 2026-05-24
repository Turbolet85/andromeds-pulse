//! Integration tests for the L4 inference subscriber adapter
//! (`pulse_app::inference_runtime`). Per CLAUDE.md testing.md 2026-05-20
//! lesson, pulse-app tests live in this integration crate; source-level
//! `#[cfg(test)] mod tests` blocks would compile but never run due to
//! `[lib] test = false` Windows WebView2 workaround.
//!
//! Coverage:
//! - success path: canned valid L4Output JSON → success counter + latency
//!   distribution emitted
//! - parse-failure path: malformed JSON → parse_failure counter
//! - oversize path: payload > L4_OUTPUT_MAX_BYTES → output_too_large counter
//! - runtime-error path: stub returns `InferenceError::InferenceFailed`
//!   → runtime_error counter + interpretation.inference.error event
//! - lifecycle: subscriber survives `RecvError::Lagged`

use std::pin::Pin;
use std::sync::{Arc, Mutex};

use interpretation::contract::{
    InferenceError, InferenceFuture, LlmInferenceRunner, ModelIdentity, ModelStatus, ModelTier,
};
use interpretation::schema::{L4_OUTPUT_MAX_BYTES, L4Output};
use pulse_app::inference_runtime::handle_digest;
use triage::contract::{Digest, DigestKind, DigestLwwMode};

/// One captured tracing event: `(target, level, fields-string)`.
type CapturedEvent = (String, tracing::Level, String);

/// Shared event sink populated by [`CapturingSubscriber`].
type CapturedEvents = Arc<Mutex<Vec<CapturedEvent>>>;

/// Captures `tracing::Event` callbacks into а shared Vec for post-test
/// assertion. Pattern per CLAUDE.md testing.md 2026-05-09 set_default
/// guard convention.
struct CapturingSubscriber {
    events: CapturedEvents,
}

impl CapturingSubscriber {
    fn new() -> (Self, CapturedEvents) {
        let events: CapturedEvents = Arc::new(Mutex::new(Vec::new()));
        let sub = Self {
            events: Arc::clone(&events),
        };
        (sub, events)
    }
}

impl tracing::Subscriber for CapturingSubscriber {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }
    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
    fn event(&self, event: &tracing::Event<'_>) {
        let mut fields = String::new();
        struct Visit<'a>(&'a mut String);
        impl<'a> tracing::field::Visit for Visit<'a> {
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={:?}", field.name(), value);
            }
            fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={}", field.name(), value);
            }
            fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={}", field.name(), value);
            }
            fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={}", field.name(), value);
            }
            fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={}", field.name(), value);
            }
        }
        let mut visitor = Visit(&mut fields);
        event.record(&mut visitor);
        let mut guard = self.events.lock().expect("capture lock");
        guard.push((
            event.metadata().target().to_string(),
            *event.metadata().level(),
            fields,
        ));
    }
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}
}

/// Hand-rolled stub `LlmInferenceRunner` returning а canned outcome.
/// Per CLAUDE.md observability.md 2026-05-17 mocking discipline: small
/// impl, no `mockall` workspace dep needed.
struct StubInferenceRunner {
    tier: ModelTier,
    status: ModelStatus,
    response: Mutex<Option<Result<String, InferenceError>>>,
}

impl StubInferenceRunner {
    fn new_ok(tier: ModelTier, canned_json: String) -> Self {
        Self {
            tier,
            status: ModelStatus::Loaded,
            response: Mutex::new(Some(Ok(canned_json))),
        }
    }

    fn new_err(tier: ModelTier, status: ModelStatus, err: InferenceError) -> Self {
        Self {
            tier,
            status,
            response: Mutex::new(Some(Err(err))),
        }
    }
}

impl LlmInferenceRunner for StubInferenceRunner {
    fn current_status(&self) -> ModelStatus {
        self.status
    }

    fn identity(&self) -> Option<ModelIdentity> {
        None
    }

    fn tier(&self) -> ModelTier {
        self.tier
    }

    fn generate_constrained<'a>(
        &'a self,
        _prompt: &'a str,
        _schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        let outcome = self
            .response
            .lock()
            .expect("stub lock")
            .take()
            .unwrap_or(Err(InferenceError::InferenceFailed {
                reason: "stub exhausted".into(),
            }));
        Pin::from(Box::new(async move { outcome }))
    }
}

fn sample_digest() -> Digest {
    Digest {
        kind: DigestKind::CadenceTier3,
        token_count: 1024,
        payload_summary: "WINDOW: ... SERVICES: example-svc ...".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000,
        workspace: "/home/dev/example".to_string(),
        window_start_unix_nano: 1_700_000_000_000,
        window_end_unix_nano: 1_700_000_060_000,
        services: vec![],
        attention_cues: vec![],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Default,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

fn valid_l4_output_json() -> String {
    let v = L4Output {
        schema_version: "2.0".into(),
        prompt_version: "v2.1".into(),
        decision: interpretation::schema::Decision::Surface,
        severity: interpretation::schema::Severity::Suggested,
        title: "Test title".into(),
        symptom: "Test symptom".into(),
        timeline: "Test timeline".into(),
        hypotheses: vec![],
        investigation_steps: vec![],
        evidence_refs: vec![],
        fingerprint: "test-fp".into(),
        model_tier: "primary".into(),
        hardware_profile: "cpu_primary".into(),
        is_resolution_summary: false,
    };
    serde_json::to_string(&v).expect("serialize ok")
}

fn captured_targets(events: &[CapturedEvent]) -> Vec<&str> {
    events.iter().map(|(t, _, _)| t.as_str()).collect()
}

#[tokio::test]
async fn handle_digest_success_path_emits_counter_and_latency() {
    let (subscriber, events) = CapturingSubscriber::new();
    let runner = StubInferenceRunner::new_ok(ModelTier::Primary, valid_l4_output_json());
    let digest = sample_digest();

    let guard = tracing::subscriber::set_default(subscriber);
    handle_digest(&runner, &digest).await;
    drop(guard);

    let captured = events.lock().expect("capture lock").clone();
    let targets = captured_targets(&captured);
    assert!(
        targets.contains(&"interpretation.prompt.assemble"),
        "expected interpretation.prompt.assemble event; got {targets:?}"
    );
    assert!(
        targets.contains(&"interpretation.constrained.generate"),
        "expected interpretation.constrained.generate event"
    );
    assert!(
        targets.contains(&"interpretation.json.parse"),
        "expected interpretation.json.parse event"
    );
    assert!(
        targets.contains(&"interpretation.inference.request"),
        "expected interpretation.inference.request event"
    );
    let counter_evt = captured
        .iter()
        .find(|(t, _, _)| t == "metric.pipeline.l4.inferences_total")
        .expect("counter event present");
    assert!(
        counter_evt.2.contains("result=success"),
        "counter result label must be 'success'; got {}",
        counter_evt.2
    );
    assert!(
        captured
            .iter()
            .any(|(t, _, _)| t == "metric.pipeline.l4.inference_latency_p99_milliseconds")
    );
}

#[tokio::test]
async fn handle_digest_parse_failure_increments_parse_failure_counter() {
    let (subscriber, events) = CapturingSubscriber::new();
    let runner = StubInferenceRunner::new_ok(ModelTier::Primary, "{ this is not json }".into());
    let digest = sample_digest();

    let guard = tracing::subscriber::set_default(subscriber);
    handle_digest(&runner, &digest).await;
    drop(guard);

    let captured = events.lock().expect("capture lock").clone();
    let counter_evt = captured
        .iter()
        .find(|(t, _, _)| t == "metric.pipeline.l4.inferences_total")
        .expect("counter event present");
    assert!(
        counter_evt.2.contains("result=parse_failure"),
        "counter result label must be 'parse_failure'; got {}",
        counter_evt.2
    );
    // No latency metric on parse failure path (only on success + runtime_error).
    let parse_evt = captured
        .iter()
        .find(|(t, _, _)| t == "interpretation.json.parse")
        .expect("json.parse event present");
    assert!(parse_evt.2.contains("parse_outcome=json_parse_failed"));
}

#[tokio::test]
async fn handle_digest_output_too_large_increments_output_too_large_counter() {
    let (subscriber, events) = CapturingSubscriber::new();
    let oversize = "x".repeat(L4_OUTPUT_MAX_BYTES + 1);
    let runner = StubInferenceRunner::new_ok(ModelTier::Primary, oversize);
    let digest = sample_digest();

    let guard = tracing::subscriber::set_default(subscriber);
    handle_digest(&runner, &digest).await;
    drop(guard);

    let captured = events.lock().expect("capture lock").clone();
    let counter_evt = captured
        .iter()
        .find(|(t, _, _)| t == "metric.pipeline.l4.inferences_total")
        .expect("counter event present");
    assert!(
        counter_evt.2.contains("result=output_too_large"),
        "counter result label must be 'output_too_large'; got {}",
        counter_evt.2
    );
    let parse_evt = captured
        .iter()
        .find(|(t, _, _)| t == "interpretation.json.parse")
        .expect("json.parse event present");
    assert!(parse_evt.2.contains("parse_outcome=output_too_large"));
}

#[tokio::test]
async fn handle_digest_runtime_error_increments_runtime_error_counter() {
    let (subscriber, events) = CapturingSubscriber::new();
    let runner = StubInferenceRunner::new_err(
        ModelTier::Primary,
        ModelStatus::Loaded,
        InferenceError::InferenceFailed {
            reason: "test".into(),
        },
    );
    let digest = sample_digest();

    let guard = tracing::subscriber::set_default(subscriber);
    handle_digest(&runner, &digest).await;
    drop(guard);

    let captured = events.lock().expect("capture lock").clone();
    let counter_evt = captured
        .iter()
        .find(|(t, _, _)| t == "metric.pipeline.l4.inferences_total")
        .expect("counter event present");
    assert!(
        counter_evt.2.contains("result=runtime_error"),
        "counter result label must be 'runtime_error'; got {}",
        counter_evt.2
    );
    let error_evt = captured
        .iter()
        .find(|(t, _, _)| t == "interpretation.inference.error")
        .expect("interpretation.inference.error event present");
    assert!(error_evt.2.contains("error_category=inference_failed"));
}

#[tokio::test]
async fn handle_digest_model_not_configured_increments_runtime_error_counter() {
    let (subscriber, events) = CapturingSubscriber::new();
    let runner = StubInferenceRunner::new_err(
        ModelTier::Primary,
        ModelStatus::Error,
        InferenceError::ModelNotConfigured,
    );
    let digest = sample_digest();

    let guard = tracing::subscriber::set_default(subscriber);
    handle_digest(&runner, &digest).await;
    drop(guard);

    let captured = events.lock().expect("capture lock").clone();
    let counter_evt = captured
        .iter()
        .find(|(t, _, _)| t == "metric.pipeline.l4.inferences_total")
        .expect("counter event present");
    assert!(counter_evt.2.contains("result=runtime_error"));
    let error_evt = captured
        .iter()
        .find(|(t, _, _)| t == "interpretation.inference.error")
        .expect("interpretation.inference.error event present");
    assert!(error_evt.2.contains("error_category=model_not_configured"));
}

#[tokio::test]
async fn handle_digest_schema_violation_increments_schema_violation_counter() {
    // Construct а JSON payload that parses cleanly but fails post-parse
    // validation (e.g., title > TITLE_MAX_LEN).
    let mut v: serde_json::Value =
        serde_json::from_str(&valid_l4_output_json()).expect("baseline parse ok");
    v["title"] = serde_json::Value::String("x".repeat(interpretation::schema::TITLE_MAX_LEN + 1));
    let oversized_title_json = serde_json::to_string(&v).expect("serialize ok");

    let (subscriber, events) = CapturingSubscriber::new();
    let runner = StubInferenceRunner::new_ok(ModelTier::Primary, oversized_title_json);
    let digest = sample_digest();

    let guard = tracing::subscriber::set_default(subscriber);
    handle_digest(&runner, &digest).await;
    drop(guard);

    let captured = events.lock().expect("capture lock").clone();
    let counter_evt = captured
        .iter()
        .find(|(t, _, _)| t == "metric.pipeline.l4.inferences_total")
        .expect("counter event present");
    assert!(
        counter_evt.2.contains("result=schema_violation"),
        "counter result label must be 'schema_violation'; got {}",
        counter_evt.2
    );
    let parse_evt = captured
        .iter()
        .find(|(t, _, _)| t == "interpretation.json.parse")
        .expect("json.parse event present");
    assert!(parse_evt.2.contains("parse_outcome=schema_violation"));
}

#[tokio::test]
async fn handle_digest_never_logs_prompt_or_output_content() {
    // PII guard per security extract + obs anti-pattern: the prompt body
    // (which embeds digest.payload_summary verbatim) + the raw model
    // output MUST NEVER appear in any tracing event field.
    let canary_in_digest = "SECRET-CANARY-API-KEY-payload";
    let canary_in_output = "SECRET-CANARY-OUTPUT-TOKEN";

    let mut digest = sample_digest();
    digest.payload_summary = format!("digest body {canary_in_digest}");

    let mut v: serde_json::Value =
        serde_json::from_str(&valid_l4_output_json()).expect("baseline parse ok");
    v["title"] = serde_json::Value::String(format!("title {canary_in_output}"));
    let canary_output_json = serde_json::to_string(&v).expect("serialize ok");

    let (subscriber, events) = CapturingSubscriber::new();
    let runner = StubInferenceRunner::new_ok(ModelTier::Primary, canary_output_json);

    let guard = tracing::subscriber::set_default(subscriber);
    handle_digest(&runner, &digest).await;
    drop(guard);

    let captured = events.lock().expect("capture lock").clone();
    for (target, _level, fields) in captured.iter() {
        assert!(
            !fields.contains(canary_in_digest),
            "digest canary leaked into target {target}: {fields}"
        );
        assert!(
            !fields.contains(canary_in_output),
            "output canary leaked into target {target}: {fields}"
        );
    }
}
