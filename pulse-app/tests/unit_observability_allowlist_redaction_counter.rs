//! Resolver probe for the P-047 redaction counter on `buffer.tick`.
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! The emit site (`pulse-app/src/heartbeat.rs::emit_buffer_tick`) is the
//! authority for this field list. `buffer.tick` has no exact key of its own; it
//! resolves through `for_target`'s `.tick`-strip to the `buffer` set, so the
//! discriminating assertion is that the resolved set carries the counter
//! alongside the fields that already ride the same event.

use pulse_app::observability::AllowList;

const BUFFER_TICK: &str = "buffer.tick";
const REDACTION_FIELD: &str = "redactions_applied";

#[test]
fn buffer_tick_allows_the_redaction_counter_its_emit_site_emits() {
    let al = AllowList::production();
    let set = al
        .for_target(BUFFER_TICK)
        .expect("buffer.tick must resolve to an allowlist entry");

    assert!(
        set.contains(REDACTION_FIELD),
        "`{REDACTION_FIELD}` is emitted at {BUFFER_TICK} and must not be redacted",
    );
}

#[test]
fn redaction_counter_rides_the_same_entry_as_the_rest_of_the_tick() {
    // A leaf that names the counter but drops its siblings would leave the
    // target PARTLY redacted — the recurring defect four consecutive obs-plan
    // amendments corrected. Assert the whole emitted set together so a future
    // narrowing of the entry fails here rather than silently in production.
    let al = AllowList::production();
    let set = al
        .for_target(BUFFER_TICK)
        .expect("buffer.tick must resolve to an allowlist entry");

    for field in [
        "rows_ingested",
        "retention_window_active",
        "eviction_count",
        "memory_bytes",
        "retention_window_seconds",
        "eviction_count_since_last_tick",
        "drain_template_count",
        "drain_lru_evictions_since_tick",
        "span_events_seen",
        "fingerprints_computed",
        "observer_invocations",
        REDACTION_FIELD,
    ] {
        assert!(
            set.contains(field),
            "`{field}` is emitted at {BUFFER_TICK} and must not be redacted",
        );
    }
}

#[test]
fn redaction_counter_never_admits_value_bearing_fields() {
    // The counter is an aggregate. Nothing that could carry the matched
    // credential, its category, or the attribute it came from may ride the
    // same entry (per obs-plan §5 cardinality + §8 data classification).
    let al = AllowList::production();
    let set = al
        .for_target(BUFFER_TICK)
        .expect("buffer.tick must resolve to an allowlist entry");

    for banned in [
        "redacted_value",
        "matched_value",
        "redaction_category",
        "attribute_key",
        "service_name",
        "body",
        "exception_message",
    ] {
        assert!(
            !set.contains(banned),
            "`{banned}` must never be permitted on {BUFFER_TICK}",
        );
    }
}
