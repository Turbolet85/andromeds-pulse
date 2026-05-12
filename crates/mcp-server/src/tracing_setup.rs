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

use crate::contract::Error;

pub const SERVICE_NAME: &str = "andromeda-pulse-mcp";

#[derive(Clone)]
pub(crate) struct DefaultFields {
    service_name: &'static str,
    service_version: &'static str,
    deployment_environment: String,
}

impl DefaultFields {
    pub(crate) fn from_env() -> Self {
        let deployment_environment = match env::var("GITHUB_REF") {
            Ok(r) if r.starts_with("refs/tags/v") => "production".into(),
            Ok(r) if r.starts_with("refs/tags/alpha") || r.starts_with("refs/tags/beta") => {
                "staging".into()
            }
            Ok(_) => "dev".into(),
            Err(_) => "production".into(),
        };
        DefaultFields {
            service_name: SERVICE_NAME,
            service_version: env!("CARGO_PKG_VERSION"),
            deployment_environment,
        }
    }
}

pub(crate) struct AllowList {
    by_target: HashMap<&'static str, HashSet<&'static str>>,
}

impl AllowList {
    pub(crate) fn for_mcp_server() -> Self {
        let mut by_target: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
        // Chunk #48 baseline (feature gate + framing) + chunk #49 extensions
        // (tool dispatch fields). New sub-targets like mcp.tools.call.request
        // cascade via split('.').next() → "mcp" so a single entry covers them.
        by_target.insert(
            "mcp",
            [
                "feature_flag_enabled",
                "env_var_mcp_enabled",
                "reason",
                "protocol_version",
                "method",
                "request_id",
                "params_count",
                "result_type",
                "result_count",
                "duration_ms",
                // Chunk #49 — tool dispatch fields
                "tool_name",
                "query_id",
                "param_count",
                "traceparent",
                "error_detail",
                "tool_name_unknown",
            ]
            .iter()
            .copied()
            .collect(),
        );
        by_target.insert(
            "app.panic.fatal",
            ["message", "location", "spantrace"]
                .iter()
                .copied()
                .collect(),
        );
        // Chunk #49 — per-tool latency metric event (cardinality bounded к 4
        // enumerated tool names per obs plan §5 Metric label cardinality discipline).
        by_target.insert(
            "metric.mcp.tool_call_duration_ms",
            ["value", "method", "result_count"]
                .iter()
                .copied()
                .collect(),
        );
        AllowList { by_target }
    }

    pub(crate) fn for_target(&self, target: &str) -> Option<&HashSet<&'static str>> {
        if let Some(set) = self.by_target.get(target) {
            return Some(set);
        }
        if let Some(stripped) = target.strip_suffix(".tick") {
            if let Some(set) = self.by_target.get(stripped) {
                return Some(set);
            }
        }
        if let Some((first, _)) = target.split_once('.') {
            if let Some(set) = self.by_target.get(first) {
                return Some(set);
            }
        }
        if let Some((first, _)) = target.split_once("::") {
            if let Some(set) = self.by_target.get(first) {
                return Some(set);
            }
        }
        None
    }
}

struct JsonFieldVisitor<'a> {
    output: &'a mut Map<String, Value>,
    allowed: Option<&'a HashSet<&'static str>>,
}

impl<'a> JsonFieldVisitor<'a> {
    fn allowed(&self, name: &str) -> bool {
        if name == "message" {
            return true;
        }
        match self.allowed {
            Some(set) => set.contains(name),
            None => false,
        }
    }

    fn store_str(&mut self, name: &str, value: String) {
        if self.allowed(name) {
            self.output.insert(name.to_string(), Value::String(value));
        } else {
            self.output
                .insert(name.to_string(), Value::String("[redacted]".into()));
        }
    }

    fn store_bool(&mut self, name: &str, value: bool) {
        if self.allowed(name) {
            self.output.insert(name.to_string(), Value::Bool(value));
        } else {
            self.output
                .insert(name.to_string(), Value::String("[redacted]".into()));
        }
    }

    fn store_number(&mut self, name: &str, value: serde_json::Number) {
        if self.allowed(name) {
            self.output.insert(name.to_string(), Value::Number(value));
        } else {
            self.output
                .insert(name.to_string(), Value::String("[redacted]".into()));
        }
    }
}

impl<'a> Visit for JsonFieldVisitor<'a> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.store_str(field.name(), value.to_string());
    }
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.store_bool(field.name(), value);
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.store_number(field.name(), value.into());
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.store_number(field.name(), value.into());
    }
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        let formatted = format!("{value:?}");
        self.store_str(field.name(), formatted);
    }
}

pub(crate) struct JsonWithDefaults {
    defaults: DefaultFields,
    allow: AllowList,
}

impl<S, N> FormatEvent<S, N> for JsonWithDefaults
where
    S: Subscriber + for<'a> LookupSpan<'a>,
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
        let allowed = self.allow.for_target(target);

        let mut fields = Map::new();
        let mut visitor = JsonFieldVisitor {
            output: &mut fields,
            allowed,
        };
        event.record(&mut visitor);

        let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        let mut line = Map::new();
        line.insert("timestamp".into(), Value::String(timestamp));
        line.insert("level".into(), Value::String(metadata.level().to_string()));
        line.insert("target".into(), Value::String(target.to_string()));
        line.insert(
            "service.name".into(),
            Value::String(self.defaults.service_name.to_string()),
        );
        line.insert(
            "service.version".into(),
            Value::String(self.defaults.service_version.to_string()),
        );
        line.insert(
            "deployment.environment".into(),
            Value::String(self.defaults.deployment_environment.clone()),
        );
        if let Some(message) = fields.remove("message") {
            line.insert("message".into(), message);
        }
        if !fields.is_empty() {
            line.insert("fields".into(), Value::Object(fields));
        }

        writeln!(writer, "{}", Value::Object(line))
    }
}

pub fn init(data_dir: &Path) -> Result<WorkerGuard, Error> {
    let log_dir = data_dir.join("logs");
    if let Err(e) = fs::create_dir_all(&log_dir) {
        return Err(Error::TracingInit {
            detail: format!("create log dir {}: {e}", log_dir.display()),
        });
    }

    let file_appender = tracing_appender::rolling::daily(&log_dir, "agent-latest.jsonl");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let defaults_stderr = DefaultFields::from_env();
    let defaults_file = defaults_stderr.clone();
    let stderr_event_format = JsonWithDefaults {
        defaults: defaults_stderr,
        allow: AllowList::for_mcp_server(),
    };
    let file_event_format = JsonWithDefaults {
        defaults: defaults_file,
        allow: AllowList::for_mcp_server(),
    };

    let env_filter = EnvFilter::try_from_env("ANDROMEDA_PULSE_LOG_LEVEL")
        .or_else(|_| EnvFilter::try_from_default_env())
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .event_format(stderr_event_format)
                .with_writer(std::io::stderr),
        )
        .with(
            fmt::layer()
                .event_format(file_event_format)
                .with_writer(file_writer),
        )
        .with(ErrorLayer::default());

    registry.try_init().map_err(|e| Error::TracingInit {
        detail: format!("subscriber init: {e}"),
    })?;

    install_panic_hook();
    Ok(guard)
}

fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let message = payload
            .downcast_ref::<&'static str>()
            .copied()
            .map(|s| s.to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "non-string panic payload".to_string());
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());
        let spantrace = SpanTrace::capture();
        tracing::error!(
            target: "app.panic.fatal",
            message = message,
            location = location,
            spantrace = ?spantrace,
            "panic",
        );
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowlist_for_mcp_target_allows_gate_fields() {
        let al = AllowList::for_mcp_server();
        let set = al
            .for_target("mcp.feature.gate.check")
            .expect("mcp.feature.gate.check resolves via split('.').next() → mcp");
        assert!(set.contains("feature_flag_enabled"));
        assert!(set.contains("env_var_mcp_enabled"));
        assert!(set.contains("reason"));
    }

    #[test]
    fn allowlist_for_panic_target_allows_panic_fields() {
        let al = AllowList::for_mcp_server();
        let set = al
            .for_target("app.panic.fatal")
            .expect("app.panic.fatal exact-match");
        assert!(set.contains("message"));
        assert!(set.contains("location"));
        assert!(set.contains("spantrace"));
    }

    #[test]
    fn allowlist_redacts_unknown_target() {
        let al = AllowList::for_mcp_server();
        assert!(al.for_target("unknown.target.name").is_none());
    }

    #[test]
    fn allowlist_for_mcp_tools_call_request_cascades_to_mcp_via_split() {
        let al = AllowList::for_mcp_server();
        let set = al
            .for_target("mcp.tools.call.request")
            .expect("split('.').next() resolves to mcp");
        assert!(set.contains("tool_name"));
        assert!(set.contains("traceparent"));
        assert!(set.contains("duration_ms"));
    }

    #[test]
    fn allowlist_for_mcp_tools_call_response_includes_chunk_49_fields() {
        let al = AllowList::for_mcp_server();
        let set = al
            .for_target("mcp.tools.call.response")
            .expect("split('.').next() resolves to mcp");
        assert!(set.contains("result_type"));
        assert!(set.contains("result_count"));
        assert!(set.contains("duration_ms"));
        assert!(set.contains("tool_name"));
    }

    #[test]
    fn allowlist_for_metric_mcp_tool_call_duration_ms_allows_value_and_method() {
        let al = AllowList::for_mcp_server();
        let set = al
            .for_target("metric.mcp.tool_call_duration_ms")
            .expect("exact-match");
        assert!(set.contains("value"));
        assert!(set.contains("method"));
        assert!(set.contains("result_count"));
    }

    #[test]
    fn default_fields_service_name_is_sidecar() {
        let f = DefaultFields::from_env();
        assert_eq!(f.service_name, SERVICE_NAME);
        assert_eq!(f.service_name, "andromeda-pulse-mcp");
    }

    #[test]
    fn default_fields_service_version_matches_cargo_pkg() {
        let f = DefaultFields::from_env();
        assert_eq!(f.service_version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn default_fields_default_environment_is_production() {
        unsafe {
            std::env::remove_var("GITHUB_REF");
        }
        let f = DefaultFields::from_env();
        assert_eq!(f.deployment_environment, "production");
    }

    #[test]
    fn init_creates_log_dir_when_missing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let data_dir = tmp.path().join("subdir");
        let log_dir = data_dir.join("logs");
        assert!(!log_dir.exists());
        let result = init(&data_dir);
        if result.is_ok() {
            assert!(log_dir.exists());
        }
        // init may fail with "already set" when a sibling test in this process
        // already registered a global subscriber; we only assert the dir-creation
        // side-effect here regardless of init's eventual outcome.
        assert!(log_dir.exists(), "log dir created as side effect");
    }
}
