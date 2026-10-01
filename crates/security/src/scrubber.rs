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
    for arm in patterns() {
        if (arm.predicate)(&arm.regex, value) {
            return ScrubbedValue::Redacted {
                category: arm.category,
            };
        }
    }
    ScrubbedValue::Allowed(value.to_string())
}

/// Outcome of span-level masking. `text` keeps every byte outside a masked
/// span verbatim; `redacted` is exactly `scrub_attribute(value).is_redacted()`;
/// `spans` counts the merged spans the first pass masked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaskedValue {
    pub text: String,
    pub redacted: bool,
    pub spans: usize,
}

/// Mask each secret where it sits instead of replacing the whole value.
///
/// Detection is `scrub_attribute`'s verdict, unchanged. Each arm match widens
/// by its arm's class (keyed: to the end of the line; bare: to the enclosing
/// whitespace-delimited token; card: the whole digit-group run), overlapping
/// or touching spans merge under the earliest arm in catalog order, and
/// `placeholder` renders each merged span from its category so every consumer
/// keeps its own spelling.
pub fn mask_secret_spans<F>(value: &str, placeholder: F) -> MaskedValue
where
    F: Fn(&'static str) -> String,
{
    let category = match scrub_attribute(value) {
        ScrubbedValue::Allowed(text) => {
            return MaskedValue {
                text,
                redacted: false,
                spans: 0,
            };
        }
        ScrubbedValue::Redacted { category } => category,
    };
    let Some((mut text, spans)) = mask_once(value, &placeholder) else {
        // Fail closed: a verdict with no locatable span masks the whole value.
        return MaskedValue {
            text: placeholder(category),
            redacted: true,
            spans: 1,
        };
    };
    // Re-mask to a fixpoint so layered sites can re-scrub without change: a
    // mask can expose a match its neighbour hid (a digit run fused to a key
    // word gains a word boundary once the key is masked).
    for _ in 0..MAX_MASK_PASSES {
        match mask_once(&text, &placeholder) {
            Some((next, _)) if next != text => text = next,
            _ => break,
        }
    }
    MaskedValue {
        text,
        redacted: true,
        spans,
    }
}

const MAX_MASK_PASSES: usize = 4;

/// One masking pass; `None` when no arm locates a span.
fn mask_once<F>(value: &str, placeholder: &F) -> Option<(String, usize)>
where
    F: Fn(&'static str) -> String,
{
    let arms = patterns();
    let mut ranges: Vec<(usize, usize, usize)> = Vec::new();
    for (index, arm) in arms.iter().enumerate() {
        for found in arm.regex.find_iter(value) {
            if !(arm.predicate)(&arm.regex, found.as_str()) {
                continue;
            }
            let (start, end) = match arm.class {
                ArmClass::Keyed => (found.start(), line_end(value, found.end())),
                ArmClass::Bare => (
                    token_start(value, found.start()),
                    token_end(value, found.end()),
                ),
                ArmClass::DigitRun => (found.start(), found.end()),
            };
            ranges.push((start, end, index));
        }
    }
    if ranges.is_empty() {
        return None;
    }
    ranges.sort_unstable();

    let mut merged: Vec<(usize, usize, usize)> = Vec::with_capacity(ranges.len());
    for (start, end, index) in ranges {
        match merged.last_mut() {
            Some(last) if start <= last.1 => {
                last.1 = last.1.max(end);
                last.2 = last.2.min(index);
            }
            _ => merged.push((start, end, index)),
        }
    }

    let mut out = String::with_capacity(value.len());
    let mut cursor = 0;
    for &(start, end, index) in &merged {
        out.push_str(&value[cursor..start]);
        out.push_str(&placeholder(arms[index].category));
        cursor = end;
    }
    out.push_str(&value[cursor..]);
    Some((out, merged.len()))
}

/// End of the line holding `from`: the next `\n` / `\r`, or the value's end.
fn line_end(value: &str, from: usize) -> usize {
    value[from..]
        .find(['\n', '\r'])
        .map_or(value.len(), |offset| from + offset)
}

/// Start of the whitespace-delimited token holding `at`.
fn token_start(value: &str, at: usize) -> usize {
    value[..at]
        .char_indices()
        .rev()
        .find(|(_, c)| c.is_whitespace())
        .map_or(0, |(offset, c)| offset + c.len_utf8())
}

/// End of the whitespace-delimited token holding `from`.
fn token_end(value: &str, from: usize) -> usize {
    value[from..]
        .char_indices()
        .find(|(_, c)| c.is_whitespace())
        .map_or(value.len(), |(offset, _)| from + offset)
}

/// How an arm decides a match from its compiled regex.
type ArmPredicate = fn(&Regex, &str) -> bool;

/// How far a mask extends past an arm's raw match. Every catalog arm declares
/// one, so a new arm cannot join the catalog without choosing its extent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArmClass {
    /// Key-anchored: from the key word to the end of the line, because the
    /// secret after a key can run past what the regex sees (a multi-word value,
    /// a base64 tail past `+` or `/`).
    Keyed,
    /// Bare shape: the whole whitespace-delimited token holding the match.
    Bare,
    /// The whole candidate digit-group run.
    DigitRun,
}

struct Arm {
    regex: Regex,
    category: &'static str,
    predicate: ArmPredicate,
    class: ArmClass,
}

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
fn patterns() -> &'static [Arm] {
    static PATTERNS: OnceLock<Vec<Arm>> = OnceLock::new();
    PATTERNS
        .get_or_init(|| {
            // Order: more-specific patterns first; bearer/JWT before generic
            // base64 to avoid mis-categorization. Patterns chosen for
            // recall over precision — false positives are acceptable
            // (over-redaction); false negatives leak secrets.
            let catalog: Vec<(Regex, &'static str, ArmPredicate, ArmClass)> = vec![
                // JWT — three base64url-encoded segments joined by `.`.
                // Detected before bearer to claim JWT-shaped values explicitly.
                (
                    Regex::new(r"eyJ[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+")
                        .expect("JWT regex compiles"),
                    "jwt",
                    regex_matches,
                    ArmClass::Bare,
                ),
                // Bearer token in HTTP header form.
                (
                    Regex::new(r"(?i)bearer\s+[A-Za-z0-9_\-\.=]{16,}")
                        .expect("bearer regex compiles"),
                    "bearer",
                    regex_matches,
                    ArmClass::Keyed,
                ),
                // Generic API key in key=value form. `(?i)` for case-insensitive
                // key name; `[\w\-]{12,}` for the value (≥12 chars filters
                // most noise).
                (
                    Regex::new(r"(?i)(api[_\-]?key|access[_\-]?token|secret[_\-]?key|auth[_\-]?token)[\s=:]+[\w\-]{12,}")
                        .expect("api_key regex compiles"),
                    "api_key",
                    regex_matches,
                    ArmClass::Keyed,
                ),
                // Secret-like key=value pairs (password, secret, token in key
                // name with any value). Matches headers + structured logging.
                (
                    Regex::new(r"(?i)(password|passwd|secret|token)[\s=:]+\S+")
                        .expect("secret_kv regex compiles"),
                    "secret_kv",
                    regex_matches,
                    ArmClass::Keyed,
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
                    ArmClass::Bare,
                ),
                // Email — RFC 5322 simplified (sufficient for most PII recall).
                (
                    Regex::new(r"[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}")
                        .expect("email regex compiles"),
                    "email",
                    regex_matches,
                    ArmClass::Bare,
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
                    ArmClass::DigitRun,
                ),
                // US Social Security Number — three digits, two digits, four
                // digits with optional separators.
                (
                    Regex::new(r"\b\d{3}[ \-]?\d{2}[ \-]?\d{4}\b")
                        .expect("ssn regex compiles"),
                    "ssn",
                    regex_matches,
                    ArmClass::Bare,
                ),
            ];
            catalog
                .into_iter()
                .map(|(regex, category, predicate, class)| Arm {
                regex,
                category,
                predicate,
                class,
            })
            .collect()
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

    fn mark(category: &'static str) -> String {
        format!("<{category}>")
    }

    /// The recall corpus above, as data: every value here must redact.
    const RECALL_CORPUS: &[&str] = &[
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
        "Authorization: Bearer abc123def456ghi789jkl",
        "api_key=sk-proj-1234567890abcdef",
        "password=hunter2",
        "sk_live_51NotARealKeyOnlyForPulseTests00", // gitleaks:allow
        "sk-proj-NotARealKeyOnlyForPulseTests0000", // gitleaks:allow
        "ghp_NotARealTokenOnlyForPulseTests000000", // gitleaks:allow
        "AKIANOTAREALKEYID000",                     // gitleaks:allow
        "user@example.com signed up",
        "4532-0151-1283-0366",
        "123-45-6789",
        "4532015112830366",
        "4532 0151 1283 0366",
        "4111111111111111",
        "378282246310005",
        "3782 822463 10005",
        "card=4111111111111111",
        "ref 12 4111111111111111",
        "4111 1111 1111 1111 22",
    ];

    /// The false-positive and identity corpora above: every value must pass.
    const ALLOWED_CORPUS: &[&str] = &[
        "rm-20260923-093840",
        "/tmp/rm-20260923-093840",
        r"C:\Users\runner\AppData\Local\Temp\rm-20260923-093840",
        "workspace=/tmp/rm-20260923-093840",
        "PROJECT: rm-20260923-093840 (vcs=git)",
        "1790699962319180900",
        "4532-1234-5678-9010",
        "hello world",
        "user logged in",
        "trace_id=abc-123",
        "status=200",
        "a3f5b8c2d1e4f6a7b8c9d0e1f2a3b4c5d6e7f8a9",
        "550e8400-e29b-41d4-a716-446655440000",
        "dGhpcyBpcyBub3QgYSBzZWNyZXQgYXQgYWxs",
        "task-runner-scheduled-batch-0000000042",
        "checkout-service.orders.v2.handler",
        "checkout-service",
        "payment-api",
        "svc.orders.worker-03",
        "exception",
        "requests.total",
        "process.runtime.jvm.memory.used",
        "ERROR",
    ];

    #[test]
    fn span_mask_redaction_verdict_equals_scrub_attribute_over_both_corpora() {
        for value in RECALL_CORPUS.iter().chain(ALLOWED_CORPUS) {
            assert_eq!(
                mask_secret_spans(value, mark).redacted,
                scrub_attribute(value).is_redacted(),
                "verdict diverged for {value:?}"
            );
        }
    }

    #[test]
    fn span_mask_leaves_every_allowed_value_byte_identical() {
        for value in ALLOWED_CORPUS {
            let masked = mask_secret_spans(value, mark);
            assert_eq!(masked.text, *value);
            assert_eq!(masked.spans, 0);
        }
    }

    #[rstest]
    #[case(
        "before eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIn0.signaturedata123456 after",
        "before <jwt> after"
    )]
    #[case(
        "header Bearer abc123def456ghi789jkl sent\nnext line",
        "header <bearer>\nnext line"
    )]
    #[case(
        "cfg api_key=abcdef1234567890 loaded\nnext line",
        "cfg <api_key>\nnext line"
    )]
    #[case("login password=hunter2 ok\nnext line", "login <secret_kv>\nnext line")]
    #[case(
        "using sk_live_51NotARealKeyOnlyForPulseTests00 now", // gitleaks:allow
        "using <provider_key> now"
    )]
    #[case("user bob@example.com signed up", "user <email> signed up")]
    #[case("paid with 4111 1111 1111 1111 today", "paid with <credit_card> today")]
    #[case("ssn 123-45-6789 on file", "ssn <ssn> on file")]
    fn span_mask_masks_one_embedded_canary_per_category(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let masked = mask_secret_spans(input, mark);
        assert!(masked.redacted);
        assert_eq!(masked.text, expected);
    }

    #[test]
    fn span_mask_keyed_extent_runs_to_the_end_of_the_line() {
        let masked = mask_secret_spans(
            "retry for db password: hunter 2 (host=a)\nsecond line stays",
            mark,
        );
        assert_eq!(masked.text, "retry for db <secret_kv>\nsecond line stays");
    }

    #[test]
    fn span_mask_keyed_extent_covers_a_base64_bearer_tail() {
        let masked = mask_secret_spans("auth Bearer abcdefghijklmnop+qrs/tuv== ok\r\nnext", mark);
        assert_eq!(masked.text, "auth <bearer>\r\nnext");
    }

    #[test]
    fn span_mask_keyed_match_crossing_a_newline_ends_at_the_value_line() {
        let masked = mask_secret_spans("password:\nhunter2 tail\nkept", mark);
        assert_eq!(masked.text, "<secret_kv>\nkept");
    }

    #[test]
    fn span_mask_bare_extent_runs_to_the_whitespace_boundaries() {
        let masked = mask_secret_spans(
            "key rotated id=sk_live_51NotARealKeyOnlyForPulseTests00; retrying", // gitleaks:allow
            mark,
        );
        assert_eq!(masked.text, "key rotated <provider_key> retrying");
    }

    #[test]
    fn span_mask_merges_an_api_key_and_its_provider_value_under_api_key() {
        let masked = mask_secret_spans("api_key=sk-proj-1234567890abcdef", mark);
        assert_eq!(masked.text, "<api_key>");
        assert_eq!(masked.spans, 1);
    }

    #[test]
    fn span_mask_merges_a_token_bearer_overlap_under_bearer() {
        let masked = mask_secret_spans(
            "AuthError: token Bearer abc123def456ghi789jkl rejected",
            mark,
        );
        assert_eq!(masked.text, "AuthError: <bearer>");
        assert_eq!(masked.spans, 1);
    }

    #[test]
    fn span_mask_keeps_separate_secrets_as_separate_placeholders() {
        let masked = mask_secret_spans("a@b.co then 123-45-6789 then c@d.co", mark);
        assert_eq!(masked.text, "<email> then <ssn> then <email>");
        assert_eq!(masked.spans, 3);
    }

    #[rstest]
    #[case("sk_live_51NotARealKeyOnlyForPulseTests00", "provider_key")] // gitleaks:allow
    #[case("AKIANOTAREALKEYID000", "provider_key")] // gitleaks:allow
    #[case("123-45-6789", "ssn")]
    #[case("4532-0151-1283-0366", "credit_card")]
    #[case("bob@example.com", "email")]
    #[case("password=hunter2", "secret_kv")]
    fn span_mask_single_token_value_masks_exactly_as_whole_value_replacement(
        #[case] input: &str,
        #[case] category: &'static str,
    ) {
        assert_eq!(mask_secret_spans(input, mark).text, mark(category));
    }

    #[test]
    fn span_mask_never_leaves_a_placeholder_that_any_arm_matches() {
        for arm in patterns() {
            for placeholder in [
                format!("[REDACTED:{}]", arm.category),
                format!("[redacted:{}]", arm.category),
                format!("[redacted: {}]", arm.category),
            ] {
                assert!(
                    !scrub_attribute(&placeholder).is_redacted(),
                    "placeholder {placeholder:?} re-matches the catalog"
                );
            }
        }
    }

    #[test]
    fn span_mask_is_idempotent_over_the_recall_corpus() {
        for value in RECALL_CORPUS {
            let once = mask_secret_spans(value, mark).text;
            let twice = mask_secret_spans(&once, mark).text;
            assert_eq!(twice, once, "re-masking changed {value:?}");
        }
    }

    #[test]
    fn span_mask_reaches_a_fixpoint_when_a_mask_exposes_a_fused_digit_run() {
        let once = mask_secret_spans("4111111111111111password=x", mark).text;
        assert_eq!(mask_secret_spans(&once, mark).text, once);
        assert!(!once.contains("4111111111111111"));
    }

    #[test]
    fn span_mask_handles_multibyte_text_around_a_secret() {
        let masked = mask_secret_spans("ошибка для bob@example.com — повтор", mark);
        assert_eq!(masked.text, "ошибка для <email> — повтор");
    }

    fn secret_or_word() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("sk_live_51NotARealKeyOnlyForPulseTests00".to_string()), // gitleaks:allow
            Just("bob@example.com".to_string()),
            Just("4111 1111 1111 1111".to_string()),
            Just("123-45-6789".to_string()),
            Just("password".to_string()),
            Just("api_key".to_string()),
            Just("Bearer".to_string()),
            Just("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIn0.sig123".to_string()),
            "[a-z0-9]{1,8}",
            Just(" ".to_string()),
            Just("\n".to_string()),
            Just("=".to_string()),
            Just(":".to_string()),
        ]
    }

    proptest! {
        #[test]
        fn span_mask_verdict_matches_scrub_attribute_for_any_text(s in ".{0,200}") {
            prop_assert_eq!(
                mask_secret_spans(&s, mark).redacted,
                scrub_attribute(&s).is_redacted()
            );
        }

        #[test]
        fn span_mask_verdict_matches_scrub_attribute_for_secret_mixes(
            parts in prop::collection::vec(secret_or_word(), 0..24)
        ) {
            let s = parts.concat();
            prop_assert_eq!(
                mask_secret_spans(&s, mark).redacted,
                scrub_attribute(&s).is_redacted()
            );
        }

        #[test]
        fn span_mask_is_idempotent_for_secret_mixes(
            parts in prop::collection::vec(secret_or_word(), 0..24)
        ) {
            let once = mask_secret_spans(&parts.concat(), mark).text;
            let twice = mask_secret_spans(&once, mark).text;
            prop_assert_eq!(twice, once);
        }

        #[test]
        fn span_mask_is_idempotent_for_any_text(s in ".{0,200}") {
            let once = mask_secret_spans(&s, mark).text;
            let twice = mask_secret_spans(&once, mark).text;
            prop_assert_eq!(twice, once);
        }
    }
}
