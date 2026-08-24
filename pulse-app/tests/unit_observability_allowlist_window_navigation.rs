//! Resolver probe for the boot navigation-check leaf (`app.boot.window.navigation`).
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! The emit site (`pulse-app/src/window.rs::spawn_navigation_check`) is the
//! authority for this field list. The target is `app.`-prefixed, and NO bare
//! `app` key is registered (measured), so without its EXACT leaf
//! `for_target`'s first-`.`-segment fallback resolves nothing and every field
//! is silently redacted — the same trap already recorded for
//! `app.boot.workspace_key` and `app.boot.buffer.degraded`. A redacted
//! `navigated` field would make the plain-boot measurement this record exists
//! to enable read as all-zeroes.

use pulse_app::observability::AllowList;

const NAVIGATION: &str = "app.boot.window.navigation";
const FIELDS: [&str; 3] = ["window_label", "navigated", "reason"];

#[test]
fn navigation_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(NAVIGATION).is_some(),
        "`{NAVIGATION}` must resolve — without its own leaf the `app` fallback redacts it",
    );
}

#[test]
fn navigation_leaf_carries_exactly_the_emitted_field_set() {
    // Assert the set both ways: a narrowed leaf (a field dropped → redacted in
    // production) and a widened one (fields the emit site never sends) must
    // both fail here.
    let al = AllowList::production();
    let set = al
        .for_target(NAVIGATION)
        .expect("app.boot.window.navigation must resolve to an allowlist entry");

    for field in FIELDS {
        assert!(
            set.contains(field),
            "`{field}` is emitted at {NAVIGATION} and must not be redacted",
        );
    }
    assert_eq!(
        set.len(),
        FIELDS.len(),
        "the {NAVIGATION} leaf must enumerate exactly the emit site's field list",
    );
}

#[test]
fn navigation_never_admits_the_url_or_window_content() {
    // The whole point of the check is a URL comparison, so the URL is the field
    // most likely to be added for "debuggability" — and it is exactly what must
    // not ship. obs-plan §8 bans window content/title/coordinates on window
    // telemetry, and a webview URL can carry query state from the host app.
    let al = AllowList::production();
    let set = al
        .for_target(NAVIGATION)
        .expect("app.boot.window.navigation must resolve to an allowlist entry");

    for banned in [
        "url",
        "webview_url",
        "title",
        "window_title",
        "x",
        "y",
        "path",
    ] {
        assert!(
            !set.contains(banned),
            "`{banned}` must never be admitted at {NAVIGATION}",
        );
    }
}

#[test]
fn navigation_does_not_leak_into_the_bare_app_fallback() {
    // If a bare `app` key ever carried these fields, every future `app.*`
    // target would inherit them — the prefix-widening failure the corpus and
    // triage families are guarded against.
    let al = AllowList::production();
    if let Some(app_set) = al.for_target("app") {
        for field in FIELDS {
            assert!(
                !app_set.contains(field),
                "`{field}` must live on the exact {NAVIGATION} leaf, not the bare `app` key",
            );
        }
    }
}
