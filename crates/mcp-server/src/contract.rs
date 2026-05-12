use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("MCP feature not enabled at compile time: {reason}")]
    FeatureNotEnabled { reason: &'static str },

    #[error("ANDROMEDA_PULSE_MCP_ENABLED is unset or not truthy; sidecar disabled")]
    EnvVarDisabled,

    #[error("rmcp server init failed: {detail}")]
    RmcpInit { detail: String },

    #[error("JSON-RPC 2.0 framing error: {detail}")]
    JsonRpcFraming { detail: String },

    #[error("I/O error during sidecar boot or run loop")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("tracing-subscriber init failed: {detail}")]
    TracingInit { detail: String },

    #[error("tool dispatch failed for `{tool_name}`: {reason}")]
    ToolDispatchFailed { tool_name: String, reason: String },

    #[error("tool arguments invalid for `{tool_name}`: {reason}")]
    ToolArgsInvalid { tool_name: String, reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_not_enabled_renders_with_reason() {
        let e = Error::FeatureNotEnabled {
            reason: "compile-time cfg(feature = \"mcp-server\") not set",
        };
        let rendered = format!("{e}");
        assert!(rendered.contains("MCP feature not enabled"));
        assert!(rendered.contains("compile-time cfg"));
    }

    #[test]
    fn env_var_disabled_renders_canonical_message() {
        let e = Error::EnvVarDisabled;
        let rendered = format!("{e}");
        assert!(rendered.contains("ANDROMEDA_PULSE_MCP_ENABLED"));
        assert!(rendered.contains("disabled"));
    }

    #[test]
    fn rmcp_init_renders_with_detail() {
        let e = Error::RmcpInit {
            detail: "stdio transport unavailable".into(),
        };
        let rendered = format!("{e}");
        assert!(rendered.contains("rmcp server init failed"));
        assert!(rendered.contains("stdio transport unavailable"));
    }

    #[test]
    fn json_rpc_framing_renders_with_detail() {
        let e = Error::JsonRpcFraming {
            detail: "missing jsonrpc field".into(),
        };
        let rendered = format!("{e}");
        assert!(rendered.contains("JSON-RPC 2.0 framing error"));
        assert!(rendered.contains("missing jsonrpc field"));
    }

    #[test]
    fn io_wraps_std_io_error_via_from() {
        let inner = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "stdin closed");
        let e: Error = inner.into();
        let rendered = format!("{e}");
        assert!(rendered.contains("I/O error"));
    }

    #[test]
    fn tracing_init_renders_with_detail() {
        let e = Error::TracingInit {
            detail: "subscriber already set".into(),
        };
        let rendered = format!("{e}");
        assert!(rendered.contains("tracing-subscriber init failed"));
        assert!(rendered.contains("subscriber already set"));
    }

    #[test]
    fn error_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    #[test]
    fn tool_dispatch_failed_renders_with_tool_name_and_reason() {
        let e = Error::ToolDispatchFailed {
            tool_name: "query_traces".into(),
            reason: "buffer empty".into(),
        };
        let rendered = format!("{e}");
        assert!(rendered.contains("tool dispatch failed"));
        assert!(rendered.contains("query_traces"));
        assert!(rendered.contains("buffer empty"));
    }

    #[test]
    fn tool_args_invalid_renders_with_tool_name_and_reason() {
        let e = Error::ToolArgsInvalid {
            tool_name: "generate_snapshot".into(),
            reason: "missing token_budget".into(),
        };
        let rendered = format!("{e}");
        assert!(rendered.contains("tool arguments invalid"));
        assert!(rendered.contains("generate_snapshot"));
        assert!(rendered.contains("missing token_budget"));
    }
}
