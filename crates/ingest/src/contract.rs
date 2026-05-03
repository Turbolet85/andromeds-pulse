use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("placeholder")]
    Placeholder,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IngestHeartbeat {
    pub span_count: u64,
    pub buffer_capacity_pct: f64,
    pub broadcast_subscribers: u32,
}

pub fn heartbeat_payload() -> IngestHeartbeat {
    IngestHeartbeat::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_is_callable() {
        let h = heartbeat_payload();
        assert_eq!(h.span_count, 0);
        assert_eq!(h.broadcast_subscribers, 0);
    }
}
