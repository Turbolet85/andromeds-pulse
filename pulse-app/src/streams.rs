use std::sync::Arc;

use buffer::{
    BroadcastSenders, MAX_PAYLOAD_BYTES, STREAM_NAME_LOGS, STREAM_NAME_METRICS, STREAM_NAME_SPANS,
};
use bytes::Bytes;
use tauri::ipc::Channel;
use tokio::sync::broadcast;
use ui_bridge::AppError;

#[taurpc::procedures(path = "streams")]
pub trait StreamsApi {
    async fn subscribe_spans(channel: Channel<Vec<u8>>) -> Result<(), AppError>;
    async fn subscribe_metrics(channel: Channel<Vec<u8>>) -> Result<(), AppError>;
    async fn subscribe_logs(channel: Channel<Vec<u8>>) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct StreamsApiImpl {
    senders: Arc<BroadcastSenders>,
}

impl StreamsApiImpl {
    pub fn new(senders: Arc<BroadcastSenders>) -> Self {
        Self { senders }
    }
}

#[taurpc::resolvers]
impl StreamsApi for StreamsApiImpl {
    async fn subscribe_spans(self, channel: Channel<Vec<u8>>) -> Result<(), AppError> {
        let receiver = self.senders.spans.subscribe();
        tokio::spawn(forward_loop(STREAM_NAME_SPANS, receiver, channel));
        Ok(())
    }

    async fn subscribe_metrics(self, channel: Channel<Vec<u8>>) -> Result<(), AppError> {
        let receiver = self.senders.metrics.subscribe();
        tokio::spawn(forward_loop(STREAM_NAME_METRICS, receiver, channel));
        Ok(())
    }

    async fn subscribe_logs(self, channel: Channel<Vec<u8>>) -> Result<(), AppError> {
        let receiver = self.senders.logs.subscribe();
        tokio::spawn(forward_loop(STREAM_NAME_LOGS, receiver, channel));
        Ok(())
    }
}

async fn forward_loop(
    stream_name: &'static str,
    mut receiver: broadcast::Receiver<Bytes>,
    channel: Channel<Vec<u8>>,
) {
    loop {
        match receiver.recv().await {
            Ok(bytes) => {
                let payload_size_bytes = bytes.len();
                if payload_size_bytes > MAX_PAYLOAD_BYTES {
                    tracing::error!(
                        target: "tauri.channel.emit.size_exceeded",
                        channel_name = stream_name,
                        payload_size_bytes = payload_size_bytes,
                        limit_bytes = MAX_PAYLOAD_BYTES,
                        error_type = "size_exceeded",
                        "Arrow IPC payload exceeded broadcast size cap; dropping",
                    );
                    continue;
                }
                let payload: Vec<u8> = bytes.to_vec();
                if channel.send(payload).is_err() {
                    // Webview disconnected; exit forwarding task and release subscriber slot
                    break;
                }
                tracing::info!(
                    target: "tauri.channel.emit",
                    channel_name = stream_name,
                    payload_size_bytes = payload_size_bytes,
                    schema_match = true,
                    "Arrow IPC payload emitted",
                );
            }
            Err(broadcast::error::RecvError::Lagged(dropped)) => {
                tracing::warn!(
                    target: "tauri.channel.lag",
                    channel_name = stream_name,
                    dropped_count = dropped,
                    "broadcast receiver lagged; dropped messages",
                );
            }
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use buffer::broadcast as broadcast_module;

    #[test]
    fn streams_api_impl_constructor_holds_senders() {
        let senders = Arc::new(broadcast_module::create());
        let api = StreamsApiImpl::new(Arc::clone(&senders));
        assert_eq!(api.senders.spans.receiver_count(), 0);
        assert_eq!(api.senders.metrics.receiver_count(), 0);
        assert_eq!(api.senders.logs.receiver_count(), 0);
    }

    #[test]
    fn streams_api_impl_clones_share_senders() {
        let senders = Arc::new(broadcast_module::create());
        let api1 = StreamsApiImpl::new(Arc::clone(&senders));
        let api2 = api1.clone();
        let _r1 = api1.senders.spans.subscribe();
        assert_eq!(api2.senders.spans.receiver_count(), 1);
    }
}
