use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tokio::sync::watch;

use crate::baseline::{
    Q1RedRow, Q2OperationRow, Q3FingerprintRow, Q4InteractionRow, Q5CardinalityRow, Q6LogRow,
    Q7CriticalPathRow, SqlAggregationError,
};
use crate::cadence::broadcast::{
    CadenceEvent, CadenceEventBroadcast, DigestTrigger, DigestTriggerBroadcast,
};
use crate::cadence::config::CadenceConfig;
use crate::cadence::{
    TARGET_CADENCE_CONFIG_RELOAD_APPLIED, TARGET_CADENCE_TICK, TARGET_CADENCE_TRIGGER,
    TARGET_METRIC_PIPELINE_L3_DIGESTS_ASSEMBLED_TOTAL,
};
use crate::contract::{AttentionCue, CueKind, PriorityTier};
use crate::cue::AttentionCueBroadcast;
use crate::cue::CadenceTriggerChannel;

/// Heartbeat tick interval per obs-plan §3 Heartbeat ticks (15s default).
const HEARTBEAT_TICK_INTERVAL: Duration = Duration::from_secs(15);

/// Cadence dispatch mode per pulse-distillation-architecture.md §Cadence
/// and Event Triggers. Four-value bounded set; serializes as snake_case
/// strings for broadcast payload + obs metric label compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CadenceMode {
    Tier1,
    Tier2,
    Tier3,
    Reflection,
}

/// Static bounded enum label for `CadenceMode`. Used as tracing-field
/// value + broadcast payload `mode_label` + metric mode-label per obs
/// allowlist 4-value enumeration.
pub fn mode_label(mode: CadenceMode) -> &'static str {
    match mode {
        CadenceMode::Tier1 => "tier1",
        CadenceMode::Tier2 => "tier2",
        CadenceMode::Tier3 => "tier3",
        CadenceMode::Reflection => "reflection",
    }
}

/// Hardware profile classification (forward-binding to chunk #82 per
/// pulse-distillation-architecture.md §L4 Hardware Profile Matrix).
/// `Unknown` is the boot-before-#82 default; coordinator treats `Unknown`
/// as Tier-2-enabled per security plan defense-in-depth posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareProfile {
    Unknown,
    GpuPrimary,
    GpuFallback,
    CpuPrimary,
    CpuFallback,
}

/// Cross-crate state delivery trait for hardware profile detection. Per
/// arch §Cross-cutting Patterns "Module dependency direction" — declared
/// in the lower crate (triage) so cadence consumes it via dep-injection
/// from the binary boundary (`pulse-app`) without dragging future chunk
/// #82 implementation crates into the triage dep graph.
pub trait HardwareProfileSource: Send + Sync {
    fn current_profile(&self) -> HardwareProfile;
}

/// Default `HardwareProfileSource` returning `HardwareProfile::Unknown`.
/// Boot-before-#82 safety per security plan §Error Handling boundary
/// discipline (tolerate the chunk #82 profile source being absent by
/// defaulting to the safe posture).
#[derive(Debug, Default)]
pub struct UnknownHardwareProfile;

impl HardwareProfileSource for UnknownHardwareProfile {
    fn current_profile(&self) -> HardwareProfile {
        HardwareProfile::Unknown
    }
}

/// Future return-position alias used by `SqlQueryRunner` methods. Manual
/// `Pin<Box<dyn Future + Send + 'a>>` returns keep the trait object-safe
/// (`Arc<dyn SqlQueryRunner>`) without `async-trait` crate dep.
type SqlFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, SqlAggregationError>> + Send + 'a>>;

/// L1a SQL query runner contract. Concrete impl lives at the binary
/// boundary (`pulse-app/src/cadence_runner.rs::CadenceSqlRunner`)
/// delegating to chunk #79 `crates/triage/src/baseline/sql.rs` Q1-Q7
/// async public API. Tests use an in-module stub mock impl.
pub trait SqlQueryRunner: Send + Sync {
    fn run_q1<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q1RedRow>>;
    fn run_q2<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q2OperationRow>>;
    fn run_q3<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q3FingerprintRow>>;
    fn run_q4<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q4InteractionRow>>;
    fn run_q5<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q5CardinalityRow>>;
    fn run_q6<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q6LogRow>>;
    fn run_q7<'a>(&'a self, window: Duration) -> SqlFuture<'a, Vec<Q7CriticalPathRow>>;
}

/// Per-cycle counter struct surfaced from `run_one_coordinator_cycle` for
/// test assertions + heartbeat tick fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CoordinatorCycleStats {
    pub mode: Option<CadenceMode>,
    pub queries_executed: u32,
    pub queries_succeeded: u32,
    pub digest_emitted: bool,
    pub tier2_skipped_cpu_primary: bool,
}

/// Coordinator orchestrating cadence-driven L1a + L3 + L4 invocations.
/// Composed at the binary boundary with `Arc<dyn SqlQueryRunner>` +
/// `Arc<CadenceEventBroadcast>` + `Arc<AttentionCueBroadcast>` (subscribed
/// for Tier-1 Autonomous cues) + `Arc<CadenceTriggerChannel>` (subscribed
/// for Tier-2 Suggested cues per chunk #62 routing) + `Arc<dyn
/// HardwareProfileSource>` + `Arc<CadenceConfig>`.
#[derive(Debug)]
pub struct CadenceCoordinator {
    pub config: Arc<CadenceConfig>,
}

impl CadenceCoordinator {
    pub fn new(config: Arc<CadenceConfig>) -> Self {
        Self { config }
    }
}

/// Synchronous wrapper for deterministic test-target invocation. Async
/// because the SQL runner methods are async — but callers may pass an
/// explicit `now_nanos` to sidestep wall-clock dependence.
///
/// Tier-2 skip path: if `mode == Tier2 && hw.current_profile() ==
/// CpuPrimary`, returns immediately with `tier2_skipped_cpu_primary = true`
/// and ZERO queries executed (per pulse-distillation-architecture v3
/// §Tier 2 hardware-conditional availability).
pub async fn run_one_coordinator_cycle(
    mode: CadenceMode,
    sql_runner: &Arc<dyn SqlQueryRunner>,
    broadcast_handle: &Arc<CadenceEventBroadcast>,
    hw_profile: &Arc<dyn HardwareProfileSource>,
    now_nanos: i64,
    triggering_cue: Option<&AttentionCue>,
) -> CoordinatorCycleStats {
    let mode_str = mode_label(mode);
    let cue_kind_label_opt = triggering_cue.map(|c| cue_kind_label(c.kind));
    let cue_priority_label_opt = triggering_cue.map(|c| priority_tier_label(c.priority_tier));

    if matches!(mode, CadenceMode::Tier2)
        && matches!(hw_profile.current_profile(), HardwareProfile::CpuPrimary)
    {
        tracing::info!(
            target: TARGET_CADENCE_TRIGGER,
            tier = mode_str,
            mode = mode_str,
            cue_kind = cue_kind_label_opt.unwrap_or(""),
            priority = cue_priority_label_opt.unwrap_or(""),
            "tier2 skipped: cpu-primary profile disables acceleration",
        );
        return CoordinatorCycleStats {
            mode: Some(mode),
            queries_executed: 0,
            queries_succeeded: 0,
            digest_emitted: false,
            tier2_skipped_cpu_primary: true,
        };
    }

    let window = cycle_window_for(mode);
    tracing::info!(
        target: TARGET_CADENCE_TRIGGER,
        tier = mode_str,
        mode = mode_str,
        cue_kind = cue_kind_label_opt.unwrap_or(""),
        priority = cue_priority_label_opt.unwrap_or(""),
        "cadence trigger fired",
    );

    let mut queries_executed: u32 = 0;
    let mut queries_succeeded: u32 = 0;

    macro_rules! drive_query {
        ($call:expr) => {{
            queries_executed += 1;
            if $call.await.is_ok() {
                queries_succeeded += 1;
            }
        }};
    }

    drive_query!(sql_runner.run_q1(window));
    drive_query!(sql_runner.run_q2(window));
    drive_query!(sql_runner.run_q3(window));
    drive_query!(sql_runner.run_q4(window));
    drive_query!(sql_runner.run_q5(window));
    drive_query!(sql_runner.run_q6(window));
    drive_query!(sql_runner.run_q7(window));

    tracing::info!(
        target: TARGET_METRIC_PIPELINE_L3_DIGESTS_ASSEMBLED_TOTAL,
        value = 1_u64,
        mode = mode_str,
        queries_executed = queries_executed as u64,
        "digest assembled",
    );

    let event = CadenceEvent {
        mode,
        mode_label: mode_str,
        scheduled_at_unix_nano: now_nanos,
        executed_at_unix_nano: now_nanos,
        cue_kind_label: cue_kind_label_opt,
        cue_priority_label: cue_priority_label_opt,
        queries_executed,
        queries_succeeded,
    };
    let _ = broadcast_handle.sender().send(event);

    CoordinatorCycleStats {
        mode: Some(mode),
        queries_executed,
        queries_succeeded,
        digest_emitted: true,
        tier2_skipped_cpu_primary: false,
    }
}

/// Per-tier SQL-window k-value lookup. Tier-1/Tier-2 use the baseline
/// 60s window for short-horizon detection; Tier-3 uses the same baseline
/// window; Reflection uses a 30-minute window for cumulative pattern
/// detection per dist-arch v3 §Background reflection cadence.
fn cycle_window_for(mode: CadenceMode) -> Duration {
    match mode {
        CadenceMode::Tier1 | CadenceMode::Tier2 | CadenceMode::Tier3 => Duration::from_secs(60),
        CadenceMode::Reflection => Duration::from_secs(1800),
    }
}

fn cue_kind_label(kind: CueKind) -> &'static str {
    match kind {
        CueKind::ErrorRateSpike => "error_rate_spike",
        CueKind::LatencyRegression => "latency_regression",
        CueKind::RestartEvent => "restart_event",
        CueKind::ServiceWentSilent => "service_went_silent",
        CueKind::RetryStorm => "retry_storm",
        CueKind::ReflectionTrend => "reflection_trend",
    }
}

fn priority_tier_label(tier: PriorityTier) -> &'static str {
    match tier {
        PriorityTier::Autonomous => "autonomous",
        PriorityTier::Suggested => "suggested",
        PriorityTier::Curious => "curious",
    }
}

/// Long-running cadence coordinator task spawned at boot. `tokio::select!`
/// over:
/// - Tier-3 baseline interval at `cfg.baseline_seconds`
/// - Reflection interval at `cfg.reflection_seconds`
/// - Heartbeat interval at 15s (per obs-plan §3 Heartbeat ticks)
/// - `cue_broadcast.subscribe()` filtered for `PriorityTier::Autonomous`
///   → Tier-1 immediate path
/// - `cadence_handle.subscribe()` (Suggested cues per chunk #62) →
///   Tier-2 immediate path (skipped on cpu-primary profile)
#[allow(clippy::too_many_arguments)]
pub async fn start_cadence_coordinator(
    sql_runner: Arc<dyn SqlQueryRunner>,
    broadcast_handle: Arc<CadenceEventBroadcast>,
    digest_trigger: Arc<DigestTriggerBroadcast>,
    cue_broadcast: Arc<AttentionCueBroadcast>,
    cadence_handle: Arc<CadenceTriggerChannel>,
    hw_profile: Arc<dyn HardwareProfileSource>,
    cadence_config: Arc<CadenceConfig>,
    mut config_rx: watch::Receiver<CadenceConfig>,
) {
    // `current` mirrors the latest validated cadence config; the config_rx
    // arm rebuilds the tier-3 + reflection intervals when it changes —
    // prospective: the next tick uses the new rate, missed ticks are not
    // replayed. CadenceConfig is Copy.
    let mut current: CadenceConfig = *cadence_config;
    let mut tier3_interval =
        tokio::time::interval(Duration::from_secs(current.baseline_seconds as u64));
    let mut reflection_interval =
        tokio::time::interval(Duration::from_secs(current.reflection_seconds as u64));
    let mut heartbeat_interval = tokio::time::interval(HEARTBEAT_TICK_INTERVAL);
    // Skip first immediate tick per chunk #62 emitter precedent.
    tier3_interval.tick().await;
    reflection_interval.tick().await;
    heartbeat_interval.tick().await;

    let mut cue_rx = cue_broadcast.subscribe();
    let mut cadence_rx = cadence_handle.subscribe();

    let mut cumulative_cycles: u64 = 0;
    let mut cumulative_queries: u64 = 0;
    let mut config_open = true;

    loop {
        tokio::select! {
            res = config_rx.changed(), if config_open => {
                match res {
                    Ok(()) => {
                        current = *config_rx.borrow_and_update();
                        tier3_interval = tokio::time::interval(Duration::from_secs(
                            current.baseline_seconds as u64,
                        ));
                        reflection_interval = tokio::time::interval(Duration::from_secs(
                            current.reflection_seconds as u64,
                        ));
                        tier3_interval.tick().await;
                        reflection_interval.tick().await;
                        tracing::info!(
                            target: TARGET_CADENCE_CONFIG_RELOAD_APPLIED,
                            baseline_seconds = current.baseline_seconds as u64,
                            reflection_seconds = current.reflection_seconds as u64,
                            tier2_acceleration_enabled = current.tier2_acceleration_enabled,
                            "cadence config hot-reloaded",
                        );
                    }
                    Err(_) => {
                        // All config senders dropped; park this arm so it does
                        // not busy-loop. The coordinator keeps running with the
                        // last-known config.
                        config_open = false;
                    }
                }
            }
            _ = tier3_interval.tick() => {
                let now = current_unix_nanos();
                let stats = run_one_coordinator_cycle(
                    CadenceMode::Tier3,
                    &sql_runner,
                    &broadcast_handle,
                    &hw_profile,
                    now,
                    None,
                ).await;
                if stats.digest_emitted {
                    let _ = digest_trigger.sender().send(DigestTrigger {
                        mode: CadenceMode::Tier3,
                        executed_at_unix_nano: now,
                        triggering_cue: None,
                    });
                }
                cumulative_cycles += 1;
                cumulative_queries += stats.queries_executed as u64;
            }
            _ = reflection_interval.tick() => {
                let now = current_unix_nanos();
                let stats = run_one_coordinator_cycle(
                    CadenceMode::Reflection,
                    &sql_runner,
                    &broadcast_handle,
                    &hw_profile,
                    now,
                    None,
                ).await;
                if stats.digest_emitted {
                    let _ = digest_trigger.sender().send(DigestTrigger {
                        mode: CadenceMode::Reflection,
                        executed_at_unix_nano: now,
                        triggering_cue: None,
                    });
                }
                cumulative_cycles += 1;
                cumulative_queries += stats.queries_executed as u64;
            }
            _ = heartbeat_interval.tick() => {
                tracing::info!(
                    target: TARGET_CADENCE_TICK,
                    tier = "tier3",
                    mode = "tier3",
                    queries_executed = cumulative_queries,
                    queries_succeeded = cumulative_queries,
                    next_due_ms = (current.baseline_seconds as u64) * 1_000,
                    last_executed_at_ms = 0_u64,
                    tier2_acceleration_enabled = current.tier2_acceleration_enabled,
                    "cadence heartbeat",
                );
            }
            recv = cue_rx.recv() => {
                match recv {
                    Ok(cue) if matches!(cue.priority_tier, PriorityTier::Autonomous) => {
                        let now = current_unix_nanos();
                        let stats = run_one_coordinator_cycle(
                            CadenceMode::Tier1,
                            &sql_runner,
                            &broadcast_handle,
                            &hw_profile,
                            now,
                            Some(&cue),
                        ).await;
                        if stats.digest_emitted {
                            let _ = digest_trigger.sender().send(DigestTrigger {
                                mode: CadenceMode::Tier1,
                                executed_at_unix_nano: now,
                                triggering_cue: Some(cue.clone()),
                            });
                        }
                        cumulative_cycles += 1;
                        cumulative_queries += stats.queries_executed as u64;
                    }
                    Ok(_) => {
                        // Non-Autonomous cues (Suggested / Curious) flow through
                        // their dedicated channels — ignored here.
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                        cue_rx = cue_broadcast.subscribe();
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
                }
            }
            recv = cadence_rx.recv() => {
                match recv {
                    Ok(cue) => {
                        if current.tier2_acceleration_enabled
                            && !matches!(
                                hw_profile.current_profile(),
                                HardwareProfile::CpuPrimary
                            )
                        {
                            let now = current_unix_nanos();
                            let stats = run_one_coordinator_cycle(
                                CadenceMode::Tier2,
                                &sql_runner,
                                &broadcast_handle,
                                &hw_profile,
                                now,
                                Some(&cue),
                            ).await;
                            if stats.digest_emitted {
                                let _ = digest_trigger.sender().send(DigestTrigger {
                                    mode: CadenceMode::Tier2,
                                    executed_at_unix_nano: now,
                                    triggering_cue: Some(cue.clone()),
                                });
                            }
                            cumulative_cycles += 1;
                            cumulative_queries += stats.queries_executed as u64;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                        cadence_rx = cadence_handle.subscribe();
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
                }
            }
        }
        let _ = cumulative_cycles; // suppress unused-warning on stub variant
    }
}

fn current_unix_nanos() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CueKind, CueScope};
    use std::sync::Mutex;
    use std::time::Duration;

    #[derive(Default)]
    struct CountingSqlRunner {
        invocations: Mutex<Vec<&'static str>>,
        fail_after: Option<usize>,
    }

    impl CountingSqlRunner {
        fn new() -> Self {
            Self::default()
        }

        fn record(&self, name: &'static str) {
            self.invocations.lock().unwrap().push(name);
        }

        fn count(&self) -> usize {
            self.invocations.lock().unwrap().len()
        }

        fn invoked(&self, name: &'static str) -> bool {
            self.invocations.lock().unwrap().contains(&name)
        }

        fn maybe_fail<T>(&self, default: T) -> Result<T, SqlAggregationError> {
            match self.fail_after {
                Some(n) if self.count() > n => Err(SqlAggregationError::QueryFailed {
                    query_id: "stub_fail",
                }),
                _ => Ok(default),
            }
        }
    }

    impl SqlQueryRunner for CountingSqlRunner {
        fn run_q1<'a>(&'a self, _window: Duration) -> SqlFuture<'a, Vec<Q1RedRow>> {
            Box::pin(async move {
                self.record("q1");
                self.maybe_fail(Vec::new())
            })
        }
        fn run_q2<'a>(&'a self, _window: Duration) -> SqlFuture<'a, Vec<Q2OperationRow>> {
            Box::pin(async move {
                self.record("q2");
                self.maybe_fail(Vec::new())
            })
        }
        fn run_q3<'a>(&'a self, _window: Duration) -> SqlFuture<'a, Vec<Q3FingerprintRow>> {
            Box::pin(async move {
                self.record("q3");
                self.maybe_fail(Vec::new())
            })
        }
        fn run_q4<'a>(&'a self, _window: Duration) -> SqlFuture<'a, Vec<Q4InteractionRow>> {
            Box::pin(async move {
                self.record("q4");
                self.maybe_fail(Vec::new())
            })
        }
        fn run_q5<'a>(&'a self, _window: Duration) -> SqlFuture<'a, Vec<Q5CardinalityRow>> {
            Box::pin(async move {
                self.record("q5");
                self.maybe_fail(Vec::new())
            })
        }
        fn run_q6<'a>(&'a self, _window: Duration) -> SqlFuture<'a, Vec<Q6LogRow>> {
            Box::pin(async move {
                self.record("q6");
                self.maybe_fail(Vec::new())
            })
        }
        fn run_q7<'a>(&'a self, _window: Duration) -> SqlFuture<'a, Vec<Q7CriticalPathRow>> {
            Box::pin(async move {
                self.record("q7");
                self.maybe_fail(Vec::new())
            })
        }
    }

    struct FixedProfile(HardwareProfile);

    impl HardwareProfileSource for FixedProfile {
        fn current_profile(&self) -> HardwareProfile {
            self.0
        }
    }

    fn arc_runner() -> (Arc<CountingSqlRunner>, Arc<dyn SqlQueryRunner>) {
        let counting = Arc::new(CountingSqlRunner::new());
        let dyn_runner: Arc<dyn SqlQueryRunner> = Arc::clone(&counting) as Arc<dyn SqlQueryRunner>;
        (counting, dyn_runner)
    }

    fn arc_profile(profile: HardwareProfile) -> Arc<dyn HardwareProfileSource> {
        Arc::new(FixedProfile(profile))
    }

    fn sample_cue(priority: PriorityTier) -> AttentionCue {
        AttentionCue {
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some("svc-a".to_string()),
            magnitude: 4.0,
            absolute_value: 0.04,
            persistence_seconds: 30,
            confidence: 0.85,
            priority_tier: priority,
            suppression_bypassed: false,
            fingerprint: None,
        }
    }

    #[test]
    fn mode_label_returns_bounded_four_value_enumeration() {
        assert_eq!(mode_label(CadenceMode::Tier1), "tier1");
        assert_eq!(mode_label(CadenceMode::Tier2), "tier2");
        assert_eq!(mode_label(CadenceMode::Tier3), "tier3");
        assert_eq!(mode_label(CadenceMode::Reflection), "reflection");
    }

    #[test]
    fn unknown_hardware_profile_returns_unknown() {
        let h = UnknownHardwareProfile;
        assert_eq!(h.current_profile(), HardwareProfile::Unknown);
    }

    #[tokio::test]
    async fn run_one_coordinator_cycle_invokes_all_seven_queries_for_tier3() {
        let (counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let _rx = broadcast.subscribe();
        let profile = arc_profile(HardwareProfile::Unknown);

        let stats = run_one_coordinator_cycle(
            CadenceMode::Tier3,
            &runner,
            &broadcast,
            &profile,
            1_000_000_000,
            None,
        )
        .await;

        assert_eq!(stats.queries_executed, 7);
        assert_eq!(stats.queries_succeeded, 7);
        assert!(stats.digest_emitted);
        assert!(!stats.tier2_skipped_cpu_primary);
        for q in ["q1", "q2", "q3", "q4", "q5", "q6", "q7"] {
            assert!(counting.invoked(q), "expected {q} invoked");
        }
    }

    #[tokio::test]
    async fn run_one_coordinator_cycle_skips_tier2_when_hardware_profile_is_cpu_primary() {
        let (counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let profile = arc_profile(HardwareProfile::CpuPrimary);

        let stats = run_one_coordinator_cycle(
            CadenceMode::Tier2,
            &runner,
            &broadcast,
            &profile,
            1_000_000_000,
            None,
        )
        .await;

        assert_eq!(stats.queries_executed, 0);
        assert_eq!(counting.count(), 0);
        assert!(stats.tier2_skipped_cpu_primary);
        assert!(!stats.digest_emitted);
    }

    #[tokio::test]
    async fn run_one_coordinator_cycle_runs_tier2_on_non_cpu_primary_profile() {
        let (_counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let _rx = broadcast.subscribe();
        let profile = arc_profile(HardwareProfile::GpuPrimary);

        let stats = run_one_coordinator_cycle(
            CadenceMode::Tier2,
            &runner,
            &broadcast,
            &profile,
            1_000_000_000,
            None,
        )
        .await;

        assert_eq!(stats.queries_executed, 7);
        assert!(!stats.tier2_skipped_cpu_primary);
    }

    #[tokio::test]
    async fn run_one_coordinator_cycle_emits_event_to_broadcast() {
        let (_counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut rx = broadcast.subscribe();
        let profile = arc_profile(HardwareProfile::Unknown);

        let _ = run_one_coordinator_cycle(
            CadenceMode::Tier3,
            &runner,
            &broadcast,
            &profile,
            42_424_242,
            None,
        )
        .await;

        let event = rx.try_recv().expect("broadcast event received");
        assert_eq!(event.mode, CadenceMode::Tier3);
        assert_eq!(event.mode_label, "tier3");
        assert_eq!(event.scheduled_at_unix_nano, 42_424_242);
        assert_eq!(event.queries_executed, 7);
        assert!(event.cue_kind_label.is_none());
    }

    #[tokio::test]
    async fn run_one_coordinator_cycle_emits_event_with_cue_metadata_for_tier1() {
        let (_counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut rx = broadcast.subscribe();
        let profile = arc_profile(HardwareProfile::Unknown);
        let cue = sample_cue(PriorityTier::Autonomous);

        let _ = run_one_coordinator_cycle(
            CadenceMode::Tier1,
            &runner,
            &broadcast,
            &profile,
            100,
            Some(&cue),
        )
        .await;

        let event = rx.try_recv().expect("broadcast event received");
        assert_eq!(event.mode, CadenceMode::Tier1);
        assert_eq!(event.cue_kind_label, Some("error_rate_spike"));
        assert_eq!(event.cue_priority_label, Some("autonomous"));
    }

    #[tokio::test]
    async fn run_one_coordinator_cycle_handles_partial_query_failures() {
        let counting = Arc::new(CountingSqlRunner {
            invocations: Mutex::new(Vec::new()),
            fail_after: Some(3),
        });
        let runner: Arc<dyn SqlQueryRunner> = Arc::clone(&counting) as Arc<dyn SqlQueryRunner>;
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let _rx = broadcast.subscribe();
        let profile = arc_profile(HardwareProfile::Unknown);

        let stats =
            run_one_coordinator_cycle(CadenceMode::Tier3, &runner, &broadcast, &profile, 1, None)
                .await;

        assert_eq!(stats.queries_executed, 7);
        assert!(stats.queries_succeeded < 7);
        assert!(stats.digest_emitted, "digest still emitted on partial fail");
    }

    #[tokio::test]
    async fn run_one_coordinator_cycle_reflection_emits_reflection_mode() {
        let (_counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut rx = broadcast.subscribe();
        let profile = arc_profile(HardwareProfile::Unknown);

        let _ = run_one_coordinator_cycle(
            CadenceMode::Reflection,
            &runner,
            &broadcast,
            &profile,
            1,
            None,
        )
        .await;

        let event = rx.try_recv().expect("broadcast event received");
        assert_eq!(event.mode, CadenceMode::Reflection);
        assert_eq!(event.mode_label, "reflection");
    }

    #[tokio::test]
    async fn start_cadence_coordinator_spawnable_and_abortable() {
        let (_counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
        let cadence_handle = Arc::new(CadenceTriggerChannel::new());
        let profile = arc_profile(HardwareProfile::Unknown);
        let config =
            Arc::new(CadenceConfig::try_new(5, 1, 300, true).expect("test config at min floors"));

        let handle = tokio::spawn(start_cadence_coordinator(
            runner,
            broadcast,
            Arc::new(DigestTriggerBroadcast::new()),
            cue_broadcast,
            cadence_handle,
            profile,
            config,
            watch::channel(CadenceConfig::default()).1,
        ));
        tokio::time::sleep(Duration::from_millis(20)).await;
        handle.abort();
        let result = handle.await;
        assert!(result.is_err(), "abort must yield cancellation");
        assert!(result.unwrap_err().is_cancelled());
    }

    #[tokio::test]
    async fn start_cadence_coordinator_responds_to_tier1_autonomous_cue() {
        let (counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut event_rx = broadcast.subscribe();
        let digest_trigger = Arc::new(DigestTriggerBroadcast::new());
        let mut trigger_rx = digest_trigger.subscribe();
        let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
        let cadence_handle = Arc::new(CadenceTriggerChannel::new());
        let profile = arc_profile(HardwareProfile::Unknown);
        let config = Arc::new(CadenceConfig::default());

        let handle = tokio::spawn(start_cadence_coordinator(
            runner,
            Arc::clone(&broadcast),
            Arc::clone(&digest_trigger),
            Arc::clone(&cue_broadcast),
            cadence_handle,
            profile,
            config,
            watch::channel(CadenceConfig::default()).1,
        ));
        // Give the spawned task a tick to subscribe.
        tokio::time::sleep(Duration::from_millis(20)).await;

        let cue = sample_cue(PriorityTier::Autonomous);
        cue_broadcast
            .sender()
            .send(cue)
            .expect("send autonomous cue ok");

        // Poll the event broadcast for up to 500ms.
        let mut received_tier1 = false;
        for _ in 0..50 {
            tokio::time::sleep(Duration::from_millis(10)).await;
            while let Ok(event) = event_rx.try_recv() {
                if event.mode == CadenceMode::Tier1 {
                    received_tier1 = true;
                    break;
                }
            }
            if received_tier1 {
                break;
            }
        }

        // P-074 fix: the Tier-1 trigger carries the full cue — incl. scope_id —
        // to the digest assembler, off the PII-free L6 cadence-events topic.
        let mut trigger_cue_scope = None;
        for _ in 0..20 {
            while let Ok(trigger) = trigger_rx.try_recv() {
                if trigger.mode == CadenceMode::Tier1 {
                    trigger_cue_scope = trigger.triggering_cue.and_then(|c| c.scope_id);
                    break;
                }
            }
            if trigger_cue_scope.is_some() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        handle.abort();
        let _ = handle.await;
        assert!(received_tier1, "expected Tier-1 event after Autonomous cue");
        assert!(
            counting.invoked("q1"),
            "Tier-1 cycle should have invoked q1"
        );
        assert_eq!(
            trigger_cue_scope.as_deref(),
            Some("svc-a"),
            "Tier-1 DigestTrigger must convey the cue's scope_id to the assembler"
        );
    }

    /// The Tier-1 arm is Autonomous-ONLY, and that is what makes a short storm
    /// produce no incident: the retry-storm detector emits `Suggested` at
    /// `DEFAULT_SUGGESTED_THRESHOLD` occurrences and only escalates to
    /// `Autonomous` at `DEFAULT_AUTONOMOUS_THRESHOLD`, so a burst that stops in
    /// between is dropped here by design rather than by defect. Measured at
    /// chunk `2026-08-15-tier-1-incident-path-investigation`, whose premise
    /// check found a 6-occurrence external canary sitting in exactly that band.
    ///
    /// The Autonomous send at the end is a POSITIVE CONTROL, not decoration: it
    /// proves the coordinator was alive and subscribed for the whole negative
    /// window, so the no-Tier-1 assertion cannot pass vacuously.
    #[tokio::test]
    async fn start_cadence_coordinator_ignores_suggested_cue_on_the_attention_broadcast() {
        let (counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut event_rx = broadcast.subscribe();
        let digest_trigger = Arc::new(DigestTriggerBroadcast::new());
        let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
        let cadence_handle = Arc::new(CadenceTriggerChannel::new());
        let profile = arc_profile(HardwareProfile::Unknown);
        let config = Arc::new(CadenceConfig::default());

        let handle = tokio::spawn(start_cadence_coordinator(
            runner,
            Arc::clone(&broadcast),
            Arc::clone(&digest_trigger),
            Arc::clone(&cue_broadcast),
            cadence_handle,
            profile,
            config,
            watch::channel(CadenceConfig::default()).1,
        ));
        tokio::time::sleep(Duration::from_millis(20)).await;

        cue_broadcast
            .sender()
            .send(sample_cue(PriorityTier::Suggested))
            .expect("send suggested cue ok");

        let mut tier1_after_suggested = false;
        for _ in 0..30 {
            tokio::time::sleep(Duration::from_millis(10)).await;
            while let Ok(event) = event_rx.try_recv() {
                if event.mode == CadenceMode::Tier1 {
                    tier1_after_suggested = true;
                }
            }
        }
        let q1_after_suggested = counting.invoked("q1");

        cue_broadcast
            .sender()
            .send(sample_cue(PriorityTier::Autonomous))
            .expect("send autonomous cue ok");

        let mut tier1_after_autonomous = false;
        for _ in 0..50 {
            tokio::time::sleep(Duration::from_millis(10)).await;
            while let Ok(event) = event_rx.try_recv() {
                if event.mode == CadenceMode::Tier1 {
                    tier1_after_autonomous = true;
                }
            }
            if tier1_after_autonomous {
                break;
            }
        }

        handle.abort();
        let _ = handle.await;

        assert!(
            !tier1_after_suggested,
            "a Suggested cue on the attention-cue broadcast must NOT run a Tier-1 cycle"
        );
        assert!(
            !q1_after_suggested,
            "a Suggested cue must not reach the L1a query layer via Tier-1"
        );
        assert!(
            tier1_after_autonomous,
            "positive control: an Autonomous cue on the same channel MUST run Tier-1 — \
             without this the negative assertions above could pass on a dead coordinator"
        );
    }

    #[tokio::test]
    async fn start_cadence_coordinator_responds_to_tier2_suggested_cadence_trigger() {
        let (counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut event_rx = broadcast.subscribe();
        let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
        let cadence_handle = Arc::new(CadenceTriggerChannel::new());
        let profile = arc_profile(HardwareProfile::GpuPrimary);
        let config = Arc::new(CadenceConfig::default());

        let handle = tokio::spawn(start_cadence_coordinator(
            runner,
            Arc::clone(&broadcast),
            Arc::new(DigestTriggerBroadcast::new()),
            cue_broadcast,
            Arc::clone(&cadence_handle),
            profile,
            config,
            watch::channel(CadenceConfig::default()).1,
        ));
        tokio::time::sleep(Duration::from_millis(20)).await;

        let cue = sample_cue(PriorityTier::Suggested);
        cadence_handle
            .sender()
            .send(cue)
            .expect("send suggested cue ok");

        let mut received_tier2 = false;
        for _ in 0..50 {
            tokio::time::sleep(Duration::from_millis(10)).await;
            while let Ok(event) = event_rx.try_recv() {
                if event.mode == CadenceMode::Tier2 {
                    received_tier2 = true;
                    break;
                }
            }
            if received_tier2 {
                break;
            }
        }
        handle.abort();
        let _ = handle.await;
        assert!(received_tier2, "expected Tier-2 event after Suggested cue");
        assert!(counting.invoked("q1"));
    }

    #[tokio::test]
    async fn start_cadence_coordinator_tier2_skipped_on_cpu_primary_profile() {
        let (counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut event_rx = broadcast.subscribe();
        let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
        let cadence_handle = Arc::new(CadenceTriggerChannel::new());
        let profile = arc_profile(HardwareProfile::CpuPrimary);
        let config = Arc::new(CadenceConfig::default());

        let handle = tokio::spawn(start_cadence_coordinator(
            runner,
            Arc::clone(&broadcast),
            Arc::new(DigestTriggerBroadcast::new()),
            cue_broadcast,
            Arc::clone(&cadence_handle),
            profile,
            config,
            watch::channel(CadenceConfig::default()).1,
        ));
        tokio::time::sleep(Duration::from_millis(20)).await;

        let cue = sample_cue(PriorityTier::Suggested);
        cadence_handle
            .sender()
            .send(cue)
            .expect("send suggested cue ok");

        // Wait short window; no Tier-2 event should arrive.
        let mut received_tier2 = false;
        for _ in 0..20 {
            tokio::time::sleep(Duration::from_millis(10)).await;
            while let Ok(event) = event_rx.try_recv() {
                if event.mode == CadenceMode::Tier2 {
                    received_tier2 = true;
                    break;
                }
            }
        }
        handle.abort();
        let _ = handle.await;
        assert!(
            !received_tier2,
            "Tier-2 must be skipped on cpu-primary profile"
        );
        assert_eq!(counting.count(), 0, "no queries on skipped Tier-2");
    }

    #[tokio::test]
    async fn run_one_coordinator_cycle_does_not_leak_pii_canary_into_broadcast_payload() {
        let canary = "secret-canary-API-key-12345";
        let (_counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut rx = broadcast.subscribe();
        let profile = arc_profile(HardwareProfile::Unknown);
        let mut cue = sample_cue(PriorityTier::Autonomous);
        cue.scope_id = Some(canary.to_string());

        let _ = run_one_coordinator_cycle(
            CadenceMode::Tier1,
            &runner,
            &broadcast,
            &profile,
            1,
            Some(&cue),
        )
        .await;

        let event = rx.try_recv().expect("event received");
        let json = serde_json::to_string(&event).expect("serialize ok");
        assert!(
            !json.contains(canary),
            "canary leaked into CadenceEvent JSON: {json}"
        );
    }

    #[tokio::test]
    async fn start_cadence_coordinator_survives_config_hot_reload() {
        let (counting, runner) = arc_runner();
        let broadcast = Arc::new(CadenceEventBroadcast::new());
        let mut event_rx = broadcast.subscribe();
        let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
        let cadence_handle = Arc::new(CadenceTriggerChannel::new());
        let profile = arc_profile(HardwareProfile::Unknown);
        let config = Arc::new(CadenceConfig::default());
        let (cfg_tx, cfg_rx) = watch::channel(CadenceConfig::default());

        let handle = tokio::spawn(start_cadence_coordinator(
            runner,
            Arc::clone(&broadcast),
            Arc::new(DigestTriggerBroadcast::new()),
            Arc::clone(&cue_broadcast),
            cadence_handle,
            profile,
            config,
            cfg_rx,
        ));
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Hot-reload the cadence config; the coordinator rebuilds its intervals
        // prospectively and keeps running.
        cfg_tx
            .send(CadenceConfig::try_new(5, 1, 300, false).expect("valid config"))
            .expect("send config update");
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Still alive + functional: an Autonomous cue still triggers Tier-1.
        cue_broadcast
            .sender()
            .send(sample_cue(PriorityTier::Autonomous))
            .expect("send autonomous cue");

        let mut received_tier1 = false;
        for _ in 0..50 {
            tokio::time::sleep(Duration::from_millis(10)).await;
            while let Ok(event) = event_rx.try_recv() {
                if event.mode == CadenceMode::Tier1 {
                    received_tier1 = true;
                    break;
                }
            }
            if received_tier1 {
                break;
            }
        }
        handle.abort();
        let _ = handle.await;
        assert!(
            received_tier1,
            "coordinator must survive + stay functional after a hot-reload"
        );
        assert!(counting.invoked("q1"));
        drop(cfg_tx);
    }
}
