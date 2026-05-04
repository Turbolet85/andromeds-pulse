use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;
use tonic::Code;

async fn start_test_server() -> (SocketAddr, Arc<IngestState>, tokio::task::JoinHandle<()>) {
    let state = Arc::new(IngestState::new());
    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port must succeed in test");
    let bound = listener.local_addr().expect("listener has local_addr");
    let state_clone = Arc::clone(&state);
    let handle = tokio::spawn(async move {
        let _ = ingest::grpc::serve_on(listener, state_clone).await;
    });
    // Best-effort wait for the server's accept loop to be ready before clients connect.
    tokio::time::sleep(Duration::from_millis(50)).await;
    (bound, state, handle)
}

fn make_span(trace_id: [u8; 16], span_id: [u8; 8]) -> Span {
    Span {
        trace_id: trace_id.to_vec(),
        span_id: span_id.to_vec(),
        name: "test-span".to_string(),
        kind: 0,
        start_time_unix_nano: 0,
        end_time_unix_nano: 0,
        ..Default::default()
    }
}

fn make_export_request(span_count: usize) -> ExportTraceServiceRequest {
    let spans: Vec<Span> = (0..span_count)
        .map(|i| make_span([1u8; 16], [(i % 250) as u8 + 1; 8]))
        .collect();
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn p1_grpc_export_response_ok_and_state_records_spans() {
    let (addr, state, handle) = start_test_server().await;
    let endpoint = format!("http://{}", addr);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client must connect to loopback receiver");
    let resp = client
        .export(make_export_request(100))
        .await
        .expect("100-span export must respond OK");
    assert!(resp.into_inner().partial_success.is_none());
    assert_eq!(state.snapshot().span_count, 100);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn export_payload_over_8mb_is_rejected() {
    let (addr, _state, handle) = start_test_server().await;
    let endpoint = format!("http://{}", addr);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client must connect to loopback receiver");
    // Each span carries a 1KB ballast in `name` to inflate payload size cheaply.
    let mut spans = Vec::new();
    let ballast: String = "x".repeat(1024);
    // 9 MB / 1 KB ≈ 9_216 spans, plus header overhead pushes the encoded payload
    // comfortably over the 8 MB cap.
    for i in 0..9_500 {
        let mut s = make_span([1u8; 16], [(i % 250) as u8 + 1; 8]);
        s.name = ballast.clone();
        spans.push(s);
    }
    let req = ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    };
    let result = client.export(req).await;
    let status = result.expect_err("oversized payload must be rejected without panic");
    // tonic 0.14 returns OutOfRange for messages exceeding max_decoding_message_size.
    assert!(
        matches!(
            status.code(),
            Code::OutOfRange | Code::ResourceExhausted | Code::InvalidArgument | Code::Internal
        ),
        "expected size-rejection error code, got {:?}: {}",
        status.code(),
        status.message(),
    );
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn try_bind_rejects_collision_with_bind_failed() {
    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("first bind must succeed");
    let bound = listener.local_addr().unwrap();
    let collision = ingest::grpc::try_bind(bound).await;
    assert!(
        matches!(collision, Err(ingest::contract::Error::BindFailed { .. })),
        "second bind on same port must return BindFailed; got {:?}",
        collision,
    );
}

#[test]
fn no_non_loopback_bind_literals_in_ingest_src() {
    // Security gate: the receiver must NEVER bind to 0.0.0.0 / [::] /
    // Ipv4Addr::UNSPECIFIED — loopback is the de-facto authorization boundary
    // (per security plan §Security Anti-Patterns § API + CLAUDE.md universal
    // invariant). This test is a grep gate: scan crates/ingest/src/ for the
    // forbidden patterns and assert zero matches.
    use std::fs;
    use std::path::Path;

    fn walk(dir: &Path, hits: &mut Vec<String>) {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, hits);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(contents) = fs::read_to_string(&path) {
                    for (lineno, line) in contents.lines().enumerate() {
                        // skip line if it's commented (rough heuristic — strip everything after //)
                        let code = line.split("//").next().unwrap_or("");
                        for needle in [
                            "0.0.0.0",
                            "Ipv4Addr::UNSPECIFIED",
                            "Ipv6Addr::UNSPECIFIED",
                            "[::]",
                            "([0, 0, 0, 0]",
                        ] {
                            if code.contains(needle) {
                                hits.push(format!(
                                    "{}:{}: {}",
                                    path.display(),
                                    lineno + 1,
                                    line.trim()
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut hits = Vec::new();
    walk(&src_root, &mut hits);
    assert!(
        hits.is_empty(),
        "non-loopback bind literals found in crates/ingest/src/:\n{}",
        hits.join("\n")
    );
}
