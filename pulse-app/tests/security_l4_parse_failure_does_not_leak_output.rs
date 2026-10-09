//! Chunk #86 negative-canary PII test for the L4 parse-failure path.
//!
//! Injects a stubbed `LlmInferenceRunner` returning a malformed JSON
//! response containing a 32-char canary substring. Drives a single L4
//! inference invocation via `handle_digest_outcome`. Captures all tracing
//! events with the chunk #44 CapturingSubscriber + FieldCollector pattern
//! (per CLAUDE.md testing.md 2026-05-11 entry). Asserts the canary
//! substring is ABSENT from every event's target + every field value
//! (parse-failure event emission MUST NOT leak raw output bytes per
//! security plan §Anti-Patterns §Logging row 1).

use std::pin::Pin;
use std::sync::{Arc, Mutex};

use interpretation::contract::{
    InferenceFuture, LlmInferenceRunner, ModelIdentity, ModelStatus, ModelTier,
};
use pulse_app::inference_runtime::{L4DigestOutcome, handle_digest_outcome};
use triage::contract::{Digest, DigestKind, DigestLwwMode};

const CANARY_SUBSTRING: &str = "secret-canary-API-key-12345-svc";

struct CanaryMalformedRunner;

impl LlmInferenceRunner for CanaryMalformedRunner {
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
        _schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        // Malformed JSON with canary embedded; chunk #86 parse-failure path
        // MUST drop the canary at the boundary instead of logging it.
        let response = format!("{{\"not-valid-json\": \"{CANARY_SUBSTRING}-{CANARY_SUBSTRING}\"");
        Pin::from(Box::new(async move { Ok(response) }))
    }
}

struct CanarySchemaViolationRunner;

impl LlmInferenceRunner for CanarySchemaViolationRunner {
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
        _schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        // Well-formed JSON but missing required fields → SchemaViolation;
        // canary embedded in free-text field that MUST NOT be logged verbatim.
        let response = format!("{{\"schema_version\": \"{CANARY_SUBSTRING}\"}}");
        Pin::from(Box::new(async move { Ok(response) }))
    }
}

#[derive(Default)]
struct CapturedEvent {
    target: String,
    fields: String,
}

#[derive(Default)]
struct CapturingSubscriber {
    events: Arc<Mutex<Vec<CapturedEvent>>>,
}

impl CapturingSubscriber {
    fn new() -> (Self, Arc<Mutex<Vec<CapturedEvent>>>) {
        let events: Arc<Mutex<Vec<CapturedEvent>>> = Arc::new(Mutex::new(Vec::new()));
        (
            CapturingSubscriber {
                events: Arc::clone(&events),
            },
            events,
        )
    }
}

struct FieldCollector<'a> {
    sink: &'a mut String,
}

impl<'a> tracing::field::Visit for FieldCollector<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value:?};", field.name());
    }
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value};", field.name());
    }
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value};", field.name());
    }
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value};", field.name());
    }
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value};", field.name());
    }
}

impl tracing::Subscriber for CapturingSubscriber {
    fn enabled(&self, _meta: &tracing::Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, _span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }
    fn record(&self, _id: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _id: &tracing::span::Id, _follows: &tracing::span::Id) {}
    fn event(&self, event: &tracing::Event<'_>) {
        let metadata = event.metadata();
        let mut sink = String::new();
        let mut collector = FieldCollector { sink: &mut sink };
        event.record(&mut collector);
        self.events.lock().expect("lock").push(CapturedEvent {
            target: metadata.target().to_string(),
            fields: sink,
        });
    }
    fn enter(&self, _span: &tracing::span::Id) {}
    fn exit(&self, _span: &tracing::span::Id) {}
}

fn sample_digest() -> Digest {
    Digest {
        kind: DigestKind::CadenceTier3,
        token_count: 1024,
        payload_summary: "[redacted] sample digest".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000_000_000,
        workspace: "/home/dev/example".to_string(),
        window_start_unix_nano: 1_700_000_000_000_000_000,
        window_end_unix_nano: 1_700_000_001_000_000_000,
        services: vec![],
        attention_cues: vec![],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Default,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

#[tokio::test]
async fn l4_parse_failure_path_does_not_leak_canary_to_tracing() {
    let (subscriber, events) = CapturingSubscriber::new();
    let _guard = tracing::subscriber::set_default(subscriber);

    let runner = Arc::new(CanaryMalformedRunner);
    let digest = sample_digest();
    let outcome = handle_digest_outcome(runner.as_ref(), &digest).await;
    assert!(matches!(outcome, L4DigestOutcome::ParseFailure));

    drop(_guard);
    let captured = events.lock().expect("lock").drain(..).collect::<Vec<_>>();

    // Canary MUST NOT appear in any event's target OR field value across
    // the entire parse-failure code path. Per CLAUDE.md security 2026-05-11
    // CapturingSubscriber + FieldCollector pattern (chunk #44 precedent for
    // snapshot PII negative-canary, extended here to L4 parse-failure path).
    for ev in &captured {
        assert!(
            !ev.target.contains(CANARY_SUBSTRING),
            "canary leaked into event target: {}",
            ev.target
        );
        assert!(
            !ev.fields.contains(CANARY_SUBSTRING),
            "canary leaked into event fields ({}): {}",
            ev.target,
            ev.fields
        );
    }
    // Verify at least one parse-failure event WAS emitted (sanity — proves
    // the path was exercised, not silently skipped).
    let parse_events: Vec<&CapturedEvent> = captured
        .iter()
        .filter(|e| e.target == "interpretation.json.parse")
        .collect();
    assert!(
        !parse_events.is_empty(),
        "expected at least one interpretation.json.parse event"
    );
}

#[tokio::test]
async fn l4_schema_violation_path_does_not_leak_canary_to_tracing() {
    let (subscriber, events) = CapturingSubscriber::new();
    let _guard = tracing::subscriber::set_default(subscriber);

    let runner = Arc::new(CanarySchemaViolationRunner);
    let digest = sample_digest();
    let outcome = handle_digest_outcome(runner.as_ref(), &digest).await;
    // The exact failure variant depends on how serde + the bounded post-
    // parse validator interpret the partial JSON. Either ParseFailure
    // (deserialize rejects missing required fields) OR SchemaViolation
    // (deserialize succeeds + post-parse bounds check fails) is a valid
    // chunk #86 failure path for the canary-redaction guarantee.
    assert!(
        matches!(
            outcome,
            L4DigestOutcome::SchemaViolation | L4DigestOutcome::ParseFailure
        ),
        "expected SchemaViolation or ParseFailure outcome, got {outcome:?}",
    );

    drop(_guard);
    let captured = events.lock().expect("lock").drain(..).collect::<Vec<_>>();
    for ev in &captured {
        assert!(
            !ev.target.contains(CANARY_SUBSTRING),
            "canary leaked into schema-violation event target: {}",
            ev.target
        );
        assert!(
            !ev.fields.contains(CANARY_SUBSTRING),
            "canary leaked into schema-violation event fields ({}): {}",
            ev.target,
            ev.fields
        );
    }
}
