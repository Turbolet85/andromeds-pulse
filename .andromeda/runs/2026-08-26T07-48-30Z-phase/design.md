# design extract

## No domain coverage

This chunk is a Rust-side ingest/consumer stall root-cause plus an obs-log observable (`crates/ingest/src/channel.rs`, `crates/buffer/src/consumer.rs`, `pulse-app/src/heartbeat.rs`, `pulse-app/src/observability.rs` — every anchor in the scope's Surfaces table is backend, and Deliverable B targets "the app's own record", i.e. a log/allowlist leaf, explicitly *not* something a human reads off a rendered heartbeat), so no design-system surface is exercised: nothing renders, so no color/typography/spacing/radius tokens, no motion or reduce-motion override, no iconography, and no webview component pattern applies.
