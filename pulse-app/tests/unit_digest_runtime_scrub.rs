// Migrated 2026-08-30 from `pulse-app/src/digest_runtime.rs::tests` — that
// crate sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS.

use pulse_app::digest_runtime::pii_scrub_closure;

#[test]
fn pii_scrub_closure_redacts_jwt() {
    let scrub = pii_scrub_closure();
    let input = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIn0.signaturedata123456";
    let out = scrub(input);
    assert!(out.starts_with("[redacted:"), "expected redaction: {out}");
}

#[test]
fn pii_scrub_closure_allows_clean_text() {
    let scrub = pii_scrub_closure();
    let out = scrub("hello world");
    assert_eq!(out, "hello world");
}
