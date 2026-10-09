//! Pattern-discrimination shapes for the L4 decision probe: families A (a
//! pattern today's digest carries but no detector flags), B (healthy traffic
//! the model must not alarm on) and C (a pattern only a digest carrying
//! baselines and a short trend shows). Each shape carries its ground truth,
//! written before any real-model run.
//!
//! Scoring is labels-only from the model's JSON: `valid`, `detect` and
//! `cause`, each a closed set. The texts a run keeps under `target/` feed the
//! whole-series re-grade, the blind audit draw and the table; the table
//! applies the recommendation rule fixed before the series ran.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use interpretation::prompt::build_primary_tier_prompt;
use interpretation::schema::{Decision, L4Output, parse_bounded};
use pulse_app::llamacli_inference::{extract_json_object_bounded, validate_prompt_bounded};
use serde_json::{Map, Value, json};
use triage::contract::{
    AttentionCue, CueKind, CueScope, DigestCueRef, DigestProjectContext, DigestRecentCommit,
    DigestServiceRow, EvidenceRefs, Incident, IncidentStatus, PriorityTier,
    Severity as IncidentSeverity, cue_summary, format_corpus_match_line, render_payload,
};

use super::{
    Metrics, RENDER_NOW_UNIX_NANO, WORKSPACE, decision_label, p50, reading, severity_label,
    thinking_label,
};

pub const PATTERN_IDS: [&str; 13] = [
    "A1", "A2", "A3", "A4", "A5", "A6", "A7", "B1", "B2", "B3", "C1", "C2", "C3",
];
pub const VALID_LABELS: [&str; 7] = [
    "valid",
    "parse_failed",
    "thinking",
    "timeout",
    "exit_failure",
    "output_too_large",
    "stdout_utf8_invalid",
];
pub const DETECT_LABELS: [&str; 5] = ["hit", "miss", "false_alarm", "quiet", "no_reading"];
pub const CAUSE_LABELS: [&str; 6] = [
    "hit",
    "service_only",
    "word_only",
    "none",
    "not_scored",
    "no_reading",
];
/// The process outcomes that leave no stdout to re-grade.
const FAILURE_LABELS: [&str; 4] = [
    "timeout",
    "exit_failure",
    "output_too_large",
    "stdout_utf8_invalid",
];
const CUELESS_MODE_LABEL: &str = "tier3";
const CUED_MODE_LABEL: &str = "tier1";
const WINDOW: Duration = Duration::from_secs(60);
const A6_FINGERPRINT: &str = "3b8d1f60c2a94e7d85f0b6c13a2e9d47";
const A7_FINGERPRINT: &str = "8f4c2a7e1d903b65c0e8a4f71b2d6c39";
const DAY_NANOS: i64 = 86_400_000_000_000;
const SERVICES_HEADER: &str = "SERVICES (rate, error%, p99 vs baselines):";
const TREND_HEADER: &str = "TREND (last 6 windows of 60s, oldest first):";
/// The gpu-primary L4 latency budget (`xtask/ci/l4-latency-p99.sh`).
const GPU_PRIMARY_BUDGET_MS: u64 = 10000;
const RULE_MARGIN_POINTS: f64 = 10.0;
const SEP: &str = " \u{b7} ";
pub const AUDIT_SAMPLE: &str = "audit-sample.md";
pub const AUDIT_KEY: &str = "audit-key.json";
pub const AUDIT_VERDICTS: &str = "audit-verdicts.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    A,
    B,
    C,
}

impl Family {
    pub fn label(self) -> &'static str {
        match self {
            Family::A => "A",
            Family::B => "B",
            Family::C => "C",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Render {
    Today,
    Enriched,
}

impl Render {
    pub fn label(self) -> &'static str {
        match self {
            Render::Today => "today",
            Render::Enriched => "enriched",
        }
    }

    pub fn parse(s: &str) -> Option<Render> {
        match s {
            "today" => Some(Render::Today),
            "enriched" => Some(Render::Enriched),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TrendMetric {
    Rate,
    P99,
}

pub struct Trend {
    pub service: &'static str,
    pub metric: TrendMetric,
    pub values: [f64; 6],
}

pub struct Truth {
    pub expect_surface: bool,
    /// Naming any one of these services satisfies the service atom.
    pub services: &'static [&'static str],
    pub cause_words: &'static [&'static str],
}

pub struct PatternShape {
    pub id: &'static str,
    pub family: Family,
    pub services: Vec<DigestServiceRow>,
    pub cue: Option<AttentionCue>,
    pub commits: Vec<DigestRecentCommit>,
    pub corpus_matches: Vec<String>,
    /// Read only by the enriched render.
    pub trend: Vec<Trend>,
    pub truth: Truth,
}

impl PatternShape {
    /// A cueless digest takes the 60 s Tier3 cadence; a cued one Tier1.
    pub fn mode_label(&self) -> &'static str {
        if self.cue.is_some() {
            CUED_MODE_LABEL
        } else {
            CUELESS_MODE_LABEL
        }
    }

    /// C shapes run every selected render; A and B run today's only.
    pub fn renders(&self, selected: &[Render]) -> Vec<Render> {
        if self.family == Family::C {
            selected.to_vec()
        } else {
            vec![Render::Today]
        }
    }
}

pub fn is_pattern_id(id: &str) -> bool {
    PATTERN_IDS.contains(&id)
}

/// Rate and p99 pairs are `(current, baseline)`; error rates are fractions.
fn svc(name: &str, rate: (f64, f64), error_rate: (f64, f64), p99: (f64, f64)) -> DigestServiceRow {
    DigestServiceRow {
        service: name.to_string(),
        rate_per_sec: rate.0,
        rate_baseline_per_sec: rate.1,
        error_rate: error_rate.0,
        error_rate_baseline: error_rate.1,
        p99_latency_ms: p99.0,
        p99_baseline_ms: p99.1,
    }
}

fn checkout() -> DigestServiceRow {
    svc("checkout-api", (12.0, 12.0), (0.004, 0.004), (180.0, 175.0))
}

fn search() -> DigestServiceRow {
    svc("search-api", (30.0, 29.0), (0.001, 0.001), (90.0, 88.0))
}

fn auth() -> DigestServiceRow {
    svc("auth-service", (15.0, 15.0), (0.0, 0.0), (40.0, 42.0))
}

fn inventory() -> DigestServiceRow {
    svc(
        "inventory-service",
        (7.0, 7.0),
        (0.002, 0.002),
        (60.0, 62.0),
    )
}

fn payment() -> DigestServiceRow {
    svc("payment-service", (9.0, 9.0), (0.002, 0.002), (95.0, 95.0))
}

fn commit(basename: &str, age_seconds: u64, files_changed_count: u32) -> DigestRecentCommit {
    DigestRecentCommit {
        basename: basename.to_string(),
        age_seconds,
        files_changed_count,
    }
}

fn resolved_incident(id: i64, age_nanos: i64) -> Incident {
    let opened = RENDER_NOW_UNIX_NANO - age_nanos;
    Incident {
        id,
        workspace: WORKSPACE.to_string(),
        fingerprint: A6_FINGERPRINT.to_string(),
        title: "Error-rate spike: inventory-service error rate above baseline".to_string(),
        detail: String::new(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: Some("inventory-service".to_string()),
        status: IncidentStatus::Resolved,
        severity: IncidentSeverity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: Vec::new(),
            fingerprint_hashes: Vec::new(),
            timestamps_unix_nano: Vec::new(),
        },
        opened_at_unix_nano: opened,
        updated_at_unix_nano: opened,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: Some(opened + 1_800_000_000_000),
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
}

fn wrong_service_cue() -> AttentionCue {
    AttentionCue {
        kind: CueKind::RetryStorm,
        scope: CueScope::Service,
        scope_id: Some("checkout-api".to_string()),
        magnitude: 8.0,
        absolute_value: 0.12,
        persistence: 30,
        confidence: 0.95,
        priority_tier: PriorityTier::Autonomous,
        suppression_bypassed: false,
        fingerprint: Some(A7_FINGERPRINT.to_string()),
    }
}

const NO_ALARM: Truth = Truth {
    expect_surface: false,
    services: &[],
    cause_words: &[],
};

fn shape(
    id: &'static str,
    family: Family,
    services: Vec<DigestServiceRow>,
    truth: Truth,
) -> PatternShape {
    PatternShape {
        id,
        family,
        services,
        cue: None,
        commits: Vec::new(),
        corpus_matches: Vec::new(),
        trend: Vec::new(),
        truth,
    }
}

pub fn pattern_shapes() -> Vec<PatternShape> {
    vec![
        PatternShape {
            commits: vec![
                commit("cart-service: new price cache client", 180, 4),
                commit("docs: update the on-call runbook", 5700, 1),
            ],
            ..shape(
                "A1",
                Family::A,
                vec![
                    checkout(),
                    svc("cart-service", (10.0, 10.0), (0.18, 0.004), (210.0, 200.0)),
                    search(),
                    auth(),
                ],
                Truth {
                    expect_surface: true,
                    services: &["cart-service"],
                    cause_words: &[
                        "deploy", "release", "commit", "change", "rollout", "rollback",
                    ],
                },
            )
        },
        shape(
            "A2",
            Family::A,
            vec![
                svc("checkout-api", (12.0, 12.0), (0.30, 0.004), (5150.0, 180.0)),
                svc("payment-service", (9.0, 9.0), (0.02, 0.002), (5000.0, 95.0)),
                inventory(),
                auth(),
            ],
            Truth {
                expect_surface: true,
                services: &["payment-service"],
                cause_words: &[
                    "latency",
                    "slow",
                    "timeout",
                    "timed out",
                    "time out",
                    "p99",
                    "response time",
                ],
            },
        ),
        shape(
            "A3",
            Family::A,
            vec![
                checkout(),
                svc("search-api", (29.0, 29.0), (0.008, 0.001), (1400.0, 88.0)),
                svc(
                    "inventory-service",
                    (7.0, 7.0),
                    (0.006, 0.002),
                    (1250.0, 62.0),
                ),
                auth(),
            ],
            Truth {
                expect_surface: true,
                services: &["search-api", "inventory-service"],
                cause_words: &[
                    "shared",
                    "common",
                    "dependency",
                    "downstream",
                    "database",
                    "infrastructure",
                    "network",
                ],
            },
        ),
        shape(
            "A4",
            Family::A,
            vec![
                checkout(),
                search(),
                svc("fraud-scorer", (0.2, 0.2), (0.50, 0.005), (300.0, 280.0)),
                auth(),
            ],
            Truth {
                expect_surface: true,
                services: &["fraud-scorer"],
                cause_words: &["error", "fail", "exception", "broken", "crash"],
            },
        ),
        shape(
            "A5",
            Family::A,
            vec![
                checkout(),
                search(),
                svc("auth-service", (15.0, 15.0), (0.0, 0.0), (2400.0, 42.0)),
                inventory(),
            ],
            Truth {
                expect_surface: true,
                services: &["auth-service"],
                cause_words: &[
                    "saturat",
                    "dependency",
                    "downstream",
                    "database",
                    "capacity",
                    "overload",
                    "contention",
                    "queue",
                    "pool",
                    "exhaust",
                    "bottleneck",
                ],
            },
        ),
        PatternShape {
            corpus_matches: [1, 3, 5]
                .into_iter()
                .zip(21..)
                .map(|(days, id)| {
                    format_corpus_match_line(
                        &resolved_incident(id, days * DAY_NANOS),
                        RENDER_NOW_UNIX_NANO,
                    )
                })
                .collect(),
            ..shape(
                "A6",
                Family::A,
                vec![
                    checkout(),
                    search(),
                    svc(
                        "inventory-service",
                        (7.0, 7.0),
                        (0.06, 0.002),
                        (140.0, 62.0),
                    ),
                    auth(),
                ],
                Truth {
                    expect_surface: true,
                    services: &["inventory-service"],
                    cause_words: &[
                        "recur", "again", "repeat", "previous", "before", "known", "history",
                        "past", "prior",
                    ],
                },
            )
        },
        PatternShape {
            cue: Some(wrong_service_cue()),
            ..shape(
                "A7",
                Family::A,
                vec![
                    svc("checkout-api", (30.0, 12.0), (0.12, 0.004), (950.0, 180.0)),
                    svc("payment-service", (9.0, 9.0), (0.40, 0.002), (900.0, 95.0)),
                    inventory(),
                    auth(),
                ],
                Truth {
                    expect_surface: true,
                    services: &["payment-service"],
                    cause_words: &[
                        "error",
                        "fail",
                        "timeout",
                        "timed out",
                        "unavailable",
                        "down",
                        "latency",
                        "slow",
                        "origin",
                        "root cause",
                    ],
                },
            )
        },
        shape(
            "B1",
            Family::B,
            vec![checkout(), payment(), search(), auth(), inventory()],
            NO_ALARM,
        ),
        shape(
            "B2",
            Family::B,
            vec![
                checkout(),
                svc("search-api", (120.0, 118.0), (0.003, 0.003), (95.0, 92.0)),
                svc("billing-batch", (0.5, 0.5), (0.0, 0.0), (8200.0, 8000.0)),
                auth(),
            ],
            NO_ALARM,
        ),
        PatternShape {
            commits: vec![
                commit("checkout-api: tidy request logging", 240, 2),
                commit("search-api: bump the client timeout constant", 1200, 1),
            ],
            ..shape(
                "B3",
                Family::B,
                vec![checkout(), search(), auth(), inventory()],
                NO_ALARM,
            )
        },
        PatternShape {
            trend: vec![Trend {
                service: "notification-worker",
                metric: TrendMetric::Rate,
                values: [40.2, 39.8, 40.5, 21.0, 6.3, 4.1],
            }],
            ..shape(
                "C1",
                Family::C,
                vec![
                    checkout(),
                    search(),
                    svc("notification-worker", (4.1, 40.0), (0.0, 0.0), (35.0, 38.0)),
                    auth(),
                ],
                Truth {
                    expect_surface: true,
                    services: &["notification-worker"],
                    cause_words: &[
                        "drop",
                        "declin",
                        "fell",
                        "fall",
                        "traffic",
                        "throughput",
                        "volume",
                        "stall",
                        "stopp",
                        "dead",
                        "idle",
                        "consum",
                    ],
                },
            )
        },
        PatternShape {
            trend: vec![Trend {
                service: "cart-service",
                metric: TrendMetric::P99,
                values: [185.0, 230.0, 290.0, 340.0, 400.0, 460.0],
            }],
            ..shape(
                "C2",
                Family::C,
                vec![
                    checkout(),
                    svc("cart-service", (10.0, 10.0), (0.004, 0.004), (460.0, 180.0)),
                    search(),
                    auth(),
                ],
                Truth {
                    expect_surface: true,
                    services: &["cart-service"],
                    cause_words: &[
                        "creep", "rising", "rise", "increas", "grow", "gradual", "degrad", "trend",
                        "climb", "leak",
                    ],
                },
            )
        },
        PatternShape {
            trend: vec![Trend {
                service: "search-indexer",
                metric: TrendMetric::P99,
                values: [2050.0, 135.0, 140.0, 2080.0, 138.0, 2100.0],
            }],
            ..shape(
                "C3",
                Family::C,
                vec![
                    checkout(),
                    search(),
                    svc(
                        "search-indexer",
                        (3.0, 3.0),
                        (0.005, 0.004),
                        (2100.0, 140.0),
                    ),
                    auth(),
                ],
                Truth {
                    expect_surface: true,
                    services: &["search-indexer"],
                    cause_words: &[
                        "periodic", "cron", "schedul", "gc", "garbage", "batch", "interval",
                        "every", "cycle", "recur",
                    ],
                },
            )
        },
    ]
}

/// The listed pattern shapes, in `PATTERN_IDS` order whatever the list order.
pub fn select_pattern_shapes(ids: &[String]) -> Vec<PatternShape> {
    pattern_shapes()
        .into_iter()
        .filter(|s| ids.iter().any(|id| id == s.id))
        .collect()
}

fn find_shape(id: &str) -> Option<PatternShape> {
    pattern_shapes().into_iter().find(|s| s.id == id)
}

fn project(shape: &PatternShape) -> DigestProjectContext {
    DigestProjectContext {
        workspace_canonical_path: WORKSPACE.to_string(),
        project_name: Some("demo-shop".to_string()),
        vcs_type: Some("git"),
        recent_commits: shape.commits.clone(),
        framework_signals: Vec::new(),
    }
}

/// `render_payload`'s SERVICES row, reproduced so the enriched transform can
/// anchor on it.
fn today_service_line(row: &DigestServiceRow) -> String {
    format!(
        "  {}     {:.1}/s | {:.1}% | {:.0}ms",
        row.service,
        row.rate_per_sec,
        row.error_rate * 100.0,
        row.p99_latency_ms
    )
}

fn enriched_service_line(row: &DigestServiceRow) -> String {
    format!(
        "  {}     {:.1}/s (baseline {:.1}/s) | {:.1}% (baseline {:.1}%) | {:.0}ms (baseline {:.0}ms)",
        row.service,
        row.rate_per_sec,
        row.rate_baseline_per_sec,
        row.error_rate * 100.0,
        row.error_rate_baseline * 100.0,
        row.p99_latency_ms,
        row.p99_baseline_ms
    )
}

fn trend_line(t: &Trend) -> String {
    let (metric, values) = match t.metric {
        TrendMetric::Rate => (
            "rate/s",
            t.values
                .iter()
                .map(|v| format!("{v:.1}"))
                .collect::<Vec<_>>(),
        ),
        TrendMetric::P99 => (
            "p99 ms",
            t.values
                .iter()
                .map(|v| format!("{v:.0}"))
                .collect::<Vec<_>>(),
        ),
    };
    format!("  {} {metric}: {}", t.service, values.join(" "))
}

/// Today's payload with every SERVICES row carrying its baselines and a TREND
/// block after the SERVICES block. Each anchor must occur exactly once.
pub fn enrich(payload: &str, shape: &PatternShape) -> Result<String, String> {
    let mut lines: Vec<String> = payload.lines().map(str::to_string).collect();
    if lines.iter().filter(|l| *l == SERVICES_HEADER).count() != 1 {
        return Err("enriched: SERVICES header not found exactly once".to_string());
    }
    if lines.iter().any(|l| l.starts_with("TREND")) {
        return Err("enriched: the payload already carries a TREND block".to_string());
    }
    if shape.trend.is_empty() {
        return Err(format!("enriched: {} carries no trend", shape.id));
    }
    let mut last_row = 0;
    for row in &shape.services {
        let anchor = today_service_line(row);
        let hits: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| **l == anchor)
            .map(|(i, _)| i)
            .collect();
        let [at] = hits[..] else {
            return Err(format!(
                "enriched: SERVICES row {} found {} times, expected 1",
                row.service,
                hits.len()
            ));
        };
        lines[at] = enriched_service_line(row);
        last_row = last_row.max(at);
    }
    let block = std::iter::once(TREND_HEADER.to_string()).chain(shape.trend.iter().map(trend_line));
    lines.splice(last_row + 1..last_row + 1, block);
    Ok(lines.join("\n") + "\n")
}

pub struct Composed {
    pub payload: String,
    pub prompt: String,
}

/// The shape rendered through the product's own `render_payload` and
/// `build_primary_tier_prompt`, checked against the production prompt bound.
pub fn compose(shape: &PatternShape, render: Render) -> Result<Composed, String> {
    let cues: Vec<DigestCueRef> = shape
        .cue
        .iter()
        .map(|c| DigestCueRef {
            kind: c.kind,
            priority_tier: c.priority_tier,
            summary: cue_summary(c),
            scope: c.scope,
            fingerprint: c.fingerprint.clone(),
            scope_id: c.scope_id.clone(),
        })
        .collect();
    let mut payload = render_payload(
        WINDOW,
        shape.mode_label(),
        &project(shape),
        &shape.services,
        &cues,
        &shape.corpus_matches,
        &[],
        false,
    );
    if render == Render::Enriched {
        payload = enrich(&payload, shape)?;
    }
    let citable: Vec<String> = shape
        .cue
        .iter()
        .filter_map(|c| c.fingerprint.clone())
        .collect();
    let prompt =
        build_primary_tier_prompt(&payload, &format!("workspace={WORKSPACE}"), "", &citable);
    validate_prompt_bounded(&prompt).map_err(|r| {
        format!(
            "{} {}: prompt rejected: {}",
            shape.id,
            render.label(),
            r.label()
        )
    })?;
    Ok(Composed { payload, prompt })
}

/// Whether `term` occurs in `lower` at the start of a word.
fn word_start(lower: &str, term: &str) -> bool {
    lower.match_indices(term).any(|(at, _)| {
        lower[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_ascii_alphanumeric())
    })
}

/// The full name, the name with spaces for hyphens, or its first segment at
/// a word start (`payment-service` is named by "payments").
fn names_service(lower: &str, service: &str) -> bool {
    let stem = service.split('-').next().unwrap_or(service);
    lower.contains(service) || lower.contains(&service.replace('-', " ")) || word_start(lower, stem)
}

/// `hit` / `miss` for A and C, `false_alarm` / `quiet` for B; a row that is
/// not `valid` has no reading.
pub fn detect_label(family: Family, valid: &str, decision: Option<Decision>) -> &'static str {
    if valid != "valid" {
        return "no_reading";
    }
    let surfaced = decision == Some(Decision::Surface);
    match (family, surfaced) {
        (Family::B, true) => "false_alarm",
        (Family::B, false) => "quiet",
        (_, true) => "hit",
        (_, false) => "miss",
    }
}

/// Reads the FIRST hypothesis's statement only: the ground-truth service and
/// a cause word both, either alone, or neither. B is not scored.
pub fn cause_label(
    family: Family,
    valid: &str,
    first_statement: Option<&str>,
    truth: &Truth,
) -> &'static str {
    if valid != "valid" {
        return "no_reading";
    }
    if family == Family::B {
        return "not_scored";
    }
    let Some(statement) = first_statement else {
        return "none";
    };
    let lower = statement.to_ascii_lowercase();
    let service = truth.services.iter().any(|s| names_service(&lower, s));
    let word = truth.cause_words.iter().any(|w| word_start(&lower, w));
    match (service, word) {
        (true, true) => "hit",
        (true, false) => "service_only",
        (false, true) => "word_only",
        (false, false) => "none",
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Graded {
    pub valid: &'static str,
    pub detect: &'static str,
    pub cause: &'static str,
    pub decision: Option<&'static str>,
    pub severity: Option<&'static str>,
}

pub enum Outcome<'a> {
    Stdout(&'a str),
    Failed(&'static str),
}

/// One generation's labels, from its outcome alone: the same path grades at
/// generation time and over stored outputs.
pub fn grade(shape: &PatternShape, outcome: Outcome<'_>) -> Graded {
    let (valid, parsed) = match outcome {
        Outcome::Failed(why) => (why, None),
        Outcome::Stdout(text) => {
            let parsed: Option<L4Output> = extract_json_object_bounded(text)
                .ok()
                .and_then(|obj| parse_bounded(obj.as_bytes()).ok());
            let valid = if thinking_label(Some(text)) == "present" {
                "thinking"
            } else if parsed.is_some() {
                "valid"
            } else {
                "parse_failed"
            };
            (valid, parsed)
        }
    };
    let first = parsed
        .as_ref()
        .and_then(|o| o.hypotheses.first())
        .map(|h| h.statement.as_str());
    Graded {
        valid,
        detect: detect_label(shape.family, valid, parsed.as_ref().map(|o| o.decision)),
        cause: cause_label(shape.family, valid, first, &shape.truth),
        decision: parsed.as_ref().map(|o| decision_label(o.decision)),
        severity: parsed.as_ref().map(|o| severity_label(o.severity)),
    }
}

pub fn text_path(dir: &Path, shape: &str, render: Render, run: u32, kind: &str) -> PathBuf {
    dir.join("texts")
        .join(format!("{shape}-{}-{run}.{kind}.txt", render.label()))
}

pub fn row_json(
    arm: &str,
    shape: &PatternShape,
    render: Render,
    run: u32,
    graded: &Graded,
    thinking: &str,
    metrics: Metrics,
) -> Value {
    json!({
        "arm": arm,
        "shape": shape.id,
        "family": shape.family.label(),
        "render": render.label(),
        "run": run,
        "valid": graded.valid,
        "detect": graded.detect,
        "cause": graded.cause,
        "decision": graded.decision,
        "severity": graded.severity,
        "thinking": thinking,
        "elapsed_ms": metrics.elapsed_ms,
        "peak_rss_kib": metrics.peak_rss_kib,
        "peak_vram_mib": metrics.peak_vram_mib,
    })
}

fn refused_outside_target() -> String {
    "the out dir must resolve under target/ (texts are kept there only)".to_string()
}

/// `dir` made absolute against `cwd`, refused unless it lies lexically under
/// `cwd`'s `target/` with no `..` component.
fn lexically_under_target(dir: &Path, cwd: &Path) -> Result<PathBuf, String> {
    let absolute = if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        cwd.join(dir)
    };
    if absolute.components().any(|c| c == Component::ParentDir)
        || !absolute.starts_with(cwd.join("target"))
    {
        return Err(refused_outside_target());
    }
    Ok(absolute)
}

/// The existing `absolute` resolved, refused unless it still lies under the
/// resolved `target/` (so no symlink leads out).
fn resolved_under_target(absolute: &Path, cwd: &Path) -> Result<PathBuf, String> {
    let target = cwd
        .join("target")
        .canonicalize()
        .map_err(|_| "target/ not resolvable".to_string())?;
    let resolved = absolute
        .canonicalize()
        .map_err(|_| "output directory not resolvable".to_string())?;
    if !resolved.starts_with(&target) {
        return Err(refused_outside_target());
    }
    Ok(resolved)
}

/// The out dir with its `texts/` created, only under `cwd`'s `target/`: the
/// lexical check runs first, so nothing is created anywhere else.
pub fn texts_root(out: &Path, cwd: &Path) -> Result<PathBuf, String> {
    let absolute = lexically_under_target(out, cwd)?;
    std::fs::create_dir_all(absolute.join("texts"))
        .map_err(|_| "output directory not creatable".to_string())?;
    resolved_under_target(&absolute, cwd)
}

/// An existing series root, under `cwd`'s `target/` like the dirs it holds.
pub fn series_root(root: &Path, cwd: &Path) -> Result<PathBuf, String> {
    resolved_under_target(&lexically_under_target(root, cwd)?, cwd)
}

/// A pattern row as read back from `runs.json`, re-graded by the current code.
pub struct StoredRow {
    pub shape: String,
    pub family: Family,
    pub render: Render,
    pub run: u32,
    pub stored: Graded,
    pub graded: Graded,
    pub elapsed_ms: Option<u64>,
    pub peak_rss_kib: Option<u64>,
    pub peak_vram_mib: Option<u64>,
}

fn str_field<'a>(row: &'a Map<String, Value>, key: &str) -> Result<&'a str, String> {
    row.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("runs.json: a row has no {key}"))
}

fn closed(set: &[&'static str], value: &str, what: &str) -> Result<&'static str, String> {
    set.iter()
        .find(|v| **v == value)
        .copied()
        .ok_or_else(|| format!("runs.json: {what} {value} is outside its closed set"))
}

fn opt_closed(row: &Map<String, Value>, key: &str) -> Option<&'static str> {
    const LABELS: [&str; 7] = [
        "surface",
        "dismiss",
        "watch",
        "autonomous",
        "suggested",
        "curious",
        "none",
    ];
    let v = row.get(key)?.as_str()?;
    LABELS.iter().find(|l| **l == v).copied()
}

/// Every row of one model dir, re-graded from its stored stdout through the
/// same extract, parse and score path; nothing is spawned.
pub fn regrade_model_dir(dir: &Path) -> Result<Vec<StoredRow>, String> {
    let text = std::fs::read_to_string(dir.join("runs.json"))
        .map_err(|_| format!("{}: runs.json unreadable", super::basename(dir)))?;
    let rows: Vec<Value> =
        serde_json::from_str(&text).map_err(|_| "runs.json is not a JSON array".to_string())?;
    let mut out = Vec::new();
    for row in &rows {
        let row = row.as_object().ok_or("runs.json: a row is not an object")?;
        let shape_id = str_field(row, "shape")?;
        let shape = find_shape(shape_id)
            .ok_or_else(|| format!("runs.json: {shape_id} is not a pattern shape"))?;
        let render = Render::parse(str_field(row, "render")?)
            .ok_or("runs.json: a row's render is unknown")?;
        let run = row
            .get("run")
            .and_then(Value::as_u64)
            .and_then(|r| u32::try_from(r).ok())
            .ok_or("runs.json: a row has no run")?;
        let stored = Graded {
            valid: closed(&VALID_LABELS, str_field(row, "valid")?, "valid")?,
            detect: closed(&DETECT_LABELS, str_field(row, "detect")?, "detect")?,
            cause: closed(&CAUSE_LABELS, str_field(row, "cause")?, "cause")?,
            decision: opt_closed(row, "decision"),
            severity: opt_closed(row, "severity"),
        };
        let graded = if FAILURE_LABELS.contains(&stored.valid) {
            grade(&shape, Outcome::Failed(stored.valid))
        } else {
            let path = text_path(dir, shape_id, render, run, "stdout");
            let stdout = std::fs::read_to_string(&path).map_err(|_| {
                format!("stored output missing: {shape_id}-{}-{run}", render.label())
            })?;
            grade(&shape, Outcome::Stdout(&stdout))
        };
        let reading = |key: &str| row.get(key).and_then(Value::as_u64);
        out.push(StoredRow {
            shape: shape_id.to_string(),
            family: shape.family,
            render,
            run,
            stored,
            graded,
            elapsed_ms: reading("elapsed_ms"),
            peak_rss_kib: reading("peak_rss_kib"),
            peak_vram_mib: reading("peak_vram_mib"),
        });
    }
    Ok(out)
}

/// The model dirs of a series root (each holding a `runs.json`), by name.
pub fn model_dirs(root: &Path) -> Result<Vec<(String, PathBuf)>, String> {
    let entries = std::fs::read_dir(root).map_err(|_| "series root unreadable".to_string())?;
    let mut dirs: Vec<(String, PathBuf)> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.join("runs.json").is_file())
        .map(|p| (super::basename(&p), p))
        .collect();
    dirs.sort();
    if dirs.is_empty() {
        return Err("series root holds no model dir with a runs.json".to_string());
    }
    Ok(dirs)
}

pub struct SplitMix64(u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// ceil(10 %) of `n` distinct indices, by a seeded partial Fisher-Yates.
pub fn audit_draw_indices(n: usize, seed: u64) -> Vec<usize> {
    let k = n.div_ceil(10);
    let mut idx: Vec<usize> = (0..n).collect();
    let mut rng = SplitMix64::new(seed);
    for i in 0..k {
        let span = u64::try_from(n - i).unwrap_or(u64::MAX);
        let j = i + usize::try_from(rng.next_u64() % span).unwrap_or(0);
        idx.swap(i, j);
    }
    idx.truncate(k);
    idx
}

fn truth_text(shape: &PatternShape) -> String {
    if !shape.truth.expect_surface {
        return "expected decision: not surface (dismiss or watch); cause not scored".to_string();
    }
    format!(
        "expected decision: surface; service: {}; cause words: {}",
        shape.truth.services.join(" | "),
        shape.truth.cause_words.join(" | ")
    )
}

/// The drawn rows with the model hidden: `audit-sample.md` carries an opaque
/// id per row, `audit-key.json` maps each id back to its model.
pub fn audit_draw(root: &Path, seed: u64) -> Result<(usize, usize), String> {
    let mut all = Vec::new();
    for (stem, dir) in model_dirs(root)? {
        for row in regrade_model_dir(&dir)? {
            all.push((stem.clone(), dir.clone(), row));
        }
    }
    let drawn = audit_draw_indices(all.len(), seed);
    let mut sample = String::from(
        "# Blind audit sample\n\nOne section per drawn row; the model is hidden. Record one line per row in \
         audit-verdicts.txt beside this file: `{id} agree` when the keyword labels match what the \
         output says against the ground truth, `{id} disagree` when they do not.\n",
    );
    let mut key = Map::new();
    for (j, &i) in drawn.iter().enumerate() {
        let (stem, dir, row) = &all[i];
        let id = format!("s{:03}", j + 1);
        let shape = find_shape(&row.shape).ok_or("a drawn row's shape is unknown")?;
        let digest =
            std::fs::read_to_string(text_path(dir, &row.shape, row.render, row.run, "digest"))
                .map_err(|_| format!("stored digest missing for {id}"))?;
        let output =
            std::fs::read_to_string(text_path(dir, &row.shape, row.render, row.run, "stdout"))
                .ok()
                .and_then(|s| extract_json_object_bounded(&s).ok().map(str::to_string))
                .unwrap_or_else(|| format!("(no JSON object: {})", row.graded.valid));
        sample.push_str(&format!(
            "\n## {id}\n\n- shape: {} ({} render)\n- ground truth: {}\n- keyword labels: valid {}, detect {}, cause {}\n\nDigest:\n\n```text\n{}```\n\nModel output:\n\n```json\n{}\n```\n",
            row.shape,
            row.render.label(),
            truth_text(&shape),
            row.graded.valid,
            row.graded.detect,
            row.graded.cause,
            digest,
            output
        ));
        key.insert(
            id,
            json!({
                "model": stem,
                "shape": row.shape,
                "render": row.render.label(),
                "run": row.run,
            }),
        );
    }
    std::fs::write(root.join(AUDIT_SAMPLE), sample).map_err(|_| "audit-sample.md not writable")?;
    let key_text = serde_json::to_string_pretty(&Value::Object(key))
        .map_err(|_| "audit-key.json not serializable".to_string())?;
    std::fs::write(root.join(AUDIT_KEY), key_text).map_err(|_| "audit-key.json not writable")?;
    Ok((drawn.len(), all.len()))
}

/// The disagreements among `verdicts` over exactly the drawn `ids`.
pub fn grade_audit(ids: &[String], verdicts: &str) -> Result<(usize, usize), String> {
    let mut seen: BTreeMap<&str, bool> = BTreeMap::new();
    for (n, line) in verdicts.lines().enumerate() {
        let line = line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        let (id, word) = line
            .split_once(' ')
            .ok_or_else(|| format!("audit-verdicts.txt line {} unreadable", n + 1))?;
        let disagree = match word.trim() {
            "agree" => false,
            "disagree" => true,
            _ => return Err(format!("audit-verdicts.txt line {} unreadable", n + 1)),
        };
        if !ids.iter().any(|i| i == id) {
            return Err(format!("audit-verdicts.txt names an undrawn row {id}"));
        }
        if seen.insert(id, disagree).is_some() {
            return Err(format!("audit-verdicts.txt names {id} twice"));
        }
    }
    if let Some(missing) = ids.iter().find(|i| !seen.contains_key(i.as_str())) {
        return Err(format!("no verdict for {missing}"));
    }
    Ok((seen.values().filter(|d| **d).count(), ids.len()))
}

/// `regrade` when the disagreement is above 10 % of the sample.
pub fn audit_disposition(disagree: usize, sample: usize) -> &'static str {
    if disagree * 10 > sample {
        "regrade"
    } else {
        "agree"
    }
}

pub fn audit_grade(root: &Path) -> Result<(usize, usize), String> {
    let key: Map<String, Value> = std::fs::read_to_string(root.join(AUDIT_KEY))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .ok_or("audit-key.json unreadable (run --audit-draw first)")?;
    let ids: Vec<String> = key.keys().cloned().collect();
    let verdicts = std::fs::read_to_string(root.join(AUDIT_VERDICTS))
        .map_err(|_| "audit-verdicts.txt unreadable".to_string())?;
    grade_audit(&ids, &verdicts)
}

/// A percentage cell: `hits` of the `readable` rows; the unreadable rows are
/// tallied by their `valid` label and excluded from the percentage.
#[derive(Default, Clone)]
pub struct Cell {
    pub hits: u32,
    pub readable: u32,
    pub unreadable: BTreeMap<&'static str, u32>,
}

impl Cell {
    fn add(&mut self, valid: &'static str, hit: bool) {
        if valid == "valid" {
            self.readable += 1;
            self.hits += u32::from(hit);
        } else {
            *self.unreadable.entry(valid).or_default() += 1;
        }
    }

    pub fn pct(&self) -> Option<f64> {
        (self.readable > 0).then(|| f64::from(self.hits) * 100.0 / f64::from(self.readable))
    }

    pub fn text(&self) -> String {
        match self.pct() {
            Some(p) => format!("{p:.0}% ({}/{})", self.hits, self.readable),
            None if self.unreadable.is_empty() => "cannot-evaluate (no rows)".to_string(),
            None => format!(
                "cannot-evaluate (0 readable: {})",
                self.unreadable
                    .iter()
                    .map(|(k, v)| format!("{k} {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Default, Clone)]
pub struct ModelSummary {
    pub stem: String,
    pub a_detect: Cell,
    pub a_cause: Cell,
    pub b_false_alarm: Cell,
    pub c_today_detect: Cell,
    pub c_today_cause: Cell,
    pub c_enriched_detect: Cell,
    pub c_enriched_cause: Cell,
    pub no_reading: u32,
    pub rows: u32,
    pub changed: u32,
    pub elapsed: Vec<u64>,
    pub peak_vram_mib: Option<u64>,
    pub peak_rss_kib: Option<u64>,
}

pub fn summarize(stem: &str, rows: &[StoredRow]) -> ModelSummary {
    let mut s = ModelSummary {
        stem: stem.to_string(),
        ..ModelSummary::default()
    };
    for r in rows {
        let g = &r.graded;
        let (detect_hit, cause_hit) = (g.detect == "hit", g.cause == "hit");
        match (r.family, r.render) {
            (Family::A, _) => {
                s.a_detect.add(g.valid, detect_hit);
                s.a_cause.add(g.valid, cause_hit);
            }
            (Family::B, _) => s.b_false_alarm.add(g.valid, g.detect == "false_alarm"),
            (Family::C, Render::Today) => {
                s.c_today_detect.add(g.valid, detect_hit);
                s.c_today_cause.add(g.valid, cause_hit);
            }
            (Family::C, Render::Enriched) => {
                s.c_enriched_detect.add(g.valid, detect_hit);
                s.c_enriched_cause.add(g.valid, cause_hit);
            }
        }
        s.rows += 1;
        s.no_reading += u32::from(g.valid != "valid");
        s.changed += u32::from(r.graded != r.stored);
        s.elapsed.extend(r.elapsed_ms);
        s.peak_vram_mib = s.peak_vram_mib.max(r.peak_vram_mib);
        s.peak_rss_kib = s.peak_rss_kib.max(r.peak_rss_kib);
    }
    s
}

/// One run's per-family counts: hits over readable rows.
pub fn patterns_line(s: &ModelSummary) -> String {
    let n = |c: &Cell| format!("{}/{}", c.hits, c.readable);
    [
        format!(
            "patterns A detect {} cause {}",
            n(&s.a_detect),
            n(&s.a_cause)
        ),
        format!("B false_alarm {}", n(&s.b_false_alarm)),
        format!(
            "C today detect {} cause {}",
            n(&s.c_today_detect),
            n(&s.c_today_cause)
        ),
        format!(
            "C enriched detect {} cause {}",
            n(&s.c_enriched_detect),
            n(&s.c_enriched_cause)
        ),
        format!("no_reading {}/{}", s.no_reading, s.rows),
    ]
    .join(SEP)
}

pub fn table_line(s: &ModelSummary) -> String {
    let over = s
        .elapsed
        .iter()
        .filter(|ms| **ms > GPU_PRIMARY_BUDGET_MS)
        .count();
    [
        format!("table: {}", s.stem),
        format!("A detect {}", s.a_detect.text()),
        format!("A cause {}", s.a_cause.text()),
        format!("B false_alarm {}", s.b_false_alarm.text()),
        format!(
            "C today detect {} cause {}",
            s.c_today_detect.text(),
            s.c_today_cause.text()
        ),
        format!(
            "C enriched detect {} cause {}",
            s.c_enriched_detect.text(),
            s.c_enriched_cause.text()
        ),
        format!("no_reading {}/{}", s.no_reading, s.rows),
        format!(
            "gpu_ms p50 {} max {}",
            reading(p50(&s.elapsed)),
            reading(s.elapsed.iter().copied().max())
        ),
        format!("over_{GPU_PRIMARY_BUDGET_MS}ms {over}/{}", s.elapsed.len()),
        format!("peak_vram_mib {}", reading(s.peak_vram_mib)),
        format!("peak_rss_kib {}", reading(s.peak_rss_kib)),
    ]
    .join(SEP)
}

pub struct Recommendation {
    pub best_a_cause: Option<f64>,
    pub best_b_false_alarm: Option<f64>,
    pub qualifying: Vec<String>,
    pub pick: Option<String>,
}

/// The lightest model (max peak VRAM, then max peak RSS) whose A-cause is
/// within 10 points of the best and whose B false alarm is no worse than the
/// best + 10 points. A model with either cell unevaluable never qualifies.
pub fn recommend(models: &[ModelSummary]) -> Recommendation {
    let best_a = models
        .iter()
        .filter_map(|m| m.a_cause.pct())
        .fold(None, |acc: Option<f64>, p| {
            Some(acc.map_or(p, |a| a.max(p)))
        });
    let best_b = models
        .iter()
        .filter_map(|m| m.b_false_alarm.pct())
        .fold(None, |acc: Option<f64>, p| {
            Some(acc.map_or(p, |b| b.min(p)))
        });
    let mut qualifying: Vec<&ModelSummary> = models
        .iter()
        .filter(
            |m| match (m.a_cause.pct(), m.b_false_alarm.pct(), best_a, best_b) {
                (Some(a), Some(b), Some(ba), Some(bb)) => {
                    a >= ba - RULE_MARGIN_POINTS - 1e-9 && b <= bb + RULE_MARGIN_POINTS + 1e-9
                }
                _ => false,
            },
        )
        .collect();
    qualifying.sort_by_key(|m| {
        (
            m.peak_vram_mib.unwrap_or(u64::MAX),
            m.peak_rss_kib.unwrap_or(u64::MAX),
            m.stem.clone(),
        )
    });
    Recommendation {
        best_a_cause: best_a,
        best_b_false_alarm: best_b,
        pick: qualifying.first().map(|m| m.stem.clone()),
        qualifying: qualifying.iter().map(|m| m.stem.clone()).collect(),
    }
}

/// The whole series re-graded from its stored outputs: one table line per
/// model, the rule's inputs and the recommendation.
pub fn table(root: &Path) -> Result<Vec<String>, String> {
    let mut models = Vec::new();
    for (stem, dir) in model_dirs(root)? {
        models.push(summarize(&stem, &regrade_model_dir(&dir)?));
    }
    let mut lines: Vec<String> = models.iter().map(table_line).collect();
    let changed: u32 = models.iter().map(|m| m.changed).sum();
    let rows: u32 = models.iter().map(|m| m.rows).sum();
    lines.push(format!(
        "table: regrade changed {changed} of {rows} row labels"
    ));
    let rec = recommend(&models);
    let pct =
        |p: Option<f64>| p.map_or_else(|| "cannot-evaluate".to_string(), |p| format!("{p:.0}%"));
    lines.push(format!(
        "rule: best A cause {}{SEP}best B false_alarm {}{SEP}qualifying {}",
        pct(rec.best_a_cause),
        pct(rec.best_b_false_alarm),
        if rec.qualifying.is_empty() {
            "none".to_string()
        } else {
            rec.qualifying.join(",")
        }
    ));
    lines.push(format!(
        "recommendation: {}",
        rec.pick.as_deref().unwrap_or("none")
    ));
    Ok(lines)
}

/// The shape-render pairs a run composes, in shape order.
pub fn shape_renders(shapes: &[PatternShape], renders: &[Render]) -> Vec<(usize, Render)> {
    let mut out = Vec::new();
    for (i, s) in shapes.iter().enumerate() {
        for r in s.renders(renders) {
            out.push((i, r));
        }
    }
    out
}

/// The distinct renders listed, in `Render` order.
pub fn parse_renders(value: &str) -> Result<Vec<Render>, String> {
    let mut out = BTreeSet::new();
    for item in value.split(',').map(str::trim) {
        out.insert(Render::parse(item).ok_or_else(|| format!("unknown render {item}"))?);
    }
    Ok(out.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use interpretation::schema::{Confidence, Hypothesis, Severity};

    fn shape_of(id: &str) -> PatternShape {
        find_shape(id).unwrap_or_else(|| panic!("pattern shape {id} exists"))
    }

    fn model_json(decision: &str, statement: &str) -> String {
        let out = L4Output {
            schema_version: "2.0".to_string(),
            prompt_version: "v2.4".to_string(),
            decision: match decision {
                "surface" => Decision::Surface,
                "watch" => Decision::Watch,
                _ => Decision::Dismiss,
            },
            severity: Severity::Suggested,
            title: "t".to_string(),
            symptom: "s".to_string(),
            timeline: "Started 2m ago".to_string(),
            hypotheses: vec![Hypothesis {
                statement: statement.to_string(),
                confidence: Confidence::High,
                justification: "observed".to_string(),
            }],
            investigation_steps: Vec::new(),
            evidence_refs: Vec::new(),
            fingerprint: "f".to_string(),
            model_tier: "primary".to_string(),
            hardware_profile: "gpu_primary".to_string(),
            is_resolution_summary: false,
        };
        let json = serde_json::to_string(&out).unwrap_or_default();
        format!("> prompt\n{json}\n[ Prompt: 1 t/s | Generation: 1 t/s ]\n")
    }

    fn graded(id: &str, decision: &str, statement: &str) -> Graded {
        let stdout = model_json(decision, statement);
        grade(&shape_of(id), Outcome::Stdout(&stdout))
    }

    #[test]
    fn pattern_shape_ids_are_the_thirteen_in_order() {
        let ids: Vec<&str> = pattern_shapes().iter().map(|s| s.id).collect();
        assert_eq!(ids, PATTERN_IDS);
        assert!(PATTERN_IDS.iter().all(|id| is_pattern_id(id)));
        assert!(!is_pattern_id("S1"));
    }

    #[test]
    fn pattern_ground_truth_matches_the_family() {
        for s in pattern_shapes() {
            let scored = s.family != Family::B;
            assert_eq!(s.truth.expect_surface, scored, "{}", s.id);
            assert_eq!(!s.truth.services.is_empty(), scored, "{}", s.id);
            assert_eq!(!s.truth.cause_words.is_empty(), scored, "{}", s.id);
            for row in &s.services {
                assert!(row.service.is_ascii(), "{}", s.id);
            }
            if scored {
                for want in s.truth.services {
                    assert!(
                        s.services.iter().any(|r| r.service == *want),
                        "{}: {want} is in the digest",
                        s.id
                    );
                }
            }
        }
    }

    #[test]
    fn every_pattern_shape_and_render_composes_within_the_production_bound() {
        let shapes = pattern_shapes();
        let pairs = shape_renders(&shapes, &[Render::Today, Render::Enriched]);
        assert_eq!(pairs.len(), 16, "13 shapes, C in two renders");
        for (i, render) in pairs {
            let s = &shapes[i];
            let composed = compose(s, render);
            let Ok(c) = composed else {
                panic!("{} {} composes: {:?}", s.id, render.label(), composed.err());
            };
            assert!(c.prompt.len() <= pulse_app::llamacli_inference::MAX_PROMPT_BYTES);
            assert!(c.prompt.contains(&c.payload), "{}", s.id);
        }
    }

    #[test]
    fn pattern_cueless_shapes_render_no_trigger_and_a_nominal_overall() {
        for s in pattern_shapes() {
            let Ok(c) = compose(&s, Render::Today) else {
                panic!("{} composes", s.id);
            };
            let triggers = c
                .payload
                .lines()
                .filter(|l| l.starts_with("TRIGGER: "))
                .count();
            if s.id == "A7" {
                assert_eq!(triggers, 1, "A7 carries one TRIGGER line");
                assert!(c.payload.contains("WINDOW: 60s, tier1 cadence"));
                assert!(c.payload.contains("OVERALL: anomalous"));
            } else {
                assert_eq!(triggers, 0, "{} is cueless", s.id);
                assert!(c.payload.contains("WINDOW: 60s, tier3 cadence"), "{}", s.id);
                assert!(c.payload.contains("OVERALL: nominal"), "{}", s.id);
            }
        }
    }

    #[test]
    fn pattern_a7_cue_sits_on_the_wrong_service() {
        let s = shape_of("A7");
        let cue_service = s.cue.as_ref().and_then(|c| c.scope_id.as_deref());
        assert_eq!(cue_service, Some("checkout-api"));
        assert_eq!(s.truth.services, ["payment-service"]);
        let row = |name: &str| {
            s.services
                .iter()
                .find(|r| r.service == name)
                .map(|r| r.error_rate)
        };
        assert!(
            row("payment-service") > row("checkout-api"),
            "Y is the origin by the numbers"
        );
    }

    #[test]
    fn pattern_a4_is_low_volume_at_half_errors() {
        let s = shape_of("A4");
        let Some(row) = s.services.iter().find(|r| r.service == "fraud-scorer") else {
            panic!("A4 carries fraud-scorer");
        };
        assert!((row.rate_per_sec - 0.2).abs() < 1e-9);
        assert!((row.error_rate - 0.5).abs() < 1e-9);
    }

    #[test]
    fn pattern_a6_renders_one_fingerprint_several_times() {
        let Ok(c) = compose(&shape_of("A6"), Render::Today) else {
            panic!("A6 composes");
        };
        let matches = c
            .payload
            .lines()
            .filter(|l| l.starts_with("  - [") && l.contains(A6_FINGERPRINT))
            .count();
        assert_eq!(matches, 3);
        assert_eq!(c.payload.matches(", resolved").count(), 3);
    }

    #[test]
    fn enriched_transform_changes_only_services_and_adds_trend_once() {
        for s in pattern_shapes()
            .into_iter()
            .filter(|s| s.family == Family::C)
        {
            let (Ok(today), Ok(enriched)) =
                (compose(&s, Render::Today), compose(&s, Render::Enriched))
            else {
                panic!("{} composes in both renders", s.id);
            };
            let headers = enriched
                .payload
                .lines()
                .filter(|l| *l == TREND_HEADER)
                .count();
            assert_eq!(headers, 1, "{}", s.id);
            let trend: Vec<String> = s.trend.iter().map(trend_line).collect();
            let kept: Vec<&str> = enriched
                .payload
                .lines()
                .filter(|l| *l != TREND_HEADER && !trend.iter().any(|t| t == l))
                .collect();
            let base: Vec<&str> = today.payload.lines().collect();
            assert_eq!(kept.len(), base.len(), "{}", s.id);
            for (k, b) in kept.iter().zip(&base) {
                let row = s.services.iter().find(|r| today_service_line(r) == *b);
                match row {
                    Some(r) => assert_eq!(*k, enriched_service_line(r), "{}", s.id),
                    None => assert_eq!(k, b, "{}: a non-SERVICES line moved", s.id),
                }
            }
            assert!(enriched.payload.contains("(baseline"), "{}", s.id);
            assert!(!today.payload.contains("TREND"), "{}", s.id);
        }
    }

    #[test]
    fn enriched_transform_refuses_a_missing_anchor() {
        let s = shape_of("C1");
        let refused = enrich("WINDOW: 60s, tier3 cadence\n", &s).err();
        assert_eq!(
            refused.as_deref(),
            Some("enriched: SERVICES header not found exactly once")
        );
        let no_trend = shape_of("A1");
        let Ok(c) = compose(&no_trend, Render::Today) else {
            panic!("A1 composes");
        };
        assert!(enrich(&c.payload, &no_trend).is_err());
    }

    #[test]
    fn scorer_label_sets_are_exactly_the_closed_sets() {
        let mut valid = BTreeSet::new();
        let mut detect = BTreeSet::new();
        let mut cause = BTreeSet::new();
        let outcomes: Vec<String> = vec![
            model_json("surface", "cart-service errors after the deploy"),
            model_json("dismiss", "cart-service is fine"),
            model_json("surface", "a recent deploy"),
            model_json("watch", "nothing"),
            "[Start thinking]\nhmm\n[End thinking]\n".to_string() + &model_json("surface", "x"),
            "no json here".to_string(),
        ];
        for id in ["A1", "B1", "C1"] {
            let s = shape_of(id);
            for o in &outcomes {
                let g = grade(&s, Outcome::Stdout(o));
                valid.insert(g.valid);
                detect.insert(g.detect);
                cause.insert(g.cause);
            }
            for f in FAILURE_LABELS {
                let g = grade(&s, Outcome::Failed(f));
                valid.insert(g.valid);
                detect.insert(g.detect);
                cause.insert(g.cause);
            }
        }
        let set = |a: &[&'static str]| a.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(valid, set(&VALID_LABELS));
        assert_eq!(detect, set(&DETECT_LABELS));
        assert_eq!(cause, set(&CAUSE_LABELS));
    }

    #[test]
    fn scorer_detect_pairs_split_surface_from_the_rest() {
        assert_eq!(graded("A1", "surface", "x").detect, "hit");
        assert_eq!(graded("A1", "dismiss", "x").detect, "miss");
        assert_eq!(graded("B1", "surface", "x").detect, "false_alarm");
        assert_eq!(graded("B1", "watch", "x").detect, "quiet");
    }

    #[test]
    fn scorer_cause_needs_the_service_and_a_cause_word() {
        let hit = graded(
            "A1",
            "surface",
            "The cart-service deploy 3 minutes ago broke pricing",
        );
        assert_eq!(hit.cause, "hit");
        let service_only = graded("A1", "surface", "cart-service errors are elevated");
        assert_eq!(service_only.cause, "service_only");
        let word_only = graded(
            "A7",
            "surface",
            "checkout-api retries amplify its own errors",
        );
        assert_eq!(word_only.cause, "word_only");
        let none = graded("A7", "surface", "checkout-api is busy");
        assert_eq!(none.cause, "none");
        assert_eq!(
            graded("B3", "dismiss", "the deploy changed nothing").cause,
            "not_scored"
        );
    }

    #[test]
    fn scorer_cause_reads_the_first_hypothesis_only() {
        let s = shape_of("A1");
        let mut out: L4Output = serde_json::from_str(
            extract_json_object_bounded(&model_json("surface", "load rose")).unwrap_or("{}"),
        )
        .unwrap_or_else(|_| panic!("the fixture parses"));
        out.hypotheses.push(Hypothesis {
            statement: "the cart-service deploy".to_string(),
            confidence: Confidence::Low,
            justification: "x".to_string(),
        });
        let stdout = serde_json::to_string(&out).unwrap_or_default();
        assert_eq!(grade(&s, Outcome::Stdout(&stdout)).cause, "none");
    }

    #[test]
    fn scorer_zero_generation_or_parse_failed_row_is_no_reading() {
        let banner_only = "Loading model...\nbuild: b9305\n> prompt\n\nExiting...\n";
        for id in ["A1", "B1", "C2"] {
            let g = grade(&shape_of(id), Outcome::Stdout(banner_only));
            assert_eq!(g.valid, "parse_failed", "{id}");
            assert_eq!(g.detect, "no_reading", "{id}");
            assert_eq!(g.cause, "no_reading", "{id}");
        }
        let truncated = grade(
            &shape_of("B2"),
            Outcome::Stdout("{\"decision\":\"dismiss\""),
        );
        assert_eq!(truncated.detect, "no_reading");
        for f in FAILURE_LABELS {
            assert_eq!(
                grade(&shape_of("B1"), Outcome::Failed(f)).detect,
                "no_reading"
            );
        }
    }

    #[test]
    fn scorer_thinking_present_is_not_valid() {
        let stdout = "[Start thinking]\nplan\n[End thinking]\n".to_string()
            + &model_json("surface", "cart-service deploy");
        let g = grade(&shape_of("A1"), Outcome::Stdout(&stdout));
        assert_eq!(g.valid, "thinking");
        assert_eq!(g.detect, "no_reading");
        assert_eq!(g.decision, Some("surface"));
    }

    #[test]
    fn scorer_service_match_takes_the_stem_at_a_word_start_only() {
        let lower = "the payments path is slow";
        assert!(names_service(lower, "payment-service"));
        assert!(!names_service("prepayment checks", "payment-service"));
        assert!(names_service("search api is slow", "search-api"));
        assert!(word_start("saturation of the pool", "saturat"));
        assert!(!word_start("unsaturated", "saturat"));
    }

    fn temp_root() -> tempfile::TempDir {
        tempfile::tempdir().unwrap_or_else(|_| panic!("a temp dir"))
    }

    #[test]
    fn out_dir_guard_accepts_only_paths_under_target() {
        let cwd = temp_root();
        let cwd = cwd.path();
        assert!(std::fs::create_dir_all(cwd.join("target")).is_ok());
        assert!(texts_root(Path::new("target/l4/x"), cwd).is_ok());
        assert!(cwd.join("target/l4/x/texts").is_dir());
        for refused in ["elsewhere/x", "target/../elsewhere", "/"] {
            assert!(texts_root(Path::new(refused), cwd).is_err(), "{refused}");
        }
        assert!(
            !cwd.join("elsewhere").exists(),
            "nothing created outside target/"
        );
    }

    #[cfg(unix)]
    #[test]
    fn out_dir_guard_refuses_a_symlink_out_of_target() {
        let cwd = temp_root();
        let cwd = cwd.path();
        assert!(std::fs::create_dir_all(cwd.join("target")).is_ok());
        assert!(std::fs::create_dir_all(cwd.join("outside")).is_ok());
        assert!(std::os::unix::fs::symlink(cwd.join("outside"), cwd.join("target/link")).is_ok());
        assert!(texts_root(Path::new("target/link/x"), cwd).is_err());
    }

    #[test]
    fn audit_draw_is_seeded_and_takes_ceil_ten_percent() {
        let a = audit_draw_indices(960, 20261005);
        assert_eq!(a.len(), 96);
        assert_eq!(a, audit_draw_indices(960, 20261005));
        assert_ne!(a, audit_draw_indices(960, 7));
        assert_eq!(a.iter().collect::<BTreeSet<_>>().len(), 96, "distinct rows");
        assert!(a.iter().all(|i| *i < 960));
        assert_eq!(audit_draw_indices(11, 1).len(), 2);
        assert!(audit_draw_indices(0, 1).is_empty());
    }

    /// One model dir written the way a run writes it.
    fn write_model_dir(
        root: &Path,
        stem: &str,
        rows: &[(&str, Render, u32, Option<String>)],
    ) -> Vec<Graded> {
        let dir = root.join(stem);
        assert!(std::fs::create_dir_all(dir.join("texts")).is_ok());
        let metrics = Metrics {
            elapsed_ms: 3000,
            peak_rss_kib: Some(2_000_000),
            peak_vram_mib: Some(2500),
        };
        let mut json_rows = Vec::new();
        let mut labels = Vec::new();
        for (id, render, run, stdout) in rows {
            let s = shape_of(id);
            let Ok(c) = compose(&s, *render) else {
                panic!("{id} composes");
            };
            assert!(
                std::fs::write(text_path(&dir, id, *render, *run, "digest"), &c.payload).is_ok()
            );
            let g = match stdout {
                Some(text) => {
                    assert!(
                        std::fs::write(text_path(&dir, id, *render, *run, "stdout"), text).is_ok()
                    );
                    grade(&s, Outcome::Stdout(text))
                }
                None => grade(&s, Outcome::Failed("timeout")),
            };
            json_rows.push(row_json("gb", &s, *render, *run, &g, "absent", metrics));
            labels.push(g);
        }
        let text = serde_json::to_string_pretty(&json_rows).unwrap_or_default();
        assert!(std::fs::write(dir.join("runs.json"), text).is_ok());
        labels
    }

    fn sample_rows() -> Vec<(&'static str, Render, u32, Option<String>)> {
        vec![
            (
                "A1",
                Render::Today,
                1,
                Some(model_json("surface", "the cart-service deploy")),
            ),
            (
                "A7",
                Render::Today,
                1,
                Some(model_json("surface", "checkout-api errors")),
            ),
            (
                "B1",
                Render::Today,
                1,
                Some(model_json("watch", "all fine")),
            ),
            (
                "C1",
                Render::Enriched,
                1,
                Some("Loading model...\n".to_string()),
            ),
            ("C3", Render::Today, 2, None),
        ]
    }

    #[test]
    fn regrade_of_stored_outputs_reproduces_the_generation_labels() {
        let root = temp_root();
        let labels = write_model_dir(root.path(), "Model-A", &sample_rows());
        let Ok(rows) = regrade_model_dir(&root.path().join("Model-A")) else {
            panic!("the model dir re-grades");
        };
        let regraded: Vec<Graded> = rows.iter().map(|r| r.graded.clone()).collect();
        assert_eq!(regraded, labels);
        assert!(rows.iter().all(|r| r.graded == r.stored));
        assert_eq!(rows[3].graded.detect, "no_reading");
    }

    #[test]
    fn audit_sample_hides_the_model_and_its_key_names_it() {
        let root = temp_root();
        let mut rows = sample_rows();
        rows.extend(
            sample_rows()
                .into_iter()
                .map(|(s, r, n, o)| (s, r, n + 10, o)),
        );
        write_model_dir(root.path(), "Llama-3.2-3B-Instruct-Q4_K_M", &rows);
        write_model_dir(root.path(), "Qwen3.5-2B-Q4_K_M", &rows);
        let Ok((k, n)) = audit_draw(root.path(), 42) else {
            panic!("the audit draws");
        };
        assert_eq!((k, n), (2, 20));
        let sample = std::fs::read_to_string(root.path().join(AUDIT_SAMPLE)).unwrap_or_default();
        for stem in ["Llama", "Qwen", "Q4_K_M"] {
            assert!(!sample.contains(stem), "the sample names {stem}");
        }
        assert!(sample.contains("## s001") && sample.contains("## s002"));
        let key = std::fs::read_to_string(root.path().join(AUDIT_KEY)).unwrap_or_default();
        assert!(key.contains("Q4_K_M"), "the key maps ids to models");
    }

    #[test]
    fn audit_grade_counts_disagreement_against_the_ten_percent_bar() {
        let ids: Vec<String> = (1..=20).map(|i| format!("s{i:03}")).collect();
        let mut verdicts: String = ids.iter().map(|i| format!("{i} agree\n")).collect();
        assert_eq!(grade_audit(&ids, &verdicts), Ok((0, 20)));
        verdicts = verdicts.replacen("s001 agree", "s001 disagree", 1);
        verdicts = verdicts.replacen("s002 agree", "s002 disagree", 1);
        assert_eq!(grade_audit(&ids, &verdicts), Ok((2, 20)));
        assert_eq!(audit_disposition(2, 20), "agree");
        assert_eq!(audit_disposition(3, 20), "regrade");
        assert!(
            grade_audit(&ids, "s001 agree\n").is_err(),
            "a missing verdict"
        );
        assert!(
            grade_audit(&ids, &format!("{verdicts}s001 agree\n")).is_err(),
            "a duplicate"
        );
        assert!(
            grade_audit(&ids, &format!("{verdicts}s999 agree\n")).is_err(),
            "an undrawn id"
        );
        assert!(grade_audit(&ids, &format!("{verdicts}s003 maybe\n")).is_err());
        let annotated = format!(
            "# reader notes\n{}",
            verdicts.replace(" agree\n", " agree   # matches the ground truth\n")
        );
        assert_eq!(
            grade_audit(&ids, &annotated),
            Ok((2, 20)),
            "a trailing comment is ignored"
        );
    }

    fn summary(
        stem: &str,
        a_cause: (u32, u32),
        b_false: (u32, u32),
        vram: u64,
        rss: u64,
    ) -> ModelSummary {
        ModelSummary {
            stem: stem.to_string(),
            a_cause: Cell {
                hits: a_cause.0,
                readable: a_cause.1,
                unreadable: BTreeMap::new(),
            },
            b_false_alarm: Cell {
                hits: b_false.0,
                readable: b_false.1,
                unreadable: BTreeMap::new(),
            },
            peak_vram_mib: Some(vram),
            peak_rss_kib: Some(rss),
            ..ModelSummary::default()
        }
    }

    #[test]
    fn recommend_picks_the_lightest_qualifying_model_by_vram() {
        let models = [
            // The lowest RSS among the qualifying pair: an RSS order would pick it.
            summary("heavy-best", (60, 70), (0, 30), 3900, 1_200_000),
            summary("light-close", (55, 70), (2, 30), 2100, 3_400_000),
            summary("lighter-weak", (30, 70), (0, 30), 2000, 1_600_000),
            summary("noisy", (60, 70), (12, 30), 1800, 1_500_000),
        ];
        let rec = recommend(&models);
        assert_eq!(rec.pick.as_deref(), Some("light-close"));
        assert_eq!(rec.qualifying, ["light-close", "heavy-best"]);
    }

    #[test]
    fn recommend_breaks_a_vram_tie_by_rss() {
        let models = [
            summary("more-rss", (60, 70), (0, 30), 2300, 3_400_000),
            summary("less-rss", (60, 70), (0, 30), 2300, 1_700_000),
        ];
        assert_eq!(recommend(&models).pick.as_deref(), Some("less-rss"));
    }

    #[test]
    fn recommend_reads_none_when_nothing_qualifies_or_cells_are_empty() {
        let cannot = ModelSummary {
            stem: "empty".to_string(),
            ..ModelSummary::default()
        };
        let rec = recommend(std::slice::from_ref(&cannot));
        assert_eq!(rec.pick, None);
        assert!(rec.best_a_cause.is_none());
        let mut unread = Cell::default();
        unread.add("parse_failed", false);
        unread.add("parse_failed", false);
        assert_eq!(
            unread.text(),
            "cannot-evaluate (0 readable: parse_failed 2)"
        );
        assert_eq!(Cell::default().text(), "cannot-evaluate (no rows)");
        assert!(table_line(&cannot).contains("A cause cannot-evaluate"));
    }

    #[test]
    fn table_grades_a_series_root_and_prints_one_recommendation() {
        let root = temp_root();
        write_model_dir(root.path(), "Model-A", &sample_rows());
        write_model_dir(root.path(), "Model-B", &sample_rows());
        let Ok(lines) = table(root.path()) else {
            panic!("the table grades");
        };
        assert!(lines.iter().any(|l| l.starts_with("table: Model-A")));
        assert!(lines.iter().any(|l| l.starts_with("table: Model-B")));
        assert_eq!(
            lines
                .iter()
                .filter(|l| l.starts_with("recommendation: "))
                .count(),
            1
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "table: regrade changed 0 of 10 row labels")
        );
    }

    #[test]
    fn renders_parse_in_render_order_and_refuse_an_unknown_one() {
        assert_eq!(
            parse_renders("enriched,today"),
            Ok(vec![Render::Today, Render::Enriched])
        );
        assert_eq!(parse_renders("today"), Ok(vec![Render::Today]));
        assert_eq!(
            parse_renders("today,later").err().as_deref(),
            Some("unknown render later")
        );
        let shapes = select_pattern_shapes(&["C1".to_string(), "A1".to_string()]);
        let pairs = shape_renders(&shapes, &[Render::Today, Render::Enriched]);
        assert_eq!(pairs.len(), 3, "A1 today, C1 today and enriched");
    }
}
