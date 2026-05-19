//! Chunk #69 Phase B Session 7+ E2E coverage — Drain template-mining
//! end-to-end through the OTLP HTTP receiver. Bootstraps a real in-process
//! pipeline (DuckDB connection + buffer state + DrainMiner + run_consumer +
//! `ingest::http` axum receiver on an ephemeral loopback port), drives 100
//! protobuf-encoded log records covering 4 distinct template shapes (one
//! PII-canary-bearing email body to exercise the security-scrubber
//! redaction path), and asserts:
//!
//! 1. `log_records.template_id` is non-NULL for every ingested row (the
//!    Drain assignment fires for every log record per chunk #69 Phase B
//!    plan §Acceptance Criteria (arch) `log_templates` table present /
//!    `log_records.template_id` populated).
//! 2. `DrainMiner::template_distribution(50)` returns ≥1 template with
//!    every `occurrence_count > 0` (plan §Acceptance Criteria (tests)
//!    integration test "≥1 template returned + all returned templates have
//!    `occurrence_count > 0`").
//! 3. No `log_templates.template_content` row carries the raw canary email
//!    pattern; at least one row carries the stable `[REDACTED:email]`
//!    marker (PII negative canary per security plan §Logging NEVER-log
//!    discipline + capability P-047).
//! 4. The `metric.pipeline.l1c.drain_template_count_total` event surfaces
//!    on the buffer.tick heartbeat path with the cumulative template
//!    count (asserted indirectly via miner.template_count()).

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use buffer::fingerprint::FingerprintObserver;
use buffer::{BroadcastSenders, BufferState, DrainConfig, DrainMiner, create_schema, run_consumer};
use duckdb::Connection;
use ingest::channel::build_channel;
use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::ExportLogsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::state::IngestState;
use prost::Message;

const CANARY_EMAIL_BODY: &str =
    "user signup from canary-secret-email@example.com on beta-tier startup";

fn str_attr(key: &str, value: &str) -> KeyValue {
    KeyValue {
        key: key.into(),
        value: Some(AnyValue {
            value: Some(any_value::Value::StringValue(value.into())),
        }),
    }
}

fn make_log_record(body: &str, ts_nano: u64) -> LogRecord {
    LogRecord {
        time_unix_nano: ts_nano,
        observed_time_unix_nano: 0,
        severity_number: 9,
        severity_text: "INFO".into(),
        body: Some(AnyValue {
            value: Some(any_value::Value::StringValue(body.into())),
        }),
        attributes: Vec::new(),
        dropped_attributes_count: 0,
        flags: 0,
        trace_id: Vec::new(),
        span_id: Vec::new(),
        event_name: String::new(),
    }
}

/// Build the 100-record diverse log batch:
///   - 50 records: "request completed in <N> ms" — collapses to 1 template
///     via the default NUM masking pattern.
///   - 30 records: "connection from <IP> dropped" — 1 template via IPv4 mask.
///   - 19 records: "user logged in successfully alpha tier" — 1 template
///     (identical body; no masking-required tokens).
///   - 1 record: CANARY_EMAIL_BODY (containing raw email) — drives the
///     security-scrubber redaction path through `write_template_to_table`.
///
/// Expected post-ingest state: ≥3 templates persisted in `log_templates`;
/// the email-bearing one stored as `[REDACTED:email]` per scrubber.
fn make_diverse_logs_request() -> ExportLogsServiceRequest {
    let mut log_records: Vec<LogRecord> = Vec::with_capacity(100);
    let base_ts: u64 = 1_700_000_000_000;
    for i in 0..50 {
        log_records.push(make_log_record(
            &format!("request completed in {} ms", i * 7 + 1),
            base_ts + i as u64,
        ));
    }
    for i in 0..30 {
        log_records.push(make_log_record(
            &format!("connection from 10.0.0.{} dropped", i + 1),
            base_ts + 1000 + i as u64,
        ));
    }
    for i in 0..19 {
        log_records.push(make_log_record(
            "user logged in successfully alpha tier",
            base_ts + 2000 + i as u64,
        ));
    }
    log_records.push(make_log_record(CANARY_EMAIL_BODY, base_ts + 3000));

    let resource = Resource {
        attributes: vec![str_attr("service.name", "drain-e2e-app")],
        dropped_attributes_count: 0,
    };
    ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(resource),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

async fn wait_for_log_rows(conn: &Arc<Mutex<Connection>>, target: i64, timeout: Duration) -> bool {
    let deadline = tokio::time::Instant::now() + timeout;
    while tokio::time::Instant::now() < deadline {
        let count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM log_records", [], |r| r.get(0))
            .unwrap_or(0);
        if count >= target {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    false
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn chunk_69_phase_b_drain_assigns_template_id_persists_log_templates_redacts_pii() {
    // ===== Bootstrap production wiring in-process =====
    let conn = {
        let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
        create_schema(&raw).expect("buffer schema creates");
        Arc::new(Mutex::new(raw))
    };
    let buffer_state = Arc::new(BufferState::new());
    let ingest_state = Arc::new(IngestState::new());
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());

    let drain_miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));

    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);

    let no_fingerprint_observer: Option<Arc<dyn FingerprintObserver>> = None;
    let _consumer_handle = tokio::spawn(run_consumer(
        receiver,
        Arc::clone(&conn),
        Arc::clone(&buffer_state),
        Arc::clone(&broadcast_senders),
        Arc::new(ingest::observer::NoopSpanObserver),
        no_fingerprint_observer,
        Some(Arc::clone(&drain_miner)),
    ));

    let listener = ingest::http::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port succeeds");
    let bound = listener.local_addr().expect("listener exposes local_addr");
    assert!(
        bound.ip().is_loopback(),
        "test receiver must bind only to loopback; got {}",
        bound.ip()
    );

    let _serve_handle = {
        let state = Arc::clone(&ingest_state);
        let sender_clone = Arc::clone(&sender);
        tokio::spawn(async move {
            let _ = ingest::http::serve_on(listener, state, sender_clone).await;
        })
    };

    // Best-effort wait for the axum server's accept loop to be ready.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // ===== Drive 100 diverse log records via OTLP HTTP =====
    let endpoint = format!("http://{}/v1/logs", bound);
    let body = make_diverse_logs_request().encode_to_vec();
    let resp = reqwest::Client::new()
        .post(&endpoint)
        .header("content-type", "application/x-protobuf")
        .body(body)
        .send()
        .await
        .expect("HTTP POST must reach loopback receiver");
    assert!(
        resp.status().is_success(),
        "OTLP HTTP /v1/logs must accept the 100-record batch; got {}",
        resp.status()
    );

    // ===== Wait for buffer consumer to persist all 100 rows =====
    let persisted = wait_for_log_rows(&conn, 100, Duration::from_secs(5)).await;
    assert!(
        persisted,
        "buffer should ingest 100 log_records within 5s; got count = {}",
        conn.lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM log_records", [], |r| r
                .get::<_, i64>(0))
            .unwrap_or(-1)
    );

    // ===== Assert every log_records row has non-NULL template_id =====
    let with_template: i64 = conn
        .lock()
        .unwrap()
        .query_row(
            "SELECT COUNT(*) FROM log_records WHERE template_id IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .expect("count template_id");
    assert_eq!(
        with_template, 100,
        "DrainMiner MUST assign a non-NULL template_id for every ingested log record \
         (chunk #69 Phase B plan §Acceptance Criteria arch row 3)"
    );

    // ===== Assert DrainMiner template distribution is healthy =====
    let dist = drain_miner.template_distribution(50);
    assert!(
        !dist.is_empty(),
        "diagnostics.template_distribution(50) MUST return ≥1 template after 100-record \
         ingest; got empty distribution"
    );
    for entry in &dist {
        assert!(
            entry.occurrence_count > 0,
            "every distribution entry MUST have occurrence_count > 0; entry id={} \
             content={:?} count={}",
            entry.id,
            entry.content,
            entry.occurrence_count
        );
    }
    // The 4 distinct shapes (NUM-masked / IPv4-masked / identical / canary)
    // collapse to ≥3 templates after the masker fires; assert lower bound only
    // (variations across masker tuning would shift the upper bound).
    assert!(
        dist.len() >= 3,
        "expected ≥3 distinct templates from 4 distinct shape inputs; got {} templates: {:?}",
        dist.len(),
        dist.iter().map(|e| &e.content).collect::<Vec<_>>()
    );

    // ===== Assert PII negative canary on log_templates persistence =====
    let templates: Vec<String> = conn
        .lock()
        .unwrap()
        .prepare("SELECT template_content FROM log_templates")
        .expect("prepare log_templates select")
        .query_map([], |r| r.get::<_, String>(0))
        .expect("query log_templates")
        .map(|r| r.expect("read row"))
        .collect();

    assert!(
        !templates.is_empty(),
        "log_templates MUST hold at least one persisted template row"
    );

    for stored in &templates {
        assert!(
            !stored.contains("canary-secret-email@example.com"),
            "PII canary leaked into log_templates.template_content: {stored:?}"
        );
        assert!(
            !stored.contains("@example.com"),
            "PII fragment leaked into log_templates.template_content: {stored:?}"
        );
    }
    assert!(
        templates.iter().any(|t| t.contains("[REDACTED:email]")),
        "expected ≥1 log_templates.template_content row with [REDACTED:email] marker \
         (security::scrubber category label); got: {templates:?}"
    );

    // ===== Sanity: template count matches between miner + log_templates =====
    let template_rows: i64 = conn
        .lock()
        .unwrap()
        .query_row("SELECT COUNT(*) FROM log_templates", [], |r| r.get(0))
        .expect("count log_templates");
    assert_eq!(
        template_rows as u64,
        drain_miner.template_count(),
        "log_templates row count MUST equal DrainMiner.template_count() \
         (every newly-created cluster persists exactly one row)"
    );
}
