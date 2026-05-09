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
            ]
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
            ["panic_message", "location", "spantrace"]
                .iter()
                .copied()
                .collect(),
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
    std::panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown".to_string());
        let msg = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("(non-string panic payload)");
        tracing::error!(
            target: "app.panic.fatal",
            panic_message = %msg,
            location = %location,
            spantrace = ?SpanTrace::capture(),
            "panic captured",
        );
    }));
}

pub(crate) fn init(data_dir: &Path) -> WorkerGuard {
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
