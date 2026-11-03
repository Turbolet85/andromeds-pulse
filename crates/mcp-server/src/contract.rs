use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON-RPC 2.0 framing error: {detail}")]
    JsonRpcFraming { detail: String },

    #[error("tool dispatch failed for `{tool_name}`: {reason}")]
    ToolDispatchFailed { tool_name: String, reason: String },

    #[error("tool arguments invalid for `{tool_name}`: {reason}")]
    ToolArgsInvalid { tool_name: String, reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;

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
