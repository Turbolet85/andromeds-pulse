//! Exception fingerprint computation + observer trait — chunk #66.
//!
//! L1c distillation layer per pulse v0.2.0 plan Phase 2: hash
//! `(exception.type + normalized first-3-frame stacktrace)` to a deterministic
//! 16-byte fingerprint that is BOTH written to the `span_events.fingerprint`
//! BLOB column (via `appender::build_span_events_record_batch`) AND fed to
//! `triage::pattern::storm::RetryStormDetector` for retry-storm pattern
//! detection (via the `FingerprintObserver` trait declared here).
//!
//! Per arch §Cross-cutting Patterns Module dependency direction, the
//! `FingerprintObserver` trait lives in this lower-tier crate (buffer is
//! the orchestrator of the per-row fingerprint compute); the
//! `StormObserverAdapter` impl lives at the `pulse-app` binary boundary
//! (depends on both buffer + triage). Mirrors chunks #59/#62/#63
//! trait-in-lower-crate + impl-at-pulse-app precedent.
//!
//! ## Hash discipline (security)
//!
//! Per security plan §Logging & Monitoring NEVER-log list + chunk #66 spec
//! "fingerprint hash discipline — must not leak content":
//!
//! - Hash input is `exception.type` (classification-grade per chunk #65
//!   redaction discipline) + the normalized first-3-frame stacktrace. The raw
//!   `exception.message` content NEVER enters the hash preimage — this
//!   guarantees that incidentally-captured secrets in `exception.message`
//!   cannot influence the broadcast fingerprint OR the DuckDB column bytes.
//! - Normalization strips absolute paths, memory addresses, and line numbers
//!   BEFORE hashing so the hash is stable across host-environment differences
//!   AND incidental path/address secrets do not co-mingle with the fingerprint.
//! - Output is 16 bytes (truncated `blake3` hash); ample collision-resistance
//!   for in-memory `DashMap<[u8; 16], _>` cardinality bounds.

/// 16-byte deterministic hash of an exception's identity. Computed from
/// `(exception.type + normalized first-3-frame stacktrace)`; same identity
/// → same fingerprint regardless of host-environment path/address/line-number
/// variation.
pub type ExceptionFingerprint = [u8; 16];

/// Per-span-event hook invoked once per `span_events` row whose
/// `exception.type` is non-empty. Implementations must be `Send + Sync` so
/// the buffer consumer task can hold an `Arc<dyn FingerprintObserver>`.
///
/// The `fingerprint` is the computed 16-byte hash; `service_name` is the
/// OTLP `service.name` resource attribute (identifier-class per chunk #61
/// service-identity discipline); `ts_unix_nano` is the span_event's
/// `time_unix_nano` (used by the storm detector's rolling-window state).
pub trait FingerprintObserver: Send + Sync {
    fn on_fingerprint(
        &self,
        fingerprint: ExceptionFingerprint,
        service_name: &str,
        ts_unix_nano: i64,
    );
}

/// No-op implementation used as default in tests and as a fallback at boot
/// when no storm detector is wired in.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopFingerprintObserver;

impl FingerprintObserver for NoopFingerprintObserver {
    fn on_fingerprint(
        &self,
        _fingerprint: ExceptionFingerprint,
        _service_name: &str,
        _ts_unix_nano: i64,
    ) {
    }
}

/// Compute the exception fingerprint for one span event. Returns `None` if
/// `exception_type` is `None` OR empty — non-exception span events get no
/// fingerprint (column stays NULL on the DuckDB side per chunk #65 substrate
/// semantics).
///
/// Hash preimage = `exception_type.as_bytes() ++ b'\0' ++ normalize_stacktrace(stack).as_bytes()`.
/// Null-byte separator prevents cross-field collision (e.g., `type="Ax"` +
/// `stack="B"` vs `type="A"` + `stack="xB"`).
pub fn compute_exception_fingerprint(
    exception_type: Option<&str>,
    stacktrace: Option<&str>,
) -> Option<ExceptionFingerprint> {
    let exc_type = exception_type?;
    if exc_type.is_empty() {
        return None;
    }
    let normalized_stack = normalize_stacktrace(stacktrace.unwrap_or(""));
    let mut hasher = blake3::Hasher::new();
    hasher.update(exc_type.as_bytes());
    hasher.update(b"\0");
    hasher.update(normalized_stack.as_bytes());
    let hash = hasher.finalize();
    let mut out = [0u8; 16];
    out.copy_from_slice(&hash.as_bytes()[..16]);
    Some(out)
}

/// Format the fingerprint's first 4 bytes as 8-char lowercase hex for use as
/// a bounded-cardinality tracing field value. Mirrors the `cue_kind_label`
/// pattern from `triage::cue::classify` — opaque-but-stable identifier safe
/// to log in self-observation events.
pub fn fingerprint_to_hex_prefix(fingerprint: &ExceptionFingerprint) -> String {
    let mut out = String::with_capacity(8);
    for byte in &fingerprint[..4] {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// Normalize a stacktrace string for fingerprint stability across hosts:
/// take the first 3 non-empty lines; for each line, strip path-shaped tokens,
/// hex memory addresses, and `:line(:col)?` suffixes; join with `\n`.
///
/// The first-3-frame truncation per chunk #66 spec keeps the fingerprint
/// stable when callers call the same function from different deep stack
/// contexts — same proximate failure surface = same fingerprint regardless
/// of how deep the rest of the stack goes.
pub(crate) fn normalize_stacktrace(raw: &str) -> String {
    let mut normalized_frames: Vec<String> = Vec::with_capacity(3);
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        normalized_frames.push(normalize_frame(trimmed));
        if normalized_frames.len() >= 3 {
            break;
        }
    }
    normalized_frames.join("\n")
}

/// Strip variable parts of a single stack frame line:
/// - absolute file paths (Unix `/...` or Windows `C:\...`) — ONLY absolute ones;
///   a relative path such as `src/a.rs` is identity-significant and is preserved
///   in full (see `is_token_boundary`)
/// - hex memory addresses (`0x...`)
/// - line/column suffixes after a colon-anchored path remnant (`:42` or `:42:7`)
///
/// Hand-rolled rather than regex-based: simple linear scan, no new dep.
fn normalize_frame(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];

        if is_hex_address_start(bytes, i) {
            i = skip_hex_address(bytes, i);
            continue;
        }

        if is_absolute_path_start(bytes, i) {
            i = skip_absolute_path(bytes, i);
            continue;
        }

        if b == b':' && is_line_number_suffix(bytes, i + 1) {
            i = skip_line_number_suffix(bytes, i);
            continue;
        }

        out.push(b as char);
        i += 1;
    }

    while out.ends_with(' ') {
        out.pop();
    }
    out
}

fn is_hex_address_start(bytes: &[u8], i: usize) -> bool {
    i + 2 < bytes.len()
        && bytes[i] == b'0'
        && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X')
        && bytes[i + 2].is_ascii_hexdigit()
}

fn skip_hex_address(bytes: &[u8], mut i: usize) -> usize {
    i += 2;
    while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
        i += 1;
    }
    i
}

/// A path token counts as ABSOLUTE only when it STARTS a token (position 0, or
/// preceded by a non-path byte). Without this precondition the Unix arm fires on
/// every `/`, and since `is_path_char` itself admits `/`, `skip_absolute_path`
/// then swallows the remainder of the token so only the leading segment survives
/// — making `src/a.rs` and `src/b/c.rs` one fingerprint. Relative path structure
/// is identity-significant.
fn is_token_boundary(bytes: &[u8], i: usize) -> bool {
    i == 0 || !is_path_char(bytes[i - 1])
}

fn is_absolute_path_start(bytes: &[u8], i: usize) -> bool {
    if !is_token_boundary(bytes, i) {
        return false;
    }
    if i < bytes.len() && bytes[i] == b'/' && i + 1 < bytes.len() && is_path_char(bytes[i + 1]) {
        return true;
    }
    if i + 2 < bytes.len()
        && bytes[i].is_ascii_alphabetic()
        && bytes[i + 1] == b':'
        && (bytes[i + 2] == b'\\' || bytes[i + 2] == b'/')
    {
        return true;
    }
    false
}

fn skip_absolute_path(bytes: &[u8], mut i: usize) -> usize {
    if i + 2 < bytes.len()
        && bytes[i].is_ascii_alphabetic()
        && bytes[i + 1] == b':'
        && (bytes[i + 2] == b'\\' || bytes[i + 2] == b'/')
    {
        i += 3;
    } else {
        i += 1;
    }
    while i < bytes.len() && is_path_char(bytes[i]) {
        i += 1;
    }
    i
}

fn is_path_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'/' | b'\\' | b'_' | b'-' | b'.' | b'~')
}

fn is_line_number_suffix(bytes: &[u8], i: usize) -> bool {
    i < bytes.len() && bytes[i].is_ascii_digit()
}

fn skip_line_number_suffix(bytes: &[u8], mut i: usize) -> usize {
    i += 1;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b':' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::sync::Arc;

    const SAMPLE_TYPE: &str = "java.lang.RuntimeException";
    const SAMPLE_STACK: &str = "    at com.example.Foo.bar(Foo.java:42)\n    at com.example.Foo.baz(Foo.java:50)\n    at com.example.Foo.qux(Foo.java:58)";

    #[test]
    fn compute_returns_none_for_missing_exception_type() {
        assert!(compute_exception_fingerprint(None, Some(SAMPLE_STACK)).is_none());
    }

    #[test]
    fn compute_returns_none_for_empty_exception_type() {
        assert!(compute_exception_fingerprint(Some(""), Some(SAMPLE_STACK)).is_none());
    }

    #[test]
    fn compute_is_deterministic_across_invocations() {
        let a = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(SAMPLE_STACK))
            .expect("fingerprint computed");
        let b = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(SAMPLE_STACK))
            .expect("fingerprint computed");
        assert_eq!(a, b);
    }

    #[test]
    fn compute_with_no_stacktrace_still_produces_fingerprint() {
        let fp = compute_exception_fingerprint(Some(SAMPLE_TYPE), None)
            .expect("type alone yields fingerprint");
        assert_eq!(fp.len(), 16);
    }

    #[test]
    fn compute_with_empty_stacktrace_string_equals_compute_with_none() {
        let a = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some("")).expect("ok");
        let b = compute_exception_fingerprint(Some(SAMPLE_TYPE), None).expect("ok");
        assert_eq!(a, b);
    }

    #[rstest]
    #[case(
        "    at com.example.Foo.bar(/abs/path/Foo.java:42)",
        "    at com.example.Foo.bar(/different/path/Foo.java:99)"
    )]
    #[case(
        "    at com.example.Foo.bar(C:\\abs\\path\\Foo.java:42)",
        "    at com.example.Foo.bar(C:\\diff\\path\\Foo.java:99)"
    )]
    fn compute_strips_paths_and_line_numbers(#[case] stack_a: &str, #[case] stack_b: &str) {
        let a = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_a))
            .expect("fingerprint computed");
        let b = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_b))
            .expect("fingerprint computed");
        assert_eq!(
            a, b,
            "same type+frame-function-names with different paths/line numbers MUST hash to same fingerprint"
        );
    }

    #[test]
    fn compute_strips_hex_memory_addresses() {
        let stack_a = "    at com.example.Foo.bar @ 0xdeadbeef";
        let stack_b = "    at com.example.Foo.bar @ 0xcafebabe";
        let a = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_a)).expect("ok");
        let b = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_b)).expect("ok");
        assert_eq!(a, b, "addresses MUST be stripped pre-hash");
    }

    #[test]
    fn compute_strips_line_column_suffixes() {
        let stack_a = "    at com.example.Foo.bar(Foo.java:42:7)";
        let stack_b = "    at com.example.Foo.bar(Foo.java:999:42)";
        let a = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_a)).expect("ok");
        let b = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_b)).expect("ok");
        assert_eq!(a, b, "line:col suffixes MUST be stripped pre-hash");
    }

    #[test]
    fn compute_truncates_to_first_three_frames() {
        let three_frames =
            "    at com.example.Foo.bar\n    at com.example.Foo.baz\n    at com.example.Foo.qux";
        let four_frames = "    at com.example.Foo.bar\n    at com.example.Foo.baz\n    at com.example.Foo.qux\n    at com.example.Foo.different";
        let a = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(three_frames)).expect("ok");
        let b = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(four_frames)).expect("ok");
        assert_eq!(
            a, b,
            "4-frame stack differing only after frame 3 MUST yield same fingerprint as the 3-frame prefix"
        );
    }

    #[test]
    fn compute_differs_when_exception_type_differs() {
        let a =
            compute_exception_fingerprint(Some("java.lang.RuntimeException"), Some(SAMPLE_STACK))
                .expect("ok");
        let b = compute_exception_fingerprint(
            Some("java.lang.IllegalArgumentException"),
            Some(SAMPLE_STACK),
        )
        .expect("ok");
        assert_ne!(a, b, "different types MUST produce different fingerprints");
    }

    #[test]
    fn compute_differs_when_first_three_frames_differ() {
        let stack_a = "    at A.first\n    at A.second\n    at A.third";
        let stack_b = "    at B.first\n    at A.second\n    at A.third";
        let a = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_a)).expect("ok");
        let b = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_b)).expect("ok");
        assert_ne!(
            a, b,
            "different first-frame function name MUST produce different fingerprints"
        );
    }

    #[test]
    fn compute_does_not_include_exception_message_in_preimage() {
        let fp_canary =
            compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(SAMPLE_STACK)).expect("ok");
        let fp_clean =
            compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(SAMPLE_STACK)).expect("ok");
        assert_eq!(
            fp_canary, fp_clean,
            "fingerprint preimage MUST be (type + normalized stack) only; message NEVER feeds into hash"
        );
    }

    #[test]
    fn normalize_stacktrace_handles_empty_input() {
        assert_eq!(normalize_stacktrace(""), "");
    }

    #[test]
    fn normalize_stacktrace_skips_empty_lines() {
        let raw = "    at com.example.Foo.bar\n\n    at com.example.Foo.baz";
        let normalized = normalize_stacktrace(raw);
        assert_eq!(normalized.lines().count(), 2);
    }

    #[test]
    fn compute_differs_for_relative_paths_differing_below_leading_segment() {
        let stack_a = "    at handler(src/a.rs:10)";
        let stack_b = "    at handler(src/b/c.rs:10)";
        let a = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_a)).expect("ok");
        let b = compute_exception_fingerprint(Some(SAMPLE_TYPE), Some(stack_b)).expect("ok");
        assert_ne!(
            a, b,
            "relative path structure is identity-significant — src/a.rs and src/b/c.rs are distinct faults"
        );
    }

    #[test]
    fn normalize_stacktrace_preserves_relative_paths_in_full() {
        assert_eq!(
            normalize_stacktrace("    at handler(src/b/c.rs:10)"),
            "at handler(src/b/c.rs)"
        );
    }

    #[rstest]
    #[case("/usr/lib/thing.rs", "")]
    #[case("    at handler(/usr/lib/thing.rs:10)", "at handler()")]
    #[case("    at handler(C:\\proj\\thing.rs:10)", "at handler()")]
    fn normalize_stacktrace_strips_only_absolute_paths(#[case] raw: &str, #[case] expected: &str) {
        assert_eq!(normalize_stacktrace(raw), expected);
    }

    #[test]
    fn fingerprint_to_hex_prefix_yields_8_lowercase_hex_chars() {
        let fp: ExceptionFingerprint = [
            0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE, 0xFA, 0xCE, 0xFE, 0xED, 0xC0, 0xDE,
            0x00, 0xFF,
        ];
        let hex = fingerprint_to_hex_prefix(&fp);
        assert_eq!(hex, "deadbeef");
    }

    #[test]
    fn noop_observer_safe_to_call() {
        let observer = NoopFingerprintObserver;
        observer.on_fingerprint([0; 16], "svc", 1_000_000_000);
    }

    #[test]
    fn observer_trait_object_callable() {
        let observer: Arc<dyn FingerprintObserver> = Arc::new(NoopFingerprintObserver);
        observer.on_fingerprint([0; 16], "svc", 1_000_000_000);
    }
}
