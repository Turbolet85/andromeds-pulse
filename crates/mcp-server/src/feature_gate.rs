use std::env;

pub const ENV_MCP_ENABLED: &str = "ANDROMEDA_PULSE_MCP_ENABLED";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateState {
    Enabled,
    EnvDisabled,
    FeatureMissing,
}

pub fn feature_flag_enabled() -> bool {
    cfg!(feature = "mcp-server")
}

pub fn env_var_mcp_enabled() -> bool {
    match env::var(ENV_MCP_ENABLED) {
        Ok(raw) => matches!(
            raw.trim().to_ascii_lowercase().as_str(),
            "true" | "1" | "yes"
        ),
        Err(_) => false,
    }
}

pub fn validate_double_gate() -> GateState {
    let feat = feature_flag_enabled();
    let env_on = env_var_mcp_enabled();

    if feat && env_on {
        tracing::info!(
            target: "mcp.feature.gate.check",
            feature_flag_enabled = true,
            env_var_mcp_enabled = true,
            reason = "both_gates_active",
            "MCP double-gate validated; sidecar enabling stdio loop",
        );
        GateState::Enabled
    } else if !feat && env_on {
        tracing::warn!(
            target: "mcp.feature.gate.check",
            feature_flag_enabled = false,
            env_var_mcp_enabled = true,
            reason = "feature_missing",
            "ANDROMEDA_PULSE_MCP_ENABLED=true set against a binary built without --features mcp-server; proceeding with MCP disabled",
        );
        GateState::FeatureMissing
    } else {
        tracing::info!(
            target: "mcp.feature.gate.check",
            feature_flag_enabled = feat,
            env_var_mcp_enabled = false,
            reason = "env_disabled",
            "MCP sidecar disabled (env var unset or not truthy)",
        );
        GateState::EnvDisabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tracing::field::{Field, Visit};
    use tracing::{Level, Subscriber};

    struct CapturingSubscriber {
        events: Arc<Mutex<Vec<(String, Level, String)>>>,
    }

    struct FieldCollector {
        sink: String,
    }

    impl Visit for FieldCollector {
        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            use std::fmt::Write;
            let _ = write!(self.sink, "{}={:?} ", field.name(), value);
        }

        fn record_str(&mut self, field: &Field, value: &str) {
            use std::fmt::Write;
            let _ = write!(self.sink, "{}={} ", field.name(), value);
        }

        fn record_bool(&mut self, field: &Field, value: bool) {
            use std::fmt::Write;
            let _ = write!(self.sink, "{}={} ", field.name(), value);
        }
    }

    impl Subscriber for CapturingSubscriber {
        fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }
        fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
        fn event(&self, event: &tracing::Event<'_>) {
            let meta = event.metadata();
            let mut collector = FieldCollector {
                sink: String::new(),
            };
            event.record(&mut collector);
            self.events.lock().expect("lock").push((
                meta.target().to_string(),
                *meta.level(),
                collector.sink,
            ));
        }
        fn enter(&self, _: &tracing::span::Id) {}
        fn exit(&self, _: &tracing::span::Id) {}
    }

    fn capture<F: FnOnce()>(f: F) -> Vec<(String, Level, String)> {
        let events = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: Arc::clone(&events),
        };
        tracing::subscriber::with_default(subscriber, f);
        events.lock().expect("lock").clone()
    }

    #[test]
    fn env_var_mcp_enabled_returns_false_when_unset() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(!env_var_mcp_enabled());
    }

    #[test]
    fn env_var_mcp_enabled_returns_true_for_literal_true() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "true");
        }
        let result = env_var_mcp_enabled();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(result);
    }

    #[test]
    fn env_var_mcp_enabled_returns_true_for_one() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "1");
        }
        let result = env_var_mcp_enabled();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(result);
    }

    #[test]
    fn env_var_mcp_enabled_returns_true_for_yes_case_insensitive() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "YES");
        }
        let result = env_var_mcp_enabled();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(result);
    }

    #[test]
    fn env_var_mcp_enabled_returns_false_for_literal_false() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "false");
        }
        let result = env_var_mcp_enabled();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(!result);
    }

    #[test]
    fn env_var_mcp_enabled_returns_false_for_malformed() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "truth");
        }
        let result = env_var_mcp_enabled();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(!result);
    }

    #[test]
    fn env_var_mcp_enabled_returns_false_for_empty_string() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "");
        }
        let result = env_var_mcp_enabled();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(!result);
    }

    #[test]
    fn env_var_mcp_enabled_trims_whitespace() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "  true  ");
        }
        let result = env_var_mcp_enabled();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(result);
    }

    #[test]
    fn feature_flag_enabled_matches_cfg() {
        let expected = cfg!(feature = "mcp-server");
        assert_eq!(feature_flag_enabled(), expected);
    }

    #[test]
    fn validate_double_gate_emits_check_span_when_env_disabled() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        let captured = capture(|| {
            let state = validate_double_gate();
            assert_eq!(state, GateState::EnvDisabled);
        });
        assert!(
            captured
                .iter()
                .any(|(t, _, _)| t == "mcp.feature.gate.check"),
            "expected mcp.feature.gate.check event; captured: {captured:?}"
        );
    }

    #[test]
    fn validate_double_gate_emits_warn_when_env_set_without_feature() {
        if cfg!(feature = "mcp-server") {
            return;
        }
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "true");
        }
        let captured = capture(|| {
            let state = validate_double_gate();
            assert_eq!(state, GateState::FeatureMissing);
        });
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(
            captured
                .iter()
                .any(|(t, lvl, _)| t == "mcp.feature.gate.check" && *lvl == Level::WARN),
            "expected mcp.feature.gate.check WARN event; captured: {captured:?}"
        );
    }

    #[test]
    fn validate_double_gate_returns_enabled_when_both_gates_active() {
        if !cfg!(feature = "mcp-server") {
            return;
        }
        const TEST_ENV: &str = "ANDROMEDA_PULSE_MCP_ENABLED";
        unsafe {
            std::env::set_var(TEST_ENV, "true");
        }
        let state = validate_double_gate();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert_eq!(state, GateState::Enabled);
    }
}
