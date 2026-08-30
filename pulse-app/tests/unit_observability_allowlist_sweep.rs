// Allowlist guards for the 2026-08-30 diagnostics un-muting sweep — every
// leaf this chunk added or completed, asserted by field-set EQUALITY IN BOTH
// DIRECTIONS plus a fallback discriminator per obs-plan section 8 (a
// resolver-only probe passes vacuously when a bare prefix key can serve the
// target; a narrowed leaf passes the resolve probe while fields redact).
// Lives under `pulse-app/tests/` where it actually runs ([lib] test = false).

use std::collections::BTreeSet;

use pulse_app::observability::AllowList;

fn resolved(target: &str) -> BTreeSet<&'static str> {
    AllowList::production()
        .for_target(target)
        .unwrap_or_else(|| panic!("target {target} must resolve"))
        .iter()
        .copied()
        .collect()
}

fn expected(fields: &[&'static str]) -> BTreeSet<&'static str> {
    fields.iter().copied().collect()
}

// ── Family A: the four l1a leaves (bare-`metric` fallback keeps `value`
//    and silently redacts every label — the leaf must exist AND be exact).

#[test]
fn l1a_query_count_total_leaf_matches_emit_site_exactly() {
    assert_eq!(
        resolved("metric.pipeline.l1a.query_count_total"),
        expected(&["query_name", "value"]),
    );
}

#[test]
fn l1a_query_latency_leaf_matches_emit_site_exactly() {
    assert_eq!(
        resolved("metric.pipeline.l1a.query_latency_p99_milliseconds"),
        expected(&["query_name", "duration_ms", "row_count_returned"]),
    );
}

#[test]
fn l1a_q7_timeout_leaf_matches_emit_site_exactly() {
    assert_eq!(
        resolved("metric.pipeline.l1a.q7_timeout_count_total"),
        expected(&["value", "timeout_ms", "rejection_reason"]),
    );
}

#[test]
fn l1a_q7_fallback_leaf_matches_emit_site_exactly() {
    assert_eq!(
        resolved("metric.pipeline.l1a.q7_fallback_count_total"),
        expected(&["value", "fallback_query_kind", "cause"]),
    );
}

#[test]
fn l1a_labels_are_absent_from_the_bare_metric_fallback_set() {
    // The discriminator: deleting an l1a exact leaf leaves the target
    // resolving via the bare `metric` key — which must NOT carry the labels,
    // or this pair of tests could both pass with the leaf gone.
    let bare = resolved("metric");
    for label in [
        "query_name",
        "row_count_returned",
        "timeout_ms",
        "rejection_reason",
        "fallback_query_kind",
        "cause",
    ] {
        assert!(
            !bare.contains(label),
            "bare `metric` set must not carry `{label}`",
        );
    }
}

// ── Family A: the one target that resolved to NOTHING (the census's
//    accurate case) — its muting had a measured cost: a smoke wait keyed on
//    `resolved_count` rendered "<redacted>" and ran a 300s ceiling.

#[test]
fn auto_resolve_tick_leaf_matches_emit_site_exactly() {
    assert_eq!(
        resolved("triage.incident.auto_resolve.tick"),
        expected(&["evaluated_count", "resolved_count", "duration_ms"]),
    );
}

#[test]
fn auto_resolve_tick_has_no_fallback_to_hide_behind() {
    // Deleting the exact leaf must redact EVERYTHING: the `.tick`-strip
    // target and the bare `triage` prefix both resolve to nothing.
    let al = AllowList::production();
    assert!(al.for_target("triage.incident.auto_resolve").is_none());
    assert!(al.for_target("triage").is_none());
}

// ── Family A: interpretation.model.load — the leaf now EQUALS the emit-site
//    union (inference_mode joined; the never-emitted file_size_bytes left).

#[test]
fn model_load_leaf_matches_emit_site_union_exactly() {
    assert_eq!(
        resolved("interpretation.model.load"),
        expected(&["model_identity", "tier", "load_status", "inference_mode"]),
    );
}

#[test]
fn model_load_leaf_no_longer_registers_the_never_emitted_field() {
    assert!(
        !resolved("interpretation.model.load").contains("file_size_bytes"),
        "file_size_bytes had no emit site anywhere — leaf-equals-emit-set",
    );
}

// ── Family A: the bare `interpretation` prefix key is GONE (obs-plan §8
//    invariant, recorded VIOLATED 2026-08-26 and owned by this sweep).

#[test]
fn no_bare_interpretation_prefix_key_exists() {
    let al = AllowList::production();
    assert!(
        al.for_target("interpretation").is_none(),
        "a bare `interpretation` key would silently serve every future \
         unregistered interpretation.* target a stale field set",
    );
    // The widening path itself: an unregistered dotted sibling must resolve
    // to NOTHING, not to a populated fallback set.
    assert!(
        al.for_target("interpretation.__unregistered_probe")
            .is_none()
    );
}

// ── Family A: the two boot records adopt basename-only (security-plan
//    KNOWN CARRIED EXCEPTION discharged — the full-path fields are GONE).

#[test]
fn boot_tracing_init_leaf_is_basename_only() {
    let set = resolved("app.boot.tracing.init");
    assert_eq!(
        set,
        expected(&["log_dir_basename", "default_fields_active"]),
    );
    assert!(!set.contains("log_dir"), "full-path field must be gone");
}

#[test]
fn boot_pid_leaf_is_basename_only() {
    let set = resolved("app.boot.pid");
    assert_eq!(
        set,
        expected(&[
            "pid",
            "path_basename",
            "error",
            "run_dir_basename",
            "data_dir_basename",
        ]),
    );
    for full in ["path", "run_dir", "data_dir"] {
        assert!(!set.contains(full), "full-path field `{full}` must be gone");
    }
}

// ── C1: `append_rejections` rides buffer.tick (the `.tick`-strip resolves
//    to the bare `buffer` set, which carries the tick fields by design).

#[test]
fn buffer_tick_carries_append_rejections() {
    assert!(resolved("buffer.tick").contains("append_rejections"));
}

// ── C4: the samples-truth rename reached the wire names.

#[test]
fn cue_leaves_carry_persistence_not_the_seconds_lie() {
    for target in ["triage.cue.suppression_check", "triage.cue.emit"] {
        let set = resolved(target);
        assert!(
            set.contains("persistence"),
            "{target} carries `persistence`"
        );
        assert!(
            !set.contains("persistence_seconds"),
            "{target} must not carry the retired `persistence_seconds`",
        );
    }
}

// ── B4's mechanical guard: `pulse-app/src` may hold ZERO `#[test]` fns —
//    `[lib] test = false` makes a src-level `mod tests` compile and never
//    run, and the rule has been violated after three prose restatements.
//    This count assertion is the fourth restatement made mechanical.

#[test]
fn pulse_app_src_carries_no_new_dead_test_attributes() {
    // Ratchet, not a flat zero: this guard's FIRST run measured ~101 MORE
    // dead src-level tests across the 14 lib files below (beyond the 130 in
    // observability.rs the route entry named — migrated 2026-08-30). Their
    // migration is a surfaced follow-up, not this chunk; meanwhile the
    // ratchet reds on ANY new file gaining a `#[test]` and on ANY legacy
    // file GROWING. `main.rs` is the [[bin]] target — its tests genuinely
    // run under nextest — so it is exempt.
    const LEGACY_DEAD_BASELINE: &[(&str, usize)] = &[
        ("baseline_observer.rs", 2),
        ("connection_router.rs", 4),
        ("diagnostics_router.rs", 11),
        ("digest_runtime.rs", 2),
        ("heartbeat.rs", 25),
        ("mcp_router.rs", 6),
        ("plugins_router.rs", 7),
        ("restart_observer.rs", 4),
        ("snapshot_runtime.rs", 7),
        ("storage_router.rs", 7),
        ("storm_observer.rs", 2),
        ("streams.rs", 2),
        ("tray.rs", 10),
        ("window.rs", 13),
    ];
    let baseline: std::collections::BTreeMap<&str, usize> =
        LEGACY_DEAD_BASELINE.iter().copied().collect();

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut violations = Vec::new();
    for entry in std::fs::read_dir(&src_dir).expect("read pulse-app/src") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        if name == "main.rs" {
            continue;
        }
        let content = std::fs::read_to_string(&path).expect("read src file");
        let count = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                t == "#[test]" || t == "#[tokio::test]"
            })
            .count();
        let allowed = baseline.get(name.as_str()).copied().unwrap_or(0);
        if count > allowed {
            violations.push(format!("{name}: {count} (allowed {allowed})"));
        }
    }
    assert!(
        violations.is_empty(),
        "src-level tests never run under [lib] test = false — move new ones          to pulse-app/tests/ (2026-05-20 precedent) and shrink, never grow,          the legacy baseline: {violations:?}",
    );
}
