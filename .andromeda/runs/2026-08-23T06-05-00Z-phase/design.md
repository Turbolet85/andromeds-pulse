# design extract

## No domain coverage

This chunk is entirely backend — PII scrub coverage for four DuckDB ring-buffer columns written in `crates/buffer/src/appender.rs`, with per-class treatment logic in `crates/security/src/scrubber.rs` and consumer maps read from `crates/triage/src/baseline/sql.rs` and `crates/corpus/src/contract.rs` — and renders no surface, so no design-system.md token, typography, motion, iconography, or component-pattern mandate applies (the focus guide places backend/API/IPC and implementation code out of design scope).
