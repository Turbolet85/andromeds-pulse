use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use arrow::record_batch::RecordBatch;
use bytes::Bytes;
use duckdb::Connection;
use ingest::channel::{Batch, IngestReceiver};
use ingest::grpc::proto::opentelemetry::proto::trace::v1::ResourceSpans;
use ingest::observer::SpanObserver;

use crate::appender::{
    append_record_batch_to_table, build_logs_record_batch, build_metrics_record_batch,
    build_span_events_record_batch, build_spans_record_batch, extract_service_name,
};
use crate::broadcast::{
    self, BroadcastSenders, MAX_PAYLOAD_BYTES, STREAM_NAME_LOGS, STREAM_NAME_METRICS,
    STREAM_NAME_SPANS,
};
use crate::contract::Error;
use crate::drain::DrainMiner;
use crate::fingerprint::FingerprintObserver;
use crate::state::BufferState;

/// Long-running consumer task that drains the ingest mpsc receiver and writes
/// each batch to DuckDB via the Arrow appender. Runs DuckDB calls inside
/// `tokio::task::spawn_blocking` to keep the tokio runtime healthy under
/// DuckDB's blocking-IO semantics.
///
/// After successful append, the same RecordBatch is encoded as Arrow IPC bytes
/// and fanned out via `tokio::sync::broadcast` to per-stream subscribers
/// (chunk #23). Encode happens BEFORE append (so broadcast and DuckDB see the
/// same data); broadcast emit happens AFTER append succeeds (so subscribers
/// only see durably-stored data).
pub async fn run_consumer(
    mut receiver: IngestReceiver,
    conn: Arc<Mutex<Connection>>,
    state: Arc<BufferState>,
    broadcast_senders: Arc<BroadcastSenders>,
    span_observer: Arc<dyn SpanObserver>,
    fingerprint_observer: Option<Arc<dyn FingerprintObserver>>,
    // Chunk #69 Phase B — Drain log-template miner. When `Some`, log records
    // pass through the miner before DuckDB append; `log_records.template_id`
    // gets populated per assignment. When `None`, log_records.template_id
    // stays NULL (pre-Drain-assignment mode).
    drain_miner: Option<Arc<DrainMiner>>,
) {
    while let Some(batch) = receiver.recv().await {
        // Chunk #62 baseline tap: per-span observation feeds the triage
        // BaselineState before DuckDB write so baseline tracking is
        // decoupled from storage path. Only span batches participate —
        // metrics/logs do not feed the streaming baseline trackers.
        if let Batch::Spans(ref spans) = batch {
            observe_spans_for_baseline(spans, &span_observer);
        }

        let conn_clone = Arc::clone(&conn);
        let state_clone = Arc::clone(&state);
        let senders_clone = Arc::clone(&broadcast_senders);
        let fingerprint_observer_clone = fingerprint_observer.clone();
        let drain_miner_clone = drain_miner.clone();

        let join = tokio::task::spawn_blocking(move || {
            dispatch_batch(
                &conn_clone,
                &state_clone,
                &senders_clone,
                batch,
                fingerprint_observer_clone.as_deref(),
                drain_miner_clone.as_deref(),
            )
        })
        .await;

        match join {
            Ok(Ok(_)) => {
                // Successful dispatch; row count already recorded into BufferState.
            }
            Ok(Err(e)) => {
                tracing::error!(
                    target: "duckdb.append",
                    reject_reason = %describe_error(&e),
                    "appender returned error",
                );
            }
            Err(join_err) => {
                tracing::error!(
                    target: "duckdb.append",
                    reject_reason = if join_err.is_panic() { "panic" } else { "join_error" },
                    "blocking dispatch task failed",
                );
            }
        }
    }
}

fn dispatch_batch(
    conn: &Arc<Mutex<Connection>>,
    state: &BufferState,
    senders: &BroadcastSenders,
    batch: Batch,
    fingerprint_observer: Option<&dyn FingerprintObserver>,
    drain_miner: Option<&DrainMiner>,
) -> Result<(), Error> {
    let guard = conn.lock().map_err(|_| Error::ConnectionLost)?;

    let (rows, sender, encoded) = match batch {
        Batch::Spans(s) => {
            let Some(rb) = build_spans_record_batch(&s)? else {
                return Ok(());
            };
            let encoded = encode_or_log(broadcast::encode_spans(&rb), STREAM_NAME_SPANS);
            let rows = append_table_traced(&guard, "spans", rb)?;
            // chunk #65: span_events append on same guard. Row count NOT added
            // to `rows` — BufferState::rows_ingested keeps per-batch-type
            // semantics ("1 Batch::Spans = 1 increment"); span_events surface
            // independently via the duckdb.append tracing event.
            // chunk #66: fingerprint_observer (when Some) receives per-row
            // exception fingerprints during the build pass for storm detection.
            if let Some(events_rb) = build_span_events_record_batch(&s, fingerprint_observer)? {
                append_table_traced(&guard, "span_events", events_rb)?;
            }
            (rows, &senders.spans, encoded)
        }
        Batch::Metrics(m) => {
            let Some(rb) = build_metrics_record_batch(&m)? else {
                return Ok(());
            };
            let encoded = encode_or_log(broadcast::encode_metrics(&rb), STREAM_NAME_METRICS);
            let rows = append_table_traced(&guard, "metrics_points", rb)?;
            (rows, &senders.metrics, encoded)
        }
        Batch::Logs(l) => {
            // chunk #69 Phase B: drain_miner (when Some) assigns a
            // template_id per log record between OTLP decode and DuckDB
            // append; populates the nullable log_records.template_id
            // column. When None, template_id stays NULL.
            let Some(rb) = build_logs_record_batch(&l, drain_miner)? else {
                return Ok(());
            };
            let encoded = encode_or_log(broadcast::encode_logs(&rb), STREAM_NAME_LOGS);
            let rows = append_table_traced(&guard, "log_records", rb)?;
            (rows, &senders.logs, encoded)
        }
    };

    drop(guard);
    state.record_rows_appended(rows);

    if let Some(bytes) = encoded {
        // SendError when no subscribers — silently drop (best-effort emit).
        let _ = sender.send(bytes);
    }
    Ok(())
}

// Production-path duckdb.append tracing emission. Each table append surfaces
// one info event under target "duckdb.append" with rows_appended +
// duration_ms + table_name fields per obs-plan §4 must-trace P1 + the
// "duckdb" AllowList::production() entry. Test-side wrappers in appender.rs
// (#[cfg(test)] append_*_batch) keep their independent emission so unit
// tests don't depend on dispatch_batch wiring.
fn append_table_traced(
    guard: &Connection,
    table_name: &'static str,
    rb: RecordBatch,
) -> Result<u64, Error> {
    let start = Instant::now();
    let rows = append_record_batch_to_table(guard, table_name, rb)?;
    let duration_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        target: "duckdb.append",
        rows_appended = rows,
        duration_ms = duration_ms,
        table_name = table_name,
        "Arrow appender wrote rows",
    );
    Ok(rows)
}

fn encode_or_log(result: Result<Bytes, Error>, channel_name: &'static str) -> Option<Bytes> {
    match result {
        Ok(b) => Some(b),
        Err(Error::BroadcastSizeCapExceeded { payload_bytes }) => {
            tracing::error!(
                target: "tauri.channel.emit.size_exceeded",
                channel_name = channel_name,
                payload_size_bytes = payload_bytes,
                limit_bytes = MAX_PAYLOAD_BYTES,
                error_type = "size_exceeded",
                "Arrow IPC payload exceeded broadcast size cap; dropping",
            );
            None
        }
        Err(_other) => {
            tracing::error!(
                target: "tauri.channel.emit.error",
                channel_name = channel_name,
                error_type = "encode_failed",
                "Arrow IPC encode failed; dropping broadcast",
            );
            None
        }
    }
}

/// Chunk #62: invoke the span observer for each post-decode span in the
/// batch. Wall-clock derived once per batch (cheap; multiple spans share);
/// span fields (`service_name`, `operation_name`, `status_code`,
/// `latency_ms`) extracted from the OTLP proto types. Empty trace_id or
/// span_id spans are skipped — receiver-side invariants (chunk #18) already
/// reject these, but the tap stays defensive at the boundary mirroring
/// `build_spans_record_batch` discipline.
fn observe_spans_for_baseline(batch: &[ResourceSpans], observer: &Arc<dyn SpanObserver>) {
    let now_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0);
    for rs in batch {
        let service_name = extract_service_name(rs.resource.as_ref());
        for ss in &rs.scope_spans {
            for span in &ss.spans {
                if span.trace_id.is_empty() || span.span_id.is_empty() {
                    continue;
                }
                let status_code = span.status.as_ref().map(|s| s.code).unwrap_or(0) as u8;
                let latency_nanos = span
                    .end_time_unix_nano
                    .saturating_sub(span.start_time_unix_nano);
                let latency_ms = latency_nanos / 1_000_000;
                observer.observe_span(
                    &service_name,
                    &span.name,
                    status_code,
                    latency_ms,
                    now_nanos,
                );
            }
        }
    }
}

fn describe_error(e: &Error) -> &'static str {
    match e {
        Error::Init { .. } => "init_failed",
        Error::SchemaCreate { .. } => "schema_create_failed",
        Error::Append { .. } => "append_failed",
        Error::ConnectionLost => "connection_lost",
        Error::InvalidBatch { .. } => "invalid_batch",
        Error::Retention { .. } => "retention_failed",
        Error::BroadcastEncode { .. } => "broadcast_encode_failed",
        Error::BroadcastSizeCapExceeded { .. } => "broadcast_size_cap_exceeded",
        // Chunk #69 Phase B — Drain operation errors.
        Error::Drain { .. } => "drain_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::create_schema;
    use ingest::channel::build_channel;
    use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
    use ingest::observer::NoopSpanObserver;

    fn noop_observer() -> Arc<dyn SpanObserver> {
        Arc::new(NoopSpanObserver)
    }

    fn fresh_conn_with_schema() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().expect("open_in_memory");
        create_schema(&conn).expect("schema create");
        Arc::new(Mutex::new(conn))
    }

    fn span_batch(trace_id: u8, span_id: u8) -> Batch {
        Batch::Spans(vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![Span {
                    trace_id: vec![trace_id; 16],
                    span_id: vec![span_id; 8],
                    name: "x".into(),
                    start_time_unix_nano: 1_700_000_000_000_000_000,
                    end_time_unix_nano: 1_700_000_000_000_000_001,
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }])
    }

    fn span_batch_with_events(trace_id: u8, span_id: u8, event_count: usize) -> Batch {
        use ingest::grpc::proto::opentelemetry::proto::trace::v1::span;
        let events: Vec<span::Event> = (0..event_count)
            .map(|i| span::Event {
                time_unix_nano: 1_700_000_000_000_000_000 + i as u64,
                name: format!("evt-{i}"),
                attributes: Vec::new(),
                dropped_attributes_count: 0,
            })
            .collect();
        Batch::Spans(vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![Span {
                    trace_id: vec![trace_id; 16],
                    span_id: vec![span_id; 8],
                    name: "parent".into(),
                    start_time_unix_nano: 1_700_000_000_000_000_000,
                    end_time_unix_nano: 1_700_000_000_000_000_010,
                    events,
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }])
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn run_consumer_drains_receiver_and_records_rows() {
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());
        let senders = Arc::new(broadcast::create());

        let (sender, receiver) = build_channel();

        sender
            .try_send(span_batch(1, 1))
            .expect("send within capacity");
        sender
            .try_send(span_batch(2, 2))
            .expect("send within capacity");

        let handle = tokio::spawn(run_consumer(
            receiver,
            Arc::clone(&conn),
            Arc::clone(&state),
            Arc::clone(&senders),
            noop_observer(),
            None,
            None,
        ));

        // Drop sender so consumer's recv() returns None and the task exits.
        drop(sender);

        handle.await.expect("consumer must complete cleanly");

        assert_eq!(state.snapshot().rows_ingested, 2);

        let actual: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
            .expect("count");
        assert_eq!(actual, 2);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn run_consumer_broadcasts_encoded_arrow_to_subscriber() {
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());
        let senders = Arc::new(broadcast::create());

        // Subscribe BEFORE sending the batch so the receiver is attached.
        let mut spans_rx = senders.spans.subscribe();

        let (sender, receiver) = build_channel();
        sender
            .try_send(span_batch(7, 7))
            .expect("send within capacity");

        let handle = tokio::spawn(run_consumer(
            receiver,
            Arc::clone(&conn),
            Arc::clone(&state),
            Arc::clone(&senders),
            noop_observer(),
            None,
            None,
        ));

        let payload = tokio::time::timeout(std::time::Duration::from_secs(5), spans_rx.recv())
            .await
            .expect("recv within timeout")
            .expect("recv ok");
        assert!(!payload.is_empty(), "broadcast payload must be non-empty");

        // Verify Arrow IPC stream decodes
        use arrow::ipc::reader::StreamReader;
        let reader = StreamReader::try_new(payload.as_ref(), None).expect("stream reader");
        let batches: Vec<_> = reader.collect::<Result<_, _>>().expect("collect");
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].num_rows(), 1);

        drop(sender);
        handle.await.expect("consumer must complete cleanly");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn run_consumer_three_subscribers_each_receive_same_payload() {
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());
        let senders = Arc::new(broadcast::create());

        let mut r1 = senders.spans.subscribe();
        let mut r2 = senders.spans.subscribe();
        let mut r3 = senders.spans.subscribe();

        let (sender, receiver) = build_channel();
        sender
            .try_send(span_batch(9, 9))
            .expect("send within capacity");

        let handle = tokio::spawn(run_consumer(
            receiver,
            Arc::clone(&conn),
            Arc::clone(&state),
            Arc::clone(&senders),
            noop_observer(),
            None,
            None,
        ));

        let p1 = tokio::time::timeout(std::time::Duration::from_secs(5), r1.recv())
            .await
            .expect("r1 timeout")
            .expect("r1 ok");
        let p2 = tokio::time::timeout(std::time::Duration::from_secs(5), r2.recv())
            .await
            .expect("r2 timeout")
            .expect("r2 ok");
        let p3 = tokio::time::timeout(std::time::Duration::from_secs(5), r3.recv())
            .await
            .expect("r3 timeout")
            .expect("r3 ok");

        assert_eq!(p1, p2);
        assert_eq!(p2, p3);

        drop(sender);
        handle.await.expect("consumer must complete cleanly");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn run_consumer_with_no_subscribers_still_appends_to_duckdb() {
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());
        let senders = Arc::new(broadcast::create());

        let (sender, receiver) = build_channel();
        sender
            .try_send(span_batch(1, 1))
            .expect("send within capacity");

        let handle = tokio::spawn(run_consumer(
            receiver,
            Arc::clone(&conn),
            Arc::clone(&state),
            Arc::clone(&senders),
            noop_observer(),
            None,
            None,
        ));

        drop(sender);
        handle.await.expect("consumer must complete cleanly");

        // DuckDB row should still be present even with zero broadcast subscribers
        assert_eq!(state.snapshot().rows_ingested, 1);
        let actual: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
            .expect("count");
        assert_eq!(actual, 1);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn dispatch_batch_writes_span_events_alongside_spans_under_same_lock_guard() {
        // chunk #65: a Batch::Spans carrying events populates BOTH spans
        // AND span_events tables in a single dispatch. BufferState::rows_ingested
        // reflects parent spans only (per chunk #65 plan: span events count
        // surfaces via duckdb.append tracing, not via the rows_ingested
        // heartbeat field, to preserve "1 Batch = 1 increment" semantics).
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());
        let senders = Arc::new(broadcast::create());

        let (sender, receiver) = build_channel();
        sender
            .try_send(span_batch_with_events(11, 11, 3))
            .expect("send within capacity");

        let handle = tokio::spawn(run_consumer(
            receiver,
            Arc::clone(&conn),
            Arc::clone(&state),
            Arc::clone(&senders),
            noop_observer(),
            None,
            None,
        ));

        drop(sender);
        handle.await.expect("consumer must complete cleanly");

        // Parent spans: 1 (Batch contains 1 span)
        let spans_count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
            .expect("count spans");
        assert_eq!(spans_count, 1);

        // Span events: 3 (the span carries 3 events)
        let events_count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM span_events", [], |row| row.get(0))
            .expect("count span_events");
        assert_eq!(events_count, 3);

        // BufferState::rows_ingested reflects parent spans only, not events.
        assert_eq!(state.snapshot().rows_ingested, 1);
    }

    // chunk #66: end-to-end integration of the fingerprint observer hook
    // through the consumer's spawn_blocking dispatch. Verifies the observer
    // receives one callback per exception-bearing span event.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn run_consumer_invokes_fingerprint_observer_on_exception_span_events() {
        use crate::fingerprint::{ExceptionFingerprint, FingerprintObserver};
        use ingest::grpc::proto::opentelemetry::proto::common::v1::{
            AnyValue, KeyValue, any_value,
        };
        use ingest::grpc::proto::opentelemetry::proto::trace::v1::span;

        struct CapturingObs {
            captured: Arc<Mutex<Vec<(ExceptionFingerprint, String, i64)>>>,
        }
        impl FingerprintObserver for CapturingObs {
            fn on_fingerprint(
                &self,
                fingerprint: ExceptionFingerprint,
                service_name: &str,
                ts_unix_nano: i64,
            ) {
                self.captured.lock().unwrap().push((
                    fingerprint,
                    service_name.to_string(),
                    ts_unix_nano,
                ));
            }
        }

        let captured = Arc::new(Mutex::new(Vec::new()));
        let observer: Arc<dyn FingerprintObserver> = Arc::new(CapturingObs {
            captured: Arc::clone(&captured),
        });

        fn str_attr(key: &str, value: &str) -> KeyValue {
            KeyValue {
                key: key.into(),
                value: Some(AnyValue {
                    value: Some(any_value::Value::StringValue(value.into())),
                }),
            }
        }

        let event_with_exception = span::Event {
            time_unix_nano: 1_700_000_000_000_000_001,
            name: "exception".into(),
            attributes: vec![
                str_attr("exception.type", "java.lang.RuntimeException"),
                str_attr("exception.message", "boom"),
                str_attr(
                    "exception.stacktrace",
                    "    at com.example.Foo.bar(Foo.java:42)",
                ),
            ],
            dropped_attributes_count: 0,
        };
        let plain_event = span::Event {
            time_unix_nano: 1_700_000_000_000_000_002,
            name: "plain".into(),
            attributes: vec![],
            dropped_attributes_count: 0,
        };

        let batch = Batch::Spans(vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![Span {
                    trace_id: vec![20u8; 16],
                    span_id: vec![20u8; 8],
                    name: "parent".into(),
                    start_time_unix_nano: 1_700_000_000_000_000_000,
                    end_time_unix_nano: 1_700_000_000_000_000_010,
                    events: vec![event_with_exception, plain_event],
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }]);

        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());
        let senders = Arc::new(broadcast::create());

        let (sender, receiver) = build_channel();
        sender.try_send(batch).expect("send within capacity");

        let handle = tokio::spawn(run_consumer(
            receiver,
            Arc::clone(&conn),
            Arc::clone(&state),
            Arc::clone(&senders),
            noop_observer(),
            Some(observer),
            None,
        ));

        drop(sender);
        handle.await.expect("consumer must complete cleanly");

        let captured = captured.lock().unwrap();
        assert_eq!(
            captured.len(),
            1,
            "observer MUST be invoked exactly once (one exception event in batch)"
        );
        assert_eq!(captured[0].2, 1_700_000_000_000_000_001);

        // span_events row count is 2 (both events landed in DuckDB); only the
        // exception-bearing one fed the observer.
        let events_count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM span_events", [], |row| row.get(0))
            .expect("count span_events");
        assert_eq!(events_count, 2);

        let with_fp: i64 = conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT COUNT(*) FROM span_events WHERE fingerprint IS NOT NULL",
                [],
                |row| row.get(0),
            )
            .expect("count");
        assert_eq!(with_fp, 1);
    }

    #[test]
    fn describe_error_returns_constant_strings() {
        assert_eq!(
            describe_error(&Error::Init { reason: "x".into() }),
            "init_failed"
        );
        assert_eq!(
            describe_error(&Error::SchemaCreate { reason: "y".into() }),
            "schema_create_failed"
        );
        assert_eq!(
            describe_error(&Error::Append { reason: "z".into() }),
            "append_failed"
        );
        assert_eq!(describe_error(&Error::ConnectionLost), "connection_lost");
        assert_eq!(
            describe_error(&Error::InvalidBatch { kind: "x" }),
            "invalid_batch"
        );
        assert_eq!(
            describe_error(&Error::Retention { reason: "x".into() }),
            "retention_failed"
        );
        assert_eq!(
            describe_error(&Error::BroadcastEncode { reason: "x".into() }),
            "broadcast_encode_failed"
        );
        assert_eq!(
            describe_error(&Error::BroadcastSizeCapExceeded { payload_bytes: 9 }),
            "broadcast_size_cap_exceeded"
        );
        assert_eq!(
            describe_error(&Error::Drain { reason: "x".into() }),
            "drain_failed"
        );
    }
}
