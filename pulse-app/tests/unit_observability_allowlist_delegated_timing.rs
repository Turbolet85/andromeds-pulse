//! Resolver probes for the three delegated timing observables (P-025 / P-027 / P-045).
//!
//! These live here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a guard.
//!
//! The emit sites (`crates/ui-bridge/src/telemetry.rs`, the three
//! `record_*` resolvers) are the authority for these field lists.
//!
//! Unlike the corpus and triage families, a bare `metric` key DOES exist and is
//! legitimate, so "no bare prefix key" is not the available guard here. The
//! discriminating property instead is that each target resolves to its OWN leaf
//! rather than falling back to that bare set — asserted below by requiring a
//! field the fallback set does not contain.

use pulse_app::observability::AllowList;

const HUE: &str = "metric.constellation.hue_update_ms";
const CONSTELLATION: &str = "metric.constellation.discovery_ms";
const FINDINGS: &str = "metric.findings.counter_refresh_ms";

#[test]
fn each_delegated_timing_target_allows_every_field_its_emit_site_emits() {
    let al = AllowList::production();

    for (target, required) in [
        (HUE, vec!["duration_ms", "severity_tier"]),
        (CONSTELLATION, vec!["duration_ms", "discovered_count"]),
        (FINDINGS, vec!["duration_ms"]),
    ] {
        let set = al
            .for_target(target)
            .unwrap_or_else(|| panic!("{target} must have an explicit leaf entry"));
        for field in required {
            assert!(
                set.contains(field),
                "`{field}` is emitted at {target} and must not be redacted",
            );
        }
    }
}

#[test]
fn delegated_timing_targets_resolve_to_their_own_leaf_not_the_bare_metric_fallback() {
    // `for_target` falls back to the first dotted segment, and a bare `metric`
    // key IS registered with only ["value", "unit", "module"]. Without its own
    // exact leaf a target would silently keep `value` and lose every label —
    // the mechanism behind the still-open `metric.pipeline.l1a.*` backlog. The
    // inequality below is what makes this test discriminate.
    let al = AllowList::production();
    let fallback = al
        .for_target("metric")
        .expect("the bare `metric` key is expected to exist");

    assert!(
        !fallback.contains("severity_tier")
            && !fallback.contains("discovered_count")
            && !fallback.contains("duration_ms"),
        "the bare `metric` fallback must NOT carry these fields, or this test cannot \
         distinguish an exact leaf from the fallback; fallback was: {fallback:?}",
    );

    for target in [HUE, CONSTELLATION, FINDINGS] {
        let set = al.for_target(target).expect("leaf entry");
        assert!(
            set.contains("duration_ms"),
            "{target} resolved to a set without `duration_ms` — it fell through to the \
             bare `metric` fallback instead of its own leaf",
        );
    }
}

#[test]
fn delegated_timing_targets_admit_no_per_service_identifier() {
    let al = AllowList::production();

    for target in [HUE, CONSTELLATION, FINDINGS] {
        let set = al.for_target(target).expect("leaf entry");
        for banned in [
            "service_name",
            "service",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
        ] {
            assert!(
                !set.contains(banned),
                "`{banned}` must stay out of {target} — `service.name` is an OTLP resource \
                 attribute (user content) and the measure is aggregate-only",
            );
        }
    }
}
