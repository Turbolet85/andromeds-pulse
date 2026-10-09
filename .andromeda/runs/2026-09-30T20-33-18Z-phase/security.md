# security extract

## Relevance
relevant. The chunk changes what the P-047 scrubber primitive replaces, and that primitive is the redaction control for every persistence, digest and prompt boundary this plan names.

## Constraints
- **Detection must not weaken.** Per security-plan §Security Anti-Patterns → Logging (the "Uniform scrubber coverage" catalog paragraph), the catalog is 8 categories in a fixed order: JWT · bearer · api_key · secret_kv · provider_key · email · credit_card · ssn.
  - `credit_card` is precision-gated by Luhn over whole groups only, never arbitrary digit offsets.
  - The false-positive corpus keeps ordinary high-entropy identifiers `Allowed`.
  - Span masking may change only what is REPLACED. Every arm's recall and every false-positive-corpus verdict must hold unchanged.
- **The "whole value" sentence becomes a required amendment.** The same paragraph states "Whole-value replacement is unchanged (P-048): a true positive anywhere in a scrubbed value still replaces the WHOLE value".
  - The chunk makes that sentence false, so the plan needs an amendment to §Security Anti-Patterns → Logging, which the entry names.
  - The P-048 matrix entry is the paired target.
  - A new shape crossing the scrubber boundary is a candidate Boundary widening for P4, as the scope states.
- **Keyed matches must not re-expose the value.** Per §Security Anti-Patterns → Logging ("A THIRD scrub SHAPE exists, and it is not optional"), `metrics_points.labels` scrubs the JOINED `key=value` form, unioned with the bare value.
  - Keys stay verbatim. Only the value becomes `[REDACTED:{category}]`, rebuilt from the key already held.
  - The `secret_kv` / `api_key` arms are key-name-anchored, so a match span covers the key AND the value.
  - Span masking must not leave the value half readable.
  - It must not let two distinct keys collapse into one placeholder, which is the collision class that section records as closed.
- **Choke-point and primary-key identity must hold.** Per §Security Anti-Patterns → Logging:
  - `service_name` is scrubbed inside `extract_service_name` because that one fn feeds THREE consumers: the `spans` column, the storm `FingerprintObserver`, and the baseline tap, whose `CompositeSpanObserver` also keys the lifecycle `ServiceRegistry`.
  - A span-masked identity must stay byte-identical across all of them, so DuckDB and the registries do not desync.
  - `metrics_points.metric_name` sits inside a PRIMARY KEY whose collision fix is a content-INDEPENDENT `seq` ordinal. Deriving a stable identifier from the secret stays a rejected alternative.
- **The corpus summary path owes a disposition.** Per §Security Anti-Patterns → Logging ("MEASURED reality"), `Incident.resolution_summary_text` holds a `scrub_attribute`-scrubbed JSON projection of `L4Output`, written at three paths (`scrubbed_l4_json`).
  - Today `Redacted` collapses the whole projection to the category marker, and the report then renders the honest-degraded pending notice.
  - Span masking changes this path. Whether the JSON stays parseable after an in-place span replacement, and whether the degraded branch still applies, need an explicit disposition.
  - Whether this site does a whole-value replacement today is research's question.
- **The prompt bound must still hold.** Per §Input Validation (the L4 inference argv prompt row) and §Security Anti-Patterns → Code Patterns, OTLP-derived digest text reaches the `-p` argv operand only through `validate_prompt_bounded`: a 16384-byte ceiling and NUL/C0/C1 rejection, with `\n` / `\r` / `\t` whitelisted.
  - An over-long prompt is rejected, never truncated.
  - Span masking keeps MORE digest content, so a longer prompt must still be rejected whole.
  - The plan states the upstream `Digest::scrubbed_clone` is what makes that content PII-scrubbed, so span masking must keep that guarantee.
- **Masked text is still telemetry.** Per §Logging & Monitoring ("What NEVER to log") and the first bullet of §Security Anti-Patterns → Logging, span-masked text is still OTLP-derived user content.
  - Masking does not make it log-safe.
  - The same NEVER-log rules keep covering it, including snapshot / clipboard / MCP tool response bodies.

## Patterns to follow
- **Closure injection into leaf crates.** Per §Security Anti-Patterns → Logging, `StormStateSnapshot::scrubbed_clone` receives the scrubber as a closure. Leaf crates such as `triage` gain no dependency on `security`, and a changed scrubber result shape must travel through the same injection.
- **Key reconstruction.** Per the third-scrub-shape paragraph of §Security Anti-Patterns → Logging, rebuild `{key}=[REDACTED:{category}]` from the key already held, and scrub the joined form unioned with the bare value. That is the template for keeping a keyed match's value masked under span semantics.
- **Discriminating pins.** Per the catalog paragraph of §Security Anti-Patterns → Logging, the pins are mutation-checked in both directions. `scrubber_allows_non_card_digit_runs` is the precedent: every inverted or new pin must discriminate, not just pass.
- **Real-path verification.** Per the "Residual" paragraph of §Security Anti-Patterns → Logging:
  - Verify RED-before / GREEN-after on the real OTLP path.
  - Run a mutation check.
  - Run a live wire leg recording zero canary occurrences in the log.

## Anti-patterns to avoid
- **Placeholders that leak.** Never let a placeholder carry anything derived from the matched content: no prefix, suffix, length, hash or stable derived identifier. Per §Security Anti-Patterns → Logging, a stable derived identifier of the secret is the rejected alternative, and placeholders must stay category-only and content-independent.
- **Logging masked content.** Never log the span-masked text or its surrounding content on any self-observation path. Per §Security Anti-Patterns → Logging, raw OTLP values, content payloads, snapshot, clipboard and MCP bodies are NEVER-log; tracing records carry counts and categories only.
- **Truncating to fit.** Never truncate a lengthened digest or prompt to fit the ceiling. Per §Input Validation (the L4 argv row), over-long is rejected and the digest is skipped, because silent truncation corrupts model input.

## Contract bindings
- **security ↔ tests.**
  - The pin `pii_scrub_closure_still_collapses_a_digest_carrying_a_card` inverts. The false-positive corpus and the both-classes canary corpora carry the never-weaken guarantee: a keyed `password=…` canary AND a bare `sk_live_…` canary, since a bare-only corpus passes while the keyed class leaks.
  - The snapshot/clipboard resolver arm is still UNVERIFIED, owed via test-plan §1 `snapshot-resolver-level-coverage`. A span-masking claim about snapshot output cannot lean on that arm.
- **security ↔ obs.** Scrub outcomes surface in self-observation only as aggregate counts or categories (for example the `redactions_applied` count cited in §Security Anti-Patterns → Logging). Whether span masking changes what a "redaction" counts is a question to settle with the obs plan.
- **security ↔ arch / capability matrix.** P-048's semantics change. Per the scope's Boundaries, a new `ScrubbedValue` shape read by consumers across crates is the candidate Boundary widening that needs the founder's word at P4.
- **security ↔ interpretation.** Digest content that span masking keeps flows into the L4 prompt. It is bounded by `validate_prompt_bounded` and scrubbed upstream by `Digest::scrubbed_clone`, per §Input Validation.

## Acceptance criteria contributions
- **Per-category canaries.** For each of the 8 categories, embed a canary mid-value inside larger text and run it through the migrated consumers.
  - The output contains the secret 0×.
  - It contains the category placeholder, and the non-secret surrounding content survives.
  - The false-positive corpus (including `scrubber_allows_non_card_digit_runs`) stays `Allowed`.
  - (per security-plan §Security Anti-Patterns → Logging, catalog paragraph)
- **Keyed label canary.** Write a label `{password: "hunter2"}`, plus a keyed canary embedded in a longer label value, through the real OTLP path. The stored `metrics_points.labels` contains `hunter2` 0× and keeps the key verbatim. A mutation that neutralizes the joined-form scrub reddens the pin. (per security-plan §Security Anti-Patterns → Logging, third scrub shape)
- **Digest pin.** The inverted digest pin shows the card span masked and the other digest lines intact. The assembled L4 prompt still passes `validate_prompt_bounded`: 16384-byte ceiling, reject-not-truncate. (per security-plan §Input Validation, L4 inference argv prompt row)
- **Live wire leg.** It records 0 occurrences of every seeded canary across the self-observation log. This is the operator's slot, per the scope's operator directives. (per security-plan §Security Anti-Patterns → Logging, Residual paragraph and NEVER-log bullets)
