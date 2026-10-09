//! Resolver probes for `interpretation.incident.skipped` — the record a
//! cleanly-parsed L4 generation leaves when it creates no incident.
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level
//! tests in `pulse-app` compile but never run.
//!
//! The emit site (`pulse-app/src/inference_runtime.rs::emit_incident_skipped`)
//! is the authority for the field list.

use std::collections::BTreeSet;

use pulse_app::inference_runtime::TARGET_L4_INCIDENT_SKIPPED;
use pulse_app::observability::AllowList;

const SKIP_FIELDS: [&str; 4] = ["skip_reason", "decision", "severity", "digest_kind"];

#[test]
fn incident_skip_target_resolves_to_an_exact_leaf() {
    assert_eq!(
        TARGET_L4_INCIDENT_SKIPPED,
        "interpretation.incident.skipped"
    );
    let al = AllowList::production();
    assert!(
        al.for_target(TARGET_L4_INCIDENT_SKIPPED).is_some(),
        "`{TARGET_L4_INCIDENT_SKIPPED}` must resolve to an allowlist entry",
    );
}

#[test]
fn incident_skip_leaf_equals_the_emitted_field_set_in_both_directions() {
    let al = AllowList::production();
    let set = al
        .for_target(TARGET_L4_INCIDENT_SKIPPED)
        .expect("interpretation.incident.skipped must resolve to an allowlist entry");
    for field in SKIP_FIELDS {
        assert!(
            set.contains(field),
            "`{TARGET_L4_INCIDENT_SKIPPED}` leaf is missing `{field}` — it would be redacted in production",
        );
    }
    let expected: BTreeSet<&str> = SKIP_FIELDS.into_iter().collect();
    for field in set.iter() {
        assert!(
            expected.contains(field),
            "`{TARGET_L4_INCIDENT_SKIPPED}` leaf carries `{field}`, which the emit site does not send",
        );
    }
    assert_eq!(set.len(), SKIP_FIELDS.len());
}

#[test]
fn incident_skip_leaf_admits_no_banned_field() {
    let al = AllowList::production();
    let set = al
        .for_target(TARGET_L4_INCIDENT_SKIPPED)
        .expect("interpretation.incident.skipped must resolve to an allowlist entry");
    for banned in ["scope_id", "title", "symptom", "payload_summary"] {
        assert!(
            !set.contains(banned),
            "`{TARGET_L4_INCIDENT_SKIPPED}` must never admit `{banned}` (obs-plan section 8 default-deny)",
        );
    }
}

/// The discriminator: with no bare `interpretation` key, a deleted exact
/// leaf resolves NOTHING, so the probes above cannot pass against a fallback.
#[test]
fn incident_skip_has_no_interpretation_fallback() {
    let al = AllowList::production();
    assert!(
        al.for_target("interpretation").is_none(),
        "a bare `interpretation` key would let a deleted skip leaf resolve to the wrong set",
    );
}
