pub mod contract;
pub mod health;
pub mod snapshot_ipc;
pub mod telemetry;

pub use contract::{
    AppError, AppInfo, ReadyChecks, ReadyEnvelope, Settings, Theme, WidgetPosition,
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
