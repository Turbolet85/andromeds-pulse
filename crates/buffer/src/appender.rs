use std::sync::Arc;
#[cfg(test)]
use std::time::Instant;

use arrow::array::{
    BinaryArray, Float64Array, Int32Array, Int64Array, StringArray, TimestampMicrosecondArray,
};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use arrow::record_batch::RecordBatch;
use duckdb::Connection;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::logs::v1::ResourceLogs;
use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{
    NumberDataPoint, ResourceMetrics, metric, number_data_point,
};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
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
    let mut service_names: Vec<String> = Vec::new();
    let mut end_time_unix_nanos: Vec<i64> = Vec::new();
    let mut status_codes: Vec<i32> = Vec::new();

    for rs in batch {
        let service_name = extract_service_name(rs.resource.as_ref());
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
                service_names.push(service_name.clone());
                end_time_unix_nanos.push(span.end_time_unix_nano as i64);
                // OTLP Status.code: 0 Unset, 1 Ok, 2 Error per spec.
                status_codes.push(span.status.as_ref().map(|s| s.code).unwrap_or(0));
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
    let service_name_array = StringArray::from(service_names);
    let end_time_unix_nano_array = Int64Array::from(end_time_unix_nanos);
    let status_code_array = Int32Array::from(status_codes);

    let schema = Arc::new(Schema::new(vec![
        Field::new("trace_id", DataType::Binary, false),
        Field::new("span_id", DataType::Binary, false),
        Field::new("ts", timestamp_tz_type(), false),
        Field::new("ts_unix_nano", DataType::Int64, false),
        Field::new("service_name", DataType::Utf8, false),
        Field::new("end_time_unix_nano", DataType::Int64, false),
        Field::new("status_code", DataType::Int32, false),
    ]));

    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(trace_id_array),
            Arc::new(span_id_array),
            Arc::new(ts_array),
            Arc::new(ts_unix_nano_array),
            Arc::new(service_name_array),
            Arc::new(end_time_unix_nano_array),
            Arc::new(status_code_array),
        ],
    )
    .map_err(|e| Error::Append {
        reason: format!("record_batch(spans): {}", short_err(&e.to_string())),
    })?;

    Ok(Some(record_batch))
}

pub(crate) fn extract_service_name(resource: Option<&Resource>) -> String {
    let Some(r) = resource else {
        return String::new();
    };
    for kv in &r.attributes {
        if kv.key == "service.name"
            && let Some(av) = &kv.value
            && let Some(any_value::Value::StringValue(s)) = &av.value
        {
            return s.clone();
        }
    }
    String::new()
}

pub(crate) fn build_metrics_record_batch(
    batch: &[ResourceMetrics],
) -> Result<Option<RecordBatch>, Error> {
    let mut metric_names: Vec<String> = Vec::new();
    let mut tss: Vec<i64> = Vec::new();
    let mut ts_unix_nanos: Vec<i64> = Vec::new();
    let mut resource_hashes: Vec<Vec<u8>> = Vec::new();
    let mut values: Vec<f64> = Vec::new();
    let mut kinds: Vec<i32> = Vec::new();

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
                        &mut values,
                        &mut kinds,
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
    let value_array = Float64Array::from(values);
    let kind_array = Int32Array::from(kinds);

    let schema = Arc::new(Schema::new(vec![
        Field::new("metric_name", DataType::Utf8, false),
        Field::new("ts", timestamp_tz_type(), false),
        Field::new("ts_unix_nano", DataType::Int64, false),
        Field::new("resource_hash", DataType::Binary, false),
        Field::new("value", DataType::Float64, false),
        Field::new("data_point_kind", DataType::Int32, false),
    ]));

    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(metric_name_array),
            Arc::new(ts_array),
            Arc::new(ts_unix_nano_array),
            Arc::new(resource_hash_array),
            Arc::new(value_array),
            Arc::new(kind_array),
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
    let mut bodies: Vec<String> = Vec::new();
    let mut severity_texts: Vec<String> = Vec::new();
    let mut trace_ids: Vec<Vec<u8>> = Vec::new();
    let mut span_ids: Vec<Vec<u8>> = Vec::new();

    for rl in batch {
        let resource_hash = hash_resource_logs(rl);
        for sl in &rl.scope_logs {
            for log in &sl.log_records {
                let ns = log.time_unix_nano as i64;
                tss.push(ns / 1_000);
                ts_unix_nanos.push(ns);
                resource_hashes.push(resource_hash.clone());
                severities.push(log.severity_number);
                bodies.push(extract_log_body(log.body.as_ref()));
                severity_texts.push(log.severity_text.clone());
                trace_ids.push(log.trace_id.clone());
                span_ids.push(log.span_id.clone());
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
    let body_array = StringArray::from(bodies);
    let severity_text_array = StringArray::from(severity_texts);
    let trace_id_array = BinaryArray::from_iter_values(trace_ids.iter().map(|v| v.as_slice()));
    let span_id_array = BinaryArray::from_iter_values(span_ids.iter().map(|v| v.as_slice()));

    let schema = Arc::new(Schema::new(vec![
        Field::new("ts", timestamp_tz_type(), false),
        Field::new("ts_unix_nano", DataType::Int64, false),
        Field::new("resource_hash", DataType::Binary, false),
        Field::new("severity_number", DataType::Int32, false),
        Field::new("body", DataType::Utf8, false),
        Field::new("severity_text", DataType::Utf8, false),
        Field::new("trace_id", DataType::Binary, false),
        Field::new("span_id", DataType::Binary, false),
    ]));

    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(ts_array),
            Arc::new(ts_unix_nano_array),
            Arc::new(resource_hash_array),
            Arc::new(severity_array),
            Arc::new(body_array),
            Arc::new(severity_text_array),
            Arc::new(trace_id_array),
            Arc::new(span_id_array),
        ],
    )
    .map_err(|e| Error::Append {
        reason: format!("record_batch(log_records): {}", short_err(&e.to_string())),
    })?;

    Ok(Some(record_batch))
}

fn extract_log_body(body: Option<&AnyValue>) -> String {
    let Some(av) = body else {
        return String::new();
    };
    match &av.value {
        Some(any_value::Value::StringValue(s)) => s.clone(),
        _ => String::new(),
    }
}

fn extract_data_point_value(p: &NumberDataPoint) -> f64 {
    match &p.value {
        Some(number_data_point::Value::AsInt(i)) => *i as f64,
        Some(number_data_point::Value::AsDouble(d)) => *d,
        None => 0.0,
    }
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

// data_point_kind discriminants per OTLP `metric::Data` enum order:
// Gauge=0, Sum=1, Histogram=2, ExponentialHistogram=3, Summary=4. Histogram
// / ExponentialHistogram / Summary points have no scalar `value` shape; we
// encode 0.0 as placeholder (future scope: extend with bucket aggregates).
#[allow(clippy::too_many_arguments)]
fn collect_metric_points(
    name: &str,
    data: &metric::Data,
    resource_hash: &[u8],
    metric_names: &mut Vec<String>,
    tss: &mut Vec<i64>,
    ts_unix_nanos: &mut Vec<i64>,
    resource_hashes: &mut Vec<Vec<u8>>,
    values: &mut Vec<f64>,
    kinds: &mut Vec<i32>,
) {
    match data {
        metric::Data::Gauge(g) => {
            for p in &g.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    extract_data_point_value(p),
                    0,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                    values,
                    kinds,
                );
            }
        }
        metric::Data::Sum(s) => {
            for p in &s.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    extract_data_point_value(p),
                    1,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                    values,
                    kinds,
                );
            }
        }
        metric::Data::Histogram(h) => {
            for p in &h.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    0.0,
                    2,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                    values,
                    kinds,
                );
            }
        }
        metric::Data::ExponentialHistogram(eh) => {
            for p in &eh.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    0.0,
                    3,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                    values,
                    kinds,
                );
            }
        }
        metric::Data::Summary(s) => {
            for p in &s.data_points {
                push_metric_row(
                    name,
                    p.time_unix_nano as i64,
                    resource_hash,
                    0.0,
                    4,
                    metric_names,
                    tss,
                    ts_unix_nanos,
                    resource_hashes,
                    values,
                    kinds,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn push_metric_row(
    name: &str,
    ts_ns: i64,
    resource_hash: &[u8],
    value: f64,
    kind: i32,
    metric_names: &mut Vec<String>,
    tss: &mut Vec<i64>,
    ts_unix_nanos: &mut Vec<i64>,
    resource_hashes: &mut Vec<Vec<u8>>,
    values: &mut Vec<f64>,
    kinds: &mut Vec<i32>,
) {
    metric_names.push(name.to_string());
    tss.push(ts_ns / 1_000);
    ts_unix_nanos.push(ts_ns);
    resource_hashes.push(resource_hash.to_vec());
    values.push(value);
    kinds.push(kind);
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
        assert_eq!(rb.num_columns(), 7);
    }

    #[test]
    fn build_spans_record_batch_extracts_service_name_from_resource() {
        let span = span_with_ids(vec![1u8; 16], vec![1u8; 8], 1_700_000_000_000_000_000);
        let batch = vec![ResourceSpans {
            resource: Some(make_resource("svc-traces")),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![span],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }];
        let conn = fresh_conn_with_schema();
        append_spans_batch(&conn, &batch).expect("append");
        let service: String = conn
            .query_row("SELECT service_name FROM spans LIMIT 1", [], |r| r.get(0))
            .expect("service_name read");
        assert_eq!(service, "svc-traces");
    }

    #[test]
    fn build_spans_record_batch_writes_end_time_and_status_code() {
        let mut span = span_with_ids(vec![5u8; 16], vec![5u8; 8], 1_700_000_000_000_000_000);
        span.end_time_unix_nano = 1_700_000_000_005_000_000; // +5ms
        span.status = Some(
            ingest::grpc::proto::opentelemetry::proto::trace::v1::Status {
                code: 2, // Error per OTLP spec
                message: String::new(),
            },
        );
        let conn = fresh_conn_with_schema();
        append_spans_batch(&conn, &wrap_spans(vec![span])).expect("append");
        let (end_ns, status_code): (i64, i32) = conn
            .query_row(
                "SELECT end_time_unix_nano, status_code FROM spans LIMIT 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .expect("read");
        assert_eq!(end_ns, 1_700_000_000_005_000_000);
        assert_eq!(status_code, 2);
    }

    #[test]
    fn build_spans_record_batch_handles_missing_resource_as_empty_service() {
        let span = span_with_ids(vec![6u8; 16], vec![6u8; 8], 1_700_000_000_000_000_000);
        let conn = fresh_conn_with_schema();
        append_spans_batch(&conn, &wrap_spans(vec![span])).expect("append");
        let service: String = conn
            .query_row("SELECT service_name FROM spans LIMIT 1", [], |r| r.get(0))
            .expect("service_name read");
        assert_eq!(service, "");
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

    #[test]
    fn build_metrics_record_batch_extracts_value_and_kind() {
        let conn = fresh_conn_with_schema();
        let batch = vec![ResourceMetrics {
            resource: Some(make_resource("svc-vk")),
            scope_metrics: vec![ScopeMetrics {
                scope: None,
                metrics: vec![
                    Metric {
                        name: "cpu.usage".into(),
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
                    },
                    Metric {
                        name: "request.duration_ms".into(),
                        description: String::new(),
                        unit: String::new(),
                        metadata: Vec::new(),
                        data: Some(metric::Data::Sum(
                            ingest::grpc::proto::opentelemetry::proto::metrics::v1::Sum {
                                data_points: vec![NumberDataPoint {
                                    attributes: Vec::new(),
                                    start_time_unix_nano: 0,
                                    time_unix_nano: 1_700_000_000_000_000_001,
                                    exemplars: Vec::new(),
                                    flags: 0,
                                    value: Some(number_data_point::Value::AsDouble(7.5)),
                                }],
                                aggregation_temporality: 0,
                                is_monotonic: false,
                            },
                        )),
                    },
                ],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }];

        append_metrics_batch(&conn, &batch).expect("append");

        let rows: Vec<(String, f64, i32)> = conn
            .prepare(
                "SELECT metric_name, value, data_point_kind FROM metrics_points \
                 ORDER BY ts_unix_nano",
            )
            .expect("prepare")
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .expect("query")
            .map(|r| r.expect("row"))
            .collect();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "cpu.usage");
        assert_eq!(rows[0].1, 42.0);
        assert_eq!(rows[0].2, 0); // Gauge
        assert_eq!(rows[1].0, "request.duration_ms");
        assert_eq!(rows[1].1, 7.5);
        assert_eq!(rows[1].2, 1); // Sum
    }

    #[test]
    fn build_logs_record_batch_extracts_body_and_severity_text() {
        let conn = fresh_conn_with_schema();
        let batch = vec![ResourceLogs {
            resource: Some(make_resource("svc-c")),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: vec![LogRecord {
                    time_unix_nano: 1_700_000_000_000_000_000,
                    observed_time_unix_nano: 0,
                    severity_number: 17,
                    severity_text: "ERROR".into(),
                    body: Some(AnyValue {
                        value: Some(any_value::Value::StringValue("connection refused".into())),
                    }),
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

        append_logs_batch(&conn, &batch).expect("append");

        let (body, severity_text): (String, String) = conn
            .query_row(
                "SELECT body, severity_text FROM log_records LIMIT 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .expect("read");
        assert_eq!(body, "connection refused");
        assert_eq!(severity_text, "ERROR");
    }

    #[test]
    fn build_logs_record_batch_extracts_trace_id_and_span_id_for_correlation() {
        let conn = fresh_conn_with_schema();
        let batch = vec![ResourceLogs {
            resource: Some(make_resource("svc-d")),
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
                    trace_id: vec![3u8; 16],
                    span_id: vec![4u8; 8],
                    event_name: String::new(),
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }];

        append_logs_batch(&conn, &batch).expect("append");

        let (trace_id, span_id): (Vec<u8>, Vec<u8>) = conn
            .query_row(
                "SELECT trace_id, span_id FROM log_records LIMIT 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .expect("read");
        assert_eq!(trace_id, vec![3u8; 16]);
        assert_eq!(span_id, vec![4u8; 8]);
    }
}
