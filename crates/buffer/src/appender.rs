use std::sync::Arc;
#[cfg(test)]
use std::time::Instant;

use arrow::array::{
    BinaryArray, Float64Array, Int32Array, Int64Array, StringArray, TimestampMicrosecondArray,
};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use arrow::record_batch::RecordBatch;
use duckdb::Connection;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::logs::v1::ResourceLogs;
use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{
    NumberDataPoint, ResourceMetrics, metric, number_data_point,
};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::ResourceSpans;

use crate::contract::Error;
use crate::drain::DrainMiner;
use crate::fingerprint::{
    ExceptionFingerprint, FingerprintObserver, compute_exception_fingerprint,
};

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
    drain_miner: Option<&DrainMiner>,
) -> Result<Option<RecordBatch>, Error> {
    let mut tss: Vec<i64> = Vec::new();
    let mut ts_unix_nanos: Vec<i64> = Vec::new();
    let mut resource_hashes: Vec<Vec<u8>> = Vec::new();
    let mut severities: Vec<i32> = Vec::new();
    let mut bodies: Vec<String> = Vec::new();
    let mut severity_texts: Vec<String> = Vec::new();
    let mut trace_ids: Vec<Vec<u8>> = Vec::new();
    let mut span_ids: Vec<Vec<u8>> = Vec::new();
    // Chunk #69 Phase B — per-record template_id assignment. Nullable column
    // (all-None when drain_miner is None; populated via miner.assign_at when
    // Some). Hot-path budget per Phase A spike: <50μs p99; spike measured
    // 2μs in isolation = 25× headroom for masking + LRU + cluster merge.
    let mut template_ids: Vec<Option<i64>> = Vec::new();

    for rl in batch {
        let resource_hash = hash_resource_logs(rl);
        for sl in &rl.scope_logs {
            for log in &sl.log_records {
                let ns = log.time_unix_nano as i64;
                let body = extract_log_body(log.body.as_ref());
                let template_id = drain_miner
                    .and_then(|m| m.assign_at(&body, ns))
                    .map(|id| id as i64);
                tss.push(ns / 1_000);
                ts_unix_nanos.push(ns);
                resource_hashes.push(resource_hash.clone());
                severities.push(log.severity_number);
                bodies.push(body);
                severity_texts.push(log.severity_text.clone());
                trace_ids.push(log.trace_id.clone());
                span_ids.push(log.span_id.clone());
                template_ids.push(template_id);
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
    let template_id_array = Int64Array::from(template_ids);

    let schema = Arc::new(Schema::new(vec![
        Field::new("ts", timestamp_tz_type(), false),
        Field::new("ts_unix_nano", DataType::Int64, false),
        Field::new("resource_hash", DataType::Binary, false),
        Field::new("severity_number", DataType::Int32, false),
        Field::new("body", DataType::Utf8, false),
        Field::new("severity_text", DataType::Utf8, false),
        Field::new("trace_id", DataType::Binary, false),
        Field::new("span_id", DataType::Binary, false),
        // Nullable: existing rows pre-Drain-assignment OR all rows when
        // Drain disabled retain NULL template_id per chunk #69 Phase B
        // plan §Acceptance Criteria (tests) schema migration scenario.
        Field::new("template_id", DataType::Int64, true),
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
            Arc::new(template_id_array),
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

pub(crate) fn extract_string_attribute(attrs: &[KeyValue], key: &str) -> Option<String> {
    for kv in attrs {
        if kv.key == key
            && let Some(av) = &kv.value
            && let Some(any_value::Value::StringValue(s)) = &av.value
        {
            return Some(s.clone());
        }
    }
    None
}

pub(crate) fn build_span_events_record_batch(
    batch: &[ResourceSpans],
    fingerprint_observer: Option<&dyn FingerprintObserver>,
) -> Result<Option<RecordBatch>, Error> {
    let mut trace_ids: Vec<Vec<u8>> = Vec::new();
    let mut span_ids: Vec<Vec<u8>> = Vec::new();
    let mut event_indices: Vec<i32> = Vec::new();
    let mut tss: Vec<i64> = Vec::new();
    let mut ts_unix_nanos: Vec<i64> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut exception_types: Vec<Option<String>> = Vec::new();
    let mut exception_messages: Vec<Option<String>> = Vec::new();
    let mut exception_stacktraces: Vec<Option<String>> = Vec::new();
    let mut fingerprints: Vec<Option<ExceptionFingerprint>> = Vec::new();
    let mut service_names: Vec<String> = Vec::new();

    for rs in batch {
        let service_name = extract_service_name(rs.resource.as_ref());
        for ss in &rs.scope_spans {
            for span in &ss.spans {
                if span.trace_id.is_empty() || span.span_id.is_empty() {
                    continue;
                }
                for (event_index, event) in span.events.iter().enumerate() {
                    let exception_type =
                        extract_string_attribute(&event.attributes, "exception.type");
                    let exception_message =
                        extract_string_attribute(&event.attributes, "exception.message");
                    let exception_stacktrace =
                        extract_string_attribute(&event.attributes, "exception.stacktrace");
                    let fingerprint = compute_exception_fingerprint(
                        exception_type.as_deref(),
                        exception_stacktrace.as_deref(),
                    );

                    trace_ids.push(span.trace_id.clone());
                    span_ids.push(span.span_id.clone());
                    event_indices.push(event_index as i32);
                    let ns = event.time_unix_nano as i64;
                    tss.push(ns / 1_000);
                    ts_unix_nanos.push(ns);
                    names.push(event.name.clone());
                    exception_types.push(exception_type);
                    exception_messages.push(exception_message);
                    exception_stacktraces.push(exception_stacktrace);
                    fingerprints.push(fingerprint);
                    service_names.push(service_name.clone());
                }
            }
        }
    }

    if trace_ids.is_empty() {
        return Ok(None);
    }

    // Chunk #66: fan-out fingerprints к observer BEFORE consuming ts_unix_nanos
    // into the Arrow Int64Array. service_names + ts_unix_nanos remain owned
    // by the function until the array constructors below consume them; the
    // observer hook receives copies (i64 + &str borrow).
    if let Some(observer) = fingerprint_observer {
        for (row_idx, fp_opt) in fingerprints.iter().enumerate() {
            if let Some(fp) = fp_opt {
                observer.on_fingerprint(*fp, &service_names[row_idx], ts_unix_nanos[row_idx]);
            }
        }
    }

    let trace_id_array = BinaryArray::from_iter_values(trace_ids.iter().map(|v| v.as_slice()));
    let span_id_array = BinaryArray::from_iter_values(span_ids.iter().map(|v| v.as_slice()));
    let event_index_array = Int32Array::from(event_indices);
    let ts_array = TimestampMicrosecondArray::from(tss).with_timezone(TS_TZ_UTC);
    let ts_unix_nano_array = Int64Array::from(ts_unix_nanos);
    let name_array = StringArray::from(names);
    let exception_type_array = StringArray::from(exception_types);
    let exception_message_array = StringArray::from(exception_messages);
    let exception_stacktrace_array = StringArray::from(exception_stacktraces);
    // Chunk #66 populates fingerprint inline via hash(exception.type + normalized
    // stacktrace); rows without exception.type stay NULL per chunk #65 substrate.
    let fingerprint_array = BinaryArray::from_iter(
        fingerprints
            .iter()
            .map(|f| f.as_ref().map(|b| b.as_slice())),
    );

    let schema = Arc::new(Schema::new(vec![
        Field::new("trace_id", DataType::Binary, false),
        Field::new("span_id", DataType::Binary, false),
        Field::new("event_index", DataType::Int32, false),
        Field::new("ts", timestamp_tz_type(), false),
        Field::new("ts_unix_nano", DataType::Int64, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("exception_type", DataType::Utf8, true),
        Field::new("exception_message", DataType::Utf8, true),
        Field::new("exception_stacktrace", DataType::Utf8, true),
        Field::new("fingerprint", DataType::Binary, true),
    ]));

    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(trace_id_array),
            Arc::new(span_id_array),
            Arc::new(event_index_array),
            Arc::new(ts_array),
            Arc::new(ts_unix_nano_array),
            Arc::new(name_array),
            Arc::new(exception_type_array),
            Arc::new(exception_message_array),
            Arc::new(exception_stacktrace_array),
            Arc::new(fingerprint_array),
        ],
    )
    .map_err(|e| Error::Append {
        reason: format!("record_batch(span_events): {}", short_err(&e.to_string())),
    })?;

    Ok(Some(record_batch))
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
    // Test helper passes None for drain_miner; Drain integration tests live
    // in `crates/buffer/src/drain.rs::tests` + the Session 5 e2e integration
    // test at `pulse-app/tests/e2e_drain_template_assignment.rs`.
    let Some(record_batch) = build_logs_record_batch(batch, None)? else {
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

#[cfg(test)]
pub(crate) fn append_span_events_batch(
    conn: &Connection,
    batch: &[ResourceSpans],
) -> Result<u64, Error> {
    let start = Instant::now();
    let Some(record_batch) = build_span_events_record_batch(batch, None)? else {
        return Ok(0);
    };
    let row_count = append_record_batch_to_table(conn, "span_events", record_batch)?;

    let duration_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        target: "duckdb.append",
        rows_appended = row_count,
        duration_ms = duration_ms,
        table_name = "span_events",
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
    use std::sync::Mutex;

    use crate::schema::create_schema;
    use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
    use ingest::grpc::proto::opentelemetry::proto::logs::v1::{LogRecord, ScopeLogs};
    use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{
        Gauge, Metric, NumberDataPoint, ScopeMetrics, number_data_point,
    };
    use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
    use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ScopeSpans, Span, span};

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

    // Chunk #69 Phase B Session 7+ (Step 27): PII negative canary.
    // Closes test-plan §12 2026-05-08 PII Vector 1 gap (raw OTLP attribute
    // values at the new log_templates persistence surface). The default
    // masker does NOT strip email-shaped tokens (its catalog is IP / path /
    // hex / number); the security::scrubber catches them BEFORE
    // `write_template_to_table` writes the row to DuckDB.
    #[test]
    fn drain_pii_canary_email_redacted_in_log_templates() {
        use crate::drain::{DrainConfig, DrainMiner, write_template_to_table};

        let conn = fresh_conn_with_schema();
        let miner = DrainMiner::new(DrainConfig::default_config(), None);

        let canary_body = "user logged in from canary-secret-email@example.com on alpha-tier";
        let batch = vec![ResourceLogs {
            resource: Some(make_resource("svc-pii-canary")),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: vec![LogRecord {
                    time_unix_nano: 1_700_000_000_000_000_000,
                    observed_time_unix_nano: 0,
                    severity_number: 9,
                    severity_text: "INFO".into(),
                    body: Some(AnyValue {
                        value: Some(any_value::Value::StringValue(canary_body.into())),
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

        let record_batch = build_logs_record_batch(&batch, Some(&miner))
            .expect("build batch ok")
            .expect("non-empty batch");
        append_record_batch_to_table(&conn, "log_records", record_batch).expect("append rows");

        let new_templates = miner.drain_newly_created_templates();
        assert!(
            !new_templates.is_empty(),
            "one canary log line must create exactly one new template"
        );
        for record in &new_templates {
            write_template_to_table(&conn, record).expect("write template row");
        }

        let templates: Vec<String> = conn
            .prepare("SELECT template_content FROM log_templates")
            .expect("prepare select")
            .query_map([], |r| r.get::<_, String>(0))
            .expect("query templates")
            .map(|r| r.expect("read row"))
            .collect();

        assert!(
            !templates.is_empty(),
            "PII canary test expects at least one stored template row"
        );

        for stored in &templates {
            assert!(
                !stored.contains("canary-secret-email@example.com"),
                "raw email leaked into log_templates.template_content: {stored:?}"
            );
            assert!(
                !stored.contains("@example.com"),
                "raw email fragment leaked into log_templates.template_content: {stored:?}"
            );
        }

        assert!(
            templates.iter().any(|t| t.contains("[REDACTED:email]")),
            "expected at least one log_templates.template_content row to carry the \
             stable [REDACTED:email] marker (security::scrubber category label); \
             got templates: {templates:?}"
        );
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

    // chunk #65 span_events tests follow.

    fn string_attr(key: &str, value: &str) -> KeyValue {
        KeyValue {
            key: key.into(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(value.into())),
            }),
        }
    }

    fn exception_attrs(typ: &str, msg: &str, stack: &str) -> Vec<KeyValue> {
        vec![
            string_attr("exception.type", typ),
            string_attr("exception.message", msg),
            string_attr("exception.stacktrace", stack),
        ]
    }

    fn span_event(name: &str, time_ns: u64, attrs: Vec<KeyValue>) -> span::Event {
        span::Event {
            time_unix_nano: time_ns,
            name: name.into(),
            attributes: attrs,
            dropped_attributes_count: 0,
        }
    }

    fn span_with_events(
        trace_id: Vec<u8>,
        span_id: Vec<u8>,
        start_ns: u64,
        events: Vec<span::Event>,
    ) -> Span {
        let mut s = span_with_ids(trace_id, span_id, start_ns);
        s.events = events;
        s
    }

    #[test]
    fn append_span_events_batch_inserts_rows_and_returns_count() {
        let conn = fresh_conn_with_schema();
        let span = span_with_events(
            vec![1u8; 16],
            vec![1u8; 8],
            1_700_000_000_000_000_000,
            vec![
                span_event("evt1", 1_700_000_000_000_000_001, vec![]),
                span_event("evt2", 1_700_000_000_000_000_002, vec![]),
                span_event("evt3", 1_700_000_000_000_000_003, vec![]),
            ],
        );
        let batch = wrap_spans(vec![span]);

        let count = append_span_events_batch(&conn, &batch).expect("append must succeed");
        assert_eq!(count, 3);

        let actual: i64 = conn
            .query_row("SELECT COUNT(*) FROM span_events", [], |row| row.get(0))
            .expect("count");
        assert_eq!(actual, 3);
    }

    #[test]
    fn append_span_events_batch_round_trips_exception_attributes() {
        let conn = fresh_conn_with_schema();
        let span = span_with_events(
            vec![2u8; 16],
            vec![2u8; 8],
            1_700_000_000_000_000_000,
            vec![span_event(
                "exception",
                1_700_000_000_000_000_001,
                exception_attrs("ValueError", "bad input", "at foo:42\nat bar:13"),
            )],
        );
        append_span_events_batch(&conn, &wrap_spans(vec![span])).expect("append");

        let (etype, emsg, estack): (Option<String>, Option<String>, Option<String>) = conn
            .query_row(
                "SELECT exception_type, exception_message, exception_stacktrace FROM span_events LIMIT 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .expect("read");
        assert_eq!(etype.as_deref(), Some("ValueError"));
        assert_eq!(emsg.as_deref(), Some("bad input"));
        assert_eq!(estack.as_deref(), Some("at foo:42\nat bar:13"));
    }

    #[test]
    fn append_span_events_batch_round_trips_name() {
        let conn = fresh_conn_with_schema();
        let span = span_with_events(
            vec![3u8; 16],
            vec![3u8; 8],
            1_700_000_000_000_000_000,
            vec![
                span_event("first", 1_700_000_000_000_000_001, vec![]),
                span_event("second", 1_700_000_000_000_000_002, vec![]),
            ],
        );
        append_span_events_batch(&conn, &wrap_spans(vec![span])).expect("append");

        let names: Vec<String> = conn
            .prepare("SELECT name FROM span_events ORDER BY event_index")
            .expect("prepare")
            .query_map([], |r| r.get::<_, String>(0))
            .expect("query")
            .map(|r| r.expect("row"))
            .collect();
        assert_eq!(names, vec!["first".to_string(), "second".to_string()]);
    }

    #[test]
    fn append_span_events_batch_handles_missing_exception_attributes_as_null() {
        let conn = fresh_conn_with_schema();
        let span = span_with_events(
            vec![4u8; 16],
            vec![4u8; 8],
            1_700_000_000_000_000_000,
            vec![span_event(
                "non-exception-event",
                1_700_000_000_000_000_001,
                vec![string_attr("custom.key", "custom-value")],
            )],
        );
        append_span_events_batch(&conn, &wrap_spans(vec![span])).expect("append");

        let nulls: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM span_events \
                 WHERE exception_type IS NULL \
                   AND exception_message IS NULL \
                   AND exception_stacktrace IS NULL",
                [],
                |r| r.get(0),
            )
            .expect("count");
        assert_eq!(nulls, 1);
    }

    #[test]
    fn append_span_events_batch_assigns_monotonic_event_index_per_span() {
        let conn = fresh_conn_with_schema();
        let span = span_with_events(
            vec![5u8; 16],
            vec![5u8; 8],
            1_700_000_000_000_000_000,
            vec![
                span_event("a", 1_700_000_000_000_000_001, vec![]),
                span_event("b", 1_700_000_000_000_000_002, vec![]),
                span_event("c", 1_700_000_000_000_000_003, vec![]),
                span_event("d", 1_700_000_000_000_000_004, vec![]),
            ],
        );
        append_span_events_batch(&conn, &wrap_spans(vec![span])).expect("append");

        let indices: Vec<i32> = conn
            .prepare(
                "SELECT event_index FROM span_events \
                 WHERE trace_id = ? AND span_id = ? ORDER BY event_index",
            )
            .expect("prepare")
            .query_map(duckdb::params![vec![5u8; 16], vec![5u8; 8]], |r| {
                r.get::<_, i32>(0)
            })
            .expect("query")
            .map(|r| r.expect("row"))
            .collect();
        assert_eq!(indices, vec![0, 1, 2, 3]);
    }

    #[test]
    fn append_span_events_batch_skips_when_parent_span_ids_empty() {
        let conn = fresh_conn_with_schema();
        let mut malformed = span_with_events(
            vec![],
            vec![],
            1_700_000_000_000_000_000,
            vec![span_event("ghost", 1_700_000_000_000_000_001, vec![])],
        );
        malformed.start_time_unix_nano = 1_700_000_000_000_000_000;
        let count = append_span_events_batch(&conn, &wrap_spans(vec![malformed])).expect("append");
        assert_eq!(count, 0);

        let actual: i64 = conn
            .query_row("SELECT COUNT(*) FROM span_events", [], |row| row.get(0))
            .expect("count");
        assert_eq!(actual, 0);
    }

    #[test]
    fn append_span_events_batch_empty_input_returns_zero() {
        let conn = fresh_conn_with_schema();
        let count = append_span_events_batch(&conn, &[]).expect("empty append");
        assert_eq!(count, 0);
    }

    #[test]
    fn build_span_events_record_batch_returns_none_when_no_events_in_any_span() {
        // Span exists but events vec is empty; expect Ok(None).
        let span = span_with_ids(vec![6u8; 16], vec![6u8; 8], 1_700_000_000_000_000_000);
        let batch = wrap_spans(vec![span]);
        let result = build_span_events_record_batch(&batch, None).expect("build");
        assert!(result.is_none());
    }

    #[test]
    fn append_span_events_batch_leaves_fingerprint_null_for_non_exception_events() {
        // Per chunk #66 semantics, span events WITHOUT `exception.type` attribute
        // get NULL fingerprint (the BLOB column is reserved for events that
        // represent exceptions). This guards chunk #65's substrate behavior:
        // non-exception events keep the column NULL post-chunk-#66.
        let conn = fresh_conn_with_schema();
        let span = span_with_events(
            vec![7u8; 16],
            vec![7u8; 8],
            1_700_000_000_000_000_000,
            vec![span_event("x", 1_700_000_000_000_000_001, vec![])],
        );
        append_span_events_batch(&conn, &wrap_spans(vec![span])).expect("append");

        let nulls: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM span_events WHERE fingerprint IS NULL",
                [],
                |r| r.get(0),
            )
            .expect("count");
        assert_eq!(nulls, 1);
    }

    // PII negative-canary subscriber pattern per testing.md Session Addition
    // 2026-05-11 (chunk #44 FieldCollector + 2026-05-07 CapturingSubscriber).
    // Buffer crate cannot import from pulse-app (workspace direction); helpers
    // are copied per convention.
    mod pii_canary {
        use std::sync::{Arc, Mutex};
        use tracing::field::{Field, Visit};
        use tracing::span::{Attributes, Id, Record};
        use tracing::{Event, Metadata, Subscriber};

        pub(super) struct CapturingSubscriber {
            pub events: Arc<Mutex<Vec<(String, String)>>>,
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
                    .expect("event lock")
                    .push((metadata.target().to_string(), collector.sink));
            }
            fn enter(&self, _: &Id) {}
            fn exit(&self, _: &Id) {}
        }
    }

    #[test]
    fn append_span_events_batch_does_not_log_exception_canaries() {
        // Canary substrings MUST NOT appear in any captured tracing emission
        // across the decode + append path (per security plan §Logging row 1 +
        // obs-plan §11 Logs vector 1). Verifies the redaction discipline at
        // chunk-authoring time per testing.md 2026-05-11 pattern.
        let canary_message = "secret-canary-EXCEPTION-MESSAGE-PII-12345";
        let canary_stack = "/home/user/.creds/private.key:42\nat handler:13";
        let canary_type = "CanaryValueError";

        let events = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
        let subscriber = pii_canary::CapturingSubscriber {
            events: Arc::clone(&events),
        };

        tracing::subscriber::with_default(subscriber, || {
            let conn = fresh_conn_with_schema();
            let span = span_with_events(
                vec![8u8; 16],
                vec![8u8; 8],
                1_700_000_000_000_000_000,
                vec![span_event(
                    "exception",
                    1_700_000_000_000_000_001,
                    exception_attrs(canary_type, canary_message, canary_stack),
                )],
            );
            append_span_events_batch(&conn, &wrap_spans(vec![span])).expect("append");
        });

        let captured = events.lock().expect("events lock").clone();
        for (target, fields) in &captured {
            for canary in [canary_type, canary_message, canary_stack] {
                assert!(
                    !target.contains(canary),
                    "canary leaked into target: target={target} fields={fields}"
                );
                assert!(
                    !fields.contains(canary),
                    "canary leaked into fields: target={target} fields={fields}"
                );
            }
        }
    }

    // Chunk #66 — fingerprint population + observer invocation tests.
    //
    // Mock `FingerprintObserver` collecting `(fingerprint, service, ts)` tuples
    // for assertion. `Arc<Mutex<Vec<_>>>` capture pattern mirrors the chunk #62
    // CapturingSubscriber discipline; cannot import from pulse-app per
    // workspace direction.
    mod chunk_66 {
        use crate::fingerprint::{ExceptionFingerprint, FingerprintObserver};
        use std::sync::{Arc, Mutex};

        pub(super) struct CapturingFingerprintObserver {
            pub captured: Arc<Mutex<Vec<(ExceptionFingerprint, String, i64)>>>,
        }

        impl CapturingFingerprintObserver {
            pub(super) fn new() -> Self {
                Self {
                    captured: Arc::new(Mutex::new(Vec::new())),
                }
            }
        }

        impl FingerprintObserver for CapturingFingerprintObserver {
            fn on_fingerprint(
                &self,
                fingerprint: ExceptionFingerprint,
                service_name: &str,
                ts_unix_nano: i64,
            ) {
                self.captured.lock().expect("lock").push((
                    fingerprint,
                    service_name.to_string(),
                    ts_unix_nano,
                ));
            }
        }
    }

    #[test]
    fn build_span_events_record_batch_populates_fingerprint_for_exception_events() {
        use arrow::array::Array;
        let span = span_with_events(
            vec![9u8; 16],
            vec![9u8; 8],
            1_700_000_000_000_000_000,
            vec![span_event(
                "exception",
                1_700_000_000_000_000_001,
                exception_attrs(
                    "java.lang.RuntimeException",
                    "boom",
                    "    at com.example.Foo.bar(Foo.java:42)",
                ),
            )],
        );
        let batch = wrap_spans(vec![span]);
        let record_batch = build_span_events_record_batch(&batch, None)
            .expect("build")
            .expect("non-empty batch yields record_batch");

        let fingerprint_col = record_batch
            .column_by_name("fingerprint")
            .expect("fingerprint column present")
            .as_any()
            .downcast_ref::<BinaryArray>()
            .expect("fingerprint column is BinaryArray");
        assert_eq!(fingerprint_col.len(), 1);
        assert!(
            !fingerprint_col.is_null(0),
            "exception event MUST produce non-NULL fingerprint"
        );
        assert_eq!(
            fingerprint_col.value(0).len(),
            16,
            "fingerprint MUST be exactly 16 bytes (truncated blake3)"
        );
    }

    #[test]
    fn build_span_events_record_batch_observer_invoked_per_exception_event() {
        let observer = chunk_66::CapturingFingerprintObserver::new();
        let captured = Arc::clone(&observer.captured);

        let span = span_with_events(
            vec![10u8; 16],
            vec![10u8; 8],
            1_700_000_000_000_000_000,
            vec![
                span_event(
                    "exception",
                    1_700_000_000_000_000_001,
                    exception_attrs("ErrA", "m", "    at Foo.bar"),
                ),
                span_event("non-exception", 1_700_000_000_000_000_002, vec![]),
                span_event(
                    "exception",
                    1_700_000_000_000_000_003,
                    exception_attrs("ErrB", "m", "    at Bar.baz"),
                ),
            ],
        );
        let batch = wrap_spans(vec![span]);

        let _record_batch = build_span_events_record_batch(&batch, Some(&observer))
            .expect("build")
            .expect("record_batch built");

        let captured = captured.lock().expect("lock");
        assert_eq!(
            captured.len(),
            2,
            "observer MUST be invoked exactly once per exception event"
        );
        assert_eq!(captured[0].2, 1_700_000_000_000_000_001);
        assert_eq!(captured[1].2, 1_700_000_000_000_000_003);
        assert_ne!(
            captured[0].0, captured[1].0,
            "different exception types MUST produce different fingerprints"
        );
    }

    #[test]
    fn build_span_events_record_batch_observer_not_invoked_when_no_exception_events() {
        let observer = chunk_66::CapturingFingerprintObserver::new();
        let captured = Arc::clone(&observer.captured);

        let span = span_with_events(
            vec![11u8; 16],
            vec![11u8; 8],
            1_700_000_000_000_000_000,
            vec![span_event("plain-event", 1_700_000_000_000_000_001, vec![])],
        );
        let batch = wrap_spans(vec![span]);

        let _record_batch = build_span_events_record_batch(&batch, Some(&observer))
            .expect("build")
            .expect("record_batch built");

        assert!(
            captured.lock().expect("lock").is_empty(),
            "observer MUST NOT be invoked for non-exception events"
        );
    }
}
