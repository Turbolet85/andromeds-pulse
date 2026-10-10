use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

use interpretation::contract::LlmInferenceRunner;
use tauri::Manager;
use triage::contract::HardwareProfileSource;
use ui_bridge::Settings;
use ui_bridge::health::{IntrospectionApi, IntrospectionApiImpl};
use ui_bridge::telemetry::{TelemetryApi, TelemetryApiImpl};
use ui_bridge::workspace_ipc::{WorkspaceApi, WorkspaceApiImpl};
use viz::VizState;

use pulse_app::taurpc_export_config;
use pulse_app::{
    engine_boot, heartbeat, observability, render_posture, tray, window, window_geometry,
    xlib_threads,
};

use pulse_app::config_router::{ConfigApi, ConfigApiImpl};
use pulse_app::connection_router::{ConnectionApi, ConnectionApiImpl};
use pulse_app::diagnostics_router::{DiagnosticsApi, DiagnosticsApiImpl};
use pulse_app::engine_boot::{EngineConfig, EngineInputs, InterpretationSeat, Program, SeatKind};
use pulse_app::incidents_router::{IncidentsApi, IncidentsApiImpl};
use pulse_app::investigate_router::{InvestigateApi, InvestigateApiImpl};
use pulse_app::llamacli_inference::LlamaCliInference;
#[cfg(feature = "mcp-server")]
use pulse_app::mcp_router::{McpApi, McpApiImpl};
use pulse_app::model_router::{ModelApi, ModelApiImpl, tier_for_profile};
use pulse_app::plugins_router::{PluginsApi, PluginsApiImpl};
use pulse_app::services_router::{ServicesApi, ServicesApiImpl};
use pulse_app::snapshot_runtime::{SnapshotApi, SnapshotApiImpl};
use pulse_app::storage_router::{StorageApi, StorageApiImpl};
use pulse_app::streams::{StreamsApi, StreamsApiImpl};
use pulse_app::viz_routers::{
    LogsApi, LogsApiImpl, MetricsApi, MetricsApiImpl, TracesApi, TracesApiImpl,
};

// Resolves the path to the andromeda-pulse-mcp sidecar binary. Sibling of
// the current executable (same dir, with .exe on Windows). Falls back to a
// non-resolving placeholder if the current binary path cannot be read —
// `mcp.start` then surfaces AppError::Internal at spawn time. Chunk #49.
#[cfg(feature = "mcp-server")]
fn resolve_mcp_sidecar_binary_path() -> std::path::PathBuf {
    let binary_name = if cfg!(target_os = "windows") {
        "andromeda-pulse-mcp.exe"
    } else {
        "andromeda-pulse-mcp"
    };
    match std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
    {
        Some(dir) => dir.join(binary_name),
        None => std::path::PathBuf::from(binary_name),
    }
}

fn main() {
    let render_posture = render_posture::apply_linux_default();
    xlib_threads::init();

    // taurpc's `Router::into_handler()` spawns a background handler-manager
    // task during binding emission and requires a tokio runtime in scope, but
    // Tauri's Builder doesn't establish one until `.run()` (which happens
    // after the router is constructed). Build + enter our own runtime first
    // and hand it to Tauri via `async_runtime::set` so taurpc's pre-`run()`
    // spawns, the engine boot's spawns and Tauri's own share a single runtime.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    let _enter = runtime.enter();
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let data_dir = engine_boot::resolve_data_dir();
    engine_boot::init_process(&data_dir);
    window::emit_boot_spans();
    render_posture::emit_posture(render_posture);

    // Chunk #82 — the real `HardwareProfileDetector`. Detector probes GPU
    // presence + CPU core count at construction; cached for subsequent reads.
    // Env var `ANDROMEDA_PULSE_HARDWARE_PROFILE` overrides detection for
    // tests + degraded-environment validation.
    let hardware_profile: Arc<dyn HardwareProfileSource> =
        Arc::new(pulse_app::hardware_profile::HardwareProfileDetector::new());
    let detected_profile = hardware_profile.current_profile();
    let model_tier = tier_for_profile(detected_profile);
    let model_status_broadcast = interpretation::broadcast::ModelStatusBroadcast::new();
    let llamacli_inference = Arc::new(LlamaCliInference::new(
        model_tier,
        detected_profile,
        model_status_broadcast.clone(),
    ));
    // Deterministic env-gated L4 mode: when the gate is truthy the canned-output
    // runner is the active L4 runner (reproducible incident path, no model);
    // otherwise the real llama-cli subprocess runner. The llama-cli readiness
    // check below still runs in either mode (cheap; file-existence only, no
    // subprocess).
    let (llm_runner, seat_kind): (Arc<dyn LlmInferenceRunner>, SeatKind) =
        if pulse_app::deterministic_inference::deterministic_mode_enabled() {
            tracing::info!(
                target: "interpretation.model.load",
                inference_mode = "deterministic",
                "L4 deterministic mode active (ANDROMEDA_PULSE_L4_DETERMINISTIC); canned output, no model",
            );
            (
                Arc::new(
                    pulse_app::deterministic_inference::DeterministicInferenceRunner::new(
                        model_tier,
                    ),
                ),
                SeatKind::Deterministic,
            )
        } else {
            tracing::info!(
                target: "interpretation.model.load",
                inference_mode = "real",
                "L4 real mode (llama-cli subprocess D1)",
            );
            (
                Arc::clone(&llamacli_inference) as Arc<dyn LlmInferenceRunner>,
                SeatKind::Model,
            )
        };
    let model_impl = ModelApiImpl::new(Arc::clone(&llm_runner), Arc::clone(&hardware_profile));

    // Chunk #84 — boot-time llama-cli readiness check. Fire-and-forget: if
    // `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH` or `ANDROMEDA_PULSE_MODEL_PATH`
    // are unset OR resolve to invalid paths, the runner stays in `ModelStatus::Error`
    // and `generate_constrained` returns `ModelNotConfigured` — the L4 subscriber
    // catches it as a runtime_error and skips. App boots cleanly in graceful-
    // degraded mode either way. Subprocess D1 means no actual model load happens
    // here (load happens per-generation inside llama-cli).
    {
        let runner_for_load = Arc::clone(&llamacli_inference);
        tokio::spawn(async move {
            let _ = runner_for_load.load_from_env_if_configured().await;
        });
    }

    // The engine itself — receivers, buffer, detectors, corpus, its ticks — is
    // built and spawned by the one boot both programs call.
    let engine = engine_boot::start(
        EngineConfig::from_env(data_dir.clone()),
        EngineInputs {
            program: Program::Window,
            key_backend: engine_boot::os_key_backend(),
            hardware_profile: Arc::clone(&hardware_profile),
            interpretation: Some(InterpretationSeat {
                runner: Arc::clone(&llm_runner),
                kind: seat_kind,
            }),
        },
    );
    let heartbeat_state = Arc::clone(&engine.heartbeat_state);
    let broadcast_senders = Arc::clone(&engine.broadcast_senders);
    let buffer_conn = engine.buffer_conn.clone();

    let viz_state = Arc::new(VizState::new());
    let connection_impl = ConnectionApiImpl::new(
        Arc::clone(&engine.ingest_state),
        Arc::clone(&engine.bind_status),
    );
    // Absent when the corpus did not open at boot: storage.inspect /
    // storage.path then return AppError::Storage at IPC time.
    let storage_impl = engine
        .corpus_reader
        .as_ref()
        .zip(engine.corpus_writer.as_ref())
        .map(|(r, w)| StorageApiImpl::new(Arc::clone(r), Arc::clone(w)));
    let incidents_impl = engine.incident_persistence.as_ref().map(|p| {
        IncidentsApiImpl::new(
            Arc::clone(&engine.incident_registry),
            Arc::clone(&engine.incident_broadcast),
            Arc::clone(p),
            engine.incident_workspace_key.clone(),
        )
    });
    // Its per-service severity join reads the active-incident registry to
    // enrich each ServiceListItem.priority_tier for the constellation dots.
    let services_impl = ServicesApiImpl::new(
        Arc::clone(&engine.lifecycle_registry),
        Arc::clone(&engine.incident_registry),
        engine.incident_workspace_key.clone(),
        Arc::clone(&engine.lifecycle_broadcast),
    );
    let config_impl = ConfigApiImpl::new(
        Arc::clone(&engine.config_handle_slot),
        Arc::clone(&engine.config_status),
    );
    let diagnostics_impl = DiagnosticsApiImpl::new(
        Arc::clone(&engine.drain_miner),
        Arc::clone(&engine.degraded_mode),
        Arc::clone(&engine.reevaluator),
        Arc::clone(&llm_runner),
        Arc::clone(&hardware_profile),
    );

    // features array surfaces in `app_info.features`; populated from compile-time
    // cfg!() checks. mcp-server is the only known opt-in feature at chunk #27.
    #[cfg(feature = "mcp-server")]
    let features: Vec<String> = vec!["mcp-server".to_string()];
    #[cfg(not(feature = "mcp-server"))]
    let features: Vec<String> = Vec::new();
    let introspection_impl = IntrospectionApiImpl::new(
        data_dir.clone(),
        features,
        Some(Arc::clone(&broadcast_senders)),
        Some(Arc::clone(&engine.buffer_state)),
        engine.retention_seconds,
    );

    // Chunk #44: SnapshotApiImpl constructed BEFORE the router build so the
    // sibling clone (snapshot_impl_for_setup) can survive into the setup
    // closure to populate AppHandle. The Arc<OnceLock<AppHandle<Wry>>> is
    // shared between clones; setup populates once, resolver reads on every
    // IPC call.
    let snapshot_impl = SnapshotApiImpl::new(buffer_conn.clone(), data_dir.clone());
    let snapshot_impl_for_setup = snapshot_impl.clone();
    // P-072 — Investigate-action resolver. Reuses the boot-built `llm_runner`
    // (deterministic-aware) + the buffer connection for the telemetry context.
    let investigate_impl = InvestigateApiImpl::new(buffer_conn.clone(), Arc::clone(&llm_runner));

    // Chunk #47: plugin host wiring. Engine is constructed at boot (shared
    // across all plugin operations); plugin dir is resolved + canonicalized
    // (missing-dir is non-fatal — boot proceeds with empty registry); initial
    // discovery populates the registry. The registry Mutex is shared with
    // the heartbeat task via Arc::clone.
    let plugin_engine = Arc::new(plugins::engine::build_engine().expect("plugin engine builds"));
    let plugin_dir_raw = plugins::loader::resolve_plugin_dir();
    let canonical_plugin_dir = match plugins::loader::canonicalize_plugin_dir(&plugin_dir_raw) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(
                target: "plugin.load.boot",
                error_msg = %e,
                "plugin dir canonicalization failed; plugin host disabled this boot",
            );
            plugin_dir_raw.clone()
        }
    };
    let canonical_plugin_dir = Arc::new(canonical_plugin_dir);
    let initial_plugins =
        plugins::loader::discover_plugins(&plugin_engine, canonical_plugin_dir.as_path())
            .unwrap_or_else(|e| {
                tracing::warn!(
                    target: "plugin.load.boot",
                    error_msg = %e,
                    "plugin discovery failed at boot; registry starts empty",
                );
                Vec::new()
            });
    let mut boot_registry = plugins::loader::PluginRegistry::empty();
    boot_registry.replace_plugins(initial_plugins);
    let plugins_registry = Arc::new(Mutex::new(boot_registry));
    let plugins_impl = PluginsApiImpl::new(
        Arc::clone(&plugin_engine),
        Arc::clone(&plugins_registry),
        Arc::clone(&canonical_plugin_dir),
    );

    // Chunk #49: mcp.* router wiring. The sidecar binary path is resolved
    // by joining the current binary's parent directory with the sidecar
    // executable name (with .exe extension on Windows). If the current
    // binary's path cannot be resolved, fall back to a relative name that
    // will fail at spawn time (mcp.start surfaces AppError::Internal then).
    #[cfg(feature = "mcp-server")]
    let mcp_impl = {
        let mcp_sidecar_binary_path = Arc::new(resolve_mcp_sidecar_binary_path());
        McpApiImpl::new(Arc::clone(&mcp_sidecar_binary_path))
    };

    let invoke_router = match buffer_conn.as_ref() {
        Some(conn) => {
            let base = taurpc::Router::new()
                .export_config(taurpc_export_config())
                .merge(introspection_impl.clone().into_handler())
                .merge(TracesApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
                .merge(MetricsApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
                .merge(LogsApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
                .merge(StreamsApiImpl::new(Arc::clone(&broadcast_senders)).into_handler())
                .merge(TelemetryApiImpl::new().into_handler())
                .merge(snapshot_impl.clone().into_handler())
                .merge(WorkspaceApiImpl::new().into_handler())
                .merge(plugins_impl.clone().into_handler())
                .merge(connection_impl.clone().into_handler())
                .merge(services_impl.clone().into_handler())
                .merge(diagnostics_impl.clone().into_handler())
                .merge(model_impl.clone().into_handler())
                .merge(investigate_impl.clone().into_handler())
                .merge(config_impl.clone().into_handler());
            let base = match storage_impl.as_ref() {
                Some(s) => base.merge(s.clone().into_handler()),
                None => base,
            };
            let base = match incidents_impl.as_ref() {
                Some(i) => base.merge(i.clone().into_handler()),
                None => base,
            };
            #[cfg(feature = "mcp-server")]
            let base = base.merge(mcp_impl.clone().into_handler());
            base
        }
        None => {
            let base = taurpc::Router::new()
                .export_config(taurpc_export_config())
                .merge(introspection_impl.clone().into_handler())
                .merge(StreamsApiImpl::new(Arc::clone(&broadcast_senders)).into_handler())
                .merge(TelemetryApiImpl::new().into_handler())
                .merge(snapshot_impl.clone().into_handler())
                .merge(WorkspaceApiImpl::new().into_handler())
                .merge(plugins_impl.clone().into_handler())
                .merge(connection_impl.clone().into_handler())
                .merge(services_impl.clone().into_handler())
                .merge(diagnostics_impl.clone().into_handler())
                .merge(model_impl.clone().into_handler())
                .merge(investigate_impl.clone().into_handler())
                .merge(config_impl.clone().into_handler());
            let base = match storage_impl.as_ref() {
                Some(s) => base.merge(s.clone().into_handler()),
                None => base,
            };
            let base = match incidents_impl.as_ref() {
                Some(i) => base.merge(i.clone().into_handler()),
                None => base,
            };
            #[cfg(feature = "mcp-server")]
            let base = base.merge(mcp_impl.clone().into_handler());
            base
        }
    };

    // Window geometry (remembered position) — Rust-owned, decoupled from the
    // webview Settings contract so an update_settings can never clobber it.
    // The event handler records moves (throttled) + flushes on close-to-tray;
    // boot restores it from the snapshot taken in the setup closure.
    let window_geometry = Arc::new(Mutex::new(window_geometry::GeometryStore::load(&data_dir)));
    let window_geometry_for_event = Arc::clone(&window_geometry);
    let data_dir_for_event = data_dir.clone();
    // Resize generation for the compact-widget aspect clamp (intent F2 /
    // P-062): the handler debounces on this so the clamp snaps once after the
    // resize settles instead of fighting the drag frame-by-frame.
    let aspect_resize_gen = Arc::new(AtomicU64::new(0));

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .on_window_event(move |w, e| {
            window::on_window_event(
                w,
                e,
                window_geometry_for_event.as_ref(),
                &data_dir_for_event,
                &aspect_resize_gen,
            )
        })
        .invoke_handler(invoke_router.into_handler())
        .setup(move |app| {
            // Chunk #44: populate the deferred AppHandle into SnapshotApiImpl.
            // Setup runs after invoke_handler is locked, but the Arc<OnceLock>
            // shared between snapshot_impl (in router) and snapshot_impl_for_setup
            // (this closure) means the resolver sees the handle on every
            // subsequent IPC call.
            snapshot_impl_for_setup.set_app_handle(app.handle().clone());

            window::show_compact_widget(app);
            let settings = Settings::load_from_data_dir(&data_dir);
            let geometry_snapshot = window_geometry
                .lock()
                .map(|s| s.snapshot())
                .unwrap_or_default();
            window::apply_widget_settings(app, &settings);
            window::restore_main_window_position(app, &geometry_snapshot);
            window::spawn_navigation_check(app.handle().clone());
            let tray_icon = tray::setup_tray(app.handle(), Arc::clone(&broadcast_senders))?;
            app.manage(tray_icon);

            // The window's own two ticks; the engine's three are spawned by
            // the engine boot.
            let _window_tick_handles =
                heartbeat::spawn_window_ticks(heartbeat_state, viz_state, plugins_registry);
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    // `run` would end in tao's `process::exit` with the log worker undrained;
    // `run_return` hands the code back so the exit is recorded first.
    let exit_code = app.run_return(|_, _| {});
    observability::exit_after_event_loop(exit_code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use buffer::{BroadcastSenders, DrainConfig, DrainMiner};
    use duckdb::Connection;
    use ingest::state::IngestState;
    use pulse_app::incident_persistence::CorpusIncidentPersistence;
    use pulse_app::reevaluation::{LiveReevaluator, RecentWindowReevaluator};
    use triage::contract::{
        InMemoryIncidentRegistry, InMemoryServiceRegistry, IncidentLifecycleBroadcast,
        IncidentPersistence, IncidentRegistry, ServiceLifecycleBroadcast, ServiceRegistry,
        UnknownHardwareProfile,
    };

    // Bindings emission test (chunk #25). taurpc 0.7 emits TS bindings at
    // `Router::into_handler()` call time when `tauri::is_dev()` returns true
    // (which is `!cfg!(feature = "custom-protocol")`, true in test builds).
    // The emission target path on the HealthApi `#[taurpc::procedures]` macro
    // is `ui/src/bindings/index.ts` — relative to the test runtime cwd, which
    // is `pulse-app/` for `cargo nextest -p pulse-app`. The merged router
    // here mirrors the production wiring in `main()` (when buffer init OK).
    // `#[tokio::test]` provides the runtime context taurpc::TauRpcHandler::spawn()
    // requires.
    #[tokio::test]
    async fn emit_taurpc_bindings() {
        let conn = Arc::new(Mutex::new(
            Connection::open_in_memory().expect("in-memory DuckDB"),
        ));
        let viz_state = Arc::new(VizState::new());
        let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());
        let data_dir = std::env::temp_dir().join("andromeda-pulse-bindings-test");
        let introspection_impl = IntrospectionApiImpl::new(
            data_dir.clone(),
            vec![],
            Some(Arc::clone(&broadcast_senders)),
            None,
            600,
        );

        let snapshot_impl = SnapshotApiImpl::new(Some(Arc::clone(&conn)), data_dir.clone());

        // Chunk #47: PluginsApiImpl participates in the emit so the bindings.ts
        // ARGS_MAP includes plugins.list / reload / invoke; xtask capability-drift
        // depends on this emission to verify EXPECTED_PROCEDURES sync.
        let plugin_engine = Arc::new(
            plugins::engine::build_engine().expect("plugin engine builds for bindings test"),
        );
        let plugins_registry = Arc::new(Mutex::new(plugins::loader::PluginRegistry::empty()));
        let canonical_plugin_dir =
            Arc::new(std::env::temp_dir().join("andromeda-pulse-plugins-bindings-test"));
        let plugins_impl = PluginsApiImpl::new(
            Arc::clone(&plugin_engine),
            Arc::clone(&plugins_registry),
            Arc::clone(&canonical_plugin_dir),
        );

        // Chunk #59: ConnectionApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes connection.current_state (quadruple-binding 4th slot
        // per .claude/rules/security.md Session Additions 2026-05-12).
        let connection_ingest = Arc::new(IngestState::new());
        struct TestBindStatus;
        impl ingest::connection::ReceiverBindStatus for TestBindStatus {
            fn any_receiver_failed(&self) -> bool {
                false
            }
        }
        let connection_bind: Arc<dyn ingest::connection::ReceiverBindStatus> =
            Arc::new(TestBindStatus);
        let connection_impl = ConnectionApiImpl::new(connection_ingest, connection_bind);

        // Chunk #67: ServicesApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes services.list_with_states (quadruple-binding 4th slot
        // per .claude/rules/security.md Session Additions 2026-05-12).
        let services_registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
        let services_incident_registry: Arc<dyn IncidentRegistry> =
            Arc::new(InMemoryIncidentRegistry::new());
        let services_broadcast = Arc::new(ServiceLifecycleBroadcast::new());
        let services_impl = ServicesApiImpl::new(
            services_registry,
            services_incident_registry,
            "bindings-test-workspace".to_string(),
            services_broadcast,
        );

        // Chunk #68: StorageApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes storage.inspect / storage.path (quadruple-binding
        // 4th slot per .claude/rules/security.md Session Additions 2026-05-12).
        // In-memory corpus backed by FakeKeychainBackend keeps the test
        // hermetic — no real OS keychain or on-disk SQLite file.
        let storage_keychain: Arc<dyn corpus::contract::KeychainBackend> =
            Arc::new(corpus::contract::FakeKeychainBackend::new());
        let storage_corpus = corpus::contract::Corpus::open_in_memory(storage_keychain)
            .expect("in-memory corpus opens with fake keychain");
        let storage_corpus_arc = Arc::new(storage_corpus);
        let storage_reader: Arc<dyn corpus::contract::CorpusReader> =
            Arc::clone(&storage_corpus_arc) as Arc<dyn corpus::contract::CorpusReader>;
        let storage_writer: Arc<dyn corpus::contract::CorpusWriter> =
            Arc::clone(&storage_corpus_arc) as Arc<dyn corpus::contract::CorpusWriter>;
        let storage_impl = StorageApiImpl::new(storage_reader, storage_writer);

        // Chunk #69 Phase B Session 3: DiagnosticsApiImpl participates in
        // the emit so bindings.ts ARGS_MAP includes diagnostics.template_distribution
        // (quadruple-binding 4th slot per .claude/rules/security.md Session
        // Additions 2026-05-12). In-memory miner with no persistence keeps
        // the test hermetic.
        let diagnostics_miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
        // Chunk #86 — DiagnosticsApiImpl now takes Arc<dyn DegradedModeStatus>;
        // in-memory LocalDegradedModeStatus keeps the test hermetic.
        let diagnostics_degraded_mode: Arc<dyn interpretation::degraded_mode::DegradedModeStatus> =
            Arc::new(pulse_app::degraded_mode_runtime::LocalDegradedModeStatus::new());
        // Chunk #96 — DiagnosticsApiImpl now takes a RecentWindowReevaluator;
        // a LiveReevaluator over fresh in-memory handles keeps the test hermetic.
        let reeval_registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
        let reeval_broadcast = Arc::new(ServiceLifecycleBroadcast::new());
        let reeval_baseline = Arc::new(triage::contract::BaselineState::new());
        let (_reeval_thresh_tx, reeval_thresh_rx) =
            tokio::sync::watch::channel(triage::contract::LifecycleThresholds {
                dormant_after_secs: 3_600,
                archived_after_secs: 86_400,
            });
        let diagnostics_reevaluator: Arc<dyn RecentWindowReevaluator> =
            Arc::new(LiveReevaluator::new(
                reeval_registry,
                reeval_broadcast,
                reeval_baseline,
                reeval_thresh_rx,
            ));
        // Chunk #97 — DiagnosticsApiImpl now takes an LlmInferenceRunner +
        // HardwareProfileSource for diagnostics.snapshot(); dedicated hermetic
        // instances keep the bindings emit deterministic (UnknownHardwareProfile
        // + an env-unset LlamaCliInference → status: Error, identity: None).
        let diagnostics_hardware: Arc<dyn HardwareProfileSource> = Arc::new(UnknownHardwareProfile);
        let diagnostics_model_bcast = interpretation::broadcast::ModelStatusBroadcast::new();
        let diagnostics_runner: Arc<dyn interpretation::contract::LlmInferenceRunner> =
            Arc::new(LlamaCliInference::new(
                interpretation::contract::ModelTier::Primary,
                diagnostics_hardware.current_profile(),
                diagnostics_model_bcast,
            ));
        let diagnostics_impl = DiagnosticsApiImpl::new(
            diagnostics_miner,
            diagnostics_degraded_mode,
            diagnostics_reevaluator,
            diagnostics_runner,
            diagnostics_hardware,
        );

        // Chunk #78: IncidentsApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes incidents.list_active / acknowledge / mark_resolved
        // (quadruple-binding 4th slot per .claude/rules/security.md Session
        // Additions 2026-05-12). In-memory corpus + fresh registry + broadcast
        // keep the test hermetic.
        let incident_keychain: Arc<dyn corpus::contract::KeychainBackend> =
            Arc::new(corpus::contract::FakeKeychainBackend::new());
        let incident_corpus = corpus::contract::Corpus::open_in_memory(incident_keychain)
            .expect("in-memory incident corpus opens with fake keychain");
        let incident_corpus_arc = Arc::new(incident_corpus);
        let incident_writer: Arc<dyn corpus::contract::CorpusWriter> =
            Arc::clone(&incident_corpus_arc) as Arc<dyn corpus::contract::CorpusWriter>;
        let incident_persistence_test: Arc<dyn IncidentPersistence> =
            Arc::new(CorpusIncidentPersistence::new(incident_writer));
        let incident_registry_test: Arc<dyn IncidentRegistry> =
            Arc::new(InMemoryIncidentRegistry::new());
        let incident_broadcast_test = Arc::new(IncidentLifecycleBroadcast::new());
        let incidents_impl = IncidentsApiImpl::new(
            incident_registry_test,
            incident_broadcast_test,
            incident_persistence_test,
            "bindings-test-workspace".to_string(),
        );

        // Chunk #82: ModelApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes model.current_profile (5-place binding 5th slot
        // per .claude/rules/security.md Session Additions 2026-05-12).
        // Test uses UnknownHardwareProfile (deterministic) + a stub
        // LlamaCliInference (status: Error, identity: None — env vars unset
        // in test process so binary_path + model_path both resolve to None).
        let model_hardware: Arc<dyn HardwareProfileSource> = Arc::new(UnknownHardwareProfile);
        let model_status_bcast_test = interpretation::broadcast::ModelStatusBroadcast::new();
        let model_runner_test: Arc<dyn interpretation::contract::LlmInferenceRunner> =
            Arc::new(LlamaCliInference::new(
                interpretation::contract::ModelTier::Primary,
                model_hardware.current_profile(),
                model_status_bcast_test,
            ));
        let model_impl = ModelApiImpl::new(model_runner_test, model_hardware);

        // P-072: InvestigateApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes investigate.run_action. Deterministic runner keeps
        // the bindings test hermetic (no env / model / subprocess).
        let investigate_impl = InvestigateApiImpl::new(
            Some(Arc::clone(&conn)),
            Arc::new(
                pulse_app::deterministic_inference::DeterministicInferenceRunner::new(
                    interpretation::contract::ModelTier::Primary,
                ),
            ),
        );

        // Chunk #96: ConfigApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes config.reload / config.status. Empty handle slot
        // (no watcher in the bindings test) + default status keep it hermetic.
        let config_impl = ConfigApiImpl::new(
            Arc::new(std::sync::OnceLock::new()),
            Arc::new(std::sync::Mutex::new(
                config_watcher::ConfigStatus::default(),
            )),
        );

        // Chunk #49: McpApiImpl participates in the emit so the bindings.ts
        // ARGS_MAP includes mcp.status / mcp.start / mcp.stop (4th binding
        // per .claude/rules/security.md Session Additions 2026-05-12).
        #[cfg(feature = "mcp-server")]
        let mcp_impl = {
            let mcp_sidecar_path =
                Arc::new(std::env::temp_dir().join("andromeda-pulse-mcp-bindings-test"));
            McpApiImpl::new(mcp_sidecar_path)
        };

        let router: taurpc::Router<tauri::Wry> = {
            let base = taurpc::Router::new()
                .export_config(taurpc_export_config())
                .merge(introspection_impl.into_handler())
                .merge(TracesApiImpl::new(Arc::clone(&conn), Arc::clone(&viz_state)).into_handler())
                .merge(
                    MetricsApiImpl::new(Arc::clone(&conn), Arc::clone(&viz_state)).into_handler(),
                )
                .merge(LogsApiImpl::new(Arc::clone(&conn), Arc::clone(&viz_state)).into_handler())
                .merge(StreamsApiImpl::new(Arc::clone(&broadcast_senders)).into_handler())
                .merge(TelemetryApiImpl::new().into_handler())
                .merge(snapshot_impl.into_handler())
                .merge(WorkspaceApiImpl::new().into_handler())
                .merge(plugins_impl.into_handler())
                .merge(connection_impl.into_handler())
                .merge(services_impl.into_handler())
                .merge(storage_impl.into_handler())
                .merge(diagnostics_impl.into_handler())
                .merge(incidents_impl.into_handler())
                .merge(model_impl.into_handler())
                .merge(investigate_impl.into_handler())
                .merge(config_impl.into_handler());
            #[cfg(feature = "mcp-server")]
            let base = base.merge(mcp_impl.into_handler());
            base
        };

        // into_handler() triggers export_types() in dev mode.
        let _handler = router.into_handler();

        let bindings_path = std::path::Path::new("ui/src/bindings/index.ts");
        assert!(
            bindings_path.exists(),
            "bindings file not emitted at {bindings_path:?} (cwd={:?})",
            std::env::current_dir().ok()
        );
    }
}
