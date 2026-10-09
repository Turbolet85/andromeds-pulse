use std::sync::{Arc, Mutex};

use duckdb::Connection;
use ui_bridge::AppError;
use viz::query::read_connection;
use viz::{
    LogRow, LogsQueryArgs, MetricRow, MetricsQueryArgs, PaginatedResponse, TraceRow,
    TracesQueryArgs, VizState, query_logs, query_metrics, query_traces,
};

#[taurpc::procedures(path = "traces")]
pub trait TracesApi {
    async fn query(args: TracesQueryArgs) -> Result<PaginatedResponse<TraceRow>, AppError>;
}

#[derive(Clone)]
pub struct TracesApiImpl {
    conn: Arc<Mutex<Connection>>,
    state: Arc<VizState>,
}

impl TracesApiImpl {
    pub fn new(conn: Arc<Mutex<Connection>>, state: Arc<VizState>) -> Self {
        Self {
            conn: read_connection(&conn),
            state,
        }
    }
}

#[taurpc::resolvers]
impl TracesApi for TracesApiImpl {
    async fn query(self, args: TracesQueryArgs) -> Result<PaginatedResponse<TraceRow>, AppError> {
        let conn = Arc::clone(&self.conn);
        let state = Arc::clone(&self.state);
        let join = tokio::task::spawn_blocking(move || query_traces(&conn, &state, &args)).await;
        match join {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(AppError::from(e)),
            Err(_) => Err(AppError::internal("query task failed")),
        }
    }
}

#[taurpc::procedures(path = "metrics")]
pub trait MetricsApi {
    async fn query(args: MetricsQueryArgs) -> Result<PaginatedResponse<MetricRow>, AppError>;
}

#[derive(Clone)]
pub struct MetricsApiImpl {
    conn: Arc<Mutex<Connection>>,
    state: Arc<VizState>,
}

impl MetricsApiImpl {
    pub fn new(conn: Arc<Mutex<Connection>>, state: Arc<VizState>) -> Self {
        Self {
            conn: read_connection(&conn),
            state,
        }
    }
}

#[taurpc::resolvers]
impl MetricsApi for MetricsApiImpl {
    async fn query(self, args: MetricsQueryArgs) -> Result<PaginatedResponse<MetricRow>, AppError> {
        let conn = Arc::clone(&self.conn);
        let state = Arc::clone(&self.state);
        let join = tokio::task::spawn_blocking(move || query_metrics(&conn, &state, &args)).await;
        match join {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(AppError::from(e)),
            Err(_) => Err(AppError::internal("query task failed")),
        }
    }
}

#[taurpc::procedures(path = "logs")]
pub trait LogsApi {
    async fn query(args: LogsQueryArgs) -> Result<PaginatedResponse<LogRow>, AppError>;
}

#[derive(Clone)]
pub struct LogsApiImpl {
    conn: Arc<Mutex<Connection>>,
    state: Arc<VizState>,
}

impl LogsApiImpl {
    pub fn new(conn: Arc<Mutex<Connection>>, state: Arc<VizState>) -> Self {
        Self {
            conn: read_connection(&conn),
            state,
        }
    }
}

#[taurpc::resolvers]
impl LogsApi for LogsApiImpl {
    async fn query(self, args: LogsQueryArgs) -> Result<PaginatedResponse<LogRow>, AppError> {
        let conn = Arc::clone(&self.conn);
        let state = Arc::clone(&self.state);
        let join = tokio::task::spawn_blocking(move || query_logs(&conn, &state, &args)).await;
        match join {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(AppError::from(e)),
            Err(_) => Err(AppError::internal("query task failed")),
        }
    }
}
