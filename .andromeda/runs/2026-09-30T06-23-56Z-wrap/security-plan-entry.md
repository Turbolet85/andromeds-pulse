
## 2026-09-29-scrubber-path-false-positive — credit_card arm Luhn-gated · npm current state re-read
**Section:** §Security Anti-Patterns → Logging (Uniform scrubber coverage · A THIRD scrub SHAPE · Residual) · §Dependency Security → npm channel → current state
**Change:**
- The `credit_card` arm was "13–19 digits with optional separators, no checksum" (`\b(?:\d[ \-]?){13,19}\b`) under a catalog-wide recall-over-precision posture. Now it is precision-gated: a candidate run of ASCII digit groups joined by single ` `/`-` matches only when some window of consecutive WHOLE groups carrying 13–19 digits passes Luhn.
- Recall over precision is scoped to the other seven arms, both in the catalog paragraph and in the labels failure-mode clause.
- Accepted recall trade: a Luhn-invalid (mistyped) card number, or one fused into a longer single digit group, no longer redacts.
- Closed: a date-time stamp such as the harness data-dir basename (path, `workspace=` key and digest `PROJECT:` forms) and a 19-digit nanosecond timestamp now return `Allowed`.
- Whole-value replacement (P-048) is unchanged.
- The Residual's "catalog unchanged, no arm retuned" is tied to `2026-08-23-ingestion-scrub-coverage`, and the later predicate retune is noted.
- npm current state re-read: `green-with-dispositions`, same four exceptions. Five advisories reported after the prior green run were closed by in-range bumps with no exception: brace-expansion 1.1.21 / 5.0.12, ip-address 10.7.2 (GHSA-6j4f, -qhr7, -q2hr, -h3mg, -j6r3).
**Why:** a date-stamped workspace key redacted as a card, wiping the digest the model read and the Report's Project Context. Every issued card number passes Luhn, so whole-group windows keep recall on real cards. Windows at arbitrary offsets would pass most long runs by chance. This is a boundary widening, ratified by the founder at /phase P4 (2026-09-30). The npm advisories had fixed releases, so the founder's no-deferral, no-exception-when-fixable ruling applied.
**Kept:** the whole-value replacement that blanks an entire digest on one true positive — a founder-ordered span-level redaction route entry owns it.
**Ref:** .andromeda/runs/2026-09-30T06-23-56Z-wrap/
