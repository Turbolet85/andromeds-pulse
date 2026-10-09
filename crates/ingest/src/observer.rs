//! Span observer trait for chunk #62 — attention cue emitter baseline
//! tracker integration.
//!
//! Defines the abstraction that lets buffer's consumer hand off decoded
//! spans to the chunk #61 streaming baseline tracker (`triage::BaselineState`)
//! without buffer depending on triage. Trait lives in ingest because spans
//! originate from ingest decode; implementation lives at pulse-app boundary
//! (see `pulse-app/src/baseline_observer.rs`) which already depends on both
//! ingest + triage.
//!
//! Mirrors the trait-in-lower-crate + impl-in-pulse-app pattern established
//! by chunk #59 `connection::ReceiverBindStatus` per arch §Cross-cutting
//! Patterns Module dependency direction.

/// Hook invoked once per decoded OTLP span for streaming baseline observation
/// (chunk #62 P-021 algorithmic attention cue substrate). Implementations
/// must be `Send + Sync` so the buffer consumer task can hold an
/// `Arc<dyn SpanObserver>`.
///
/// The `service_name` + `operation_name` arguments are derived from the
/// span's parent ResourceSpans + Span message; `status_code` is the OTLP
/// trace v1 spec value (0 Unset / 1 Ok / 2 Error); `latency_ms` is the
/// span's `end_time_unix_nano - start_time_unix_nano` divided by 1_000_000;
/// `now_nanos` is the observation wall-clock (used by EWMA tracker decay).
pub trait SpanObserver: Send + Sync {
    fn observe_span(
        &self,
        service_name: &str,
        operation_name: &str,
        status_code: u8,
        latency_ms: u64,
        now_nanos: i64,
    );
}

/// No-op implementation used as default in tests and as a fallback when
/// no baseline tracker is wired in (e.g., disabled-by-feature paths or
/// boot-time fallback if `BaselineState` init fails).
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopSpanObserver;

impl SpanObserver for NoopSpanObserver {
    fn observe_span(
        &self,
        _service_name: &str,
        _operation_name: &str,
        _status_code: u8,
        _latency_ms: u64,
        _now_nanos: i64,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug, Default)]
    struct CountingObserver {
        count: AtomicUsize,
    }

    impl SpanObserver for CountingObserver {
        fn observe_span(
            &self,
            _service_name: &str,
            _operation_name: &str,
            _status_code: u8,
            _latency_ms: u64,
            _now_nanos: i64,
        ) {
            self.count.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn noop_observer_safe_to_call() {
        let observer = NoopSpanObserver;
        observer.observe_span("svc", "op", 0, 100, 1_000_000);
    }

    #[test]
    fn observer_trait_object_callable() {
        let observer: Arc<dyn SpanObserver> = Arc::new(NoopSpanObserver);
        observer.observe_span("svc", "op", 0, 100, 1_000_000);
    }

    #[test]
    fn counting_observer_increments_on_each_call() {
        let observer = CountingObserver::default();
        observer.observe_span("svc", "op-a", 0, 100, 1_000);
        observer.observe_span("svc", "op-b", 2, 200, 2_000);
        observer.observe_span("svc", "op-c", 0, 50, 3_000);
        assert_eq!(observer.count.load(Ordering::Relaxed), 3);
    }
}
