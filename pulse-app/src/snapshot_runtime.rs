// Snapshot.generate runtime — relocated from `crates/ui-bridge/src/snapshot_ipc.rs`
// per chunk #44 architectural decision (matches viz_routers precedent for
// resolvers that need both AppHandle + buffer Connection). Library crate
// `crates/snapshot/` stays Tauri-free per arch §Cross-cutting Patterns Module
// dependency direction; the AppHandle binding lives in `pulse-app/` (the only
// crate permitted to depend on the Tauri runtime).
//
// AppHandle injection is deferred via `Arc<OnceLock<AppHandle<Wry>>>` because
// `SnapshotApiImpl` is constructed at router-build time (before the Tauri
// setup closure runs); `pulse-app/src/main.rs` populates the OnceLock inside
// the setup closure via `set_app_handle()` on a sibling clone. The clones
// share the same Arc so the resolver sees the populated handle by the time
// the first IPC call lands.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use chrono::Utc;
use duckdb::Connection;
use snapshot::contract::{
    CurationOutput, FormatError, SpanRecord, TokenBudget, curate, format_markdown,
};
use tauri::{AppHandle, Emitter, Wry};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_notification::NotificationExt;
use ui_bridge::contract::{AppError, PresetPromptDto, Settings, SnapshotPreset, SnapshotResultDto};

// Spans table is lean (chunk #44 reality per crates/buffer/src/schema.rs):
// 7 columns; no parent_span_id / name / attributes — those default to None /
// empty when SpanRecord is reconstructed. Deeper attribute storage is a
// future-chunk scope.
const SELECT_SPANS_RECENT: &str = "\
    SELECT trace_id, span_id, ts_unix_nano, service_name, end_time_unix_nano, status_code \
    FROM spans \
    WHERE ts_unix_nano >= ? \
    ORDER BY ts_unix_nano DESC \
    LIMIT ?";

const SPANS_RECENT_LIMIT: usize = 5000;

// 5-minute window per arch §Inherited Defaults retention range (300–600s
// default). Snapshot pulls a recent slice rather than the whole buffer so
// the curation pipeline stays bounded.
const SNAPSHOT_TIME_WINDOW_NS: i64 = 300 * 1_000_000_000;

const PRESET_PROMPT_DEFINITIONS: &[(&str, &str)] = &[
    ("diagnose-latency-outlier", "Diagnose latency outlier"),
    ("find-error-correlation", "Find error correlation"),
    ("trace-failed-request", "Trace failed request"),
    ("summarize-service-health", "Summarize service health"),
];

fn preset_prompts() -> Vec<PresetPromptDto> {
    PRESET_PROMPT_DEFINITIONS
        .iter()
        .map(|(id, label)| PresetPromptDto {
            id: (*id).to_string(),
            label: (*label).to_string(),
        })
        .collect()
}

fn preset_label(preset: SnapshotPreset) -> &'static str {
    match preset {
        SnapshotPreset::Conservative => "conservative",
        SnapshotPreset::Balanced => "balanced",
        SnapshotPreset::Detailed => "detailed",
    }
}

fn preset_to_budget(preset: SnapshotPreset) -> TokenBudget {
    match preset {
        SnapshotPreset::Conservative => TokenBudget::Conservative,
        SnapshotPreset::Balanced => TokenBudget::Balanced,
        SnapshotPreset::Detailed => TokenBudget::Detailed,
    }
}

fn load_recent_spans(
    conn: &Connection,
    since_ns: i64,
    limit: usize,
) -> Result<Vec<SpanRecord>, AppError> {
    let mut stmt = conn
        .prepare(SELECT_SPANS_RECENT)
        .map_err(|_| AppError::Storage {
            message: "snapshot: prepare failed".to_string(),
        })?;
    let limit_i64 = i64::try_from(limit).unwrap_or(i64::MAX);
    let rows = stmt
        .query_map(duckdb::params![since_ns, limit_i64], |row| {
            let trace_id_blob: Vec<u8> = row.get(0)?;
            let span_id_blob: Vec<u8> = row.get(1)?;
            let start_ns: i64 = row.get(2)?;
            let service_name: String = row.get(3)?;
            let end_ns: i64 = row.get(4)?;
            let status: i32 = row.get(5)?;

            let mut trace_id = [0u8; 16];
            let mut span_id = [0u8; 8];
            if trace_id_blob.len() == 16 {
                trace_id.copy_from_slice(&trace_id_blob);
            }
            if span_id_blob.len() == 8 {
                span_id.copy_from_slice(&span_id_blob);
            }

            Ok(SpanRecord {
                trace_id,
                span_id,
                parent_span_id: None,
                service_name,
                name: String::new(),
                start_time_unix_nano: start_ns,
                end_time_unix_nano: end_ns,
                status_code: u8::try_from(status).unwrap_or(0),
                attributes: Vec::new(),
            })
        })
        .map_err(|_| AppError::Storage {
            message: "snapshot: query failed".to_string(),
        })?;

    rows.collect::<Result<Vec<SpanRecord>, duckdb::Error>>()
        .map_err(|_| AppError::Storage {
            message: "snapshot: decode failed".to_string(),
        })
}

/// Load the recent span window, curate it, and render the curated markdown
/// context. Shared by `snapshot.generate` and `investigate.run_action` (P-072)
/// so both consume one telemetry-context path. An empty buffer yields a bounded
/// near-empty markdown (curate returns the default `CurationOutput`) — still a
/// valid context, not an error.
pub(crate) fn load_curated_markdown(conn: &Connection) -> Result<String, AppError> {
    let now_ns = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let since_ns = now_ns.saturating_sub(SNAPSHOT_TIME_WINDOW_NS);
    let spans = load_recent_spans(conn, since_ns, SPANS_RECENT_LIMIT)?;
    let curated: CurationOutput = curate(&spans)?;
    let report = format_markdown(&curated, TokenBudget::Balanced).map_err(|_: FormatError| {
        AppError::Internal {
            message: "snapshot: format failed".to_string(),
        }
    })?;
    Ok(report.markdown)
}

#[taurpc::procedures(path = "snapshot")]
pub trait SnapshotApi {
    async fn generate(
        preset: SnapshotPreset,
        workspace_root: Option<String>,
    ) -> Result<SnapshotResultDto, AppError>;
}

#[derive(Clone)]
pub struct SnapshotApiImpl {
    conn: Option<Arc<Mutex<Connection>>>,
    data_dir: PathBuf,
    app_handle: Arc<OnceLock<AppHandle<Wry>>>,
}

impl SnapshotApiImpl {
    pub fn new(conn: Option<Arc<Mutex<Connection>>>, data_dir: PathBuf) -> Self {
        Self {
            conn,
            data_dir,
            app_handle: Arc::new(OnceLock::new()),
        }
    }

    pub fn set_app_handle(&self, handle: AppHandle<Wry>) {
        let _ = self.app_handle.set(handle);
    }
}

#[taurpc::resolvers]
impl SnapshotApi for SnapshotApiImpl {
    #[tracing::instrument(skip_all)]
    async fn generate(
        self,
        preset: SnapshotPreset,
        workspace_root: Option<String>,
    ) -> Result<SnapshotResultDto, AppError> {
        // workspace_root currently informational only (chunk #44 Open Q3
        // resolution: deferred). Resolver accepts it for forward-compat
        // with future workspace integration but does not act on it.
        let _ = workspace_root;
        let budget_label = preset_label(preset);
        let budget = preset_to_budget(preset);

        let conn = self.conn.ok_or(AppError::Storage {
            message: "buffer unavailable".to_string(),
        })?;
        let data_dir = self.data_dir.clone();

        let now_ns = Utc::now().timestamp_nanos_opt().unwrap_or(0);
        let since_ns = now_ns.saturating_sub(SNAPSHOT_TIME_WINDOW_NS);

        let conn_for_load = Arc::clone(&conn);
        let spans = tokio::task::spawn_blocking(move || {
            let guard = conn_for_load.lock().map_err(|_| AppError::Storage {
                message: "snapshot: lock poisoned".to_string(),
            })?;
            load_recent_spans(&guard, since_ns, SPANS_RECENT_LIMIT)
        })
        .await
        .map_err(|_| AppError::internal("snapshot: query task failed"))??;

        let curated: CurationOutput = curate(&spans)?;
        let report = format_markdown(&curated, budget).map_err(|e: FormatError| {
            // Sanitized per security plan §Error Handling — internals
            // (kinds / limits) do not cross the bridge.
            let _ = e;
            AppError::Internal {
                message: "snapshot: format failed".to_string(),
            }
        })?;

        let snapshot_id = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
        let snapshots_dir = data_dir.join("snapshots");
        std::fs::create_dir_all(&snapshots_dir).map_err(|_| AppError::Storage {
            message: "snapshot: create_dir_all failed".to_string(),
        })?;
        let md_basename = format!("{}.md", snapshot_id);
        let json_basename = format!("{}.json", snapshot_id);
        let md_path = snapshots_dir.join(&md_basename);
        let json_path = snapshots_dir.join(&json_basename);

        std::fs::write(&md_path, &report.markdown).map_err(|_| AppError::Storage {
            message: "snapshot: write md failed".to_string(),
        })?;
        let curation_json =
            serde_json::to_string_pretty(&curated).map_err(|_| AppError::Internal {
                message: "snapshot: serialize failed".to_string(),
            })?;
        std::fs::write(&json_path, &curation_json).map_err(|_| AppError::Storage {
            message: "snapshot: write json failed".to_string(),
        })?;

        let byte_count = u64::try_from(report.markdown.len()).unwrap_or(0);
        let token_count = u64::try_from(report.token_count).unwrap_or(0);
        let dedup_count = u64::try_from(curated.dedup_count).unwrap_or(0);
        let unique_span_count = curated.unique_spans.len();
        let prompts = preset_prompts();
        let dual_file_paths_basenames = format!("[{},{}]", md_basename, json_basename);

        // AppHandle-dependent ops — best-effort. Failures surface as
        // success=false in the canonical tracing target rather than
        // propagating, so partial completion remains visible.
        let mut clipboard_success = false;
        let mut dispatch_success = false;
        if let Some(handle) = self.app_handle.get() {
            clipboard_success = handle
                .clipboard()
                .write_text(report.markdown.clone())
                .is_ok();

            let _ = handle.emit(
                "pulse://stream/snapshot-progress",
                serde_json::json!({
                    "stage": "clipboard_written",
                    "byte_count": byte_count,
                    "snapshot_id": &snapshot_id,
                }),
            );

            let settings = Settings::load_from_data_dir(&data_dir);
            if settings.notifications_enabled {
                dispatch_success = handle
                    .notification()
                    .builder()
                    .title("Snapshot ready")
                    .body(format!(
                        "{} tokens, {} spans",
                        token_count, unique_span_count
                    ))
                    .show()
                    .is_ok();
            }
        }

        tracing::info!(
            target: "snapshot.clipboard.write",
            byte_count = byte_count,
            success = clipboard_success,
            dual_file_paths_basenames = %dual_file_paths_basenames,
        );
        tracing::info!(
            target: "snapshot.notification.dispatch",
            notification_kind = "snapshot_ready",
            dispatch_success = dispatch_success,
        );
        tracing::info!(
            target: "snapshot.generate.request",
            budget = budget_label,
            result_kind = "success",
            token_count = token_count,
            dual_file_paths_basenames = %dual_file_paths_basenames,
            preset_prompts_count = prompts.len() as u64,
        );

        Ok(SnapshotResultDto {
            token_count,
            markdown_path_basename: md_basename,
            json_path_basename: json_basename,
            preset_prompts: prompts,
            byte_count,
            dedup_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tracing::field::{Field, Visit};
    use tracing::span::{Attributes, Id, Record};
    use tracing::{Event, Metadata, Subscriber};

    // CapturingSubscriber: extends the chunk #43 substrate pattern by
    // recording field values (not just target + level) so PII negative-
    // canary tests can substring-search captured field strings.
    struct CapturingSubscriber {
        events: Arc<Mutex<Vec<(String, String)>>>,
    }

    struct FieldCollector {
        sink: String,
    }

    impl Visit for FieldCollector {
        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            use std::fmt::Write;
            let _ = write!(&mut self.sink, " {}={:?}", field.name(), value);
        }
        fn record_str(&mut self, field: &Field, value: &str) {
            use std::fmt::Write;
            let _ = write!(&mut self.sink, " {}={}", field.name(), value);
        }
        fn record_bool(&mut self, field: &Field, value: bool) {
            use std::fmt::Write;
            let _ = write!(&mut self.sink, " {}={}", field.name(), value);
        }
        fn record_u64(&mut self, field: &Field, value: u64) {
            use std::fmt::Write;
            let _ = write!(&mut self.sink, " {}={}", field.name(), value);
        }
        fn record_i64(&mut self, field: &Field, value: i64) {
            use std::fmt::Write;
            let _ = write!(&mut self.sink, " {}={}", field.name(), value);
        }
    }

    impl Subscriber for CapturingSubscriber {
        fn enabled(&self, _: &Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &Attributes<'_>) -> Id {
            Id::from_u64(1)
        }
        fn record(&self, _: &Id, _: &Record<'_>) {}
        fn record_follows_from(&self, _: &Id, _: &Id) {}
        fn event(&self, event: &Event<'_>) {
            let metadata = event.metadata();
            let mut collector = FieldCollector {
                sink: String::new(),
            };
            event.record(&mut collector);
            self.events
                .lock()
                .expect("event lock not poisoned")
                .push((metadata.target().to_string(), collector.sink));
        }
        fn enter(&self, _: &Id) {}
        fn exit(&self, _: &Id) {}
    }

    fn seed_span(conn: &Connection, service_name: &str, ts_ns: i64, span_byte: u8) {
        conn.execute(
            "INSERT INTO spans (trace_id, span_id, ts, ts_unix_nano, service_name, end_time_unix_nano, status_code) VALUES (?, ?, '2026-05-11T00:00:00Z'::TIMESTAMPTZ, ?, ?, ?, ?)",
            duckdb::params![
                vec![1u8; 16],
                vec![span_byte; 8],
                ts_ns,
                service_name,
                ts_ns + 1_000_000,
                0i32,
            ],
        )
        .expect("insert span");
    }

    #[test]
    fn preset_prompts_returns_all_four_canonical_entries() {
        let p = preset_prompts();
        assert_eq!(p.len(), 4);
        let ids: Vec<&str> = p.iter().map(|d| d.id.as_str()).collect();
        assert!(ids.contains(&"diagnose-latency-outlier"));
        assert!(ids.contains(&"find-error-correlation"));
        assert!(ids.contains(&"trace-failed-request"));
        assert!(ids.contains(&"summarize-service-health"));
    }

    #[test]
    fn preset_label_maps_each_variant_to_lowercase_string() {
        assert_eq!(preset_label(SnapshotPreset::Conservative), "conservative");
        assert_eq!(preset_label(SnapshotPreset::Balanced), "balanced");
        assert_eq!(preset_label(SnapshotPreset::Detailed), "detailed");
    }

    #[test]
    fn preset_to_budget_maps_each_variant_to_matching_budget() {
        assert_eq!(
            preset_to_budget(SnapshotPreset::Conservative),
            TokenBudget::Conservative
        );
        assert_eq!(
            preset_to_budget(SnapshotPreset::Balanced),
            TokenBudget::Balanced
        );
        assert_eq!(
            preset_to_budget(SnapshotPreset::Detailed),
            TokenBudget::Detailed
        );
    }

    #[tokio::test]
    async fn generate_returns_storage_error_when_conn_is_none() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir");
        let impl_ = SnapshotApiImpl::new(None, temp_dir.path().to_path_buf());
        match impl_.generate(SnapshotPreset::Balanced, None).await {
            Err(AppError::Storage { message }) => assert_eq!(message, "buffer unavailable"),
            other => panic!("expected Storage(buffer unavailable), got {other:?}"),
        }
    }

    #[tokio::test]
    async fn generate_emits_three_canonical_tracing_targets_on_success() {
        let conn = Connection::open_in_memory().expect("in-memory DuckDB");
        buffer::create_schema(&conn).expect("schema create");
        let now_ns = chrono::Utc::now()
            .timestamp_nanos_opt()
            .expect("nanos in range");
        seed_span(&conn, "checkout", now_ns - 1_000_000_000, 1);
        seed_span(&conn, "checkout", now_ns - 2_000_000_000, 2);

        let temp_dir = tempfile::TempDir::new().expect("tempdir");
        let impl_ = SnapshotApiImpl::new(
            Some(Arc::new(Mutex::new(conn))),
            temp_dir.path().to_path_buf(),
        );

        let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        let guard = tracing::subscriber::set_default(subscriber);
        let dto = impl_
            .generate(SnapshotPreset::Balanced, None)
            .await
            .expect("generate ok");
        drop(guard);

        assert_eq!(dto.preset_prompts.len(), 4);
        assert!(dto.markdown_path_basename.ends_with(".md"));
        assert!(dto.json_path_basename.ends_with(".json"));

        let captured = events.lock().expect("lock").clone();
        for required in [
            "snapshot.generate.request",
            "snapshot.clipboard.write",
            "snapshot.notification.dispatch",
        ] {
            assert!(
                captured.iter().any(|(t, _)| t == required),
                "expected target `{required}` in captured events: {captured:?}"
            );
        }
    }

    #[tokio::test]
    async fn generate_writes_dual_md_and_json_files_under_data_dir_snapshots() {
        let conn = Connection::open_in_memory().expect("in-memory DuckDB");
        buffer::create_schema(&conn).expect("schema create");

        let temp_dir = tempfile::TempDir::new().expect("tempdir");
        let impl_ = SnapshotApiImpl::new(
            Some(Arc::new(Mutex::new(conn))),
            temp_dir.path().to_path_buf(),
        );
        let dto = impl_
            .generate(SnapshotPreset::Conservative, None)
            .await
            .expect("generate ok");

        let snapshots_dir = temp_dir.path().join("snapshots");
        assert!(snapshots_dir.exists(), "snapshots dir must be created");
        assert!(
            snapshots_dir.join(&dto.markdown_path_basename).exists(),
            "md file `{}` must exist under {snapshots_dir:?}",
            dto.markdown_path_basename
        );
        assert!(
            snapshots_dir.join(&dto.json_path_basename).exists(),
            "json file `{}` must exist under {snapshots_dir:?}",
            dto.json_path_basename
        );
    }

    #[tokio::test]
    async fn generate_does_not_log_snapshot_or_clipboard_or_workspace_path_canaries() {
        let canary = "secret-canary-API-key-12345-service";
        let conn = Connection::open_in_memory().expect("in-memory DuckDB");
        buffer::create_schema(&conn).expect("schema create");
        let now_ns = chrono::Utc::now()
            .timestamp_nanos_opt()
            .expect("nanos in range");
        seed_span(&conn, canary, now_ns - 1_000_000_000, 1);

        let temp_dir = tempfile::TempDir::new().expect("tempdir");
        let impl_ = SnapshotApiImpl::new(
            Some(Arc::new(Mutex::new(conn))),
            temp_dir.path().to_path_buf(),
        );

        let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        let guard = tracing::subscriber::set_default(subscriber);
        let _ = impl_.generate(SnapshotPreset::Balanced, None).await;
        drop(guard);

        let captured = events.lock().expect("lock").clone();
        for (target, fields) in captured.iter() {
            assert!(
                !target.contains(canary),
                "canary leaked into tracing target: {target}"
            );
            assert!(
                !fields.contains(canary),
                "canary leaked into tracing fields ({target}): {fields}"
            );
        }
    }
}
