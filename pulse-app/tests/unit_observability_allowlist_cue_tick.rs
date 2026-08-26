//! Resolver probe for the `triage.cue.tick` leaf.
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! The emit site (`crates/triage/src/cue/emitter.rs::run_one_emit_cycle`) is
//! the authority for this field list. The leaf existed before this chunk but
//! named only five of the seven fields the site emitted, so `cues_suppressed`
//! and `bypass_triggered` rendered `"<redacted>"` in production while their
//! siblings rendered fine — a PARTLY redacted target, which every gate passes.
//!
//! The fallback shape here is the opposite of `buffer.consumer.stalled`'s:
//! NO bare `triage` or `triage.cue` key exists, so deleting the exact leaf
//! resolves to nothing and redacts all nine fields at once. That is what
//! `no_bare_triage_prefix_key_can_serve_the_cue_tick_target` pins — without it
//! this probe could pass vacuously against a leaf that had been deleted.

use pulse_app::observability::AllowList;

const CUE_TICK: &str = "triage.cue.tick";
const CUE_TICK_FIELDS: [&str; 9] = [
    "cues_evaluated",
    "cues_emitted",
    "cadence_triggers_emitted",
    "services_tracked",
    "operations_tracked",
    "cues_suppressed",
    "bypass_triggered",
    "cues_latched",
    "latch_tracked",
];

#[test]
fn cue_tick_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(CUE_TICK).is_some(),
        "`{CUE_TICK}` must resolve to an allowlist entry",
    );
}

#[test]
fn cue_tick_leaf_carries_exactly_the_emitted_field_set() {
    // Both directions: a narrowed leaf (a field dropped → silently redacted in
    // production, the defect this chunk found) and a widened one (fields the
    // emit site never sends) fail.
    let al = AllowList::production();
    let set = al
        .for_target(CUE_TICK)
        .expect("triage.cue.tick must resolve to an allowlist entry");
    for field in CUE_TICK_FIELDS {
        assert!(
            set.contains(field),
            "`{CUE_TICK}` leaf is missing `{field}` — it would be redacted in production",
        );
    }
    assert_eq!(
        set.len(),
        CUE_TICK_FIELDS.len(),
        "`{CUE_TICK}` leaf carries fields the emit site does not send: {set:?}",
    );
}

/// The suppression diagnostic specifically — the field that could not be read
/// when this chunk needed to answer "did suppression engage, or never fire?".
#[test]
fn cue_tick_leaf_unmutes_the_suppression_diagnostic() {
    let al = AllowList::production();
    let set = al
        .for_target(CUE_TICK)
        .expect("triage.cue.tick must resolve to an allowlist entry");
    for field in ["cues_suppressed", "bypass_triggered"] {
        assert!(
            set.contains(field),
            "`{field}` is muted again — the suppression posture becomes unreadable \
             from any log, which is what forced a 131 MB forensic read",
        );
    }
}

/// The cadence cycle RATE, folded onto the existing 15s heartbeat rather than
/// emitted per trigger (obs-plan §5 counter rows + §11 hot-path ban). Without
/// it the rate is readable only by counting per-trigger `cadence.trigger`
/// records out of the log, which is the forensic read this chunk removes.
///
/// The src-level assertion in `observability.rs`'s own `mod tests` cannot
/// cover this — `[lib] test = false` means it compiles but never runs.
#[test]
fn cadence_tick_carries_the_cycle_rate() {
    let al = AllowList::production();
    let set = al
        .for_target("cadence.tick")
        .expect("cadence.tick must resolve to an allowlist entry");
    assert!(
        set.contains("cycles_executed"),
        "`cadence.tick` is missing `cycles_executed` — the cadence cycle rate \
         would be redacted, leaving a future runaway as unreadable as this one",
    );
}

/// The discriminator. Deleting the exact leaf must leave the target resolving
/// to NOTHING, so the leaf is load-bearing rather than decorative. If a bare
/// `triage` or `triage.cue` key were ever added, it would silently serve every
/// sibling `triage.*` target one field set and this probe could no longer
/// detect a deleted leaf.
#[test]
fn no_bare_triage_prefix_key_can_serve_the_cue_tick_target() {
    let al = AllowList::production();
    for prefix in ["triage", "triage.cue"] {
        assert!(
            al.for_target(prefix).is_none(),
            "a bare `{prefix}` key exists — `{CUE_TICK}` would resolve through it after \
             the exact leaf was deleted, so this guard would pass vacuously",
        );
    }
}
