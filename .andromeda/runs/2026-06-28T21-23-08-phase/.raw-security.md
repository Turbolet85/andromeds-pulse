# security extract

## Relevance — partial
This chunk touches persistent corpus (AES-256-GCM encryption), self-observation (cardinality discipline), and internal triage processing; does not cross new external trust boundaries (OTLP input validation pre-exists per §Threat Model); corpus persistence layer is load-bearing.

## Constraints
1. Corpus cell-level AES-256-GCM encryption maintained at all persistence boundaries for digest assembly / coalescing / queue state (per security-plan §Data Protection §At rest — corpus.db row).
2. Self-observation follows cardinality discipline with no per-service identifiers per chunk scope requirement + security-plan §Logging & Monitoring; aggregate-only counters (`pipeline.l3.*` family) for coalesce count, queue depth, heartbeat-drain ticks.
3. OTLP attribute values NEVER logged across coalescing / elastic queue / drain path (per security-plan §Logging & Monitoring NEVER-log list).
4. Corpus encryption key sourced from OS keychain primary or passphrase-fallback-with-warning per security-plan §Secret Management (chunk #73 P-049 decision); all writes invoke `keyring` crate via `crates/corpus/src/keychain.rs::OsKeychainBackend`.
5. No plaintext digest state survives to disk; all corpus writes opaque without decryption key (per security-plan §Data Protection §At rest — "table + column names plaintext, BLOB cell payloads opaque").

## Patterns to follow
1. Reuse existing `pipeline.l3.*` counter/gauge family for new observability (coalesce count, elastic queue depth) per scope; no new cardinality dimensions.
2. Apply uniform-scrubber coverage at all corpus persistence boundaries (digest assembly write, queue drain write, coalesced digest write) per chunk #72 framing + security-plan §Anti-Patterns §Logging.
3. Wire corpus key access at heartbeat-drain / elastic-queue-flush sites with `#[tracing::instrument(skip(key))]` discipline per §Secret Management runtime key handling.

## Anti-patterns to avoid
1. NEVER log full OTLP attribute values / telemetry payloads / hard-signal details in digest observability (per §Logging & Monitoring NEVER-log).
2. NEVER add per-service or per-fingerprint cardinality to observability (aggregate-only discipline per scope + §Logging & Monitoring).
3. NEVER bypass corpus AES-256-GCM encryption at coalesce/queue sites — all writes through encrypted persistence layer.

## Contract bindings
- **Corpus encryption ↔ Secret Management:** corpus key custody via `keyring` crate OS keychain primary + passphrase fallback (security-plan §Secret Management Storage / runtime); all writes invoke key via backend.
- **Self-observation ↔ obs plan:** cardinality discipline (no per-service identifiers); counter/gauge family reuse (`pipeline.l3.*` existing family per scope).
- **PII scrubbing ↔ chunk #72 uniform-scrubber coverage:** digest assembly + queue drain + corpus write boundaries all apply `security::scrubber::scrub_attribute` per §Data Protection + §Anti-Patterns §Logging.
- **Error boundaries at corpus writes:** sanitized errors per §Error Handling; internal logs carry full context with corpus key skipped.

## Acceptance criteria contributions
1. (security) Corpus cell-level AES-256-GCM encryption maintained — all digest assembly / coalesce / queue / drain writes verify encryption envelope intact (grep `crates/corpus/src/keychain.rs` + cipher operations, fixture test with encrypted corpus open/verify cycle).
2. (security) Self-observation aggregate-only — grep `pipeline.l3.*` observability additions verify no per-service/per-fingerprint dimensions added to metrics.
3. (security) No OTLP attribute values in logs — integration test sweeps observability output for attribute-value leakage; redaction verified at scrubber coverage sites.
4. (security) Corpus key access skipped in tracing — `#[tracing::instrument(skip(key))]` present on heartbeat-drain + elastic-queue-flush + coalesce call sites.

## Relevant amendment history
- **2026-05-22** (chunk #77 corpus reconciliation): §Threat Model added 5th Type for persistent incident corpus (corpus.db cell-level AES-256-GCM + OS keychain custody); §Data Protection added corpus.db at-rest row with encryption detail; §Secret Management added Storage Runtime subsection (OS keychain primary + passphrase-fallback-with-warning per chunk #73 P-049); "What counts as secret" extended with corpus encryption key entry. Directly load-bearing for this chunk's corpus write posture.
- **2026-06-28-deterministic-env-gated-l4-mode** (chunk P-073 registration): Registered `ANDROMEDA_PULSE_L4_DETERMINISTIC` env var to §Input Validation CLI / env var inputs row; chunk P-074 (this chunk) builds on deterministic L4 mode for reproducible storm-handling acceptance test, no new secret/key material introduced.
