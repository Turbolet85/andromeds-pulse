use std::sync::Arc;
#[cfg(test)]
use std::time::Instant;

use arrow::array::{BinaryArray, Int32Array, Int64Array, StringArray, TimestampMicrosecondArray};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use arrow::record_batch::RecordBatch;
use duckdb::Connection;
use ingest::grpc::proto::opentelemetry::proto::logs::v1::ResourceLogs;
use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{ResourceMetrics, metric};
use ingest::grpc::proto::opentelemetry::proto::trace::v1::ResourceSpans;

use crate::contract::Error;

const TS_TZ_UTC: &str = "UTC";

fn timestamp_tz_type() -> DataType {
    DataType::Timestamp(TimeUnit::Microsecond, Some(TS_TZ_UTC.into()))
}

pub(crate) fn build_spans_record_batch(
    batch: &[ResourceSpans],
) -> Result<Option<RecordBatch>, Error> {
    let mut trace_ids: Vec<Vec<u8>> = Vec::new();
    let mut span_ids: Vec<Vec<u8>> = Vec::new();
    let mut tss: Vec<i64> = Vec::new();
    let mut ts_unix_nanos: Vec<i64> = Vec::new();

    for rs in batch {
        for ss in &rs.scope_spans {
            for span in &ss.spans {
                if span.trace_id.is_empty() || span.span_id.is_empty() {
                    // Receiver-side invariants (chunk #18) already reject these,
                    // but the appender stays defensive at the boundary.
                    continue;
                }
                trace_ids.push(span.trace_id.clone());
                span_ids.push(span.span_id.clone());
                let ns = span.start_time_unix_nano as i64;
                tss.push(ns / 1_000);
                ts_unix_nanos.push(ns);
            }
        }
    }

    if trace_ids.is_empty() {
        return Ok(None);
    }

    let trace_id_array = BinaryArray::from_iter_values(trace_ids.iter().map(|v| v.as_slice()));
    let span_id_array = BinaryArray::from_iter_values(span_ids.iter().map(|v| v.as_slice()));
    let ts_array = TimestampMicrosecondArray::from(tss).with_timezone(TS_TZ_UTC);
    let ts_unix_nano_array = Int64Array::from(ts_unix_nanos);

    let schema = Arc::new(Schema::new(vec![
        Field::new("trace_id", DataType::Binary, false),
        Field::new("span_id", DataType::Binary, false),
        Field::new("ts", timestamp_tz_type(), false),
        Field::new("ts_unix_nano", DataType::Int64, false),
    ]));

    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(trace_id_array),
            Arc::new(span_id_array),
            Arc::new(ts_array),
            Arc::new(ts_unix_nano_array),
        ],
    )
    .map_err(|e| Error::Append {
        reason: format!("record_batch(spans): {}", short_err(&e.to_string())),
    })?;

    Ok(Some(record_batch))
}

pub(crate) fn build_metrics_record_batch(
    batch: &[ResourceMetrics],
) -> Result<Option<RecordBatch>, Error> {
    let mut metric_names: Vec<String> = Vec::new();
    let mut tss: Vec<i64> = Vec::new();
    let mut ts_unix_nanos: Vec<i64> = Vec::new();
    let mut resource_hashes: Vec<Vec<u8>> = Vec::new();

    for rm in batch {
        let resource_hash = hash_resource(rm);
        for sm in &rm.scope_metrics {
            for m in &sm.metrics {
                if let Some(data) = &m.data {
                    collect_metric_points(
                        &m.name,
                        data,
                        &resource_hash,
                        &mut metric_names,
                        &mut tss,
                        &mut ts_unix_nanos,
                        &mut resource_hashes,
                    );
                }
            }
        }
    }

    if metric_names.is_empty() {
        return Ok(None);
    }

    let metric_name_array = StringArray::from(metric_names);
    let ts_array = TimestampMicrosecondArray::from(tss).with_timezone(TS_TZ_UTC);
    let ts_unix_nano_array = Int64Array::from(ts_unix_nanos);
    let resource_hash_array =
        BinaryArray::from_iter_values(resource_hashes.iter().map(|v| v.as_slice()));

    let schema = Arc::new(Schema::new(vec![
        Field::new("metric_name", DataType::Utf8, false),
        Field::new("ts", timestamp_tz_type(), false),
        Field::new("ts_unix_nano", DataType::Int64, false),
        Field::new("resource_hash", DataType::Binary, false),
    ]));

    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(metric_name_array),
            Arc::new(ts_array),
            Arc::new(ts_unix_nano_array),
            Arc::new(resource_hash_array),
        ],
    )
    .map_err(|e| Error::Append {
        reason: format!(
            "record_batch(metrics_points): {}",
            short_err(&e.to_string())
        ),
    })?;

    Ok(Some(record_batch))
}

pub(crate) fn build_logs_record_batch(
    batch: &[ResourceLogs],
) -> Result<Option<RecordBatch>, Error> {
    let mut tss: Vec<i64> = Vec::new();
    let mut ts_unix_nanos: Vec<i64> = Vec::new();
    let mut resource_hashes: Vec<Vec<u8>> = Vec::new();
    let mut severities: Vec<i32> = Vec::new();

    for rl in batch {
        let resource_hash = hash_resource_logs(rl);
        for sl in &rl.scope_logs {
            for log in &sl.log_records {
                let ns = log.time_unix_nano as i64;
                tss.push(ns / 1_000);
                ts_unix_nanos.push(ns);
                resource_hashes.push(resource_hash.clone());
                severities.push(log.severity_number);
            }
        }
    }

    if tss.is_empty() {
        return Ok(None);
    }

    let ts_array = TimestampMicrosecondArray::from(tss).with_timezone(TS_TZ_UTC);
    let ts_unix_nano_array = Int64Array::from(ts_unix_nanos);
    let resource_hash_array =
        BinaryArray::from_iter_values(resource_hashes.iter().map(|v| v.as_slice()));
    let severity_array = Int32Array::from(severities);

    let schema = Arc::new(Schema::new(vec![
        Field::new("ts", timestamp_tz_type(), false),
        Field::new("ts_unix_nano", DataType::Int64, false),
        Field::new("resource_hash", DataType::Binary, false),
        Field::new("severity_number", DataType::Int32, false),
    ]));

    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(ts_array),
            Arc::new(ts_unix_nano_array),
            Arc::new(resource_hash_array),
            Arc::new(severity_array),
        ],
    )
    .map_err(|e| Error::Append {
        reason: format!("record_batch(log_records): {}", short_err(&e.to_string())),
    })?;

    Ok(Some(record_batch))
}

pub(crate) fn append_record_batch_to_table(
    conn: &Connection,
    table_name: &'static str,
    record_batch: RecordBatch,
) -> Result<u64, Error> {
    let row_count = record_batch.num_rows() as u64;
    let mut appender = conn.appender(table_name).map_err(|e| Error::Append {
        reason: format!("appender({table_name}): {}", short_err(&e.to_string())),
    })?;
    appender
        .append_record_batch(record_batch)
        .map_err(|e| Error::Append {
            reason: format!(
                "append_record_batch({table_name}): {}",
                short_err(&e.to_string())
            ),
        })?;
    appender.flush().map_err(|e| Error::Append {
        reason: format!("flush({table_name}): {}", short_err(&e.to_string())),
    })?;
    Ok(row_count)
}

// Convenience wrappers used by appender's own tests. After chunk #23, the
// production path is consumer::dispatch_batch which calls build_*_record_batch
// + append_record_batch_to_table directly so the same RecordBatch can be
// reused for broadcast emission. These wrappers preserve the chunk #20 test
// surface without re-emerging in the production call graph.
#[cfg(test)]
pub(crate) fn append_spans_batch(conn: &Connection, batch: &[ResourceSpans]) -> Result<u64, Error> {
    let start = Instant::now();
    let Some(record_batch) = build_spans_record_batch(batch)? else {
        return Ok(0);
    };
    let row_count = append_record_batch_to_table(conn, "spans", record_batch)?;

    let duration_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        target: "duckdb.append",
        rows_appended = row_count,
        duration_ms = duration_ms,
        table_name = "spans",
        "Arrow appender wrote rows",
    );

    Ok(row_count)
}

#[cfg(test)]
pub(crate) fn append_metrics_batch(
    conn: &Connection,
    batch: &[ResourceMetrics],
) -> Result<u64, Error> {
    let start = Instant::now();
    let Some(record_batch) = build_metrics_record_batch(batch)? else {
        return Ok(0);
    };
    let row_count = append_record_batch_to_table(conn, "metrics_points", record_batch)?;

    let duration_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        target: "duckdb.append",
        rows_appended = row_count,
        duration_ms = duration_ms,
        table_name = "metrics_points",
        "Arrow appender wrote rows",
    );

    Ok(row_count)
}

#[cfg(test)]
pub(crate) fn append_logs_batch(conn: &Connection, batch: &[ResourceLogs]) -> Result<u64, Error> {
    let start = Instant::now();
    let Some(record_batch) = build_logs_record_batch(batch)? else {
        return Ok(0);
    };
    let row_count = append_record_batch_to_table(conn, "log_records", record_batch)?;

    let duration_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        target: "duckdb.append",
        rows_appended = row_count,
        duration_ms = duration_ms,
        table_name = "log_records",
        "Arrow appender wrote rows",
    );

    Ok(row_count)
}

fn collect_metric_points(
    name: &str,
    data: &metric::Data,
    resource_hash: &[u8],
    metric_names: &mut Vec<String>,
    tss: &mut Vec<i64>,
    ts_unix_nanos: &mut Vec<i64>,
    resource_hashes: &mut Vec<Vec<u8>>,
) {
    match data {
        metric::Data::Gauge(g) => {
            for p in &g.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                );
            }
        }
        metric::Data::Sum(s) => {
            for p in &s.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                );
            }
        }
        metric::Data::Histogram(h) => {
            for p in &h.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                );
            }
        }
        metric::Data::ExponentialHistogram(eh) => {
            for p in &eh.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                );
            }
        }
        metric::Data::Summary(s) => {
            for p in &s.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                );
            }
        }
    }
}

fn push_metric_row(
    name: &str,
    ts_ns: i64,
    resource_hash: &[u8],
    metric_names: &mut Vec<String>,
    tss: &mut Vec<i64>,
    ts_unix_nanos: &mut Vec<i64>,
    resource_hashes: &mut Vec<Vec<u8>>,
) {
    metric_names.push(name.to_string());
    tss.push(ts_ns / 1_000);
    ts_unix_nanos.push(ts_ns);
    resource_hashes.push(resource_hash.to_vec());
}

fn hash_resource(rm: &ResourceMetrics) -> Vec<u8> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    if let Some(r) = &rm.resource {
        for kv in &r.attributes {
            kv.key.hash(&mut hasher);
        }
        r.dropped_attributes_count.hash(&mut hasher);
    }
    hasher.finish().to_le_bytes().to_vec()
}

fn hash_resource_logs(rl: &ResourceLogs) -> Vec<u8> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    if let Some(r) = &rl.resource {
        for kv in &r.attributes {
            kv.key.hash(&mut hasher);
        }
        r.dropped_attributes_count.hash(&mut hasher);
    }
    hasher.finish().to_le_bytes().to_vec()
}

fn short_err(raw: &str) -> String {
    raw.lines().next().unwrap_or("").chars().take(120).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::create_schema;
    use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
    use ingest::grpc::proto::opentelemetry::proto::logs::v1::{LogRecord, ScopeLogs};
    use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{
        Gauge, Metric, NumberDataPoint, ScopeMetrics, number_data_point,
    };
    use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
    use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ScopeSpans, Span};

    fn fresh_conn_with_schema() -> Connection {
        let conn = Connection::open_in_memory().expect("open_in_memory");
        create_schema(&conn).expect("schema");
        conn
    }

    fn span_with_ids(trace_id: Vec<u8>, span_id: Vec<u8>, start_ns: u64) -> Span {
        Span {
            trace_id,
            span_id,
            name: "test-span".into(),
            start_time_unix_nano: start_ns,
            end_time_unix_nano: start_ns + 1,
            ..Default::default()
        }
    }

    fn wrap_spans(spans: Vec<Span>) -> Vec<ResourceSpans> {
        vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }]
    }

    #[test]
    fn append_spans_batch_inserts_rows_and_returns_count() {
        let conn = fresh_conn_with_schema();
        let batch = wrap_spans(vec![
            span_with_ids(vec![1u8; 16], vec![1u8; 8], 1_700_000_000_000_000_000),
            span_with_ids(vec![2u8; 16], vec![2u8; 8], 1_700_000_000_000_000_001),
        ]);

        let count = append_spans_batch(&conn, &batch).expect("append must succeed");
        assert_eq!(count, 2);

        let actual: i64 = conn
            .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
            .expect("count query");
        assert_eq!(actual, 2);
    }

    #[test]
    fn append_spans_batch_round_trips_nanosecond_precision() {
        let conn = fresh_conn_with_schema();
        let ns = 1_700_000_000_123_456_789_u64;
        let batch = wrap_spans(vec![span_with_ids(vec![3u8; 16], vec![3u8; 8], ns)]);

        append_spans_batch(&conn, &batch).expect("append");
        let read_back: i64 = conn
            .query_row("SELECT ts_unix_nano FROM spans LIMIT 1", [], |row| {
                row.get(0)
            })
            .expect("query_row");
        assert_eq!(read_back, ns as i64);
    }

    #[test]
    fn append_spans_batch_skips_malformed_zero_ids() {
        let conn = fresh_conn_with_schema();
        let mut malformed = span_with_ids(vec![], vec![], 1);
        malformed.start_time_unix_nano = 1_700_000_000_000_000_000;
        let batch = wrap_spans(vec![malformed]);

        let count = append_spans_batch(&conn, &batch).expect("append on empty input must succeed");
        assert_eq!(count, 0);
    }

    #[test]
    fn append_spans_batch_empty_input_returns_zero() {
        let conn = fresh_conn_with_schema();
        let count = append_spans_batch(&conn, &[]).expect("empty append");
        assert_eq!(count, 0);
    }

    #[test]
    fn build_spans_record_batch_returns_some_for_valid_input() {
        let batch = wrap_spans(vec![span_with_ids(
            vec![1u8; 16],
            vec![1u8; 8],
            1_700_000_000_000_000_000,
        )]);
        let result = build_spans_record_batch(&batch).expect("build");
        let rb = result.expect("must be Some for valid input");
        assert_eq!(rb.num_rows(), 1);
        assert_eq!(rb.num_columns(), 4);
    }

    #[test]
    fn build_spans_record_batch_returns_none_for_empty_input() {
        let result = build_spans_record_batch(&[]).expect("build");
        assert!(result.is_none(), "empty input must return None");
    }

    #[test]
    fn build_spans_record_batch_returns_none_when_all_spans_malformed() {
        let mut malformed = span_with_ids(vec![], vec![], 0);
        malformed.start_time_unix_nano = 1;
        let batch = wrap_spans(vec![malformed]);
        let result = build_spans_record_batch(&batch).expect("build");
        assert!(result.is_none(), "all-malformed input must return None");
    }

    fn make_resource(service: &str) -> Resource {
        Resource {
            attributes: vec![KeyValue {
                key: "service.name".into(),
                value: Some(AnyValue {
                    value: Some(any_value::Value::StringValue(service.into())),
                }),
            }],
            dropped_attributes_count: 0,
        }
    }

    #[test]
    fn append_metrics_batch_inserts_rows_and_returns_count() {
        let conn = fresh_conn_with_schema();
        let batch = vec![ResourceMetrics {
            resource: Some(make_resource("svc-a")),
            scope_metrics: vec![ScopeMetrics {
                scope: None,
                metrics: vec![Metric {
                    name: "requests.total".into(),
                    description: String::new(),
                    unit: String::new(),
                    metadata: Vec::new(),
                    data: Some(metric::Data::Gauge(Gauge {
                        data_points: vec![NumberDataPoint {
                            attributes: Vec::new(),
                            start_time_unix_nano: 0,
                            time_unix_nano: 1_700_000_000_000_000_000,
                            exemplars: Vec::new(),
                            flags: 0,
                            value: Some(number_data_point::Value::AsInt(42)),
                        }],
                    })),
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }];

        let count = append_metrics_batch(&conn, &batch).expect("append metrics");
        assert_eq!(count, 1);

        let actual: i64 = conn
            .query_row("SELECT COUNT(*) FROM metrics_points", [], |row| row.get(0))
            .expect("count");
        assert_eq!(actual, 1);
    }

    #[test]
    fn append_logs_batch_inserts_rows_and_returns_count() {
        let conn = fresh_conn_with_schema();
        let batch = vec![ResourceLogs {
            resource: Some(make_resource("svc-b")),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: vec![LogRecord {
                    time_unix_nano: 1_700_000_000_000_000_000,
                    observed_time_unix_nano: 0,
                    severity_number: 9,
                    severity_text: "INFO".into(),
                    body: None,
                    attributes: Vec::new(),
                    dropped_attributes_count: 0,
                    flags: 0,
                    trace_id: Vec::new(),
                    span_id: Vec::new(),
                    event_name: String::new(),
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }];

        let count = append_logs_batch(&conn, &batch).expect("append logs");
        assert_eq!(count, 1);

        let actual: i64 = conn
            .query_row("SELECT COUNT(*) FROM log_records", [], |row| row.get(0))
            .expect("count");
        assert_eq!(actual, 1);
    }
}
