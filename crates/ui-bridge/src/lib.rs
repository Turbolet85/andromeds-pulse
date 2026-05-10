pub mod contract;
pub mod health;
pub mod snapshot_ipc;
pub mod telemetry;
pub mod workspace_ipc;

pub use contract::{
    AppError, AppInfo, PresetPromptDto, ReadyChecks, ReadyEnvelope, Settings, SnapshotResultDto,
    Theme, WidgetPosition, WorkspaceContextDto,
};
pub use health::{HealthEnvelope, HealthStatus, SubsystemStatus, SubsystemStatuses};
pub use telemetry::{
    FrameDurationInput, TimingMethod, WebviewBackend, WgpuBackend, validate_duration_ms,
};

#[cfg(feature = "taurpc-runtime")]
pub use health::{IntrospectionApi, IntrospectionApiImpl};
#[cfg(feature = "taurpc-runtime")]
pub use snapshot_ipc::{SnapshotApi, SnapshotApiImpl};
#[cfg(feature = "taurpc-runtime")]
pub use telemetry::{TelemetryApi, TelemetryApiImpl};
#[cfg(feature = "taurpc-runtime")]
pub use workspace_ipc::{WorkspaceApi, WorkspaceApiImpl};
