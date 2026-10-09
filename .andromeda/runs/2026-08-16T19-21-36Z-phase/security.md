# security extract

## Relevance
**partial** — the decision itself is a product/arch call, but both layers write to persistence boundaries the security plan governs (`span_events.fingerprint`, storm state, `incidents`/`incident_events` in corpus.db), both touch emission surfaces, and the chunk carries a folded §Dependency Security deferral prereq.

## Constraints
- Any attribute-derived value crossing into corpus.db or the self-observation log sink MUST pass `security::scrubber::scrub_attribute` first — the uniform-coverage invariant explicitly names the `span_events` appender write path and `StormStateSnapshot::scrubbed_clone` (per security-plan §Security Anti-Patterns → Logging, uniform-scrubber-coverage paragraph). A Layer-1 change to what the fingerprint preimage retains, or a Layer-2 change that writes additional incident rows, MUST preserve it. Whether the fingerprint/normalized-frame path already routes through the scrubber is research's question.
- Diagnostics on either layer MUST NOT emit raw OTLP attribute values, span content payloads, or full file paths — basename of a canonicalized path only (per security-plan §Security Anti-Patterns → Logging, bullets 1–2; §Logging & Monitoring "What NEVER to log"). This binds tightly here because Layer 1's subject matter *is* stacktrace text arriving as user-controlled OTLP attributes.
- Post-`prost`-decode invariants MUST bound attribute keys/values before the batch reaches `buffer::Appender` (per security-plan §Input Validation, OTLP/gRPC `:4317` row; §Security Anti-Patterns → Input). A token-leading rewrite replaces one greedy sweep with per-token scanning, so this bound is what keeps a pathological stacktrace from becoming a CPU-amplification surface. Whether ingest already enforces that bound upstream of `normalize_stacktrace` is research's question.
- Any query added or changed to filter by fingerprint / `scope_id` / service MUST use `Connection::prepare` with `?` placeholders; identifier or predicate interpolation is banned outright (per security-plan §Input Validation, DuckDB query-parameters row; §Security Anti-Patterns → Input).
- Incident rows MUST remain cell-level AES-256-GCM encrypted with the key sourced per the OS-credential-store-primary / opt-in-passphrase-fallback posture; no plaintext runtime state on disk (per security-plan §Data Protection → At rest → Persistent incident corpus; §Secret Management → Storage → Runtime). An incident-per-identity choice raises row volume but MUST NOT introduce an unencrypted side-channel for the extra incidents.
- If incident volume rises, corpus retention/compaction MUST still bound it (per security-plan §Data Protection → Data lifecycle, Corpus retention).
- The `cargo audit` standing deferral applies to this wrap: basis + named overlap (`cargo deny check advisories`) MUST be re-verified and the report MUST record `probe skipped per ratified interval (next: 28)` — never a silent skip (per security-plan §Dependency Security → CI integration, "Standing deferral — `cargo audit` unrunnable").

## Patterns to follow
- The `scrubbed_clone` shape already used by `BaselineState` and `StormStateSnapshot` is the established way to persist a struct carrying attribute-derived content (per security-plan §Security Anti-Patterns → Logging).
- Field redaction is applied at the `tracing-subscriber` layer, not at individual log call sites (per security-plan §Logging & Monitoring, Log format).
- Sensitive-bearing arguments are kept out of span fields via `#[tracing::instrument(skip(...))]` discipline at the call site (per security-plan §Secret Management, "What counts as secret").
- User-visible errors crossing the TauRPC bridge collapse to the `serde`-friendly `AppError` variants with a sanitized one-liner; the full chain stays in the internal log (per security-plan §Error Handling → TauRPC bridge; §Bootstrap phases `error-sanitization-wire`).

## Anti-patterns to avoid
- NEVER log raw OTLP attribute values, span/log content payloads, snapshot contents, or MCP tool response bodies (per security-plan §Security Anti-Patterns → Logging).
- NEVER `format!`-interpolate a fingerprint, service name, or `scope_id` into SQL against the `duckdb`/corpus connection (per security-plan §Security Anti-Patterns → Input).
- NEVER surface stack traces, Rust struct names, or file paths through `AppError::Internal { message }` to the webview — sanitize at the `From<thiserror::Error>` impl (per security-plan §Security Anti-Patterns → Logging).

## Contract bindings
- **PII scrubbing ↔ obs + tests:** the scrubber invariant (security-plan §Security Anti-Patterns → Logging) is the same surface obs instruments; tests pinning the decided semantic MUST use synthetic stacktraces/paths, never real PII or secret-shaped fixtures.
- **CI security gate ↔ tests §CI Integration:** `cargo deny check bans licenses sources advisories` is the gate that carries the `cargo audit` deferral's named overlap; its result and the deferral disposition belong in this chunk's wrap report (security-plan §Dependency Security → CI integration).
- **Corpus at-rest ↔ arch §Occupied Resources Corpus SQLite:** security-plan §Data Protection cross-references arch for the locked-schema view of the same encrypted-persistence reality; a Layer-2 change to incident write cardinality touches both.

## Acceptance criteria contributions
- (security) The wrap report records the `cargo audit` deferral disposition `probe skipped per ratified interval (next: 28)` with basis and named overlap re-verified, and `cargo deny check` passes with visible dispositions on its findings (per security-plan §Dependency Security → CI integration, Standing deferral).
- (security) No emission added or changed on the fingerprint or incident-dedupe path carries raw stacktrace text, OTLP attribute values, or a full file path (per security-plan §Security Anti-Patterns → Logging).
- (security) Every persistence write touched by the decision — `span_events.fingerprint`, storm state, `incidents`/`incident_events` — still routes attribute-derived content through `scrub_attribute()` before write (per security-plan §Security Anti-Patterns → Logging, uniform-scrubber coverage).
- (security) Any query added or modified to filter by fingerprint or `scope_id` uses prepared statements with `?` binding; no `format!`-built SQL predicate appears in the diff (per security-plan §Input Validation, DuckDB query-parameters row).

## Relevant amendment history
- **2026-08-16 (0-pending route-adaptation wrap)** — re-ratified the `cargo audit` standing deferral at pin #3, converted the probe from per-wrap to an every-3rd-wrap interval, and migrated the pin off `.claude/session-handoff.md` onto the working-route entry. This is the direct source of this chunk's folded PREREQ; it also states why recording the interval in the body was mandatory (a route pin contradicting the spec would be the same truth-in-diagnostics failure class this version's sweep owns).
- **2026-08-16-baseline-family-reachability** — the immediately prior chunk; absorbed the pin and recorded `probe skipped per ratified interval (next: 28)`, which is the disposition shape this chunk inherits.
- **2026-08-15-corpus-key-persistence** — origin of the deferral; closed corpus key custody at six sites and registered the ID-scoped `[bans] skip` carve-out discipline. Relevant because Layer 2's incident rows land in the corpus whose at-rest guarantee that amendment established.
- **2026-05-22 (chunk #77)** — prepended the uniform-scrubber-coverage framing to §Anti-Patterns → Logging, replacing the pre-#72 single-site framing and naming the storm-state and appender persist paths explicitly. This is the invariant both layers' persistence changes must preserve.
- **Registry-completeness precedent (2026-06-28 `L4_DETERMINISTIC`, 2026-06-29 `window-geometry.json`, 2026-08-16 baseline-bootstrap)** — if this chunk introduces any new external-input boundary, the established routine is to register it in §Input Validation when it is code-validated and unit-tested rather than escalate. No new boundary is apparent from the scope; noted so the precedent is available if one appears.
