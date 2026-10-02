//! Resolver probes for the cold-start-window override notice.
//!
//! These live here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! The emit site (`crates/triage/src/cue/thresholds.rs::warn_bootstrap_window`)
//! is the authority for this list: it emits three fields, so a two-field leaf
//! would leave the target partly redacted while every gate still passed.

use pulse_app::observability::AllowList;

const TARGET: &str = "triage.baseline.bootstrap_window.override";

#[test]
fn bootstrap_window_override_allows_every_field_its_emit_site_emits() {
    let al = AllowList::production();
    let set = al
        .for_target(TARGET)
        .expect("triage.baseline.bootstrap_window.override must have an explicit leaf entry");

    for required in ["resolved_seconds", "default_seconds", "reason"] {
        assert!(
            set.contains(required),
            "`{required}` is emitted by warn_bootstrap_window and must not be redacted",
        );
    }
}

#[test]
fn bootstrap_window_override_admits_no_per_service_identifier() {
    let al = AllowList::production();
    let set = al.for_target(TARGET).expect("leaf entry");

    for banned in [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
    ] {
        assert!(
            !set.contains(banned),
            "`{banned}` must stay out of a self-observation event (chunk #62/#63 PII discipline)",
        );
    }
}

#[test]
fn no_bare_triage_prefix_key_shadows_the_leaf() {
    // `for_target` falls back to the first dotted segment, so a bare `triage`
    // key would resolve every `triage.*` target to one field set and silently
    // widen every sibling.
    let al = AllowList::production();
    assert!(
        al.for_target("triage").is_none(),
        "a bare `triage` prefix key must not exist",
    );
    assert!(
        al.for_target("triage.baseline").is_none(),
        "a bare `triage.baseline` prefix key must not exist",
    );
}
