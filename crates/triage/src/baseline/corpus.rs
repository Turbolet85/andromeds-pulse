use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use super::{BaselineError, BaselineState, SCHEMA_VERSION};

pub const DEFAULT_MAX_SIZE_BYTES: u64 = 1024 * 1024;
pub const DEFAULT_SERVICE_COUNT_CAP: usize = 1024;

#[derive(Debug, Clone, Copy)]
pub struct PersistStats {
    pub bytes_written: u64,
    pub service_count: usize,
}

#[derive(Debug)]
pub enum BootstrapResult {
    /// Corpus loaded successfully; `age_nanos` is the elapsed time since
    /// `persisted_at_unix_nanos` was last written. Caller decides whether to
    /// keep loaded state or fall through to fresh based on age threshold.
    Loaded {
        state: BaselineState,
        age_nanos: i64,
    },
    /// No usable corpus; caller should bootstrap fresh state. `reason_kind`
    /// is one of "cold_start" / "corpus_corrupt_reset" / "schema_mismatch"
    /// / "size_cap_exceeded" / "service_count_cap_exceeded" for the
    /// `pipeline.l1b.bootstrap_count_total{kind}` metric label.
    Fresh { reason_kind: &'static str },
}

pub fn resolve_corpus_path(data_dir: &Path) -> Result<PathBuf, BaselineError> {
    let triage_dir = data_dir.join("triage");
    fs::create_dir_all(&triage_dir).map_err(|e| BaselineError::Io { kind: e.kind() })?;
    let canonical_data = fs::canonicalize(data_dir)
        .map_err(|e| BaselineError::PathCanonicalize { kind: e.kind() })?;
    let canonical_triage = fs::canonicalize(&triage_dir)
        .map_err(|e| BaselineError::PathCanonicalize { kind: e.kind() })?;
    if !canonical_triage.starts_with(&canonical_data) {
        return Err(BaselineError::PathTraversal);
    }
    Ok(canonical_triage.join("baseline-corpus.bin"))
}

pub fn persist_state(path: &Path, state: &BaselineState) -> Result<PersistStats, BaselineError> {
    let bytes = bincode::serialize(state).map_err(|_| BaselineError::Serialize)?;
    let service_count = state.service_count();

    let parent = path.parent().ok_or(BaselineError::PathTraversal)?;
    if !parent.exists() {
        fs::create_dir_all(parent).map_err(|e| BaselineError::Io { kind: e.kind() })?;
    }

    let mut tmp_path = path.to_path_buf();
    let mut tmp_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "baseline-corpus.bin".to_string());
    tmp_name.push_str(".tmp");
    tmp_path.set_file_name(tmp_name);

    fs::write(&tmp_path, &bytes).map_err(|e| BaselineError::Io { kind: e.kind() })?;
    fs::rename(&tmp_path, path).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        BaselineError::Io { kind: e.kind() }
    })?;

    Ok(PersistStats {
        bytes_written: bytes.len() as u64,
        service_count,
    })
}

pub fn load_state(
    path: &Path,
    max_size_bytes: u64,
    service_count_cap: usize,
) -> Result<BaselineState, BaselineError> {
    if !path.exists() {
        return Err(BaselineError::Io {
            kind: io::ErrorKind::NotFound,
        });
    }

    let metadata = fs::metadata(path).map_err(|e| BaselineError::Io { kind: e.kind() })?;
    let file_size = metadata.len();
    if file_size > max_size_bytes {
        return Err(BaselineError::SizeCapExceeded {
            actual: file_size,
            max: max_size_bytes,
        });
    }

    let mut file = fs::File::open(path).map_err(|e| BaselineError::Io { kind: e.kind() })?;
    let mut buf = Vec::with_capacity(file_size as usize);
    file.read_to_end(&mut buf)
        .map_err(|e| BaselineError::Io { kind: e.kind() })?;

    if (buf.len() as u64) > max_size_bytes {
        return Err(BaselineError::SizeCapExceeded {
            actual: buf.len() as u64,
            max: max_size_bytes,
        });
    }

    let state: BaselineState =
        bincode::deserialize(&buf).map_err(|_| BaselineError::Deserialize)?;

    if state.schema_version() != SCHEMA_VERSION {
        return Err(BaselineError::SchemaVersionMismatch {
            expected: SCHEMA_VERSION,
            got: state.schema_version(),
        });
    }

    let service_count = state.service_count();
    if service_count > service_count_cap {
        return Err(BaselineError::ServiceCountCapExceeded {
            count: service_count,
            max: service_count_cap,
        });
    }

    Ok(state)
}

pub fn bootstrap_from_corpus(
    path: &Path,
    max_size_bytes: u64,
    service_count_cap: usize,
    now_nanos: i64,
) -> BootstrapResult {
    if !path.exists() {
        return BootstrapResult::Fresh {
            reason_kind: "cold_start",
        };
    }
    match load_state(path, max_size_bytes, service_count_cap) {
        Ok(state) => {
            let persisted_at = state.persisted_at_unix_nanos();
            let age_nanos = now_nanos.saturating_sub(persisted_at);
            BootstrapResult::Loaded { state, age_nanos }
        }
        Err(BaselineError::SchemaVersionMismatch { .. }) => BootstrapResult::Fresh {
            reason_kind: "schema_mismatch",
        },
        Err(BaselineError::SizeCapExceeded { .. }) => BootstrapResult::Fresh {
            reason_kind: "size_cap_exceeded",
        },
        Err(BaselineError::ServiceCountCapExceeded { .. }) => BootstrapResult::Fresh {
            reason_kind: "service_count_cap_exceeded",
        },
        Err(_) => BootstrapResult::Fresh {
            reason_kind: "corpus_corrupt_reset",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn resolve_corpus_path_creates_triage_subdir() {
        let tmp = TempDir::new().expect("tmp dir");
        let resolved = resolve_corpus_path(tmp.path()).expect("resolve");
        assert!(resolved.starts_with(tmp.path().canonicalize().unwrap()));
        assert_eq!(resolved.file_name().unwrap(), "baseline-corpus.bin");
        assert!(
            resolved.parent().unwrap().exists(),
            "triage subdir not created"
        );
    }

    #[test]
    fn persist_load_round_trip_preserves_state() {
        let tmp = TempDir::new().expect("tmp dir");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");

        let state = BaselineState::new();
        state.observe_span("svc-a", "op-1", 0, 100, 1_000);
        state.observe_span("svc-a", "op-2", 2, 200, 2_000);
        state.observe_span("svc-b", "op-1", 0, 50, 3_000);
        state.set_persisted_at_unix_nanos(5_000);

        let stats = persist_state(&path, &state).expect("persist");
        assert!(stats.bytes_written > 0);
        assert_eq!(stats.service_count, 2);

        let loaded =
            load_state(&path, DEFAULT_MAX_SIZE_BYTES, DEFAULT_SERVICE_COUNT_CAP).expect("load");
        assert_eq!(loaded.schema_version(), SCHEMA_VERSION);
        assert_eq!(loaded.service_count(), 2);
        assert_eq!(loaded.persisted_at_unix_nanos(), 5_000);
    }

    #[test]
    fn load_missing_file_returns_io_not_found() {
        let tmp = TempDir::new().expect("tmp dir");
        let path = tmp.path().join("missing-baseline-corpus.bin");
        let result = load_state(&path, DEFAULT_MAX_SIZE_BYTES, DEFAULT_SERVICE_COUNT_CAP);
        match result {
            Err(BaselineError::Io { kind }) => assert_eq!(kind, io::ErrorKind::NotFound),
            other => panic!("expected Io NotFound, got {other:?}"),
        }
    }

    #[test]
    fn load_oversize_file_returns_size_cap_exceeded() {
        let tmp = TempDir::new().expect("tmp dir");
        let triage = tmp.path().join("triage");
        fs::create_dir_all(&triage).unwrap();
        let path = triage.join("baseline-corpus.bin");
        let huge = vec![0u8; 2 * 1024];
        fs::write(&path, &huge).unwrap();
        let result = load_state(&path, 1024, DEFAULT_SERVICE_COUNT_CAP);
        match result {
            Err(BaselineError::SizeCapExceeded { actual, max }) => {
                assert_eq!(actual, 2 * 1024);
                assert_eq!(max, 1024);
            }
            other => panic!("expected SizeCapExceeded, got {other:?}"),
        }
    }

    #[test]
    fn load_truncated_corrupt_bytes_returns_deserialize_error() {
        let tmp = TempDir::new().expect("tmp dir");
        let triage = tmp.path().join("triage");
        fs::create_dir_all(&triage).unwrap();
        let path = triage.join("baseline-corpus.bin");
        fs::write(&path, b"\x00\x01\x02\x03\x04").unwrap();
        let result = load_state(&path, DEFAULT_MAX_SIZE_BYTES, DEFAULT_SERVICE_COUNT_CAP);
        assert!(matches!(result, Err(BaselineError::Deserialize)));
    }

    #[test]
    fn bootstrap_missing_corpus_returns_cold_start() {
        let tmp = TempDir::new().expect("tmp dir");
        let path = tmp.path().join("nonexistent-baseline-corpus.bin");
        let result = bootstrap_from_corpus(
            &path,
            DEFAULT_MAX_SIZE_BYTES,
            DEFAULT_SERVICE_COUNT_CAP,
            1_000,
        );
        assert!(matches!(
            result,
            BootstrapResult::Fresh {
                reason_kind: "cold_start"
            }
        ));
    }

    #[test]
    fn bootstrap_corrupt_corpus_returns_corpus_corrupt_reset() {
        let tmp = TempDir::new().expect("tmp dir");
        let triage = tmp.path().join("triage");
        fs::create_dir_all(&triage).unwrap();
        let path = triage.join("baseline-corpus.bin");
        fs::write(&path, b"corrupt").unwrap();
        let result = bootstrap_from_corpus(
            &path,
            DEFAULT_MAX_SIZE_BYTES,
            DEFAULT_SERVICE_COUNT_CAP,
            1_000,
        );
        assert!(matches!(
            result,
            BootstrapResult::Fresh {
                reason_kind: "corpus_corrupt_reset"
            }
        ));
    }

    #[test]
    fn bootstrap_loaded_returns_age_nanos() {
        let tmp = TempDir::new().expect("tmp dir");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let state = BaselineState::new();
        state.set_persisted_at_unix_nanos(1_000);
        persist_state(&path, &state).expect("persist");

        let result = bootstrap_from_corpus(
            &path,
            DEFAULT_MAX_SIZE_BYTES,
            DEFAULT_SERVICE_COUNT_CAP,
            1_500,
        );
        match result {
            BootstrapResult::Loaded { age_nanos, .. } => assert_eq!(age_nanos, 500),
            other => panic!("expected Loaded, got {other:?}"),
        }
    }

    #[test]
    fn persisted_corpus_does_not_contain_pii_secret_canary() {
        let tmp = TempDir::new().expect("tmp dir");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let state = BaselineState::new();
        let canary_service = "service-a";
        state.observe_span(canary_service, "op-1", 0, 100, 1_000);
        state.observe_span(canary_service, "op-2", 2, 200, 2_000);
        persist_state(&path, &state).expect("persist");

        let on_disk = fs::read(&path).expect("read");
        let on_disk_str = String::from_utf8_lossy(&on_disk);
        let pii_patterns = ["span_id", "trace_id", "attribute_value", "operation_name"];
        for pattern in &pii_patterns {
            assert!(
                !on_disk_str.contains(pattern),
                "PII pattern {pattern:?} leaked into corpus bytes"
            );
        }
    }
}
