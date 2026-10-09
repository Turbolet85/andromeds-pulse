# `security` — PII Scrubber Primitive

## Responsibility
PII scrubbing primitive for OTLP attribute values (chunk #68 — Epoch 9 Foundation v0.2.0). Eight P-047 categories, detected via an OnceLock-cached compiled pattern catalog. Consumed at every persistence / egress scrub boundary (buffer appender + labels, Drain, corpus persists, digest `scrubbed_clone`, Report projection, MCP / training export) — security-plan §Security Anti-Patterns → Logging "Uniform scrubber coverage" owns the enumeration. Since chunk 2026-09-30-span-level-redaction every boundary but `metrics_points.labels` MASKS each secret where it sits (`mask_secret_spans`) instead of replacing the whole value.

## Key integrations

### Consumes from
- Caller-provided `&str` references (OTLP attribute values; tracing field values).

### Publishes to
- `ScrubbedValue::Allowed(String)` / `ScrubbedValue::Redacted { category: &'static str }` — the detection VERDICT (`scrub_attribute`); its one production caller is `encode_labels` (whole VALUE per label, key verbatim).
- `MaskedValue { text, redacted, spans }` — the span-masked value (`mask_secret_spans`); `redacted` equals the verdict, `spans` counts the first pass's merged spans.
- `crates/corpus` consumes at write boundary (PII never persists to disk).
- Future: `pulse-app/src/observability.rs` subscriber Layer (defense-in-depth at log emission; chunk #70+).

### Dependencies
- `regex v1.x` (workspace-pinned; chunk #68 dep addition for PII pattern catalog).
- Workspace-inherited: `serde` (for `ScrubbedValue` serde derives), `thiserror` (error enum). Zero external deps beyond regex.

## Internal conventions
- **Module layout:** `lib.rs` (`pub mod scrubber;` — callers import `security::scrubber::{…}`) / `scrubber.rs` (pattern catalog + OnceLock cache + verdict fn + span-masking fn + tests).
- **8 P-047 redaction categories, first match wins for the verdict, in this order** (each arm also carries its mask CLASS):
  - `jwt` (bare) — `eyJ[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+` (three base64url segments)
  - `bearer` (keyed) — `(?i)bearer\s+[A-Za-z0-9_\-\.=]{16,}`
  - `api_key` (keyed) — key-name-anchored: `(?i)(api[_\-]?key|access[_\-]?token|secret[_\-]?key|auth[_\-]?token)[\s=:]+[\w\-]{12,}`
  - `secret_kv` (keyed) — key-name-anchored: `(?i)(password|passwd|secret|token)[\s=:]+\S+`
  - `provider_key` (bare) — a BARE credential: anchored issuer prefixes plus a length floor (`sk_live_`/`sk_test_`/`rk_…`, `sk-`, `gh[pousr]_`, `AKIA`, `xox[baprs]-`, `AIza`), never entropy scoring
  - `email` (bare) — RFC 5322 simplified pattern
  - `credit_card` (digit run) — a candidate run of ASCII digit groups joined by single ` `/`-` (`\b[0-9]+(?:[ \-][0-9]+)*\b`) matches only when some window of consecutive WHOLE groups with 13–19 digits passes Luhn (chunk 2026-09-29-scrubber-path-false-positive)
  - `ssn` (bare) — `\b\d{3}[ \-]?\d{2}[ \-]?\d{4}\b`
- **OnceLock-cached catalog:** the first call compiles all 8 patterns into a `Vec<Arm>` (`regex` · `category` · `predicate` · `class: ArmClass {Keyed, Bare, DigitRun}`). The per-arm predicate is `regex_matches` for seven arms and `card_number_matches` (candidate → windows → Luhn) for `credit_card`; a new arm cannot be added without choosing its class.
- **Span masking** (`mask_secret_spans(value, placeholder)`): gated by `scrub_attribute`'s verdict; each arm's raw matches (filtered by the arm's own predicate) widen by class — keyed → to the end of the line holding the match end (`\n`/`\r`), bare → the enclosing whitespace-delimited token, digit run → the candidate as matched; overlapping or touching ranges merge under the lowest catalog index; text outside the ranges is copied verbatim; scanning uses char boundaries. A flagged value with no locatable span masks whole (fail closed). The pass re-runs to a fixpoint (≤ 4 extra passes).
- **`ScrubbedValue` derives:** `Clone`, `Debug`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`. `MaskedValue` derives `Clone`, `Debug`, `PartialEq`, `Eq`.
- **Public contract surface:** `pub fn scrub_attribute(value: &str) -> ScrubbedValue` + the enum + accessors (`is_redacted` / `category`), and `pub fn mask_secret_spans<F: Fn(&'static str) -> String>(value: &str, placeholder: F) -> MaskedValue` + `MaskedValue`. Everything else in `scrubber.rs` is private.

## Service-specific gotchas
- **Recall over precision for seven arms, precision-gated for `credit_card`.** The keyed arms over-redact on a key match (`password=1`), which is the intended trade (false-positive scrub over false-negative leak). The card arm is Luhn-gated, so a date-time stamp (`rm-20260923-093840`) or a 19-digit nanosecond timestamp no longer redacts. The accepted recall trade is that a mistyped (Luhn-invalid) card number, or one fused into a longer single digit group, does not redact either. Luhn at arbitrary digit offsets was rejected: most long digit runs would pass by chance.
- **A redaction masks the matched span, not the whole value** (since chunk 2026-09-30-span-level-redaction; founder-ratified class-aware extent). A card line in a digest `payload_summary` masks only the card; the other lines reach the model. The keyed extent runs to the end of the LINE — a key word swallows the rest of its line, never the next line. A single-token secret masks exactly as whole-value replacement did. Only `encode_labels` still replaces a whole (label) value.
- **Never mask serialized JSON as text** — `secret_kv`'s `\S+` runs across compact-JSON delimiters and leaves the document unparseable; mask each string leaf, then serialize (`scrubbed_l4_json`, the training export's `interpretation`).
- **Placeholder spelling belongs to the caller** — `[REDACTED:{c}]` (buffer, drain, persistence adapters), `[redacted:{c}]` (digest closure), `[redacted: {c}]` (markdown, inference, investigate, training export). No placeholder alone matches any arm; a count of redacted fields must look for a placeholder ANYWHERE in the field.
- **Allowed values pass through verbatim** — values that match none come back unchanged (`Allowed(String)` / `MaskedValue { redacted: false, .. }`). Callers should NOT layer additional scrubbers on top without explicit reason; masking already-masked text changes nothing.
- **The key-anchored arms need the key INSIDE the string** — a structured key/value boundary scrubs the joined `key=value` form (see `.claude/rules/security.md` Session Additions 2026-08-23).
- **Pattern catalog evolution:** adding or retuning a category requires:
  1. The arm in `scrubber.rs::patterns()` (order is load-bearing) with its `ArmClass`
  2. Recall AND false-positive `#[rstest]` cases plus `span_mask` extent cases, mutation-checked
  3. The security-plan §Security Anti-Patterns → Logging catalog paragraph
  4. Coordinate with the consumers if persistence semantics shift

## Entry points for modification
- **Pattern catalog:** `crates/security/src/scrubber.rs::patterns()` (the 8-arm OnceLock-initialized list)
- **Span masking:** `crates/security/src/scrubber.rs::{mask_secret_spans, mask_once}` (extent helpers `line_end` / `token_start` / `token_end`)
- **`ScrubbedValue` / `MaskedValue`:** `crates/security/src/scrubber.rs`
- **Public module:** `crates/security/src/lib.rs` (`pub mod scrubber;`)
- **Tests:** 85 in the crate suite (`cargo nextest list -p security --profile ci`, chunk 2026-09-30-span-level-redaction; 54 before). They cover per-category recall, the false-positive corpora (high-entropy identifiers, key/predicate column identities, non-card digit runs), Luhn-valid card forms, a proptest that `Redacted` never carries content, and 31 `span_mask` pins (embedded canaries per category, verdict equality, extents, merges, single-token parity, idempotence) with seeds in `proptest-regressions/scrubber.txt`.
