//! Resolver probe for the process-end leaf (`app.exit`).
//!
//! Lives under `pulse-app/tests/` because `[lib] test = false` means a
//! src-level `mod tests` in `pulse-app` compiles and never runs.
//!
//! The emit site (`pulse-app/src/observability.rs::record_exit`) is the
//! authority for this field list. The target is `app.`-prefixed and NO bare
//! `app` key is registered, so without its EXACT leaf `for_target`'s
//! first-`.`-segment fallback resolves nothing and every field is redacted -
//! the record would name no exit class at all.

use pulse_app::observability::AllowList;

const APP_EXIT: &str = "app.exit";
const FIELDS: [&str; 4] = ["exit_class", "exit_code", "exit_code_known", "signal"];

#[test]
fn app_exit_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(APP_EXIT).is_some(),
        "`{APP_EXIT}` must resolve - without its own leaf every field is redacted",
    );
}

#[test]
fn app_exit_leaf_carries_exactly_the_emitted_field_set() {
    // Both ways: a narrowed leaf (a field redacted in production) and a
    // widened one (a field the emit site never sends) must both fail.
    let al = AllowList::production();
    let set = al
        .for_target(APP_EXIT)
        .expect("app.exit must resolve to an allowlist entry");
    for field in FIELDS {
        assert!(
            set.contains(field),
            "`{field}` is emitted at {APP_EXIT} and must not be redacted",
        );
    }
    assert_eq!(
        set.len(),
        FIELDS.len(),
        "the {APP_EXIT} leaf must enumerate exactly the emit site's field list",
    );
}

#[test]
fn app_exit_never_admits_paths_or_payload_text() {
    // A panic payload, an error Display or a native message is free text that
    // can carry host-app secrets; the record is a closed enum by construction.
    let al = AllowList::production();
    let set = al
        .for_target(APP_EXIT)
        .expect("app.exit must resolve to an allowlist entry");
    for banned in [
        "path",
        "data_dir",
        "log_dir",
        "exe_path",
        "panic_message",
        "error",
        "error_msg",
        "reason",
        "detail",
        "payload",
    ] {
        assert!(
            !set.contains(banned),
            "`{banned}` must never be admitted at {APP_EXIT}",
        );
    }
    assert!(
        !set.iter().any(|f| f.ends_with("_path")),
        "no `*_path` field may be admitted at {APP_EXIT}",
    );
}

#[test]
fn app_exit_has_no_bare_app_fallback() {
    // The discriminator: with no bare `app` key, deleting the exact leaf
    // leaves `app.exit` resolving to nothing, so the exact-resolve pin above
    // cannot pass vacuously through a fallback set.
    let al = AllowList::production();
    assert!(
        al.for_target("app").is_none(),
        "a bare `app` key would widen every `app.*` target without its own leaf",
    );
}
