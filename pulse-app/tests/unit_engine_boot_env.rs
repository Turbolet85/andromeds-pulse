//! The boot helpers' environment resolvers (`pulse_app::engine_boot`): the OTLP
//! port variables and the retention variable.
//!
//! Lives under `pulse-app/tests/` because `[lib] test = false` means a
//! src-level `mod tests` in `pulse-app` compiles and never runs.

use ingest::contract::Error as IngestError;
use pulse_app::engine_boot::{
    ENV_RETENTION_SECONDS, RETENTION_SECONDS_DEFAULT, RETENTION_SECONDS_MAX, RETENTION_SECONDS_MIN,
    resolve_port, resolve_retention_seconds,
};

// SAFETY: env::set_var / remove_var are unsafe in Rust 2024 edition because
// they race with concurrent threads' env reads. cargo-nextest gives us
// per-process test isolation (per .claude/rules/testing.md §Framework), and
// each test uses a unique env-var name so there is no overlap with sibling
// tests sharing the same process. Calls are scoped narrowly and the env
// var is removed at end-of-test.

#[test]
fn resolve_port_unset_env_returns_default() {
    const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_UNSET";
    unsafe {
        std::env::remove_var(TEST_ENV);
    }
    let result = resolve_port(TEST_ENV, 4317);
    let port = result.expect("unset env returns default port");
    assert_eq!(port.value(), 4317);
}

#[test]
fn resolve_port_unparseable_env_returns_invalid_port_err() {
    const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_UNPARSEABLE";
    unsafe {
        std::env::set_var(TEST_ENV, "not-a-number");
    }
    let result = resolve_port(TEST_ENV, 4317);
    unsafe {
        std::env::remove_var(TEST_ENV);
    }
    assert!(matches!(result, Err(IngestError::InvalidPort { .. })));
}

#[test]
fn resolve_port_overflow_env_returns_invalid_port_err() {
    const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_OVERFLOW";
    unsafe {
        std::env::set_var(TEST_ENV, "99999");
    }
    let result = resolve_port(TEST_ENV, 4317);
    unsafe {
        std::env::remove_var(TEST_ENV);
    }
    assert!(matches!(result, Err(IngestError::InvalidPort { .. })));
}

#[test]
fn resolve_port_privileged_env_returns_invalid_port_err() {
    const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_PRIVILEGED";
    unsafe {
        std::env::set_var(TEST_ENV, "80");
    }
    let result = resolve_port(TEST_ENV, 4317);
    unsafe {
        std::env::remove_var(TEST_ENV);
    }
    assert!(matches!(result, Err(IngestError::InvalidPort { .. })));
}

#[test]
fn resolve_port_zero_env_returns_invalid_port_err() {
    const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_ZERO";
    unsafe {
        std::env::set_var(TEST_ENV, "0");
    }
    let result = resolve_port(TEST_ENV, 4317);
    unsafe {
        std::env::remove_var(TEST_ENV);
    }
    assert!(matches!(result, Err(IngestError::InvalidPort { .. })));
}

#[test]
fn resolve_port_valid_non_privileged_returns_ok() {
    const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_VALID";
    unsafe {
        std::env::set_var(TEST_ENV, "9000");
    }
    let result = resolve_port(TEST_ENV, 4317);
    unsafe {
        std::env::remove_var(TEST_ENV);
    }
    let port = result.expect("non-privileged port must pass");
    assert_eq!(port.value(), 9000);
}

#[test]
fn resolve_port_spec_default_via_env_returns_ok() {
    const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_SPEC_DEFAULT";
    unsafe {
        std::env::set_var(TEST_ENV, "4318");
    }
    let result = resolve_port(TEST_ENV, 4317);
    unsafe {
        std::env::remove_var(TEST_ENV);
    }
    let port = result.expect("spec-default 4318 via env must pass");
    assert_eq!(port.value(), 4318);
}

// resolve_retention_seconds tests (chunk #21). Same `unsafe { std::env::set_var }`
// discipline as resolve_port tests above — cargo-nextest gives per-process
// isolation, each test uses a distinct fixture by removing/setting the
// same env var (ANDROMEDA_PULSE_RETENTION_SECONDS) within a narrow scope.

#[test]
fn resolve_retention_seconds_unset_returns_default() {
    unsafe {
        std::env::remove_var(ENV_RETENTION_SECONDS);
    }
    assert_eq!(resolve_retention_seconds(), RETENTION_SECONDS_DEFAULT);
}

#[test]
fn resolve_retention_seconds_unparseable_falls_back_to_default() {
    unsafe {
        std::env::set_var(ENV_RETENTION_SECONDS, "not-a-number");
    }
    let result = resolve_retention_seconds();
    unsafe {
        std::env::remove_var(ENV_RETENTION_SECONDS);
    }
    assert_eq!(result, RETENTION_SECONDS_DEFAULT);
}

#[test]
fn resolve_retention_seconds_below_min_falls_back_to_default() {
    unsafe {
        std::env::set_var(ENV_RETENTION_SECONDS, "30");
    }
    let result = resolve_retention_seconds();
    unsafe {
        std::env::remove_var(ENV_RETENTION_SECONDS);
    }
    assert_eq!(result, RETENTION_SECONDS_DEFAULT);
}

#[test]
fn resolve_retention_seconds_above_max_falls_back_to_default() {
    unsafe {
        std::env::set_var(ENV_RETENTION_SECONDS, "999999");
    }
    let result = resolve_retention_seconds();
    unsafe {
        std::env::remove_var(ENV_RETENTION_SECONDS);
    }
    assert_eq!(result, RETENTION_SECONDS_DEFAULT);
}

#[test]
fn resolve_retention_seconds_in_range_returns_parsed_value() {
    unsafe {
        std::env::set_var(ENV_RETENTION_SECONDS, "300");
    }
    let result = resolve_retention_seconds();
    unsafe {
        std::env::remove_var(ENV_RETENTION_SECONDS);
    }
    assert_eq!(result, 300);
}

#[test]
fn resolve_retention_seconds_at_min_boundary_returns_min() {
    unsafe {
        std::env::set_var(ENV_RETENTION_SECONDS, "60");
    }
    let result = resolve_retention_seconds();
    unsafe {
        std::env::remove_var(ENV_RETENTION_SECONDS);
    }
    assert_eq!(result, RETENTION_SECONDS_MIN);
}

#[test]
fn resolve_retention_seconds_at_max_boundary_returns_max() {
    unsafe {
        std::env::set_var(ENV_RETENTION_SECONDS, "86400");
    }
    let result = resolve_retention_seconds();
    unsafe {
        std::env::remove_var(ENV_RETENTION_SECONDS);
    }
    assert_eq!(result, RETENTION_SECONDS_MAX);
}
