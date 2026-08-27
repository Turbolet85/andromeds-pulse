//! Resolver probes for the generation-damper observables: the
//! `interpretation.generation.damper` transition leaf and the two
//! counter fields completing `metric.pipeline.l4.backoff_remaining_seconds`.
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level
//! tests in `pulse-app` compile but never run.
//!
//! The emit sites (`pulse-app/src/inference_runtime.rs::process_digest`
//! for the transition target, `::spawn_l4_backoff_remaining_heartbeat`
//! for the heartbeat fields) are the authority for these field lists.
//!
//! A bare `interpretation` key EXISTS in the allowlist (a recorded
//! invariant breach owned by the diagnostics-sweep route entry), so the
//! failure mode for the transition leaf is not "resolves to nothing" but
//! "resolves to the WRONG set" — the fallback-set probes below are what
//! keep these exact leaves load-bearing rather than decorative.

use pulse_app::observability::AllowList;

const DAMPER: &str = "interpretation.generation.damper";
const DAMPER_FIELDS: [&str; 4] = ["decision", "reason", "digest_kind", "suppressed_run_len"];

const BACKOFF_HEARTBEAT: &str = "metric.pipeline.l4.backoff_remaining_seconds";
const BACKOFF_FIELDS: [&str; 3] = [
    "value",
    "generations_suppressed_total",
    "generations_run_total",
];

#[test]
fn damper_transition_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(DAMPER).is_some(),
        "`{DAMPER}` must resolve to an allowlist entry",
    );
}

#[test]
fn damper_leaf_carries_exactly_the_emitted_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target(DAMPER)
        .expect("interpretation.generation.damper must resolve to an allowlist entry");
    for field in DAMPER_FIELDS {
        assert!(
            set.contains(field),
            "`{DAMPER}` leaf is missing `{field}` — it would be redacted in production",
        );
    }
    assert_eq!(
        set.len(),
        DAMPER_FIELDS.len(),
        "`{DAMPER}` leaf carries fields the emit site does not send: {set:?}",
    );
}

/// The discriminator. If the exact damper leaf were deleted, `for_target`
/// would fall back to the bare `interpretation` key — this asserts that
/// set cannot serve the transition target, so a resolver-only probe can
/// never pass vacuously against the fallback.
#[test]
fn damper_fields_are_absent_from_the_interpretation_fallback_set() {
    let al = AllowList::production();
    let fallback = al
        .for_target("interpretation")
        .expect("the bare `interpretation` key exists (recorded breach, sweep-owned)");
    for field in DAMPER_FIELDS {
        assert!(
            !fallback.contains(field),
            "`{field}` is in the bare `interpretation` fallback set, so this probe cannot \
             detect a deleted `{DAMPER}` leaf — the guard would pass vacuously",
        );
    }
}

#[test]
fn backoff_heartbeat_leaf_carries_exactly_the_completed_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target(BACKOFF_HEARTBEAT)
        .expect("metric.pipeline.l4.backoff_remaining_seconds must resolve");
    for field in BACKOFF_FIELDS {
        assert!(
            set.contains(field),
            "`{BACKOFF_HEARTBEAT}` leaf is missing `{field}` — it would be redacted in production",
        );
    }
    assert_eq!(
        set.len(),
        BACKOFF_FIELDS.len(),
        "`{BACKOFF_HEARTBEAT}` leaf carries fields the emit site does not send: {set:?}",
    );
}

/// The `metric.*` inversion of the discriminator: the bare `metric` key
/// legitimately exists and KEEPS `value`, so a deleted exact leaf would
/// silently redact only the damper counters while `value` survives —
/// exactly the partly-redacted-while-probes-pass shape. Assert the
/// counter fields are absent from the fallback so the set-completeness
/// pin above is the load-bearing guard.
#[test]
fn damper_counters_are_absent_from_the_metric_fallback_set() {
    let al = AllowList::production();
    let fallback = al
        .for_target("metric")
        .expect("the bare `metric` key exists (arch-recorded fallback for metric.* targets)");
    for field in ["generations_suppressed_total", "generations_run_total"] {
        assert!(
            !fallback.contains(field),
            "`{field}` is in the bare `metric` fallback set — the completed-leaf pin would \
             not discriminate a deleted `{BACKOFF_HEARTBEAT}` leaf",
        );
    }
}
