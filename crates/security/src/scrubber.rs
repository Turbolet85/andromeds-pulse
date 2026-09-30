//! PII scrubber primitive — chunk #68.
//!
//! Pattern catalog covering the 8 P-047 categories. Compiled-once via
//! `OnceLock`; per-call lookup is regex-set scan. Each pattern carries a
//! stable `category` label — only the category appears in scrubbed
//! output, never the matched value content.

use std::sync::OnceLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Outcome of scrubbing a single attribute value. `Allowed` carries the
/// original string when no pattern matched; `Redacted` carries only the
/// stable category label — never the raw value content (per capability
/// P-048 "No Raw OTLP Attribute Values Stored").
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrubbedValue {
    Allowed(String),
    Redacted { category: &'static str },
}

impl ScrubbedValue {
    /// True if the value matched any PII pattern.
    pub fn is_redacted(&self) -> bool {
        matches!(self, ScrubbedValue::Redacted { .. })
    }

    /// Category label when redacted; `None` when allowed.
    pub fn category(&self) -> Option<&'static str> {
        match self {
            ScrubbedValue::Redacted { category } => Some(category),
            ScrubbedValue::Allowed(_) => None,
        }
    }
}

/// Scrub a single attribute value against the P-047 pattern catalog.
/// Returns `Redacted` on first matching category; `Allowed` otherwise.
/// First-match wins — order matters when patterns overlap.
pub fn scrub_attribute(value: &str) -> ScrubbedValue {
    for (pattern, category, matches) in patterns() {
        if matches(pattern, value) {
            return ScrubbedValue::Redacted { category };
        }
    }
    ScrubbedValue::Allowed(value.to_string())
}

/// How an arm decides a match from its compiled regex.
type ArmPredicate = fn(&Regex, &str) -> bool;

fn regex_matches(pattern: &Regex, value: &str) -> bool {
    pattern.is_match(value)
}

const CARD_DIGITS_MIN: usize = 13;
const CARD_DIGITS_MAX: usize = 19;

fn card_number_matches(candidates: &Regex, value: &str) -> bool {
    candidates
        .find_iter(value)
        .any(|candidate| has_luhn_valid_window(candidate.as_str()))
}

/// True when some run of consecutive WHOLE separator groups carries 13-19
/// digits and passes Luhn. Windows never split a group: at arbitrary digit
/// offsets a long run offers so many windows that most non-card runs would
/// contain a Luhn-valid one by chance.
fn has_luhn_valid_window(candidate: &str) -> bool {
    let groups: Vec<&[u8]> = candidate
        .as_bytes()
        .split(|b| !b.is_ascii_digit())
        .filter(|group| !group.is_empty())
        .collect();
    let mut digits: Vec<u8> = Vec::with_capacity(CARD_DIGITS_MAX);
    for start in 0..groups.len() {
        digits.clear();
        for group in &groups[start..] {
            digits.extend_from_slice(group);
            if digits.len() > CARD_DIGITS_MAX {
                break;
            }
            if digits.len() >= CARD_DIGITS_MIN && luhn_valid(&digits) {
                return true;
            }
        }
    }
    false
}

fn luhn_valid(digits: &[u8]) -> bool {
    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(position, b)| {
            let digit = u32::from(b - b'0');
            if position % 2 == 1 {
                let doubled = digit * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                digit
            }
        })
        .sum();
    sum % 10 == 0
}

/// Lazy-init pattern catalog. Compiled regexes cached for process lifetime.
fn patterns() -> &'static [(Regex, &'static str, ArmPredicate)] {
    static PATTERNS: OnceLock<Vec<(Regex, &'static str, ArmPredicate)>> = OnceLock::new();
    PATTERNS
        .get_or_init(|| {
            // Order: more-specific patterns first; bearer/JWT before generic
            // base64 to avoid mis-categorization. Patterns chosen for
            // recall over precision — false positives are acceptable
            // (over-redaction); false negatives leak secrets.
            vec![
                // JWT — three base64url-encoded segments joined by `.`.
                // Detected before bearer to claim JWT-shaped values explicitly.
                (
                    Regex::new(r"eyJ[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+")
                        .expect("JWT regex compiles"),
                    "jwt",
                    regex_matches,
                ),
                // Bearer token in HTTP header form.
                (
                    Regex::new(r"(?i)bearer\s+[A-Za-z0-9_\-\.=]{16,}")
                        .expect("bearer regex compiles"),
                    "bearer",
                    regex_matches,
                ),
                // Generic API key in key=value form. `(?i)` for case-insensitive
                // key name; `[\w\-]{12,}` for the value (≥12 chars filters
                // most noise).
                (
                    Regex::new(r"(?i)(api[_\-]?key|access[_\-]?token|secret[_\-]?key|auth[_\-]?token)[\s=:]+[\w\-]{12,}")
                        .expect("api_key regex compiles"),
                    "api_key",
                    regex_matches,
                ),
                // Secret-like key=value pairs (password, secret, token in key
                // name with any value). Matches headers + structured logging.
                (
                    Regex::new(r"(?i)(password|passwd|secret|token)[\s=:]+\S+")
                        .expect("secret_kv regex compiles"),
                    "secret_kv",
                    regex_matches,
                ),
                // Bare provider credential — a standalone token carrying no
                // key name, so the keyed arms above cannot reach it. Anchored
                // on published issuer prefixes plus a length floor rather than
                // entropy scoring, per obs-plan §8 (which names this shape
                // family and warns that over-broad patterns false-positive on
                // ordinary high-entropy identifiers). Placed after the keyed
                // arms so `api_key=…` keeps its own category, and before
                // credit_card so a digit-heavy key is not mis-labelled.
                (
                    Regex::new(concat!(
                        r"\b(?:",
                        r"[sr]k_(?:live|test)_[A-Za-z0-9]{16,}",
                        r"|sk-[A-Za-z0-9_\-]{20,}",
                        r"|gh[pousr]_[A-Za-z0-9]{36}",
                        r"|AKIA[0-9A-Z]{16}",
                        r"|xox[baprs]-[A-Za-z0-9\-]{10,}",
                        r"|AIza[0-9A-Za-z_\-]{35}",
                        r")",
                    ))
                    .expect("provider_key regex compiles"),
                    "provider_key",
                    regex_matches,
                ),
                // Email — RFC 5322 simplified (sufficient for most PII recall).
                (
                    Regex::new(r"[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}")
                        .expect("email regex compiles"),
                    "email",
                    regex_matches,
                ),
                // Credit card — a run of ASCII digit groups joined by single
                // ` ` or `-` matches only when some window of whole groups
                // with 13-19 digits passes Luhn, which every issued card
                // number does (ISO/IEC 7812). Luhn is a structural check like
                // provider_key's anchored prefix, never entropy scoring; it
                // stops date stamps, timestamps and path segments from
                // redacting as cards. Accepted recall trade: a mistyped
                // (Luhn-invalid) card number, or one fused into a longer
                // digit group, is no longer redacted.
                (
                    Regex::new(r"\b[0-9]+(?:[ \-][0-9]+)*\b")
                        .expect("credit_card regex compiles"),
                    "credit_card",
                    card_number_matches,
                ),
                // US Social Security Number — three digits, two digits, four
                // digits with optional separators.
                (
                    Regex::new(r"\b\d{3}[ \-]?\d{2}[ \-]?\d{4}\b")
                        .expect("ssn regex compiles"),
                    "ssn",
                    regex_matches,
                ),
            ]
        })
        .as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
        "jwt"
    )]
    #[case("Authorization: Bearer abc123def456ghi789jkl", "bearer")]
    #[case("api_key=sk-proj-1234567890abcdef", "api_key")]
    #[case("password=hunter2", "secret_kv")]
    #[case("sk_live_51NotARealKeyOnlyForPulseTests00", "provider_key")] // gitleaks:allow
    #[case("sk-proj-NotARealKeyOnlyForPulseTests0000", "provider_key")] // gitleaks:allow
    #[case("ghp_NotARealTokenOnlyForPulseTests000000", "provider_key")] // gitleaks:allow
    #[case("AKIANOTAREALKEYID000", "provider_key")] // gitleaks:allow
    #[case("user@example.com signed up", "email")]
    #[case("4532-0151-1283-0366", "credit_card")]
    #[case("123-45-6789", "ssn")]
    fn scrubber_redacts_each_p047_category(#[case] input: &str, #[case] expected: &str) {
        let result = scrub_attribute(input);
        assert!(
            result.is_redacted(),
            "expected redaction for {input:?}, got {result:?}"
        );
        assert_eq!(result.category(), Some(expected));
    }

    #[rstest]
    #[case("4532015112830366")]
    #[case("4532 0151 1283 0366")]
    #[case("4111111111111111")]
    #[case("378282246310005")]
    #[case("3782 822463 10005")]
    #[case("card=4111111111111111")]
    #[case("ref 12 4111111111111111")]
    #[case("4111 1111 1111 1111 22")]
    fn scrubber_redacts_luhn_valid_card_forms(#[case] input: &str) {
        let result = scrub_attribute(input);
        assert_eq!(
            result.category(),
            Some("credit_card"),
            "expected a credit_card redaction for {input:?}, got {result:?}"
        );
    }

    /// Digit runs that are not card numbers. The date-time stamp is the shape
    /// of a harness data-dir basename that reached the model's digest and the
    /// Report as `[redacted: credit_card]` (Conductor `c97f697`, b2 capture);
    /// the values here are synthetic literals of that shape.
    #[rstest]
    #[case("rm-20260923-093840")]
    #[case("/tmp/rm-20260923-093840")]
    #[case(r"C:\Users\runner\AppData\Local\Temp\rm-20260923-093840")]
    #[case("workspace=/tmp/rm-20260923-093840")]
    #[case("PROJECT: rm-20260923-093840 (vcs=git)")]
    #[case("1790699962319180900")]
    // Card-shaped but Luhn-invalid: the documented precision boundary.
    #[case("4532-1234-5678-9010")]
    fn scrubber_allows_non_card_digit_runs(#[case] input: &str) {
        let result = scrub_attribute(input);
        match result {
            ScrubbedValue::Allowed(s) => assert_eq!(s, input, "allowed value was altered"),
            ScrubbedValue::Redacted { category } => {
                panic!("non-card digit run {input:?} was redacted as {category}")
            }
        }
    }

    #[rstest]
    #[case("hello world")]
    #[case("user logged in")]
    #[case("trace_id=abc-123")]
    #[case("status=200")]
    // Ordinary high-entropy identifiers the provider_key arm must NOT claim —
    // the recall/false-positive boundary this catalog trades on.
    #[case("a3f5b8c2d1e4f6a7b8c9d0e1f2a3b4c5d6e7f8a9")]
    #[case("550e8400-e29b-41d4-a716-446655440000")]
    #[case("dGhpcyBpcyBub3QgYSBzZWNyZXQgYXQgYWxs")]
    #[case("task-runner-scheduled-batch-0000000042")]
    #[case("checkout-service.orders.v2.handler")]
    fn scrubber_allows_non_pii_values(#[case] input: &str) {
        let result = scrub_attribute(input);
        assert!(
            !result.is_redacted(),
            "expected pass-through for {input:?}, got {result:?}"
        );
    }

    #[rstest]
    // Key / predicate columns routed through the scrubber by the ingestion
    // scrub-coverage chunk. These are IDENTITIES, not free text: a false
    // positive here does not merely over-redact one cell, it forks a service
    // or metric into two identities across every GROUP BY, and on
    // `metrics_points.metric_name` it can collide the primary key.
    #[case("checkout-service")]
    #[case("payment-api")]
    #[case("api-gateway")]
    #[case("auth-service-v2")]
    #[case("svc.orders.worker-03")]
    // `span_events.name` — the literal that gates Q3_EXCEPTION_FINGERPRINTS.
    #[case("exception")]
    #[case("http.server.request")]
    // `metrics_points.metric_name` — PK member.
    #[case("requests.total")]
    #[case("http.server.duration")]
    #[case("process.runtime.jvm.memory.used")]
    // `log_records.severity_text` — the standard level vocabulary.
    #[case("TRACE")]
    #[case("DEBUG")]
    #[case("INFO")]
    #[case("WARN")]
    #[case("ERROR")]
    #[case("FATAL")]
    fn scrubber_preserves_key_and_predicate_column_identities(#[case] input: &str) {
        let result = scrub_attribute(input);
        assert!(
            !result.is_redacted(),
            "identity column value was redacted — this forks one identity into two: {input:?}"
        );
        match result {
            ScrubbedValue::Allowed(s) => assert_eq!(s, input, "identity value was altered"),
            ScrubbedValue::Redacted { .. } => unreachable!("asserted not redacted above"),
        }
    }

    #[test]
    fn scrubbed_value_is_redacted_returns_true_for_redacted_variant() {
        let v = ScrubbedValue::Redacted { category: "email" };
        assert!(v.is_redacted());
        assert_eq!(v.category(), Some("email"));
    }

    #[test]
    fn scrubbed_value_is_redacted_returns_false_for_allowed_variant() {
        let v = ScrubbedValue::Allowed("clean".to_string());
        assert!(!v.is_redacted());
        assert_eq!(v.category(), None);
    }

    proptest! {
        /// Invariant: scrubber output never contains the raw matched
        /// value content (only the category label). Verified by
        /// inspecting the enum shape — `Redacted` cannot carry value
        /// content by construction.
        #[test]
        fn redacted_variant_never_carries_value_content(s in ".{1,200}") {
            let result = scrub_attribute(&s);
            if let ScrubbedValue::Redacted { category } = result {
                prop_assert!(!category.is_empty());
                prop_assert!(category.is_ascii());
            }
        }
    }
}
