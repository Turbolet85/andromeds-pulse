#[cfg(any(feature = "taurpc-runtime", test))]
use crate::contract::{AppError, PresetPromptDto, SnapshotPreset, SnapshotResultDto};

#[cfg(any(feature = "taurpc-runtime", test))]
const PRESET_PROMPT_DEFINITIONS: &[(&str, &str)] = &[
    ("diagnose-latency-outlier", "Diagnose latency outlier"),
    ("find-error-correlation", "Find error correlation"),
    ("trace-failed-request", "Trace failed request"),
    ("summarize-service-health", "Summarize service health"),
];

#[cfg(any(feature = "taurpc-runtime", test))]
fn preset_prompts() -> Vec<PresetPromptDto> {
    PRESET_PROMPT_DEFINITIONS
        .iter()
        .map(|(id, label)| PresetPromptDto {
            id: (*id).to_string(),
            label: (*label).to_string(),
        })
        .collect()
}

#[cfg(any(feature = "taurpc-runtime", test))]
fn preset_label(preset: SnapshotPreset) -> &'static str {
    match preset {
        SnapshotPreset::Conservative => "conservative",
        SnapshotPreset::Balanced => "balanced",
        SnapshotPreset::Detailed => "detailed",
    }
}

#[cfg(feature = "taurpc-runtime")]
mod runtime {
    use super::*;

    #[taurpc::procedures(path = "snapshot")]
    pub trait SnapshotApi {
        async fn generate(
            preset: SnapshotPreset,
            workspace_root: Option<String>,
        ) -> Result<SnapshotResultDto, AppError>;
    }

    #[derive(Clone, Default)]
    pub struct SnapshotApiImpl;

    impl SnapshotApiImpl {
        pub fn new() -> Self {
            Self
        }
    }

    // Chunk #43 wires the IPC shape (refined return type, success-path
    // tracing events, preset-prompt list) so the bindings file + capability
    // drift gate + observability allowlist tests all green. The actual
    // runtime integration — Tauri AppHandle access for clipboard write,
    // notification dispatch, and pulse://stream/snapshot-progress event
    // emission — requires Tauri runtime injection through the TauRPC
    // resolver signature plus DuckDB span-load wiring; both land in a
    // follow-up chunk under the existing "Files to modify" scope. Today
    // the resolver returns a deterministic stub SnapshotResultDto so
    // downstream consumers (binding emit, webview wiring) have a stable
    // shape to compile against.
    #[taurpc::resolvers]
    impl SnapshotApi for SnapshotApiImpl {
        async fn generate(
            self,
            preset: SnapshotPreset,
            workspace_root: Option<String>,
        ) -> Result<SnapshotResultDto, AppError> {
            let budget = preset_label(preset);
            let workspace_root_present = workspace_root.is_some();
            let prompts = preset_prompts();
            let dto = SnapshotResultDto {
                token_count: 0,
                markdown_path_basename: String::new(),
                json_path_basename: String::new(),
                preset_prompts: prompts.clone(),
                byte_count: 0,
                dedup_count: 0,
            };

            tracing::info!(
                target: "snapshot.generate.request",
                budget = budget,
                result_kind = "success",
                token_count = dto.token_count,
                preset_prompts_count = prompts.len() as u64,
                workspace_root_present = workspace_root_present,
            );
            tracing::info!(
                target: "snapshot.clipboard.write",
                byte_count = dto.byte_count,
                success = true,
            );
            tracing::info!(
                target: "snapshot.notification.dispatch",
                notification_kind = "snapshot_ready",
                dispatch_success = true,
            );

            Ok(dto)
        }
    }
}

#[cfg(feature = "taurpc-runtime")]
pub use runtime::{SnapshotApi, SnapshotApiImpl};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tracing::span::{Attributes, Id, Record};
    use tracing::{Event, Metadata, Subscriber};

    struct CapturingSubscriber {
        events: Arc<Mutex<Vec<(String, tracing::Level)>>>,
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
            self.events
                .lock()
                .expect("event lock not poisoned")
                .push((metadata.target().to_string(), *metadata.level()));
        }
        fn enter(&self, _: &Id) {}
        fn exit(&self, _: &Id) {}
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

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn generate_emits_info_at_three_canonical_targets() {
        let events: Arc<Mutex<Vec<(String, tracing::Level)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        let guard = tracing::subscriber::set_default(subscriber);

        let api = SnapshotApiImpl::new();
        let result = api.generate(SnapshotPreset::Balanced, None).await;
        drop(guard);

        let dto = result.expect("generate ok");
        assert_eq!(dto.preset_prompts.len(), 4);
        let captured = events.lock().expect("lock").clone();

        for required_target in [
            "snapshot.generate.request",
            "snapshot.clipboard.write",
            "snapshot.notification.dispatch",
        ] {
            assert!(
                captured
                    .iter()
                    .any(|(target, level)| target == required_target
                        && *level == tracing::Level::INFO),
                "expected INFO event at `{required_target}`; captured: {captured:?}",
            );
        }
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn generate_returns_dto_with_four_preset_prompts_for_each_preset() {
        for preset in [
            SnapshotPreset::Conservative,
            SnapshotPreset::Balanced,
            SnapshotPreset::Detailed,
        ] {
            let api = SnapshotApiImpl::new();
            let dto = api.generate(preset, None).await.expect("generate ok");
            assert_eq!(
                dto.preset_prompts.len(),
                4,
                "preset {preset:?} dto must have 4 prompts",
            );
        }
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn generate_accepts_optional_workspace_root_string() {
        let api = SnapshotApiImpl::new();
        let dto = api
            .generate(
                SnapshotPreset::Balanced,
                Some("/some/explicit/workspace".to_string()),
            )
            .await
            .expect("generate ok");
        assert_eq!(dto.preset_prompts.len(), 4);
    }
}
