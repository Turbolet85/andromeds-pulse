use tokio::sync::mpsc;

use crate::contract::Error;
use crate::grpc::proto::opentelemetry::proto::logs::v1::ResourceLogs;
use crate::grpc::proto::opentelemetry::proto::metrics::v1::ResourceMetrics;
use crate::grpc::proto::opentelemetry::proto::trace::v1::ResourceSpans;

pub const MPSC_CAPACITY: usize = 1024;

#[derive(Debug)]
pub enum Batch {
    Spans(Vec<ResourceSpans>),
    Metrics(Vec<ResourceMetrics>),
    Logs(Vec<ResourceLogs>),
}

pub type IngestReceiver = mpsc::Receiver<Batch>;

#[derive(Debug, Clone)]
pub struct IngestSender {
    tx: mpsc::Sender<Batch>,
    capacity: usize,
}

impl IngestSender {
    pub fn try_send(&self, batch: Batch) -> Result<(), Error> {
        match self.tx.try_send(batch) {
            Ok(()) => Ok(()),
            Err(mpsc::error::TrySendError::Full(_)) => Err(Error::ChannelFull),
            Err(mpsc::error::TrySendError::Closed(_)) => Err(Error::ChannelFull),
        }
    }

    pub fn capacity_pct(&self) -> f64 {
        let free = self.tx.capacity();
        let used = self.capacity.saturating_sub(free);
        (used as f64 / self.capacity as f64) * 100.0
    }
}

pub fn build_channel() -> (IngestSender, IngestReceiver) {
    build_channel_with_capacity(MPSC_CAPACITY)
}

pub fn build_channel_with_capacity(capacity: usize) -> (IngestSender, IngestReceiver) {
    let (tx, rx) = mpsc::channel(capacity);
    (IngestSender { tx, capacity }, rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn build_channel_default_capacity_matches_constant() {
        let (sender, _rx) = build_channel();
        assert_eq!(sender.capacity, MPSC_CAPACITY);
        assert_eq!(sender.capacity_pct(), 0.0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn try_send_succeeds_within_capacity() {
        let (sender, mut rx) = build_channel_with_capacity(2);
        sender
            .try_send(Batch::Spans(Vec::new()))
            .expect("first send within capacity");
        sender
            .try_send(Batch::Spans(Vec::new()))
            .expect("second send within capacity");
        let _ = rx.try_recv();
        let _ = rx.try_recv();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn try_send_returns_channel_full_when_saturated() {
        let (sender, _rx) = build_channel_with_capacity(1);
        sender
            .try_send(Batch::Spans(Vec::new()))
            .expect("first send fills channel");
        let result = sender.try_send(Batch::Spans(Vec::new()));
        assert!(matches!(result, Err(Error::ChannelFull)));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn try_send_returns_channel_full_when_receiver_dropped() {
        let (sender, rx) = build_channel_with_capacity(1);
        drop(rx);
        let result = sender.try_send(Batch::Spans(Vec::new()));
        assert!(matches!(result, Err(Error::ChannelFull)));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn capacity_pct_reflects_used_slots() {
        let (sender, _rx) = build_channel_with_capacity(4);
        assert_eq!(sender.capacity_pct(), 0.0);
        sender
            .try_send(Batch::Spans(Vec::new()))
            .expect("send within capacity");
        // 1 of 4 used = 25%.
        let pct = sender.capacity_pct();
        assert!((pct - 25.0).abs() < 0.01);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn capacity_pct_recovers_after_drain() {
        let (sender, mut rx) = build_channel_with_capacity(2);
        sender
            .try_send(Batch::Spans(Vec::new()))
            .expect("send within capacity");
        sender
            .try_send(Batch::Spans(Vec::new()))
            .expect("send within capacity");
        assert!((sender.capacity_pct() - 100.0).abs() < 0.01);
        let _ = rx.recv().await.expect("receiver drains one batch");
        let pct = sender.capacity_pct();
        assert!((pct - 50.0).abs() < 0.01);
    }
}
