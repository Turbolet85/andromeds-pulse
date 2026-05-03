use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("placeholder")]
    Placeholder,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BufferHeartbeat {
    pub rows_ingested: u64,
    pub retention_window_active: bool,
    pub eviction_count: u64,
}

pub fn heartbeat_payload() -> BufferHeartbeat {
    BufferHeartbeat::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_is_callable() {
        let h = heartbeat_payload();
        assert_eq!(h.rows_ingested, 0);
        assert!(!h.retention_window_active);
        assert_eq!(h.eviction_count, 0);
    }
}
