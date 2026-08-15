//! Resolver probes for the corpus key-custody observability surface.
//!
//! These live here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level tests
//! in `pulse-app` compile but never run — a probe that cannot fail is not a
//! guard.
//!
//! `corpus.open.error` is the reason this file exists at all: it emitted from
//! the boot path with no resolvable entry, so the one diagnostic that would
//! have named the per-process-ephemeral-key defect was itself redacted.

use pulse_app::observability::AllowList;

const CORPUS_TARGETS: &[&str] = &[
    "corpus.open.error",
    "corpus.keychain.fallback",
    "corpus.read.undecryptable",
    "corpus.orphan.disposition",
];

#[test]
fn corpus_open_error_resolves_and_allows_its_error_kind() {
    let al = AllowList::production();
    let set = al
        .for_target("corpus.open.error")
        .expect("corpus.open.error must have an explicit leaf entry");

    assert!(
        set.contains("error_kind"),
        "the corpus boot-failure diagnostic must not be redacted"
    );
}

#[test]
fn keychain_fallback_target_allows_its_bounded_fields() {
    let al = AllowList::production();
    let set = al
        .for_target("corpus.keychain.fallback")
        .expect("the fallback warning must have an explicit leaf entry");

    for field in ["backend_kind", "reason", "consequence"] {
        assert!(set.contains(field), "fallback warning must allow `{field}`");
    }
}

#[test]
fn undecryptable_read_target_allows_only_the_aggregate_fields() {
    let al = AllowList::production();
    let set = al
        .for_target("corpus.read.undecryptable")
        .expect("the skipped-row warning must have an explicit leaf entry");

    assert!(set.contains("query_id"));
    assert!(set.contains("rows_skipped"));
}

#[test]
fn disposition_target_allows_its_outcome_and_counts() {
    let al = AllowList::production();
    let set = al
        .for_target("corpus.orphan.disposition")
        .expect("the disposition summary must have an explicit leaf entry");

    for field in [
        "disposition_outcome",
        "rows_purged",
        "tables_affected",
        "error_category",
    ] {
        assert!(set.contains(field), "disposition must allow `{field}`");
    }
}

#[test]
fn corpus_targets_are_not_served_by_a_bare_prefix_entry() {
    // `for_target` falls back exact -> strip `.tick` -> split('.') -> split("::").
    // A bare `corpus` key would silently widen every future `corpus.*` target to
    // one field set, so its ABSENCE is the invariant under test.
    let al = AllowList::production();

    assert!(
        al.for_target("corpus").is_none(),
        "a bare `corpus` allowlist key must not exist"
    );
    assert!(
        al.for_target("corpus.not.a.real.target").is_none(),
        "an unregistered corpus target must resolve to None, not to a sibling's fields"
    );
}

#[test]
fn corpus_targets_carry_no_key_material_or_identity_fields() {
    // obs-plan §5 cardinality discipline + security-plan §Logging: the key, the
    // passphrase, and per-row identity never reach a log field.
    let al = AllowList::production();

    for target in CORPUS_TARGETS {
        let set = al.for_target(target).expect("target resolves");
        for banned in [
            "key",
            "key_bytes",
            "passphrase",
            "secret",
            "payload",
            "ciphertext",
            "workspace",
            "path",
            "incident_id",
            "service_name",
        ] {
            assert!(!set.contains(banned), "{target} must not carry `{banned}`");
        }
    }
}
