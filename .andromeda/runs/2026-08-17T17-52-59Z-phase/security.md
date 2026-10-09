# security extract

## Relevance
**partial** — no auth/API/secret/dependency surface is added, but the chunk changes what *kind* of value crosses a corpus persistence boundary (scrubbing + anonymization rules apply) and carries a live §Dependency Security probe obligation at its wrap.

## Constraints
- The repaired `Incident.fingerprint` must satisfy the uniform pre-persist invariant: **any attribute value crossing into persistent storage (`corpus.db`) or the self-observation sink must pass `security::scrubber::scrub_attribute()` first**, per security-plan.md §Security Anti-Patterns → Logging (uniform scrubber coverage). A model-authored free-form string is telemetry-derived content and is squarely in that invariant's scope; a blake3 hex digest may be exempt by construction. **Whether the current producer path already routes this field through the scrubber, and whether the digest form is treated as exempt, is research's question.**
- The value-kind change must keep the `incidents` row inside the single canonical AES-256-GCM cell-encrypted corpus with **no plaintext runtime state on disk** and pre-write PII scrubbing applied uniformly, per security-plan.md §Data Protection → At rest → Persistent incident corpus. No side artifact / debug dump of the fingerprint or its inputs.
- New instrumentation on the `AttentionCue` → `DigestCueRef` → producer threading must not emit raw OTLP attribute values, span/log content payloads, or exception/stack text — only the hash or an identifier — per security-plan.md §Logging & Monitoring ("What NEVER to log") and §Security Anti-Patterns → Logging.
- `ANDROMEDA_PULSE_L4_DETERMINISTIC` must remain a **bounded truthy-parse** (`1|true|yes`, default false, no unbounded string) per security-plan.md §Input Validation → CLI / env var inputs row. Touching the deterministic-fixture path must not regress that parse.
- If the chunk introduces **any** new external-input boundary (new env var, new cross-process file read, new IPC argument — scope says none expected, "to be confirmed at P3"), it must be registered in the §Input Validation boundary table, per security-plan.md §Input Validation. The amendment history shows this registry-completeness rule fired three times as a finding.
- The `cargo audit` **standing deferral's ratified re-run interval lands on this chunk's wrap**: the probe must be run for real and its result recorded, with the named overlap `cargo deny check advisories` re-observed and its findings carrying visible dispositions (no-safe-upgrade → ID-scoped ignore; upgradeable → named owner, never ignore-listed), per security-plan.md §Dependency Security → CI integration.

## Patterns to follow
- **`scrubbed_clone` at the corpus persist boundary** — the shape already used for `BaselineState` and `StormStateSnapshot` per security-plan.md §Security Anti-Patterns → Logging; the incidents-path producer should follow the same pre-write gate rather than inventing a new one.
- **Opaque-identifier discipline**, modeled on the published-workspace-key row in security-plan.md §Input Validation: a value consumed only as an opaque grouping/filter string — never opened, joined, or used as a path — with only a safe projection logged.
- **`#[tracing::instrument(skip(...))]` at sensitive-value call sites**, the discipline security-plan.md §Secret Management applies to key handling; the same shape is the right default for telemetry-derived payloads at the new threading sites.
- **Bounded env-var parse** for the deterministic gate per security-plan.md §Input Validation → CLI / env var inputs row.

## Anti-patterns to avoid
- NEVER log raw OTLP attribute values, span/log/metric content payloads, or exception-message/stack-frame text at the new cue→producer threading sites (security-plan.md §Security Anti-Patterns → Logging).
- NEVER let a telemetry-derived free-form string land in a persisted corpus cell without passing the scrubber gate — that is exactly the invariant the uniform-coverage framing asserts (security-plan.md §Security Anti-Patterns → Logging).
- NEVER relax `multiple-versions = "deny"`, skip the `tonic` canary, or ignore-list an advisory finding that has a stated safe upgrade (security-plan.md §Dependency Security → CI integration; §Security Anti-Patterns → Universal).

## Contract bindings
- **PII scrubbing ↔ obs + tests**: the field-selection choice on the new threading path is obs's schema call, but the NEVER-log list is security's; and tests' fixtures must carry hash-shaped, non-PII fingerprint values (no real telemetry strings in fixtures).
- **`cargo audit` probe + `cargo deny` overlap ↔ tests §CI Integration**: one workflow, security gate as a job; the probe result and overlap dispositions must land in this chunk's wrap report, not only in CI output.
- **Deterministic-mode constant fingerprint ↔ tests**: the `select_previously_seen` all-matches-all consequence under `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` is a tests-owned pin question; security's stake is only that the fixture value stays a bounded, non-telemetry-derived constant.

## Acceptance criteria contributions
- (security) `cargo audit` is RUN at this chunk's wrap (ratified interval point) and its verbatim result recorded in the report; a third consecutive silent skip is a fail (per security-plan.md §Dependency Security → CI integration).
- (security) `cargo deny check bans licenses sources` and `cargo deny check advisories` are run and recorded, with `multiple-versions = "deny"` unrelaxed, the `tonic` canary unskipped, and every advisory finding carrying a visible disposition (per security-plan.md §Dependency Security → CI integration; §Security Anti-Patterns → Universal).
- (security) Every value written into an `incidents` corpus cell on the repaired producer path is shown to pass `security::scrubber::scrub_attribute()` before write, or is shown to be a construction-exempt digest (per security-plan.md §Security Anti-Patterns → Logging; §Data Protection → At rest).
- (security) No tracing field added on the cue→producer threading path carries an exception message, stack-frame text, or OTLP attribute value — hash/identifier only (per security-plan.md §Logging & Monitoring).

## Relevant amendment history
- **2026-08-16 (0-pending route-adaptation wrap)** — re-ratified the `cargo audit` standing deferral at pin #3, set the every-3rd-wrap re-run INTERVAL, and migrated the pin off the session handoff onto the working-route entry. This is the amendment that makes the folded PREREQ in this chunk's scope binding, and it explicitly forbids a silent skip.
- **2026-08-15-corpus-key-persistence** — origin of that deferral; also closed corpus key custody at six sites and recorded the ID-scoped `[bans] skip` carve-out discipline. Relevant because the at-rest AES-256-GCM guarantee this chunk's persisted fingerprint rides on was established here.
- **2026-05-22 (chunk #77)** — prepended the uniform-scrubber-coverage framing to §Security Anti-Patterns → Logging and added the corpus rows to §Threat Model / §Data Protection / §Secret Management. That framing is the exact rule the repaired fingerprint value has to satisfy at the corpus persist boundary.
- **2026-06-28-deterministic-env-gated-l4-mode** — registered `ANDROMEDA_PULSE_L4_DETERMINISTIC` in §Input Validation with its bounded truthy-parse. Same env gate whose constant fixture this chunk's "known consequence" section names.
- **2026-06-29-window-geometry-movable-shell / 2026-08-16-baseline-family-reachability** — the two registry-completeness precedents (a new input boundary shipped unregistered is a D-security-input finding, ruled *routine* when the report shows it code-validated + unit-tested). This is the disposition path if P3 finds the chunk does introduce an input boundary after all.
