#[cfg(any(feature = "taurpc-runtime", test))]
use crate::contract::{AppError, SnapshotPreset};

#[cfg(any(feature = "taurpc-runtime", test))]
const PLACEHOLDER_MESSAGE: &str = "Investigate not yet wired (lands chunk #43)";

#[cfg(feature = "taurpc-runtime")]
mod runtime {
    use super::*;

    #[taurpc::procedures(path = "snapshot")]
    pub trait SnapshotApi {
        async fn generate(preset: SnapshotPreset) -> Result<(), AppError>;
    }

    #[derive(Clone, Default)]
    pub struct SnapshotApiImpl;

    impl SnapshotApiImpl {
        pub fn new() -> Self {
            Self
        }
    }

    #[taurpc::resolvers]
    impl SnapshotApi for SnapshotApiImpl {
        async fn generate(self, preset: SnapshotPreset) -> Result<(), AppError> {
            let budget = match preset {
                SnapshotPreset::Conservative => "conservative",
                SnapshotPreset::Balanced => "balanced",
                SnapshotPreset::Detailed => "detailed",
            };
            tracing::warn!(
                target: "snapshot.generate.request",
                budget = budget,
                result_kind = "placeholder",
                message = PLACEHOLDER_MESSAGE,
            );
            Err(AppError::Internal {
                message: PLACEHOLDER_MESSAGE.to_string(),
            })
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

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn generate_emits_warn_at_snapshot_generate_request_target() {
        let events: Arc<Mutex<Vec<(String, tracing::Level)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        let guard = tracing::subscriber::set_default(subscriber);

        let api = SnapshotApiImpl::new();
        let result = api.generate(SnapshotPreset::Balanced).await;
        drop(guard);

        assert!(matches!(result, Err(AppError::Internal { .. })));
        let captured = events.lock().expect("lock").clone();
        assert!(
            captured
                .iter()
                .any(|(target, level)| target == "snapshot.generate.request"
                    && *level == tracing::Level::WARN),
            "expected snapshot.generate.request WARN event; captured: {captured:?}",
        );
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn generate_returns_sanitized_internal_error_without_path_or_struct_name() {
        let api = SnapshotApiImpl::new();
        let result = api.generate(SnapshotPreset::Conservative).await;
        match result {
            Err(AppError::Internal { message }) => {
                assert!(
                    message.contains("Investigate not yet wired"),
                    "expected placeholder message, got: {message}",
                );
                for forbidden in ["panic", "unwrap", "at /", "at C:\\", "::"] {
                    assert!(
                        !message.contains(forbidden),
                        "sanitized message must not contain `{forbidden}`; got: {message}",
                    );
                }
            }
            other => panic!("expected AppError::Internal, got {other:?}"),
        }
    }

    #[cfg(feature = "taurpc-runtime")]
    #[tokio::test]
    async fn generate_accepts_all_three_presets() {
        for preset in [
            SnapshotPreset::Conservative,
            SnapshotPreset::Balanced,
            SnapshotPreset::Detailed,
        ] {
            let api = SnapshotApiImpl::new();
            let result = api.generate(preset).await;
            assert!(
                matches!(result, Err(AppError::Internal { .. })),
                "expected placeholder Internal error for preset {preset:?}, got {result:?}",
            );
        }
    }
}
