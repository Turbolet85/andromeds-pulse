# `security` — PII Scrubber Primitive

## Responsibility
PII scrubbing primitive for OTLP attribute values (chunk #68 — Epoch 9 Foundation v0.2.0). Seven P-047 categories detected via OnceLock-cached compiled regex set. Consumed by `corpus` at ingestion boundary AND eventually by `pulse-app::observability` subscriber Layer (defense-in-depth; deferred to chunk #70+).

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
- **Module layout:** `lib.rs` (re-exports `scrubber::ScrubbedValue` + `scrubber::scrub_attribute`) / `scrubber.rs` (pattern catalog + OnceLock cache + scrub fn + 180-line impl + tests).
- **7 P-047 redaction categories:**
  - `jwt` — JWT bearer header pattern `eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+`
  - `bearer-token` — `Bearer\s+[A-Za-z0-9._~+/=-]{20,}` (≥20 chars after `Bearer `)
  - `api-key` — heuristic prefixed-key shapes (`sk-...` / `pk_...` / `Bearer ghp_...` etc.)
  - `secret-kv` — KV with sensitive key patterns (`api_key=...` / `password=...` / `token=...`)
  - `email` — RFC 5322 simplified pattern
  - `credit-card` — Luhn-validated 13-19 digit sequences with optional separators
  - `ssn` — `\d{3}-\d{2}-\d{4}` US SSN format
- **OnceLock-cached compiled regex set:** first call to `scrub_attribute()` compiles all 7 patterns; subsequent calls hit the cached `Vec<(&'static str, Regex)>`.
- **`ScrubbedValue` derives:** `Clone`, `Debug`, `PartialEq`, `Eq`, `Hash`, `Serialize`, `Deserialize`.
- **Public contract surface:** `pub fn scrub_attribute(value: &str) -> ScrubbedValue` + the enum + accessors (`is_redacted` / `category`).

## Service-specific gotchas
- **Regex false-positive rate is bounded but non-zero** — e.g., a 13-19 digit numeric sequence may match credit-card pattern even when it's a legitimate ID. The trade-off is intentional per P-051 capability spec (false-positive scrub is preferred over false-negative leak).
- **Allowed values pass through verbatim** — only the FIRST matching category triggers redaction; values that match none pass as `Allowed(String)`. Callers should NOT layer additional scrubbers on top of `Allowed` without explicit reason.
- **Performance:** ~1 µs/call after OnceLock warm-up; not a hot-path bottleneck at chunk #68 ingestion volumes.
- **Pattern catalog evolution:** adding a new category requires:
  1. New regex + category name in `scrubber.rs::CATALOG`
  2. New test case asserting redaction
  3. Update P-047 capability spec doc
  4. Coordinate with `crates/corpus` consumption if persistence semantics shift

## Entry points for modification
- **Pattern catalog:** `crates/security/src/scrubber.rs::CATALOG` (the 7-category OnceLock-initialized list)
- **`ScrubbedValue` enum:** `crates/security/src/scrubber.rs` (Allowed + Redacted variants)
- **Public re-exports:** `crates/security/src/lib.rs`
- **Tests:** 14 security tests (chunk #68 landed); per-category match + non-match scenarios + Hash/Eq invariants.
