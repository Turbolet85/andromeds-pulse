//! Chunk #99 four-profile load suite per dist-arch v3 §Load testing
//! (`docs/v0_2_0/pulse-distillation-architecture.md:716-734`). Release-gate
//! evidence for the v0.2.0 tag — these tests are EXCLUDED from the default
//! and ci nextest profiles via `.config/nextest.toml` `default-filter`
//! (full-duration runs reach ~12 minutes total) and run via
//! `cargo xtask perf:load-profiles` (nextest `--profile load-profiles`).
//!
//! Mirrors the chunk #54 `perf_slo_10k_spans.rs` in-process bootstrap
//! pattern (ephemeral loopback gRPC receiver, real DuckDB, `run_consumer`,
//! and paced tonic clients). Spans carry REAL `SystemTime::now()`
//! timestamps so the L1a window queries and the retention sweep see them
//! (the fixed-epoch timestamps of the chunk #54 generator would age out
//! instantly).
//!
//! Per-profile pass criteria (dist-arch v3, verbatim targets):
//! baseline 1000 spans/s x 60s with zero drops; high 10000 spans/s x 5min
//! with zero drops and L1a Q1-Q6 p99 under 500ms measured DURING load
//! (Q7 runs through its bounded-timeout path and reports timeout counts —
//! Q7 timeouts are an expected operational mode per P-058); burst
//! 50000 spans/s x 30s absorbed without crash and recovery to clean
//! baseline ingestion within 60s (boundary rejections under backpressure
//! are in-contract observable behavior, never weakened limits); sustained
//! extreme 50000 spans/s x 5min degraded-but-functional with retention
//! eviction active and zero L0 pipeline loss (every accepted span lands;
//! only the retention sweep removes rows).
//!
//! Offered-vs-achieved discipline (zero-flake budget): wall-clock
//! throughput floors are asserted only where the chunk #54 substrate proved
//! them feasible (baseline + high). Burst + sustained assert FUNCTIONAL
//! invariants and REPORT the achieved offered rate — generator capability
//! is host-dependent and a hard floor there would be flaky by construction.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer};
use duckdb::Connection;
use ingest::channel::build_channel;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;
use triage::contract::{TriageSqlState, run_q1, run_q2, run_q3, run_q4, run_q5, run_q6, run_q7};

const BATCH_SIZE: usize = 1_000;

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

struct LoadRig {
    conn: Arc<Mutex<Connection>>,
    buffer_state: Arc<BufferState>,
    addr: SocketAddr,
    sender: Arc<ingest::channel::IngestSender>,
    consumer_handle: tokio::task::JoinHandle<()>,
    serve_handle: tokio::task::JoinHandle<()>,
}

async fn boot_rig() -> LoadRig {
    let conn = {
        let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
        create_schema(&raw).expect("buffer schema creates");
        Arc::new(Mutex::new(raw))
    };
    let buffer_state = Arc::new(BufferState::new());
    let ingest_state = Arc::new(IngestState::new());
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());

    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);

    let consumer_handle = tokio::spawn(run_consumer(
        receiver,
        Arc::clone(&conn),
        Arc::clone(&buffer_state),
        Arc::clone(&broadcast_senders),
        Arc::new(ingest::observer::NoopSpanObserver),
        None,
        None,
    ));

    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port succeeds");
    let addr = listener.local_addr().expect("listener exposes local_addr");
    assert!(
        addr.ip().is_loopback(),
        "load-profile receiver must bind only to loopback; got {}",
        addr.ip()
    );

    let serve_handle = {
        let state = Arc::clone(&ingest_state);
        let sender = Arc::clone(&sender);
        tokio::spawn(async move {
            let _ = ingest::grpc::serve_on(listener, state, sender).await;
        })
    };
    tokio::time::sleep(Duration::from_millis(100)).await;

    LoadRig {
        conn,
        buffer_state,
        addr,
        sender,
        consumer_handle,
        serve_handle,
    }
}

async fn shutdown_rig(rig: LoadRig) {
    rig.serve_handle.abort();
    drop(rig.sender);
    let _ = tokio::time::timeout(Duration::from_secs(5), rig.consumer_handle).await;
}

fn make_batch(
    span_counter: &AtomicU64,
    span_count: usize,
    service: &str,
) -> ExportTraceServiceRequest {
    let base = span_counter.fetch_add(span_count as u64, Ordering::Relaxed);
    let start_ns = now_ns();
    let mut trace_id = [1u8; 16];
    trace_id[..8].copy_from_slice(&base.to_le_bytes());
    let spans: Vec<Span> = (0..span_count)
        .map(|i| {
            let id = base + i as u64 + 1;
            Span {
                trace_id: trace_id.to_vec(),
                span_id: id.to_le_bytes().to_vec(),
                name: "load-profile-span".to_string(),
                kind: 0,
                start_time_unix_nano: start_ns + i as u64,
                end_time_unix_nano: start_ns + i as u64 + 1_000_000,
                ..Default::default()
            }
        })
        .collect();
    let resource = Resource {
        attributes: vec![KeyValue {
            key: "service.name".into(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(service.into())),
            }),
        }],
        dropped_attributes_count: 0,
    };
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(resource),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

struct InjectionOutcome {
    accepted_spans: u64,
    rejected_batches: u64,
    elapsed: Duration,
}

/// Paced multi-client injection: `clients` parallel tonic connections, each
/// sending `BATCH_SIZE`-span exports on a fixed interval so the aggregate
/// offered rate is `rate_spans_per_sec`, until `duration` elapses.
async fn run_paced_injection(
    addr: SocketAddr,
    clients: usize,
    rate_spans_per_sec: u64,
    duration: Duration,
    span_counter: Arc<AtomicU64>,
    service: &str,
) -> InjectionOutcome {
    let interval_ms = (1_000u64 * BATCH_SIZE as u64 * clients as u64) / rate_spans_per_sec;
    let accepted = Arc::new(AtomicU64::new(0));
    let rejected = Arc::new(AtomicU64::new(0));
    let started = Instant::now();
    let deadline = tokio::time::Instant::now() + duration;

    let mut handles = Vec::with_capacity(clients);
    for _ in 0..clients {
        let endpoint = format!("http://{addr}");
        let counter = Arc::clone(&span_counter);
        let accepted = Arc::clone(&accepted);
        let rejected = Arc::clone(&rejected);
        let service = service.to_string();
        handles.push(tokio::spawn(async move {
            let mut client = TraceServiceClient::connect(endpoint)
                .await
                .expect("client connects to loopback receiver");
            let mut tick = tokio::time::interval(Duration::from_millis(interval_ms.max(1)));
            loop {
                tick.tick().await;
                if tokio::time::Instant::now() >= deadline {
                    break;
                }
                let req = make_batch(&counter, BATCH_SIZE, &service);
                match client.export(req).await {
                    Ok(_) => {
                        accepted.fetch_add(BATCH_SIZE as u64, Ordering::Relaxed);
                    }
                    Err(_) => {
                        rejected.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }));
    }
    for h in handles {
        let _ = h.await;
    }
    InjectionOutcome {
        accepted_spans: accepted.load(Ordering::Relaxed),
        rejected_batches: rejected.load(Ordering::Relaxed),
        elapsed: started.elapsed(),
    }
}

async fn wait_for_rows(state: &Arc<BufferState>, target: u64, timeout: Duration) -> bool {
    let deadline = tokio::time::Instant::now() + timeout;
    while tokio::time::Instant::now() < deadline {
        if state.snapshot().rows_ingested >= target {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    false
}

fn p99_ms(samples: &mut [u128]) -> u128 {
    samples.sort_unstable();
    if samples.is_empty() {
        return 0;
    }
    let idx = ((samples.len() * 99).div_ceil(100)).min(samples.len()) - 1;
    samples[idx]
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn load_profile_baseline_1k_60s_zero_drops() {
    let rig = boot_rig().await;
    let counter = Arc::new(AtomicU64::new(0));

    let outcome = run_paced_injection(
        rig.addr,
        1,
        1_000,
        Duration::from_secs(60),
        Arc::clone(&counter),
        "load-baseline",
    )
    .await;

    assert_eq!(
        outcome.rejected_batches, 0,
        "baseline profile must accept every export (zero drops)"
    );
    assert!(
        outcome.accepted_spans >= 50_000,
        "baseline profile must offer ~1k spans/s for 60s; offered only {} spans in {:?}",
        outcome.accepted_spans,
        outcome.elapsed
    );
    let drained = wait_for_rows(
        &rig.buffer_state,
        outcome.accepted_spans,
        Duration::from_secs(30),
    )
    .await;
    let snapshot = rig.buffer_state.snapshot();
    assert!(
        drained,
        "baseline profile pipeline must land every accepted span; accepted={} rows_ingested={}",
        outcome.accepted_spans, snapshot.rows_ingested
    );
    println!(
        "load-profile baseline PASS: offered={} spans in {:?} (~{}/s), rows_ingested={}",
        outcome.accepted_spans,
        outcome.elapsed,
        outcome.accepted_spans / outcome.elapsed.as_secs().max(1),
        snapshot.rows_ingested
    );
    shutdown_rig(rig).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn load_profile_high_10k_5min_l1a_p99_under_500ms() {
    let rig = boot_rig().await;
    let counter = Arc::new(AtomicU64::new(0));
    let sql_state = TriageSqlState::new(Arc::clone(&rig.conn));

    // L1a sampler at the PRODUCTION cadence floor (20s = the chunk #80
    // cadence coordinator's accelerated tier): each tick runs Q1-Q6
    // (latency recorded per query family, asserted <500ms p99) + Q7
    // through its bounded-timeout-with-fallback path (latency reported
    // only — Q7 worst-case bounds + timeout fallback are a documented
    // operational mode per P-058). An earlier 5s sampler was 4x harsher
    // than any production mode and let one tick's Q1 overlap the prior
    // tick's Q7 interrupt window. Spike forensics: any single query over
    // 300ms logs its sample index so tail samples stay attributable.
    let sampler_stop = Arc::new(AtomicU64::new(0));
    let sampler = {
        let stop = Arc::clone(&sampler_stop);
        tokio::spawn(async move {
            let mut q_latencies: [Vec<u128>; 6] = Default::default();
            let mut q7_latencies: Vec<u128> = Vec::new();
            let window = Duration::from_secs(60);
            let mut tick = tokio::time::interval(Duration::from_secs(20));
            tick.tick().await;
            let mut sample_idx = 0u32;
            while stop.load(Ordering::Relaxed) == 0 {
                tick.tick().await;
                sample_idx += 1;
                let spike = |q: usize, ms: u128| {
                    if ms > 300 {
                        println!("load-profile high spike: sample={sample_idx} Q{q} took {ms}ms");
                    }
                };
                let t = Instant::now();
                let _ = run_q1(&sql_state, window).await;
                let ms = t.elapsed().as_millis();
                spike(1, ms);
                q_latencies[0].push(ms);
                let t = Instant::now();
                let _ = run_q2(&sql_state, window).await;
                let ms = t.elapsed().as_millis();
                spike(2, ms);
                q_latencies[1].push(ms);
                let t = Instant::now();
                let _ = run_q3(&sql_state, window).await;
                let ms = t.elapsed().as_millis();
                spike(3, ms);
                q_latencies[2].push(ms);
                let t = Instant::now();
                let _ = run_q4(&sql_state, window).await;
                let ms = t.elapsed().as_millis();
                spike(4, ms);
                q_latencies[3].push(ms);
                let t = Instant::now();
                let _ = run_q5(&sql_state, window).await;
                let ms = t.elapsed().as_millis();
                spike(5, ms);
                q_latencies[4].push(ms);
                let t = Instant::now();
                let _ = run_q6(&sql_state, window).await;
                let ms = t.elapsed().as_millis();
                spike(6, ms);
                q_latencies[5].push(ms);
                let t = Instant::now();
                let _ = run_q7(&sql_state, window).await;
                let ms = t.elapsed().as_millis();
                spike(7, ms);
                q7_latencies.push(ms);
            }
            (q_latencies, q7_latencies)
        })
    };

    let outcome = run_paced_injection(
        rig.addr,
        4,
        10_000,
        Duration::from_secs(300),
        Arc::clone(&counter),
        "load-high",
    )
    .await;
    sampler_stop.store(1, Ordering::Relaxed);
    let (mut q_latencies, mut q7_latencies) =
        tokio::time::timeout(Duration::from_secs(30), sampler)
            .await
            .expect("sampler joins after stop flag")
            .expect("sampler task completes");

    assert_eq!(
        outcome.rejected_batches, 0,
        "high profile must accept every export (no drops at 10k spans/s)"
    );
    assert!(
        outcome.accepted_spans >= 2_500_000,
        "high profile must offer ~10k spans/s for 5min; offered only {} spans in {:?}",
        outcome.accepted_spans,
        outcome.elapsed
    );
    let drained = wait_for_rows(
        &rig.buffer_state,
        outcome.accepted_spans,
        Duration::from_secs(60),
    )
    .await;
    let snapshot = rig.buffer_state.snapshot();
    assert!(
        drained,
        "high profile pipeline must land every accepted span; accepted={} rows_ingested={}",
        outcome.accepted_spans, snapshot.rows_ingested
    );

    for (i, samples) in q_latencies.iter_mut().enumerate() {
        let p99 = p99_ms(samples);
        let p50 = samples[(samples.len().saturating_sub(1)) / 2];
        let p95 = samples[((samples.len() * 95).div_ceil(100))
            .min(samples.len())
            .saturating_sub(1)];
        println!(
            "load-profile high L1a Q{} p50={}ms p95={}ms p99={}ms over {} samples",
            i + 1,
            p50,
            p95,
            p99,
            samples.len()
        );
        assert!(
            p99 < 500,
            "L1a Q{} p99 under high load must stay <500ms (dist-arch v3 high-load criterion); got {}ms",
            i + 1,
            p99
        );
    }
    let q7_p99 = p99_ms(&mut q7_latencies);
    println!(
        "load-profile high PASS: offered={} spans in {:?} (~{}/s); Q7 p99={}ms over {} samples (bounded-timeout-with-fallback path, reported not asserted)",
        outcome.accepted_spans,
        outcome.elapsed,
        outcome.accepted_spans / outcome.elapsed.as_secs().max(1),
        q7_p99,
        q7_latencies.len()
    );
    shutdown_rig(rig).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn load_profile_burst_50k_30s_recovers_within_60s() {
    let rig = boot_rig().await;
    let counter = Arc::new(AtomicU64::new(0));

    let outcome = run_paced_injection(
        rig.addr,
        8,
        50_000,
        Duration::from_secs(30),
        Arc::clone(&counter),
        "load-burst",
    )
    .await;
    println!(
        "load-profile burst: offered={} spans in {:?} (~{}/s), rejected_batches={} (boundary rejections under backpressure are in-contract)",
        outcome.accepted_spans,
        outcome.elapsed,
        outcome.accepted_spans / outcome.elapsed.as_secs().max(1),
        outcome.rejected_batches
    );

    // Recovery criterion: within 60s of burst end the pipeline drains every
    // accepted span AND a clean baseline injection is fully accepted again.
    let drained = wait_for_rows(
        &rig.buffer_state,
        outcome.accepted_spans,
        Duration::from_secs(60),
    )
    .await;
    let snapshot = rig.buffer_state.snapshot();
    assert!(
        drained,
        "burst profile must recover within 60s (drain accepted spans); accepted={} rows_ingested={}",
        outcome.accepted_spans, snapshot.rows_ingested
    );

    let recovery = run_paced_injection(
        rig.addr,
        1,
        1_000,
        Duration::from_secs(5),
        Arc::clone(&counter),
        "load-burst-recovery",
    )
    .await;
    assert_eq!(
        recovery.rejected_batches, 0,
        "post-burst baseline injection must be fully accepted (recovered to normal operation)"
    );
    let recovered = wait_for_rows(
        &rig.buffer_state,
        outcome.accepted_spans + recovery.accepted_spans,
        Duration::from_secs(15),
    )
    .await;
    assert!(recovered, "post-burst baseline spans must drain promptly");
    println!(
        "load-profile burst PASS: recovery injection accepted={} spans",
        recovery.accepted_spans
    );
    shutdown_rig(rig).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn load_profile_sustained_extreme_50k_5min_no_l0_loss_with_retention() {
    let rig = boot_rig().await;
    let counter = Arc::new(AtomicU64::new(0));

    // Short retention (60s -> 10s sweep cadence) so eviction is exercised
    // within the 5-minute window; spans carry real now() timestamps so only
    // aged rows are swept.
    let retention_handle = tokio::spawn(buffer::retention::run_retention(
        Arc::clone(&rig.conn),
        Arc::clone(&rig.buffer_state),
        60,
    ));

    // Forensics monitor (chunk #99 stall diagnosis): every 30s print append
    // + eviction progress and whether the shared connection mutex is held —
    // attributes a stall to the sweep (lock held, evictions frozen) vs the
    // appender (lock held, rows frozen) vs neither (consumer logic).
    let monitor_stop = Arc::new(AtomicU64::new(0));
    let monitor = {
        let stop = Arc::clone(&monitor_stop);
        let state = Arc::clone(&rig.buffer_state);
        let conn = Arc::clone(&rig.conn);
        tokio::spawn(async move {
            let started = Instant::now();
            while stop.load(Ordering::Relaxed) == 0 {
                tokio::time::sleep(Duration::from_secs(30)).await;
                let snap = state.snapshot();
                let lock_state = match conn.try_lock() {
                    Ok(_g) => "free",
                    Err(std::sync::TryLockError::WouldBlock) => "HELD",
                    Err(_) => "poisoned",
                };
                println!(
                    "load-profile sustained monitor t+{:?}: rows_ingested={} eviction_count={} conn_mutex={}",
                    started.elapsed(),
                    snap.rows_ingested,
                    snap.eviction_count,
                    lock_state
                );
            }
        })
    };

    let outcome = run_paced_injection(
        rig.addr,
        8,
        50_000,
        Duration::from_secs(300),
        Arc::clone(&counter),
        "load-sustained",
    )
    .await;
    println!(
        "load-profile sustained: offered={} spans in {:?} (~{}/s), rejected_batches={} (degraded-but-functional; LWW-style boundary rejections acceptable)",
        outcome.accepted_spans,
        outcome.elapsed,
        outcome.accepted_spans / outcome.elapsed.as_secs().max(1),
        outcome.rejected_batches
    );

    // Zero L0 pipeline loss: every ACCEPTED span lands in DuckDB (only the
    // retention sweep removes rows afterwards; rows_ingested is cumulative
    // appended and never decremented by eviction). PROGRESS-BASED drain:
    // the backlog after a 50k/s offered window is up to the full 1024-batch
    // channel (~1M spans). The spec's "degraded-but-functional + zero L0
    // loss" admits TRANSIENT consumer pauses — at this volume DuckDB's
    // append path performs multi-row-group maintenance that can block a
    // single append inside spawn_blocking for >60s (observed up to ~2min on
    // a 12.7M-row in-memory table) before resuming and landing everything.
    // So there is deliberately NO eager no-progress panic here: the only
    // failure is not reaching accepted==rows_ingested within the hard cap.
    // The 5s drain curve stays as the diagnostic for cap-failures.
    let drain_started = Instant::now();
    let mut last_rows = rig.buffer_state.snapshot().rows_ingested;
    let drained = loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let rows = rig.buffer_state.snapshot().rows_ingested;
        println!(
            "load-profile sustained drain: rows_ingested={} / accepted={} (+{} in 5s; t+{:?})",
            rows,
            outcome.accepted_spans,
            rows.saturating_sub(last_rows),
            drain_started.elapsed()
        );
        last_rows = rows;
        if rows >= outcome.accepted_spans {
            break true;
        }
        if drain_started.elapsed() > Duration::from_secs(420) {
            break false; // hard cap: even a maintenance-stalled consumer must recover + drain within 7 minutes
        }
    };
    let snapshot = rig.buffer_state.snapshot();
    assert!(
        drained,
        "sustained-extreme: every accepted span must land in L0 (no pipeline loss / no consumer stall); accepted={} rows_ingested={}",
        outcome.accepted_spans, snapshot.rows_ingested
    );
    assert!(
        snapshot.eviction_count > 0,
        "sustained-extreme: retention eviction must fire during the 5-minute window (60s retention, 10s cadence); eviction_count=0"
    );
    assert!(
        snapshot.retention_window_active,
        "sustained-extreme: retention window must be marked active after first sweep"
    );

    monitor_stop.store(1, Ordering::Relaxed);
    monitor.abort();
    retention_handle.abort();

    // Degraded-but-functional: the receiver still accepts a clean post-run export.
    let post = run_paced_injection(
        rig.addr,
        1,
        1_000,
        Duration::from_secs(2),
        Arc::clone(&counter),
        "load-sustained-post",
    )
    .await;
    assert_eq!(
        post.rejected_batches, 0,
        "sustained-extreme: receiver must remain functional after the extreme window"
    );
    println!(
        "load-profile sustained PASS: rows_ingested={} eviction_count={} post-run accepted={}",
        snapshot.rows_ingested, snapshot.eviction_count, post.accepted_spans
    );
    shutdown_rig(rig).await;
}
