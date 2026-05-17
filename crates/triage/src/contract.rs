//! Triage contract types for Pulse v0.2.0 L1 streaming distillation
//! (capability spec P-019 Three-Tier Severity Model + P-021 Algorithmic
//! Attention Cues — see `docs/v0_2_0/pulse-capability-spec.md`). Empty
//! implementations at this scaffold stage; consumed by chunks #61+.

use serde::{Deserialize, Serialize};

// Chunk #61 — streaming baseline trackers + corpus persistence.
// Re-export public-API types from `triage::baseline` so future chunks (#62
// attention cue emitter, #63 restart event detector) import one shape per
// arch §Conventions "Module visibility discipline" + chunks #58/#60 precedent.
pub use crate::baseline::{
    BaselineError, BaselineState, BootstrapResult, DEFAULT_ALPHA_5MIN_WINDOW,
    DEFAULT_MAX_SIZE_BYTES, DEFAULT_PERSIST_INTERVAL_NANOS, DEFAULT_SERVICE_COUNT_CAP,
    OperationMetricSnapshot, PersistStats, SCHEMA_VERSION, STATE_AGE_THRESHOLD_NANOS,
    ServiceMetricSnapshot, bootstrap_state, persist_on_shutdown, resolve_corpus_path,
    run_persist_cycle, run_persist_loop,
};

// Chunk #62 — attention cue emitter. Re-export public-API types from
// `triage::cue` so pulse-app boot wiring + future v0.2.0 chunks import one
// shape per chunk #61 precedent.
pub use crate::cue::{
    AttentionCueBroadcast, BROADCAST_CAPACITY, CHANNEL_NAME_CADENCE_TRIGGERS,
    CadenceTriggerChannel, DEFAULT_BASE_ERROR_RATE, DEFAULT_BASE_LATENCY_MS,
    DEFAULT_ERROR_RATE_MULTIPLIER, DEFAULT_LATENCY_MULTIPLIER, DEFAULT_LATENCY_PERCENTILE,
    DEFAULT_MIN_PERSISTENCE_SECONDS, DEFAULT_TICK_INTERVAL, MIN_EWMA_SAMPLES,
    STREAM_NAME_ATTENTION_CUES, Thresholds, ThresholdsError, classify_priority,
    dual_condition_bypass, evaluate_thresholds, run_one_emit_cycle, start_emitter,
};

/// Kind of detected condition emitted as an attention cue. Bounded
/// enumeration; future kinds are added explicitly (no `Other(String)`
/// catch-all). Variants serialize as snake_case strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CueKind {
    ErrorRateSpike,
    LatencyRegression,
    RestartEvent,
    ServiceWentSilent,
    RetryStorm,
}

/// Scope an attention cue applies to: a single service, a single operation
/// within a service, or the global pipeline. Bounded enumeration; variants
/// serialize as snake_case strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CueScope {
    Service,
    Operation,
    Global,
}

/// Three-tier severity model per capability spec P-019. Autonomous = model
/// is highly confident (surfaces with prominent halo shift and counter
/// increment); Suggested = model believes likely problem with reservations
/// (quiet counter increment, minimal halo); Curious = worth recording for
/// pattern learning, no interruption (Findings dropdown collapsed section).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriorityTier {
    Autonomous,
    Suggested,
    Curious,
}

/// Severity level for an incident. Bounded enumeration; mirrors
/// `tracing::Level` ordering for future event integration. Variants
/// serialize as snake_case strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warn,
    Error,
    Critical,
}

/// Lifecycle state of an incident per capability spec P-022 auto-resolution
/// flow. Active = currently surfaced; Acknowledged = user acknowledged but
/// underlying signal still active; Resolved = signal has not re-emitted for
/// the cool-down window (120s default).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentStatus {
    Active,
    Acknowledged,
    Resolved,
}

/// Kind of digest emitted at the L4 output layer. Bounded enumeration;
/// future kinds are added explicitly. Variants serialize as snake_case
/// strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestKind {
    Snapshot,
    IncidentSummary,
    BaselineState,
    AttentionCueDigest,
}

/// References to evidence supporting an incident or attention cue. Stores
/// OTLP-native identifiers (raw bytes per OTLP spec; 16-byte trace_id,
/// 8-byte span_id) plus optional anonymized fingerprint hashes per
/// capability spec P-047 redaction-by-construction posture. Hex string
/// conversion (for tracing event field values) happens at emission site,
/// not in the contract type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRefs {
    pub trace_id: Option<[u8; 16]>,
    pub span_ids: Vec<[u8; 8]>,
    pub fingerprint_hashes: Vec<String>,
    pub timestamps_unix_nano: Vec<i64>,
}

/// Algorithmic attention cue emitted by triage detectors per capability
/// spec P-021. Structured record naming the detected condition, the
/// affected scope, and the magnitude of the deviation. Consumed by the
/// model's input context for subsequent interpretation passes. Attention
/// cues never become incidents by themselves; they are internal
/// infrastructure ensuring statistically significant patterns are not
/// overlooked.
///
/// `AttentionCue` derives `PartialEq` (not `Eq`) because the `f64`
/// fields (`magnitude`, `absolute_value`, `confidence`) do not implement
/// `Eq` per Rust's IEEE 754 floating-point semantics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttentionCue {
    pub kind: CueKind,
    pub scope: CueScope,
    /// Identifier of the cue's affected scope (`service.name` when scope =
    /// Service; `operation.name` when scope = Operation; `None` when
    /// scope = Global). Bounded by OTLP cardinality discipline at emission
    /// site.
    pub scope_id: Option<String>,
    /// Detected magnitude as a multiplier vs the streaming baseline
    /// (e.g., 3.5 = 3.5x baseline value).
    pub magnitude: f64,
    /// Absolute value of the detected metric (e.g., absolute error rate
    /// as a fraction; absolute p95 latency in milliseconds).
    pub absolute_value: f64,
    /// How long the condition has persisted, in seconds.
    pub persistence_seconds: u64,
    /// Detector confidence in (0.0, 1.0].
    pub confidence: f64,
    /// Priority tier assigned by the detector based on magnitude +
    /// confidence + persistence per capability spec P-019.
    pub priority_tier: PriorityTier,
    /// True if dual-condition bypass triggered per capability spec P-057
    /// (e.g., `magnitude > 10x` baseline OR `absolute_error_rate > 5%`
    /// overrides restart-window suppression).
    pub suppression_bypassed: bool,
}

/// User-visible incident produced when the model interprets an attention
/// cue as a real problem worth surfacing per capability spec P-020.
/// Carries narrative content (`title`, `detail`) separable from
/// agent-readable enumerated metadata (`kind`, `severity`, `status`) so
/// future emitters can apply `#[tracing::instrument(skip(title, detail))]`
/// + Layer-redaction at the formatter stage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Incident {
    /// Stable incident identifier (UUID-shaped string).
    pub id: String,
    /// Anonymized fingerprint hash for cross-incident grouping per
    /// capability spec P-047 redaction-by-construction posture.
    pub fingerprint: String,
    /// Free-text summary content; MUST be populated with redacted /
    /// fingerprinted summary content only. NEVER raw OTLP attribute
    /// values, span/log/metric content payloads. Per security plan
    /// §Logging "What NEVER to log" bullet 1 + §Security Anti-Patterns
    /// §Logging bullet 1.
    pub title: String,
    /// Free-text incident detail; MUST be populated with redacted /
    /// fingerprinted summary content only. NEVER raw OTLP attribute
    /// values, span/log/metric content payloads. Per security plan
    /// §Logging "What NEVER to log" bullet 1 + §Security Anti-Patterns
    /// §Logging bullet 1.
    pub detail: String,
    pub status: IncidentStatus,
    pub severity: Severity,
    pub priority_tier: PriorityTier,
    pub evidence_refs: EvidenceRefs,
    pub opened_at_unix_nano: i64,
    pub resolved_at_unix_nano: Option<i64>,
}

/// Output of the L4 distillation layer — a digest summarizing incidents,
/// baselines, or attention cues for downstream consumption (paste-to-AI
/// surface, MCP tool response, snapshot file).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Digest {
    pub kind: DigestKind,
    pub token_count: usize,
    /// Free-text digest payload; MUST be populated with redacted /
    /// fingerprinted summary content only. NEVER raw OTLP attribute
    /// values, span/log/metric content payloads. Per security plan
    /// §Logging "What NEVER to log" bullet 1 + §Security Anti-Patterns
    /// §Logging bullet 1.
    pub payload_summary: String,
    /// References to incident IDs included in this digest.
    pub incident_refs: Vec<String>,
    pub generated_at_unix_nano: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_evidence_refs() -> EvidenceRefs {
        EvidenceRefs {
            trace_id: Some([0xAB; 16]),
            span_ids: vec![[0xCD; 8], [0xEF; 8]],
            fingerprint_hashes: vec!["abc123".to_string(), "def456".to_string()],
            timestamps_unix_nano: vec![1_700_000_000_000, 1_700_000_001_000],
        }
    }

    fn sample_attention_cue() -> AttentionCue {
        AttentionCue {
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some("checkout".to_string()),
            magnitude: 3.5,
            absolute_value: 0.075,
            persistence_seconds: 45,
            confidence: 0.87,
            priority_tier: PriorityTier::Suggested,
            suppression_bypassed: false,
        }
    }

    fn sample_incident() -> Incident {
        Incident {
            id: "11111111-2222-3333-4444-555555555555".to_string(),
            fingerprint: "fp-checkout-err-spike".to_string(),
            title: "[redacted] error rate spike in checkout".to_string(),
            detail: "[redacted] sustained 3.5x baseline for 45s".to_string(),
            status: IncidentStatus::Active,
            severity: Severity::Warn,
            priority_tier: PriorityTier::Suggested,
            evidence_refs: sample_evidence_refs(),
            opened_at_unix_nano: 1_700_000_000_000,
            resolved_at_unix_nano: None,
        }
    }

    fn sample_digest() -> Digest {
        Digest {
            kind: DigestKind::IncidentSummary,
            token_count: 1024,
            payload_summary: "[redacted] 1 active incident, 3 resolved".to_string(),
            incident_refs: vec!["11111111-2222-3333-4444-555555555555".to_string()],
            generated_at_unix_nano: 1_700_000_002_000,
        }
    }

    #[test]
    fn cue_kind_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&CueKind::ErrorRateSpike).unwrap(),
            "\"error_rate_spike\""
        );
        assert_eq!(
            serde_json::to_string(&CueKind::LatencyRegression).unwrap(),
            "\"latency_regression\""
        );
        assert_eq!(
            serde_json::to_string(&CueKind::RestartEvent).unwrap(),
            "\"restart_event\""
        );
        assert_eq!(
            serde_json::to_string(&CueKind::ServiceWentSilent).unwrap(),
            "\"service_went_silent\""
        );
        assert_eq!(
            serde_json::to_string(&CueKind::RetryStorm).unwrap(),
            "\"retry_storm\""
        );
    }

    #[test]
    fn cue_scope_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&CueScope::Service).unwrap(),
            "\"service\""
        );
        assert_eq!(
            serde_json::to_string(&CueScope::Operation).unwrap(),
            "\"operation\""
        );
        assert_eq!(
            serde_json::to_string(&CueScope::Global).unwrap(),
            "\"global\""
        );
    }

    #[test]
    fn priority_tier_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&PriorityTier::Autonomous).unwrap(),
            "\"autonomous\""
        );
        assert_eq!(
            serde_json::to_string(&PriorityTier::Suggested).unwrap(),
            "\"suggested\""
        );
        assert_eq!(
            serde_json::to_string(&PriorityTier::Curious).unwrap(),
            "\"curious\""
        );
    }

    #[test]
    fn severity_serializes_snake_case() {
        assert_eq!(serde_json::to_string(&Severity::Info).unwrap(), "\"info\"");
        assert_eq!(serde_json::to_string(&Severity::Warn).unwrap(), "\"warn\"");
        assert_eq!(
            serde_json::to_string(&Severity::Error).unwrap(),
            "\"error\""
        );
        assert_eq!(
            serde_json::to_string(&Severity::Critical).unwrap(),
            "\"critical\""
        );
    }

    #[test]
    fn incident_status_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&IncidentStatus::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&IncidentStatus::Acknowledged).unwrap(),
            "\"acknowledged\""
        );
        assert_eq!(
            serde_json::to_string(&IncidentStatus::Resolved).unwrap(),
            "\"resolved\""
        );
    }

    #[test]
    fn digest_kind_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&DigestKind::Snapshot).unwrap(),
            "\"snapshot\""
        );
        assert_eq!(
            serde_json::to_string(&DigestKind::IncidentSummary).unwrap(),
            "\"incident_summary\""
        );
        assert_eq!(
            serde_json::to_string(&DigestKind::BaselineState).unwrap(),
            "\"baseline_state\""
        );
        assert_eq!(
            serde_json::to_string(&DigestKind::AttentionCueDigest).unwrap(),
            "\"attention_cue_digest\""
        );
    }

    #[test]
    fn cue_kind_round_trips_through_serde() {
        for kind in [
            CueKind::ErrorRateSpike,
            CueKind::LatencyRegression,
            CueKind::RestartEvent,
            CueKind::ServiceWentSilent,
            CueKind::RetryStorm,
        ] {
            let json = serde_json::to_string(&kind).expect("serialize");
            let parsed: CueKind = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, kind);
        }
    }

    #[test]
    fn cue_scope_round_trips_through_serde() {
        for scope in [CueScope::Service, CueScope::Operation, CueScope::Global] {
            let json = serde_json::to_string(&scope).expect("serialize");
            let parsed: CueScope = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, scope);
        }
    }

    #[test]
    fn priority_tier_round_trips_through_serde() {
        for tier in [
            PriorityTier::Autonomous,
            PriorityTier::Suggested,
            PriorityTier::Curious,
        ] {
            let json = serde_json::to_string(&tier).expect("serialize");
            let parsed: PriorityTier = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, tier);
        }
    }

    #[test]
    fn severity_round_trips_through_serde() {
        for sev in [
            Severity::Info,
            Severity::Warn,
            Severity::Error,
            Severity::Critical,
        ] {
            let json = serde_json::to_string(&sev).expect("serialize");
            let parsed: Severity = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, sev);
        }
    }

    #[test]
    fn incident_status_round_trips_through_serde() {
        for status in [
            IncidentStatus::Active,
            IncidentStatus::Acknowledged,
            IncidentStatus::Resolved,
        ] {
            let json = serde_json::to_string(&status).expect("serialize");
            let parsed: IncidentStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, status);
        }
    }

    #[test]
    fn digest_kind_round_trips_through_serde() {
        for kind in [
            DigestKind::Snapshot,
            DigestKind::IncidentSummary,
            DigestKind::BaselineState,
            DigestKind::AttentionCueDigest,
        ] {
            let json = serde_json::to_string(&kind).expect("serialize");
            let parsed: DigestKind = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, kind);
        }
    }

    #[test]
    fn evidence_refs_round_trips_through_serde() {
        let e = sample_evidence_refs();
        let json = serde_json::to_string(&e).expect("serialize");
        let parsed: EvidenceRefs = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, e);
    }

    #[test]
    fn attention_cue_round_trips_through_serde() {
        let cue = sample_attention_cue();
        let json = serde_json::to_string(&cue).expect("serialize");
        let parsed: AttentionCue = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, cue);
    }

    #[test]
    fn incident_round_trips_through_serde() {
        let inc = sample_incident();
        let json = serde_json::to_string(&inc).expect("serialize");
        let parsed: Incident = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, inc);
    }

    #[test]
    fn digest_round_trips_through_serde() {
        let d = sample_digest();
        let json = serde_json::to_string(&d).expect("serialize");
        let parsed: Digest = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, d);
    }
}
