#[cfg(any(feature = "taurpc-runtime", test))]
use crate::contract::{AppError, WorkspaceContextDto};

#[cfg(any(feature = "taurpc-runtime", test))]
use workspace_detector::{VcsType, WorkspaceContext};

#[cfg(any(feature = "taurpc-runtime", test))]
fn vcs_type_label(t: VcsType) -> &'static str {
    match t {
        VcsType::Git => "git",
    }
}

#[cfg(any(feature = "taurpc-runtime", test))]
fn ctx_to_dto(ctx: WorkspaceContext) -> WorkspaceContextDto {
    let root_basename = ctx
        .root
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    let vcs_root_basename = ctx
        .vcs
        .as_ref()
        .and_then(|v| v.vcs_root.file_name())
        .and_then(|s| s.to_str())
        .map(|s| s.to_string());
    let vcs_type = ctx.vcs.as_ref().map(|v| vcs_type_label(v.vcs_type).to_string());

    WorkspaceContextDto {
        root_basename,
        project_name: ctx.project_name,
        vcs_type,
        vcs_root_basename,
        has_andromeda_marker: ctx.has_andromeda_marker,
    }
}

#[cfg(feature = "taurpc-runtime")]
mod runtime {
    use super::*;
    use std::path::Path;

    #[taurpc::procedures(path = "workspace")]
    pub trait WorkspaceApi {
        async fn detect(candidate_root: String) -> Result<WorkspaceContextDto, AppError>;
    }

    #[derive(Clone, Default)]
    pub struct WorkspaceApiImpl;

    impl WorkspaceApiImpl {
        pub fn new() -> Self {
            Self
        }
    }

    #[taurpc::resolvers]
    impl WorkspaceApi for WorkspaceApiImpl {
        #[tracing::instrument(skip_all)]
        async fn detect(self, candidate_root: String) -> Result<WorkspaceContextDto, AppError> {
            let started = std::time::Instant::now();
            let ctx = workspace_detector::detect(Path::new(&candidate_root))?;
            let dto = ctx_to_dto(ctx);
            let detection_latency_ms = started.elapsed().as_millis() as u64;
            tracing::info!(
                target: "workspace.detect",
                workspace_root_basename = %dto.root_basename,
                project_name = ?dto.project_name,
                vcs_type = ?dto.vcs_type,
                vcs_root_basename = ?dto.vcs_root_basename,
                marker_present = dto.has_andromeda_marker,
                detection_latency_ms = detection_latency_ms,
            );
            Ok(dto)
        }
    }
}

#[cfg(feature = "taurpc-runtime")]
pub use runtime::{WorkspaceApi, WorkspaceApiImpl};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Arc, Mutex};
    use tempfile::TempDir;
    use tracing::span::{Attributes, Id, Record};
    use tracing::{Event, Metadata, Subscriber};
    use workspace_detector::{VcsMetadata, VcsType, WorkspaceContext};

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
    fn ctx_to_dto_extracts_basenames_only() {
        let ctx = WorkspaceContext {
            root: std::path::PathBuf::from("/tmp/secret-dir-canary/example"),
            project_name: Some("example".to_string()),
            vcs: Some(VcsMetadata {
                vcs_type: VcsType::Git,
                vcs_root: std::path::PathBuf::from("/tmp/secret-dir-canary/example"),
                head_commit_basename: Some("12345678".to_string()),
            }),
            has_andromeda_marker: true,
        };
        let dto = ctx_to_dto(ctx);
        assert_eq!(dto.root_basename, "example");
        assert_eq!(dto.vcs_root_basename.as_deref(), Some("example"));
        assert_eq!(dto.vcs_type.as_deref(), Some("git"));
        assert!(dto.has_andromeda_marker);
        // Verify no full path leaks in either field
        assert!(!dto.root_basename.contains("secret-dir-canary"));
        assert!(!dto.vcs_root_basename.unwrap_or_default().contains("secret-dir-canary"));
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn detect_emits_info_at_workspace_detect_target_for_marker_dir() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".andromeda")).unwrap();

        let events: Arc<Mutex<Vec<(String, tracing::Level)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        let guard = tracing::subscriber::set_default(subscriber);

        let api = WorkspaceApiImpl::new();
        let result = api.detect(tmp.path().to_string_lossy().to_string()).await;
        drop(guard);

        let dto = result.expect("detect ok");
        assert!(dto.has_andromeda_marker);

        let captured = events.lock().expect("lock").clone();
        assert!(
            captured
                .iter()
                .any(|(target, level)| target == "workspace.detect"
                    && *level == tracing::Level::INFO),
            "expected workspace.detect INFO event; captured: {captured:?}",
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn detect_returns_validation_appror_for_traversal_path() {
        let api = WorkspaceApiImpl::new();
        let result = api.detect("/tmp/foo/../bar".to_string()).await;
        match result {
            Err(AppError::Validation { field, reason }) => {
                assert_eq!(field, "candidate_root");
                assert!(reason.contains("traversal"));
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn detect_returns_validation_appror_for_nonexistent_path() {
        let api = WorkspaceApiImpl::new();
        let result = api
            .detect("/nonexistent-path-xyz-canary-123".to_string())
            .await;
        match result {
            Err(AppError::Validation { field, .. }) => {
                assert_eq!(field, "candidate_root");
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }
}
