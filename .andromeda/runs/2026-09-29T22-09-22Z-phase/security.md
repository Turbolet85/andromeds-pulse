# security extract

## Relevance
relevant — the chunk retunes a P-047 catalog arm inside `security::scrubber::scrub_attribute`, the scrubber every persistence and egress boundary in security-plan §Security Anti-Patterns → Logging depends on.

## Constraints
- The catalog stays at EIGHT categories, with its documented arm ORDER intact. `provider_key` is placed after the keyed arms and before `credit_card` so that a digit-heavy key is not mis-labelled. A precision fix to `credit_card` must not add, drop or reorder arms (per security-plan §Security Anti-Patterns → Logging, "Uniform scrubber coverage").
- Recall over precision is the module's stated posture. It is bounded by a false-positive corpus that keeps ordinary high-entropy identifiers `Allowed`. The fix narrows ONE arm's precision and must not lower recall on any real card form the arm targets. Every existing known positive must stay `Redacted(credit_card)` (per security-plan §Security Anti-Patterns → Logging, "Uniform scrubber coverage").
- The INTENDED posture is that any attribute value crossing into corpus.db or the self-observation sink passes through `scrub_attribute()` first. The fix must stay inside the scrubber. It must not add a per-field or per-consumer exemption, such as skipping the scrub for workspace keys or paths, because that removes a boundary from the enumerated coverage (per security-plan §Security Anti-Patterns → Logging, "INTENDED posture" + "MEASURED reality").
- The pair-shaped (third) scrub shape feeds `scrub_attribute` the JOINED `key=value` form unioned with the bare value. Any new card-arm context rule (a word boundary, delimiter grammar or surrounding-character check) must still match a card number when a `key=` prefix sits in front of it. Otherwise `metrics_points.labels` loses card redaction while the bare-value tests stay green (per security-plan §Security Anti-Patterns → Logging, "A THIRD scrub SHAPE").
- The published workspace key is validated as untrusted on read (bounded, UTF-8, control-char-rejecting) and consumed only as an opaque filter string, never as a path. The chunk reads that path to pin the end-to-end form and must not change the key's write or read validation (per security-plan §Input Validation, "Published workspace key" row; §Threat Model Summary, filesystem-reads vector).
- No log record may carry the scrubbed input, the matched digit run or the offending substring. Diagnostic tracing added to the arm may emit only a category or count (per security-plan §Logging & Monitoring, "What NEVER to log"; §Security Anti-Patterns → Logging).
- Implementing a checksum (such as Luhn) needs no dependency. If a crate is added anyway, it inherits the pinning and `cargo deny` / `cargo audit` gates (per security-plan §Dependency Security).

## Patterns to follow
- Use the `provider_key` design precedent: an anchored shape with a length floor, bounded by the false-positive corpus in `crates/security/src/scrubber.rs`, never entropy scoring. A card check that adds structural validation (checksum, separator grammar) fits this pattern (per security-plan §Security Anti-Patterns → Logging).
- Pin both directions in the scrubber's test module. The measured false positive (`rm-20260923-093840` and its workspace-key form) goes into the false-positive corpus as `Allowed`. The real card forms stay as `Redacted(credit_card)` pins (per security-plan §Security Anti-Patterns → Logging, "bounded by a false-positive corpus").
- Verify with a mutation check, RED before and GREEN after. Restoring the bare 13–19-digit shape must turn the false-positive pin red. Neutralising the new check must turn a known-positive pin red. This is the verification form the plan records for every prior scrubber change (per security-plan §Security Anti-Patterns → Logging, "Verified RED-before / GREEN-after … mutation check").
- `ScrubbedValue::{Allowed, Redacted}` keeps its category-only `Redacted` payload. Consumers such as the resolver-side redaction marker and the joined-form label reconstruction depend on that shape (per security-plan §Security Anti-Patterns → Logging, "A THIRD scrub SHAPE").

## Anti-patterns to avoid
- Do not exempt a field (workspace key, path, `project_context`) from scrubbing to silence the false positive. That weakens redaction at a boundary the plan enumerates as covered (per security-plan §Security Anti-Patterns → Logging, "INTENDED posture").
- Do not use an entropy or heuristic score in place of structural validation. The plan rejects entropy scoring because it false-positives on trace ids and hashes (per security-plan §Security Anti-Patterns → Logging, "Uniform scrubber coverage").
- Do not log raw values in a new debug or trace line inside the scrubber: no matched text, no excerpt (per security-plan §Security Anti-Patterns → Logging, NEVER bullets).

## Contract bindings
- security ↔ tests: the false-positive corpus and the known-positive pins live in the scrubber's colocated tests, along with the mutation check. The test fixtures use synthetic card-shaped values, not real PII.
- security ↔ obs: any `redactions_applied` aggregate counter drops for inputs that no longer match. That is an expected behaviour change, not a regression, and obs owns the counter's meaning.
- security ↔ interpretation (L4 prompt / PROJECT line) and the resolver-side `scrub_string` / `project_context`: every consumer inherits the arm change. Which site scrubs the workspace key before it reaches the model is a question for research at HEAD. The plan does not say.
- security-plan's own body: the plan text names `credit_card` only as a catalog member and does not state the arm's shape. A grep for "Luhn" or "13-19" in security-plan.md finds nothing. The "13-19 digits … Doesn't check Luhn" description the scope cites lives in another artifact (probably the P-047 capability spec). Research should locate it, and it is the likely home of any wrap amendment.

## Acceptance criteria contributions
- `scrub_attribute("rm-20260923-093840")` and the workspace-key form that value reached the model in both return `Allowed`. No other arm, `ssn` included, picks the value up once the card arm stops matching it (per security-plan §Security Anti-Patterns → Logging, "bounded by a false-positive corpus").
- Every pre-existing known positive still returns `Redacted(credit_card)`, and the catalog count stays at 8. This includes `4532-1234-5678-9010`, which does NOT pass a Luhn checksum (computed digit sum 66). A Luhn-only design would therefore un-redact the scrubber's own named positive, and research must reconcile the check's form with that pin rather than rewrite the pin (per security-plan §Security Anti-Patterns → Logging, "Uniform scrubber coverage").
- A card-shaped value behind a key prefix still redacts under the pair-shaped (joined) scrub. Example: a label pair with a card value, scrubbed as `card=<value>` unioned with the bare value (per security-plan §Security Anti-Patterns → Logging, "A THIRD scrub SHAPE").
- `cargo deny check bans licenses sources` passes as its own invocation, and the dependency graph gains no new crate unless it passed that gate and `cargo audit` (per security-plan §Dependency Security).
