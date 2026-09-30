// Migrated 2026-08-30 from `pulse-app/src/observability.rs::tests` — that
// crate sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS (130 tests sat dead;
// 2026-05-20 precedent moved 47 the same way). Internals reach here via
// `pub` + `#[doc(hidden)]` per test-plan section 2/4.

use pulse_app::observability::*;

use std::path::Path;

use serde_json::Value;
use tracing_subscriber::fmt;

use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

#[derive(Clone)]
struct VecMakeWriter(Arc<Mutex<Vec<u8>>>);

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for VecMakeWriter {
    type Writer = VecWriter;
    fn make_writer(&'a self) -> Self::Writer {
        VecWriter(Arc::clone(&self.0))
    }
}

struct VecWriter(Arc<Mutex<Vec<u8>>>);

impl Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn make_defaults(ci_run_id: Option<&str>, git_commit_sha: Option<&str>) -> DefaultFields {
    DefaultFields {
        service_name: SERVICE_NAME,
        service_version: env!("CARGO_PKG_VERSION"),
        deployment_environment: "production".into(),
        ci_run_id: ci_run_id.map(String::from),
        git_commit_sha: git_commit_sha.map(String::from),
    }
}

fn capture_json_lines<F: FnOnce()>(defaults: DefaultFields, f: F) -> Vec<Value> {
    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let writer = VecMakeWriter(Arc::clone(&buf));
    let layer = fmt::layer()
        .event_format(JsonWithDefaults {
            defaults,
            allowlist: AllowList::production(),
        })
        .with_writer(writer);
    let subscriber = Registry::default().with(layer);
    tracing::subscriber::with_default(subscriber, f);
    let bytes = buf.lock().unwrap().clone();
    let s = String::from_utf8(bytes).expect("captured bytes should be valid UTF-8");
    s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).expect("each line should parse as JSON"))
        .collect()
}

#[test]
fn default_fields_present_when_ci_env_set() {
    let defaults = make_defaults(Some("123456"), Some("abc1234"));
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "ingest", span_count = 5_i64, "test event");
    });
    assert_eq!(lines.len(), 1);
    let line = &lines[0];
    assert_eq!(line["fields"]["service.name"], "com.andromeda.pulse");
    assert_eq!(line["fields"]["service.version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(line["fields"]["deployment.environment"], "production");
    assert_eq!(line["fields"]["ci.run.id"], "123456");
    assert_eq!(line["fields"]["git.commit.sha"], "abc1234");
}

#[test]
fn default_fields_absent_when_ci_env_unset() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "ingest", span_count = 5_i64, "test event");
    });
    let line = &lines[0];
    assert!(line["fields"].get("ci.run.id").is_none());
    assert!(line["fields"].get("git.commit.sha").is_none());
    assert_eq!(line["fields"]["service.name"], "com.andromeda.pulse");
}

#[test]
fn validate_ci_run_id_accepts_digits() {
    assert!(validate_ci_run_id("1"));
    assert!(validate_ci_run_id("123456"));
    assert!(validate_ci_run_id("99999999999999999999999999999999"));
}

#[test]
fn validate_ci_run_id_rejects_garbage() {
    assert!(!validate_ci_run_id(""));
    assert!(!validate_ci_run_id("../../etc/passwd"));
    assert!(!validate_ci_run_id("hello"));
    assert!(!validate_ci_run_id("123abc"));
    assert!(!validate_ci_run_id("123456789012345678901234567890123"));
}

#[test]
fn validate_git_commit_sha_accepts_lowercase_hex() {
    assert!(validate_git_commit_sha("abc1234"));
    assert!(validate_git_commit_sha(
        "0123456789abcdef0123456789abcdef01234567"
    ));
    assert!(validate_git_commit_sha(&"a".repeat(40)));
    assert!(validate_git_commit_sha(&"f".repeat(64)));
}

#[test]
fn validate_git_commit_sha_rejects_garbage() {
    assert!(!validate_git_commit_sha(""));
    assert!(!validate_git_commit_sha("ABC1234"));
    assert!(!validate_git_commit_sha("ghijkl1"));
    assert!(!validate_git_commit_sha("ab"));
    assert!(!validate_git_commit_sha(&"a".repeat(65)));
}

#[test]
fn scrubber_redacts_non_allowlisted_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "ingest", attribute_value = "secret123", "test");
    });
    let line = &lines[0];
    let json_str = serde_json::to_string(line).unwrap();
    assert!(
        !json_str.contains("secret123"),
        "scrubbed value should not appear in JSON: {json_str}"
    );
    assert_eq!(line["fields"]["attribute_value"], "<redacted>");
}

#[test]
fn scrubber_passes_allowlisted_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "ingest", span_count = 42_i64, "ingest event");
    });
    assert_eq!(lines[0]["fields"]["span_count"], 42);
}

#[test]
fn scrubber_passes_a11y_diagnostic_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "a11y::assertion",
            wcag_criterion = "SC 1.4.3",
            selector = ".settings-modal form input[type='text']",
            remediation = "Increase foreground darkness; use --color-text-primary token",
            "a11y violation",
        );
    });
    let line = &lines[0];
    assert_eq!(line["fields"]["wcag_criterion"], "SC 1.4.3");
    assert_eq!(
        line["fields"]["selector"],
        ".settings-modal form input[type='text']"
    );
    assert!(
        line["fields"]["remediation"]
            .as_str()
            .unwrap()
            .contains("--color-text-primary")
    );
}

#[test]
fn scrubber_passes_metric_prefix_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "metric.ingest.throughput", value = 1234_i64, "throughput");
    });
    assert_eq!(lines[0]["fields"]["value"], 1234);
}

#[test]
fn scrubber_passes_tick_suffix_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "ingest.tick", span_count = 7_i64, "heartbeat");
    });
    assert_eq!(lines[0]["fields"]["span_count"], 7);
}

#[test]
fn scrubber_passes_ingest_tick_obs_plan_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "ingest.tick",
            span_count = 12_u64,
            buffer_capacity_pct = 0.42_f64,
            broadcast_subscribers = 3_u64,
            "heartbeat",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["span_count"], 12);
    assert_eq!(fields["buffer_capacity_pct"], 0.42);
    assert_eq!(fields["broadcast_subscribers"], 3);
}

#[test]
fn scrubber_passes_buffer_tick_obs_plan_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "buffer.tick",
            rows_ingested = 1024_u64,
            retention_window_active = true,
            eviction_count = 7_u64,
            "heartbeat",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["rows_ingested"], 1024);
    assert_eq!(fields["retention_window_active"], true);
    assert_eq!(fields["eviction_count"], 7);
}

#[test]
fn scrubber_passes_viz_tick_obs_plan_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "viz.tick",
            query_latency_ms = 18.5_f64,
            subscribers_active = 2_u64,
            "heartbeat",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["query_latency_ms"], 18.5);
    assert_eq!(fields["subscribers_active"], 2);
}

#[test]
fn scrubber_passes_plugins_tick_obs_plan_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "plugins.tick",
            loaded_count = 4_u64,
            active_invocations = 1_u64,
            "heartbeat",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["loaded_count"], 4);
    assert_eq!(fields["active_invocations"], 1);
}

#[test]
fn scrubber_redacts_non_allowlisted_field_on_each_tick_target() {
    let defaults = make_defaults(None, None);
    for target in ["ingest.tick", "buffer.tick", "viz.tick", "plugins.tick"] {
        let lines = capture_json_lines(defaults.clone(), || match target {
            "ingest.tick" => {
                tracing::info!(target: "ingest.tick", attribute_value = "secret-leak", "heartbeat");
            }
            "buffer.tick" => {
                tracing::info!(target: "buffer.tick", query_param = "DROP TABLE", "heartbeat");
            }
            "viz.tick" => {
                tracing::info!(target: "viz.tick", attribute_value = "leaks", "heartbeat");
            }
            "plugins.tick" => {
                tracing::info!(target: "plugins.tick", plugin_path = "/secret/path.wasm", "heartbeat");
            }
            _ => unreachable!(),
        });
        assert_eq!(lines.len(), 1, "expected one line for {target}");
        let fields = &lines[0]["fields"];
        for (k, v) in fields.as_object().unwrap() {
            if matches!(
                k.as_str(),
                "service.name"
                    | "service.version"
                    | "deployment.environment"
                    | "ci.run.id"
                    | "git.commit.sha"
            ) {
                continue;
            }
            assert_eq!(
                v, "<redacted>",
                "field {k} should be redacted for target {target}"
            );
        }
    }
}

#[test]
fn scrubber_redacts_unknown_target() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "unknown_module", arbitrary = "leaks", "test");
    });
    assert_eq!(lines[0]["fields"]["arbitrary"], "<redacted>");
}

#[test]
fn json_schema_conformance() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "ingest", span_count = 1_i64, "first");
        tracing::warn!(target: "ingest", span_count = 2_i64, "second");
        tracing::error!(target: "ingest", span_count = 3_i64, "third");
    });
    assert_eq!(lines.len(), 3);
    for line in &lines {
        assert!(line.get("timestamp").is_some(), "missing timestamp");
        assert!(line.get("level").is_some(), "missing level");
        assert!(line.get("target").is_some(), "missing target");
        assert!(line.get("message").is_some(), "missing message");
        assert!(line.get("fields").is_some(), "missing fields");
        assert!(line["fields"].get("service.name").is_some());
        assert!(line["fields"].get("service.version").is_some());
        assert!(line["fields"].get("deployment.environment").is_some());
    }
}

#[test]
fn timestamp_is_rfc3339_with_millis() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(target: "ingest", span_count = 1_i64, "ping");
    });
    let ts = lines[0]["timestamp"].as_str().unwrap();
    assert!(
        chrono::DateTime::parse_from_rfc3339(ts).is_ok(),
        "timestamp should parse as RFC 3339: {ts}"
    );
}

#[test]
fn log_basename_returns_filename_only() {
    assert_eq!(
        log_basename(Path::new(
            "/home/user/.andromeda-pulse/plugins/my-plugin.wasm"
        )),
        Some("my-plugin.wasm")
    );
    assert_eq!(
        log_basename(Path::new(
            "C:/Users/alice/AppData/andromeda-pulse/plugins/x.wasm"
        )),
        Some("x.wasm")
    );
    assert_eq!(log_basename(Path::new("plain.wasm")), Some("plain.wasm"));
}

#[test]
fn allowlist_for_target_handles_prefixes_and_ticks() {
    let al = AllowList::production();
    assert!(al.for_target("ingest").is_some(), "exact match");
    assert!(al.for_target("ingest.tick").is_some(), "tick suffix");
    assert!(al.for_target("ingest.grpc.export").is_some(), "dot prefix");
    assert!(
        al.for_target("ingest::grpc::request").is_some(),
        "colon prefix"
    );
    assert!(
        al.for_target("a11y::assertion").is_some(),
        "exact a11y target"
    );
    assert!(
        al.for_target("metric.snapshot.token_count_ms").is_some(),
        "metric.* prefix"
    );
    assert!(
        al.for_target("unknown.module.target").is_none(),
        "unknown target redacts everything"
    );
}

#[test]
fn allowlist_for_target_resolves_ui_bridge_error_namespace() {
    // chunk #26: From<E> for AppError impls in crates/ui-bridge/src/contract.rs
    // emit tracing::warn! at target "ui-bridge.error.{variant}". Each variant
    // MUST resolve to its own per-variant entry with `error_category`,
    // `source_kind`, `source_crate` allowed (plus per-variant structured
    // handle: `field` for validation, `plugin_id` for plugin, `resource`
    // for not_found). Without these, default-deny redacts the conversion
    // event payload and the obs-plan §10 module-boundary error logging
    // contract is silently violated.
    let al = AllowList::production();
    for variant in [
        "ui-bridge.error.validation",
        "ui-bridge.error.not_found",
        "ui-bridge.error.internal",
        "ui-bridge.error.plugin",
        "ui-bridge.error.storage",
        "ui-bridge.error.ingest",
    ] {
        let set = al
            .for_target(variant)
            .unwrap_or_else(|| panic!("expected per-variant entry for {variant}"));
        for required in ["error_category", "source_kind", "source_crate"] {
            assert!(set.contains(required), "{variant} must permit `{required}`",);
        }
    }
    let validation = al
        .for_target("ui-bridge.error.validation")
        .expect("validation entry");
    assert!(
        validation.contains("field"),
        "ui-bridge.error.validation must permit `field` for aria-describedby binding"
    );
    let plugin = al
        .for_target("ui-bridge.error.plugin")
        .expect("plugin entry");
    assert!(
        plugin.contains("plugin_id"),
        "ui-bridge.error.plugin must permit `plugin_id`"
    );
    let not_found = al
        .for_target("ui-bridge.error.not_found")
        .expect("not_found entry");
    assert!(
        not_found.contains("resource"),
        "ui-bridge.error.not_found must permit `resource`"
    );
}

#[test]
fn allowlist_for_target_resolves_introspection_namespace() {
    // chunk #27: IntrospectionApi resolvers in crates/ui-bridge/src/health.rs
    // emit tracing::info! at target "ui-bridge.{procedure}". Each procedure
    // MUST resolve to its own per-procedure entry containing `method_name`
    // and `result_type` (the boundary-call wrapper field set per obs-plan
    // §6) plus per-procedure structured handles. Without exact-match
    // entries, the resolver's split('.').next() fallback collapses these
    // to the bare `ui-bridge` entry whose field set differs.
    let al = AllowList::production();
    for procedure in [
        "ui-bridge.app_info",
        "ui-bridge.health",
        "ui-bridge.ready",
        "ui-bridge.get_settings",
        "ui-bridge.update_settings",
    ] {
        let set = al
            .for_target(procedure)
            .unwrap_or_else(|| panic!("expected per-procedure entry for {procedure}"));
        for required in ["method_name", "result_type"] {
            assert!(
                set.contains(required),
                "{procedure} must permit `{required}`",
            );
        }
    }
    let app_info = al.for_target("ui-bridge.app_info").expect("app_info entry");
    for required in [
        "name",
        "version",
        "rust_version",
        "tauri_version",
        "build_profile",
    ] {
        assert!(
            app_info.contains(required),
            "ui-bridge.app_info must permit `{required}`"
        );
    }
    let ready = al.for_target("ui-bridge.ready").expect("ready entry");
    for required in [
        "ready",
        "duckdb_connection",
        "ingest_mpsc_capacity_pct",
        "broadcast_subscribers",
        "plugins_loaded",
        "mcp_server_enabled",
    ] {
        assert!(
            ready.contains(required),
            "ui-bridge.ready must permit `{required}`"
        );
    }
    let update_settings = al
        .for_target("ui-bridge.update_settings")
        .expect("update_settings entry");
    assert!(
        update_settings.contains("setting_keys_changed"),
        "ui-bridge.update_settings must permit `setting_keys_changed`"
    );
    let drift = al
        .for_target("xtask.capability_drift")
        .expect("xtask capability_drift entry");
    for required in [
        "missing_count",
        "extra_count",
        "drift_state",
        "top_5_drifted_names",
    ] {
        assert!(
            drift.contains(required),
            "xtask.capability_drift must permit `{required}`"
        );
    }
}

#[test]
fn allowlist_for_target_resolves_duckdb_append_to_arrow_appender_fields() {
    // chunk #65: production consumer::dispatch_batch emits one
    // tracing::info!(target: "duckdb.append", ...) per table append
    // (spans / span_events / metrics_points / log_records) with the
    // rows_appended + duration_ms + table_name field shape. Error path
    // (target: "duckdb.append", reject_reason = ...) is also covered.
    // Without this exact entry, the resolver's split('.').next() fallback
    // returns None ("duckdb" key absent) and JsonFieldVisitor silently
    // redacts all fields per the default-deny posture.
    let al = AllowList::production();
    let entry = al
        .for_target("duckdb.append")
        .expect("expected duckdb entry resolving via split('.').next() from duckdb.append");
    for required in [
        "rows_appended",
        "duration_ms",
        "table_name",
        "reject_reason",
    ] {
        assert!(
            entry.contains(required),
            "duckdb.append must permit `{required}`"
        );
    }
}

#[test]
fn allowlist_for_target_resolves_metric_webgpu_frame_duration_ms() {
    // chunk #28: WebGPU canvas substrate emits per-frame timing via
    // TauRPC telemetry.frontend.record_frame_ms (resolver lands chunk #29);
    // the metric event itself rides at target "metric.webgpu.frame_duration_ms"
    // with bounded enum labels per obs-plan §5 cardinality discipline.
    let al = AllowList::production();
    let frame = al
        .for_target("metric.webgpu.frame_duration_ms")
        .expect("metric.webgpu.frame_duration_ms entry");
    for required in [
        "duration_ms",
        "wgpu_backend",
        "webview_backend",
        "timing_method",
    ] {
        assert!(
            frame.contains(required),
            "metric.webgpu.frame_duration_ms must permit `{required}`"
        );
    }
}

#[test]
fn allowlist_for_target_resolves_ui_layout_transition_to_expanded_field_set() {
    // chunk #30: apply_widget_settings extends the existing chunk #24
    // ui.layout.transition allowlist with always_on_top + duration_ms
    // beyond the original layout_mode_from / layout_mode_to / tray_visible
    // triplet. Without these, default-deny redacts the new fields and
    // the apply-settings observability contract silently regresses.
    let al = AllowList::production();
    let entry = al
        .for_target("ui.layout.transition")
        .expect("ui.layout.transition entry");
    for required in [
        "layout_mode_from",
        "layout_mode_to",
        "tray_visible",
        "always_on_top",
        "duration_ms",
    ] {
        assert!(
            entry.contains(required),
            "ui.layout.transition must permit `{required}`"
        );
    }
}

#[test]
fn allowlist_for_target_resolves_snapshot_curate_field_set() {
    // chunk #39: curation primitives in `crates/snapshot/{dedupe,anomaly,
    // critical_path}.rs` instrument as `snapshot.curate.{operation}` and
    // emit per-pipeline counts (input_row_count / output_row_count /
    // dedup_count / latency_outlier_count / error_cluster_count /
    // cardinality_spike_count / critical_path_span_count / total_span_count
    // / orphan_parent_count / anomaly_markers_count / duration_ms). For_target
    // resolves these via split('.').next() fallback to the `snapshot` entry,
    // so the entry must permit ALL fields used across the curation pipeline
    // OR default-deny silently redacts them and obs-plan §3 P2 + §8
    // snapshot whitelist contract regresses.
    //
    // chunk #40: aggregation + attribute_filter primitives extend the
    // namespace with `snapshot.curate.aggregate` + `snapshot.curate.attribute_filter`
    // targets emitting metric_input_count / metric_output_count /
    // service_count / kept_attribute_count / dropped_attribute_count /
    // p50_ms / p95_ms / p99_ms / max_ms. Same fall-through resolution
    // path; same `snapshot` entry must permit them.
    let al = AllowList::production();
    for target in [
        "snapshot.curate.dedup",
        "snapshot.curate.anomaly_detect",
        "snapshot.curate.critical_path_extract",
        "snapshot.curate.full_pipeline",
        "snapshot.curate.aggregate",
        "snapshot.curate.attribute_filter",
    ] {
        let set = al
            .for_target(target)
            .unwrap_or_else(|| panic!("expected snapshot entry resolution for {target}"));
        for required in [
            "input_row_count",
            "output_row_count",
            "dedup_count",
            "latency_outlier_count",
            "error_cluster_count",
            "cardinality_spike_count",
            "critical_path_span_count",
            "total_span_count",
            "orphan_parent_count",
            "anomaly_markers_count",
            "duration_ms",
            "metric_input_count",
            "metric_output_count",
            "service_count",
            "kept_attribute_count",
            "dropped_attribute_count",
            "p50_ms",
            "p95_ms",
            "p99_ms",
            "max_ms",
        ] {
            assert!(
                set.contains(required),
                "{target} must permit `{required}` (snapshot allowlist coverage)",
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_baseline_migrate_field_set() {
    // chunk #70: BaselineState → corpus migration emits one-time legacy
    // migration tracing events at `triage.baseline.migrate` (info) +
    // `triage.baseline.migrate.failed` (warn). Both targets require
    // explicit-leaf AllowList entries (no fall-through to the broader
    // `triage` entry, whose field set is unrelated). PII discipline
    // per .claude/rules/observability.md 2026-05-17 session 84 +
    // chunks #62/#63/#64 aggregate-only mandate: bounded enum +
    // integer-value fields ONLY; NO `service_name` / `scope_id` /
    // `span_id` / `trace_id` / `operation_name` / `legacy_path`
    // admitted (the latter excludes raw file paths per Vector 6).
    let al = AllowList::production();

    let migrate = al
        .for_target("triage.baseline.migrate")
        .expect("expected triage.baseline.migrate entry");
    for required in [
        "legacy_state_size_bytes",
        "migrated_service_count",
        "duration_ms",
        "migration_outcome",
    ] {
        assert!(
            migrate.contains(required),
            "triage.baseline.migrate must permit `{required}`",
        );
    }

    let failed = al
        .for_target("triage.baseline.migrate.failed")
        .expect("expected triage.baseline.migrate.failed entry");
    for required in ["error_category", "duration_ms"] {
        assert!(
            failed.contains(required),
            "triage.baseline.migrate.failed must permit `{required}`",
        );
    }

    // PII guard: neither entry may admit per-service or per-record
    // identifiers (aggregate-only convention).
    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
        "legacy_path",
        "attribute",
        "body",
    ];
    for target_set in [migrate, failed] {
        for k in &banned {
            assert!(
                !target_set.contains(k),
                "baseline migrate AllowList must NOT permit PII field `{k}`",
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_lifecycle_persist_field_set() {
    // chunk #71: ServiceRegistry corpus persistence emits aggregate-only
    // events at `triage.lifecycle.persist` (info, per-tick success) +
    // `triage.lifecycle.persist.error` (warn, per-tick failure).
    // PII discipline mirrors chunk #70 baseline.persist precedent —
    // bounded enum tags + integer-value fields ONLY.
    let al = AllowList::production();

    let persist = al
        .for_target("triage.lifecycle.persist")
        .expect("expected triage.lifecycle.persist entry");
    for required in [
        "service_count",
        "state_size_bytes",
        "duration_ms",
        "persist_kind",
        "corpus_basename",
    ] {
        assert!(
            persist.contains(required),
            "triage.lifecycle.persist must permit `{required}`",
        );
    }

    let persist_err = al
        .for_target("triage.lifecycle.persist.error")
        .expect("expected triage.lifecycle.persist.error entry");
    for required in ["error_category", "duration_ms"] {
        assert!(
            persist_err.contains(required),
            "triage.lifecycle.persist.error must permit `{required}`",
        );
    }

    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
        "legacy_path",
        "attribute",
        "body",
    ];
    for target_set in [persist, persist_err] {
        for k in &banned {
            assert!(
                !target_set.contains(k),
                "lifecycle persist AllowList must NOT permit PII field `{k}`",
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_lifecycle_corpus_restore_extended_field_set() {
    // chunk #67 stub fields preserved; chunk #71 adds
    // `restored_service_count` + `duration_ms` for production
    // emission per plan §11.
    let al = AllowList::production();
    let entry = al
        .for_target("triage.lifecycle.corpus_restore")
        .expect("expected triage.lifecycle.corpus_restore entry");
    for required in ["kind", "count", "restored_service_count", "duration_ms"] {
        assert!(
            entry.contains(required),
            "triage.lifecycle.corpus_restore must permit `{required}` (chunk #71 EXTEND)",
        );
    }
    // PII guard.
    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
    ];
    for k in &banned {
        assert!(
            !entry.contains(k),
            "lifecycle.corpus_restore must NOT permit PII field `{k}`",
        );
    }

    // Metric counter sibling.
    let metric = al
        .for_target("metric.triage.lifecycle.corpus_restore_count_total")
        .expect("expected metric.triage.lifecycle.corpus_restore_count_total entry");
    for required in ["value", "kind"] {
        assert!(
            metric.contains(required),
            "metric.triage.lifecycle.corpus_restore_count_total must permit `{required}`",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_storm_persist_field_set() {
    // chunk #71: RetryStormState corpus persistence emits aggregate-only
    // events at `triage.pattern.storm.persist` (info) +
    // `triage.pattern.storm.persist.error` (warn). PII discipline
    // mirrors baseline / lifecycle pattern — bounded enum tags +
    // integer-value fields ONLY.
    let al = AllowList::production();

    let persist = al
        .for_target("triage.pattern.storm.persist")
        .expect("expected triage.pattern.storm.persist entry");
    for required in [
        "fingerprint_count",
        "state_size_bytes",
        "duration_ms",
        "persist_kind",
        "corpus_basename",
    ] {
        assert!(
            persist.contains(required),
            "triage.pattern.storm.persist must permit `{required}`",
        );
    }

    let persist_err = al
        .for_target("triage.pattern.storm.persist.error")
        .expect("expected triage.pattern.storm.persist.error entry");
    for required in ["error_category", "duration_ms"] {
        assert!(
            persist_err.contains(required),
            "triage.pattern.storm.persist.error must permit `{required}`",
        );
    }

    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
        "fingerprint_hex_full",
        "legacy_path",
    ];
    for target_set in [persist, persist_err] {
        for k in &banned {
            assert!(
                !target_set.contains(k),
                "storm persist AllowList must NOT permit PII field `{k}`",
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_storm_corpus_restore_field_set() {
    // chunk #71: storm corpus restore tracing target + metric
    // counter. Both NEW entries per plan §11.
    let al = AllowList::production();

    let restore = al
        .for_target("triage.pattern.storm.corpus_restore")
        .expect("expected triage.pattern.storm.corpus_restore entry");
    for required in ["restored_fingerprint_count", "duration_ms", "kind"] {
        assert!(
            restore.contains(required),
            "triage.pattern.storm.corpus_restore must permit `{required}`",
        );
    }

    let metric = al
        .for_target("metric.triage.pattern.storm.corpus_restore_count_total")
        .expect("expected metric.triage.pattern.storm.corpus_restore_count_total entry");
    for required in ["value", "kind"] {
        assert!(
            metric.contains(required),
            "metric.triage.pattern.storm.corpus_restore_count_total must permit `{required}`",
        );
    }

    // PII guard on both.
    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
        "fingerprint_hex_full",
    ];
    for target_set in [restore, metric] {
        for k in &banned {
            assert!(
                !target_set.contains(k),
                "storm restore AllowList must NOT permit PII field `{k}`",
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_curation_field_set() {
    // chunk #58: curation primitives (dedupe / anomaly / critical_path /
    // aggregation) moved from crates/snapshot to crates/curation. The
    // #[tracing::instrument] decorators emit spans at default module-path
    // targets `curation::dedupe`, `curation::anomaly`, etc. for_target
    // resolves via split("::").next() fall-through to the `curation`
    // entry, which MUST permit the union of fields emitted across the
    // 4 primitives or default-deny silently redacts them.
    let al = AllowList::production();
    for target in [
        "curation::dedupe",
        "curation::anomaly",
        "curation::critical_path",
        "curation::aggregation",
    ] {
        let set = al
            .for_target(target)
            .unwrap_or_else(|| panic!("expected curation entry resolution for {target}"));
        for required in [
            "input_row_count",
            "output_row_count",
            "dedup_count",
            "latency_outlier_count",
            "error_cluster_count",
            "cardinality_spike_count",
            "critical_path_span_count",
            "total_span_count",
            "orphan_parent_count",
            "anomaly_markers_count",
            "duration_ms",
            "metric_input_count",
            "metric_output_count",
            "service_count",
            "p50_ms",
            "p95_ms",
            "p99_ms",
            "max_ms",
        ] {
            assert!(
                set.contains(required),
                "{target} must permit `{required}` (curation allowlist coverage)",
            );
        }
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_snapshot_curate_field() {
    // chunk #39: per security plan §Logging Vector 1, raw OTLP attribute
    // values (service_name / span attributes / span content) MUST be
    // redacted at the formatter layer. The snapshot allowlist only
    // permits scalar counts + categorical bounds; non-allowlisted fields
    // like `service_name`, `attributes`, `query_text` MUST be stripped
    // even though they would semantically resolve via split('.').next() to
    // the `snapshot` entry.
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "snapshot.curate.dedup",
            dedup_count = 5_u64,
            service_name = "checkout",
            attributes = "{\"user_id\": \"abc\"}",
            query_text = "SELECT * FROM spans",
            "dedup complete",
        );
    });
    let fields = &lines[0]["fields"];
    // Allowlisted field present.
    assert_eq!(fields["dedup_count"], 5);
    // Non-allowlisted fields redacted to "<redacted>" sentinel.
    for forbidden in ["service_name", "attributes", "query_text"] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted per obs-plan Vector 1+5 + snapshot allowlist (default-deny)",
        );
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_snapshot_aggregate_field() {
    // chunk #40: aggregation primitive emits at `snapshot.curate.aggregate`
    // target which resolves via fall-through to the `snapshot` allowlist
    // entry. Non-allowlisted fields (raw attribute values that may
    // incidentally carry secrets per security plan §Logging Vector 1)
    // MUST be redacted by the JsonFieldVisitor.
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "snapshot.curate.aggregate",
            p99_ms = 42_u64,
            attribute_value = "secret-token",
            attribute_key = "user.id",
            raw_metric_payload = "{\"a\":1}",
            "aggregation complete",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["p99_ms"], 42);
    for forbidden in ["attribute_value", "attribute_key", "raw_metric_payload"] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted at snapshot.curate.aggregate target (default-deny via snapshot fall-through)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_service_went_silent_evaluate_field_set() {
    // chunk #64: per-tick aggregate event for ServiceWentSilent gating.
    // Bounded-cardinality count fields only — no per-service identifiers
    // per chunk #62/#63 PII discipline.
    let al = AllowList::production();
    let set = al
        .for_target("triage.baseline.service_went_silent.evaluate")
        .expect("triage.baseline.service_went_silent.evaluate entry");
    for required in [
        "services_tracked",
        "services_in_bootstrap",
        "services_ready",
        "silence_cues_emitted",
    ] {
        assert!(
            set.contains(required),
            "service_went_silent.evaluate must permit `{required}`",
        );
    }
    for banned in [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
    ] {
        assert!(
            !set.contains(banned),
            "service_went_silent.evaluate must NOT permit `{banned}` (chunk #62/#63 PII discipline)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_service_cap_exceeded_field_set() {
    // chunk #64: aggregate warn when service.name cardinality cap is
    // reached. dropped_count + cap fields only — no per-service identifiers.
    let al = AllowList::production();
    let set = al
        .for_target("triage.baseline.service_cap_exceeded")
        .expect("triage.baseline.service_cap_exceeded entry");
    for required in ["dropped_count", "cap"] {
        assert!(
            set.contains(required),
            "service_cap_exceeded must permit `{required}`",
        );
    }
    for banned in ["service_name", "scope_id"] {
        assert!(
            !set.contains(banned),
            "service_cap_exceeded must NOT permit `{banned}` (chunk #62/#63 PII discipline)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_metric_activity_floor_bootstrap_state_field_set() {
    // chunk #64: per-tick bootstrap-state gauge. Aggregate count fields
    // only — no per-service identifiers.
    let al = AllowList::production();
    let set = al
        .for_target("metric.triage.activity_floor.bootstrap_state")
        .expect("metric.triage.activity_floor.bootstrap_state entry");
    for required in [
        "value",
        "services_in_bootstrap",
        "services_ready",
        "services_tracked",
    ] {
        assert!(
            set.contains(required),
            "metric.triage.activity_floor.bootstrap_state must permit `{required}`",
        );
    }
    for banned in ["service_name", "scope_id"] {
        assert!(
            !set.contains(banned),
            "metric.triage.activity_floor.bootstrap_state must NOT permit `{banned}` (chunk #62/#63 PII discipline)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_storm_detected_field_set() {
    // chunk #66: retry storm detector emits at triage.pattern.storm.*
    // targets. Aggregate identifier-class fields only — fingerprint_hex
    // is the 8-char hash prefix (bounded cardinality opaque ID),
    // severity_hint + cue_kind are bounded enum tags, occurrence_count
    // + window_seconds are bounded numerics. NO `service_name`/`scope_id`
    // even though the cue payload carries them (broadcast IS the surface
    // for those; self-observation log is aggregate-only per chunk #62/
    // #63/#64 convention reaffirmed by testing.md 2026-05-17 session 84).
    let al = AllowList::production();
    for target in [
        "triage.pattern.storm.detected",
        "triage.pattern.storm.emit",
        "triage.pattern.storm.tick",
        "metric.triage.pattern.storm_detected_count",
        "metric.triage.pattern.fingerprints_tracked",
        "metric.triage.pattern.fingerprint_evicted_count",
    ] {
        let set = al
            .for_target(target)
            .unwrap_or_else(|| panic!("{target} allowlist entry MUST exist"));
        for banned in [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "attribute",
            "instrumentation_scope",
            "body",
            "exception_message",
            "exception_stacktrace",
        ] {
            assert!(
                !set.contains(banned),
                "{target} must NOT permit `{banned}` (chunk #62/#63/#64 PII discipline)",
            );
        }
    }
    // Spot-check required fields on the primary detection target.
    let detected = al
        .for_target("triage.pattern.storm.detected")
        .expect("storm.detected entry");
    for required in [
        "cue_kind",
        "severity_hint",
        "occurrence_count",
        "window_seconds",
        "fingerprint_hex",
    ] {
        assert!(
            detected.contains(required),
            "triage.pattern.storm.detected must permit `{required}`",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_triage_lifecycle_tick_field_set() {
    // chunk #67: service lifecycle heartbeat tick emits aggregate
    // per-state counts only. NO per-service identifiers — those ride
    // the `pulse://stream/service-lifecycle` broadcast topic instead.
    let al = AllowList::production();
    let set = al
        .for_target("triage.lifecycle.tick")
        .expect("triage.lifecycle.tick entry");
    for required in [
        "tracked_services_total",
        "services_unknown",
        "services_bootstrapping",
        "services_active",
        "services_quiet",
        "services_silent",
        "services_dormant",
        "services_archived",
    ] {
        assert!(
            set.contains(required),
            "triage.lifecycle.tick must permit `{required}`",
        );
    }
    for banned in [
        "service_name",
        "service_id",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
        "attribute",
        "instrumentation_scope",
        "body",
    ] {
        assert!(
            !set.contains(banned),
            "triage.lifecycle.tick must NOT permit `{banned}` (chunk #62/#63/#64 PII discipline)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_triage_lifecycle_transition_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("triage.lifecycle.transition")
        .expect("triage.lifecycle.transition entry");
    for required in ["from_state", "to_state", "count"] {
        assert!(
            set.contains(required),
            "triage.lifecycle.transition must permit `{required}`",
        );
    }
    for banned in [
        "service",
        "service_name",
        "service_id",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
    ] {
        assert!(
            !set.contains(banned),
            "triage.lifecycle.transition must NOT permit `{banned}` (aggregate-only per chunk #67)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_triage_lifecycle_corpus_restore_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("triage.lifecycle.corpus_restore")
        .expect("triage.lifecycle.corpus_restore entry");
    for required in ["kind", "count"] {
        assert!(
            set.contains(required),
            "triage.lifecycle.corpus_restore must permit `{required}`",
        );
    }
    for banned in ["service_name", "service_id", "scope_id"] {
        assert!(
            !set.contains(banned),
            "triage.lifecycle.corpus_restore must NOT permit `{banned}` (chunk #67 PII discipline)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_services_list_with_states_request_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("services.list_with_states.request")
        .expect("services.list_with_states.request entry");
    for required in ["item_count", "traceparent"] {
        assert!(
            set.contains(required),
            "services.list_with_states.request must permit `{required}`",
        );
    }
    for banned in [
        "service_name",
        "service_id",
        "scope_id",
        "items",
        "manual_override",
    ] {
        assert!(
            !set.contains(banned),
            "services.list_with_states.request must NOT permit `{banned}` (chunk #67 PII discipline — response body excluded from log)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_pipeline_l1b_tracked_services_total_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("pipeline.l1b.tracked_services_total")
        .expect("pipeline.l1b.tracked_services_total entry");
    assert!(set.contains("value"), "must permit `value`");
    for banned in ["service_name", "service_id", "scope_id"] {
        assert!(
            !set.contains(banned),
            "pipeline.l1b.tracked_services_total must NOT permit `{banned}` (chunk #67 PII discipline)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_metric_triage_lifecycle_state_distribution_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("metric.triage.lifecycle.state_distribution")
        .expect("metric.triage.lifecycle.state_distribution entry");
    for required in [
        "value",
        "services_unknown",
        "services_bootstrapping",
        "services_active",
        "services_quiet",
        "services_silent",
        "services_dormant",
        "services_archived",
    ] {
        assert!(
            set.contains(required),
            "metric.triage.lifecycle.state_distribution must permit `{required}`",
        );
    }
    for banned in ["service_name", "service_id", "scope_id"] {
        assert!(
            !set.contains(banned),
            "metric.triage.lifecycle.state_distribution must NOT permit `{banned}` (chunk #67 PII discipline)",
        );
    }
}

// ===== Chunk #69 Phase B Session 6 — Drain + diagnostics AllowList tests =====

#[test]
fn allowlist_for_target_extends_buffer_with_drain_heartbeat_fields() {
    let al = AllowList::production();
    let set = al.for_target("buffer").expect("buffer entry");
    for required in ["drain_template_count", "drain_lru_evictions_since_tick"] {
        assert!(
            set.contains(required),
            "buffer must permit `{required}` (chunk #69 buffer.tick drain fields)",
        );
    }
    // Chunk #69 AGGREGATE-ONLY discipline applies even though we did
    // not remove existing buffer fields — the new drain fields are
    // bounded counts; per-template-content fields stay forbidden.
    for banned in ["template_text", "sample_message", "service_name"] {
        assert!(
            !set.contains(banned),
            "buffer must NOT permit `{banned}` (chunk #69 aggregate-only discipline)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_drain_persistence_load_ok_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("drain.persistence.load.ok")
        .expect("drain.persistence.load.ok entry");
    assert!(
        set.contains("template_count"),
        "drain.persistence.load.ok must permit `template_count`",
    );
    for banned in [
        "template_text",
        "sample_message",
        "service_name",
        "scope_id",
    ] {
        assert!(
            !set.contains(banned),
            "drain.persistence.load.ok must NOT permit `{banned}` (chunk #69 aggregate-only)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_drain_persistence_unavailable_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("drain.persistence.unavailable")
        .expect("drain.persistence.unavailable entry");
    assert!(
        set.contains("reason"),
        "drain.persistence.unavailable must permit `reason`",
    );
    for banned in [
        "template_text",
        "sample_message",
        "service_name",
        "stacktrace",
    ] {
        assert!(
            !set.contains(banned),
            "drain.persistence.unavailable must NOT permit `{banned}` (PII / opaque-error discipline)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_diagnostics_template_distribution_request_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("diagnostics.template_distribution.request")
        .expect("diagnostics.template_distribution.request entry");
    for required in ["top_n", "result_count", "duration_ms"] {
        assert!(
            set.contains(required),
            "diagnostics.template_distribution.request must permit `{required}`",
        );
    }
    for banned in [
        "template_text",
        "sample_message",
        "service_name",
        "templates",
        "content",
    ] {
        assert!(
            !set.contains(banned),
            "diagnostics.template_distribution.request must NOT permit `{banned}` (chunk #69 aggregate-only)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_metric_pipeline_l1c_drain_template_count_total_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("metric.pipeline.l1c.drain_template_count_total")
        .expect("metric.pipeline.l1c.drain_template_count_total entry");
    assert!(
        set.contains("value"),
        "metric.pipeline.l1c.drain_template_count_total must permit `value`",
    );
    for banned in [
        "template_text",
        "sample_message",
        "service_name",
        "template_id",
    ] {
        assert!(
            !set.contains(banned),
            "metric.pipeline.l1c.drain_template_count_total must NOT permit `{banned}` (chunk #69 aggregate-only)",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_metric_pipeline_l1c_drain_assignment_latency_p99_microseconds_field_set()
 {
    let al = AllowList::production();
    let set = al
        .for_target("metric.pipeline.l1c.drain_assignment_latency_p99_microseconds")
        .expect("metric.pipeline.l1c.drain_assignment_latency_p99_microseconds entry");
    assert!(
        set.contains("value"),
        "metric.pipeline.l1c.drain_assignment_latency_p99_microseconds must permit `value`",
    );
    for banned in [
        "template_text",
        "sample_message",
        "service_name",
        "template_id",
    ] {
        assert!(
            !set.contains(banned),
            "metric.pipeline.l1c.drain_assignment_latency_p99_microseconds must NOT permit `{banned}` (chunk #69 aggregate-only)",
        );
    }
}

#[test]
fn allowlist_resolves_each_cadence_target_with_expected_fields() {
    let al = AllowList::production();

    let tick = al.for_target("cadence.tick").expect("cadence.tick entry");
    for required in [
        "tier",
        "mode",
        "cycles_executed",
        "queries_executed",
        "queries_succeeded",
        "next_due_ms",
        "last_executed_at_ms",
        "tier2_acceleration_enabled",
    ] {
        assert!(
            tick.contains(required),
            "cadence.tick must permit `{required}`"
        );
    }

    let trigger = al
        .for_target("cadence.trigger")
        .expect("cadence.trigger entry");
    for required in ["tier", "mode", "cue_kind", "priority"] {
        assert!(
            trigger.contains(required),
            "cadence.trigger must permit `{required}`"
        );
    }

    let load = al
        .for_target("cadence.config.load")
        .expect("cadence.config.load entry");
    for required in [
        "baseline_seconds",
        "accelerated_seconds",
        "reflection_seconds",
        "tier2_acceleration_enabled",
    ] {
        assert!(
            load.contains(required),
            "cadence.config.load must permit `{required}`"
        );
    }

    let floor = al
        .for_target("cadence.config.safety_floor")
        .expect("cadence.config.safety_floor entry");
    for required in ["field", "requested_seconds", "enforced_seconds"] {
        assert!(
            floor.contains(required),
            "cadence.config.safety_floor must permit `{required}`"
        );
    }

    let metric = al
        .for_target("metric.pipeline.l3.digests_assembled_total")
        .expect("metric.pipeline.l3.digests_assembled_total entry");
    for required in ["value", "mode", "queries_executed"] {
        assert!(
            metric.contains(required),
            "metric.pipeline.l3.digests_assembled_total must permit `{required}`"
        );
    }
}

#[test]
fn cadence_targets_ban_pii_fields_per_chunk_62_precedent() {
    let al = AllowList::production();
    let cadence_targets = [
        "cadence.tick",
        "cadence.trigger",
        "cadence.config.load",
        "cadence.config.safety_floor",
        "metric.pipeline.l3.digests_assembled_total",
    ];
    let banned_fields = [
        "service",
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
        "digest_body",
        "inference_input",
        "inference_output",
    ];
    for target in cadence_targets {
        let set = al
            .for_target(target)
            .unwrap_or_else(|| panic!("expected allowlist entry for `{target}`"));
        for banned in banned_fields {
            assert!(
                !set.contains(banned),
                "cadence target `{target}` must NOT permit `{banned}` field (chunk #62 aggregate-only precedent)"
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_diagnostics_prefix_fallback_to_diagnostics_entry() {
    // Verify the dotted-target prefix-strip fallback per CLAUDE.md
    // 2026-05-07: `diagnostics.X` (unknown sub-namespace) resolves via
    // split('.').next() → "diagnostics" entry instead of redacting all
    // fields. This is the safety net under the explicit-leaf entries.
    let al = AllowList::production();
    let set = al
        .for_target("diagnostics.unknown.future.target")
        .expect("diagnostics fallback entry");
    for required in ["top_n", "result_count", "duration_ms"] {
        assert!(
            set.contains(required),
            "diagnostics fallback must permit `{required}`",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_snapshot_render_markdown_field_set() {
    // chunk #41: markdown formatter emits at `snapshot.render.markdown`
    // target. Registered explicitly so for_target() exact-match wins over
    // the `snapshot` crate-level fallback (which would silently redact
    // `markdown_size_bytes` / `budget_token_limit` / `truncated_*_count`)
    // per .claude/rules/observability.md Session Addition 2026-05-07.
    let al = AllowList::production();
    let set = al
        .for_target("snapshot.render.markdown")
        .expect("snapshot.render.markdown entry");
    for required in [
        "markdown_size_bytes",
        "section_count",
        "token_count_actual",
        "budget_token_limit",
        "anomaly_marker_count",
        "critical_path_span_count",
        "truncated_span_count",
        "truncated_attribute_count",
        "duration_ms",
    ] {
        assert!(
            set.contains(required),
            "snapshot.render.markdown must permit `{required}`",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_snapshot_token_count_validate_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("snapshot.token.count.validate")
        .expect("snapshot.token.count.validate entry");
    for required in [
        "token_count_actual",
        "token_budget_limit",
        "budget_exceeded",
        "duration_ms",
    ] {
        assert!(
            set.contains(required),
            "snapshot.token.count.validate must permit `{required}`",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_metric_snapshot_token_count_ms_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("metric.snapshot.token_count_ms")
        .expect("metric.snapshot.token_count_ms entry");
    for required in [
        "value",
        "duration_ms",
        "token_budget",
        "time_range_minutes",
        "token_count_actual",
        "dedup_count",
        "budget_exceeded",
    ] {
        assert!(
            set.contains(required),
            "metric.snapshot.token_count_ms must permit `{required}`",
        );
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_snapshot_render_markdown_field() {
    // chunk #41: markdown formatter emits at `snapshot.render.markdown`;
    // security plan §Logging Vectors 1 + 4 demand markdown body contents
    // + raw OTLP attribute values + citation anchor labels MUST be
    // redacted. The `target:` argument to tracing macros must be a
    // `&'static str` literal (macro expands to `static __CALLSITE`), so
    // each target gets its own test rather than a for-loop.
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "snapshot.render.markdown",
            duration_ms = 100_u64,
            markdown_body = "# title\nsecret content",
            citation_anchor_label = "user-name=alice",
            attribute_value = "secret-token",
            raw_attribute_value = "Bearer abc",
            service_name_raw = "checkout-prod",
            snapshot_file_content = "## Anomaly\nSensitive",
            raw_span_name = "POST /pay",
            "format event",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["duration_ms"], 100);
    for forbidden in [
        "markdown_body",
        "citation_anchor_label",
        "attribute_value",
        "raw_attribute_value",
        "service_name_raw",
        "snapshot_file_content",
        "raw_span_name",
    ] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted at snapshot.render.markdown target",
        );
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_snapshot_token_count_validate_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "snapshot.token.count.validate",
            token_count_actual = 12_345_u64,
            markdown_body = "# title\nsecret content",
            attribute_value = "secret-token",
            raw_attribute_value = "Bearer abc",
            "validate event",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["token_count_actual"], 12345);
    for forbidden in ["markdown_body", "attribute_value", "raw_attribute_value"] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted at snapshot.token.count.validate target",
        );
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_metric_snapshot_token_count_ms_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.snapshot.token_count_ms",
            value = 42.0_f64,
            duration_ms = 42_u64,
            markdown_body = "# title\nsecret content",
            attribute_value = "secret-token",
            snapshot_file_content = "## Anomaly\nSensitive",
            "metric event",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["duration_ms"], 42);
    for forbidden in ["markdown_body", "attribute_value", "snapshot_file_content"] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted at metric.snapshot.token_count_ms target",
        );
    }
}

#[test]
fn allowlist_for_target_resolves_snapshot_generate_request_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("snapshot.generate.request")
        .expect("snapshot.generate.request entry");
    for required in [
        "budget",
        "result_kind",
        "message",
        "token_count",
        "dual_file_paths_basenames",
        "preset_prompts_count",
    ] {
        assert!(
            set.contains(required),
            "snapshot.generate.request must permit `{required}`",
        );
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_snapshot_generate_request_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "snapshot.generate.request",
            budget = "balanced",
            result_kind = "placeholder",
            message = "Investigate not yet wired (lands chunk #43)",
            clipboard_content = "secret data",
            snapshot_file_content = "## Anomaly\nSensitive",
            raw_attribute_value = "Bearer abc",
            "Investigate trigger reached placeholder",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["budget"], "balanced");
    assert_eq!(fields["result_kind"], "placeholder");
    for forbidden in [
        "clipboard_content",
        "snapshot_file_content",
        "raw_attribute_value",
    ] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted at snapshot.generate.request target",
        );
    }
}

// chunk #43 — paired tests for snapshot.clipboard.write per-leaf entry
#[test]
fn allowlist_for_target_resolves_snapshot_clipboard_write_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("snapshot.clipboard.write")
        .expect("snapshot.clipboard.write entry");
    for required in ["byte_count", "success", "dual_file_paths_basenames"] {
        assert!(
            set.contains(required),
            "snapshot.clipboard.write must permit `{required}`",
        );
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_snapshot_clipboard_write_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "snapshot.clipboard.write",
            byte_count = 1234_u64,
            success = true,
            clipboard_content = "secret canary payload",
            raw_markdown = "## Anomaly\nSensitive",
            "clipboard write event",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["byte_count"], 1234);
    assert_eq!(fields["success"], true);
    for forbidden in ["clipboard_content", "raw_markdown"] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted at snapshot.clipboard.write target",
        );
    }
}

// chunk #43 — paired tests for snapshot.notification.dispatch per-leaf entry
#[test]
fn allowlist_for_target_resolves_snapshot_notification_dispatch_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("snapshot.notification.dispatch")
        .expect("snapshot.notification.dispatch entry");
    for required in ["notification_kind", "dispatch_success"] {
        assert!(
            set.contains(required),
            "snapshot.notification.dispatch must permit `{required}`",
        );
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_snapshot_notification_dispatch_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "snapshot.notification.dispatch",
            notification_kind = "snapshot_ready",
            dispatch_success = true,
            toast_body = "Snapshot ready | 2.5k tokens, secret-canary",
            preset_prompt_text = "Diagnose latency outlier",
            "notification dispatched",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["notification_kind"], "snapshot_ready");
    assert_eq!(fields["dispatch_success"], true);
    for forbidden in ["toast_body", "preset_prompt_text"] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted at snapshot.notification.dispatch target",
        );
    }
}

// chunk #43 — paired tests for workspace.detect per-leaf entry
#[test]
fn allowlist_for_target_resolves_workspace_detect_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target("workspace.detect")
        .expect("workspace.detect entry");
    for required in [
        "workspace_root_basename",
        "project_name",
        "vcs_type",
        "vcs_root_basename",
        "marker_present",
        "detection_latency_ms",
    ] {
        assert!(
            set.contains(required),
            "workspace.detect must permit `{required}`",
        );
    }
}

#[test]
fn scrubber_redacts_non_allowlisted_workspace_detect_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "workspace.detect",
            workspace_root_basename = "example",
            project_name = "example",
            vcs_type = "git",
            marker_present = true,
            detection_latency_ms = 5_u64,
            workspace_root = "/tmp/secret-canary/example",
            vcs_commit_message = "fix: secret canary commit",
            "workspace detected",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["workspace_root_basename"], "example");
    assert_eq!(fields["project_name"], "example");
    assert_eq!(fields["vcs_type"], "git");
    for forbidden in ["workspace_root", "vcs_commit_message"] {
        assert_eq!(
            fields[forbidden], "<redacted>",
            "field `{forbidden}` MUST be redacted at workspace.detect target",
        );
    }
}

#[test]
fn scrubber_passes_port_validation_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "config.load.port_validation",
            env_var_name = "ANDROMEDA_PULSE_OTLP_GRPC_PORT",
            reject_reason = "out_of_range",
            "OTLP port env var rejected",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["env_var_name"], "ANDROMEDA_PULSE_OTLP_GRPC_PORT");
    assert_eq!(fields["reject_reason"], "out_of_range");
}

#[test]
fn scrubber_redacts_non_allowlisted_port_validation_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "config.load.port_validation",
            env_var_name = "ANDROMEDA_PULSE_OTLP_GRPC_PORT",
            raw_value = "99999",
            "OTLP port env var rejected",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["env_var_name"], "ANDROMEDA_PULSE_OTLP_GRPC_PORT");
    assert_eq!(
        fields["raw_value"], "<redacted>",
        "raw env-var value MUST NOT leak through scrubber per Vector 6"
    );
}

#[test]
fn scrubber_passes_grpc_rate_limit_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "ingest.grpc.rate_limit.rejected",
            quota_window_seconds = 5_u64,
            reject_reason = "rate_limit_exceeded",
            "OTLP gRPC rate limit hit",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["quota_window_seconds"], 5);
    assert_eq!(fields["reject_reason"], "rate_limit_exceeded");
}

#[test]
fn scrubber_passes_http_rate_limit_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "ingest.http.rate_limit.rejected",
            quota_window_seconds = 3_u64,
            reject_reason = "rate_limit_exceeded",
            "OTLP HTTP rate limit hit",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["quota_window_seconds"], 3);
    assert_eq!(fields["reject_reason"], "rate_limit_exceeded");
}

#[test]
fn scrubber_redacts_non_allowlisted_rate_limit_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "ingest.grpc.rate_limit.rejected",
            quota_window_seconds = 5_u64,
            client_ip = "127.0.0.1",
            "OTLP gRPC rate limit hit",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["quota_window_seconds"], 5);
    assert_eq!(
        fields["client_ip"], "<redacted>",
        "client_ip MUST NOT leak per obs-plan §11 cardinality discipline"
    );
}

#[test]
fn scrubber_passes_buffer_schema_init_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "buffer.schema.init",
            table_count = 7_u64,
            duration_ms = 12_u64,
            "ring buffer schema created",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["table_count"], 7);
    assert_eq!(fields["duration_ms"], 12);
}

#[test]
fn scrubber_redacts_non_allowlisted_buffer_schema_init_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "buffer.schema.init",
            table_count = 7_u64,
            raw_ddl = "CREATE TABLE secrets (...)",
            "ring buffer schema created",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["table_count"], 7);
    assert_eq!(
        fields["raw_ddl"], "<redacted>",
        "raw DDL MUST NOT leak per security plan §Anti-Patterns Logging"
    );
}

#[test]
fn scrubber_passes_buffer_schema_init_error_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "buffer.schema.init.error",
            error_type = "schema_create",
            spantrace = "captured",
            "DuckDB schema creation failed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["error_type"], "schema_create");
    assert_eq!(fields["spantrace"], "captured");
}

#[test]
fn scrubber_redacts_non_allowlisted_buffer_schema_init_error_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "buffer.schema.init.error",
            error_type = "schema_create",
            raw_duckdb_error_text = "internal error 0xdeadbeef",
            "DuckDB schema creation failed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["error_type"], "schema_create");
    assert_eq!(
        fields["raw_duckdb_error_text"], "<redacted>",
        "raw DuckDB error text MUST NOT leak"
    );
}

#[test]
fn scrubber_passes_duckdb_append_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "duckdb.append",
            rows_appended = 100_u64,
            duration_ms = 4_u64,
            table_name = "spans",
            "Arrow appender wrote rows",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["rows_appended"], 100);
    assert_eq!(fields["duration_ms"], 4);
    assert_eq!(fields["table_name"], "spans");
}

#[test]
fn scrubber_redacts_non_allowlisted_duckdb_append_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "duckdb.append",
            rows_appended = 100_u64,
            raw_otlp_payload = "secret_data_attribute_value",
            attribute_value = "leak",
            "Arrow appender wrote rows",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["rows_appended"], 100);
    assert_eq!(
        fields["raw_otlp_payload"], "<redacted>",
        "raw OTLP payload MUST NOT leak per Vector 1"
    );
    assert_eq!(
        fields["attribute_value"], "<redacted>",
        "OTLP attribute value MUST NOT leak per Vector 1"
    );
}

// chunk #21 retention sweep + metric.buffer.* + config.load.retention_seconds
// tests. Five pass / five redact pairs mirroring the chunk #20 precedent above.

#[test]
fn scrubber_passes_buffer_retention_sweep_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "buffer.retention.sweep",
            query_id = "buffer.retention.sweep",
            param_count = 1_u64,
            param_types = "timestamp",
            rows_evicted = 42_u64,
            duration_ms = 12_u64,
            cutoff_ts_unix_nano = 1_700_000_000_000_000_000_i64,
            "retention sweep completed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["query_id"], "buffer.retention.sweep");
    assert_eq!(fields["param_count"], 1);
    assert_eq!(fields["param_types"], "timestamp");
    assert_eq!(fields["rows_evicted"], 42);
    assert_eq!(fields["duration_ms"], 12);
    assert_eq!(fields["cutoff_ts_unix_nano"], 1_700_000_000_000_000_000_i64);
}

#[test]
fn scrubber_redacts_non_allowlisted_buffer_retention_sweep_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "buffer.retention.sweep",
            query_id = "buffer.retention.sweep",
            raw_sql = "DELETE FROM spans WHERE ts < ...",
            "retention sweep completed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["query_id"], "buffer.retention.sweep");
    assert_eq!(
        fields["raw_sql"], "<redacted>",
        "raw SQL MUST NOT leak per security plan §Anti-Patterns Logging Vector 5"
    );
}

#[test]
fn scrubber_passes_buffer_retention_sweep_error_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "buffer.retention.sweep.error",
            error_type = "connection_lost",
            duration_ms = 5_u64,
            spantrace = "captured",
            "retention sweep failed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["error_type"], "connection_lost");
    assert_eq!(fields["duration_ms"], 5);
    assert_eq!(fields["spantrace"], "captured");
}

#[test]
fn scrubber_redacts_non_allowlisted_buffer_retention_sweep_error_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "buffer.retention.sweep.error",
            error_type = "execute_failed",
            raw_duckdb_text = "internal error 0xdeadbeef at /tmp/secret",
            "retention sweep failed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["error_type"], "execute_failed");
    assert_eq!(
        fields["raw_duckdb_text"], "<redacted>",
        "raw DuckDB error text MUST NOT leak"
    );
}

#[test]
fn scrubber_passes_metric_buffer_memory_bytes_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.buffer.memory_bytes",
            value = 4096_u64,
            retention_window_seconds = 600_u64,
            rows_active = 16_u64,
            eviction_count_since_last_tick = 0_u64,
            "buffer memory gauge",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 4096);
    assert_eq!(fields["retention_window_seconds"], 600);
    assert_eq!(fields["rows_active"], 16);
    assert_eq!(fields["eviction_count_since_last_tick"], 0);
}

#[test]
fn scrubber_redacts_non_allowlisted_metric_buffer_memory_bytes_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.buffer.memory_bytes",
            value = 4096_u64,
            client_ip = "127.0.0.1",
            trace_id = "abcdef",
            "buffer memory gauge",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 4096);
    assert_eq!(
        fields["client_ip"], "<redacted>",
        "high-cardinality field MUST NOT leak per obs-plan §11"
    );
    assert_eq!(
        fields["trace_id"], "<redacted>",
        "trace_id MUST NOT leak per obs-plan §11 cardinality discipline"
    );
}

#[test]
fn scrubber_passes_metric_buffer_evicted_span_count_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.buffer.evicted_span_count",
            value = 12_u64,
            retention_window_seconds = 600_u64,
            "buffer eviction counter",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 12);
    assert_eq!(fields["retention_window_seconds"], 600);
}

#[test]
fn scrubber_redacts_non_allowlisted_metric_buffer_evicted_span_count_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.buffer.evicted_span_count",
            value = 12_u64,
            evicted_trace_ids = "leak,leak,leak",
            "buffer eviction counter",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 12);
    assert_eq!(
        fields["evicted_trace_ids"], "<redacted>",
        "evicted-row identifiers MUST NOT leak per Vector 1"
    );
}

#[test]
fn scrubber_passes_config_load_retention_seconds_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "config.load.retention_seconds",
            env_var_name = "ANDROMEDA_PULSE_RETENTION_SECONDS",
            reject_reason = "out_of_range",
            "retention seconds env var rejected",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["env_var_name"], "ANDROMEDA_PULSE_RETENTION_SECONDS");
    assert_eq!(fields["reject_reason"], "out_of_range");
}

#[test]
fn scrubber_redacts_non_allowlisted_config_load_retention_seconds_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "config.load.retention_seconds",
            env_var_name = "ANDROMEDA_PULSE_RETENTION_SECONDS",
            raw_value = "999999",
            "retention seconds env var rejected",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["env_var_name"], "ANDROMEDA_PULSE_RETENTION_SECONDS");
    assert_eq!(
        fields["raw_value"], "<redacted>",
        "raw env-var value MUST NOT leak per Vector 6"
    );
}

// chunk #22 viz.query.* + metric.trace.* + extended viz allowlist
// tests. Five pass / five redact pairs mirroring chunk #20/#21 precedent.

#[test]
fn scrubber_passes_viz_query_traces_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "viz.query.traces",
            query_id = "viz.query.traces",
            param_count = 3_u64,
            param_types = "i64,i64,i64",
            time_window_seconds = 60_u64,
            row_count = 42_u64,
            latency_ms = 18_u64,
            "viz traces query completed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["query_id"], "viz.query.traces");
    assert_eq!(fields["param_count"], 3);
    assert_eq!(fields["param_types"], "i64,i64,i64");
    assert_eq!(fields["time_window_seconds"], 60);
    assert_eq!(fields["row_count"], 42);
    assert_eq!(fields["latency_ms"], 18);
}

#[test]
fn scrubber_redacts_non_allowlisted_viz_query_traces_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "viz.query.traces",
            query_id = "viz.query.traces",
            service_filter_value = "user-controlled-payload",
            raw_sql = "SELECT * FROM spans WHERE service = 'leak'",
            "viz traces query completed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["query_id"], "viz.query.traces");
    assert_eq!(
        fields["service_filter_value"], "<redacted>",
        "user-controlled parameter value MUST NOT leak per Vector 5"
    );
    assert_eq!(
        fields["raw_sql"], "<redacted>",
        "raw SQL MUST NOT leak per Vector 5"
    );
}

#[test]
fn scrubber_passes_viz_query_error_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "viz.query.error",
            error_type = "query_failed",
            duration_ms = 12_u64,
            spantrace = "captured",
            "viz query failed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["error_type"], "query_failed");
    assert_eq!(fields["duration_ms"], 12);
    assert_eq!(fields["spantrace"], "captured");
}

#[test]
fn scrubber_redacts_non_allowlisted_viz_query_error_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "viz.query.error",
            error_type = "query_failed",
            raw_duckdb_text = "internal error 0xdeadbeef at /tmp/secret",
            "viz query failed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["error_type"], "query_failed");
    assert_eq!(
        fields["raw_duckdb_text"], "<redacted>",
        "raw DuckDB error text MUST NOT leak"
    );
}

#[test]
fn scrubber_passes_metric_trace_latency_percentiles_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.trace.latency_percentiles",
            value = 18_u64,
            service_name = "test-service",
            p50_ms = 10_u64,
            p95_ms = 50_u64,
            p99_ms = 100_u64,
            max_ms = 200_u64,
            "trace latency",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 18);
    assert_eq!(fields["service_name"], "test-service");
    assert_eq!(fields["p50_ms"], 10);
    assert_eq!(fields["p99_ms"], 100);
}

#[test]
fn scrubber_redacts_non_allowlisted_metric_trace_latency_percentiles_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.trace.latency_percentiles",
            value = 18_u64,
            trace_id = "secret-trace-id",
            client_ip = "127.0.0.1",
            "trace latency",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 18);
    assert_eq!(
        fields["trace_id"], "<redacted>",
        "trace_id MUST NOT leak per cardinality discipline"
    );
    assert_eq!(fields["client_ip"], "<redacted>", "client_ip MUST NOT leak");
}

#[test]
fn scrubber_passes_metric_trace_error_count_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.trace.error_count",
            value = 5_u64,
            service_name = "test-service",
            error_type = "query_failed",
            "trace error counter",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 5);
    assert_eq!(fields["service_name"], "test-service");
    assert_eq!(fields["error_type"], "query_failed");
}

#[test]
fn scrubber_redacts_non_allowlisted_metric_trace_error_count_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.trace.error_count",
            value = 5_u64,
            error_message_text = "duckdb internal 0xdeadbeef",
            "trace error counter",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 5);
    assert_eq!(
        fields["error_message_text"], "<redacted>",
        "raw error message MUST NOT leak"
    );
}

#[test]
fn scrubber_passes_extended_viz_allowlist_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "viz.query.metrics",
            query_id = "viz.query.metrics",
            param_count = 3_u64,
            param_types = "i64,i64,i64",
            time_window_seconds = 60_u64,
            traceparent = "00-0123456789abcdef0123456789abcdef-0123456789abcdef-01",
            filter_count = 1_u64,
            "viz metrics query completed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["param_types"], "i64,i64,i64");
    assert_eq!(fields["time_window_seconds"], 60);
    assert!(fields["traceparent"].is_string());
    assert_eq!(fields["filter_count"], 1);
}

#[test]
fn scrubber_redacts_query_parameter_value_on_viz_target() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "viz.query.logs",
            query_id = "viz.query.logs",
            cursor_value = "' OR 1=1 --",
            attribute_value = "secret-attribute",
            "viz logs query completed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["query_id"], "viz.query.logs");
    assert_eq!(
        fields["cursor_value"], "<redacted>",
        "cursor_value MUST NOT leak per Vector 5"
    );
    assert_eq!(
        fields["attribute_value"], "<redacted>",
        "OTLP attribute_value MUST NOT leak per Vector 1"
    );
}

// chunk #23 tauri.channel.* + metric.ingest.channel.broadcast_subscribers
// tests. Five pass / five redact pairs mirroring chunks #20/#21/#22 precedent.

#[test]
fn scrubber_passes_tauri_channel_emit_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "tauri.channel.emit",
            channel_name = "pulse://stream/spans",
            arrow_schema = "trace_id,span_id,ts,ts_unix_nano",
            row_count = 5_u64,
            payload_size_bytes = 1024_u64,
            schema_match = true,
            traceparent = "00-0123456789abcdef0123456789abcdef-0123456789abcdef-01",
            duration_ms = 2_u64,
            "Arrow IPC payload emitted",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["channel_name"], "pulse://stream/spans");
    assert_eq!(fields["row_count"], 5);
    assert_eq!(fields["payload_size_bytes"], 1024);
    assert_eq!(fields["schema_match"], true);
    assert_eq!(fields["duration_ms"], 2);
}

#[test]
fn scrubber_redacts_non_allowlisted_tauri_channel_emit_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "tauri.channel.emit",
            channel_name = "pulse://stream/spans",
            row_count = 5_u64,
            attribute_value = "secret-attr-leak",
            raw_payload_first_bytes = "secret-row-content",
            "Arrow IPC payload emitted",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["channel_name"], "pulse://stream/spans");
    assert_eq!(fields["row_count"], 5);
    assert_eq!(
        fields["attribute_value"], "<redacted>",
        "OTLP attribute value MUST NOT leak per Vector 1"
    );
    assert_eq!(
        fields["raw_payload_first_bytes"], "<redacted>",
        "raw payload bytes MUST NOT leak per Vector 1"
    );
}

#[test]
fn scrubber_passes_tauri_channel_emit_size_exceeded_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "tauri.channel.emit.size_exceeded",
            channel_name = "pulse://stream/spans",
            payload_size_bytes = 9_999_999_u64,
            limit_bytes = 8_388_608_u64,
            error_type = "size_exceeded",
            "Arrow IPC payload exceeded broadcast size cap",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["channel_name"], "pulse://stream/spans");
    assert_eq!(fields["payload_size_bytes"], 9_999_999);
    assert_eq!(fields["limit_bytes"], 8_388_608);
    assert_eq!(fields["error_type"], "size_exceeded");
}

#[test]
fn scrubber_redacts_non_allowlisted_tauri_channel_emit_size_exceeded_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "tauri.channel.emit.size_exceeded",
            channel_name = "pulse://stream/spans",
            payload_size_bytes = 9_999_999_u64,
            offending_row_text = "secret-row-content",
            "Arrow IPC payload exceeded broadcast size cap",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["payload_size_bytes"], 9_999_999);
    assert_eq!(
        fields["offending_row_text"], "<redacted>",
        "row content MUST NOT leak"
    );
}

#[test]
fn scrubber_passes_tauri_channel_emit_error_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "tauri.channel.emit.error",
            channel_name = "pulse://stream/metrics",
            error_type = "encode_failed",
            duration_ms = 3_u64,
            spantrace = "captured",
            "Arrow IPC encode failed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["channel_name"], "pulse://stream/metrics");
    assert_eq!(fields["error_type"], "encode_failed");
    assert_eq!(fields["duration_ms"], 3);
    assert_eq!(fields["spantrace"], "captured");
}

#[test]
fn scrubber_redacts_non_allowlisted_tauri_channel_emit_error_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::error!(
            target: "tauri.channel.emit.error",
            channel_name = "pulse://stream/metrics",
            error_type = "encode_failed",
            arrow_internal_text = "ArrowError::IpcError at /private 0xdeadbeef",
            "Arrow IPC encode failed",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["error_type"], "encode_failed");
    assert_eq!(
        fields["arrow_internal_text"], "<redacted>",
        "raw library error text MUST NOT leak"
    );
}

#[test]
fn scrubber_passes_tauri_channel_lag_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "tauri.channel.lag",
            channel_name = "pulse://stream/logs",
            dropped_count = 42_u64,
            "broadcast receiver lagged",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["channel_name"], "pulse://stream/logs");
    assert_eq!(fields["dropped_count"], 42);
}

#[test]
fn scrubber_redacts_non_allowlisted_tauri_channel_lag_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "tauri.channel.lag",
            channel_name = "pulse://stream/logs",
            dropped_count = 42_u64,
            lost_trace_ids = "id1,id2,id3",
            "broadcast receiver lagged",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["dropped_count"], 42);
    assert_eq!(
        fields["lost_trace_ids"], "<redacted>",
        "lost trace identifiers MUST NOT leak per Vector 1"
    );
}

#[test]
fn scrubber_passes_metric_ingest_channel_broadcast_subscribers_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.ingest.channel.broadcast_subscribers",
            value = 3_u64,
            channel_name = "pulse://stream/spans",
            "broadcast subscriber gauge",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 3);
    assert_eq!(fields["channel_name"], "pulse://stream/spans");
}

#[test]
fn scrubber_redacts_non_allowlisted_metric_ingest_channel_broadcast_subscribers_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.ingest.channel.broadcast_subscribers",
            value = 3_u64,
            channel_name = "pulse://stream/spans",
            client_ip = "127.0.0.1",
            subscriber_session_id = "abc-secret",
            "broadcast subscriber gauge",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["value"], 3);
    assert_eq!(
        fields["client_ip"], "<redacted>",
        "client_ip MUST NOT leak per cardinality discipline"
    );
    assert_eq!(
        fields["subscriber_session_id"], "<redacted>",
        "session id MUST NOT leak per cardinality discipline"
    );
}

// chunk #24 webview shell — boot detection spans + close→tray transition
// + tray-boundary surfaces. Five pass / five redact pairs mirroring
// chunks #20/#21/#22/#23 precedent.

#[test]
fn scrubber_passes_app_boot_webview_init_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "app.boot.webview.init",
            webview_backend = "WebView2",
            "webview backend detected at boot",
        );
    });
    assert_eq!(lines[0]["fields"]["webview_backend"], "WebView2");
}

#[test]
fn scrubber_redacts_non_allowlisted_app_boot_webview_init_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "app.boot.webview.init",
            webview_backend = "WebView2",
            user_agent_string = "Mozilla/5.0 (raw secret context)",
            "webview backend detected at boot",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["webview_backend"], "WebView2");
    assert_eq!(
        fields["user_agent_string"], "<redacted>",
        "raw user-agent string MUST NOT leak per Vector 1 cardinality discipline"
    );
}

#[test]
fn scrubber_passes_app_boot_gpu_check_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "app.boot.gpu.check",
            wgpu_backend = "metal",
            "compile-target default wgpu backend",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["wgpu_backend"], "metal");
}

#[test]
fn scrubber_redacts_non_allowlisted_app_boot_gpu_check_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "app.boot.gpu.check",
            gpu_available = false,
            wgpu_backend = "vulkan",
            adapter_vendor_id = "0xDEADBEEF",
            "compile-target default wgpu backend",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["wgpu_backend"], "vulkan");
    assert_eq!(
        fields["gpu_available"], "<redacted>",
        "the record measures no adapter, so an adapter-availability claim MUST NOT pass"
    );
    assert_eq!(
        fields["adapter_vendor_id"], "<redacted>",
        "raw adapter identifiers MUST NOT leak (cardinality discipline)"
    );
}

#[test]
fn scrubber_passes_metric_webgpu_frame_duration_ms_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.webgpu.frame_duration_ms",
            duration_ms = 16.7,
            wgpu_backend = "vulkan",
            webview_backend = "webview2",
            timing_method = "cpu",
            "frame duration recorded",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["duration_ms"], 16.7);
    assert_eq!(fields["wgpu_backend"], "vulkan");
    assert_eq!(fields["webview_backend"], "webview2");
    assert_eq!(fields["timing_method"], "cpu");
}

#[test]
fn scrubber_redacts_non_allowlisted_metric_webgpu_frame_duration_ms_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "metric.webgpu.frame_duration_ms",
            duration_ms = 33.0,
            wgpu_backend = "metal",
            webview_backend = "wkwebview",
            timing_method = "gpu",
            trace_id = "0xDEADBEEF000000000000000000000000",
            "frame duration recorded",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["duration_ms"], 33.0);
    assert_eq!(fields["wgpu_backend"], "metal");
    assert_eq!(
        fields["trace_id"], "<redacted>",
        "raw trace_id MUST NOT leak per Vector 6 cardinality discipline"
    );
}

#[test]
fn scrubber_passes_app_boot_tray_init_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "app.boot.tray.init",
            tray_api = "AppIndicator",
            "tray API selected at boot",
        );
    });
    assert_eq!(lines[0]["fields"]["tray_api"], "AppIndicator");
}

#[test]
fn scrubber_passes_ui_layout_transition_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "ui.layout.transition",
            layout_mode_from = "main",
            layout_mode_to = "hidden",
            tray_visible = true,
            "window close→minimize-to-tray",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["layout_mode_from"], "main");
    assert_eq!(fields["layout_mode_to"], "hidden");
    assert_eq!(fields["tray_visible"], true);
}

#[test]
fn scrubber_redacts_non_allowlisted_ui_layout_transition_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "ui.layout.transition",
            layout_mode_from = "compact-widget",
            layout_mode_to = "hidden",
            tray_visible = true,
            window_title = "instrumented-app: secret-customer-name",
            "window close→minimize-to-tray",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["layout_mode_from"], "compact-widget");
    assert_eq!(
        fields["window_title"], "<redacted>",
        "user-app window title MUST NOT leak per Vector 1 (titles can carry instrumented-app names)"
    );
}

#[test]
fn scrubber_passes_app_boot_window_show_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "app.boot.window.show",
            label = "compact-widget",
            error_kind = "show_failed",
            error_msg = "device not ready",
            "failed to show compact-widget window",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["label"], "compact-widget");
    assert_eq!(fields["error_kind"], "show_failed");
    assert_eq!(fields["error_msg"], "device not ready");
}

#[test]
fn scrubber_redacts_non_allowlisted_app_boot_window_show_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::warn!(
            target: "app.boot.window.show",
            label = "compact-widget",
            error_kind = "not_found",
            user_home_path = "/home/alice/private",
            "compact-widget window not registered",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["label"], "compact-widget");
    assert_eq!(
        fields["user_home_path"], "<redacted>",
        "absolute user paths MUST NOT leak per security plan §Logging Vector 2"
    );
}

#[test]
fn scrubber_passes_tray_visibility_toggle_fields() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "tray.visibility.toggle",
            tray_visible = true,
            "tray icon shown",
        );
    });
    assert_eq!(lines[0]["fields"]["tray_visible"], true);
}

#[test]
fn scrubber_redacts_non_allowlisted_tray_visibility_toggle_field() {
    let defaults = make_defaults(None, None);
    let lines = capture_json_lines(defaults, || {
        tracing::info!(
            target: "tray.visibility.toggle",
            tray_visible = true,
            clipboard_snippet = "secret-text-from-user",
            "tray icon shown",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["tray_visible"], true);
    assert_eq!(
        fields["clipboard_snippet"], "<redacted>",
        "clipboard contents MUST NOT leak per security plan §Logging Vector 4"
    );
}

#[test]
fn allowlist_resolver_picks_chunk24_targets() {
    let al = AllowList::production();
    assert!(al.for_target("app.boot.webview.init").is_some());
    assert!(al.for_target("app.boot.gpu.check").is_some());
    assert!(al.for_target("app.boot.tray.init").is_some());
    assert!(al.for_target("app.boot.window.show").is_some());
    assert!(al.for_target("ui.layout.transition").is_some());
    assert!(al.for_target("tray.visibility.toggle").is_some());
    assert!(al.for_target("tray.menu.interaction").is_some());
    assert!(al.for_target("tray.notification.dismiss").is_some());
    assert!(al.for_target("tray.notification.action").is_some());
    assert!(al.for_target("tray.signpost.shown").is_some());
}

#[test]
fn deployment_environment_returns_known_value() {
    let env_value = resolve_deployment_environment();
    assert!(matches!(
        env_value.as_str(),
        "production" | "staging" | "dev"
    ));
}

#[test]
fn subscriber_init_idempotent_under_test_repetition() {
    // Each invocation builds an isolated subscriber via with_default scope guard,
    // never calling tracing::subscriber::set_global_default. Repeated calls must
    // not panic with "subscriber already set".
    for _ in 0..3 {
        let lines = capture_json_lines(make_defaults(None, None), || {
            tracing::info!(target: "ingest", span_count = 0_i64, "ping");
        });
        assert_eq!(lines.len(), 1);
    }
}

#[test]
fn allowlist_for_target_resolves_l4_inference_pipeline_targets() {
    // chunk #83: L4 LLM interpretation pipeline emits five tracing
    // targets plus three metric targets. Required fields per the obs
    // extract; PII guard per CLAUDE.md observability.md 2026-05-17
    // session 84 aggregate-only mandate (no per-service identifiers;
    // no raw prompt body / digest payload / output bytes).
    let al = AllowList::production();

    let request = al
        .for_target("interpretation.inference.request")
        .expect("expected interpretation.inference.request entry");
    for required in [
        "model_tier",
        "hardware_profile",
        "prompt_version",
        "schema_version",
        "duration_ms",
        "result",
        "token_count_prompt",
        "token_count_output",
        "digest_kind",
    ] {
        assert!(
            request.contains(required),
            "interpretation.inference.request must permit `{required}`",
        );
    }

    let assemble = al
        .for_target("interpretation.prompt.assemble")
        .expect("expected interpretation.prompt.assemble entry");
    for required in ["token_count", "prompt_version", "duration_ms"] {
        assert!(
            assemble.contains(required),
            "interpretation.prompt.assemble must permit `{required}`",
        );
    }

    let generate = al
        .for_target("interpretation.constrained.generate")
        .expect("expected interpretation.constrained.generate entry");
    for required in ["model_tier", "hardware_profile", "duration_ms", "success"] {
        assert!(
            generate.contains(required),
            "interpretation.constrained.generate must permit `{required}`",
        );
    }

    let parse = al
        .for_target("interpretation.json.parse")
        .expect("expected interpretation.json.parse entry");
    for required in ["parse_outcome", "duration_ms", "output_bytes"] {
        assert!(
            parse.contains(required),
            "interpretation.json.parse must permit `{required}`",
        );
    }

    let error = al
        .for_target("interpretation.inference.error")
        .expect("expected interpretation.inference.error entry");
    for required in [
        "model_tier",
        "hardware_profile",
        "error_category",
        "recovery_action",
    ] {
        assert!(
            error.contains(required),
            "interpretation.inference.error must permit `{required}`",
        );
    }

    let inferences_total = al
        .for_target("metric.pipeline.l4.inferences_total")
        .expect("expected metric.pipeline.l4.inferences_total entry");
    for required in ["value", "result", "model_tier", "hardware_profile"] {
        assert!(
            inferences_total.contains(required),
            "metric.pipeline.l4.inferences_total must permit `{required}`",
        );
    }

    let latency = al
        .for_target("metric.pipeline.l4.inference_latency_p99_milliseconds")
        .expect("expected metric.pipeline.l4.inference_latency_p99_milliseconds entry");
    for required in ["value", "duration_ms", "model_tier", "hardware_profile"] {
        assert!(
            latency.contains(required),
            "metric.pipeline.l4.inference_latency_p99_milliseconds must permit `{required}`",
        );
    }

    let queue_depth = al
        .for_target("metric.pipeline.l4.inference_queue_depth")
        .expect("expected metric.pipeline.l4.inference_queue_depth entry");
    for required in ["value", "queued_count"] {
        assert!(
            queue_depth.contains(required),
            "metric.pipeline.l4.inference_queue_depth must permit `{required}`",
        );
    }

    // PII guard: none of the chunk #83 entries may admit raw
    // OTLP-derived identifiers, prompt content, output bytes, or
    // free-text payload fields. Each target re-validated.
    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
        "prompt_body",
        "prompt_text",
        "digest_payload",
        "digest_body",
        "raw_output",
        "output_text",
        "tokenized_prompt",
        "completion_bytes",
        "model_path",
        "checkpoint_url",
        "file_path",
        "fingerprint",
        "digest_id",
        "incident_id",
    ];
    for set in [
        request,
        assemble,
        generate,
        parse,
        error,
        inferences_total,
        latency,
        queue_depth,
    ] {
        for k in &banned {
            assert!(
                !set.contains(k),
                "L4 inference AllowList must NOT permit PII field `{k}`",
            );
        }
    }
}

// Chunk #86 — L4 degraded-mode + manual retry AllowList entries.
// Aggregate-only fields per chunk #86 obs constraint mirroring chunk
// #83 test pattern above. Seven new entries (4 tracing targets + 3
// metric targets) registered in AllowList::production(); the
// diagnostics.retry_interpretation.request entry is also new.
#[test]
fn allowlist_for_target_resolves_chunk_86_degraded_mode_and_retry_targets() {
    let al = AllowList::production();

    let degraded_enter = al
        .for_target("interpretation.degraded.enter")
        .expect("expected interpretation.degraded.enter entry");
    for required in ["consecutive_failures", "window_seconds", "backoff_seconds"] {
        assert!(
            degraded_enter.contains(required),
            "interpretation.degraded.enter must permit `{required}`",
        );
    }

    let degraded_exit = al
        .for_target("interpretation.degraded.exit")
        .expect("expected interpretation.degraded.exit entry");
    for required in ["consecutive_successes", "duration_seconds"] {
        assert!(
            degraded_exit.contains(required),
            "interpretation.degraded.exit must permit `{required}`",
        );
    }

    let skipped = al
        .for_target("interpretation.inference.skipped")
        .expect("expected interpretation.inference.skipped entry");
    for required in ["reason", "model_tier", "backoff_seconds_remaining"] {
        assert!(
            skipped.contains(required),
            "interpretation.inference.skipped must permit `{required}`",
        );
    }

    let persist_err = al
        .for_target("interpretation.resolution_summary.persist.error")
        .expect("expected interpretation.resolution_summary.persist.error entry");
    assert!(
        persist_err.contains("error_category"),
        "interpretation.resolution_summary.persist.error must permit `error_category`",
    );

    let degraded_active_total = al
        .for_target("metric.pipeline.l4.degraded_mode_active_seconds_total")
        .expect("expected metric.pipeline.l4.degraded_mode_active_seconds_total entry");
    assert!(degraded_active_total.contains("value"));

    let degraded_entries_total = al
        .for_target("metric.pipeline.l4.degraded_mode_entries_total")
        .expect("expected metric.pipeline.l4.degraded_mode_entries_total entry");
    assert!(degraded_entries_total.contains("value"));

    let backoff_remaining = al
        .for_target("metric.pipeline.l4.backoff_remaining_seconds")
        .expect("expected metric.pipeline.l4.backoff_remaining_seconds entry");
    assert!(backoff_remaining.contains("value"));

    let retry_req = al
        .for_target("diagnostics.retry_interpretation.request")
        .expect("expected diagnostics.retry_interpretation.request entry");
    for required in [
        "triggered",
        "current_state",
        "consecutive_failures",
        "backoff_remaining_seconds",
        "duration_ms",
    ] {
        assert!(
            retry_req.contains(required),
            "diagnostics.retry_interpretation.request must permit `{required}`",
        );
    }

    // PII guard: chunk #86 entries MUST NOT admit per-incident /
    // per-service / per-trace identifiers OR LLM-emitted content
    // (raw output / parse error message / digest body). The FSM is
    // GLOBAL per Phase 6 user-confirmed scope; per-incident
    // explosion defeats the cardinality budget per CLAUDE.md
    // observability 2026-05-17 session 84 AGGREGATE-ONLY mandate.
    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "incident_id",
        "operation_name",
        "model_output",
        "raw_output",
        "parse_error_message",
        "digest_body",
        "digest_payload",
        "prompt_body",
    ];
    for set in [
        degraded_enter,
        degraded_exit,
        skipped,
        persist_err,
        degraded_active_total,
        degraded_entries_total,
        backoff_remaining,
        retry_req,
    ] {
        for k in &banned {
            assert!(
                !set.contains(k),
                "chunk #86 AllowList entry must NOT permit PII field `{k}`",
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_chunk_97_diagnostics_snapshot_and_history_targets() {
    let al = AllowList::production();

    let snapshot_req = al
        .for_target("diagnostics.snapshot.request")
        .expect("expected diagnostics.snapshot.request entry");
    for required in [
        "tier",
        "profile",
        "load_status",
        "backoff_remaining_seconds",
        "consecutive_failures",
        "drain_template_count",
        "duration_ms",
    ] {
        assert!(
            snapshot_req.contains(required),
            "diagnostics.snapshot.request must permit `{required}`",
        );
    }

    let history_req = al
        .for_target("diagnostics.history.request")
        .expect("expected diagnostics.history.request entry");
    for required in ["metric_name", "recorded", "point_count", "duration_ms"] {
        assert!(
            history_req.contains(required),
            "diagnostics.history.request must permit `{required}`",
        );
    }

    // PII / aggregate-only guard: chunk #97 entries MUST NOT admit
    // per-service / per-trace identifiers, the raw `window_seconds`
    // value (query-anonymizer discipline), OR payload content (per
    // CLAUDE.md observability 2026-05-17 session 84 aggregate-only
    // mandate).
    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
        "window_seconds",
        "model_identity_name",
        "points",
        "value_basis_points",
    ];
    for set in [snapshot_req, history_req] {
        for k in &banned {
            assert!(
                !set.contains(k),
                "chunk #97 AllowList entry must NOT permit PII/raw field `{k}`",
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_chunk_92_incident_producer_targets() {
    let al = AllowList::production();

    let created = al
        .for_target("interpretation.incident.created")
        .expect("expected interpretation.incident.created entry");
    for required in ["created", "deduped", "severity", "priority_tier"] {
        assert!(
            created.contains(required),
            "interpretation.incident.created must permit `{required}`",
        );
    }

    let persist_err = al
        .for_target("interpretation.incident.persist.error")
        .expect("expected interpretation.incident.persist.error entry");
    assert!(
        persist_err.contains("error_category"),
        "interpretation.incident.persist.error must permit `error_category`",
    );

    let created_total = al
        .for_target("metric.pipeline.l4.incidents_created_total")
        .expect("expected metric.pipeline.l4.incidents_created_total entry");
    for required in ["value", "result"] {
        assert!(
            created_total.contains(required),
            "metric.pipeline.l4.incidents_created_total must permit `{required}`",
        );
    }

    // PII guard: the chunk #92 producer targets MUST NOT admit
    // per-incident / per-service / per-trace identifiers OR LLM-emitted
    // content. The Incident payload carries scope_id for the product
    // surface (pulse://stream/incidents), but agent-latest.jsonl is
    // self-observation only — AGGREGATE-ONLY per CLAUDE.md observability
    // 2026-05-17 session 84.
    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "incident_id",
        "operation_name",
        "title",
        "detail",
        "fingerprint",
        "workspace",
    ];
    for set in [created, persist_err, created_total] {
        for k in &banned {
            assert!(
                !set.contains(k),
                "chunk #92 AllowList entry must NOT permit PII field `{k}`",
            );
        }
    }
}

#[test]
fn allowlist_for_target_resolves_chunk_87_mark_all_read_and_bulk_acknowledged_targets() {
    let al = AllowList::production();
    let mark_all_read = al
        .for_target("incidents.mark_all_read.request")
        .expect("expected incidents.mark_all_read.request entry");
    for required in ["affected_count", "outcome", "duration_ms", "traceparent"] {
        assert!(
            mark_all_read.contains(required),
            "incidents.mark_all_read.request must permit `{required}`",
        );
    }

    let bulk_ack = al
        .for_target("incident.broadcast.bulk_acknowledged")
        .expect("expected incident.broadcast.bulk_acknowledged entry");
    assert!(
        bulk_ack.contains("affected_count"),
        "incident.broadcast.bulk_acknowledged must permit `affected_count`",
    );

    // PII guard: chunk #87 bulk action entries MUST NOT admit
    // per-incident / per-service / per-trace identifiers OR raw
    // workspace strings OR LLM-emitted content. The bulk emit is
    // aggregate-only per CLAUDE.md observability 2026-05-17 session
    // 84 AGGREGATE-ONLY mandate. `workspace` deliberately banned
    // because the broadcast event scope is workspace-resolved at the
    // resolver, not at the event payload (mirrors chunk #78
    // `incident_lifecycle_event_has_no_pii_fields` discipline).
    let banned = [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "incident_id",
        "operation_name",
        "workspace",
        "workspace_hash",
        "title",
        "detail",
        "fingerprint",
        "raw_output",
        "incident_ids",
    ];
    for set in [mark_all_read, bulk_ack] {
        for k in &banned {
            assert!(
                !set.contains(k),
                "chunk #87 AllowList entry must NOT permit PII field `{k}`",
            );
        }
    }
}
