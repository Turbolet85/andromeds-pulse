//! Resolver + field-set pins for the capability-rejected IPC leaf
//! (`ui.ipc.rejection`, chunk 2026-08-30-acl-rejection-logging).
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level
//! tests in `pulse-app` compile but never run — a probe that cannot fail is
//! not a guard.
//!
//! The emit site (`crates/ui-bridge/src/telemetry.rs::record_ipc_rejection`)
//! is the authority for this field list. The target is `ui.`-prefixed and NO
//! bare `ui` key is registered (measured), so without its EXACT leaf
//! `for_target`'s first-`.`-segment fallback resolves nothing and every field
//! is silently redacted — which would make the security-plan §Logging &
//! Monitoring "capability-rejected IPC calls" record vanish exactly the way
//! the silent rejection it exists to expose does.

use pulse_app::observability::AllowList;

const REJECTION: &str = "ui.ipc.rejection";
const FIELDS: [&str; 3] = ["error_category", "window_label", "payload_bytes"];

#[test]
fn ipc_rejection_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(REJECTION).is_some(),
        "`{REJECTION}` must resolve — with no bare `ui` key the fallback redacts everything",
    );
}

#[test]
fn ipc_rejection_leaf_carries_exactly_the_emitted_field_set() {
    // Assert the set both ways: a narrowed leaf (a field dropped → redacted in
    // production) and a widened one (fields the emit site never sends) must
    // both fail here.
    let al = AllowList::production();
    let set = al
        .for_target(REJECTION)
        .expect("ui.ipc.rejection must resolve to an allowlist entry");

    for field in FIELDS {
        assert!(
            set.contains(field),
            "`{field}` is emitted at {REJECTION} and must not be redacted",
        );
    }
    assert_eq!(
        set.len(),
        FIELDS.len(),
        "the {REJECTION} leaf must enumerate exactly the emit site's field list",
    );
}

#[test]
fn ipc_rejection_never_admits_payload_or_command_content() {
    // The record exists BECAUSE the rejected call carried user content
    // (clipboard payload); the payload, the command string, and the raw
    // rejection message are the fields most likely to be added "for
    // debuggability" and are exactly what must never ship (security-plan
    // §Security Anti-Patterns → Logging).
    let al = AllowList::production();
    let set = al
        .for_target(REJECTION)
        .expect("ui.ipc.rejection must resolve to an allowlist entry");

    for banned in [
        "payload",
        "text",
        "content",
        "command",
        "cmd",
        "message_text",
        "rejection_message",
        "error_msg",
    ] {
        assert!(
            !set.contains(banned),
            "`{banned}` must never be admitted at {REJECTION}",
        );
    }
}

#[test]
fn no_bare_ui_key_can_serve_dotted_ui_targets() {
    // The fallback discriminator: `for_target`'s first-`.`-segment fallback
    // must have nothing to resolve `ui.*` targets to. A future bare `ui` key
    // would silently widen every unlisted `ui.*` sibling to one field set —
    // the prefix-widening trap the corpus/triage/interpretation families are
    // guarded against. A leaf-less sibling resolving to None is the proof the
    // prefix cannot serve this family.
    let al = AllowList::production();
    assert!(
        al.for_target("ui").is_none(),
        "a bare `ui` allowlist key must not exist",
    );
    assert!(
        al.for_target("ui.ipc.some_unregistered_sibling").is_none(),
        "an unregistered ui.* target must resolve to NOTHING (full redaction), not a fallback set",
    );
}
