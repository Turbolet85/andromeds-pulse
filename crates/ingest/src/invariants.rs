use crate::contract::Error;
use crate::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use crate::grpc::proto::opentelemetry::proto::logs::v1::ResourceLogs;
use crate::grpc::proto::opentelemetry::proto::metrics::v1::{ResourceMetrics, metric};
use crate::grpc::proto::opentelemetry::proto::trace::v1::ResourceSpans;

pub const TRACE_ID_LEN: usize = 16;
pub const SPAN_ID_LEN: usize = 8;
pub const MAX_ATTRIBUTE_KEY_BYTES: usize = 256;
pub const MAX_ATTRIBUTE_VALUE_BYTES: usize = 4096;
pub const MAX_ATTRIBUTES_PER_SPAN: usize = 128;

pub(crate) fn validate_resource_spans(resource_spans: &[ResourceSpans]) -> Result<(), Error> {
    for rs in resource_spans {
        for ss in &rs.scope_spans {
            for span in &ss.spans {
                if span.trace_id.len() != TRACE_ID_LEN {
                    return Err(Error::InvariantViolation {
                        kind: "trace_id_length",
                        expected: TRACE_ID_LEN,
                        actual: span.trace_id.len(),
                    });
                }
                if span.span_id.len() != SPAN_ID_LEN {
                    return Err(Error::InvariantViolation {
                        kind: "span_id_length",
                        expected: SPAN_ID_LEN,
                        actual: span.span_id.len(),
                    });
                }
                if !span.parent_span_id.is_empty() && span.parent_span_id.len() != SPAN_ID_LEN {
                    return Err(Error::InvariantViolation {
                        kind: "parent_span_id_length",
                        expected: SPAN_ID_LEN,
                        actual: span.parent_span_id.len(),
                    });
                }
                if span.attributes.len() > MAX_ATTRIBUTES_PER_SPAN {
                    return Err(Error::InvariantViolation {
                        kind: "attributes_count",
                        expected: MAX_ATTRIBUTES_PER_SPAN,
                        actual: span.attributes.len(),
                    });
                }
                validate_attributes(&span.attributes)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_resource_metrics(resource_metrics: &[ResourceMetrics]) -> Result<(), Error> {
    for rm in resource_metrics {
        for sm in &rm.scope_metrics {
            for m in &sm.metrics {
                if let Some(data) = &m.data {
                    validate_metric_data(data)?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_resource_logs(resource_logs: &[ResourceLogs]) -> Result<(), Error> {
    for rl in resource_logs {
        for sl in &rl.scope_logs {
            for log in &sl.log_records {
                if !log.trace_id.is_empty() && log.trace_id.len() != TRACE_ID_LEN {
                    return Err(Error::InvariantViolation {
                        kind: "trace_id_length",
                        expected: TRACE_ID_LEN,
                        actual: log.trace_id.len(),
                    });
                }
                if !log.span_id.is_empty() && log.span_id.len() != SPAN_ID_LEN {
                    return Err(Error::InvariantViolation {
                        kind: "span_id_length",
                        expected: SPAN_ID_LEN,
                        actual: log.span_id.len(),
                    });
                }
                if log.attributes.len() > MAX_ATTRIBUTES_PER_SPAN {
                    return Err(Error::InvariantViolation {
                        kind: "attributes_count",
                        expected: MAX_ATTRIBUTES_PER_SPAN,
                        actual: log.attributes.len(),
                    });
                }
                validate_attributes(&log.attributes)?;
            }
        }
    }
    Ok(())
}

fn validate_metric_data(data: &metric::Data) -> Result<(), Error> {
    match data {
        metric::Data::Gauge(g) => validate_number_points(&g.data_points),
        metric::Data::Sum(s) => validate_number_points(&s.data_points),
        metric::Data::Histogram(h) => {
            for p in &h.data_points {
                if p.attributes.len() > MAX_ATTRIBUTES_PER_SPAN {
                    return Err(Error::InvariantViolation {
                        kind: "attributes_count",
                        expected: MAX_ATTRIBUTES_PER_SPAN,
                        actual: p.attributes.len(),
                    });
                }
                validate_attributes(&p.attributes)?;
            }
            Ok(())
        }
        metric::Data::ExponentialHistogram(eh) => {
            for p in &eh.data_points {
                if p.attributes.len() > MAX_ATTRIBUTES_PER_SPAN {
                    return Err(Error::InvariantViolation {
                        kind: "attributes_count",
                        expected: MAX_ATTRIBUTES_PER_SPAN,
                        actual: p.attributes.len(),
                    });
                }
                validate_attributes(&p.attributes)?;
            }
            Ok(())
        }
        metric::Data::Summary(s) => {
            for p in &s.data_points {
                if p.attributes.len() > MAX_ATTRIBUTES_PER_SPAN {
                    return Err(Error::InvariantViolation {
                        kind: "attributes_count",
                        expected: MAX_ATTRIBUTES_PER_SPAN,
                        actual: p.attributes.len(),
                    });
                }
                validate_attributes(&p.attributes)?;
            }
            Ok(())
        }
    }
}

fn validate_number_points(
    points: &[crate::grpc::proto::opentelemetry::proto::metrics::v1::NumberDataPoint],
) -> Result<(), Error> {
    for p in points {
        if p.attributes.len() > MAX_ATTRIBUTES_PER_SPAN {
            return Err(Error::InvariantViolation {
                kind: "attributes_count",
                expected: MAX_ATTRIBUTES_PER_SPAN,
                actual: p.attributes.len(),
            });
        }
        validate_attributes(&p.attributes)?;
    }
    Ok(())
}

fn validate_attributes(attributes: &[KeyValue]) -> Result<(), Error> {
    for kv in attributes {
        if kv.key.len() > MAX_ATTRIBUTE_KEY_BYTES {
            return Err(Error::InvariantViolation {
                kind: "attribute_key_length",
                expected: MAX_ATTRIBUTE_KEY_BYTES,
                actual: kv.key.len(),
            });
        }
        if let Some(v) = &kv.value {
            let value_size = any_value_size(v);
            if value_size > MAX_ATTRIBUTE_VALUE_BYTES {
                return Err(Error::InvariantViolation {
                    kind: "attribute_value_length",
                    expected: MAX_ATTRIBUTE_VALUE_BYTES,
                    actual: value_size,
                });
            }
        }
    }
    Ok(())
}

fn any_value_size(v: &AnyValue) -> usize {
    match &v.value {
        Some(any_value::Value::StringValue(s)) => s.len(),
        Some(any_value::Value::BytesValue(b)) => b.len(),
        Some(any_value::Value::ArrayValue(a)) => a.values.iter().map(any_value_size).sum(),
        Some(any_value::Value::KvlistValue(kv)) => kv
            .values
            .iter()
            .map(|e| e.key.len() + e.value.as_ref().map(any_value_size).unwrap_or(0))
            .sum(),
        Some(any_value::Value::BoolValue(_))
        | Some(any_value::Value::IntValue(_))
        | Some(any_value::Value::DoubleValue(_))
        | None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::proto::opentelemetry::proto::common::v1::AnyValue;
    use crate::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};

    fn span_with_ids(trace_id_len: usize, span_id_len: usize) -> Span {
        Span {
            trace_id: vec![0u8; trace_id_len],
            span_id: vec![0u8; span_id_len],
            ..Default::default()
        }
    }

    fn wrap_span(span: Span) -> Vec<ResourceSpans> {
        vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![span],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }]
    }

    #[test]
    fn validate_spans_accepts_canonical_ids() {
        let span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        assert!(validate_resource_spans(&wrap_span(span)).is_ok());
    }

    #[test]
    fn validate_spans_rejects_short_trace_id() {
        let span = span_with_ids(8, SPAN_ID_LEN);
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "trace_id_length",
                expected: 16,
                actual: 8,
            }
        ));
    }

    #[test]
    fn validate_spans_rejects_long_trace_id() {
        let span = span_with_ids(24, SPAN_ID_LEN);
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "trace_id_length",
                ..
            }
        ));
    }

    #[test]
    fn validate_spans_rejects_short_span_id() {
        let span = span_with_ids(TRACE_ID_LEN, 4);
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "span_id_length",
                expected: 8,
                actual: 4,
            }
        ));
    }

    #[test]
    fn validate_spans_rejects_zero_length_trace_id() {
        let span = span_with_ids(0, SPAN_ID_LEN);
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "trace_id_length",
                ..
            }
        ));
    }

    #[test]
    fn validate_spans_accepts_empty_parent_span_id() {
        let span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        assert!(validate_resource_spans(&wrap_span(span)).is_ok());
    }

    #[test]
    fn validate_spans_rejects_wrong_parent_span_id_length() {
        let mut span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        span.parent_span_id = vec![0u8; 6];
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "parent_span_id_length",
                ..
            }
        ));
    }

    #[test]
    fn validate_spans_rejects_attribute_count_over_limit() {
        let mut span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        span.attributes = (0..(MAX_ATTRIBUTES_PER_SPAN + 1))
            .map(|i| KeyValue {
                key: format!("k{i}"),
                value: None,
            })
            .collect();
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "attributes_count",
                ..
            }
        ));
    }

    #[test]
    fn validate_spans_rejects_attribute_key_over_limit() {
        let mut span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        span.attributes = vec![KeyValue {
            key: "x".repeat(MAX_ATTRIBUTE_KEY_BYTES + 1),
            value: None,
        }];
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "attribute_key_length",
                ..
            }
        ));
    }

    #[test]
    fn validate_spans_rejects_attribute_value_string_over_limit() {
        let mut span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        span.attributes = vec![KeyValue {
            key: "k".to_string(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(
                    "v".repeat(MAX_ATTRIBUTE_VALUE_BYTES + 1),
                )),
            }),
        }];
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "attribute_value_length",
                ..
            }
        ));
    }

    #[test]
    fn validate_spans_rejects_attribute_value_bytes_over_limit() {
        let mut span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        span.attributes = vec![KeyValue {
            key: "k".to_string(),
            value: Some(AnyValue {
                value: Some(any_value::Value::BytesValue(vec![
                    0u8;
                    MAX_ATTRIBUTE_VALUE_BYTES
                        + 1
                ])),
            }),
        }];
        let err = validate_resource_spans(&wrap_span(span)).unwrap_err();
        assert!(matches!(
            err,
            Error::InvariantViolation {
                kind: "attribute_value_length",
                ..
            }
        ));
    }

    #[test]
    fn validate_spans_accepts_attribute_value_within_limit() {
        let mut span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        span.attributes = vec![KeyValue {
            key: "k".to_string(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue("ok".to_string())),
            }),
        }];
        assert!(validate_resource_spans(&wrap_span(span)).is_ok());
    }

    #[test]
    fn validate_spans_accepts_numeric_attribute_value() {
        let mut span = span_with_ids(TRACE_ID_LEN, SPAN_ID_LEN);
        span.attributes = vec![KeyValue {
            key: "n".to_string(),
            value: Some(AnyValue {
                value: Some(any_value::Value::IntValue(42)),
            }),
        }];
        assert!(validate_resource_spans(&wrap_span(span)).is_ok());
    }

    #[test]
    fn validate_resource_spans_empty_input_is_ok() {
        assert!(validate_resource_spans(&[]).is_ok());
    }

    #[test]
    fn validate_resource_metrics_empty_input_is_ok() {
        assert!(validate_resource_metrics(&[]).is_ok());
    }

    #[test]
    fn validate_resource_logs_empty_input_is_ok() {
        assert!(validate_resource_logs(&[]).is_ok());
    }
}
