//! `SqlQueryRunner` adapter over `triage::contract::TriageSqlState`
//! (chunk #80). Lives at the binary boundary per arch §Cross-cutting
//! Patterns "Module dependency direction" — the trait declaration in
//! `crates/triage/src/cadence/coordinator.rs` keeps the leaf crate free
//! of sibling-crate deps; this adapter wires the concrete chunk #79
//! Q1-Q7 async API into the cadence coordinator's trait-injected runner
//! slot (per CLAUDE.md 2026-05-16 trait-in-lower-crate session learning).

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use triage::contract::{
    Q1RedRow, Q2OperationRow, Q3FingerprintRow, Q4InteractionRow, Q5CardinalityRow, Q6LogRow,
    Q7CriticalPathRow, SqlAggregationError, SqlQueryRunner, TriageSqlState, run_q1, run_q2, run_q3,
    run_q4, run_q5, run_q6, run_q7,
};

type SqlFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, SqlAggregationError>> + Send + 'a>>;

pub struct CadenceSqlRunner {
    state: Arc<TriageSqlState>,
}

impl CadenceSqlRunner {
    pub fn new(state: Arc<TriageSqlState>) -> Self {
        Self { state }
    }
}

impl SqlQueryRunner for CadenceSqlRunner {
    fn run_q1<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q1RedRow>> {
        Box::pin(async move { run_q1(&self.state, window).await })
    }
    fn run_q2<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q2OperationRow>> {
        Box::pin(async move { run_q2(&self.state, window).await })
    }
    fn run_q3<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q3FingerprintRow>> {
        Box::pin(async move { run_q3(&self.state, window).await })
    }
    fn run_q4<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q4InteractionRow>> {
        Box::pin(async move { run_q4(&self.state, window).await })
    }
    fn run_q5<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q5CardinalityRow>> {
        Box::pin(async move { run_q5(&self.state, window).await })
    }
    fn run_q6<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q6LogRow>> {
        Box::pin(async move { run_q6(&self.state, window).await })
    }
    fn run_q7<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q7CriticalPathRow>> {
        Box::pin(async move { run_q7(&self.state, window).await })
    }
}
