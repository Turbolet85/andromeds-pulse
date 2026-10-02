# design extract

## No domain coverage

This chunk is entirely backend ingestion identity — the `metrics_points` primary key across three DDL representations (`crates/buffer/src/schema.rs:77`, `:174`, `crates/viz/src/query.rs:488`), the Arrow `Appender` flush-time collision behavior, an OTLP producer example, and `buffer` regression coverage; it renders no surface, so no design-system token, typography, motion, iconography, or component-pattern mandate applies (the label-half decision, whether this-chunk or CARRY, lands as a DB column + `viz` response shape, not as UI — any later Metrics-view rendering of labels would be a separate chunk that does fall in this domain).
