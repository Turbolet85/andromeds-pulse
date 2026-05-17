use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BaselineError {
    #[error("io error during corpus operation: {kind:?}")]
    Io { kind: io::ErrorKind },
    #[error("corpus serialize failed")]
    Serialize,
    #[error("corpus deserialize failed")]
    Deserialize,
    #[error("corpus schema version mismatch: expected {expected}, got {got}")]
    SchemaVersionMismatch { expected: u32, got: u32 },
    #[error("corpus file size cap exceeded: {actual} bytes > max {max}")]
    SizeCapExceeded { actual: u64, max: u64 },
    #[error("corpus service count cap exceeded: {count} > max {max}")]
    ServiceCountCapExceeded { count: usize, max: usize },
    #[error("corpus path traversal blocked: resolved outside data dir")]
    PathTraversal,
    #[error("corpus path canonicalize failed: {kind:?}")]
    PathCanonicalize { kind: io::ErrorKind },
}

impl BaselineError {
    pub fn error_category(&self) -> &'static str {
        match self {
            Self::Io { .. } | Self::PathCanonicalize { .. } => "io",
            Self::Serialize | Self::Deserialize | Self::SchemaVersionMismatch { .. } => "serialize",
            Self::SizeCapExceeded { .. }
            | Self::ServiceCountCapExceeded { .. }
            | Self::PathTraversal => "permission",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_category_groups_io_serialize_permission() {
        assert_eq!(
            BaselineError::Io {
                kind: io::ErrorKind::NotFound
            }
            .error_category(),
            "io"
        );
        assert_eq!(
            BaselineError::PathCanonicalize {
                kind: io::ErrorKind::PermissionDenied
            }
            .error_category(),
            "io"
        );
        assert_eq!(BaselineError::Serialize.error_category(), "serialize");
        assert_eq!(BaselineError::Deserialize.error_category(), "serialize");
        assert_eq!(
            BaselineError::SchemaVersionMismatch {
                expected: 1,
                got: 2
            }
            .error_category(),
            "serialize"
        );
        assert_eq!(
            BaselineError::SizeCapExceeded {
                actual: 100,
                max: 50
            }
            .error_category(),
            "permission"
        );
        assert_eq!(
            BaselineError::ServiceCountCapExceeded { count: 10, max: 5 }.error_category(),
            "permission"
        );
        assert_eq!(BaselineError::PathTraversal.error_category(), "permission");
    }

    #[test]
    fn error_display_does_not_leak_raw_paths_or_chain() {
        let e = BaselineError::Io {
            kind: io::ErrorKind::NotFound,
        };
        let msg = format!("{e}");
        assert!(msg.contains("NotFound"));
        assert!(!msg.contains("/home"));
        assert!(!msg.contains("\\Users"));
    }
}
