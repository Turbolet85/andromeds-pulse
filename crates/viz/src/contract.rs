use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("placeholder")]
    Placeholder,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VizHeartbeat {
    pub query_latency_ms: f64,
    pub subscribers_active: u32,
}

pub fn heartbeat_payload() -> VizHeartbeat {
    VizHeartbeat::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_is_callable() {
        let h = heartbeat_payload();
        assert_eq!(h.query_latency_ms, 0.0);
        assert_eq!(h.subscribers_active, 0);
    }
}
