//! Chunk #94 — P-038 content-identity for the MCP incident/report tools.
//!
//! The MCP `retrieve_report(id)` tool and the in-app `incidents.get_report`
//! resolver MUST produce byte-identical markdown for the same incident
//! (capability P-038 single-source-of-truth). Both go through
//! `interpretation::markdown::{assemble_report, serialize_report}`; this test
//! drives the MCP tool dispatch against a corpus-seeded incident and asserts
//! the tool's markdown equals the markdown the resolver path produces from
//! the same Incident + parsed L4Output.
//!
//! Deterministic + in-process: uses an in-memory corpus with a fake keychain
//! (no OS keychain, no subprocess). The cross-process subprocess framing is
//! covered separately by `crates/mcp-server/tests/sidecar_subprocess.rs`.

#![cfg(feature = "mcp-server")]

use std::sync::Arc;

use corpus::contract::{Corpus, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use interpretation::markdown::{assemble_report, serialize_report};
use interpretation::schema::L4Output;
use mcp_server_crate::tools::{IncidentToolContext, dispatch_tool};
use serde_json::json;
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, Incident, IncidentStatus, PriorityTier, Severity,
};
use viz::state::VizState;

fn resolved_incident_with_l4() -> Incident {
    // A Resolved incident whose resolution_summary_text is a JSON-encoded
    // L4Output — the full six-section (non-degraded) render path.
    let l4 = json!({
        "schema_version": "2.0",
        "prompt_version": "v2.1",
        "decision": "surface",
        "severity": "suggested",
        "title": "Connection pool saturation in service-a",
        "symptom": "Pool exhaustion under sustained 40 req/sec load",
        "timeline": "Saturation began 12:34Z, recovered 12:40Z",
        "hypotheses": [
            {
                "statement": "Pool size too small for load",
                "confidence": "high",
                "justification": "Pool 10 vs observed 40 req/sec"
            }
        ],
        "investigation_steps": [
            {
                "step": "Inspect pool config",
                "expected_yield": "Confirm max_connections"
            }
        ],
        "evidence_refs": ["span:0102030405060708"],
        "fingerprint": "fp-p038",
        "model_tier": "primary",
        "hardware_profile": "gpu_primary",
        "is_resolution_summary": true
    })
    .to_string();

    Incident {
        id: 0,
        workspace: "ws-p038".into(),
        fingerprint: "fp-p038".into(),
        title: "Pool saturation".into(),
        detail: "Connection pool exhausted".into(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: Some("service-a".into()),
        status: IncidentStatus::Resolved,
        severity: Severity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![[1, 2, 3, 4, 5, 6, 7, 8]],
            fingerprint_hashes: vec!["fp-p038".into()],
            timestamps_unix_nano: vec![1_700_000_000_000],
        },
        opened_at_unix_nano: 1_700_000_000_000,
        updated_at_unix_nano: 1_700_000_000_500,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: Some(1_700_000_000_900),
        read_at_unix_nano: None,
        resolution_summary_text: Some(l4),
    }
}

fn seed_corpus(incident: &Incident) -> (IncidentToolContext, i64) {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    let corpus: Arc<dyn CorpusWriter> = Arc::new(corpus);
    let payload = bincode::serialize(incident).expect("encode incident");
    let id = corpus
        .save_incident(
            &incident.workspace,
            "resolved",
            incident.opened_at_unix_nano,
            incident.updated_at_unix_nano,
            incident.resolved_at_unix_nano,
            None,
            &payload,
        )
        .expect("save incident");
    (
        IncidentToolContext {
            corpus,
            workspace_root: incident.workspace.clone(),
        },
        id,
    )
}

// The resolver path (incidents.get_report) projects the same Incident +
// parsed L4Output through assemble_report + serialize_report. Reproduce that
// projection here and assert byte-identity with the MCP tool output.
fn resolver_markdown(incident: &Incident) -> String {
    let parsed_l4: Option<L4Output> = incident
        .resolution_summary_text
        .as_deref()
        .and_then(|text| serde_json::from_str::<L4Output>(text).ok());
    let report = assemble_report(incident, parsed_l4.as_ref());
    serialize_report(&report)
}

#[test]
fn mcp_retrieve_report_markdown_is_byte_identical_to_resolver() {
    let incident = resolved_incident_with_l4();
    let expected = resolver_markdown(&incident);

    let (ctx, id) = seed_corpus(&incident);
    let conn = fresh_buffer();
    let state = VizState::new();
    let value = dispatch_tool(
        &conn,
        &state,
        Some(&ctx),
        "retrieve_report",
        &json!({ "incident_id": id }),
    )
    .expect("dispatch ok");
    let mcp_markdown = value["markdown"].as_str().expect("markdown string");

    assert_eq!(
        mcp_markdown, expected,
        "MCP retrieve_report markdown must be byte-identical to the resolver projection (P-038)"
    );
    // Resolved-with-L4 incident renders the full (non-degraded) report.
    assert_eq!(value["degraded_mode"].as_bool(), Some(false));
    assert!(mcp_markdown.contains("## Hypotheses"));
    assert!(mcp_markdown.contains("Pool size too small for load"));
}

#[test]
fn mcp_query_incident_list_returns_seeded_incident() {
    let incident = resolved_incident_with_l4();
    let mut active = incident.clone();
    active.status = IncidentStatus::Active;
    active.resolution_summary_text = None;
    active.resolved_at_unix_nano = None;

    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("corpus");
    let corpus: Arc<dyn CorpusWriter> = Arc::new(corpus);
    let payload = bincode::serialize(&active).expect("encode");
    corpus
        .save_incident(
            "ws-p038",
            "active",
            active.opened_at_unix_nano,
            active.updated_at_unix_nano,
            None,
            None,
            &payload,
        )
        .expect("save");
    let ctx = IncidentToolContext {
        corpus,
        workspace_root: "ws-p038".into(),
    };

    let conn = fresh_buffer();
    let state = VizState::new();
    let value = dispatch_tool(&conn, &state, Some(&ctx), "query_incident_list", &json!({}))
        .expect("dispatch ok");
    assert_eq!(value["total"].as_u64(), Some(1));
    let items = value["items"].as_array().expect("items");
    assert_eq!(items[0]["severity"].as_str(), Some("warn"));
}

fn fresh_buffer() -> Arc<std::sync::Mutex<duckdb::Connection>> {
    let conn = duckdb::Connection::open_in_memory().expect("duckdb");
    buffer::schema::create_schema(&conn).expect("schema");
    Arc::new(std::sync::Mutex::new(conn))
}
