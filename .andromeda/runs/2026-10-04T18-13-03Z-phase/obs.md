# obs extract

## Relevance
partial — the chunk changes L4 prompt TEXT and adds one bounded label to a dev probe; obs applies only where that work touches an emit site (a new or widened `interpretation.*` record, the probe's label output) or risks putting prompt/model/corpus text on the self-observation wire.

## Constraints
- Every `interpretation.*` tracing target resolves to its OWN exact allowlist leaf whose field set equals the emit site's, and no bare `interpretation` key may exist (`for_target("interpretation").is_none()`). Without its own leaf, a new target or a field added to an existing target (`interpretation.incident.created` / `.skipped` / `.generation.damper`) is silently redacted. The scope expects no new emit; whether any is added is for research and the plan to settle (per obs-plan §8 PII Scrubbing → `interpretation.incident.created` / `interpretation.incident.skipped` / `interpretation.generation.damper` entries).
- The L4 records are label-only. They never carry title, symptom, `payload_summary`, digest, prompt or model text, and never incident identity or `scope_id`. The new corpus-match framing and the triggering-cue naming must not bring any of that text into a log field (per obs-plan §8 PII Scrubbing → `interpretation.incident.skipped`; §6 Log Coverage required fields).
- Default-deny: only allowlisted fields are logged, and scrubbing happens at the source (`#[instrument(skip(...))]` plus explicit `fields(...)`) and again at the subscriber Layer. Corpus-match content and digest text derive from OTLP attribute values (High classification) (per obs-plan §8 PII Scrubbing → data classification table + Integration points).
- Any new label is a closed enumerated set. The names-the-trigger label on the probe is bounded and never free text (per obs-plan §5 Metric Coverage → label cardinality discipline; §11 Metrics).
- No per-generation or per-cue log spam on the L4 path. Any new signal is folded onto an existing once-per-generation record or tick, never emitted per decision (per obs-plan §11 Logs "hot path at info"; §8 → `interpretation.generation.damper`'s once-per-transition rule).
- Self-observation stays `tracing`-only, with no OTel SDK and no exporter. The probe's label output is a dev-tool readout, and if it goes through `tracing` the same JSON-per-line discipline holds. Whether the probe emits through `tracing` or prints to stdout is for research to measure (per obs-plan §1 Architectural choice; §11 Universal).

## Patterns to follow
- Exact-leaf registration with a field-set EQUALITY guard (both directions), plus a no-bare-`interpretation` fallback discriminator and a banned-field loop. Guards live under `pulse-app/tests/` because `[lib] test = false` means a src-level `mod tests` never runs. Precedents: `pulse-app/tests/unit_observability_allowlist_incident_skip.rs` and `unit_observability_allowlist_generation_damper.rs` (per obs-plan §8 → `interpretation.incident.skipped` / `interpretation.generation.damper`).
- Leaf COMPLETION, not a new target, when a field joins an existing record. The `triage.incident.persist` 2→5 and `triage.cue.tick` 5→9 precedents show that a partly-redacted target passes a resolve-only probe (per obs-plan §8 → `triage.cue.tick` / `triage.incident.persist` entries).
- Bounded `&'static str` / closed-enum labels at the emit site, so the domain is bounded by construction (per obs-plan §8 → `triage.incident.persist` `persist_kind`; `interpretation.incident.skipped` `skip_reason`).
- Wire-read verification: count `<redacted>` fields and planted-canary hits across the run's log family. The last precedent was the live storm with 0 `<redacted>` (per obs-plan §8 → `interpretation.incident.skipped` "wire-read at the chunk's live storm").

## Anti-patterns to avoid
- NEVER log prompt text, corpus-match text, digest content or generated model text (title / symptom / hypotheses). Raw OTLP-derived values and their LLM restatements are Vector 1 content (per obs-plan §11 Logs; §8 → `interpretation.incident.skipped` "never … digest, prompt or model text").
- NEVER add an `interpretation.*` field or target without its exact leaf, and never re-add a bare `interpretation` prefix key. Either way the field is silently redacted, or every sibling widens to one field set (per obs-plan §8 → `interpretation.incident.created` bare-key mechanism lesson).
- NEVER use an unbounded label value, such as a model-title substring or a cue `scope_id` (per obs-plan §11 Metrics).

## Contract bindings
- obs ↔ tests: allowlist leaf guards (exact-resolve, set equality, fallback discriminator) are test files under `pulse-app/tests/`. Producer pins for the L4 incident path live in `pulse-app/tests/unit_incident_producer.rs`. A widened `interpretation.*` record binds both (per obs-plan §8 → `interpretation.incident.skipped` guard + producer pins).
- obs ↔ security: the NEVER-log ban on model output / raw attribute values and the P-047 scrubber both apply to the prompt's new framing text. Whether the composed prompt's corpus section is already scrubbed upstream is a research question, not an obs attestation (per obs-plan §8 PII Scrubbing → Integration points; security-plan §Security Anti-Patterns → Logging).

## Acceptance criteria contributions
- (obs) If any `interpretation.*` record is added or gains a field, its exact allowlist leaf equals the emit site's field set in both directions, `for_target("interpretation").is_none()` still holds, and the guard runs under `pulse-app/tests/` (per obs-plan §8 PII Scrubbing → `interpretation.incident.skipped` / `interpretation.generation.damper`).
- (obs) A canary planted in corpus-match / digest text and in the generated title appears 0 times in the self-observation log family across an L4 generation that uses the new framing (per obs-plan §8 → `interpretation.incident.skipped` "never … digest, prompt or model text"; §11 Logs).
- (obs) The probe's names-the-trigger label takes only values from a closed enumerated set and carries no title, symptom or cue `scope_id` text (per obs-plan §5 Metric Coverage → label cardinality discipline).
- (obs) A wire read of any run that exercises the changed path shows 0 `<redacted>` fields on the `interpretation.*` targets it emits (per obs-plan §8 → muted-diagnostic backlog CLOSED; `interpretation.incident.skipped` wire-read precedent).
