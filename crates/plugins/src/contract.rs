use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("placeholder")]
    Placeholder,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PluginsHeartbeat {
    pub loaded_count: u32,
    pub active_invocations: u32,
}

pub fn heartbeat_payload() -> PluginsHeartbeat {
    PluginsHeartbeat::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_is_callable() {
        let h = heartbeat_payload();
        assert_eq!(h.loaded_count, 0);
        assert_eq!(h.active_invocations, 0);
    }
}
