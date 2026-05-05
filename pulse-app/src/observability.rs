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
                "row_count",
                "latency_ms",
                "query_latency_ms",
                "subscribers_active",
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
