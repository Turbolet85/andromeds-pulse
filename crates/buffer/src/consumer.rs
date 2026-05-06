use std::sync::{Arc, Mutex};

use duckdb::Connection;
use ingest::channel::{Batch, IngestReceiver};

use crate::appender::{append_logs_batch, append_metrics_batch, append_spans_batch};
use crate::contract::Error;
use crate::state::BufferState;

/// Long-running consumer task that drains the ingest mpsc receiver and writes
/// each batch to DuckDB via the Arrow appender. Runs DuckDB calls inside
/// `tokio::task::spawn_blocking` to keep the tokio runtime healthy under
/// DuckDB's blocking-IO semantics.
///
/// Replaces the placeholder `while rx.recv().await.is_some() { /* drop */ }`
/// in `pulse-app/src/main.rs` (chunk #18 → chunk #20 hand-off).
pub async fn run_consumer(
    mut receiver: IngestReceiver,
    conn: Arc<Mutex<Connection>>,
    state: Arc<BufferState>,
) {
    while let Some(batch) = receiver.recv().await {
        let conn_clone = Arc::clone(&conn);
        let state_clone = Arc::clone(&state);

        let join =
            tokio::task::spawn_blocking(move || dispatch_batch(&conn_clone, &state_clone, batch))
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
    batch: Batch,
) -> Result<(), Error> {
    let guard = conn.lock().map_err(|_| Error::ConnectionLost)?;
    let rows = match batch {
        Batch::Spans(s) => append_spans_batch(&guard, &s)?,
        Batch::Metrics(m) => append_metrics_batch(&guard, &m)?,
        Batch::Logs(l) => append_logs_batch(&guard, &l)?,
    };
    drop(guard);
    state.record_rows_appended(rows);
    Ok(())
}

fn describe_error(e: &Error) -> &'static str {
    match e {
        Error::Init { .. } => "init_failed",
        Error::SchemaCreate { .. } => "schema_create_failed",
        Error::Append { .. } => "append_failed",
        Error::ConnectionLost => "connection_lost",
        Error::InvalidBatch { .. } => "invalid_batch",
        Error::Retention { .. } => "retention_failed",
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
    }
}
