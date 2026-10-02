# design extract

## No domain coverage

This chunk is entirely backend Rust (`crates/security/src/scrubber.rs` regex catalog, the `crates/buffer` stored-field path, e2e/nextest corpora) plus a report record and an obs-owned redaction-counter decision (CARRY #10) — it renders no surface and touches no design token, typography, motion, iconography, or component pattern; the only UI mention of scrubbing (`pulse-app/ui/src/dashboard/routes/settings/ExportForTraining.tsx`, static copy "PII is scrubbed") lies outside the chunk's stated boundaries.
