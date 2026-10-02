//! Branch pins for the drain-progress classifier.
//!
//! Lives here rather than in `heartbeat.rs`'s own `mod tests` because
//! `[lib] test = false` means src-level tests in `pulse-app` compile but never
//! run.
//!
//! `rows_ingested` alone cannot separate a stalled consumer from an idle
//! producer — both leave it static, which is exactly why the measured wedge
//! read as healthy. The ingest channel's occupancy is the discriminator, so
//! each of the three branches is pinned independently; a classifier that
//! collapsed `ProducerIdle` into `NotDraining` would fire on every quiet
//! period, and one that collapsed the other way would never fire at all.

use pulse_app::heartbeat::{
    DrainProgress, STALL_CONSECUTIVE_TICKS, classify_drain_progress, last_append_age_seconds,
};

#[test]
fn rows_advancing_is_healthy_regardless_of_channel_occupancy() {
    assert_eq!(
        classify_drain_progress(1, 0.0),
        DrainProgress::Advancing,
        "any forward progress is healthy",
    );
    assert_eq!(
        classify_drain_progress(500, 99.9),
        DrainProgress::Advancing,
        "a full channel with rows still landing is backlog, not a stall",
    );
}

#[test]
fn static_rows_with_an_empty_channel_is_an_idle_producer() {
    assert_eq!(
        classify_drain_progress(0, 0.0),
        DrainProgress::ProducerIdle,
        "nothing queued and nothing landing means no one is sending",
    );
}

#[test]
fn static_rows_with_queued_work_is_a_stalled_consumer() {
    assert_eq!(
        classify_drain_progress(0, 0.1),
        DrainProgress::NotDraining,
        "work is queued and none of it is landing — the defect this chunk exists for",
    );
    assert_eq!(
        classify_drain_progress(0, 100.0),
        DrainProgress::NotDraining,
        "the measured wedge's terminal state: channel pinned, rows frozen",
    );
}

/// The threshold must clear obs-plan §10's in-spec bounds — a single append may
/// block >60s under row-group maintenance, and the sustained-drain profile is
/// allowed 420s. Pinned as a value, not a shape: a threshold that silently
/// dropped below 420s would fire on healthy load and train readers to ignore it.
#[test]
fn stall_threshold_clears_the_in_spec_drain_window() {
    let threshold_seconds = STALL_CONSECUTIVE_TICKS as u64 * 15;
    assert!(
        threshold_seconds > 420,
        "threshold {threshold_seconds}s must exceed the 420s sustained-drain cap",
    );
    assert_eq!(threshold_seconds, 450);
}

#[test]
fn last_append_age_is_zero_before_the_first_append() {
    assert_eq!(
        last_append_age_seconds(0, 1_000_000_000_000),
        0,
        "a buffer that has never appended reports age 0, not the epoch delta",
    );
}

#[test]
fn last_append_age_is_whole_seconds_since_the_last_append() {
    let last = 1_000_000_000_000_u64;
    assert_eq!(last_append_age_seconds(last, last), 0);
    assert_eq!(last_append_age_seconds(last, last + 5_000_000_000), 5);
    assert_eq!(
        last_append_age_seconds(last, last.saturating_sub(1)),
        0,
        "a clock that moved backwards must not underflow into a huge age",
    );
}
