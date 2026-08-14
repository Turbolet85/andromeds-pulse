//! Resolver probes for the fingerprint-feed observability surface.
//!
//! These live here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — the ~15 existing
//! `allowlist_for_target_resolves_*` probes in that file are dead code. A probe
//! that cannot fail is not a guard.

use pulse_app::observability::AllowList;

#[test]
fn buffer_tick_allows_the_three_feed_fields() {
    let al = AllowList::production();
    let set = al
        .for_target("buffer.tick")
        .expect("buffer.tick resolves via the .tick strip-suffix rule");

    for field in [
        "span_events_seen",
        "fingerprints_computed",
        "observer_invocations",
    ] {
        assert!(
            set.contains(field),
            "buffer.tick must allow `{field}` or default-deny redacts it silently"
        );
    }
}

#[test]
fn degraded_boot_target_resolves_to_its_own_leaf_entry() {
    let al = AllowList::production();
    let set = al
        .for_target("app.boot.buffer.degraded")
        .expect("the degraded-boot target must have an explicit leaf entry");

    assert!(set.contains("reason"));
    assert!(set.contains("consequence"));
}

#[test]
fn degraded_boot_target_is_not_served_by_a_prefix_fallback() {
    // `for_target` falls back exact -> strip `.tick` -> split('.') -> split("::").
    // If the leaf entry were ever dropped, resolution would slide to a broader
    // `app`-shaped entry (or None) and both fields would be redacted while the
    // emission still looked correct at the call site.
    let al = AllowList::production();
    let leaf = al
        .for_target("app.boot.buffer.degraded")
        .expect("leaf entry present");
    let sibling = al.for_target("app.boot.otlp.grpc.bind");

    if let Some(sibling) = sibling {
        assert_ne!(
            leaf, sibling,
            "the degraded-boot target must resolve to its OWN field set, \
             not to a sibling's via fallback"
        );
    }
}

#[test]
fn feed_fields_are_aggregate_only() {
    // obs-plan §5 cardinality discipline + the 2026-05-17 triage precedent:
    // self-observation carries counts, never per-service or per-span identity.
    let al = AllowList::production();
    let set = al.for_target("buffer.tick").expect("buffer.tick resolves");

    for banned in [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "fingerprint",
        "exception_type",
    ] {
        assert!(
            !set.contains(banned),
            "buffer.tick must not carry `{banned}` — it is an identity field"
        );
    }
}
