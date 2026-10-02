//! Resolver probe for the consumer-stall leaf (`buffer.consumer.stalled`) and
//! the two drain-progress fields added to `buffer.tick`.
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! The emit sites (`pulse-app/src/heartbeat.rs::run_buffer` for the stall
//! target, `::emit_buffer_tick` for the tick fields) are the authority for
//! these field lists.
//!
//! Unlike the `app.`-prefixed leaves, a bare `buffer` key DOES exist, so the
//! failure mode here is not "resolves to nothing" but "resolves to the WRONG
//! set": `for_target` falls back to the first `.`-segment, and the `buffer`
//! entry carries none of the stall fields. That silent-wrong-set path is what
//! `stall_fields_are_absent_from_the_buffer_fallback_set` pins.

use pulse_app::observability::AllowList;

const STALLED: &str = "buffer.consumer.stalled";
const STALL_FIELDS: [&str; 4] = [
    "reason",
    "consequence",
    "stalled_seconds",
    "buffer_capacity_pct",
];
const TICK: &str = "buffer.tick";
const PROGRESS_FIELDS: [&str; 2] = ["rows_ingested_delta", "last_append_age_seconds"];

#[test]
fn stalled_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(STALLED).is_some(),
        "`{STALLED}` must resolve to an allowlist entry",
    );
}

#[test]
fn stalled_leaf_carries_exactly_the_emitted_field_set() {
    // Both directions: a narrowed leaf (a field dropped → silently redacted in
    // production) and a widened one (fields the emit site never sends) fail.
    let al = AllowList::production();
    let set = al
        .for_target(STALLED)
        .expect("buffer.consumer.stalled must resolve to an allowlist entry");
    for field in STALL_FIELDS {
        assert!(
            set.contains(field),
            "`{STALLED}` leaf is missing `{field}` — it would be redacted in production",
        );
    }
    assert_eq!(
        set.len(),
        STALL_FIELDS.len(),
        "`{STALLED}` leaf carries fields the emit site does not send: {set:?}",
    );
}

/// The discriminator. If the exact leaf were deleted, `for_target` would fall
/// back to the `buffer` set — this asserts that set cannot serve the stall
/// target, so the exact leaf is load-bearing rather than decorative.
#[test]
fn stall_fields_are_absent_from_the_buffer_fallback_set() {
    let al = AllowList::production();
    let fallback = al
        .for_target("buffer")
        .expect("the bare `buffer` key must exist — buffer.tick resolves through it");
    for field in STALL_FIELDS {
        assert!(
            !fallback.contains(field),
            "`{field}` is in the `buffer` fallback set, so this probe cannot detect a \
             deleted `{STALLED}` leaf — the guard would pass vacuously",
        );
    }
}

#[test]
fn buffer_tick_carries_the_drain_progress_fields() {
    let al = AllowList::production();
    let set = al
        .for_target(TICK)
        .expect("buffer.tick must resolve (via the `.tick` strip to `buffer`)");
    for field in PROGRESS_FIELDS {
        assert!(
            set.contains(field),
            "`{TICK}` is missing `{field}` — the drain-progress reading would be \
             redacted, leaving a wedged consumer as unreadable as before",
        );
    }
}
