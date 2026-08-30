//! Resolver probes for the two leaves the L4 path guard emits on.
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! The emit sites (`pulse-app/src/llamacli_inference.rs::resolve_guarded_path`
//! and `::emit_allow_root_posture`) are the authority for these field lists.
//!
//! `interpretation.model.load.error` already existed as an exact leaf but had
//! ZERO emit sites repo-wide — a pre-allocated declaration. This chunk gives it
//! its first producer and completes it with the two fields that producer adds.
//!
//! The fallback shape here is the MOST deceptive of the three: a bare
//! `interpretation` key EXISTS and carries a full, unrelated field set, so
//! deleting either exact leaf leaves `for_target` RESOLVING SUCCESSFULLY while
//! every field vanishes. A resolver-only probe passes vacuously against that,
//! which is why each leaf carries a discriminator pin asserting the bare set
//! does not contain its fields.

use pulse_app::observability::AllowList;

const LOAD_ERROR: &str = "interpretation.model.load.error";
const LOAD_ERROR_FIELDS: [&str; 6] = [
    "error_msg",
    "model_identity",
    "recovery_action",
    "error_category",
    "env_var",
    "path_basename",
];

const ALLOW_ROOT: &str = "interpretation.model.allow_root";
const ALLOW_ROOT_FIELDS: [&str; 2] = ["confinement", "root_basename"];

const BARE_PREFIX: &str = "interpretation";

#[test]
fn load_error_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(LOAD_ERROR).is_some(),
        "`{LOAD_ERROR}` must resolve to an allowlist entry",
    );
}

#[test]
fn load_error_leaf_carries_exactly_the_emitted_field_set() {
    // Both directions: a narrowed leaf (a field dropped → silently redacted in
    // production) and a widened one (fields no emit site sends) fail.
    let al = AllowList::production();
    let set = al
        .for_target(LOAD_ERROR)
        .expect("interpretation.model.load.error must resolve to an allowlist entry");
    for field in LOAD_ERROR_FIELDS {
        assert!(
            set.contains(field),
            "`{LOAD_ERROR}` leaf is missing `{field}` — it would be redacted in production",
        );
    }
    assert_eq!(
        set.len(),
        LOAD_ERROR_FIELDS.len(),
        "`{LOAD_ERROR}` leaf must carry exactly the emitted field set; got {set:?}",
    );
}

#[test]
fn allow_root_resolves_to_an_exact_leaf() {
    let al = AllowList::production();
    assert!(
        al.for_target(ALLOW_ROOT).is_some(),
        "`{ALLOW_ROOT}` must resolve to an allowlist entry",
    );
}

#[test]
fn allow_root_leaf_carries_exactly_the_emitted_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target(ALLOW_ROOT)
        .expect("interpretation.model.allow_root must resolve to an allowlist entry");
    for field in ALLOW_ROOT_FIELDS {
        assert!(
            set.contains(field),
            "`{ALLOW_ROOT}` leaf is missing `{field}` — it would be redacted in production",
        );
    }
    assert_eq!(
        set.len(),
        ALLOW_ROOT_FIELDS.len(),
        "`{ALLOW_ROOT}` leaf must carry exactly the emitted field set; got {set:?}",
    );
}

#[test]
fn the_bare_interpretation_fallback_cannot_serve_either_target() {
    // STRENGTHENED 2026-08-30: the populated bare `interpretation` key this
    // pin guarded against was REMOVED by the diagnostics sweep. A deleted
    // exact leaf now resolves NOTHING instead of a sibling set — assert the
    // bare key stays gone (obs-plan §8 no-bare-prefix invariant).
    let al = AllowList::production();
    assert!(al.for_target(BARE_PREFIX).is_none());
}
