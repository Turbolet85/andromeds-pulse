# `security` — PII Scrubber Primitive

## Responsibility
PII scrubbing primitive for OTLP attribute values (chunk #68 — Epoch 9 Foundation v0.2.0). Eight P-047 categories, detected via an OnceLock-cached compiled pattern catalog. Consumed at every persistence / egress scrub boundary (buffer appender + labels, Drain, corpus persists, digest `scrubbed_clone`, Report projection, MCP / training export) — security-plan §Security Anti-Patterns → Logging "Uniform scrubber coverage" owns the enumeration.

## Key integrations

### Consumes from
- Caller-provided `&str` references (OTLP attribute values; tracing field values).

### Publishes to
- `ScrubbedValue::Allowed(String)` for non-PII pass-through.
- `ScrubbedValue::Redacted { category: &'static str }` for redaction matches.
- `crates/corpus` consumes at write boundary (PII never persists to disk).
- Future: `pulse-app/src/observability.rs` subscriber Layer (defense-in-depth at log emission; chunk #70+).

### Dependencies
- `regex v1.x` (workspace-pinned; chunk #68 dep addition for PII pattern catalog).
- Workspace-inherited: `serde` (for `ScrubbedValue` serde derives), `thiserror` (error enum). Zero external deps beyond regex.

## Internal conventions
- **Module layout:** `lib.rs` (re-exports `scrubber::ScrubbedValue` + `scrubber::scrub_attribute`) / `scrubber.rs` (pattern catalog + OnceLock cache + scrub fn + tests).
- **8 P-047 redaction categories, first match wins, in this order:**
  - `jwt` — `eyJ[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+` (three base64url segments)
  - `bearer` — `(?i)bearer\s+[A-Za-z0-9_\-\.=]{16,}`
  - `api_key` — key-name-anchored: `(?i)(api[_\-]?key|access[_\-]?token|secret[_\-]?key|auth[_\-]?token)[\s=:]+[\w\-]{12,}`
  - `secret_kv` — key-name-anchored: `(?i)(password|passwd|secret|token)[\s=:]+\S+`
  - `provider_key` — a BARE credential: anchored issuer prefixes plus a length floor (`sk_live_`/`sk_test_`/`rk_…`, `sk-`, `gh[pousr]_`, `AKIA`, `xox[baprs]-`, `AIza`), never entropy scoring
  - `email` — RFC 5322 simplified pattern
  - `credit_card` — a candidate run of ASCII digit groups joined by single ` `/`-` (`\b[0-9]+(?:[ \-][0-9]+)*\b`) matches only when some window of consecutive WHOLE groups with 13–19 digits passes Luhn (chunk 2026-09-29-scrubber-path-false-positive)
  - `ssn` — `\b\d{3}[ \-]?\d{2}[ \-]?\d{4}\b`
- **OnceLock-cached catalog:** the first `scrub_attribute()` call compiles all 8 patterns into `Vec<(Regex, &'static str, ArmPredicate)>`. The per-arm predicate is `regex_matches` for seven arms and `card_number_matches` (candidate → windows → Luhn) for `credit_card`.
- **`ScrubbedValue` derives:** `Clone`, `Debug`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`.
- **Public contract surface:** `pub fn scrub_attribute(value: &str) -> ScrubbedValue` + the enum + accessors (`is_redacted` / `category`). Everything else in `scrubber.rs` is private.

## Service-specific gotchas
- **Recall over precision for seven arms, precision-gated for `credit_card`.** The keyed arms over-redact on a key match (`password=1`), which is the intended trade (false-positive scrub over false-negative leak). The card arm is Luhn-gated, so a date-time stamp (`rm-20260923-093840`) or a 19-digit nanosecond timestamp no longer redacts. The accepted recall trade is that a mistyped (Luhn-invalid) card number, or one fused into a longer single digit group, does not redact either. Luhn at arbitrary digit offsets was rejected: most long digit runs would pass by chance.
- **A redaction replaces the WHOLE value** (P-048: `Redacted` carries only the category). One true positive anywhere in a multi-line value — e.g. a digest `payload_summary` — replaces the entire value with the placeholder.
- **Allowed values pass through verbatim** — only the FIRST matching category triggers redaction; values that match none pass as `Allowed(String)`. Callers should NOT layer additional scrubbers on top of `Allowed` without explicit reason.
- **The key-anchored arms need the key INSIDE the string** — a structured key/value boundary scrubs the joined `key=value` form (see `.claude/rules/security.md` Session Additions 2026-08-23).
- **Pattern catalog evolution:** adding or retuning a category requires:
  1. The arm in `scrubber.rs::patterns()` (order is load-bearing)
  2. Recall AND false-positive `#[rstest]` cases, mutation-checked
  3. The security-plan §Security Anti-Patterns → Logging catalog paragraph
  4. Coordinate with the consumers if persistence semantics shift

## Entry points for modification
- **Pattern catalog:** `crates/security/src/scrubber.rs::patterns()` (the 8-arm OnceLock-initialized list)
- **`ScrubbedValue` enum:** `crates/security/src/scrubber.rs` (Allowed + Redacted variants)
- **Public re-exports:** `crates/security/src/lib.rs`
- **Tests:** 54 in the crate suite, as measured by `cargo nextest run -p security` at chunk 2026-09-29-scrubber-path-false-positive. They cover per-category recall, the false-positive corpora (high-entropy identifiers, key/predicate column identities, non-card digit runs), Luhn-valid card forms, and a proptest that `Redacted` never carries content.
