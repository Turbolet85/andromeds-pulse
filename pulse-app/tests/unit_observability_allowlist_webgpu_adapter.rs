//! Resolver + field-set pins for the webview adapter-outcome leaf
//! (`ui.webgpu.adapter`).
//!
//! Lives here rather than in `observability.rs`'s own `mod tests` because
//! `[lib] test = false` (the Windows WebView2 workaround) means src-level
//! tests in `pulse-app` compile but never run.
//!
//! The emit site (`crates/ui-bridge/src/telemetry.rs::record_webgpu_adapter`)
//! is the authority for this field list. NO bare `ui` key is registered, so
//! without its EXACT leaf every field is silently redacted — and a frame-less
//! run's cause, which `perf:budget` reads from `fields.outcome`, would vanish.

use std::io::Write;
use std::sync::{Arc, Mutex};

use pulse_app::observability::{AllowList, DefaultFields, JsonWithDefaults, SERVICE_NAME};
use serde_json::Value;
use tracing_subscriber::Registry;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;

const ADAPTER: &str = "ui.webgpu.adapter";
const FIELDS: [&str; 2] = ["outcome", "window_label"];

#[derive(Clone)]
struct VecMakeWriter(Arc<Mutex<Vec<u8>>>);

impl<'a> fmt::MakeWriter<'a> for VecMakeWriter {
    type Writer = VecWriter;
    fn make_writer(&'a self) -> Self::Writer {
        VecWriter(Arc::clone(&self.0))
    }
}

struct VecWriter(Arc<Mutex<Vec<u8>>>);

impl Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().expect("buffer lock").extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn capture_json_lines<F: FnOnce()>(f: F) -> Vec<Value> {
    let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
    let layer = fmt::layer()
        .event_format(JsonWithDefaults {
            defaults: DefaultFields {
                service_name: SERVICE_NAME,
                service_version: env!("CARGO_PKG_VERSION"),
                deployment_environment: "production".into(),
                ci_run_id: None,
                git_commit_sha: None,
            },
            allowlist: AllowList::production(),
        })
        .with_writer(VecMakeWriter(Arc::clone(&buf)));
    tracing::subscriber::with_default(Registry::default().with(layer), f);
    let bytes = buf.lock().expect("buffer lock").clone();
    String::from_utf8(bytes)
        .expect("captured bytes are UTF-8")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).expect("each line parses as JSON"))
        .collect()
}

#[test]
fn webgpu_adapter_leaf_carries_exactly_the_emitted_field_set() {
    let al = AllowList::production();
    let set = al
        .for_target(ADAPTER)
        .expect("ui.webgpu.adapter must resolve to an exact allowlist leaf");
    for field in FIELDS {
        assert!(
            set.contains(field),
            "`{field}` is emitted at {ADAPTER} and must not be redacted"
        );
    }
    assert_eq!(
        set.len(),
        FIELDS.len(),
        "the {ADAPTER} leaf must enumerate exactly the emit site's field list"
    );
}

#[test]
fn webgpu_adapter_record_keeps_its_fields_and_redacts_any_other() {
    let lines = capture_json_lines(|| {
        tracing::warn!(
            target: "ui.webgpu.adapter",
            outcome = "adapter_null",
            window_label = "compact-widget",
            error_msg = "GPUAdapter lost: driver reset",
            "webgpu adapter request recorded",
        );
    });
    let fields = &lines[0]["fields"];
    assert_eq!(fields["outcome"], "adapter_null");
    assert_eq!(fields["window_label"], "compact-widget");
    assert_eq!(
        fields["error_msg"], "<redacted>",
        "a field the emit site never sends must be redacted at {ADAPTER}"
    );
}

#[test]
fn webgpu_adapter_fields_are_not_served_by_a_ui_fallback() {
    let al = AllowList::production();
    assert!(
        al.for_target("ui").is_none(),
        "a bare `ui` allowlist key must not exist"
    );
    assert!(
        al.for_target("ui.webgpu.some_unregistered_sibling")
            .is_none(),
        "an unregistered ui.webgpu.* target must resolve to nothing"
    );
}
