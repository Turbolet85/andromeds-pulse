use arrow::error::ArrowError;
use arrow::ipc::writer::StreamWriter;
use arrow::record_batch::RecordBatch;
use bytes::Bytes;
use tokio::sync::broadcast;

use crate::contract::Error;

pub const STREAM_NAME_SPANS: &str = "pulse://stream/spans";
pub const STREAM_NAME_METRICS: &str = "pulse://stream/metrics";
pub const STREAM_NAME_LOGS: &str = "pulse://stream/logs";

pub const MAX_PAYLOAD_BYTES: usize = 8 * 1024 * 1024;
pub const BROADCAST_CAPACITY: usize = 128;

#[derive(Debug, Clone)]
pub struct BroadcastSenders {
    pub spans: broadcast::Sender<Bytes>,
    pub metrics: broadcast::Sender<Bytes>,
    pub logs: broadcast::Sender<Bytes>,
}

pub fn create() -> BroadcastSenders {
    BroadcastSenders {
        spans: broadcast::channel::<Bytes>(BROADCAST_CAPACITY).0,
        metrics: broadcast::channel::<Bytes>(BROADCAST_CAPACITY).0,
        logs: broadcast::channel::<Bytes>(BROADCAST_CAPACITY).0,
    }
}

pub fn encode_spans(batch: &RecordBatch) -> Result<Bytes, Error> {
    encode_with_cap(batch, MAX_PAYLOAD_BYTES)
}

pub fn encode_metrics(batch: &RecordBatch) -> Result<Bytes, Error> {
    encode_with_cap(batch, MAX_PAYLOAD_BYTES)
}

pub fn encode_logs(batch: &RecordBatch) -> Result<Bytes, Error> {
    encode_with_cap(batch, MAX_PAYLOAD_BYTES)
}

pub(crate) fn encode_with_cap(batch: &RecordBatch, cap: usize) -> Result<Bytes, Error> {
    let mut buf: Vec<u8> = Vec::new();
    let schema = batch.schema();
    {
        let mut writer = StreamWriter::try_new(&mut buf, &schema).map_err(map_arrow_err)?;
        writer.write(batch).map_err(map_arrow_err)?;
        writer.finish().map_err(map_arrow_err)?;
    }
    if buf.len() > cap {
        return Err(Error::BroadcastSizeCapExceeded {
            payload_bytes: buf.len(),
        });
    }
    Ok(Bytes::from(buf))
}

fn map_arrow_err(_e: ArrowError) -> Error {
    Error::BroadcastEncode {
        reason: "encode_failed".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use arrow::array::{Int32Array, Int64Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::ipc::reader::StreamReader;

    fn one_column_int32_batch(values: Vec<i32>) -> RecordBatch {
        let schema = Arc::new(Schema::new(vec![Field::new("v", DataType::Int32, false)]));
        let array = Int32Array::from(values);
        RecordBatch::try_new(schema, vec![Arc::new(array)]).expect("build batch")
    }

    #[test]
    fn stream_name_constants_match_arch_literals() {
        assert_eq!(STREAM_NAME_SPANS, "pulse://stream/spans");
        assert_eq!(STREAM_NAME_METRICS, "pulse://stream/metrics");
        assert_eq!(STREAM_NAME_LOGS, "pulse://stream/logs");
    }

    #[test]
    fn max_payload_bytes_is_eight_mb() {
        assert_eq!(MAX_PAYLOAD_BYTES, 8 * 1024 * 1024);
    }

    #[test]
    fn broadcast_capacity_is_128() {
        assert_eq!(BROADCAST_CAPACITY, 128);
    }

    #[test]
    fn create_returns_three_independent_senders_each_with_zero_subscribers() {
        let s = create();
        assert_eq!(s.spans.receiver_count(), 0);
        assert_eq!(s.metrics.receiver_count(), 0);
        assert_eq!(s.logs.receiver_count(), 0);
    }

    #[test]
    fn encode_spans_round_trips_via_stream_reader() {
        let batch = one_column_int32_batch(vec![1, 2, 3, 4, 5]);
        let encoded = encode_spans(&batch).expect("encode");
        let reader = StreamReader::try_new(encoded.as_ref(), None).expect("reader");
        let mut out = Vec::new();
        for r in reader {
            out.push(r.expect("record batch"));
        }
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].num_rows(), 5);
    }

    #[test]
    fn encode_metrics_round_trips_via_stream_reader() {
        let schema = Arc::new(Schema::new(vec![Field::new("v", DataType::Int64, false)]));
        let array = Int64Array::from(vec![10_i64, 20]);
        let batch = RecordBatch::try_new(schema, vec![Arc::new(array)]).unwrap();

        let encoded = encode_metrics(&batch).expect("encode");
        let reader = StreamReader::try_new(encoded.as_ref(), None).expect("reader");
        let out: Vec<_> = reader.collect::<Result<_, _>>().expect("collect");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].num_rows(), 2);
    }

    #[test]
    fn encode_logs_round_trips_via_stream_reader() {
        let schema = Arc::new(Schema::new(vec![Field::new("msg", DataType::Utf8, false)]));
        let array = StringArray::from(vec!["a", "b", "c"]);
        let batch = RecordBatch::try_new(schema, vec![Arc::new(array)]).unwrap();

        let encoded = encode_logs(&batch).expect("encode");
        let reader = StreamReader::try_new(encoded.as_ref(), None).expect("reader");
        let out: Vec<_> = reader.collect::<Result<_, _>>().expect("collect");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].num_rows(), 3);
    }

    #[test]
    fn encode_with_cap_returns_size_exceeded_when_over_limit() {
        let batch = one_column_int32_batch(vec![1, 2, 3]);
        let result = encode_with_cap(&batch, 16);
        match result {
            Err(Error::BroadcastSizeCapExceeded { payload_bytes }) => {
                assert!(payload_bytes > 16, "payload_bytes must exceed cap");
            }
            other => panic!("expected BroadcastSizeCapExceeded, got {other:?}"),
        }
    }

    #[test]
    fn encode_with_cap_returns_bytes_under_size_cap() {
        let batch = one_column_int32_batch(vec![1, 2, 3]);
        let bytes = encode_with_cap(&batch, MAX_PAYLOAD_BYTES).expect("encode");
        assert!(bytes.len() <= MAX_PAYLOAD_BYTES);
        assert!(!bytes.is_empty());
    }

    #[test]
    fn send_with_zero_subscribers_returns_send_error_silently_droppable() {
        let s = create();
        let bytes = Bytes::from_static(b"x");
        let result = s.spans.send(bytes);
        assert!(
            result.is_err(),
            "send must return SendError when no subscribers"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn three_subscribers_each_receive_same_bytes() {
        let s = create();
        let mut r1 = s.spans.subscribe();
        let mut r2 = s.spans.subscribe();
        let mut r3 = s.spans.subscribe();
        assert_eq!(s.spans.receiver_count(), 3);

        let payload = Bytes::from_static(b"hello");
        let n = s.spans.send(payload.clone()).expect("send");
        assert_eq!(n, 3);

        assert_eq!(r1.recv().await.expect("r1"), payload);
        assert_eq!(r2.recv().await.expect("r2"), payload);
        assert_eq!(r3.recv().await.expect("r3"), payload);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn lag_detected_when_capacity_overrun() {
        let (sender, mut receiver) = broadcast::channel::<Bytes>(2);
        sender.send(Bytes::from_static(b"a")).expect("a");
        sender.send(Bytes::from_static(b"b")).expect("b");
        sender.send(Bytes::from_static(b"c")).expect("c");
        sender.send(Bytes::from_static(b"d")).expect("d");

        match receiver.recv().await {
            Err(broadcast::error::RecvError::Lagged(n)) => assert!(n >= 1),
            other => panic!("expected Lagged, got {other:?}"),
        }
    }

    #[test]
    fn encode_three_streams_use_same_underlying_codec() {
        let batch = one_column_int32_batch(vec![42]);
        let s1 = encode_spans(&batch).expect("encode_spans");
        let s2 = encode_metrics(&batch).expect("encode_metrics");
        let s3 = encode_logs(&batch).expect("encode_logs");
        assert_eq!(s1, s2);
        assert_eq!(s2, s3);
    }

    #[test]
    fn send_returns_count_equal_to_subscriber_count() {
        let s = create();
        let _r1 = s.metrics.subscribe();
        let _r2 = s.metrics.subscribe();
        let n = s.metrics.send(Bytes::from_static(b"x")).expect("send");
        assert_eq!(n, 2);
    }
}
