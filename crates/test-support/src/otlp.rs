//! Factories for the OTLP telemetry a test sends or hands to the buffer:
//! spans with their span events, metric points inside a named gauge, and log
//! records, assembled under one resource.
//!
//! A test sets what it reasons about and the rest takes a default. Nothing
//! here reads a clock or draws a random value, so two builds from the same
//! calls are equal. An unset event time is [`DEFAULT_TIME_UNIX_NANO`] and an
//! unset service is [`DEFAULT_SERVICE`]. A test that feeds a running engine
//! states the instant itself: the retention sweep deletes a record older than
//! its window, and the fixed default lies years in the past.
//!
//! Spans built under one resource take distinct span ids by their position
//! unless the test sets them; a test that wants a replayed identity sets it.

use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::ExportLogsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{
    Gauge, Metric, NumberDataPoint, ResourceMetrics, ScopeMetrics, metric, number_data_point,
};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span, span};

/// The event time of every record a test gives no instant.
pub const DEFAULT_TIME_UNIX_NANO: u64 = 1_700_000_000_000_000_000;
/// The `service.name` of every resource a test gives no service.
pub const DEFAULT_SERVICE: &str = "test-service";
/// How long after its start a built span ends.
pub const SPAN_DURATION_NANOS: u64 = 1_000_000;

const DEFAULT_TRACE_ID: [u8; 16] = [0x1d; 16];
const SEVERITY_NUMBER_INFO: i32 = 9;

fn string_attribute(key: &str, value: &str) -> KeyValue {
    KeyValue {
        key: key.to_string(),
        value: Some(AnyValue {
            value: Some(any_value::Value::StringValue(value.to_string())),
        }),
    }
}

/// A span event; see [`span_event`].
#[derive(Debug, Clone)]
pub struct SpanEventBuilder {
    name: String,
    time_unix_nano: u64,
    attributes: Vec<KeyValue>,
}

pub fn span_event() -> SpanEventBuilder {
    SpanEventBuilder {
        name: "test-event".to_string(),
        time_unix_nano: DEFAULT_TIME_UNIX_NANO,
        attributes: Vec::new(),
    }
}

impl SpanEventBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// The event time, in nanoseconds since the Unix epoch.
    pub fn at(mut self, unix_nano: u64) -> Self {
        self.time_unix_nano = unix_nano;
        self
    }

    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.push(string_attribute(key, value));
        self
    }

    fn build(self) -> span::Event {
        span::Event {
            time_unix_nano: self.time_unix_nano,
            name: self.name,
            attributes: self.attributes,
            dropped_attributes_count: 0,
        }
    }
}

/// A span; see [`span`].
#[derive(Debug, Clone)]
pub struct SpanBuilder {
    trace_id: [u8; 16],
    span_id: Option<[u8; 8]>,
    name: String,
    start_time_unix_nano: u64,
    attributes: Vec<KeyValue>,
    events: Vec<SpanEventBuilder>,
}

pub fn span() -> SpanBuilder {
    SpanBuilder {
        trace_id: DEFAULT_TRACE_ID,
        span_id: None,
        name: "test-span".to_string(),
        start_time_unix_nano: DEFAULT_TIME_UNIX_NANO,
        attributes: Vec::new(),
        events: Vec::new(),
    }
}

impl SpanBuilder {
    pub fn trace_id(mut self, trace_id: [u8; 16]) -> Self {
        self.trace_id = trace_id;
        self
    }

    pub fn span_id(mut self, span_id: [u8; 8]) -> Self {
        self.span_id = Some(span_id);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// The span's start, in nanoseconds since the Unix epoch. It ends
    /// [`SPAN_DURATION_NANOS`] later.
    pub fn at(mut self, unix_nano: u64) -> Self {
        self.start_time_unix_nano = unix_nano;
        self
    }

    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.push(string_attribute(key, value));
        self
    }

    pub fn event(mut self, event: SpanEventBuilder) -> Self {
        self.events.push(event);
        self
    }

    pub fn events(mut self, events: impl IntoIterator<Item = SpanEventBuilder>) -> Self {
        self.events.extend(events);
        self
    }

    fn build(self, position: usize) -> Span {
        let ordinal = u64::try_from(position).expect("a span position fits u64") + 1;
        Span {
            trace_id: self.trace_id.to_vec(),
            span_id: self.span_id.unwrap_or(ordinal.to_be_bytes()).to_vec(),
            name: self.name,
            start_time_unix_nano: self.start_time_unix_nano,
            end_time_unix_nano: self.start_time_unix_nano + SPAN_DURATION_NANOS,
            attributes: self.attributes,
            events: self
                .events
                .into_iter()
                .map(SpanEventBuilder::build)
                .collect(),
            ..Default::default()
        }
    }
}

/// A number point of a gauge; see [`metric_point`].
#[derive(Debug, Clone)]
pub struct MetricPointBuilder {
    time_unix_nano: u64,
    value: i64,
    attributes: Vec<KeyValue>,
}

pub fn metric_point() -> MetricPointBuilder {
    MetricPointBuilder {
        time_unix_nano: DEFAULT_TIME_UNIX_NANO,
        value: 1,
        attributes: Vec::new(),
    }
}

impl MetricPointBuilder {
    /// The event time, in nanoseconds since the Unix epoch.
    pub fn at(mut self, unix_nano: u64) -> Self {
        self.time_unix_nano = unix_nano;
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = value;
        self
    }

    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.push(string_attribute(key, value));
        self
    }

    fn build(self) -> NumberDataPoint {
        NumberDataPoint {
            attributes: self.attributes,
            start_time_unix_nano: 0,
            time_unix_nano: self.time_unix_nano,
            exemplars: Vec::new(),
            flags: 0,
            value: Some(number_data_point::Value::AsInt(self.value)),
        }
    }
}

/// A named gauge metric holding metric points; see [`gauge`].
#[derive(Debug, Clone)]
pub struct GaugeBuilder {
    name: String,
    points: Vec<MetricPointBuilder>,
}

pub fn gauge() -> GaugeBuilder {
    GaugeBuilder {
        name: "test.gauge".to_string(),
        points: Vec::new(),
    }
}

impl GaugeBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn point(mut self, point: MetricPointBuilder) -> Self {
        self.points.push(point);
        self
    }

    pub fn points(mut self, points: impl IntoIterator<Item = MetricPointBuilder>) -> Self {
        self.points.extend(points);
        self
    }

    fn build(self) -> Metric {
        Metric {
            name: self.name,
            description: String::new(),
            unit: String::new(),
            metadata: Vec::new(),
            data: Some(metric::Data::Gauge(Gauge {
                data_points: self
                    .points
                    .into_iter()
                    .map(MetricPointBuilder::build)
                    .collect(),
            })),
        }
    }
}

/// A log record; see [`log_record`].
#[derive(Debug, Clone)]
pub struct LogRecordBuilder {
    time_unix_nano: u64,
    severity_number: i32,
    severity_text: String,
    body: String,
    attributes: Vec<KeyValue>,
}

pub fn log_record() -> LogRecordBuilder {
    LogRecordBuilder {
        time_unix_nano: DEFAULT_TIME_UNIX_NANO,
        severity_number: SEVERITY_NUMBER_INFO,
        severity_text: "INFO".to_string(),
        body: "test log record".to_string(),
        attributes: Vec::new(),
    }
}

impl LogRecordBuilder {
    /// The event time, in nanoseconds since the Unix epoch.
    pub fn at(mut self, unix_nano: u64) -> Self {
        self.time_unix_nano = unix_nano;
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn severity(mut self, number: i32, text: &str) -> Self {
        self.severity_number = number;
        self.severity_text = text.to_string();
        self
    }

    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.push(string_attribute(key, value));
        self
    }

    fn build(self) -> LogRecord {
        LogRecord {
            time_unix_nano: self.time_unix_nano,
            severity_number: self.severity_number,
            severity_text: self.severity_text,
            body: Some(AnyValue {
                value: Some(any_value::Value::StringValue(self.body)),
            }),
            attributes: self.attributes,
            ..Default::default()
        }
    }
}

/// One resource and the telemetry sent under it; see [`resource`].
#[derive(Debug, Clone)]
pub struct ResourceBuilder {
    service: String,
    spans: Vec<SpanBuilder>,
    gauges: Vec<GaugeBuilder>,
    log_records: Vec<LogRecordBuilder>,
}

pub fn resource() -> ResourceBuilder {
    ResourceBuilder {
        service: DEFAULT_SERVICE.to_string(),
        spans: Vec::new(),
        gauges: Vec::new(),
        log_records: Vec::new(),
    }
}

impl ResourceBuilder {
    /// The resource's `service.name`.
    pub fn service(mut self, service: &str) -> Self {
        self.service = service.to_string();
        self
    }

    pub fn span(mut self, span: SpanBuilder) -> Self {
        self.spans.push(span);
        self
    }

    pub fn spans(mut self, spans: impl IntoIterator<Item = SpanBuilder>) -> Self {
        self.spans.extend(spans);
        self
    }

    pub fn gauge(mut self, gauge: GaugeBuilder) -> Self {
        self.gauges.push(gauge);
        self
    }

    pub fn gauges(mut self, gauges: impl IntoIterator<Item = GaugeBuilder>) -> Self {
        self.gauges.extend(gauges);
        self
    }

    pub fn log_record(mut self, log_record: LogRecordBuilder) -> Self {
        self.log_records.push(log_record);
        self
    }

    pub fn log_records(mut self, log_records: impl IntoIterator<Item = LogRecordBuilder>) -> Self {
        self.log_records.extend(log_records);
        self
    }

    fn build_resource(&self) -> Resource {
        Resource {
            attributes: vec![string_attribute("service.name", &self.service)],
            dropped_attributes_count: 0,
        }
    }

    /// The resource-level form of this resource's spans, as the buffer takes
    /// them.
    pub fn resource_spans(&self) -> Vec<ResourceSpans> {
        vec![ResourceSpans {
            resource: Some(self.build_resource()),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: self
                    .spans
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(position, span)| span.build(position))
                    .collect(),
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }]
    }

    /// The resource-level form of this resource's gauges.
    pub fn resource_metrics(&self) -> Vec<ResourceMetrics> {
        vec![ResourceMetrics {
            resource: Some(self.build_resource()),
            scope_metrics: vec![ScopeMetrics {
                scope: None,
                metrics: self
                    .gauges
                    .iter()
                    .cloned()
                    .map(GaugeBuilder::build)
                    .collect(),
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }]
    }

    /// The resource-level form of this resource's log records.
    pub fn resource_logs(&self) -> Vec<ResourceLogs> {
        vec![ResourceLogs {
            resource: Some(self.build_resource()),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: self
                    .log_records
                    .iter()
                    .cloned()
                    .map(LogRecordBuilder::build)
                    .collect(),
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }]
    }

    /// The export request a test sends to a trace receiver.
    pub fn trace_request(&self) -> ExportTraceServiceRequest {
        ExportTraceServiceRequest {
            resource_spans: self.resource_spans(),
        }
    }

    /// The export request a test sends to a logs receiver.
    pub fn logs_request(&self) -> ExportLogsServiceRequest {
        ExportLogsServiceRequest {
            resource_logs: self.resource_logs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSTANT: u64 = 1_800_000_000_123_456_789;

    fn string_value(attribute: &KeyValue) -> (&str, &str) {
        let Some(AnyValue {
            value: Some(any_value::Value::StringValue(value)),
        }) = &attribute.value
        else {
            panic!("a factory attribute holds a string value");
        };
        (attribute.key.as_str(), value.as_str())
    }

    fn service_of(resource: &Option<Resource>) -> &str {
        let resource = resource
            .as_ref()
            .expect("a built form carries its resource");
        assert_eq!(resource.attributes.len(), 1);
        let (key, value) = string_value(&resource.attributes[0]);
        assert_eq!(key, "service.name");
        value
    }

    fn built_spans(resource: &ResourceBuilder) -> Vec<Span> {
        resource
            .resource_spans()
            .remove(0)
            .scope_spans
            .remove(0)
            .spans
    }

    fn built_points(resource: &ResourceBuilder) -> Vec<NumberDataPoint> {
        let metric = resource
            .resource_metrics()
            .remove(0)
            .scope_metrics
            .remove(0)
            .metrics
            .remove(0);
        let Some(metric::Data::Gauge(gauge)) = metric.data else {
            panic!("a factory metric is a gauge");
        };
        gauge.data_points
    }

    fn built_log_records(resource: &ResourceBuilder) -> Vec<LogRecord> {
        resource
            .resource_logs()
            .remove(0)
            .scope_logs
            .remove(0)
            .log_records
    }

    fn one_of_each() -> ResourceBuilder {
        resource()
            .span(span().event(span_event()))
            .span(span())
            .gauge(gauge().point(metric_point()))
            .log_record(log_record())
    }

    #[test]
    fn two_builds_from_the_same_calls_are_equal() {
        let (first, second) = (one_of_each(), one_of_each());
        assert_eq!(first.resource_spans(), second.resource_spans());
        assert_eq!(first.resource_metrics(), second.resource_metrics());
        assert_eq!(first.resource_logs(), second.resource_logs());
        assert_eq!(first.trace_request(), second.trace_request());
        assert_eq!(first.logs_request(), second.logs_request());
        assert_eq!(first.resource_spans(), first.resource_spans());
    }

    #[test]
    fn an_unset_event_time_is_the_one_fixed_instant() {
        let built = one_of_each();
        for span in built_spans(&built) {
            assert_eq!(span.start_time_unix_nano, DEFAULT_TIME_UNIX_NANO);
            assert_eq!(
                span.end_time_unix_nano,
                DEFAULT_TIME_UNIX_NANO + SPAN_DURATION_NANOS
            );
            for event in span.events {
                assert_eq!(event.time_unix_nano, DEFAULT_TIME_UNIX_NANO);
            }
        }
        assert_eq!(
            built_points(&built)[0].time_unix_nano,
            DEFAULT_TIME_UNIX_NANO
        );
        assert_eq!(
            built_log_records(&built)[0].time_unix_nano,
            DEFAULT_TIME_UNIX_NANO
        );
    }

    #[test]
    fn an_unset_service_is_the_one_fixed_name_on_every_form() {
        let built = one_of_each();
        assert_eq!(
            service_of(&built.resource_spans()[0].resource),
            DEFAULT_SERVICE
        );
        assert_eq!(
            service_of(&built.resource_metrics()[0].resource),
            DEFAULT_SERVICE
        );
        assert_eq!(
            service_of(&built.resource_logs()[0].resource),
            DEFAULT_SERVICE
        );
    }

    #[test]
    fn a_set_service_is_read_back_on_every_form() {
        let built = one_of_each().service("checkout");
        assert_eq!(service_of(&built.resource_spans()[0].resource), "checkout");
        assert_eq!(
            service_of(&built.resource_metrics()[0].resource),
            "checkout"
        );
        assert_eq!(service_of(&built.resource_logs()[0].resource), "checkout");
        assert_eq!(
            service_of(&built.trace_request().resource_spans[0].resource),
            "checkout"
        );
        assert_eq!(
            service_of(&built.logs_request().resource_logs[0].resource),
            "checkout"
        );
    }

    #[test]
    fn ids_have_their_wire_lengths_and_unset_span_ids_differ_by_position() {
        let spans = built_spans(&resource().spans((0..300).map(|_| span())));
        assert_eq!(spans.len(), 300);
        let mut span_ids: Vec<&[u8]> = Vec::new();
        for span in &spans {
            assert_eq!(span.trace_id.len(), 16);
            assert_eq!(span.span_id.len(), 8);
            assert!(span.trace_id.iter().any(|byte| *byte != 0));
            assert!(span.span_id.iter().any(|byte| *byte != 0));
            span_ids.push(&span.span_id);
        }
        span_ids.sort_unstable();
        span_ids.dedup();
        assert_eq!(span_ids.len(), 300, "every unset span id is its own");
    }

    #[test]
    fn a_set_id_is_kept_and_an_unset_neighbour_keeps_its_position() {
        let spans = built_spans(
            &resource()
                .span(span().trace_id([7; 16]).span_id([9; 8]))
                .span(span()),
        );
        assert_eq!(spans[0].trace_id, [7; 16]);
        assert_eq!(spans[0].span_id, [9; 8]);
        assert_eq!(spans[1].span_id, 2u64.to_be_bytes());
    }

    #[test]
    fn what_a_test_sets_on_a_span_and_its_event_is_read_back() {
        let spans = built_spans(
            &resource().span(
                span()
                    .name("charge")
                    .at(INSTANT)
                    .attribute("http.route", "/pay")
                    .attribute("peer", "ledger")
                    .event(
                        span_event()
                            .name("exception")
                            .at(INSTANT + 5)
                            .attribute("exception.type", "Declined"),
                    ),
            ),
        );
        let span = &spans[0];
        assert_eq!(span.name, "charge");
        assert_eq!(span.start_time_unix_nano, INSTANT);
        assert_eq!(span.end_time_unix_nano, INSTANT + SPAN_DURATION_NANOS);
        let attributes: Vec<_> = span.attributes.iter().map(string_value).collect();
        assert_eq!(attributes, [("http.route", "/pay"), ("peer", "ledger")]);

        assert_eq!(span.events.len(), 1);
        let event = &span.events[0];
        assert_eq!(event.name, "exception");
        assert_eq!(event.time_unix_nano, INSTANT + 5);
        let attributes: Vec<_> = event.attributes.iter().map(string_value).collect();
        assert_eq!(attributes, [("exception.type", "Declined")]);
    }

    #[test]
    fn what_a_test_sets_on_a_metric_point_is_read_back() {
        let built = resource().gauge(
            gauge().name("queue.depth").points([
                metric_point()
                    .at(INSTANT)
                    .value(42)
                    .attribute("queue", "orders"),
                metric_point(),
            ]),
        );
        let metric = &built.resource_metrics()[0].scope_metrics[0].metrics[0];
        assert_eq!(metric.name, "queue.depth");

        let points = built_points(&built);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].time_unix_nano, INSTANT);
        assert_eq!(points[0].value, Some(number_data_point::Value::AsInt(42)));
        let attributes: Vec<_> = points[0].attributes.iter().map(string_value).collect();
        assert_eq!(attributes, [("queue", "orders")]);
        assert_eq!(points[1].value, Some(number_data_point::Value::AsInt(1)));
        assert!(points[1].attributes.is_empty());
    }

    #[test]
    fn what_a_test_sets_on_a_log_record_is_read_back() {
        let records = built_log_records(
            &resource().log_records([
                log_record()
                    .at(INSTANT)
                    .body("payment declined")
                    .severity(17, "ERROR")
                    .attribute("order", "o-1"),
                log_record(),
            ]),
        );
        assert_eq!(records.len(), 2);
        let record = &records[0];
        assert_eq!(record.time_unix_nano, INSTANT);
        assert_eq!(record.severity_number, 17);
        assert_eq!(record.severity_text, "ERROR");
        assert_eq!(
            record.body,
            Some(AnyValue {
                value: Some(any_value::Value::StringValue("payment declined".into())),
            })
        );
        let attributes: Vec<_> = record.attributes.iter().map(string_value).collect();
        assert_eq!(attributes, [("order", "o-1")]);

        assert_eq!(records[1].severity_number, SEVERITY_NUMBER_INFO);
        assert_eq!(records[1].severity_text, "INFO");
        assert!(records[1].attributes.is_empty());
    }

    #[test]
    fn a_resource_with_nothing_added_builds_empty_forms_under_its_service() {
        let built = resource();
        assert!(built.resource_spans()[0].scope_spans[0].spans.is_empty());
        assert!(
            built.resource_metrics()[0].scope_metrics[0]
                .metrics
                .is_empty()
        );
        assert!(
            built.resource_logs()[0].scope_logs[0]
                .log_records
                .is_empty()
        );
        assert_eq!(
            service_of(&built.resource_spans()[0].resource),
            DEFAULT_SERVICE
        );
    }
}
