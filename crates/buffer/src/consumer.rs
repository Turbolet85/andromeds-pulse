use std::sync::{Arc, Mutex};

use bytes::Bytes;
use duckdb::Connection;
use ingest::channel::{Batch, IngestReceiver};

use crate::appender::{
    append_record_batch_to_table, build_logs_record_batch, build_metrics_record_batch,
    build_spans_record_batch,
};
use crate::broadcast::{
    self, BroadcastSenders, MAX_PAYLOAD_BYTES, STREAM_NAME_LOGS, STREAM_NAME_METRICS,
    STREAM_NAME_SPANS,
};
use crate::contract::Error;
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
) {
    while let Some(batch) = receiver.recv().await {
        let conn_clone = Arc::clone(&conn);
        let state_clone = Arc::clone(&state);
        let senders_clone = Arc::clone(&broadcast_senders);

        let join = tokio::task::spawn_blocking(move || {
            dispatch_batch(&conn_clone, &state_clone, &senders_clone, batch)
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
) -> Result<(), Error> {
    let guard = conn.lock().map_err(|_| Error::ConnectionLost)?;

    let (rows, sender, encoded) = match batch {
        Batch::Spans(s) => {
            let Some(rb) = build_spans_record_batch(&s)? else {
                return Ok(());
            };
            let encoded = encode_or_log(broadcast::encode_spans(&rb), STREAM_NAME_SPANS);
            let rows = append_record_batch_to_table(&guard, "spans", rb)?;
            (rows, &senders.spans, encoded)
        }
        Batch::Metrics(m) => {
            let Some(rb) = build_metrics_record_batch(&m)? else {
                return Ok(());
            };
            let encoded = encode_or_log(broadcast::encode_metrics(&rb), STREAM_NAME_METRICS);
            let rows = append_record_batch_to_table(&guard, "metrics_points", rb)?;
            (rows, &senders.metrics, encoded)
        }
        Batch::Logs(l) => {
            let Some(rb) = build_logs_record_batch(&l)? else {
                return Ok(());
            };
            let encoded = encode_or_log(broadcast::encode_logs(&rb), STREAM_NAME_LOGS);
            let rows = append_record_batch_to_table(&guard, "log_records", rb)?;
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::create_schema;
    use ingest::channel::build_channel;
    use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};

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
    }
}
