//! LWW queue with active-incident exception per dist-arch v3 §Queue behavior
//! (chunk #81).
//!
//! - Default cadence-mode digests LWW-replace prior cadence digest within
//!   the same workspace (1 outstanding per workspace).
//! - Active-incident bypass: when L5 has unresolved incident severity ≥
//!   Suggested in workspace, cadence digests for that workspace skip LWW
//!   and queue independently (capped at [`ACTIVE_INCIDENT_QUEUE_CAP`]).
//! - Tier-1 hard signals never LWW-replaced (capped at
//!   [`TIER1_QUEUE_CAP`]).
//! - Reflection digests follow LWW (one outstanding reflection digest per
//!   workspace).
//!
//! Queue mutation events emit `digest.lww.{drop, replace}` tracing
//! events for observability (consumed by `pipeline.l3.lww_drop_count_total`
//! counter + `pipeline.l3.active_incident_queue_depth` gauge).

use std::collections::{HashMap, VecDeque};

use crate::contract::{Digest, DigestLwwMode};

/// Active-incident bypass queue depth cap per workspace per dist-arch v3
/// §Queue behavior. Beyond cap: oldest queued dropped with L6 warning.
pub const ACTIVE_INCIDENT_QUEUE_CAP: usize = 5;

/// Tier-1 hard-signal queue depth cap per dist-arch v3 §Queue behavior.
/// Beyond cap: oldest queued dropped with L6 warning.
pub const TIER1_QUEUE_CAP: usize = 3;

/// Outcome of a `LwwQueue::push` invocation. Used by the assembler to
/// emit appropriate `digest.lww.{drop, replace}` events.
#[derive(Debug, Clone)]
pub enum QueueAction {
    /// Digest queued; nothing displaced.
    Queued,
    /// Digest replaced prior cadence digest (LWW default mode).
    Replaced { replaced_kind: String },
    /// Digest queued past cap; oldest dropped.
    DroppedOldest { dropped_kind: String },
}

/// Cadence-mode LWW + active-incident bypass + Tier-1 hard signal queue
/// for L4 inference invocation.
///
/// Internal state:
/// - `default_per_workspace: HashMap<String, Option<Digest>>` — one
///   outstanding cadence-mode digest per workspace (LWW).
/// - `active_incident_per_workspace: HashMap<String, VecDeque<Digest>>` —
///   bypass queue per workspace, capped at `ACTIVE_INCIDENT_QUEUE_CAP`.
/// - `tier1: VecDeque<Digest>` — global Tier-1 queue capped at
///   `TIER1_QUEUE_CAP` (hard signals are typically rare; per-workspace
///   sharding deferred to L4 routing chunk).
/// - `reflection_per_workspace: HashMap<String, Option<Digest>>` — one
///   outstanding reflection digest per workspace (same LWW shape as
///   default).
#[derive(Debug, Default)]
pub struct LwwQueue {
    default_per_workspace: HashMap<String, Option<Digest>>,
    active_incident_per_workspace: HashMap<String, VecDeque<Digest>>,
    tier1: VecDeque<Digest>,
    reflection_per_workspace: HashMap<String, Option<Digest>>,
}

impl LwwQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a digest. Returns an action describing what the queue did:
    /// Queued (cap-free insert), Replaced (LWW displaced prior), or
    /// DroppedOldest (cap-bounded queue evicted oldest to make room).
    pub fn push(&mut self, digest: Digest) -> QueueAction {
        match digest.lww_mode {
            DigestLwwMode::Default => self.push_default(digest),
            DigestLwwMode::ActiveIncidentBypass => self.push_active_incident(digest),
            DigestLwwMode::Tier1NeverLww => self.push_tier1(digest),
            DigestLwwMode::Reflection => self.push_reflection(digest),
        }
    }

    fn push_default(&mut self, digest: Digest) -> QueueAction {
        let workspace = digest.workspace.clone();
        let slot = self.default_per_workspace.entry(workspace).or_insert(None);
        let prior = slot.replace(digest);
        match prior {
            Some(prior_digest) => QueueAction::Replaced {
                replaced_kind: kind_label(&prior_digest),
            },
            None => QueueAction::Queued,
        }
    }

    fn push_active_incident(&mut self, digest: Digest) -> QueueAction {
        let workspace = digest.workspace.clone();
        let queue = self
            .active_incident_per_workspace
            .entry(workspace)
            .or_default();
        if queue.len() >= ACTIVE_INCIDENT_QUEUE_CAP {
            let dropped = queue.pop_front().expect("queue non-empty at cap");
            queue.push_back(digest);
            QueueAction::DroppedOldest {
                dropped_kind: kind_label(&dropped),
            }
        } else {
            queue.push_back(digest);
            QueueAction::Queued
        }
    }

    fn push_tier1(&mut self, digest: Digest) -> QueueAction {
        if self.tier1.len() >= TIER1_QUEUE_CAP {
            let dropped = self.tier1.pop_front().expect("queue non-empty at cap");
            self.tier1.push_back(digest);
            QueueAction::DroppedOldest {
                dropped_kind: kind_label(&dropped),
            }
        } else {
            self.tier1.push_back(digest);
            QueueAction::Queued
        }
    }

    fn push_reflection(&mut self, digest: Digest) -> QueueAction {
        let workspace = digest.workspace.clone();
        let slot = self
            .reflection_per_workspace
            .entry(workspace)
            .or_insert(None);
        let prior = slot.replace(digest);
        match prior {
            Some(prior_digest) => QueueAction::Replaced {
                replaced_kind: kind_label(&prior_digest),
            },
            None => QueueAction::Queued,
        }
    }

    /// Total depth of the active-incident queue across all workspaces.
    /// Used by `pipeline.l3.active_incident_queue_depth` gauge.
    pub fn active_incident_depth(&self) -> usize {
        self.active_incident_per_workspace
            .values()
            .map(|q| q.len())
            .sum()
    }

    /// Per-workspace active-incident queue depth (snapshot).
    pub fn active_incident_depth_for(&self, workspace: &str) -> usize {
        self.active_incident_per_workspace
            .get(workspace)
            .map(|q| q.len())
            .unwrap_or(0)
    }

    /// Total depth of the Tier-1 queue (global).
    pub fn tier1_depth(&self) -> usize {
        self.tier1.len()
    }

    /// Whether the default LWW slot for the given workspace is occupied.
    pub fn default_slot_occupied(&self, workspace: &str) -> bool {
        self.default_per_workspace
            .get(workspace)
            .map(|s| s.is_some())
            .unwrap_or(false)
    }
}

fn kind_label(d: &Digest) -> String {
    match d.kind {
        crate::contract::DigestKind::Snapshot => "snapshot",
        crate::contract::DigestKind::IncidentSummary => "incident_summary",
        crate::contract::DigestKind::BaselineState => "baseline_state",
        crate::contract::DigestKind::AttentionCueDigest => "attention_cue_digest",
        crate::contract::DigestKind::CadenceTier1 => "cadence_tier1",
        crate::contract::DigestKind::CadenceTier2 => "cadence_tier2",
        crate::contract::DigestKind::CadenceTier3 => "cadence_tier3",
        crate::contract::DigestKind::Reflection => "reflection",
        crate::contract::DigestKind::ResolutionSummary => "resolution_summary",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{DigestKind, DigestLwwMode};

    fn mk_digest(workspace: &str, lww: DigestLwwMode, kind: DigestKind) -> Digest {
        Digest {
            kind,
            token_count: 100,
            payload_summary: "test digest".to_string(),
            incident_refs: vec![],
            generated_at_unix_nano: 0,
            workspace: workspace.to_string(),
            window_start_unix_nano: 0,
            window_end_unix_nano: 60_000_000_000,
            services: vec![],
            attention_cues: vec![],
            corpus_matches: vec![],
            lww_mode: lww,
            active_incident_bypass: matches!(lww, DigestLwwMode::ActiveIncidentBypass),
            resolution_event: false,
        }
    }

    #[test]
    fn first_default_push_returns_queued() {
        let mut q = LwwQueue::new();
        let result = q.push(mk_digest(
            "/ws/a",
            DigestLwwMode::Default,
            DigestKind::CadenceTier3,
        ));
        assert!(matches!(result, QueueAction::Queued));
        assert!(q.default_slot_occupied("/ws/a"));
    }

    #[test]
    fn second_default_push_same_workspace_returns_replaced() {
        let mut q = LwwQueue::new();
        let _ = q.push(mk_digest(
            "/ws/a",
            DigestLwwMode::Default,
            DigestKind::CadenceTier3,
        ));
        let result = q.push(mk_digest(
            "/ws/a",
            DigestLwwMode::Default,
            DigestKind::CadenceTier3,
        ));
        assert!(matches!(result, QueueAction::Replaced { .. }));
    }

    #[test]
    fn default_push_different_workspaces_both_queued() {
        let mut q = LwwQueue::new();
        let r1 = q.push(mk_digest(
            "/ws/a",
            DigestLwwMode::Default,
            DigestKind::CadenceTier3,
        ));
        let r2 = q.push(mk_digest(
            "/ws/b",
            DigestLwwMode::Default,
            DigestKind::CadenceTier3,
        ));
        assert!(matches!(r1, QueueAction::Queued));
        assert!(matches!(r2, QueueAction::Queued));
        assert!(q.default_slot_occupied("/ws/a"));
        assert!(q.default_slot_occupied("/ws/b"));
    }

    #[test]
    fn active_incident_bypass_does_not_replace() {
        let mut q = LwwQueue::new();
        for _ in 0..3 {
            let r = q.push(mk_digest(
                "/ws/a",
                DigestLwwMode::ActiveIncidentBypass,
                DigestKind::CadenceTier3,
            ));
            assert!(matches!(r, QueueAction::Queued));
        }
        assert_eq!(q.active_incident_depth(), 3);
    }

    #[test]
    fn active_incident_queue_caps_at_5_drops_oldest() {
        let mut q = LwwQueue::new();
        for _ in 0..ACTIVE_INCIDENT_QUEUE_CAP {
            let r = q.push(mk_digest(
                "/ws/a",
                DigestLwwMode::ActiveIncidentBypass,
                DigestKind::CadenceTier3,
            ));
            assert!(matches!(r, QueueAction::Queued));
        }
        // 6th push: drops oldest.
        let r = q.push(mk_digest(
            "/ws/a",
            DigestLwwMode::ActiveIncidentBypass,
            DigestKind::CadenceTier3,
        ));
        assert!(matches!(r, QueueAction::DroppedOldest { .. }));
        assert_eq!(q.active_incident_depth(), ACTIVE_INCIDENT_QUEUE_CAP);
    }

    #[test]
    fn tier1_never_lww_each_push_queued() {
        let mut q = LwwQueue::new();
        for _ in 0..TIER1_QUEUE_CAP {
            let r = q.push(mk_digest(
                "/ws/a",
                DigestLwwMode::Tier1NeverLww,
                DigestKind::CadenceTier1,
            ));
            assert!(matches!(r, QueueAction::Queued));
        }
        assert_eq!(q.tier1_depth(), TIER1_QUEUE_CAP);
    }

    #[test]
    fn tier1_caps_at_3_drops_oldest() {
        let mut q = LwwQueue::new();
        for _ in 0..TIER1_QUEUE_CAP {
            let _ = q.push(mk_digest(
                "/ws/a",
                DigestLwwMode::Tier1NeverLww,
                DigestKind::CadenceTier1,
            ));
        }
        let r = q.push(mk_digest(
            "/ws/a",
            DigestLwwMode::Tier1NeverLww,
            DigestKind::CadenceTier1,
        ));
        assert!(matches!(r, QueueAction::DroppedOldest { .. }));
        assert_eq!(q.tier1_depth(), TIER1_QUEUE_CAP);
    }

    #[test]
    fn reflection_lww_per_workspace() {
        let mut q = LwwQueue::new();
        let r1 = q.push(mk_digest(
            "/ws/a",
            DigestLwwMode::Reflection,
            DigestKind::Reflection,
        ));
        assert!(matches!(r1, QueueAction::Queued));
        let r2 = q.push(mk_digest(
            "/ws/a",
            DigestLwwMode::Reflection,
            DigestKind::Reflection,
        ));
        assert!(matches!(r2, QueueAction::Replaced { .. }));
    }
}
