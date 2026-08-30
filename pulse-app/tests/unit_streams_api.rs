// Migrated 2026-08-30 from `pulse-app/src/streams.rs::tests` — that crate
// sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS. The senders handle
// reaches here via the `#[doc(hidden)]` field widening per test-plan §2/§4.

use std::sync::Arc;

use buffer::broadcast as broadcast_module;

use pulse_app::streams::StreamsApiImpl;

#[test]
fn streams_api_impl_constructor_holds_senders() {
    let senders = Arc::new(broadcast_module::create());
    let api = StreamsApiImpl::new(Arc::clone(&senders));
    assert_eq!(api.senders.spans.receiver_count(), 0);
    assert_eq!(api.senders.metrics.receiver_count(), 0);
    assert_eq!(api.senders.logs.receiver_count(), 0);
}

#[test]
fn streams_api_impl_clones_share_senders() {
    let senders = Arc::new(broadcast_module::create());
    let api1 = StreamsApiImpl::new(Arc::clone(&senders));
    let api2 = api1.clone();
    let _r1 = api1.senders.spans.subscribe();
    assert_eq!(api2.senders.spans.receiver_count(), 1);
}
