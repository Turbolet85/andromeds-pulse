//! Resolver probe for the close-to-tray signpost leaf (`tray.signpost.shown`).
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! The emit site (`pulse-app/src/window.rs::maybe_show_close_signpost`) is the
//! authority for this field list. There is no bare `tray` key, so `for_target`
//! has no prefix fallback for this target: without its EXACT leaf the resolver
//! returns None and the field is silently redacted — which would leave the
//! webview-drive widget-close stage asserting bare record presence, the exact
//! shape test-plan §6's effect-FIELD rule disallows.

use pulse_app::observability::AllowList;

const SIGNPOST: &str = "tray.signpost.shown";
const LABEL_FIELD: &str = "window_label";

#[test]
fn signpost_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(SIGNPOST).is_some(),
        "`{SIGNPOST}` must resolve via its own exact leaf — no bare `tray` prefix key exists",
    );
}

#[test]
fn signpost_leaf_carries_exactly_the_emitted_field_set() {
    // The emit site emits ONE field. Assert the whole set both ways so a
    // narrowed leaf (field dropped → redacted in production) AND a widened
    // leaf (fields the emit site never sends) both fail here.
    let al = AllowList::production();
    let set = al
        .for_target(SIGNPOST)
        .expect("tray.signpost.shown must resolve to an allowlist entry");

    assert!(
        set.contains(LABEL_FIELD),
        "`{LABEL_FIELD}` is emitted at {SIGNPOST} and must not be redacted",
    );
    assert_eq!(
        set.len(),
        1,
        "the {SIGNPOST} leaf must enumerate exactly the emit site's field list",
    );
}

#[test]
fn signpost_never_admits_content_or_coordinate_fields() {
    // The signpost is a boundary event. The notification body is a static
    // string that never reaches tracing (security-plan §Logging NEVER-log),
    // and window telemetry must not carry coordinate values (security-plan
    // §Input Validation → Persisted window geometry).
    let al = AllowList::production();
    let set = al
        .for_target(SIGNPOST)
        .expect("tray.signpost.shown must resolve to an allowlist entry");

    for banned in [
        "notification_body",
        "body",
        "message",
        "title",
        "x",
        "y",
        "width",
        "height",
        "position",
    ] {
        assert!(
            !set.contains(banned),
            "`{banned}` must never be permitted on {SIGNPOST}",
        );
    }
}
