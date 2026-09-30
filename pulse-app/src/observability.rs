use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::{env, fs};

use serde_json::{Map, Value};
use tracing::Subscriber;
use tracing::field::{Field, Visit};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::{ErrorLayer, SpanTrace};
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[doc(hidden)]
pub const SERVICE_NAME: &str = "com.andromeda.pulse";

/// Chunk #73 P-003: shared atomic signaled by the panic hook so the
/// connection FSM (`crates/ingest::connection::ReceiverBindStatus`) can
/// route to the `ReceiverPanicked` reason variant. Read via the
/// `panic_signaled()` accessor below; written exactly once by
/// `install_panic_hook`'s closure. Module-level static rather than Arc-
/// injected to minimize boot-time wiring surface (one accessor pulse-app-
/// wide), per the security extract's "narrowest exposure" guidance.
static PANIC_SIGNAL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Returns true if a panic has been signaled by `install_panic_hook` since
/// process start. Used by `pulse-app::connection_router::HeartbeatBindStatus`
/// to expose panic-state to the FSM poller.
pub fn panic_signaled() -> bool {
    PANIC_SIGNAL.load(std::sync::atomic::Ordering::Relaxed)
}

/// Test-only reset for the panic atomic. `#[doc(hidden)] pub` per
/// testing.md 2026-05-20 session 107 integration-test-access pattern
/// (signals "not external API but accessible for integration tests");
/// per-process state means tests must reset between panics to avoid
/// cross-test contamination.
#[doc(hidden)]
pub fn reset_panic_signal_for_tests() {
    PANIC_SIGNAL.store(false, std::sync::atomic::Ordering::Relaxed);
}

#[derive(Clone)]
#[doc(hidden)]
pub struct DefaultFields {
    pub service_name: &'static str,
    pub service_version: &'static str,
    pub deployment_environment: String,
    pub ci_run_id: Option<String>,
    pub git_commit_sha: Option<String>,
}

#[doc(hidden)]
pub struct DefaultFieldsResult {
    pub fields: DefaultFields,
    pub warnings: Vec<String>,
}

impl DefaultFields {
    pub fn from_env() -> DefaultFieldsResult {
        let mut warnings = Vec::new();

        let ci_run_id = env::var("GITHUB_RUN_ID").ok().and_then(|raw| {
            if validate_ci_run_id(&raw) {
                Some(raw)
            } else {
                warnings.push(format!(
                    "GITHUB_RUN_ID rejected (raw_len={}): not 1-32 ascii digits",
                    raw.len()
                ));
                None
            }
        });

        let git_commit_sha = env::var("GITHUB_SHA").ok().and_then(|raw| {
            if validate_git_commit_sha(&raw) {
                Some(raw)
            } else {
                warnings.push(format!(
                    "GITHUB_SHA rejected (raw_len={}): not 7-64 lowercase hex",
                    raw.len()
                ));
                None
            }
        });

        DefaultFieldsResult {
            fields: DefaultFields {
                service_name: SERVICE_NAME,
                service_version: env!("CARGO_PKG_VERSION"),
                deployment_environment: resolve_deployment_environment(),
                ci_run_id,
                git_commit_sha,
            },
            warnings,
        }
    }
}

#[doc(hidden)]
pub fn validate_ci_run_id(s: &str) -> bool {
    !s.is_empty() && s.len() <= 32 && s.chars().all(|c| c.is_ascii_digit())
}

#[doc(hidden)]
pub fn validate_git_commit_sha(s: &str) -> bool {
    s.len() >= 7
        && s.len() <= 64
        && s.chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

#[doc(hidden)]
pub fn resolve_deployment_environment() -> String {
    match env::var("GITHUB_REF") {
        Ok(r) if r.starts_with("refs/tags/v") => "production".into(),
        Ok(r) if r.starts_with("refs/tags/alpha") || r.starts_with("refs/tags/beta") => {
            "staging".into()
        }
        Ok(_) => "dev".into(),
        Err(_) => "production".into(),
    }
}

// Per-module field allowlist for the scrubber. Default-deny: a target with
// no registered allowlist has every field redacted. Per obs-plan §8.
// `pub` + doc(hidden) so the resolver probes can run as integration tests:
// `[lib] test = false` (the Windows WebView2 workaround) means a src-level
// `mod tests` compiles but never executes.
#[doc(hidden)]
pub struct AllowList {
    by_target: HashMap<&'static str, HashSet<&'static str>>,
}

impl AllowList {
    #[doc(hidden)]
    pub fn production() -> Self {
        let mut by_target: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        // obs-plan §8 per-module allowlists. The `ingest` entry covers heartbeat
        // (`ingest.tick`) AND gRPC boundary spans (target = "ingest::grpc" via
        // `#[tracing::instrument]` module-path default; resolved through
        // for_target's split('.') / split("::") fallbacks).
        by_target.insert(
            "ingest",
            [
                "span_count",
                "service",
                "buffer_capacity_pct",
                "broadcast_subscribers",
                "rpc.system",
                "rpc.service",
                "rpc.method",
                "traceparent",
                "latency_ms",
                "method",
                "status",
                "attributes_count",
                "service_name_tag",
                "trace_ids",
                "http.method",
                "http.route",
                "http.status_code",
                "content_type",
                "route",
                "body_size_bytes",
                "limit_bytes",
                "http_response_code",
                "status_code",
                "host_header_rejected",
                "expected_host_class",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "buffer",
            [
                "rows_ingested",
                "eviction_count",
                "memory_bytes",
                "retention_window_seconds",
                "retention_window_active",
                "eviction_count_since_last_tick",
                // Chunk #69 Phase B Session 6 — Drain heartbeat tick fields.
                // Forward slot: emission lands when buffer.tick heartbeat is
                // extended to query DrainMiner.template_count() (Step 4
                // follow-up; not part of this chunk's scope). Pre-registered
                // here so `buffer.tick` events surface fields cleanly when
                // emission lands instead of redacting under default-deny.
                "drain_template_count",
                "drain_lru_evictions_since_tick",
                // Fingerprint-feed throughput on buffer.tick.
                "span_events_seen",
                "fingerprints_computed",
                "observer_invocations",
                // PII redactions applied on the OTLP persistence path — an
                // aggregate count, never the matched value or its category.
                "redactions_applied",
                // Drain progress on buffer.tick: rows landed since the previous
                // tick, and the age of the most recent append. Aggregate
                // numerics — no service, span, or fingerprint identity.
                "rows_ingested_delta",
                "last_append_age_seconds",
                // Batches the appender REJECTED (PK violation / append error),
                // folded once per failed batch — the aggregate the ERROR-level
                // duckdb.append record cannot supply as a count.
                "append_rejections",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // Consumer-stall transition target. Its OWN exact leaf is load-bearing:
        // `for_target` strips `.tick` then falls back to the first `.`-segment,
        // so without this entry "buffer.consumer.stalled" resolves to the
        // `buffer` set above and all four fields below redact — which would
        // make the one signal that distinguishes a wedged consumer from a
        // healthy one unreadable in production. Bounded static labels + two
        // numerics; never a service name, span id, or batch content.
        by_target.insert(
            "buffer.consumer.stalled",
            [
                "reason",
                "consequence",
                "stalled_seconds",
                "buffer_capacity_pct",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // chunk #65 — duckdb.append target emitted by consumer::dispatch_batch
        // per table append (spans / span_events / metrics_points / log_records).
        // Happy path: rows_appended + duration_ms + table_name. Error path
        // (consumer.rs::run_consumer dispatch error branch): reject_reason.
        // Resolver collapses "duckdb.append" → "duckdb" via split('.').next().
        by_target.insert(
            "duckdb",
            [
                "rows_appended",
                "duration_ms",
                "table_name",
                "reject_reason",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "plugin",
            [
                "plugin_name",
                "plugin_path_basename",
                "capability_name",
                "error_msg",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // obs-plan §3 plugins.tick (workspace crate name is `plugins` plural,
        // distinct from the `plugin` singular target above — both coexist).
        by_target.insert(
            "plugins",
            ["loaded_count", "active_invocations"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "snapshot",
            [
                "token_budget",
                "token_count_actual",
                "dedup_count",
                "time_range_start",
                "time_range_end",
                "input_row_count",
                "output_row_count",
                "latency_outlier_count",
                "error_cluster_count",
                "cardinality_spike_count",
                "critical_path_span_count",
                "total_span_count",
                "orphan_parent_count",
                "duration_ms",
                "anomaly_markers_count",
                // chunk #40 — aggregation + attribute_filter primitives in
                // crates/snapshot/{aggregation,attribute_filter}.rs (attribute_filter
                // stays in snapshot per chunk #58; aggregation moved to curation).
                "metric_input_count",
                "metric_output_count",
                "service_count",
                "kept_attribute_count",
                "dropped_attribute_count",
                "p50_ms",
                "p95_ms",
                "p99_ms",
                "max_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // chunk #58 — curation crate extraction. Moved modules
        // (crates/curation/src/{dedupe,anomaly,critical_path,aggregation}.rs)
        // emit #[tracing::instrument] spans at default module-path targets
        // (`curation::dedupe`, `curation::anomaly`, etc.). for_target() falls
        // through via split("::").next() → "curation" entry. Field set
        // covers union across the 4 primitives; mirrors the snapshot crate's
        // shape per .claude/rules/observability.md Session Addition 2026-05-03
        // singular-vs-plural discipline.
        by_target.insert(
            "curation",
            [
                "input_row_count",
                "output_row_count",
                "dedup_count",
                "latency_outlier_count",
                "error_cluster_count",
                "cardinality_spike_count",
                "critical_path_span_count",
                "total_span_count",
                "orphan_parent_count",
                "anomaly_markers_count",
                "duration_ms",
                "metric_input_count",
                "metric_output_count",
                "service_count",
                "p50_ms",
                "p95_ms",
                "p99_ms",
                "max_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // Chunk #59 — connection state machine emits at three targets:
        // - `connection.tick` (15s heartbeat sibling, via the .tick suffix-strip
        //   resolver → "connection" entry)
        // - `connection.state.transition` (1-2s poller, on state change only)
        // - `connection.current_state.request` (TauRPC handler instrument span)
        // Each non-tick target needs an explicit per-leaf entry per the
        // .claude/rules/observability.md Session Addition 2026-05-07
        // dotted-target resolver discipline (resolver does NOT fall back from
        // sub-namespace to crate-name; absent entry redacts ALL fields).
        by_target.insert(
            "connection",
            [
                "state",
                "last_span_ago_ms",
                "severity",
                "from_state",
                "to_state",
                "trigger_reason",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "connection.state.transition",
            [
                "from_state",
                "to_state",
                "last_span_ago_ms",
                "trigger_reason",
                "severity",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "connection.current_state.request",
            ["state", "last_span_ago_ms", "severity"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "viz",
            [
                "query_id",
                "param_count",
                "param_types",
                "row_count",
                "row_count_returned",
                "latency_ms",
                "query_latency_ms",
                "subscribers_active",
                "time_window_seconds",
                "traceparent",
                "filter_count",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "workspace-detector",
            [
                "workspace.root_path",
                "workspace.project_name",
                "vcs_type",
                "vcs_root",
                "marker_present",
                "detection_latency_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "ui-bridge",
            [
                "method_name",
                "argument_digest",
                "result_type",
                "latency_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // chunk #26 — module-boundary error events emitted from
        // crates/ui-bridge/src/contract.rs From<E> for AppError impls per
        // obs-plan §10 module-boundary error logging. Each variant is a
        // separate entry so for_target() exact-match lookup wins over the
        // split('.').next() fallback to "ui-bridge" (which has the
        // unrelated procedure-level field set above). Allowed fields cover
        // the conversion-site triple (error_category / source_kind /
        // source_crate) plus the per-variant structured handle the
        // downstream React UI binds against (field for Validation /
        // plugin_id for Plugin / resource for NotFound).
        by_target.insert(
            "ui-bridge.error.validation",
            ["error_category", "source_kind", "source_crate", "field"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "ui-bridge.error.not_found",
            ["error_category", "source_kind", "source_crate", "resource"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "ui-bridge.error.internal",
            ["error_category", "source_kind", "source_crate"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "ui-bridge.error.plugin",
            ["error_category", "source_kind", "source_crate", "plugin_id"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "ui-bridge.error.storage",
            ["error_category", "source_kind", "source_crate"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "ui-bridge.error.ingest",
            ["error_category", "source_kind", "source_crate"]
                .iter()
                .copied()
                .collect(),
        );
        // chunk #27 — top-level introspection procedure events emitted by the
        // crates/ui-bridge/src/health.rs::IntrospectionApi resolvers per
        // obs-plan §3 IPC boundaries + §6 boundary-call wrappers TauRPC row.
        // Each procedure is a separate exact-match entry so for_target()
        // wins over the split('.').next() fallback to "ui-bridge" (which has
        // the generic procedure-level field set above without the
        // per-procedure result-shape handles).
        by_target.insert(
            "ui-bridge.app_info",
            [
                "method_name",
                "argument_digest",
                "result_type",
                "latency_ms",
                "name",
                "version",
                "rust_version",
                "tauri_version",
                "build_profile",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "ui-bridge.health",
            [
                "method_name",
                "argument_digest",
                "result_type",
                "latency_ms",
                "status",
                "subsystems",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "ui-bridge.ready",
            [
                "method_name",
                "argument_digest",
                "result_type",
                "latency_ms",
                "ready",
                "duckdb_connection",
                "ingest_mpsc_capacity_pct",
                "broadcast_subscribers",
                "plugins_loaded",
                "mcp_server_enabled",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "ui-bridge.get_settings",
            [
                "method_name",
                "argument_digest",
                "result_type",
                "latency_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "ui-bridge.update_settings",
            [
                "method_name",
                "argument_digest",
                "result_type",
                "latency_ms",
                "setting_keys_changed",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // chunk #27 — xtask capability-drift gate. Emitted by the xtask binary
        // when the drift check runs; pulse-app does not currently emit at
        // this target but the entry is registered for forward-compat in case
        // a future inline drift surface lands.
        by_target.insert(
            "xtask.capability_drift",
            [
                "missing_count",
                "extra_count",
                "drift_state",
                "top_5_drifted_names",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "mcp-server",
            [
                "method",
                "result_type",
                "result_count",
                "feature_flag_enabled",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );

        // App-internal targets (chunk #4 + chunk #7-8 boot/panic events)
        by_target.insert(
            "app.boot.tracing.init",
            ["log_dir_basename", "default_fields_active"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "app.boot.pid",
            [
                "pid",
                "path_basename",
                "error",
                "run_dir_basename",
                "data_dir_basename",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert("app.boot.ci-fields", ["reason"].iter().copied().collect());
        by_target.insert(
            "app.boot.otlp.grpc.bind",
            ["bind_address", "reason"].iter().copied().collect(),
        );
        by_target.insert(
            "app.boot.otlp.grpc.port",
            ["raw_len"].iter().copied().collect(),
        );
        // Explicit leaf entry: `for_target` would otherwise fall back through
        // split('.') to the unrelated `app` field set and redact both fields.
        by_target.insert(
            "app.boot.buffer.degraded",
            ["reason", "consequence"].iter().copied().collect(),
        );
        // Explicit leaf entry, same fallback hazard. Basename only — the full
        // workspace path is never permitted here (obs-plan §5 Vector 5, the
        // chunk #43 `workspace.detect` precedent).
        by_target.insert(
            "app.boot.workspace_key",
            [
                "workspace_root_basename",
                "key_bytes",
                "error_category",
                "error_detail",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "app.boot.otlp.http.bind",
            ["bind_address", "reason"].iter().copied().collect(),
        );
        by_target.insert(
            "app.boot.otlp.http.port",
            ["raw_len"].iter().copied().collect(),
        );
        // chunk #18 invariant violation scaffold; allowlist now so future
        // populated events pass scrubbing without further allowlist edits.
        by_target.insert(
            "ingest.grpc.parse.error",
            [
                "span_field_invalid",
                "expected_length",
                "actual_length",
                "rejection_reason",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "ingest.http.parse.error",
            [
                "span_field_invalid",
                "expected_length",
                "actual_length",
                "rejection_reason",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "ingest.http.body_size.exceeded",
            ["body_size_bytes", "limit_bytes", "http_response_code"]
                .iter()
                .copied()
                .collect(),
        );
        // chunk #18 mpsc channel events
        by_target.insert(
            "ingest.channel.send",
            ["channel_name", "capacity_pct", "subscribers"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "ingest.channel.full",
            ["channel_name", "capacity_pct", "rejection_reason"]
                .iter()
                .copied()
                .collect(),
        );
        // chunk #19 rate-limit + port-validation events. Vector-6 generalization:
        // port-validation logs the env-var NAME + reject category only, never
        // the raw user-controlled value (per security plan §Logging + obs-plan
        // §11 Vector 6).
        by_target.insert(
            "config.load.port_validation",
            ["env_var_name", "reject_reason"].iter().copied().collect(),
        );
        by_target.insert(
            "ingest.grpc.rate_limit.rejected",
            ["quota_window_seconds", "reject_reason"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "ingest.http.rate_limit.rejected",
            ["quota_window_seconds", "reject_reason"]
                .iter()
                .copied()
                .collect(),
        );
        // chunk #20 buffer crate boot + Arrow appender events
        by_target.insert(
            "buffer.schema.init",
            ["table_count", "duration_ms"].iter().copied().collect(),
        );
        by_target.insert(
            "buffer.schema.init.error",
            ["error_type", "spantrace"].iter().copied().collect(),
        );
        by_target.insert(
            "duckdb.append",
            [
                "rows_appended",
                "duration_ms",
                "table_name",
                "reject_reason",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // chunk #21 retention sweep events. Direct-target lookup wins over the
        // `buffer` prefix-strip fallback per for_target resolver order.
        by_target.insert(
            "buffer.retention.sweep",
            [
                "query_id",
                "param_count",
                "param_types",
                "rows_evicted",
                "duration_ms",
                "cutoff_ts_unix_nano",
                "table_name",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "buffer.retention.sweep.error",
            ["error_type", "duration_ms", "spantrace", "table_name"]
                .iter()
                .copied()
                .collect(),
        );
        // chunk #21 retention env-var validation event (mirrors
        // config.load.port_validation precedent — name + reject category only,
        // never the raw user-controlled value per security plan §Logging
        // Vector 6).
        by_target.insert(
            "config.load.retention_seconds",
            ["env_var_name", "reject_reason"].iter().copied().collect(),
        );
        // chunk #96 configuration hot-reload events. Aggregate-only per
        // obs-plan §6/§8 + the triage cardinality discipline (CLAUDE.md
        // observability 2026-05-17): counts + bounded enum/category labels
        // ONLY — NEVER the raw config values, the full config.toml path, or
        // per-key contents (per security plan §Logging NEVER-log list).
        by_target.insert(
            "config.load",
            [
                "hot_applied_count",
                "restart_required_count",
                "silent_count",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "config.load.rejected",
            ["error_category"].iter().copied().collect(),
        );
        by_target.insert(
            "config.load.path_validation",
            ["reason"].iter().copied().collect(),
        );
        by_target.insert(
            "config.watcher.boot",
            ["error_category"].iter().copied().collect(),
        );
        by_target.insert(
            "config.reload.request",
            ["outcome", "hot_applied_count", "restart_required_count"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "config.status.request",
            ["restart_required_pending", "reload_count"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "cadence.config.reload_applied",
            [
                "baseline_seconds",
                "reflection_seconds",
                "tier2_acceleration_enabled",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "diagnostics.reevaluate_recent_window.request",
            ["services_reclassified", "transitions_emitted"]
                .iter()
                .copied()
                .collect(),
        );
        // chunk #95 storage.export_for_training — aggregate-only fields per
        // obs-plan §6/§8 + the triage cardinality discipline (CLAUDE.md
        // observability 2026-05-17). NEVER the exported corpus content, the
        // full target path, or per-record values: basename + counts only.
        by_target.insert(
            "storage.export_for_training.request",
            [
                "record_count",
                "redacted_field_count",
                "target_path_basename",
                "written",
                "duration_ms",
                "error_category",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // chunk #21 metric.buffer.* events. Specific entries take precedence
        // over the generic `metric` prefix-strip fallback (which only allows
        // value/unit/module).
        by_target.insert(
            "metric.buffer.memory_bytes",
            [
                "value",
                "retention_window_seconds",
                "rows_active",
                "eviction_count_since_last_tick",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "metric.buffer.evicted_span_count",
            ["value", "retention_window_seconds"]
                .iter()
                .copied()
                .collect(),
        );
        // chunk #22 viz query routers — error path + per-query metric events.
        // Specific entries take precedence over the `viz` prefix-strip fallback.
        by_target.insert(
            "viz.query.error",
            ["error_type", "duration_ms", "spantrace"]
                .iter()
                .copied()
                .collect(),
        );
        // metric.trace.* events emit from inside viz.query.traces; service_name
        // is the documented obs-plan §5 cardinality exemption (query-time scope
        // only, not time-series-stored).
        by_target.insert(
            "metric.trace.latency_percentiles",
            [
                "value",
                "service_name",
                "p50_ms",
                "p95_ms",
                "p99_ms",
                "max_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "metric.trace.error_count",
            ["value", "service_name", "error_type"]
                .iter()
                .copied()
                .collect(),
        );
        // chunk #23 broadcast fan-out + Tauri Channel API. Per-target entries
        // for the 5 new event surfaces (tauri.channel.emit success span +
        // size_exceeded error variant + emit.error encode-failure variant +
        // lag warn variant + per-stream broadcast subscriber gauge).
        // Specific entries take precedence over the `tauri` prefix-strip
        // fallback (which has no entry — stays default-deny outside this list).
        by_target.insert(
            "tauri.channel.emit",
            [
                "channel_name",
                "arrow_schema",
                "row_count",
                "payload_size_bytes",
                "schema_match",
                "traceparent",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "tauri.channel.emit.size_exceeded",
            [
                "channel_name",
                "payload_size_bytes",
                "limit_bytes",
                "error_type",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "tauri.channel.emit.error",
            ["channel_name", "error_type", "duration_ms", "spantrace"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "tauri.channel.lag",
            ["channel_name", "dropped_count"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.ingest.channel.broadcast_subscribers",
            ["value", "channel_name"].iter().copied().collect(),
        );
        // chunk #24 webview shell — boot-time platform detection spans + close→
        // minimize-to-tray transition + tray-boundary surfaces. Tray events
        // register here at chunk #24 (default-deny coverage); first emissions
        // land at chunk #32 when the actual tray icon + menu plumbing arrives.
        by_target.insert(
            "app.boot.webview.init",
            ["webview_backend"].iter().copied().collect(),
        );
        by_target.insert(
            "app.boot.gpu.check",
            ["wgpu_backend"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.webgpu.frame_duration_ms",
            [
                "duration_ms",
                "wgpu_backend",
                "webview_backend",
                "timing_method",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // Delegated timing observables (P-025 / P-027 / P-045). Each needs its
        // OWN exact leaf: a bare `metric` key IS registered below with only
        // ["value", "unit", "module"], and `for_target` falls back to the first
        // `.`-segment — so a leaf-less `metric.*` target keeps `value` and has
        // every other field redacted. Aggregate-only: no service identifier.
        by_target.insert(
            "metric.constellation.hue_update_ms",
            ["duration_ms", "severity_tier"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.constellation.discovery_ms",
            ["duration_ms", "discovered_count"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.findings.counter_refresh_ms",
            ["duration_ms"].iter().copied().collect(),
        );
        by_target.insert("app.boot.tray.init", ["tray_api"].iter().copied().collect());
        by_target.insert(
            "app.boot.window.show",
            ["label", "error_kind", "error_msg"]
                .iter()
                .copied()
                .collect(),
        );
        // Its OWN exact leaf. No bare `app` key is registered (measured), so
        // `for_target`'s first-`.`-segment fallback finds nothing and EVERY
        // field is redacted without this entry. All three the emit site emits —
        // a short leaf leaves the target partly redacted. `window_label` is a
        // bounded declared label, `reason` a bounded static; neither the URL nor
        // any window content is emitted.
        by_target.insert(
            "app.boot.window.navigation",
            ["window_label", "navigated", "reason"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "ui.layout.transition",
            [
                "layout_mode_from",
                "layout_mode_to",
                "tray_visible",
                "always_on_top",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // Capability-rejected webview IPC record (security-plan §Logging &
        // Monitoring "What to log"): bounded category + coerced window label +
        // byte count only — never the payload, the command name, or the raw
        // rejection text. Its OWN exact leaf: NO bare `ui` key is registered
        // (measured), so without this entry `for_target`'s first-`.`-segment
        // fallback resolves nothing and every field is silently redacted.
        by_target.insert(
            "ui.ipc.rejection",
            ["error_category", "window_label", "payload_bytes"]
                .iter()
                .copied()
                .collect(),
        );
        // One webview WebGPU adapter request: its closed outcome + a coerced
        // window label, nothing else. Same no-bare-`ui`-key reason as above.
        by_target.insert(
            "ui.webgpu.adapter",
            ["outcome", "window_label"].iter().copied().collect(),
        );
        by_target.insert(
            "tray.visibility.toggle",
            ["tray_visible"].iter().copied().collect(),
        );
        by_target.insert(
            "tray.menu.interaction",
            ["menu_item"].iter().copied().collect(),
        );
        by_target.insert(
            "tray.notification.dismiss",
            ["notification_id"].iter().copied().collect(),
        );
        by_target.insert(
            "tray.notification.action",
            ["notification_id", "action"].iter().copied().collect(),
        );
        // Close-to-tray signpost (P-063): bounded window label only — there is
        // no bare `tray` key, so without this exact leaf `for_target` resolves
        // the target to None and the field would be redacted.
        by_target.insert(
            "tray.signpost.shown",
            ["window_label"].iter().copied().collect(),
        );
        by_target.insert(
            "app.panic.fatal",
            // Chunk #73 P-003: `panic_message` removed — panic payload may
            // carry user secrets from instrumented hosts. Only location
            // (file:line of pulse source) + spantrace (user-defined span
            // hierarchy) are agent-readable surfaces.
            ["location", "spantrace"].iter().copied().collect(),
        );

        // a11y plan §3 violation JSON Required + Extension fields (forward-binding
        // for chunk #15 a11y dev stack — chunk #7's subscriber must not redact)
        by_target.insert(
            "a11y::assertion",
            [
                "wcag_criterion",
                "violation_type",
                "severity",
                "surface",
                "selector",
                "remediation",
                "tool",
                "tool_result_id",
                "token_name",
                "measured_value",
                "required_value",
                "affected_component",
            ]
            .iter()
            .copied()
            .collect(),
        );

        // metric.* prefix (obs-plan §metrics convention)
        by_target.insert(
            "metric",
            ["value", "unit", "module"].iter().copied().collect(),
        );

        // chunk #41 — markdown formatter span + budget validation + perf-budget
        // metric. Dotted targets registered explicitly so for_target() exact-match
        // wins over the snapshot crate-level fallback (which would silently
        // redact markdown_size_bytes / budget_token_limit / budget_exceeded /
        // truncated_*_count) per .claude/rules/observability.md Session
        // Addition 2026-05-07.
        by_target.insert(
            "snapshot.render.markdown",
            [
                "markdown_size_bytes",
                "section_count",
                "token_count_actual",
                "budget_token_limit",
                "anomaly_marker_count",
                "critical_path_span_count",
                "truncated_span_count",
                "truncated_attribute_count",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "snapshot.token.count.validate",
            [
                "token_count_actual",
                "token_budget_limit",
                "budget_exceeded",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "metric.snapshot.token_count_ms",
            [
                "value",
                "duration_ms",
                "token_budget",
                "time_range_minutes",
                "token_count_actual",
                "dedup_count",
                "budget_exceeded",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // chunk #42 — Investigate trigger surface; chunk #43 extends with
        // success-path fields when the resolver actually completes the
        // curate→write→clipboard→notify pipeline (token_count,
        // dual_file_paths_basenames, preset_prompts_count). Explicit per-leaf
        // entry so for_target() exact match wins over the `snapshot`
        // crate-level fallback (which carries unrelated curation fields).
        by_target.insert(
            "snapshot.generate.request",
            [
                "budget",
                "result_kind",
                "message",
                "token_count",
                "dual_file_paths_basenames",
                "preset_prompts_count",
            ]
            .iter()
            .copied()
            .collect(),
        );

        // chunk #43 — clipboard write site emits a non-suppressible
        // "X bytes copied" event per security plan §Logging clipboard
        // hygiene. Only `byte_count` + `success` + (optional)
        // `dual_file_paths_basenames` are permitted; clipboard payload
        // text and snapshot file contents are redacted by default-deny.
        by_target.insert(
            "snapshot.clipboard.write",
            ["byte_count", "success", "dual_file_paths_basenames"]
                .iter()
                .copied()
                .collect(),
        );

        // chunk #43 — notification dispatch site. Only the enum-valued
        // `notification_kind` + boolean `dispatch_success` are permitted;
        // toast body text could carry preset-prompt strings in future
        // iterations and is redacted by default-deny.
        by_target.insert(
            "snapshot.notification.dispatch",
            ["notification_kind", "dispatch_success"]
                .iter()
                .copied()
                .collect(),
        );

        // chunk #43 — workspace.detect resolver emission per obs-plan
        // §4 Scenario P7. All path fields are basenames only; raw
        // workspace path values + VCS metadata content NEVER appear.
        by_target.insert(
            "workspace.detect",
            [
                "workspace_root_basename",
                "project_name",
                "vcs_type",
                "vcs_root_basename",
                "marker_present",
                "detection_latency_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );

        // Chunk #61 — triage baseline trackers + corpus persistence
        // (Epoch 9 Foundation v0.2.0 fifth chunk; capabilities P-009 + P-011).
        // All 7 new tracing target leaves; field shape per route §3 chunk #61
        // Decisions Log + obs-plan §5 metric naming + plan.md §AllowList step 8.
        // PII discipline (per security plan §Anti-Patterns § Logging row 1 +
        // CLAUDE.md universal invariant): no `service.name` / `span_id` /
        // `trace_id` / `operation_name` / raw corpus path fields admitted.
        by_target.insert(
            "triage.baseline.tick",
            [
                "value",
                "service_id_count",
                "tdigest_centroid_count",
                "dropped_count",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.baseline.persist",
            [
                "service_id_count",
                "state_size_bytes",
                "duration_ms",
                "persist_kind",
                "corpus_basename",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.baseline.persist.error",
            ["error_category", "duration_ms"].iter().copied().collect(),
        );
        // Chunk #70 — BaselineState → corpus migration. Aggregate-only
        // event surfaces per CLAUDE.md 2026-05-17 session 84 + chunks
        // #62/#63/#64 triage AllowList convention: bounded enum
        // (`migration_outcome` / `error_category`) + integer-value fields
        // (`legacy_state_size_bytes` / `migrated_service_count` /
        // `duration_ms`) only. NO `service_name` / `scope_id` /
        // `legacy_path` / per-record content fields.
        by_target.insert(
            "triage.baseline.migrate",
            [
                "legacy_state_size_bytes",
                "migrated_service_count",
                "duration_ms",
                "migration_outcome",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.baseline.migrate.failed",
            ["error_category", "duration_ms"].iter().copied().collect(),
        );
        // Chunk #71 — ServiceRegistry + RetryStormState corpus persistence.
        // Aggregate-only fields per CLAUDE.md 2026-05-17 session 84 + chunk
        // #62/#63/#64 triage AllowList convention: bounded enum
        // (`persist_kind` / `error_category`) + bounded numeric fields
        // (`service_count` / `state_size_bytes` / `duration_ms`) + the
        // existing `corpus_basename` field-name convention (chunk #70
        // precedent). NO `service_name` / `scope_id` / per-record content.
        by_target.insert(
            "triage.lifecycle.persist",
            [
                "service_count",
                "state_size_bytes",
                "duration_ms",
                "persist_kind",
                "corpus_basename",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.lifecycle.persist.error",
            ["error_category", "duration_ms"].iter().copied().collect(),
        );
        by_target.insert(
            "triage.service_id_missing",
            ["dropped_count"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.baseline.ewma_short_window_size",
            ["value", "service_id_count"].iter().copied().collect(),
        );
        by_target.insert(
            "pipeline.l1b.persist_count_total",
            ["value", "duration_ms", "persist_kind", "state_size_bytes"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "pipeline.l1b.bootstrap_count_total",
            ["value", "kind"].iter().copied().collect(),
        );
        // The four L1a query targets need their OWN exact leaves: the bare
        // `metric` key exists (value/unit/module), so the first-`.`-segment
        // fallback would keep `value` and silently redact every label
        // (obs-plan §8 metric.* rule). Field sets enumerated from the emit
        // sites in `crates/triage/src/baseline/sql.rs`.
        by_target.insert(
            "metric.pipeline.l1a.query_count_total",
            ["query_name", "value"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.pipeline.l1a.query_latency_p99_milliseconds",
            ["query_name", "duration_ms", "row_count_returned"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.pipeline.l1a.q7_timeout_count_total",
            ["value", "timeout_ms", "rejection_reason"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.pipeline.l1a.q7_fallback_count_total",
            ["value", "fallback_query_kind", "cause"]
                .iter()
                .copied()
                .collect(),
        );

        // Chunk #62 — attention cue emitter (Epoch 9 Foundation v0.2.0
        // sixth chunk; capability P-021 + P-019 partial). 4 new tracing
        // target leaves emitted by `crates/triage/src/cue/emitter.rs`.
        // PII discipline (per security plan §Anti-Patterns § Logging row 1):
        // no `scope_id` admitted (carries user-controlled service.name);
        // only bounded-cardinality enum tags + structural numeric values.
        // `cues_suppressed` + `bypass_triggered` shipped at chunk #63 but were
        // never added here, so they rendered `"<redacted>"` in production and
        // the question "did suppression engage?" was unanswerable from any log
        // (obs-plan §8 muted-diagnostic backlog). `cues_latched` +
        // `latch_tracked` arrive with the `CueLatch` bound. Enumerated from the
        // emit site in `crates/triage/src/cue/emitter.rs` — a leaf naming fewer
        // fields than the site emits leaves the target PARTLY redacted, which
        // every gate passes.
        by_target.insert(
            "triage.cue.tick",
            [
                "cues_evaluated",
                "cues_emitted",
                "cadence_triggers_emitted",
                "services_tracked",
                "operations_tracked",
                "cues_suppressed",
                "bypass_triggered",
                "cues_latched",
                "latch_tracked",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.cue.evaluate",
            [
                "services_tracked",
                "operations_tracked",
                "error_rate_multiplier",
                "latency_multiplier",
                "source",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.cue.emit",
            [
                "kind",
                "priority",
                "scope",
                "magnitude",
                "absolute_value",
                "persistence",
                "confidence",
                "suppression_bypassed",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "metric.cue.emit_count_total",
            ["value", "kind", "priority", "scope"]
                .iter()
                .copied()
                .collect(),
        );

        // Chunk #63 — restart event detector + dual-condition bypass
        // (Epoch 9 Foundation v0.2.0 seventh chunk; capabilities P-015 +
        // P-016 + P-057). 6 new tracing target leaves: 3 emitted by
        // `crates/triage/src/pattern/{detector,broadcast}.rs` + 3 emitted
        // by `crates/triage/src/cue/emitter.rs` (suppression hook + bypass
        // metric).
        //
        // PII discipline (per security plan §Anti-Patterns § Logging row 1
        // + chunk #62 precedent): no `service`/`scope_id` / `span_id` /
        // `trace_id` / OTLP attribute value fields admitted. Only
        // identifier-class numeric + enum tag + boolean fields cross the
        // scrubber boundary. Per-leaf entries (exact-match resolver
        // precedence; per `.claude/rules/observability.md` Session
        // Additions 2026-05-07).
        by_target.insert(
            "triage.pattern.tick",
            [
                "value",
                "gap_threshold_seconds",
                "services_tracked",
                "restart_events_emitted",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.pattern.restart_detect",
            ["cue_kind", "gap_seconds", "restart_window_active"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "triage.pattern.restart_emit",
            ["cue_kind", "gap_seconds"].iter().copied().collect(),
        );
        by_target.insert(
            "triage.cue.suppression_check",
            [
                "cue_kind",
                "persistence",
                "restart_window_active",
                "suppression_bypassed",
                "bypass_reason",
                "skipped_events",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.cue.suppression_bypass",
            ["cue_kind", "bypass_reason"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.pipeline.l2.magnitude_bypass_triggered_total",
            ["value", "reason", "cue_kind", "priority"]
                .iter()
                .copied()
                .collect(),
        );

        // Chunk #64 — activity floor learning + corpus persistence (Epoch 9
        // Foundation v0.2.0 eighth chunk; capabilities P-013 + P-014). 3 new
        // tracing target leaves emitted by `crates/triage/src/cue/emitter.rs`
        // (service_went_silent aggregate evaluate + bootstrap_state metric)
        // + `crates/triage/src/baseline/mod.rs` (service-name cardinality cap
        // aggregate warn). PII discipline (per security plan §Anti-Patterns
        // § Logging row 1 + chunk #62/#63 precedent): no `service`/`scope_id`
        // / OTLP attribute fields admitted — only bounded-cardinality count +
        // structural numeric fields cross the scrubber boundary. ServiceWent-
        // Silent cue payloads on the `pulse://stream/attention-cues` broadcast
        // topic are product surface (contains `scope_id` = service.name);
        // self-observation events strictly aggregate.
        by_target.insert(
            "triage.baseline.service_went_silent.evaluate",
            [
                "services_tracked",
                "services_in_bootstrap",
                "services_ready",
                "silence_cues_emitted",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.baseline.service_cap_exceeded",
            ["dropped_count", "cap"].iter().copied().collect(),
        );
        // Once-per-boot notice that the per-service cold-start window is not
        // the default (env override, or a rejected value falling back). EXACT
        // leaf — a bare `triage` prefix key would widen every sibling target
        // to one field set. All three fields the emit site emits are listed;
        // all are bounded (two counts + a static reason label).
        by_target.insert(
            "triage.baseline.bootstrap_window.override",
            ["resolved_seconds", "default_seconds", "reason"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.triage.activity_floor.bootstrap_state",
            [
                "value",
                "services_in_bootstrap",
                "services_ready",
                "services_tracked",
            ]
            .iter()
            .copied()
            .collect(),
        );

        // Chunk #66 — retry storm detector + exception fingerprinting (Epoch
        // 9 Foundation v0.2.0 tenth chunk; capabilities P-017 + P-018).
        // 6 new tracing target leaves: 3 emitted by
        // `crates/triage/src/pattern/storm.rs::observe_and_dispatch_storm`
        // (storm detected + emit + tick) + 3 metric event targets fed by
        // the same module's cycle + dispatch helpers.
        //
        // PII discipline (per chunks #62/#63/#64 AllowList convention
        // reaffirmed by `.claude/rules/testing.md` Session Additions
        // 2026-05-17 session 84 aggregate-only mandate): NO `service_name`
        // / `scope_id` / `span_id` / `trace_id` / `attribute` / `body` /
        // `exception_message` / `exception_stacktrace` fields admitted —
        // only opaque `fingerprint_hex` (8-char hash prefix), bounded enum
        // tags (`cue_kind`, `severity_hint`), and bounded numeric fields
        // cross the scrubber boundary. RetryStorm cue payloads on the
        // `pulse://stream/attention-cues` broadcast topic are product
        // surface (carry full `scope_id` = service.name); self-observation
        // events strictly aggregate.
        by_target.insert(
            "triage.pattern.storm.detected",
            [
                "cue_kind",
                "severity_hint",
                "occurrence_count",
                "window_seconds",
                "fingerprint_hex",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.pattern.storm.emit",
            ["cue_kind", "severity_hint", "occurrence_count"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "triage.pattern.storm.tick",
            [
                "value",
                "tracked_fingerprints_count",
                "storms_detected_total",
                "fingerprints_evicted_total",
                "window_seconds",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "metric.triage.pattern.storm_detected_count",
            ["value", "severity_hint"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.triage.pattern.fingerprints_tracked",
            ["value"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.triage.pattern.fingerprint_evicted_count",
            ["value"].iter().copied().collect(),
        );
        // Chunk #71 — RetryStormState corpus persistence. Aggregate-only
        // fields per chunk #62/#63/#64 + chunk #71 plan §obs criterion.
        // Note `fingerprint_count` (bounded count of tracked fingerprints
        // in the snapshot) replaces baseline's `service_count`; otherwise
        // mirrors the lifecycle.persist shape.
        by_target.insert(
            "triage.pattern.storm.persist",
            [
                "fingerprint_count",
                "state_size_bytes",
                "duration_ms",
                "persist_kind",
                "corpus_basename",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.pattern.storm.persist.error",
            ["error_category", "duration_ms"].iter().copied().collect(),
        );
        by_target.insert(
            "triage.pattern.storm.corpus_restore",
            ["restored_fingerprint_count", "duration_ms", "kind"]
                .iter()
                .copied()
                .collect(),
        );

        // Chunk #67 — service lifecycle state machine (Epoch 9 Foundation
        // v0.2.0 final chunk; capability P-027). 6 new tracing target
        // leaves: 3 emitted by `crates/triage/src/lifecycle/mod.rs::
        // emit_tick_observability` (heartbeat tick + per-bucket transition
        // counts + corpus-restore no-op stub for chunk #69) + 1 TauRPC
        // request resolver target + 2 metric event targets.
        //
        // PII discipline (per `.claude/rules/observability.md` Session
        // Addition 2026-05-17 session 84 aggregate-only mandate + chunks
        // #62/#63/#64/#66 AllowList convention): NO `service_name` /
        // `service_id` / `scope_id` / `span_id` / `trace_id` /
        // `operation_name` / `attribute` / `body` fields admitted — only
        // bounded enum tags (`from_state`, `to_state`, `kind`) and
        // bounded numeric counts cross the scrubber. Per-service
        // identifiers ride `pulse://stream/service-lifecycle` broadcast
        // surface only.
        by_target.insert(
            "triage.lifecycle.tick",
            [
                "tracked_services_total",
                "services_unknown",
                "services_bootstrapping",
                "services_active",
                "services_quiet",
                "services_silent",
                "services_dormant",
                "services_archived",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.lifecycle.transition",
            ["from_state", "to_state", "count"]
                .iter()
                .copied()
                .collect(),
        );
        // Chunk #67 stub fields `kind` + `count` preserved; chunk #71
        // adds `restored_service_count` + `duration_ms` for production
        // emission per plan §11 EXTEND.
        by_target.insert(
            "triage.lifecycle.corpus_restore",
            ["kind", "count", "restored_service_count", "duration_ms"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "services.list_with_states.request",
            ["item_count", "traceparent"].iter().copied().collect(),
        );
        by_target.insert(
            "pipeline.l1b.tracked_services_total",
            ["value"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.triage.lifecycle.state_distribution",
            [
                "value",
                "services_unknown",
                "services_bootstrapping",
                "services_active",
                "services_quiet",
                "services_silent",
                "services_dormant",
                "services_archived",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // Chunk #71 — boot-restore counter metrics per v0.2.0 route §71
        // specialist-plan-touches line 373. Permits `value` (count) +
        // bounded `kind` enum tag only — NO per-service / per-fingerprint
        // identifiers. Emitted exactly once per boot (count = 0 on
        // cold-start).
        by_target.insert(
            "metric.triage.lifecycle.corpus_restore_count_total",
            ["value", "kind"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.triage.pattern.storm.corpus_restore_count_total",
            ["value", "kind"].iter().copied().collect(),
        );
        // Chunk #69 Phase B Session 6 — Drain template-mining + diagnostics
        // AllowList registration. Per CLAUDE.md 2026-05-07 explicit-leaf
        // pattern + 2026-05-17 session 84 AGGREGATE-ONLY discipline:
        // bounded counts + integer values + enum tags ONLY; NO template_text
        // / sample_message / service_name / scope_id appear in self-
        // observation. `message` is universally allowed by the
        // JsonFieldVisitor (per CLAUDE.md 2026-05-12 special-case) so it
        // does not need to appear here.
        by_target.insert(
            "drain",
            [
                "template_count",
                "evicted_count",
                "schema_version",
                "reason",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "drain.persistence.load.ok",
            ["template_count"].iter().copied().collect(),
        );
        by_target.insert(
            "drain.persistence.unavailable",
            ["reason"].iter().copied().collect(),
        );
        by_target.insert(
            "diagnostics",
            ["top_n", "result_count", "duration_ms"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "diagnostics.template_distribution.request",
            ["top_n", "result_count", "duration_ms"]
                .iter()
                .copied()
                .collect(),
        );
        // Chunk #97 — Settings → Diagnostics view (capability P-058).
        // Aggregate-only fields per the 2026-05-17 session-84 mandate: no
        // per-service / per-trace / payload-value labels. `metric_name` is a
        // bounded allowlist enum tag (4 known values); the `window_seconds`
        // arg is NEVER logged (query-anonymizer discipline).
        by_target.insert(
            "diagnostics.snapshot.request",
            [
                "tier",
                "profile",
                "load_status",
                "backoff_remaining_seconds",
                "consecutive_failures",
                "drain_template_count",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "diagnostics.history.request",
            ["metric_name", "recorded", "point_count", "duration_ms"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.pipeline.l1c.drain_template_count_total",
            ["value"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.pipeline.l1c.drain_assignment_latency_p99_microseconds",
            ["value"].iter().copied().collect(),
        );

        // Chunk #80 — cadence coordinator + three-tier triggering (Epoch 9
        // Foundation v0.2.0 final-chunk; capabilities P-052 + P-060). 5 new
        // tracing target leaves emitted by `crates/triage/src/cadence/
        // coordinator.rs` + `pulse-app/src/main.rs` boot wiring.
        //
        // PII discipline (per chunks #62/#63/#64/#66 AllowList convention
        // reaffirmed by `.claude/rules/testing.md` Session Additions
        // 2026-05-17 session 84 aggregate-only mandate): NO `service` /
        // `scope_id` / `span_id` / `trace_id` / `operation_name` /
        // `digest_body` / `inference_input` / `inference_output` fields
        // admitted — only bounded-cardinality enum tags (`tier`, `mode`,
        // `cue_kind`, `priority`, `field`) + structural numeric fields
        // (counts, timestamps, intervals) cross the scrubber boundary.
        // CadenceEvent broadcast payloads on `pulse://stream/cadence-events`
        // are product surface (carry mode_label + cue_kind_label +
        // cue_priority_label as bounded static strings, never user content);
        // self-observation events strictly aggregate.
        by_target.insert(
            "cadence.tick",
            [
                "tier",
                "mode",
                "cycles_executed",
                "queries_executed",
                "queries_succeeded",
                "next_due_ms",
                "last_executed_at_ms",
                "tier2_acceleration_enabled",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "cadence.trigger",
            ["tier", "mode", "cue_kind", "priority"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "cadence.config.load",
            [
                "baseline_seconds",
                "accelerated_seconds",
                "reflection_seconds",
                "tier2_acceleration_enabled",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "cadence.config.safety_floor",
            ["field", "requested_seconds", "enforced_seconds"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.pipeline.l3.digests_assembled_total",
            ["value", "mode", "queries_executed"]
                .iter()
                .copied()
                .collect(),
        );
        // Chunk #81 — L3 digest assembler tracing targets. Aggregate-only
        // fields per CLAUDE.md observability Session Learnings 2026-05-17
        // (no per-service `service_name` / `scope_id` / per-incident-id).
        by_target.insert(
            "digest.assemble.request",
            [
                "mode",
                "cue_kind",
                "cue_priority_tier",
                "token_count_actual",
                "active_incident_bypass",
                "resolution_event",
                "latency_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "digest.lww.drop",
            ["mode", "cadence_tier", "drop_reason", "dropped_kind"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "digest.lww.replace",
            [
                "mode",
                "cadence_tier",
                "active_incident_bypass",
                "replaced_kind",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "digest.token.count.validate",
            [
                "mode",
                "token_count_actual",
                "token_budget_limit",
                "soft_min",
                "soft_max",
                "budget_exceeded",
                "truncation_applied",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "digest.corpus.retrieve",
            [
                "query_id",
                "param_count",
                "row_count_returned",
                "duration_ms",
                // Chunk #100 failure path — bounded category label only.
                "error_category",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "digest.runtime.persist",
            [
                "rowid",
                "token_count",
                "digest_kind",
                "error_category",
                "error_message",
                "skipped_events",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "digest.runtime.cadence_tick",
            ["mode", "cue_present", "error_category", "skipped_events"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "digest.runtime.boot",
            ["error_message"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.pipeline.l3.digest_token_count_ms",
            [
                "value",
                "mode",
                "token_count_actual",
                "token_budget_limit",
                "budget_exceeded",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "metric.pipeline.l3.lww_drop_count_total",
            ["value", "cadence_tier", "drop_reason"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.pipeline.l3.active_incident_queue_depth",
            ["value", "severity_tier", "queue_depth"]
                .iter()
                .copied()
                .collect(),
        );

        // Chunk #82 — L4 LLM interpretation layer tracing targets.
        // Aggregate-only fields per CLAUDE.md observability Session
        // Learnings 2026-05-17 session 84 mandate (no per-service
        // identifiers + no `model_path` / `checkpoint_url` per
        // .claude/rules/security.md §Logging & Monitoring NEVER-log list).
        // `model_identity` carries semantic model name only (e.g.,
        // "llama-3.2-3b-instruct-q4_k_m") per InterpretationContract;
        // never a file path or URL.
        // No bare `interpretation` prefix key may exist (obs-plan §8): the
        // first-`.`-segment fallback would serve any future unregistered
        // `interpretation.*` target a stale field set while the resolver
        // probe still passes. Every target carries its own exact leaf.
        by_target.insert(
            "interpretation.hardware.detect",
            [
                "profile",
                "detection_latency_ms",
                "gpu_available",
                "cpu_core_count",
                "profile_detection_decision_recorded",
                "override_source",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "interpretation.model.load",
            ["model_identity", "tier", "load_status", "inference_mode"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "interpretation.model.load.error",
            [
                "error_msg",
                "model_identity",
                "recovery_action",
                "error_category",
                "env_var",
                "path_basename",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "interpretation.model.allow_root",
            ["confinement", "root_basename"].iter().copied().collect(),
        );
        by_target.insert(
            "interpretation.tokenizer.init",
            ["tokenizer_id", "init_latency_ms"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.interpretation.tokenizer_init_latency_ms",
            ["value", "tier"].iter().copied().collect(),
        );
        by_target.insert(
            "model.current_profile.request",
            ["profile", "tier", "load_status"].iter().copied().collect(),
        );

        // P-072 — Investigate-action (investigate.run_action) aggregate-only
        // targets. Bounded `action_id` (one of 4 ids OR "unknown") + `status`
        // enum + numeric `duration_ms`; NO prompt / analysis result / telemetry
        // content per obs §5 + the 2026-05-17 session 84 aggregate-only mandate.
        by_target.insert(
            "investigate.run_action.request",
            [
                "action_id",
                "status",
                "duration_ms",
                "deterministic_mode",
                "model_tier",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "metric.investigate.run_action.duration_ms",
            ["value", "duration_ms", "action_id", "status", "model_tier"]
                .iter()
                .copied()
                .collect(),
        );

        // Chunk #83 — L4 LLM interpretation pipeline tracing targets.
        // Aggregate-only fields per CLAUDE.md observability Session
        // Learnings 2026-05-17 session 84 mandate (no per-service
        // identifiers + no raw prompt body / digest payload / model
        // output bytes). Mirrors chunk #82 `interpretation` entry shape.
        by_target.insert(
            "interpretation.inference.request",
            [
                "model_tier",
                "hardware_profile",
                "prompt_version",
                "schema_version",
                "duration_ms",
                "result",
                "token_count_prompt",
                "token_count_output",
                "digest_kind",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "interpretation.prompt.assemble",
            ["token_count", "prompt_version", "duration_ms"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "interpretation.constrained.generate",
            ["model_tier", "hardware_profile", "duration_ms", "success"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "interpretation.json.parse",
            ["parse_outcome", "duration_ms", "output_bytes", "max_bytes"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "interpretation.inference.error",
            [
                "model_tier",
                "hardware_profile",
                "error_category",
                "recovery_action",
                "skipped_events",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "metric.pipeline.l4.inferences_total",
            ["value", "result", "model_tier", "hardware_profile"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.pipeline.l4.inference_latency_p99_milliseconds",
            ["value", "duration_ms", "model_tier", "hardware_profile"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.pipeline.l4.inference_queue_depth",
            ["value", "queued_count"].iter().copied().collect(),
        );

        // Chunk #86 — L4 degraded-mode FSM + manual-retry surface +
        // resolution-summary persist error tracing targets. Aggregate-only
        // fields per chunk #86 obs constraint + CLAUDE.md observability
        // 2026-05-17 session 84 AGGREGATE-ONLY mandate (no per-incident /
        // per-service / per-trace labels — degraded-mode FSM is GLOBAL).
        by_target.insert(
            "interpretation.degraded.enter",
            ["consecutive_failures", "window_seconds", "backoff_seconds"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "interpretation.degraded.exit",
            ["consecutive_successes", "duration_seconds"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "interpretation.inference.skipped",
            ["reason", "model_tier", "backoff_seconds_remaining"]
                .iter()
                .copied()
                .collect(),
        );
        // Generation-damper state transitions (idle-observer damper).
        // ONCE per engage/release, never per decision; bounded labels +
        // one aggregate numeric. Exact leaf — the bare `interpretation`
        // key must never serve this target.
        by_target.insert(
            "interpretation.generation.damper",
            ["decision", "reason", "digest_kind", "suppressed_run_len"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "interpretation.resolution_summary.persist.error",
            ["error_category"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.pipeline.l4.degraded_mode_active_seconds_total",
            ["value"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.pipeline.l4.degraded_mode_entries_total",
            ["value"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.pipeline.l4.backoff_remaining_seconds",
            [
                "value",
                "generations_suppressed_total",
                "generations_run_total",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "interpretation.incident.created",
            ["created", "deduped", "severity", "priority_tier"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "interpretation.incident.persist.error",
            ["error_category"].iter().copied().collect(),
        );
        by_target.insert(
            "metric.pipeline.l4.incidents_created_total",
            ["value", "result"].iter().copied().collect(),
        );
        by_target.insert(
            "diagnostics.retry_interpretation.request",
            [
                "triggered",
                "current_state",
                "consecutive_failures",
                "backoff_remaining_seconds",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );

        // Chunk #87 — Findings counter + dropdown "Mark all as read"
        // bulk action. Aggregate-only fields per CLAUDE.md observability
        // 2026-05-17 session 84 mandate (no per-incident-id / per-service
        // / per-trace labels — bulk action is workspace-scoped at the
        // resolver, not at the event field set). Capabilities P-028 /
        // P-029 / P-030.
        by_target.insert(
            "incidents.mark_all_read.request",
            ["affected_count", "outcome", "duration_ms", "traceparent"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "incident.broadcast.bulk_acknowledged",
            ["affected_count"].iter().copied().collect(),
        );

        // Chunk #88 — Diagnostic Report generation. Aggregate-only fields
        // per CLAUDE.md observability 2026-05-17 session 84 mandate +
        // 2026-05-11 chunk #44 snapshot/clipboard hygiene (NEVER log
        // markdown body / hypotheses_text / evidence_excerpts / symptom_text).
        // Capabilities P-031 + P-035–P-038.
        by_target.insert(
            "incidents.get_report.request",
            [
                "item_id",
                "outcome",
                "section_count",
                "markdown_size_bytes",
                "degraded_mode",
                "previously_seen_corpus_match_count",
                "duration_ms",
                "traceparent",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "report.render.markdown",
            [
                "incident_id",
                "report_section_count",
                "markdown_size_bytes",
                "degraded_mode",
                "previously_seen_corpus_match_count",
                "duration_ms",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "report.degraded_mode_notice",
            ["incident_id", "reason_category"].iter().copied().collect(),
        );
        // Chunk #100 — P-036 previously-seen retrieval failure path.
        // error_category only; never candidate content / workspace strings.
        by_target.insert(
            "report.previously_seen.retrieval_error",
            ["error_category"].iter().copied().collect(),
        );
        // Chunk #100 — P-041 boot-time pipeline-metrics retention purge.
        // Aggregate-only counts + bounded window field; exact-match key per
        // the 2026-05-07 resolver discipline (fallback would resolve to a
        // generic `corpus` key).
        by_target.insert(
            "corpus.pipeline_metrics.purge",
            [
                "purged_row_count",
                "retention_window_days",
                "duration_ms",
                "error_category",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // Corpus key custody + orphan disposition. Every leaf below is an
        // EXACT key: `for_target`'s prefix fallback would look up a bare
        // `corpus` entry, which deliberately does not exist — a prefix key
        // would silently widen every future `corpus.*` target to one field
        // set (the 2026-05-07 resolver discipline).
        //
        // `corpus.open.error` is a REPAIR, not a new target: it has emitted
        // from the boot path since chunk #68 with no resolvable entry, so its
        // `error_kind` was redacted — the one diagnostic that would have named
        // the per-process-ephemeral-key defect was itself muted.
        by_target.insert(
            "corpus.open.error",
            ["error_kind"].iter().copied().collect(),
        );
        by_target.insert(
            "corpus.keychain.fallback",
            ["backend_kind", "reason", "consequence"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "corpus.read.undecryptable",
            ["query_id", "rows_skipped"].iter().copied().collect(),
        );
        by_target.insert(
            "corpus.orphan.disposition",
            [
                "disposition_outcome",
                "rows_purged",
                "tables_affected",
                "error_category",
            ]
            .iter()
            .copied()
            .collect(),
        );
        // Incident-path diagnostics. Each leaf is an EXACT key for the same
        // reason the `corpus.*` leaves above are: `for_target`'s prefix
        // fallback would resolve these dotted targets to a bare `incidents` /
        // `triage` entry, and no such prefix key exists (nor may one be added
        // — it would silently widen every sibling to one field set).
        //
        // All three emitted with their fields redacted, so the diagnostics
        // that would have named the incident path's state were themselves
        // mute: the storm→incident classification had to read `corpus.db`
        // out-of-band instead. Aggregate counts + `&'static str` labels only
        // (`persist_kind` is typed `&'static str` at the emit site, so its
        // domain is bounded by construction) — never incident identity,
        // never workspace strings.
        by_target.insert(
            "incidents.list_active.request",
            ["item_count"].iter().copied().collect(),
        );
        by_target.insert(
            "triage.incident.persist",
            [
                "incident_count",
                "persist_kind",
                "duration_ms",
                "declined_count",
                "reconciled_count",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "triage.incident.corpus_restore",
            ["kind", "restored_incident_count"]
                .iter()
                .copied()
                .collect(),
        );
        // Exact leaf (no bare `triage` key exists, and the `.tick`-strip
        // resolves to nothing): without it all three fields render
        // "<redacted>" — the measured 300s-vacuous-wait false-negative
        // source (a smoke keyed on `resolved_count` could never match).
        by_target.insert(
            "triage.incident.auto_resolve.tick",
            ["evaluated_count", "resolved_count", "duration_ms"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "metric.report.render_ms",
            ["value", "section_count", "degraded_mode"]
                .iter()
                .copied()
                .collect(),
        );

        Self { by_target }
    }

    #[doc(hidden)]
    pub fn for_target(&self, target: &str) -> Option<&HashSet<&'static str>> {
        if let Some(set) = self.by_target.get(target) {
            return Some(set);
        }
        if let Some(stripped) = target.strip_suffix(".tick") {
            if let Some(set) = self.by_target.get(stripped) {
                return Some(set);
            }
        }
        if let Some(prefix) = target.split('.').next() {
            if let Some(set) = self.by_target.get(prefix) {
                return Some(set);
            }
        }
        if let Some(prefix) = target.split("::").next() {
            if let Some(set) = self.by_target.get(prefix) {
                return Some(set);
            }
        }
        None
    }
}

// Custom JSON formatter that:
//   1. Injects DefaultFields (service.name / service.version / deployment.environment
//      / ci.run.id / git.commit.sha) onto every event's `fields` map per obs-plan §3.
//   2. Applies per-target allowlist scrubbing per obs-plan §8 default-deny posture.
//
// Why the scrubber lives here rather than as a separate Layer<S>: tracing-subscriber
// 0.3 Layer is a read-only side-effect callback; it cannot mutate Event fields for
// downstream layers. Since field redaction must occur before serialization to file,
// the scrubber is integrated into the formatter's field visitor. The registry
// composition `EnvFilter -> json_layer (with scrubbing) -> ErrorLayer::default()`
// achieves the obs-plan §8 contract of subscriber-layer redaction.
#[doc(hidden)]
pub struct JsonWithDefaults {
    pub defaults: DefaultFields,
    pub allowlist: AllowList,
}

impl<S, N> FormatEvent<S, N> for JsonWithDefaults
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();
        let target = metadata.target();

        let mut top: Map<String, Value> = Map::new();
        top.insert(
            "timestamp".to_string(),
            Value::String(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)),
        );
        top.insert(
            "level".to_string(),
            Value::String(metadata.level().to_string()),
        );
        top.insert("target".to_string(), Value::String(target.to_string()));

        let mut fields_map: Map<String, Value> = Map::new();
        fields_map.insert(
            "service.name".into(),
            Value::String(self.defaults.service_name.into()),
        );
        fields_map.insert(
            "service.version".into(),
            Value::String(self.defaults.service_version.into()),
        );
        fields_map.insert(
            "deployment.environment".into(),
            Value::String(self.defaults.deployment_environment.clone()),
        );
        if let Some(id) = &self.defaults.ci_run_id {
            fields_map.insert("ci.run.id".into(), Value::String(id.clone()));
        }
        if let Some(sha) = &self.defaults.git_commit_sha {
            fields_map.insert("git.commit.sha".into(), Value::String(sha.clone()));
        }

        let allowlist_for_target = self.allowlist.for_target(target);
        let mut visitor = JsonFieldVisitor {
            fields: &mut fields_map,
            message: None,
            allowlist: allowlist_for_target,
        };
        event.record(&mut visitor);

        top.insert(
            "message".to_string(),
            Value::String(visitor.message.unwrap_or_default()),
        );
        top.insert("fields".to_string(), Value::Object(fields_map));

        let line = serde_json::to_string(&Value::Object(top)).map_err(|_| std::fmt::Error)?;
        writeln!(writer, "{line}")
    }
}

struct JsonFieldVisitor<'a> {
    fields: &'a mut Map<String, Value>,
    message: Option<String>,
    allowlist: Option<&'a HashSet<&'static str>>,
}

impl JsonFieldVisitor<'_> {
    fn store(&mut self, name: &str, value: Value) {
        if name == "message" {
            self.message = Some(match value {
                Value::String(s) => s,
                other => other.to_string(),
            });
            return;
        }
        let stored = match self.allowlist {
            Some(allowed) if !allowed.contains(name) => Value::String("<redacted>".into()),
            None => Value::String("<redacted>".into()),
            _ => value,
        };
        self.fields.insert(name.to_string(), stored);
    }
}

impl Visit for JsonFieldVisitor<'_> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.store(field.name(), Value::String(value.to_string()));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.store(field.name(), Value::Number(value.into()));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.store(field.name(), Value::Number(value.into()));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.store(field.name(), Value::Bool(value));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        match serde_json::Number::from_f64(value) {
            Some(n) => self.store(field.name(), Value::Number(n)),
            None => self.store(field.name(), Value::String(value.to_string())),
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.store(field.name(), Value::String(format!("{value:?}")));
    }
}

pub(crate) fn install_panic_hook() {
    // Chunk #73 P-003: chain over any previously-installed hook (e.g.,
    // tracing-error SpanTrace bridge, Tauri's hook). Capture once at install
    // time; invoke the prior hook after our own work so SpanTrace + Tauri
    // panic handling still fire.
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Signal panic for the connection FSM BEFORE emitting tracing event
        // (so a panic during the tracing emission still leaves the atomic set).
        PANIC_SIGNAL.store(true, std::sync::atomic::Ordering::Relaxed);

        // Per security plan §Anti-Patterns Logging row 1 + chunk #73 P-003
        // task brief ("panic event uses sanitized payload"): drop the
        // panic message entirely from the tracing event. Both `PanicInfo`
        // Display + raw `info.payload()` downcast expose the literal
        // `panic!()` argument verbatim — instrumented host apps may pass
        // user secrets through panic payloads, so the only safe surface
        // is location + SpanTrace (user-defined span hierarchy only, no
        // raw payload bytes).
        let location = info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown".to_string());
        tracing::error!(
            target: "app.panic.fatal",
            location = %location,
            spantrace = ?SpanTrace::capture(),
            "panic captured",
        );

        // Preserve prior-hook chaining (tracing-error SpanTrace bridge etc.).
        prev(info);
    }));
}

pub fn init(data_dir: &Path) -> WorkerGuard {
    let logs_dir = data_dir.join("logs");
    fs::create_dir_all(&logs_dir).expect("failed to create logs dir");
    set_logs_dir_permissions(&logs_dir);

    let file_appender = tracing_appender::rolling::daily(&logs_dir, "agent-latest.jsonl");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_env("ANDROMEDA_PULSE_LOG_LEVEL")
        .or_else(|_| EnvFilter::try_from_default_env())
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let DefaultFieldsResult { fields, warnings } = DefaultFields::from_env();

    let json_layer = fmt::layer()
        .event_format(JsonWithDefaults {
            defaults: fields,
            allowlist: AllowList::production(),
        })
        .with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(json_layer)
        .with(ErrorLayer::default())
        .init();

    install_panic_hook();

    for warning in &warnings {
        tracing::warn!(
            target: "app.boot.ci-fields",
            reason = %warning,
            "CI env field validation rejected raw env value",
        );
    }

    tracing::info!(
        target: "app.boot.tracing.init",
        log_dir_basename = log_basename(&logs_dir).unwrap_or("unknown"),
        default_fields_active = true,
        "tracing subscriber initialized",
    );

    guard
}

fn set_logs_dir_permissions(_logs_dir: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(_logs_dir) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o700);
            let _ = fs::set_permissions(_logs_dir, perms);
        }
    }
    // Windows: %APPDATA% inherits a user-restricted ACL by default.
    // TODO(security): explicit ACL hardening pending future security-scope-arch.
}

// Per security plan §Security Anti-Patterns Logging
// "basename of canonicalized path only".
#[doc(hidden)]
pub fn log_basename(path: &Path) -> Option<&str> {
    path.file_name().and_then(|s| s.to_str())
}

// ─────────────────────────────────────────────────────────
// Sentry before_send scrubbing (deferred — feature gate not yet wired)
//
// Per arch §Conventions "Feature flags" + obs-plan §7 Default state, Sentry
// is opt-in and currently NOT a workspace dependency. When the `sentry`
// feature is added, the before_send callback MUST run the same per-module
// allowlist as JsonWithDefaults above, plus scrub OTLP payloads / file paths
// beyond project root / Rust struct names / library versions / plugin
// basenames / DuckDB query_id+param_count only / explicit MCP response
// body skip. See obs-plan §7 + security plan §Error Handling
// "Error reporting integration".
//
// Activation checklist:
//   1. Add `sentry` + `sentry-tauri` to workspace [workspace.dependencies]
//   2. Add `sentry-redaction` Cargo feature in pulse-app/Cargo.toml
//   3. Replace this comment block with cfg-gated `pub(crate) fn before_send(...)`
//   4. Wire into Sentry init at chunk where Sentry is first activated
// ─────────────────────────────────────────────────────────

// The former src-level `mod tests` (130 fns) migrated to
// `pulse-app/tests/observability_pins.rs` 2026-08-30 — `[lib] test = false`
// meant it compiled under clippy and never ran (obs-plan section 8 rule:
// allowlist guards live under `pulse-app/tests/`, where they execute).
