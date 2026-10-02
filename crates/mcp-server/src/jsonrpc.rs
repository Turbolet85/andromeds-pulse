use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::contract::Error;

pub const JSONRPC_VERSION: &str = "2.0";
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

pub const CODE_PARSE_ERROR: i64 = -32700;
pub const CODE_INVALID_REQUEST: i64 = -32600;
pub const CODE_METHOD_NOT_FOUND: i64 = -32601;
pub const CODE_INVALID_PARAMS: i64 = -32602;
pub const CODE_INTERNAL_ERROR: i64 = -32603;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub jsonrpc: &'static str,
    pub id: Value,
    pub result: Value,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub jsonrpc: &'static str,
    pub id: Value,
    pub error: ErrorObject,
}

#[derive(Debug, Serialize)]
pub struct ErrorObject {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

pub fn parse_request(line: &str) -> Result<Request, Error> {
    serde_json::from_str::<Request>(line).map_err(|e| Error::JsonRpcFraming {
        detail: format!("parse: {e}"),
    })
}

pub fn success(id: Value, result: Value) -> SuccessResponse {
    SuccessResponse {
        jsonrpc: JSONRPC_VERSION,
        id,
        result,
    }
}

pub fn error(id: Value, code: i64, message: impl Into<String>) -> ErrorResponse {
    ErrorResponse {
        jsonrpc: JSONRPC_VERSION,
        id,
        error: ErrorObject {
            code,
            message: message.into(),
            data: None,
        },
    }
}

pub fn sanitize_error_data(raw: &str) -> Value {
    let cleaned: String = raw
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return String::new();
            }
            // Strip path-like sequences (anything resembling /a/b/c.rs or C:\a\b),
            // version-like tokens (vX.Y.Z), and Rust type paths (crate::module::Type).
            let mut out = String::new();
            for token in trimmed.split_whitespace() {
                if token.contains('/')
                    || token.contains('\\')
                    || token.contains("::")
                    || token.starts_with('v') && token.chars().any(|c| c.is_ascii_digit())
                {
                    out.push_str("[redacted] ");
                } else {
                    out.push_str(token);
                    out.push(' ');
                }
            }
            out.trim_end().to_string()
        })
        .collect::<Vec<_>>()
        .join(" ");
    json!({ "summary": cleaned.trim() })
}

pub fn initialize_result() -> Value {
    json!({
        "protocolVersion": MCP_PROTOCOL_VERSION,
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": "andromeda-pulse-mcp",
            "version": env!("CARGO_PKG_VERSION"),
        }
    })
}

pub fn empty_tools_list() -> Value {
    json!({ "tools": [] })
}

// Enumerate the #[tool] methods exposed by the rmcp sidecar. Schema follows
// the MCP `Tool` shape (name + description + inputSchema). Input schemas are
// JSON Schema draft-07 fragments; `additionalProperties: false` rejects
// unknown args for `tools/call`. Chunk #49 shipped the first 4 (live-buffer
// query tools); chunk #94 adds the 4 corpus-backed incident/report tools.
pub fn tools_list_manifest() -> Value {
    json!({
        "tools": [
            {
                "name": "query_traces",
                "description": "Query recent OTLP traces from the in-memory buffer. Returns paginated trace rows with trace_id, span_id, service, duration_ms, error_count.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "time_window_seconds": { "type": "integer", "minimum": 1, "default": 300 },
                        "limit": { "type": "integer", "minimum": 1, "maximum": 1000, "default": 100 },
                        "cursor": { "type": "string" }
                    },
                    "additionalProperties": false
                }
            },
            {
                "name": "query_metrics",
                "description": "Query recent OTLP metric points from the in-memory buffer. Returns paginated metric rows with metric_name, ts_unix_nano, value, data_point_kind.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "time_window_seconds": { "type": "integer", "minimum": 1, "default": 300 },
                        "limit": { "type": "integer", "minimum": 1, "maximum": 1000, "default": 100 },
                        "cursor": { "type": "string" }
                    },
                    "additionalProperties": false
                }
            },
            {
                "name": "query_logs",
                "description": "Query recent OTLP log records from the in-memory buffer. Returns paginated log rows with severity, body, trace correlation.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "time_window_seconds": { "type": "integer", "minimum": 1, "default": 300 },
                        "limit": { "type": "integer", "minimum": 1, "maximum": 1000, "default": 100 },
                        "cursor": { "type": "string" }
                    },
                    "additionalProperties": false
                }
            },
            {
                "name": "generate_snapshot",
                "description": "Generate a curated markdown snapshot of recent telemetry. Pipeline: dedupe → anomaly highlight → critical path → p50/p95/p99 aggregates → token-budget markdown. Token budget snaps to Conservative (10k), Balanced (25k), or Detailed (50k) preset.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "token_budget": { "type": "integer", "minimum": 1000, "default": 25000 },
                        "time_window_seconds": { "type": "integer", "minimum": 1, "default": 300 }
                    },
                    "additionalProperties": false
                }
            },
            {
                "name": "query_incident_list",
                "description": "List active incidents from the persistent corpus for the current workspace. Returns id, status, severity, title, opened_at per incident.",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }
            },
            {
                "name": "retrieve_report",
                "description": "Retrieve the six-section Diagnostic Report markdown for an incident by id. Byte-identical to the in-app Copy markdown action. Resolved incidents render full hypotheses + investigation steps; active incidents render a degraded-mode notice.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "incident_id": { "type": "integer" }
                    },
                    "required": ["incident_id"],
                    "additionalProperties": false
                }
            },
            {
                "name": "retrieve_telemetry_slice",
                "description": "Retrieve the persisted incident context for an incident by id: span and fingerprint evidence references plus timestamps. Persisted context only — live telemetry rows are served by the separate query_traces / query_logs tools against the live buffer.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "incident_id": { "type": "integer" }
                    },
                    "required": ["incident_id"],
                    "additionalProperties": false
                }
            },
            {
                "name": "mark_incident_resolved",
                "description": "Mark an incident resolved in the persistent corpus by id. The main app's in-memory registry reflects the change on next launch (eventual consistency).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "incident_id": { "type": "integer" }
                    },
                    "required": ["incident_id"],
                    "additionalProperties": false
                }
            },
            {
                "name": "retrieve_incident_events",
                "description": "Retrieve the lifecycle events of one incident by id: its status transitions, oldest first, each with event_kind (active, acknowledged, resolved) and occurred_unix_nano. Bounded; truncated is true when more exist. Creation records no event, so an incident read before its first status change returns none.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "incident_id": { "type": "integer" }
                    },
                    "required": ["incident_id"],
                    "additionalProperties": false
                }
            }
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_request_accepts_minimal_initialize() {
        let line = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let req = parse_request(line).expect("parse ok");
        assert_eq!(req.jsonrpc, JSONRPC_VERSION);
        assert_eq!(req.method, "initialize");
        assert!(req.id.is_some());
    }

    #[test]
    fn parse_request_accepts_notification_without_id() {
        let line = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
        let req = parse_request(line).expect("notification parses without id");
        assert!(req.id.is_none());
        assert_eq!(req.method, "notifications/initialized");
    }

    #[test]
    fn parse_request_rejects_garbage() {
        let result = parse_request("not json at all");
        assert!(matches!(result, Err(Error::JsonRpcFraming { .. })));
    }

    #[test]
    fn parse_request_rejects_missing_method() {
        let line = r#"{"jsonrpc":"2.0","id":1}"#;
        let result = parse_request(line);
        assert!(matches!(result, Err(Error::JsonRpcFraming { .. })));
    }

    #[test]
    fn success_response_uses_canonical_version() {
        let resp = success(json!(1), json!({"ok": true}));
        let serialized = serde_json::to_string(&resp).expect("serialize");
        assert!(serialized.contains(r#""jsonrpc":"2.0""#));
        assert!(serialized.contains(r#""ok":true"#));
    }

    #[test]
    fn error_response_includes_code_and_message() {
        let resp = error(json!(2), CODE_METHOD_NOT_FOUND, "method not found");
        let serialized = serde_json::to_string(&resp).expect("serialize");
        assert!(serialized.contains(r#""code":-32601"#));
        assert!(serialized.contains(r#""message":"method not found""#));
    }

    #[test]
    fn error_response_omits_data_when_none() {
        let resp = error(json!(3), CODE_INVALID_REQUEST, "bad");
        let serialized = serde_json::to_string(&resp).expect("serialize");
        assert!(!serialized.contains(r#""data""#));
    }

    #[test]
    fn sanitize_error_data_redacts_unix_paths() {
        let raw = "panic at /home/user/.cargo/registry/src/foo.rs line 42";
        let sanitized = sanitize_error_data(raw);
        let summary = sanitized
            .get("summary")
            .and_then(|v| v.as_str())
            .expect("summary present");
        assert!(!summary.contains("/home/user"));
        assert!(!summary.contains("foo.rs"));
        assert!(summary.contains("[redacted]"));
    }

    #[test]
    fn sanitize_error_data_redacts_windows_paths() {
        let raw = r"panic at C:\Users\user\.cargo\registry\src\foo.rs";
        let sanitized = sanitize_error_data(raw);
        let summary = sanitized
            .get("summary")
            .and_then(|v| v.as_str())
            .expect("summary present");
        assert!(!summary.contains(r"C:\Users"));
        assert!(summary.contains("[redacted]"));
    }

    #[test]
    fn sanitize_error_data_redacts_rust_type_paths() {
        let raw = "error in crate::module::Type::method";
        let sanitized = sanitize_error_data(raw);
        let summary = sanitized
            .get("summary")
            .and_then(|v| v.as_str())
            .expect("summary present");
        assert!(!summary.contains("crate::module::Type"));
        assert!(summary.contains("[redacted]"));
    }

    #[test]
    fn sanitize_error_data_redacts_version_strings() {
        let raw = "rmcp v0.6.1 failed";
        let sanitized = sanitize_error_data(raw);
        let summary = sanitized
            .get("summary")
            .and_then(|v| v.as_str())
            .expect("summary present");
        assert!(!summary.contains("v0.6.1"));
        assert!(summary.contains("[redacted]"));
    }

    #[test]
    fn sanitize_error_data_returns_object_with_summary_field() {
        let sanitized = sanitize_error_data("benign message");
        assert!(sanitized.is_object());
        assert!(sanitized.get("summary").is_some());
    }

    #[test]
    fn initialize_result_advertises_protocol_version_and_server_info() {
        let r = initialize_result();
        assert_eq!(r["protocolVersion"], MCP_PROTOCOL_VERSION);
        assert_eq!(r["serverInfo"]["name"], "andromeda-pulse-mcp");
    }

    #[test]
    fn empty_tools_list_returns_array() {
        let r = empty_tools_list();
        assert!(r["tools"].is_array());
        assert_eq!(r["tools"].as_array().expect("array").len(), 0);
    }

    #[test]
    fn tools_list_manifest_returns_every_named_tool() {
        let r = tools_list_manifest();
        let arr = r["tools"].as_array().expect("array");
        assert_eq!(arr.len(), 9);
        let names: Vec<&str> = arr
            .iter()
            .map(|t| t["name"].as_str().expect("name"))
            .collect();
        assert!(names.contains(&"query_traces"));
        assert!(names.contains(&"query_metrics"));
        assert!(names.contains(&"query_logs"));
        assert!(names.contains(&"generate_snapshot"));
        assert!(names.contains(&"query_incident_list"));
        assert!(names.contains(&"retrieve_report"));
        assert!(names.contains(&"retrieve_telemetry_slice"));
        assert!(names.contains(&"mark_incident_resolved"));
        assert!(names.contains(&"retrieve_incident_events"));
    }

    #[test]
    fn tools_list_manifest_each_has_description_and_input_schema() {
        let r = tools_list_manifest();
        for tool in r["tools"].as_array().expect("array") {
            assert!(tool["description"].as_str().is_some());
            assert!(tool["inputSchema"]["type"].as_str() == Some("object"));
            assert!(tool["inputSchema"]["additionalProperties"].as_bool() == Some(false));
        }
    }

    #[test]
    fn tools_list_manifest_query_schemas_share_time_window_property() {
        let r = tools_list_manifest();
        for name in ["query_traces", "query_metrics", "query_logs"] {
            let tool = r["tools"]
                .as_array()
                .expect("array")
                .iter()
                .find(|t| t["name"] == name)
                .unwrap_or_else(|| panic!("tool {name} present"));
            let props = tool["inputSchema"]["properties"]
                .as_object()
                .expect("props object");
            assert!(props.contains_key("time_window_seconds"));
            assert!(props.contains_key("limit"));
        }
    }

    #[test]
    fn tools_list_manifest_incident_id_tools_require_incident_id() {
        let r = tools_list_manifest();
        for name in [
            "retrieve_report",
            "retrieve_telemetry_slice",
            "mark_incident_resolved",
            "retrieve_incident_events",
        ] {
            let tool = r["tools"]
                .as_array()
                .expect("array")
                .iter()
                .find(|t| t["name"] == name)
                .unwrap_or_else(|| panic!("tool {name} present"));
            let required = tool["inputSchema"]["required"]
                .as_array()
                .expect("required array");
            assert!(required.iter().any(|v| v == "incident_id"));
        }
    }
}
