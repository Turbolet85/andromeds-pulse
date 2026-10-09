//! Corpus retrieval for retrieval-augmented interpretation (capability
//! P-044) + the report-side "Previously seen" selection (capability
//! P-036).
//!
//! The [`CorpusIncidentSource`] trait is the triage-declared seam for the
//! corpus-backed candidate query: declared here, implemented at the
//! `pulse-app` binary boundary (per arch §Module dependency direction —
//! triage carries no corpus dep edge; mirrors the chunk #80
//! `SqlQueryRunner` injection precedent). The source narrows candidates
//! by workspace + time window only; fingerprint/scope matching happens
//! in the pure selection helpers below because those fields live inside
//! the encrypted corpus payload BLOB and are only visible post-decode.
//!
//! Selection semantics per capability spec P-044 (same workspace, last
//! 30 days, fingerprint or scope match, top-N default 5) + P-036 (match
//! current incident's fingerprint / service-and-operation scope).

use std::future::Future;
use std::pin::Pin;

use thiserror::Error;

use crate::contract::{Incident, IncidentStatus};

/// Retrieval candidate window per capability spec P-044 ("prior
/// incidents from same workspace within last 30 days").
pub const CORPUS_RETRIEVAL_WINDOW_SECONDS: i64 = 30 * 24 * 60 * 60;

/// Future return-position alias for [`CorpusIncidentSource`]. Manual
/// `Pin<Box<dyn Future + Send + 'a>>` keeps the trait object-safe
/// (`Arc<dyn CorpusIncidentSource>`) without an `async-trait` dep (per
/// CLAUDE.md §Session Learnings 2026-05-23 pattern).
pub type RetrievalFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, RetrievalError>> + Send + 'a>>;

/// Error from a corpus incident source. Opaque by design — the binary-
/// boundary impl logs its own bounded `error_category` before returning;
/// the assembler degrades to empty `corpus_matches` regardless of cause
/// (per arch §Established Decisions [Error Handling Pattern] graceful
/// degradation).
#[derive(Debug, Error)]
pub enum RetrievalError {
    #[error("corpus incident retrieval failed")]
    SourceFailed,
}

/// Candidate supplier for corpus retrieval. The production impl lives at
/// `pulse-app/src/corpus_retrieval.rs` over `corpus::CorpusWriter::
/// load_incidents_for_workspace_since`; an absent corpus degrades via a
/// no-op impl returning empty candidates.
pub trait CorpusIncidentSource: Send + Sync {
    /// Load candidate incidents for `workspace` created at or after
    /// `since_unix_nano`, any status.
    fn load_candidates<'a>(
        &'a self,
        workspace: &'a str,
        since_unix_nano: i64,
    ) -> RetrievalFuture<'a, Vec<Incident>>;
}

/// No-op source for boots without a corpus (`Option<Arc<Corpus>>` =
/// `None`) and for tests that exercise the empty-matches path.
#[derive(Debug, Default)]
pub struct NoopCorpusIncidentSource;

impl CorpusIncidentSource for NoopCorpusIncidentSource {
    fn load_candidates<'a>(
        &'a self,
        _workspace: &'a str,
        _since_unix_nano: i64,
    ) -> RetrievalFuture<'a, Vec<Incident>> {
        Box::pin(async { Ok(Vec::new()) })
    }
}

/// Digest-side match selection (capability P-044): keep candidates whose
/// fingerprint appears in the current window's fingerprint set OR whose
/// `scope_id` matches a currently-active service scope; rank by recency
/// (newest first); cap at `limit`.
///
/// The fingerprint arm is correct by contract and currently STARVED, not dead.
/// `current_fingerprints` arrives as `hex_lower` of Q3's `span_events.fingerprint`
/// blake3 bytes, and `Incident.fingerprint` is documented as an anonymized
/// fingerprint hash, so both sides are meant to be the same hex namespace.
/// The L4 producer instead writes the model-authored `L4Output.fingerprint`
/// there, so in the current wiring the arm cannot fire and only `scope_match`
/// does. The mismatch is the producer's; do not "simplify" this arm away.
///
/// `triggering_scope` is the `scope_id` of the digest's triggering cue, when it
/// has one. It narrows the scope arm to that scope and only ever removes: a
/// match scoped to another active service is dropped before the cap, and one
/// the fingerprint arm keeps stays whatever its scope. A digest that listed
/// another service's incidents led the L4 model to place the signal there
/// (arch §Established Decisions [Fault Identity]). `None` keeps the matches
/// of every active scope.
pub fn select_corpus_matches(
    mut candidates: Vec<Incident>,
    current_fingerprints: &[String],
    current_scopes: &[String],
    triggering_scope: Option<&str>,
    limit: usize,
) -> Vec<Incident> {
    candidates.retain(|c| {
        let fingerprint_match =
            !c.fingerprint.is_empty() && current_fingerprints.iter().any(|f| f == &c.fingerprint);
        let scope_match = c.scope_id.as_deref().is_some_and(|s| {
            current_scopes.iter().any(|cs| cs == s) && triggering_scope.is_none_or(|t| t == s)
        });
        fingerprint_match || scope_match
    });
    candidates.sort_by_key(|c| std::cmp::Reverse(c.opened_at_unix_nano));
    candidates.truncate(limit);
    candidates
}

/// Report-side "Previously seen" selection (capability P-036): keep past
/// incidents matching THIS incident's fingerprint or scope, excluding the
/// incident itself; rank by recency; cap at `limit`.
pub fn select_previously_seen(
    current: &Incident,
    mut candidates: Vec<Incident>,
    limit: usize,
) -> Vec<Incident> {
    candidates.retain(|c| {
        if c.id == current.id {
            return false;
        }
        let fingerprint_match =
            !current.fingerprint.is_empty() && c.fingerprint == current.fingerprint;
        let scope_match = current.scope_id.is_some() && c.scope_id == current.scope_id;
        fingerprint_match || scope_match
    });
    candidates.sort_by_key(|c| std::cmp::Reverse(c.opened_at_unix_nano));
    candidates.truncate(limit);
    candidates
}

/// Format one selected match into the bounded digest line shape: the
/// match carries the anonymized fingerprint + pre-scrubbed title + age +
/// resolution outcome where known (P-036 "with timestamps and resolution
/// outcomes where known"). The assembler additionally routes the line
/// through its injected scrub closure before embedding (defense-in-depth
/// per the chunk #88 egress precedent).
pub fn format_corpus_match_line(incident: &Incident, now_unix_nano: i64) -> String {
    let age_minutes =
        (now_unix_nano.saturating_sub(incident.opened_at_unix_nano)).max(0) / 60_000_000_000;
    let outcome = match incident.status {
        IncidentStatus::Resolved => "resolved",
        IncidentStatus::Acknowledged => "acknowledged",
        IncidentStatus::Active => "active",
    };
    format!(
        "[{}] {} — {}m ago, {}",
        incident.fingerprint, incident.title, age_minutes, outcome
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CueKind, CueScope, EvidenceRefs, PriorityTier, Severity};

    fn incident(id: i64, fingerprint: &str, scope_id: Option<&str>, opened_at: i64) -> Incident {
        Incident {
            id,
            workspace: "ws".to_string(),
            fingerprint: fingerprint.to_string(),
            title: format!("[redacted] incident-{id}"),
            detail: String::new(),
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: scope_id.map(str::to_string),
            status: IncidentStatus::Active,
            severity: Severity::Warn,
            priority_tier: PriorityTier::Suggested,
            evidence_refs: EvidenceRefs {
                trace_id: None,
                span_ids: vec![],
                fingerprint_hashes: vec![],
                timestamps_unix_nano: vec![],
            },
            opened_at_unix_nano: opened_at,
            updated_at_unix_nano: opened_at,
            acknowledged_at_unix_nano: None,
            resolved_at_unix_nano: None,
            read_at_unix_nano: None,
            resolution_summary_text: None,
        }
    }

    #[test]
    fn select_corpus_matches_keeps_fingerprint_matches_only() {
        let candidates = vec![
            incident(1, "fp-a", None, 100),
            incident(2, "fp-b", None, 200),
        ];
        let selected = select_corpus_matches(candidates, &["fp-a".to_string()], &[], None, 5);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id, 1);
    }

    #[test]
    fn select_corpus_matches_keeps_scope_matches() {
        let candidates = vec![
            incident(1, "fp-a", Some("svc-api"), 100),
            incident(2, "fp-b", Some("svc-db"), 200),
        ];
        let selected = select_corpus_matches(candidates, &[], &["svc-api".to_string()], None, 5);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id, 1);
    }

    #[test]
    fn select_corpus_matches_ranks_by_recency_and_caps_at_limit() {
        let candidates = vec![
            incident(1, "fp-a", None, 100),
            incident(2, "fp-a", None, 300),
            incident(3, "fp-a", None, 200),
        ];
        let selected = select_corpus_matches(candidates, &["fp-a".to_string()], &[], None, 2);
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].id, 2, "newest first");
        assert_eq!(selected[1].id, 3);
    }

    #[test]
    fn select_corpus_matches_empty_fingerprint_never_matches_empty_set_entry() {
        let candidates = vec![incident(1, "", None, 100)];
        let selected = select_corpus_matches(candidates, &[String::new()], &[], None, 5);
        assert!(selected.is_empty(), "empty fingerprints are not identity");
    }

    /// A sibling's newest incident, the triggering scope's own, an older
    /// sibling's, and two the fingerprint arm keeps: one from a scope outside
    /// the window, one the sibling's.
    fn mixed_scope_candidates() -> Vec<Incident> {
        vec![
            incident(1, "fp-x", Some("svc-canary"), 500),
            incident(2, "fp-y", Some("svc"), 400),
            incident(3, "fp-z", Some("svc-canary"), 300),
            incident(4, "fp-w", Some("svc-elsewhere"), 200),
            incident(5, "fp-w", Some("svc-canary"), 100),
        ]
    }

    fn mixed_selected(scopes: &[&str], triggering_scope: Option<&str>, limit: usize) -> Vec<i64> {
        let scopes: Vec<String> = scopes.iter().map(|s| s.to_string()).collect();
        select_corpus_matches(
            mixed_scope_candidates(),
            &["fp-w".to_string()],
            &scopes,
            triggering_scope,
            limit,
        )
        .iter()
        .map(|i| i.id)
        .collect()
    }

    #[test]
    fn select_corpus_matches_drops_other_scopes_matches_under_a_triggering_scope() {
        assert_eq!(
            mixed_selected(&["svc", "svc-canary"], Some("svc"), 5),
            [2, 4, 5],
            "the scope's own and both fingerprint matches, the sibling's among them"
        );
    }

    #[test]
    fn select_corpus_matches_keeps_every_active_scope_without_a_triggering_scope() {
        assert_eq!(
            mixed_selected(&["svc", "svc-canary"], None, 5),
            [1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn select_corpus_matches_narrows_before_the_cap_and_only_removes() {
        assert_eq!(mixed_selected(&["svc", "svc-canary"], None, 2), [1, 2]);
        assert_eq!(
            mixed_selected(&["svc", "svc-canary"], Some("svc"), 2),
            [2, 4],
            "a match the cap cut behind the sibling's stands once the sibling's are dropped"
        );
        assert_eq!(mixed_selected(&["svc-canary"], None, 5), [1, 3, 4, 5]);
        assert_eq!(
            mixed_selected(&["svc-canary"], Some("svc"), 5),
            [4, 5],
            "a triggering scope that is no active scope adds no match"
        );
    }

    #[test]
    fn select_previously_seen_excludes_self_and_matches_fingerprint() {
        let current = incident(7, "fp-a", None, 900);
        let candidates = vec![
            incident(7, "fp-a", None, 900),
            incident(1, "fp-a", None, 100),
            incident(2, "fp-b", None, 200),
        ];
        let selected = select_previously_seen(&current, candidates, 5);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id, 1);
    }

    #[test]
    fn select_previously_seen_matches_scope_when_fingerprints_differ() {
        let current = incident(7, "fp-x", Some("svc-api"), 900);
        let candidates = vec![
            incident(1, "fp-a", Some("svc-api"), 100),
            incident(2, "fp-b", Some("svc-db"), 200),
        ];
        let selected = select_previously_seen(&current, candidates, 5);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id, 1);
    }

    #[test]
    fn select_previously_seen_none_scope_never_matches_none_scope() {
        let current = incident(7, "fp-x", None, 900);
        let candidates = vec![incident(1, "fp-y", None, 100)];
        let selected = select_previously_seen(&current, candidates, 5);
        assert!(selected.is_empty(), "None scope is not an identity match");
    }

    #[test]
    fn format_corpus_match_line_carries_fingerprint_title_age_outcome() {
        let mut i = incident(1, "fp-a3f9", None, 0);
        i.status = IncidentStatus::Resolved;
        let line = format_corpus_match_line(&i, 5 * 60_000_000_000);
        assert_eq!(line, "[fp-a3f9] [redacted] incident-1 — 5m ago, resolved");
    }

    #[test]
    fn noop_source_returns_empty_candidates() {
        let source = NoopCorpusIncidentSource;
        let result = futures_executor_block_on(source.load_candidates("ws", 0));
        assert!(result.expect("ok").is_empty());
    }

    /// Minimal block_on for the no-op source's ready-immediately future —
    /// avoids a tokio dev-dep in this pure module (the future never
    /// yields, so a single poll resolves it).
    fn futures_executor_block_on<F: Future>(fut: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn noop_raw_waker() -> RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(_: *const ()) -> RawWaker {
                noop_raw_waker()
            }
            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut cx = Context::from_waker(&waker);
        let mut fut = std::pin::pin!(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(out) => out,
            Poll::Pending => unreachable!("noop source future resolves on first poll"),
        }
    }
}
