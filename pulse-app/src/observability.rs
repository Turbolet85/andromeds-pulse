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

const SERVICE_NAME: &str = "com.andromeda.pulse";

/// Chunk #73 P-003: shared atomic signaled by the panic hook so the
/// connection FSM (`crates/ingest::connection::ReceiverBindStatus`) can
/// route to the `ReceiverPanicked` reason variant. Read via the
/// `panic_signaled()` accessor below; written exactly once by
/// `install_panic_hook`'s closure. Module-level static rather than Arc-
/// injected к minimize boot-time wiring surface (one accessor pulse-app-
/// wide), per the security extract's "narrowest exposure" guidance.
static PANIC_SIGNAL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Returns true if a panic has been signaled by `install_panic_hook` since
/// process start. Used by `pulse-app::connection_router::HeartbeatBindStatus`
/// to expose panic-state to the FSM поллер.
pub fn panic_signaled() -> bool {
    PANIC_SIGNAL.load(std::sync::atomic::Ordering::Relaxed)
}

/// Test-only reset for the panic atomic. `#[doc(hidden)] pub` per
/// testing.md 2026-05-20 session 107 integration-test-access pattern
/// (signals "not external API but accessible for integration tests");
/// per-process state means tests must reset between panics к avoid
/// cross-test contamination.
#[doc(hidden)]
pub fn reset_panic_signal_for_tests() {
    PANIC_SIGNAL.store(false, std::sync::atomic::Ordering::Relaxed);
}

#[derive(Clone)]
pub(crate) struct DefaultFields {
    service_name: &'static str,
    service_version: &'static str,
    deployment_environment: String,
    ci_run_id: Option<String>,
    git_commit_sha: Option<String>,
}

pub(crate) struct DefaultFieldsResult {
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

fn validate_ci_run_id(s: &str) -> bool {
    !s.is_empty() && s.len() <= 32 && s.chars().all(|c| c.is_ascii_digit())
}

fn validate_git_commit_sha(s: &str) -> bool {
    s.len() >= 7
        && s.len() <= 64
        && s.chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

fn resolve_deployment_environment() -> String {
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
struct AllowList {
    by_target: HashMap<&'static str, HashSet<&'static str>>,
}

impl AllowList {
    fn production() -> Self {
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
            ["log_dir", "default_fields_active"]
                .iter()
                .copied()
                .collect(),
        );
        by_target.insert(
            "app.boot.pid",
            ["pid", "path", "error", "run_dir", "data_dir"]
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
            ["gpu_available", "wgpu_backend"].iter().copied().collect(),
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
        by_target.insert("app.boot.tray.init", ["tray_api"].iter().copied().collect());
        by_target.insert(
            "app.boot.window.show",
            ["label", "error_kind", "error_msg"]
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

        // Chunk #62 — attention cue emitter (Epoch 9 Foundation v0.2.0
        // sixth chunk; capability P-021 + P-019 partial). 4 new tracing
        // target leaves emitted by `crates/triage/src/cue/emitter.rs`.
        // PII discipline (per security plan §Anti-Patterns § Logging row 1):
        // no `scope_id` admitted (carries user-controlled service.name);
        // only bounded-cardinality enum tags + structural numeric values.
        by_target.insert(
            "triage.cue.tick",
            [
                "cues_evaluated",
                "cues_emitted",
                "cadence_triggers_emitted",
                "services_tracked",
                "operations_tracked",
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
                "persistence_seconds",
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
                "persistence_seconds",
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
            [
                "mode",
                "cue_kind",
                "cue_priority",
                "error_category",
                "skipped_events",
            ]
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
        // never а file path or URL.
        by_target.insert(
            "interpretation",
            [
                "model_profile",
                "model_tier",
                "model_ready",
                "model_status",
                "override_source",
            ]
            .iter()
            .copied()
            .collect(),
        );
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
            ["model_identity", "tier", "file_size_bytes", "load_status"]
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
            ]
            .iter()
            .copied()
            .collect(),
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

        Self { by_target }
    }

    fn for_target(&self, target: &str) -> Option<&HashSet<&'static str>> {
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
struct JsonWithDefaults {
    defaults: DefaultFields,
    allowlist: AllowList,
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
        // Signal panic для the connection FSM BEFORE emitting tracing event
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
        log_dir = ?logs_dir,
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

// Intentionally unused in production until plugin-path emitters land
// (route#37+ plugin loader). Available now so any future emitter calls it
// uniformly per security plan §Security Anti-Patterns Logging
// "basename of canonicalized path only".
#[allow(dead_code)]
pub(crate) fn log_basename(path: &Path) -> Option<&str> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::Registry;
    use tracing_subscriber::layer::SubscriberExt;

    #[derive(Clone)]
    struct VecMakeWriter(Arc<Mutex<Vec<u8>>>);

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for VecMakeWriter {
        type Writer = VecWriter;
        fn make_writer(&'a self) -> Self::Writer {
            VecWriter(Arc::clone(&self.0))
        }
    }

    struct VecWriter(Arc<Mutex<Vec<u8>>>);

    impl Write for VecWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn make_defaults(ci_run_id: Option<&str>, git_commit_sha: Option<&str>) -> DefaultFields {
        DefaultFields {
            service_name: SERVICE_NAME,
            service_version: env!("CARGO_PKG_VERSION"),
            deployment_environment: "production".into(),
            ci_run_id: ci_run_id.map(String::from),
            git_commit_sha: git_commit_sha.map(String::from),
        }
    }

    fn capture_json_lines<F: FnOnce()>(defaults: DefaultFields, f: F) -> Vec<Value> {
        let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
        let writer = VecMakeWriter(Arc::clone(&buf));
        let layer = fmt::layer()
            .event_format(JsonWithDefaults {
                defaults,
                allowlist: AllowList::production(),
            })
            .with_writer(writer);
        let subscriber = Registry::default().with(layer);
        tracing::subscriber::with_default(subscriber, f);
        let bytes = buf.lock().unwrap().clone();
        let s = String::from_utf8(bytes).expect("captured bytes should be valid UTF-8");
        s.lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str::<Value>(l).expect("each line should parse as JSON"))
            .collect()
    }

    #[test]
    fn default_fields_present_when_ci_env_set() {
        let defaults = make_defaults(Some("123456"), Some("abc1234"));
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "ingest", span_count = 5_i64, "test event");
        });
        assert_eq!(lines.len(), 1);
        let line = &lines[0];
        assert_eq!(line["fields"]["service.name"], "com.andromeda.pulse");
        assert_eq!(line["fields"]["service.version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(line["fields"]["deployment.environment"], "production");
        assert_eq!(line["fields"]["ci.run.id"], "123456");
        assert_eq!(line["fields"]["git.commit.sha"], "abc1234");
    }

    #[test]
    fn default_fields_absent_when_ci_env_unset() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "ingest", span_count = 5_i64, "test event");
        });
        let line = &lines[0];
        assert!(line["fields"].get("ci.run.id").is_none());
        assert!(line["fields"].get("git.commit.sha").is_none());
        assert_eq!(line["fields"]["service.name"], "com.andromeda.pulse");
    }

    #[test]
    fn validate_ci_run_id_accepts_digits() {
        assert!(validate_ci_run_id("1"));
        assert!(validate_ci_run_id("123456"));
        assert!(validate_ci_run_id("99999999999999999999999999999999"));
    }

    #[test]
    fn validate_ci_run_id_rejects_garbage() {
        assert!(!validate_ci_run_id(""));
        assert!(!validate_ci_run_id("../../etc/passwd"));
        assert!(!validate_ci_run_id("hello"));
        assert!(!validate_ci_run_id("123abc"));
        assert!(!validate_ci_run_id("123456789012345678901234567890123"));
    }

    #[test]
    fn validate_git_commit_sha_accepts_lowercase_hex() {
        assert!(validate_git_commit_sha("abc1234"));
        assert!(validate_git_commit_sha(
            "0123456789abcdef0123456789abcdef01234567"
        ));
        assert!(validate_git_commit_sha(&"a".repeat(40)));
        assert!(validate_git_commit_sha(&"f".repeat(64)));
    }

    #[test]
    fn validate_git_commit_sha_rejects_garbage() {
        assert!(!validate_git_commit_sha(""));
        assert!(!validate_git_commit_sha("ABC1234"));
        assert!(!validate_git_commit_sha("ghijkl1"));
        assert!(!validate_git_commit_sha("ab"));
        assert!(!validate_git_commit_sha(&"a".repeat(65)));
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "ingest", attribute_value = "secret123", "test");
        });
        let line = &lines[0];
        let json_str = serde_json::to_string(line).unwrap();
        assert!(
            !json_str.contains("secret123"),
            "scrubbed value should not appear in JSON: {json_str}"
        );
        assert_eq!(line["fields"]["attribute_value"], "<redacted>");
    }

    #[test]
    fn scrubber_passes_allowlisted_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "ingest", span_count = 42_i64, "ingest event");
        });
        assert_eq!(lines[0]["fields"]["span_count"], 42);
    }

    #[test]
    fn scrubber_passes_a11y_diagnostic_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "a11y::assertion",
                wcag_criterion = "SC 1.4.3",
                selector = ".settings-modal form input[type='text']",
                remediation = "Increase foreground darkness; use --color-text-primary token",
                "a11y violation",
            );
        });
        let line = &lines[0];
        assert_eq!(line["fields"]["wcag_criterion"], "SC 1.4.3");
        assert_eq!(
            line["fields"]["selector"],
            ".settings-modal form input[type='text']"
        );
        assert!(
            line["fields"]["remediation"]
                .as_str()
                .unwrap()
                .contains("--color-text-primary")
        );
    }

    #[test]
    fn scrubber_passes_metric_prefix_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "metric.ingest.throughput", value = 1234_i64, "throughput");
        });
        assert_eq!(lines[0]["fields"]["value"], 1234);
    }

    #[test]
    fn scrubber_passes_tick_suffix_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "ingest.tick", span_count = 7_i64, "heartbeat");
        });
        assert_eq!(lines[0]["fields"]["span_count"], 7);
    }

    #[test]
    fn scrubber_passes_ingest_tick_obs_plan_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "ingest.tick",
                span_count = 12_u64,
                buffer_capacity_pct = 0.42_f64,
                broadcast_subscribers = 3_u64,
                "heartbeat",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["span_count"], 12);
        assert_eq!(fields["buffer_capacity_pct"], 0.42);
        assert_eq!(fields["broadcast_subscribers"], 3);
    }

    #[test]
    fn scrubber_passes_buffer_tick_obs_plan_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "buffer.tick",
                rows_ingested = 1024_u64,
                retention_window_active = true,
                eviction_count = 7_u64,
                "heartbeat",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["rows_ingested"], 1024);
        assert_eq!(fields["retention_window_active"], true);
        assert_eq!(fields["eviction_count"], 7);
    }

    #[test]
    fn scrubber_passes_viz_tick_obs_plan_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "viz.tick",
                query_latency_ms = 18.5_f64,
                subscribers_active = 2_u64,
                "heartbeat",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["query_latency_ms"], 18.5);
        assert_eq!(fields["subscribers_active"], 2);
    }

    #[test]
    fn scrubber_passes_plugins_tick_obs_plan_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "plugins.tick",
                loaded_count = 4_u64,
                active_invocations = 1_u64,
                "heartbeat",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["loaded_count"], 4);
        assert_eq!(fields["active_invocations"], 1);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_field_on_each_tick_target() {
        let defaults = make_defaults(None, None);
        for target in ["ingest.tick", "buffer.tick", "viz.tick", "plugins.tick"] {
            let lines = capture_json_lines(defaults.clone(), || match target {
                "ingest.tick" => {
                    tracing::info!(target: "ingest.tick", attribute_value = "secret-leak", "heartbeat");
                }
                "buffer.tick" => {
                    tracing::info!(target: "buffer.tick", query_param = "DROP TABLE", "heartbeat");
                }
                "viz.tick" => {
                    tracing::info!(target: "viz.tick", attribute_value = "leaks", "heartbeat");
                }
                "plugins.tick" => {
                    tracing::info!(target: "plugins.tick", plugin_path = "/secret/path.wasm", "heartbeat");
                }
                _ => unreachable!(),
            });
            assert_eq!(lines.len(), 1, "expected one line for {target}");
            let fields = &lines[0]["fields"];
            for (k, v) in fields.as_object().unwrap() {
                if matches!(
                    k.as_str(),
                    "service.name"
                        | "service.version"
                        | "deployment.environment"
                        | "ci.run.id"
                        | "git.commit.sha"
                ) {
                    continue;
                }
                assert_eq!(
                    v, "<redacted>",
                    "field {k} should be redacted for target {target}"
                );
            }
        }
    }

    #[test]
    fn scrubber_redacts_unknown_target() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "unknown_module", arbitrary = "leaks", "test");
        });
        assert_eq!(lines[0]["fields"]["arbitrary"], "<redacted>");
    }

    #[test]
    fn json_schema_conformance() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "ingest", span_count = 1_i64, "first");
            tracing::warn!(target: "ingest", span_count = 2_i64, "second");
            tracing::error!(target: "ingest", span_count = 3_i64, "third");
        });
        assert_eq!(lines.len(), 3);
        for line in &lines {
            assert!(line.get("timestamp").is_some(), "missing timestamp");
            assert!(line.get("level").is_some(), "missing level");
            assert!(line.get("target").is_some(), "missing target");
            assert!(line.get("message").is_some(), "missing message");
            assert!(line.get("fields").is_some(), "missing fields");
            assert!(line["fields"].get("service.name").is_some());
            assert!(line["fields"].get("service.version").is_some());
            assert!(line["fields"].get("deployment.environment").is_some());
        }
    }

    #[test]
    fn timestamp_is_rfc3339_with_millis() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(target: "ingest", span_count = 1_i64, "ping");
        });
        let ts = lines[0]["timestamp"].as_str().unwrap();
        assert!(
            chrono::DateTime::parse_from_rfc3339(ts).is_ok(),
            "timestamp should parse as RFC 3339: {ts}"
        );
    }

    #[test]
    fn log_basename_returns_filename_only() {
        assert_eq!(
            log_basename(Path::new(
                "/home/user/.andromeda-pulse/plugins/my-plugin.wasm"
            )),
            Some("my-plugin.wasm")
        );
        assert_eq!(
            log_basename(Path::new(
                "C:/Users/alice/AppData/andromeda-pulse/plugins/x.wasm"
            )),
            Some("x.wasm")
        );
        assert_eq!(log_basename(Path::new("plain.wasm")), Some("plain.wasm"));
    }

    #[test]
    fn allowlist_for_target_handles_prefixes_and_ticks() {
        let al = AllowList::production();
        assert!(al.for_target("ingest").is_some(), "exact match");
        assert!(al.for_target("ingest.tick").is_some(), "tick suffix");
        assert!(al.for_target("ingest.grpc.export").is_some(), "dot prefix");
        assert!(
            al.for_target("ingest::grpc::request").is_some(),
            "colon prefix"
        );
        assert!(
            al.for_target("a11y::assertion").is_some(),
            "exact a11y target"
        );
        assert!(
            al.for_target("metric.snapshot.token_count_ms").is_some(),
            "metric.* prefix"
        );
        assert!(
            al.for_target("unknown.module.target").is_none(),
            "unknown target redacts everything"
        );
    }

    #[test]
    fn allowlist_for_target_resolves_ui_bridge_error_namespace() {
        // chunk #26: From<E> for AppError impls in crates/ui-bridge/src/contract.rs
        // emit tracing::warn! at target "ui-bridge.error.{variant}". Each variant
        // MUST resolve to its own per-variant entry with `error_category`,
        // `source_kind`, `source_crate` allowed (plus per-variant structured
        // handle: `field` for validation, `plugin_id` for plugin, `resource`
        // for not_found). Without these, default-deny redacts the conversion
        // event payload and the obs-plan §10 module-boundary error logging
        // contract is silently violated.
        let al = AllowList::production();
        for variant in [
            "ui-bridge.error.validation",
            "ui-bridge.error.not_found",
            "ui-bridge.error.internal",
            "ui-bridge.error.plugin",
            "ui-bridge.error.storage",
            "ui-bridge.error.ingest",
        ] {
            let set = al
                .for_target(variant)
                .unwrap_or_else(|| panic!("expected per-variant entry for {variant}"));
            for required in ["error_category", "source_kind", "source_crate"] {
                assert!(set.contains(required), "{variant} must permit `{required}`",);
            }
        }
        let validation = al
            .for_target("ui-bridge.error.validation")
            .expect("validation entry");
        assert!(
            validation.contains("field"),
            "ui-bridge.error.validation must permit `field` for aria-describedby binding"
        );
        let plugin = al
            .for_target("ui-bridge.error.plugin")
            .expect("plugin entry");
        assert!(
            plugin.contains("plugin_id"),
            "ui-bridge.error.plugin must permit `plugin_id`"
        );
        let not_found = al
            .for_target("ui-bridge.error.not_found")
            .expect("not_found entry");
        assert!(
            not_found.contains("resource"),
            "ui-bridge.error.not_found must permit `resource`"
        );
    }

    #[test]
    fn allowlist_for_target_resolves_introspection_namespace() {
        // chunk #27: IntrospectionApi resolvers in crates/ui-bridge/src/health.rs
        // emit tracing::info! at target "ui-bridge.{procedure}". Each procedure
        // MUST resolve to its own per-procedure entry containing `method_name`
        // and `result_type` (the boundary-call wrapper field set per obs-plan
        // §6) plus per-procedure structured handles. Without exact-match
        // entries, the resolver's split('.').next() fallback collapses these
        // to the bare `ui-bridge` entry whose field set differs.
        let al = AllowList::production();
        for procedure in [
            "ui-bridge.app_info",
            "ui-bridge.health",
            "ui-bridge.ready",
            "ui-bridge.get_settings",
            "ui-bridge.update_settings",
        ] {
            let set = al
                .for_target(procedure)
                .unwrap_or_else(|| panic!("expected per-procedure entry for {procedure}"));
            for required in ["method_name", "result_type"] {
                assert!(
                    set.contains(required),
                    "{procedure} must permit `{required}`",
                );
            }
        }
        let app_info = al.for_target("ui-bridge.app_info").expect("app_info entry");
        for required in [
            "name",
            "version",
            "rust_version",
            "tauri_version",
            "build_profile",
        ] {
            assert!(
                app_info.contains(required),
                "ui-bridge.app_info must permit `{required}`"
            );
        }
        let ready = al.for_target("ui-bridge.ready").expect("ready entry");
        for required in [
            "ready",
            "duckdb_connection",
            "ingest_mpsc_capacity_pct",
            "broadcast_subscribers",
            "plugins_loaded",
            "mcp_server_enabled",
        ] {
            assert!(
                ready.contains(required),
                "ui-bridge.ready must permit `{required}`"
            );
        }
        let update_settings = al
            .for_target("ui-bridge.update_settings")
            .expect("update_settings entry");
        assert!(
            update_settings.contains("setting_keys_changed"),
            "ui-bridge.update_settings must permit `setting_keys_changed`"
        );
        let drift = al
            .for_target("xtask.capability_drift")
            .expect("xtask capability_drift entry");
        for required in [
            "missing_count",
            "extra_count",
            "drift_state",
            "top_5_drifted_names",
        ] {
            assert!(
                drift.contains(required),
                "xtask.capability_drift must permit `{required}`"
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_duckdb_append_to_arrow_appender_fields() {
        // chunk #65: production consumer::dispatch_batch emits one
        // tracing::info!(target: "duckdb.append", ...) per table append
        // (spans / span_events / metrics_points / log_records) with the
        // rows_appended + duration_ms + table_name field shape. Error path
        // (target: "duckdb.append", reject_reason = ...) is also covered.
        // Without this exact entry, the resolver's split('.').next() fallback
        // returns None ("duckdb" key absent) and JsonFieldVisitor silently
        // redacts all fields per the default-deny posture.
        let al = AllowList::production();
        let entry = al
            .for_target("duckdb.append")
            .expect("expected duckdb entry resolving via split('.').next() from duckdb.append");
        for required in [
            "rows_appended",
            "duration_ms",
            "table_name",
            "reject_reason",
        ] {
            assert!(
                entry.contains(required),
                "duckdb.append must permit `{required}`"
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_metric_webgpu_frame_duration_ms() {
        // chunk #28: WebGPU canvas substrate emits per-frame timing via
        // TauRPC telemetry.frontend.record_frame_ms (resolver lands chunk #29);
        // the metric event itself rides at target "metric.webgpu.frame_duration_ms"
        // with bounded enum labels per obs-plan §5 cardinality discipline.
        let al = AllowList::production();
        let frame = al
            .for_target("metric.webgpu.frame_duration_ms")
            .expect("metric.webgpu.frame_duration_ms entry");
        for required in [
            "duration_ms",
            "wgpu_backend",
            "webview_backend",
            "timing_method",
        ] {
            assert!(
                frame.contains(required),
                "metric.webgpu.frame_duration_ms must permit `{required}`"
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_ui_layout_transition_to_expanded_field_set() {
        // chunk #30: apply_widget_settings extends the existing chunk #24
        // ui.layout.transition allowlist with always_on_top + duration_ms
        // beyond the original layout_mode_from / layout_mode_to / tray_visible
        // triplet. Without these, default-deny redacts the new fields and
        // the apply-settings observability contract silently regresses.
        let al = AllowList::production();
        let entry = al
            .for_target("ui.layout.transition")
            .expect("ui.layout.transition entry");
        for required in [
            "layout_mode_from",
            "layout_mode_to",
            "tray_visible",
            "always_on_top",
            "duration_ms",
        ] {
            assert!(
                entry.contains(required),
                "ui.layout.transition must permit `{required}`"
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_snapshot_curate_field_set() {
        // chunk #39: curation primitives in `crates/snapshot/{dedupe,anomaly,
        // critical_path}.rs` instrument as `snapshot.curate.{operation}` and
        // emit per-pipeline counts (input_row_count / output_row_count /
        // dedup_count / latency_outlier_count / error_cluster_count /
        // cardinality_spike_count / critical_path_span_count / total_span_count
        // / orphan_parent_count / anomaly_markers_count / duration_ms). For_target
        // resolves these via split('.').next() fallback to the `snapshot` entry,
        // so the entry must permit ALL fields used across the curation pipeline
        // OR default-deny silently redacts them and obs-plan §3 P2 + §8
        // snapshot whitelist contract regresses.
        //
        // chunk #40: aggregation + attribute_filter primitives extend the
        // namespace with `snapshot.curate.aggregate` + `snapshot.curate.attribute_filter`
        // targets emitting metric_input_count / metric_output_count /
        // service_count / kept_attribute_count / dropped_attribute_count /
        // p50_ms / p95_ms / p99_ms / max_ms. Same fall-through resolution
        // path; same `snapshot` entry must permit them.
        let al = AllowList::production();
        for target in [
            "snapshot.curate.dedup",
            "snapshot.curate.anomaly_detect",
            "snapshot.curate.critical_path_extract",
            "snapshot.curate.full_pipeline",
            "snapshot.curate.aggregate",
            "snapshot.curate.attribute_filter",
        ] {
            let set = al
                .for_target(target)
                .unwrap_or_else(|| panic!("expected snapshot entry resolution for {target}"));
            for required in [
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
                "kept_attribute_count",
                "dropped_attribute_count",
                "p50_ms",
                "p95_ms",
                "p99_ms",
                "max_ms",
            ] {
                assert!(
                    set.contains(required),
                    "{target} must permit `{required}` (snapshot allowlist coverage)",
                );
            }
        }
    }

    #[test]
    fn allowlist_for_target_resolves_baseline_migrate_field_set() {
        // chunk #70: BaselineState → corpus migration emits one-time legacy
        // migration tracing events at `triage.baseline.migrate` (info) +
        // `triage.baseline.migrate.failed` (warn). Both targets require
        // explicit-leaf AllowList entries (no fall-through к the broader
        // `triage` entry, whose field set is unrelated). PII discipline
        // per .claude/rules/observability.md 2026-05-17 session 84 +
        // chunks #62/#63/#64 aggregate-only mandate: bounded enum +
        // integer-value fields ONLY; NO `service_name` / `scope_id` /
        // `span_id` / `trace_id` / `operation_name` / `legacy_path`
        // admitted (the latter excludes raw file paths per Vector 6).
        let al = AllowList::production();

        let migrate = al
            .for_target("triage.baseline.migrate")
            .expect("expected triage.baseline.migrate entry");
        for required in [
            "legacy_state_size_bytes",
            "migrated_service_count",
            "duration_ms",
            "migration_outcome",
        ] {
            assert!(
                migrate.contains(required),
                "triage.baseline.migrate must permit `{required}`",
            );
        }

        let failed = al
            .for_target("triage.baseline.migrate.failed")
            .expect("expected triage.baseline.migrate.failed entry");
        for required in ["error_category", "duration_ms"] {
            assert!(
                failed.contains(required),
                "triage.baseline.migrate.failed must permit `{required}`",
            );
        }

        // PII guard: neither entry may admit per-service or per-record
        // identifiers (aggregate-only convention).
        let banned = [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
            "legacy_path",
            "attribute",
            "body",
        ];
        for target_set in [migrate, failed] {
            for k in &banned {
                assert!(
                    !target_set.contains(k),
                    "baseline migrate AllowList must NOT permit PII field `{k}`",
                );
            }
        }
    }

    #[test]
    fn allowlist_for_target_resolves_lifecycle_persist_field_set() {
        // chunk #71: ServiceRegistry corpus persistence emits aggregate-only
        // events at `triage.lifecycle.persist` (info, per-tick success) +
        // `triage.lifecycle.persist.error` (warn, per-tick failure).
        // PII discipline mirrors chunk #70 baseline.persist precedent —
        // bounded enum tags + integer-value fields ONLY.
        let al = AllowList::production();

        let persist = al
            .for_target("triage.lifecycle.persist")
            .expect("expected triage.lifecycle.persist entry");
        for required in [
            "service_count",
            "state_size_bytes",
            "duration_ms",
            "persist_kind",
            "corpus_basename",
        ] {
            assert!(
                persist.contains(required),
                "triage.lifecycle.persist must permit `{required}`",
            );
        }

        let persist_err = al
            .for_target("triage.lifecycle.persist.error")
            .expect("expected triage.lifecycle.persist.error entry");
        for required in ["error_category", "duration_ms"] {
            assert!(
                persist_err.contains(required),
                "triage.lifecycle.persist.error must permit `{required}`",
            );
        }

        let banned = [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
            "legacy_path",
            "attribute",
            "body",
        ];
        for target_set in [persist, persist_err] {
            for k in &banned {
                assert!(
                    !target_set.contains(k),
                    "lifecycle persist AllowList must NOT permit PII field `{k}`",
                );
            }
        }
    }

    #[test]
    fn allowlist_for_target_resolves_lifecycle_corpus_restore_extended_field_set() {
        // chunk #67 stub fields preserved; chunk #71 adds
        // `restored_service_count` + `duration_ms` for production
        // emission per plan §11.
        let al = AllowList::production();
        let entry = al
            .for_target("triage.lifecycle.corpus_restore")
            .expect("expected triage.lifecycle.corpus_restore entry");
        for required in ["kind", "count", "restored_service_count", "duration_ms"] {
            assert!(
                entry.contains(required),
                "triage.lifecycle.corpus_restore must permit `{required}` (chunk #71 EXTEND)",
            );
        }
        // PII guard.
        let banned = [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
        ];
        for k in &banned {
            assert!(
                !entry.contains(k),
                "lifecycle.corpus_restore must NOT permit PII field `{k}`",
            );
        }

        // Metric counter sibling.
        let metric = al
            .for_target("metric.triage.lifecycle.corpus_restore_count_total")
            .expect("expected metric.triage.lifecycle.corpus_restore_count_total entry");
        for required in ["value", "kind"] {
            assert!(
                metric.contains(required),
                "metric.triage.lifecycle.corpus_restore_count_total must permit `{required}`",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_storm_persist_field_set() {
        // chunk #71: RetryStormState corpus persistence emits aggregate-only
        // events at `triage.pattern.storm.persist` (info) +
        // `triage.pattern.storm.persist.error` (warn). PII discipline
        // mirrors baseline / lifecycle pattern — bounded enum tags +
        // integer-value fields ONLY.
        let al = AllowList::production();

        let persist = al
            .for_target("triage.pattern.storm.persist")
            .expect("expected triage.pattern.storm.persist entry");
        for required in [
            "fingerprint_count",
            "state_size_bytes",
            "duration_ms",
            "persist_kind",
            "corpus_basename",
        ] {
            assert!(
                persist.contains(required),
                "triage.pattern.storm.persist must permit `{required}`",
            );
        }

        let persist_err = al
            .for_target("triage.pattern.storm.persist.error")
            .expect("expected triage.pattern.storm.persist.error entry");
        for required in ["error_category", "duration_ms"] {
            assert!(
                persist_err.contains(required),
                "triage.pattern.storm.persist.error must permit `{required}`",
            );
        }

        let banned = [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
            "fingerprint_hex_full",
            "legacy_path",
        ];
        for target_set in [persist, persist_err] {
            for k in &banned {
                assert!(
                    !target_set.contains(k),
                    "storm persist AllowList must NOT permit PII field `{k}`",
                );
            }
        }
    }

    #[test]
    fn allowlist_for_target_resolves_storm_corpus_restore_field_set() {
        // chunk #71: storm corpus restore tracing target + metric
        // counter. Both NEW entries per plan §11.
        let al = AllowList::production();

        let restore = al
            .for_target("triage.pattern.storm.corpus_restore")
            .expect("expected triage.pattern.storm.corpus_restore entry");
        for required in ["restored_fingerprint_count", "duration_ms", "kind"] {
            assert!(
                restore.contains(required),
                "triage.pattern.storm.corpus_restore must permit `{required}`",
            );
        }

        let metric = al
            .for_target("metric.triage.pattern.storm.corpus_restore_count_total")
            .expect("expected metric.triage.pattern.storm.corpus_restore_count_total entry");
        for required in ["value", "kind"] {
            assert!(
                metric.contains(required),
                "metric.triage.pattern.storm.corpus_restore_count_total must permit `{required}`",
            );
        }

        // PII guard on both.
        let banned = [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
            "fingerprint_hex_full",
        ];
        for target_set in [restore, metric] {
            for k in &banned {
                assert!(
                    !target_set.contains(k),
                    "storm restore AllowList must NOT permit PII field `{k}`",
                );
            }
        }
    }

    #[test]
    fn allowlist_for_target_resolves_curation_field_set() {
        // chunk #58: curation primitives (dedupe / anomaly / critical_path /
        // aggregation) moved from crates/snapshot к crates/curation. The
        // #[tracing::instrument] decorators emit spans at default module-path
        // targets `curation::dedupe`, `curation::anomaly`, etc. for_target
        // resolves via split("::").next() fall-through к the `curation`
        // entry, which MUST permit the union of fields emitted across the
        // 4 primitives or default-deny silently redacts them.
        let al = AllowList::production();
        for target in [
            "curation::dedupe",
            "curation::anomaly",
            "curation::critical_path",
            "curation::aggregation",
        ] {
            let set = al
                .for_target(target)
                .unwrap_or_else(|| panic!("expected curation entry resolution for {target}"));
            for required in [
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
            ] {
                assert!(
                    set.contains(required),
                    "{target} must permit `{required}` (curation allowlist coverage)",
                );
            }
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_snapshot_curate_field() {
        // chunk #39: per security plan §Logging Vector 1, raw OTLP attribute
        // values (service_name / span attributes / span content) MUST be
        // redacted at the formatter layer. The snapshot allowlist only
        // permits scalar counts + categorical bounds; non-allowlisted fields
        // like `service_name`, `attributes`, `query_text` MUST be stripped
        // even though they would semantically resolve via split('.').next() to
        // the `snapshot` entry.
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "snapshot.curate.dedup",
                dedup_count = 5_u64,
                service_name = "checkout",
                attributes = "{\"user_id\": \"abc\"}",
                query_text = "SELECT * FROM spans",
                "dedup complete",
            );
        });
        let fields = &lines[0]["fields"];
        // Allowlisted field present.
        assert_eq!(fields["dedup_count"], 5);
        // Non-allowlisted fields redacted to "<redacted>" sentinel.
        for forbidden in ["service_name", "attributes", "query_text"] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted per obs-plan Vector 1+5 + snapshot allowlist (default-deny)",
            );
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_snapshot_aggregate_field() {
        // chunk #40: aggregation primitive emits at `snapshot.curate.aggregate`
        // target which resolves via fall-through to the `snapshot` allowlist
        // entry. Non-allowlisted fields (raw attribute values that may
        // incidentally carry secrets per security plan §Logging Vector 1)
        // MUST be redacted by the JsonFieldVisitor.
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "snapshot.curate.aggregate",
                p99_ms = 42_u64,
                attribute_value = "secret-token",
                attribute_key = "user.id",
                raw_metric_payload = "{\"a\":1}",
                "aggregation complete",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["p99_ms"], 42);
        for forbidden in ["attribute_value", "attribute_key", "raw_metric_payload"] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted at snapshot.curate.aggregate target (default-deny via snapshot fall-through)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_service_went_silent_evaluate_field_set() {
        // chunk #64: per-tick aggregate event for ServiceWentSilent gating.
        // Bounded-cardinality count fields only — no per-service identifiers
        // per chunk #62/#63 PII discipline.
        let al = AllowList::production();
        let set = al
            .for_target("triage.baseline.service_went_silent.evaluate")
            .expect("triage.baseline.service_went_silent.evaluate entry");
        for required in [
            "services_tracked",
            "services_in_bootstrap",
            "services_ready",
            "silence_cues_emitted",
        ] {
            assert!(
                set.contains(required),
                "service_went_silent.evaluate must permit `{required}`",
            );
        }
        for banned in [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
        ] {
            assert!(
                !set.contains(banned),
                "service_went_silent.evaluate must NOT permit `{banned}` (chunk #62/#63 PII discipline)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_service_cap_exceeded_field_set() {
        // chunk #64: aggregate warn when service.name cardinality cap is
        // reached. dropped_count + cap fields only — no per-service identifiers.
        let al = AllowList::production();
        let set = al
            .for_target("triage.baseline.service_cap_exceeded")
            .expect("triage.baseline.service_cap_exceeded entry");
        for required in ["dropped_count", "cap"] {
            assert!(
                set.contains(required),
                "service_cap_exceeded must permit `{required}`",
            );
        }
        for banned in ["service_name", "scope_id"] {
            assert!(
                !set.contains(banned),
                "service_cap_exceeded must NOT permit `{banned}` (chunk #62/#63 PII discipline)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_metric_activity_floor_bootstrap_state_field_set() {
        // chunk #64: per-tick bootstrap-state gauge. Aggregate count fields
        // only — no per-service identifiers.
        let al = AllowList::production();
        let set = al
            .for_target("metric.triage.activity_floor.bootstrap_state")
            .expect("metric.triage.activity_floor.bootstrap_state entry");
        for required in [
            "value",
            "services_in_bootstrap",
            "services_ready",
            "services_tracked",
        ] {
            assert!(
                set.contains(required),
                "metric.triage.activity_floor.bootstrap_state must permit `{required}`",
            );
        }
        for banned in ["service_name", "scope_id"] {
            assert!(
                !set.contains(banned),
                "metric.triage.activity_floor.bootstrap_state must NOT permit `{banned}` (chunk #62/#63 PII discipline)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_storm_detected_field_set() {
        // chunk #66: retry storm detector emits at triage.pattern.storm.*
        // targets. Aggregate identifier-class fields only — fingerprint_hex
        // is the 8-char hash prefix (bounded cardinality opaque ID),
        // severity_hint + cue_kind are bounded enum tags, occurrence_count
        // + window_seconds are bounded numerics. NO `service_name`/`scope_id`
        // even though the cue payload carries them (broadcast IS the surface
        // for those; self-observation log is aggregate-only per chunk #62/
        // #63/#64 convention reaffirmed by testing.md 2026-05-17 session 84).
        let al = AllowList::production();
        for target in [
            "triage.pattern.storm.detected",
            "triage.pattern.storm.emit",
            "triage.pattern.storm.tick",
            "metric.triage.pattern.storm_detected_count",
            "metric.triage.pattern.fingerprints_tracked",
            "metric.triage.pattern.fingerprint_evicted_count",
        ] {
            let set = al
                .for_target(target)
                .unwrap_or_else(|| panic!("{target} allowlist entry MUST exist"));
            for banned in [
                "service_name",
                "scope_id",
                "span_id",
                "trace_id",
                "attribute",
                "instrumentation_scope",
                "body",
                "exception_message",
                "exception_stacktrace",
            ] {
                assert!(
                    !set.contains(banned),
                    "{target} must NOT permit `{banned}` (chunk #62/#63/#64 PII discipline)",
                );
            }
        }
        // Spot-check required fields на the primary detection target.
        let detected = al
            .for_target("triage.pattern.storm.detected")
            .expect("storm.detected entry");
        for required in [
            "cue_kind",
            "severity_hint",
            "occurrence_count",
            "window_seconds",
            "fingerprint_hex",
        ] {
            assert!(
                detected.contains(required),
                "triage.pattern.storm.detected must permit `{required}`",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_triage_lifecycle_tick_field_set() {
        // chunk #67: service lifecycle heartbeat tick emits aggregate
        // per-state counts only. NO per-service identifiers — those ride
        // the `pulse://stream/service-lifecycle` broadcast topic instead.
        let al = AllowList::production();
        let set = al
            .for_target("triage.lifecycle.tick")
            .expect("triage.lifecycle.tick entry");
        for required in [
            "tracked_services_total",
            "services_unknown",
            "services_bootstrapping",
            "services_active",
            "services_quiet",
            "services_silent",
            "services_dormant",
            "services_archived",
        ] {
            assert!(
                set.contains(required),
                "triage.lifecycle.tick must permit `{required}`",
            );
        }
        for banned in [
            "service_name",
            "service_id",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
            "attribute",
            "instrumentation_scope",
            "body",
        ] {
            assert!(
                !set.contains(banned),
                "triage.lifecycle.tick must NOT permit `{banned}` (chunk #62/#63/#64 PII discipline)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_triage_lifecycle_transition_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("triage.lifecycle.transition")
            .expect("triage.lifecycle.transition entry");
        for required in ["from_state", "to_state", "count"] {
            assert!(
                set.contains(required),
                "triage.lifecycle.transition must permit `{required}`",
            );
        }
        for banned in [
            "service",
            "service_name",
            "service_id",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
        ] {
            assert!(
                !set.contains(banned),
                "triage.lifecycle.transition must NOT permit `{banned}` (aggregate-only per chunk #67)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_triage_lifecycle_corpus_restore_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("triage.lifecycle.corpus_restore")
            .expect("triage.lifecycle.corpus_restore entry");
        for required in ["kind", "count"] {
            assert!(
                set.contains(required),
                "triage.lifecycle.corpus_restore must permit `{required}`",
            );
        }
        for banned in ["service_name", "service_id", "scope_id"] {
            assert!(
                !set.contains(banned),
                "triage.lifecycle.corpus_restore must NOT permit `{banned}` (chunk #67 PII discipline)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_services_list_with_states_request_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("services.list_with_states.request")
            .expect("services.list_with_states.request entry");
        for required in ["item_count", "traceparent"] {
            assert!(
                set.contains(required),
                "services.list_with_states.request must permit `{required}`",
            );
        }
        for banned in [
            "service_name",
            "service_id",
            "scope_id",
            "items",
            "manual_override",
        ] {
            assert!(
                !set.contains(banned),
                "services.list_with_states.request must NOT permit `{banned}` (chunk #67 PII discipline — response body excluded from log)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_pipeline_l1b_tracked_services_total_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("pipeline.l1b.tracked_services_total")
            .expect("pipeline.l1b.tracked_services_total entry");
        assert!(set.contains("value"), "must permit `value`");
        for banned in ["service_name", "service_id", "scope_id"] {
            assert!(
                !set.contains(banned),
                "pipeline.l1b.tracked_services_total must NOT permit `{banned}` (chunk #67 PII discipline)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_metric_triage_lifecycle_state_distribution_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("metric.triage.lifecycle.state_distribution")
            .expect("metric.triage.lifecycle.state_distribution entry");
        for required in [
            "value",
            "services_unknown",
            "services_bootstrapping",
            "services_active",
            "services_quiet",
            "services_silent",
            "services_dormant",
            "services_archived",
        ] {
            assert!(
                set.contains(required),
                "metric.triage.lifecycle.state_distribution must permit `{required}`",
            );
        }
        for banned in ["service_name", "service_id", "scope_id"] {
            assert!(
                !set.contains(banned),
                "metric.triage.lifecycle.state_distribution must NOT permit `{banned}` (chunk #67 PII discipline)",
            );
        }
    }

    // ===== Chunk #69 Phase B Session 6 — Drain + diagnostics AllowList tests =====

    #[test]
    fn allowlist_for_target_extends_buffer_with_drain_heartbeat_fields() {
        let al = AllowList::production();
        let set = al.for_target("buffer").expect("buffer entry");
        for required in ["drain_template_count", "drain_lru_evictions_since_tick"] {
            assert!(
                set.contains(required),
                "buffer must permit `{required}` (chunk #69 buffer.tick drain fields)",
            );
        }
        // Chunk #69 AGGREGATE-ONLY discipline applies even though we did
        // not remove existing buffer fields — the new drain fields are
        // bounded counts; per-template-content fields stay forbidden.
        for banned in ["template_text", "sample_message", "service_name"] {
            assert!(
                !set.contains(banned),
                "buffer must NOT permit `{banned}` (chunk #69 aggregate-only discipline)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_drain_persistence_load_ok_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("drain.persistence.load.ok")
            .expect("drain.persistence.load.ok entry");
        assert!(
            set.contains("template_count"),
            "drain.persistence.load.ok must permit `template_count`",
        );
        for banned in [
            "template_text",
            "sample_message",
            "service_name",
            "scope_id",
        ] {
            assert!(
                !set.contains(banned),
                "drain.persistence.load.ok must NOT permit `{banned}` (chunk #69 aggregate-only)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_drain_persistence_unavailable_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("drain.persistence.unavailable")
            .expect("drain.persistence.unavailable entry");
        assert!(
            set.contains("reason"),
            "drain.persistence.unavailable must permit `reason`",
        );
        for banned in [
            "template_text",
            "sample_message",
            "service_name",
            "stacktrace",
        ] {
            assert!(
                !set.contains(banned),
                "drain.persistence.unavailable must NOT permit `{banned}` (PII / opaque-error discipline)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_diagnostics_template_distribution_request_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("diagnostics.template_distribution.request")
            .expect("diagnostics.template_distribution.request entry");
        for required in ["top_n", "result_count", "duration_ms"] {
            assert!(
                set.contains(required),
                "diagnostics.template_distribution.request must permit `{required}`",
            );
        }
        for banned in [
            "template_text",
            "sample_message",
            "service_name",
            "templates",
            "content",
        ] {
            assert!(
                !set.contains(banned),
                "diagnostics.template_distribution.request must NOT permit `{banned}` (chunk #69 aggregate-only)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_metric_pipeline_l1c_drain_template_count_total_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("metric.pipeline.l1c.drain_template_count_total")
            .expect("metric.pipeline.l1c.drain_template_count_total entry");
        assert!(
            set.contains("value"),
            "metric.pipeline.l1c.drain_template_count_total must permit `value`",
        );
        for banned in [
            "template_text",
            "sample_message",
            "service_name",
            "template_id",
        ] {
            assert!(
                !set.contains(banned),
                "metric.pipeline.l1c.drain_template_count_total must NOT permit `{banned}` (chunk #69 aggregate-only)",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_metric_pipeline_l1c_drain_assignment_latency_p99_microseconds_field_set()
     {
        let al = AllowList::production();
        let set = al
            .for_target("metric.pipeline.l1c.drain_assignment_latency_p99_microseconds")
            .expect("metric.pipeline.l1c.drain_assignment_latency_p99_microseconds entry");
        assert!(
            set.contains("value"),
            "metric.pipeline.l1c.drain_assignment_latency_p99_microseconds must permit `value`",
        );
        for banned in [
            "template_text",
            "sample_message",
            "service_name",
            "template_id",
        ] {
            assert!(
                !set.contains(banned),
                "metric.pipeline.l1c.drain_assignment_latency_p99_microseconds must NOT permit `{banned}` (chunk #69 aggregate-only)",
            );
        }
    }

    #[test]
    fn allowlist_resolves_each_cadence_target_with_expected_fields() {
        let al = AllowList::production();

        let tick = al.for_target("cadence.tick").expect("cadence.tick entry");
        for required in [
            "tier",
            "mode",
            "queries_executed",
            "queries_succeeded",
            "next_due_ms",
            "last_executed_at_ms",
            "tier2_acceleration_enabled",
        ] {
            assert!(
                tick.contains(required),
                "cadence.tick must permit `{required}`"
            );
        }

        let trigger = al
            .for_target("cadence.trigger")
            .expect("cadence.trigger entry");
        for required in ["tier", "mode", "cue_kind", "priority"] {
            assert!(
                trigger.contains(required),
                "cadence.trigger must permit `{required}`"
            );
        }

        let load = al
            .for_target("cadence.config.load")
            .expect("cadence.config.load entry");
        for required in [
            "baseline_seconds",
            "accelerated_seconds",
            "reflection_seconds",
            "tier2_acceleration_enabled",
        ] {
            assert!(
                load.contains(required),
                "cadence.config.load must permit `{required}`"
            );
        }

        let floor = al
            .for_target("cadence.config.safety_floor")
            .expect("cadence.config.safety_floor entry");
        for required in ["field", "requested_seconds", "enforced_seconds"] {
            assert!(
                floor.contains(required),
                "cadence.config.safety_floor must permit `{required}`"
            );
        }

        let metric = al
            .for_target("metric.pipeline.l3.digests_assembled_total")
            .expect("metric.pipeline.l3.digests_assembled_total entry");
        for required in ["value", "mode", "queries_executed"] {
            assert!(
                metric.contains(required),
                "metric.pipeline.l3.digests_assembled_total must permit `{required}`"
            );
        }
    }

    #[test]
    fn cadence_targets_ban_pii_fields_per_chunk_62_precedent() {
        let al = AllowList::production();
        let cadence_targets = [
            "cadence.tick",
            "cadence.trigger",
            "cadence.config.load",
            "cadence.config.safety_floor",
            "metric.pipeline.l3.digests_assembled_total",
        ];
        let banned_fields = [
            "service",
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
            "digest_body",
            "inference_input",
            "inference_output",
        ];
        for target in cadence_targets {
            let set = al
                .for_target(target)
                .unwrap_or_else(|| panic!("expected allowlist entry for `{target}`"));
            for banned in banned_fields {
                assert!(
                    !set.contains(banned),
                    "cadence target `{target}` must NOT permit `{banned}` field (chunk #62 aggregate-only precedent)"
                );
            }
        }
    }

    #[test]
    fn allowlist_for_target_resolves_diagnostics_prefix_fallback_to_diagnostics_entry() {
        // Verify the dotted-target prefix-strip fallback per CLAUDE.md
        // 2026-05-07: `diagnostics.X` (unknown sub-namespace) resolves via
        // split('.').next() → "diagnostics" entry instead of redacting all
        // fields. This is the safety net under the explicit-leaf entries.
        let al = AllowList::production();
        let set = al
            .for_target("diagnostics.unknown.future.target")
            .expect("diagnostics fallback entry");
        for required in ["top_n", "result_count", "duration_ms"] {
            assert!(
                set.contains(required),
                "diagnostics fallback must permit `{required}`",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_snapshot_render_markdown_field_set() {
        // chunk #41: markdown formatter emits at `snapshot.render.markdown`
        // target. Registered explicitly so for_target() exact-match wins over
        // the `snapshot` crate-level fallback (which would silently redact
        // `markdown_size_bytes` / `budget_token_limit` / `truncated_*_count`)
        // per .claude/rules/observability.md Session Addition 2026-05-07.
        let al = AllowList::production();
        let set = al
            .for_target("snapshot.render.markdown")
            .expect("snapshot.render.markdown entry");
        for required in [
            "markdown_size_bytes",
            "section_count",
            "token_count_actual",
            "budget_token_limit",
            "anomaly_marker_count",
            "critical_path_span_count",
            "truncated_span_count",
            "truncated_attribute_count",
            "duration_ms",
        ] {
            assert!(
                set.contains(required),
                "snapshot.render.markdown must permit `{required}`",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_snapshot_token_count_validate_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("snapshot.token.count.validate")
            .expect("snapshot.token.count.validate entry");
        for required in [
            "token_count_actual",
            "token_budget_limit",
            "budget_exceeded",
            "duration_ms",
        ] {
            assert!(
                set.contains(required),
                "snapshot.token.count.validate must permit `{required}`",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_metric_snapshot_token_count_ms_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("metric.snapshot.token_count_ms")
            .expect("metric.snapshot.token_count_ms entry");
        for required in [
            "value",
            "duration_ms",
            "token_budget",
            "time_range_minutes",
            "token_count_actual",
            "dedup_count",
            "budget_exceeded",
        ] {
            assert!(
                set.contains(required),
                "metric.snapshot.token_count_ms must permit `{required}`",
            );
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_snapshot_render_markdown_field() {
        // chunk #41: markdown formatter emits at `snapshot.render.markdown`;
        // security plan §Logging Vectors 1 + 4 demand markdown body contents
        // + raw OTLP attribute values + citation anchor labels MUST be
        // redacted. The `target:` argument to tracing macros must be a
        // `&'static str` literal (macro expands to `static __CALLSITE`), so
        // each target gets its own test rather than a for-loop.
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "snapshot.render.markdown",
                duration_ms = 100_u64,
                markdown_body = "# title\nsecret content",
                citation_anchor_label = "user-name=alice",
                attribute_value = "secret-token",
                raw_attribute_value = "Bearer abc",
                service_name_raw = "checkout-prod",
                snapshot_file_content = "## Anomaly\nSensitive",
                raw_span_name = "POST /pay",
                "format event",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["duration_ms"], 100);
        for forbidden in [
            "markdown_body",
            "citation_anchor_label",
            "attribute_value",
            "raw_attribute_value",
            "service_name_raw",
            "snapshot_file_content",
            "raw_span_name",
        ] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted at snapshot.render.markdown target",
            );
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_snapshot_token_count_validate_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "snapshot.token.count.validate",
                token_count_actual = 12_345_u64,
                markdown_body = "# title\nsecret content",
                attribute_value = "secret-token",
                raw_attribute_value = "Bearer abc",
                "validate event",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["token_count_actual"], 12345);
        for forbidden in ["markdown_body", "attribute_value", "raw_attribute_value"] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted at snapshot.token.count.validate target",
            );
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_metric_snapshot_token_count_ms_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.snapshot.token_count_ms",
                value = 42.0_f64,
                duration_ms = 42_u64,
                markdown_body = "# title\nsecret content",
                attribute_value = "secret-token",
                snapshot_file_content = "## Anomaly\nSensitive",
                "metric event",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["duration_ms"], 42);
        for forbidden in ["markdown_body", "attribute_value", "snapshot_file_content"] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted at metric.snapshot.token_count_ms target",
            );
        }
    }

    #[test]
    fn allowlist_for_target_resolves_snapshot_generate_request_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("snapshot.generate.request")
            .expect("snapshot.generate.request entry");
        for required in [
            "budget",
            "result_kind",
            "message",
            "token_count",
            "dual_file_paths_basenames",
            "preset_prompts_count",
        ] {
            assert!(
                set.contains(required),
                "snapshot.generate.request must permit `{required}`",
            );
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_snapshot_generate_request_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "snapshot.generate.request",
                budget = "balanced",
                result_kind = "placeholder",
                message = "Investigate not yet wired (lands chunk #43)",
                clipboard_content = "secret data",
                snapshot_file_content = "## Anomaly\nSensitive",
                raw_attribute_value = "Bearer abc",
                "Investigate trigger reached placeholder",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["budget"], "balanced");
        assert_eq!(fields["result_kind"], "placeholder");
        for forbidden in [
            "clipboard_content",
            "snapshot_file_content",
            "raw_attribute_value",
        ] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted at snapshot.generate.request target",
            );
        }
    }

    // chunk #43 — paired tests for snapshot.clipboard.write per-leaf entry
    #[test]
    fn allowlist_for_target_resolves_snapshot_clipboard_write_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("snapshot.clipboard.write")
            .expect("snapshot.clipboard.write entry");
        for required in ["byte_count", "success", "dual_file_paths_basenames"] {
            assert!(
                set.contains(required),
                "snapshot.clipboard.write must permit `{required}`",
            );
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_snapshot_clipboard_write_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "snapshot.clipboard.write",
                byte_count = 1234_u64,
                success = true,
                clipboard_content = "secret canary payload",
                raw_markdown = "## Anomaly\nSensitive",
                "clipboard write event",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["byte_count"], 1234);
        assert_eq!(fields["success"], true);
        for forbidden in ["clipboard_content", "raw_markdown"] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted at snapshot.clipboard.write target",
            );
        }
    }

    // chunk #43 — paired tests for snapshot.notification.dispatch per-leaf entry
    #[test]
    fn allowlist_for_target_resolves_snapshot_notification_dispatch_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("snapshot.notification.dispatch")
            .expect("snapshot.notification.dispatch entry");
        for required in ["notification_kind", "dispatch_success"] {
            assert!(
                set.contains(required),
                "snapshot.notification.dispatch must permit `{required}`",
            );
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_snapshot_notification_dispatch_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "snapshot.notification.dispatch",
                notification_kind = "snapshot_ready",
                dispatch_success = true,
                toast_body = "Snapshot ready | 2.5k tokens, secret-canary",
                preset_prompt_text = "Diagnose latency outlier",
                "notification dispatched",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["notification_kind"], "snapshot_ready");
        assert_eq!(fields["dispatch_success"], true);
        for forbidden in ["toast_body", "preset_prompt_text"] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted at snapshot.notification.dispatch target",
            );
        }
    }

    // chunk #43 — paired tests for workspace.detect per-leaf entry
    #[test]
    fn allowlist_for_target_resolves_workspace_detect_field_set() {
        let al = AllowList::production();
        let set = al
            .for_target("workspace.detect")
            .expect("workspace.detect entry");
        for required in [
            "workspace_root_basename",
            "project_name",
            "vcs_type",
            "vcs_root_basename",
            "marker_present",
            "detection_latency_ms",
        ] {
            assert!(
                set.contains(required),
                "workspace.detect must permit `{required}`",
            );
        }
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_workspace_detect_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "workspace.detect",
                workspace_root_basename = "example",
                project_name = "example",
                vcs_type = "git",
                marker_present = true,
                detection_latency_ms = 5_u64,
                workspace_root = "/tmp/secret-canary/example",
                vcs_commit_message = "fix: secret canary commit",
                "workspace detected",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["workspace_root_basename"], "example");
        assert_eq!(fields["project_name"], "example");
        assert_eq!(fields["vcs_type"], "git");
        for forbidden in ["workspace_root", "vcs_commit_message"] {
            assert_eq!(
                fields[forbidden], "<redacted>",
                "field `{forbidden}` MUST be redacted at workspace.detect target",
            );
        }
    }

    #[test]
    fn scrubber_passes_port_validation_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "config.load.port_validation",
                env_var_name = "ANDROMEDA_PULSE_OTLP_GRPC_PORT",
                reject_reason = "out_of_range",
                "OTLP port env var rejected",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["env_var_name"], "ANDROMEDA_PULSE_OTLP_GRPC_PORT");
        assert_eq!(fields["reject_reason"], "out_of_range");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_port_validation_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "config.load.port_validation",
                env_var_name = "ANDROMEDA_PULSE_OTLP_GRPC_PORT",
                raw_value = "99999",
                "OTLP port env var rejected",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["env_var_name"], "ANDROMEDA_PULSE_OTLP_GRPC_PORT");
        assert_eq!(
            fields["raw_value"], "<redacted>",
            "raw env-var value MUST NOT leak through scrubber per Vector 6"
        );
    }

    #[test]
    fn scrubber_passes_grpc_rate_limit_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "ingest.grpc.rate_limit.rejected",
                quota_window_seconds = 5_u64,
                reject_reason = "rate_limit_exceeded",
                "OTLP gRPC rate limit hit",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["quota_window_seconds"], 5);
        assert_eq!(fields["reject_reason"], "rate_limit_exceeded");
    }

    #[test]
    fn scrubber_passes_http_rate_limit_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "ingest.http.rate_limit.rejected",
                quota_window_seconds = 3_u64,
                reject_reason = "rate_limit_exceeded",
                "OTLP HTTP rate limit hit",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["quota_window_seconds"], 3);
        assert_eq!(fields["reject_reason"], "rate_limit_exceeded");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_rate_limit_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "ingest.grpc.rate_limit.rejected",
                quota_window_seconds = 5_u64,
                client_ip = "127.0.0.1",
                "OTLP gRPC rate limit hit",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["quota_window_seconds"], 5);
        assert_eq!(
            fields["client_ip"], "<redacted>",
            "client_ip MUST NOT leak per obs-plan §11 cardinality discipline"
        );
    }

    #[test]
    fn scrubber_passes_buffer_schema_init_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "buffer.schema.init",
                table_count = 7_u64,
                duration_ms = 12_u64,
                "ring buffer schema created",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["table_count"], 7);
        assert_eq!(fields["duration_ms"], 12);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_buffer_schema_init_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "buffer.schema.init",
                table_count = 7_u64,
                raw_ddl = "CREATE TABLE secrets (...)",
                "ring buffer schema created",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["table_count"], 7);
        assert_eq!(
            fields["raw_ddl"], "<redacted>",
            "raw DDL MUST NOT leak per security plan §Anti-Patterns Logging"
        );
    }

    #[test]
    fn scrubber_passes_buffer_schema_init_error_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "buffer.schema.init.error",
                error_type = "schema_create",
                spantrace = "captured",
                "DuckDB schema creation failed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["error_type"], "schema_create");
        assert_eq!(fields["spantrace"], "captured");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_buffer_schema_init_error_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "buffer.schema.init.error",
                error_type = "schema_create",
                raw_duckdb_error_text = "internal error 0xdeadbeef",
                "DuckDB schema creation failed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["error_type"], "schema_create");
        assert_eq!(
            fields["raw_duckdb_error_text"], "<redacted>",
            "raw DuckDB error text MUST NOT leak"
        );
    }

    #[test]
    fn scrubber_passes_duckdb_append_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "duckdb.append",
                rows_appended = 100_u64,
                duration_ms = 4_u64,
                table_name = "spans",
                "Arrow appender wrote rows",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["rows_appended"], 100);
        assert_eq!(fields["duration_ms"], 4);
        assert_eq!(fields["table_name"], "spans");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_duckdb_append_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "duckdb.append",
                rows_appended = 100_u64,
                raw_otlp_payload = "secret_data_attribute_value",
                attribute_value = "leak",
                "Arrow appender wrote rows",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["rows_appended"], 100);
        assert_eq!(
            fields["raw_otlp_payload"], "<redacted>",
            "raw OTLP payload MUST NOT leak per Vector 1"
        );
        assert_eq!(
            fields["attribute_value"], "<redacted>",
            "OTLP attribute value MUST NOT leak per Vector 1"
        );
    }

    // chunk #21 retention sweep + metric.buffer.* + config.load.retention_seconds
    // tests. Five pass / five redact pairs mirroring the chunk #20 precedent above.

    #[test]
    fn scrubber_passes_buffer_retention_sweep_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "buffer.retention.sweep",
                query_id = "buffer.retention.sweep",
                param_count = 1_u64,
                param_types = "timestamp",
                rows_evicted = 42_u64,
                duration_ms = 12_u64,
                cutoff_ts_unix_nano = 1_700_000_000_000_000_000_i64,
                "retention sweep completed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["query_id"], "buffer.retention.sweep");
        assert_eq!(fields["param_count"], 1);
        assert_eq!(fields["param_types"], "timestamp");
        assert_eq!(fields["rows_evicted"], 42);
        assert_eq!(fields["duration_ms"], 12);
        assert_eq!(fields["cutoff_ts_unix_nano"], 1_700_000_000_000_000_000_i64);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_buffer_retention_sweep_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "buffer.retention.sweep",
                query_id = "buffer.retention.sweep",
                raw_sql = "DELETE FROM spans WHERE ts < ...",
                "retention sweep completed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["query_id"], "buffer.retention.sweep");
        assert_eq!(
            fields["raw_sql"], "<redacted>",
            "raw SQL MUST NOT leak per security plan §Anti-Patterns Logging Vector 5"
        );
    }

    #[test]
    fn scrubber_passes_buffer_retention_sweep_error_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "buffer.retention.sweep.error",
                error_type = "connection_lost",
                duration_ms = 5_u64,
                spantrace = "captured",
                "retention sweep failed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["error_type"], "connection_lost");
        assert_eq!(fields["duration_ms"], 5);
        assert_eq!(fields["spantrace"], "captured");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_buffer_retention_sweep_error_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "buffer.retention.sweep.error",
                error_type = "execute_failed",
                raw_duckdb_text = "internal error 0xdeadbeef at /tmp/secret",
                "retention sweep failed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["error_type"], "execute_failed");
        assert_eq!(
            fields["raw_duckdb_text"], "<redacted>",
            "raw DuckDB error text MUST NOT leak"
        );
    }

    #[test]
    fn scrubber_passes_metric_buffer_memory_bytes_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.buffer.memory_bytes",
                value = 4096_u64,
                retention_window_seconds = 600_u64,
                rows_active = 16_u64,
                eviction_count_since_last_tick = 0_u64,
                "buffer memory gauge",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 4096);
        assert_eq!(fields["retention_window_seconds"], 600);
        assert_eq!(fields["rows_active"], 16);
        assert_eq!(fields["eviction_count_since_last_tick"], 0);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_metric_buffer_memory_bytes_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.buffer.memory_bytes",
                value = 4096_u64,
                client_ip = "127.0.0.1",
                trace_id = "abcdef",
                "buffer memory gauge",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 4096);
        assert_eq!(
            fields["client_ip"], "<redacted>",
            "high-cardinality field MUST NOT leak per obs-plan §11"
        );
        assert_eq!(
            fields["trace_id"], "<redacted>",
            "trace_id MUST NOT leak per obs-plan §11 cardinality discipline"
        );
    }

    #[test]
    fn scrubber_passes_metric_buffer_evicted_span_count_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.buffer.evicted_span_count",
                value = 12_u64,
                retention_window_seconds = 600_u64,
                "buffer eviction counter",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 12);
        assert_eq!(fields["retention_window_seconds"], 600);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_metric_buffer_evicted_span_count_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.buffer.evicted_span_count",
                value = 12_u64,
                evicted_trace_ids = "leak,leak,leak",
                "buffer eviction counter",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 12);
        assert_eq!(
            fields["evicted_trace_ids"], "<redacted>",
            "evicted-row identifiers MUST NOT leak per Vector 1"
        );
    }

    #[test]
    fn scrubber_passes_config_load_retention_seconds_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "config.load.retention_seconds",
                env_var_name = "ANDROMEDA_PULSE_RETENTION_SECONDS",
                reject_reason = "out_of_range",
                "retention seconds env var rejected",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["env_var_name"], "ANDROMEDA_PULSE_RETENTION_SECONDS");
        assert_eq!(fields["reject_reason"], "out_of_range");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_config_load_retention_seconds_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "config.load.retention_seconds",
                env_var_name = "ANDROMEDA_PULSE_RETENTION_SECONDS",
                raw_value = "999999",
                "retention seconds env var rejected",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["env_var_name"], "ANDROMEDA_PULSE_RETENTION_SECONDS");
        assert_eq!(
            fields["raw_value"], "<redacted>",
            "raw env-var value MUST NOT leak per Vector 6"
        );
    }

    // chunk #22 viz.query.* + metric.trace.* + extended viz allowlist
    // tests. Five pass / five redact pairs mirroring chunk #20/#21 precedent.

    #[test]
    fn scrubber_passes_viz_query_traces_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "viz.query.traces",
                query_id = "viz.query.traces",
                param_count = 3_u64,
                param_types = "i64,i64,i64",
                time_window_seconds = 60_u64,
                row_count = 42_u64,
                latency_ms = 18_u64,
                "viz traces query completed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["query_id"], "viz.query.traces");
        assert_eq!(fields["param_count"], 3);
        assert_eq!(fields["param_types"], "i64,i64,i64");
        assert_eq!(fields["time_window_seconds"], 60);
        assert_eq!(fields["row_count"], 42);
        assert_eq!(fields["latency_ms"], 18);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_viz_query_traces_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "viz.query.traces",
                query_id = "viz.query.traces",
                service_filter_value = "user-controlled-payload",
                raw_sql = "SELECT * FROM spans WHERE service = 'leak'",
                "viz traces query completed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["query_id"], "viz.query.traces");
        assert_eq!(
            fields["service_filter_value"], "<redacted>",
            "user-controlled parameter value MUST NOT leak per Vector 5"
        );
        assert_eq!(
            fields["raw_sql"], "<redacted>",
            "raw SQL MUST NOT leak per Vector 5"
        );
    }

    #[test]
    fn scrubber_passes_viz_query_error_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "viz.query.error",
                error_type = "query_failed",
                duration_ms = 12_u64,
                spantrace = "captured",
                "viz query failed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["error_type"], "query_failed");
        assert_eq!(fields["duration_ms"], 12);
        assert_eq!(fields["spantrace"], "captured");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_viz_query_error_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "viz.query.error",
                error_type = "query_failed",
                raw_duckdb_text = "internal error 0xdeadbeef at /tmp/secret",
                "viz query failed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["error_type"], "query_failed");
        assert_eq!(
            fields["raw_duckdb_text"], "<redacted>",
            "raw DuckDB error text MUST NOT leak"
        );
    }

    #[test]
    fn scrubber_passes_metric_trace_latency_percentiles_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.trace.latency_percentiles",
                value = 18_u64,
                service_name = "test-service",
                p50_ms = 10_u64,
                p95_ms = 50_u64,
                p99_ms = 100_u64,
                max_ms = 200_u64,
                "trace latency",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 18);
        assert_eq!(fields["service_name"], "test-service");
        assert_eq!(fields["p50_ms"], 10);
        assert_eq!(fields["p99_ms"], 100);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_metric_trace_latency_percentiles_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.trace.latency_percentiles",
                value = 18_u64,
                trace_id = "secret-trace-id",
                client_ip = "127.0.0.1",
                "trace latency",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 18);
        assert_eq!(
            fields["trace_id"], "<redacted>",
            "trace_id MUST NOT leak per cardinality discipline"
        );
        assert_eq!(fields["client_ip"], "<redacted>", "client_ip MUST NOT leak");
    }

    #[test]
    fn scrubber_passes_metric_trace_error_count_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.trace.error_count",
                value = 5_u64,
                service_name = "test-service",
                error_type = "query_failed",
                "trace error counter",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 5);
        assert_eq!(fields["service_name"], "test-service");
        assert_eq!(fields["error_type"], "query_failed");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_metric_trace_error_count_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.trace.error_count",
                value = 5_u64,
                error_message_text = "duckdb internal 0xdeadbeef",
                "trace error counter",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 5);
        assert_eq!(
            fields["error_message_text"], "<redacted>",
            "raw error message MUST NOT leak"
        );
    }

    #[test]
    fn scrubber_passes_extended_viz_allowlist_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "viz.query.metrics",
                query_id = "viz.query.metrics",
                param_count = 3_u64,
                param_types = "i64,i64,i64",
                time_window_seconds = 60_u64,
                traceparent = "00-0123456789abcdef0123456789abcdef-0123456789abcdef-01",
                filter_count = 1_u64,
                "viz metrics query completed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["param_types"], "i64,i64,i64");
        assert_eq!(fields["time_window_seconds"], 60);
        assert!(fields["traceparent"].is_string());
        assert_eq!(fields["filter_count"], 1);
    }

    #[test]
    fn scrubber_redacts_query_parameter_value_on_viz_target() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "viz.query.logs",
                query_id = "viz.query.logs",
                cursor_value = "' OR 1=1 --",
                attribute_value = "secret-attribute",
                "viz logs query completed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["query_id"], "viz.query.logs");
        assert_eq!(
            fields["cursor_value"], "<redacted>",
            "cursor_value MUST NOT leak per Vector 5"
        );
        assert_eq!(
            fields["attribute_value"], "<redacted>",
            "OTLP attribute_value MUST NOT leak per Vector 1"
        );
    }

    // chunk #23 tauri.channel.* + metric.ingest.channel.broadcast_subscribers
    // tests. Five pass / five redact pairs mirroring chunks #20/#21/#22 precedent.

    #[test]
    fn scrubber_passes_tauri_channel_emit_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "tauri.channel.emit",
                channel_name = "pulse://stream/spans",
                arrow_schema = "trace_id,span_id,ts,ts_unix_nano",
                row_count = 5_u64,
                payload_size_bytes = 1024_u64,
                schema_match = true,
                traceparent = "00-0123456789abcdef0123456789abcdef-0123456789abcdef-01",
                duration_ms = 2_u64,
                "Arrow IPC payload emitted",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["channel_name"], "pulse://stream/spans");
        assert_eq!(fields["row_count"], 5);
        assert_eq!(fields["payload_size_bytes"], 1024);
        assert_eq!(fields["schema_match"], true);
        assert_eq!(fields["duration_ms"], 2);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_tauri_channel_emit_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "tauri.channel.emit",
                channel_name = "pulse://stream/spans",
                row_count = 5_u64,
                attribute_value = "secret-attr-leak",
                raw_payload_first_bytes = "secret-row-content",
                "Arrow IPC payload emitted",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["channel_name"], "pulse://stream/spans");
        assert_eq!(fields["row_count"], 5);
        assert_eq!(
            fields["attribute_value"], "<redacted>",
            "OTLP attribute value MUST NOT leak per Vector 1"
        );
        assert_eq!(
            fields["raw_payload_first_bytes"], "<redacted>",
            "raw payload bytes MUST NOT leak per Vector 1"
        );
    }

    #[test]
    fn scrubber_passes_tauri_channel_emit_size_exceeded_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "tauri.channel.emit.size_exceeded",
                channel_name = "pulse://stream/spans",
                payload_size_bytes = 9_999_999_u64,
                limit_bytes = 8_388_608_u64,
                error_type = "size_exceeded",
                "Arrow IPC payload exceeded broadcast size cap",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["channel_name"], "pulse://stream/spans");
        assert_eq!(fields["payload_size_bytes"], 9_999_999);
        assert_eq!(fields["limit_bytes"], 8_388_608);
        assert_eq!(fields["error_type"], "size_exceeded");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_tauri_channel_emit_size_exceeded_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "tauri.channel.emit.size_exceeded",
                channel_name = "pulse://stream/spans",
                payload_size_bytes = 9_999_999_u64,
                offending_row_text = "secret-row-content",
                "Arrow IPC payload exceeded broadcast size cap",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["payload_size_bytes"], 9_999_999);
        assert_eq!(
            fields["offending_row_text"], "<redacted>",
            "row content MUST NOT leak"
        );
    }

    #[test]
    fn scrubber_passes_tauri_channel_emit_error_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "tauri.channel.emit.error",
                channel_name = "pulse://stream/metrics",
                error_type = "encode_failed",
                duration_ms = 3_u64,
                spantrace = "captured",
                "Arrow IPC encode failed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["channel_name"], "pulse://stream/metrics");
        assert_eq!(fields["error_type"], "encode_failed");
        assert_eq!(fields["duration_ms"], 3);
        assert_eq!(fields["spantrace"], "captured");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_tauri_channel_emit_error_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::error!(
                target: "tauri.channel.emit.error",
                channel_name = "pulse://stream/metrics",
                error_type = "encode_failed",
                arrow_internal_text = "ArrowError::IpcError at /private 0xdeadbeef",
                "Arrow IPC encode failed",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["error_type"], "encode_failed");
        assert_eq!(
            fields["arrow_internal_text"], "<redacted>",
            "raw library error text MUST NOT leak"
        );
    }

    #[test]
    fn scrubber_passes_tauri_channel_lag_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "tauri.channel.lag",
                channel_name = "pulse://stream/logs",
                dropped_count = 42_u64,
                "broadcast receiver lagged",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["channel_name"], "pulse://stream/logs");
        assert_eq!(fields["dropped_count"], 42);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_tauri_channel_lag_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "tauri.channel.lag",
                channel_name = "pulse://stream/logs",
                dropped_count = 42_u64,
                lost_trace_ids = "id1,id2,id3",
                "broadcast receiver lagged",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["dropped_count"], 42);
        assert_eq!(
            fields["lost_trace_ids"], "<redacted>",
            "lost trace identifiers MUST NOT leak per Vector 1"
        );
    }

    #[test]
    fn scrubber_passes_metric_ingest_channel_broadcast_subscribers_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.ingest.channel.broadcast_subscribers",
                value = 3_u64,
                channel_name = "pulse://stream/spans",
                "broadcast subscriber gauge",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 3);
        assert_eq!(fields["channel_name"], "pulse://stream/spans");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_metric_ingest_channel_broadcast_subscribers_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.ingest.channel.broadcast_subscribers",
                value = 3_u64,
                channel_name = "pulse://stream/spans",
                client_ip = "127.0.0.1",
                subscriber_session_id = "abc-secret",
                "broadcast subscriber gauge",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["value"], 3);
        assert_eq!(
            fields["client_ip"], "<redacted>",
            "client_ip MUST NOT leak per cardinality discipline"
        );
        assert_eq!(
            fields["subscriber_session_id"], "<redacted>",
            "session id MUST NOT leak per cardinality discipline"
        );
    }

    // chunk #24 webview shell — boot detection spans + close→tray transition
    // + tray-boundary surfaces. Five pass / five redact pairs mirroring
    // chunks #20/#21/#22/#23 precedent.

    #[test]
    fn scrubber_passes_app_boot_webview_init_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "app.boot.webview.init",
                webview_backend = "WebView2",
                "webview backend detected at boot",
            );
        });
        assert_eq!(lines[0]["fields"]["webview_backend"], "WebView2");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_app_boot_webview_init_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "app.boot.webview.init",
                webview_backend = "WebView2",
                user_agent_string = "Mozilla/5.0 (raw secret context)",
                "webview backend detected at boot",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["webview_backend"], "WebView2");
        assert_eq!(
            fields["user_agent_string"], "<redacted>",
            "raw user-agent string MUST NOT leak per Vector 1 cardinality discipline"
        );
    }

    #[test]
    fn scrubber_passes_app_boot_gpu_check_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "app.boot.gpu.check",
                gpu_available = true,
                wgpu_backend = "metal",
                "GPU adapter check",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["gpu_available"], true);
        assert_eq!(fields["wgpu_backend"], "metal");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_app_boot_gpu_check_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "app.boot.gpu.check",
                gpu_available = false,
                wgpu_backend = "vulkan",
                adapter_vendor_id = "0xDEADBEEF",
                "GPU adapter check",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["wgpu_backend"], "vulkan");
        assert_eq!(
            fields["adapter_vendor_id"], "<redacted>",
            "raw adapter identifiers MUST NOT leak (cardinality discipline)"
        );
    }

    #[test]
    fn scrubber_passes_metric_webgpu_frame_duration_ms_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.webgpu.frame_duration_ms",
                duration_ms = 16.7,
                wgpu_backend = "vulkan",
                webview_backend = "webview2",
                timing_method = "cpu",
                "frame duration recorded",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["duration_ms"], 16.7);
        assert_eq!(fields["wgpu_backend"], "vulkan");
        assert_eq!(fields["webview_backend"], "webview2");
        assert_eq!(fields["timing_method"], "cpu");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_metric_webgpu_frame_duration_ms_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "metric.webgpu.frame_duration_ms",
                duration_ms = 33.0,
                wgpu_backend = "metal",
                webview_backend = "wkwebview",
                timing_method = "gpu",
                trace_id = "0xDEADBEEF000000000000000000000000",
                "frame duration recorded",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["duration_ms"], 33.0);
        assert_eq!(fields["wgpu_backend"], "metal");
        assert_eq!(
            fields["trace_id"], "<redacted>",
            "raw trace_id MUST NOT leak per Vector 6 cardinality discipline"
        );
    }

    #[test]
    fn scrubber_passes_app_boot_tray_init_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "app.boot.tray.init",
                tray_api = "AppIndicator",
                "tray API selected at boot",
            );
        });
        assert_eq!(lines[0]["fields"]["tray_api"], "AppIndicator");
    }

    #[test]
    fn scrubber_passes_ui_layout_transition_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "ui.layout.transition",
                layout_mode_from = "main",
                layout_mode_to = "hidden",
                tray_visible = true,
                "window close→minimize-to-tray",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["layout_mode_from"], "main");
        assert_eq!(fields["layout_mode_to"], "hidden");
        assert_eq!(fields["tray_visible"], true);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_ui_layout_transition_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "ui.layout.transition",
                layout_mode_from = "compact-widget",
                layout_mode_to = "hidden",
                tray_visible = true,
                window_title = "instrumented-app: secret-customer-name",
                "window close→minimize-to-tray",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["layout_mode_from"], "compact-widget");
        assert_eq!(
            fields["window_title"], "<redacted>",
            "user-app window title MUST NOT leak per Vector 1 (titles can carry instrumented-app names)"
        );
    }

    #[test]
    fn scrubber_passes_app_boot_window_show_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "app.boot.window.show",
                label = "compact-widget",
                error_kind = "show_failed",
                error_msg = "device not ready",
                "failed to show compact-widget window",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["label"], "compact-widget");
        assert_eq!(fields["error_kind"], "show_failed");
        assert_eq!(fields["error_msg"], "device not ready");
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_app_boot_window_show_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::warn!(
                target: "app.boot.window.show",
                label = "compact-widget",
                error_kind = "not_found",
                user_home_path = "/home/alice/private",
                "compact-widget window not registered",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["label"], "compact-widget");
        assert_eq!(
            fields["user_home_path"], "<redacted>",
            "absolute user paths MUST NOT leak per security plan §Logging Vector 2"
        );
    }

    #[test]
    fn scrubber_passes_tray_visibility_toggle_fields() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "tray.visibility.toggle",
                tray_visible = true,
                "tray icon shown",
            );
        });
        assert_eq!(lines[0]["fields"]["tray_visible"], true);
    }

    #[test]
    fn scrubber_redacts_non_allowlisted_tray_visibility_toggle_field() {
        let defaults = make_defaults(None, None);
        let lines = capture_json_lines(defaults, || {
            tracing::info!(
                target: "tray.visibility.toggle",
                tray_visible = true,
                clipboard_snippet = "secret-text-from-user",
                "tray icon shown",
            );
        });
        let fields = &lines[0]["fields"];
        assert_eq!(fields["tray_visible"], true);
        assert_eq!(
            fields["clipboard_snippet"], "<redacted>",
            "clipboard contents MUST NOT leak per security plan §Logging Vector 4"
        );
    }

    #[test]
    fn allowlist_resolver_picks_chunk24_targets() {
        let al = AllowList::production();
        assert!(al.for_target("app.boot.webview.init").is_some());
        assert!(al.for_target("app.boot.gpu.check").is_some());
        assert!(al.for_target("app.boot.tray.init").is_some());
        assert!(al.for_target("app.boot.window.show").is_some());
        assert!(al.for_target("ui.layout.transition").is_some());
        assert!(al.for_target("tray.visibility.toggle").is_some());
        assert!(al.for_target("tray.menu.interaction").is_some());
        assert!(al.for_target("tray.notification.dismiss").is_some());
        assert!(al.for_target("tray.notification.action").is_some());
    }

    #[test]
    fn deployment_environment_returns_known_value() {
        let env_value = resolve_deployment_environment();
        assert!(matches!(
            env_value.as_str(),
            "production" | "staging" | "dev"
        ));
    }

    #[test]
    fn subscriber_init_idempotent_under_test_repetition() {
        // Each invocation builds an isolated subscriber via with_default scope guard,
        // never calling tracing::subscriber::set_global_default. Repeated calls must
        // not panic with "subscriber already set".
        for _ in 0..3 {
            let lines = capture_json_lines(make_defaults(None, None), || {
                tracing::info!(target: "ingest", span_count = 0_i64, "ping");
            });
            assert_eq!(lines.len(), 1);
        }
    }
}
