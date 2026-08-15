//! Resolver probes for the incident-path observability surface.
//!
//! These live here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! All three targets emitted with their fields redacted, so the diagnostics
//! that would have shown the incident path's state were themselves mute: the
//! storm→incident classification had to read `corpus.db` out-of-band to learn
//! how many incidents existed.

use pulse_app::observability::AllowList;

const INCIDENT_TARGETS: &[&str] = &[
    "incidents.list_active.request",
    "triage.incident.persist",
    "triage.incident.corpus_restore",
];

#[test]
fn list_active_request_allows_its_item_count() {
    let al = AllowList::production();
    let set = al
        .for_target("incidents.list_active.request")
        .expect("incidents.list_active.request must have an explicit leaf entry");

    assert!(
        set.contains("item_count"),
        "the active-incident count must not be redacted"
    );
}

#[test]
fn incident_persist_allows_its_counts_and_bounded_kind() {
    let al = AllowList::production();
    let set = al
        .for_target("triage.incident.persist")
        .expect("triage.incident.persist must have an explicit leaf entry");

    // `duration_ms` is emitted alongside the other two at the emit site
    // (`crates/triage/src/incident/persistence.rs`); omitting it would leave the
    // target partly redacted, which is the defect this entry exists to close.
    for field in ["incident_count", "persist_kind", "duration_ms"] {
        assert!(
            set.contains(field),
            "incident persist cycle must allow `{field}`"
        );
    }
}

#[test]
fn incident_corpus_restore_allows_its_kind_and_restored_count() {
    let al = AllowList::production();
    let set = al
        .for_target("triage.incident.corpus_restore")
        .expect("triage.incident.corpus_restore must have an explicit leaf entry");

    for field in ["kind", "restored_incident_count"] {
        assert!(
            set.contains(field),
            "boot-time incident restore must allow `{field}`"
        );
    }
}

/// The `for_target` resolver falls back to the dotted prefix, so a bare
/// `incidents` or `triage` key would silently widen every present and future
/// sibling target to one field set. Neither may exist.
#[test]
fn no_widening_prefix_key_shadows_the_incident_leaves() {
    let al = AllowList::production();

    for prefix in ["incidents", "triage", "triage.incident"] {
        assert!(
            al.for_target(prefix).is_none(),
            "a bare `{prefix}` allowlist key would widen every sibling target"
        );
    }
}

/// Each target must resolve to its OWN entry, not to a neighbour's via the
/// prefix fallback — the trap that muted these fields in the first place.
#[test]
fn every_incident_target_resolves_to_a_distinct_exact_leaf() {
    let al = AllowList::production();

    for target in INCIDENT_TARGETS {
        let set = al
            .for_target(target)
            .unwrap_or_else(|| panic!("`{target}` must resolve to an exact leaf"));
        assert!(
            !set.is_empty(),
            "`{target}` resolved to an empty field set — it would still redact everything"
        );
    }
}

/// Aggregate counts and bounded labels only — never incident identity, never
/// the workspace string, never payload content.
#[test]
fn incident_targets_never_allow_identity_or_workspace_fields() {
    let al = AllowList::production();

    for target in INCIDENT_TARGETS {
        let set = al.for_target(target).expect("leaf entry present");
        for banned in [
            "workspace",
            "incident_id",
            "scope_id",
            "service_name",
            "title",
            "detail",
            "payload",
            "fingerprint",
        ] {
            assert!(
                !set.contains(banned),
                "`{target}` must not allow `{banned}` — aggregate counts and bounded labels only"
            );
        }
    }
}
