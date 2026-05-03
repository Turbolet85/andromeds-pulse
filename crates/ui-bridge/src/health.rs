use std::sync::OnceLock;
use std::time::Instant;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "taurpc-runtime")]
use crate::contract::AppError;

static APP_START: OnceLock<Instant> = OnceLock::new();

pub fn record_start() {
    let _ = APP_START.set(Instant::now());
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Ok,
    Degraded,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemStatus {
    pub status: String,
    pub error_msg: Option<String>,
}

impl SubsystemStatus {
    pub fn initialized() -> Self {
        Self {
            status: "initialized".to_string(),
            error_msg: None,
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemStatuses {
    pub otlp_grpc_receiver: SubsystemStatus,
    pub otlp_http_receiver: SubsystemStatus,
    pub buffer: SubsystemStatus,
    pub ingest_channel: SubsystemStatus,
}

impl SubsystemStatuses {
    pub fn placeholders() -> Self {
        Self {
            otlp_grpc_receiver: SubsystemStatus::initialized(),
            otlp_http_receiver: SubsystemStatus::initialized(),
            buffer: SubsystemStatus::initialized(),
            ingest_channel: SubsystemStatus::initialized(),
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEnvelope {
    pub status: HealthStatus,
    pub checked_at: DateTime<Utc>,
    pub subsystems: SubsystemStatuses,
    pub pid: u32,
    pub uptime_ms: u64,
}

pub fn current_health() -> HealthEnvelope {
    let uptime_ms = APP_START
        .get()
        .map(|start| start.elapsed().as_millis() as u64)
        .unwrap_or(0);

    HealthEnvelope {
        status: HealthStatus::Ok,
        checked_at: Utc::now(),
        subsystems: SubsystemStatuses::placeholders(),
        pid: std::process::id(),
        uptime_ms,
    }
}

#[cfg(feature = "taurpc-runtime")]
mod runtime {
    use super::*;

    #[taurpc::procedures(path = "health")]
    pub trait HealthApi {
        async fn check() -> Result<HealthEnvelope, AppError>;
    }

    #[derive(Clone)]
    pub struct HealthApiImpl;

    #[taurpc::resolvers]
    impl HealthApi for HealthApiImpl {
        async fn check(self) -> Result<HealthEnvelope, AppError> {
            Ok(current_health())
        }
    }
}

#[cfg(feature = "taurpc-runtime")]
pub use runtime::{HealthApi, HealthApiImpl};
