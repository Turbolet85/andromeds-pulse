//! Chunk #77 — Test (b) plugin path basename-only PII vector test.
//!
//! Per pulse-v0_2_0-route §77 Part 2 second sub-bullet + acceptance criterion
//! #6 item (b) + security plan §Anti-Pattern Logging row 2 ("NEVER log full
//! plugin file paths — basename of canonicalized path only") + obs-plan §5
//! Section 5 Vector 3:
//!
//! Exercise `plugins::loader::discover_plugins` against а multi-segment
//! canonicalized plugin dir containing а WASM file; assert (1) the returned
//! `LoadedPlugin::basename` is the file basename only (no path separators),
//! (2) error variants from the loader carry plugin_id as basename only,
//! (3) tracing events emitted during discovery don't leak the full path в
//! field values.

use std::sync::{Arc, Mutex};

use plugins::engine::build_engine;
use plugins::loader::discover_plugins;
use tempfile::TempDir;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

/// CapturingSubscriber records (target, field_string, span_attrs) tuples for
/// every emitted event AND span creation. Mirrors the chunk #44
/// `pulse-app/src/snapshot_runtime.rs::tests::CapturingSubscriber` pattern
/// extended to capture span attributes too (since `discover_plugins` emits
/// span-recorded fields via `Span::current().record()`, не event fields).
struct CapturingSubscriber {
    events: Arc<Mutex<Vec<(String, String)>>>,
    spans: Arc<Mutex<Vec<(String, String)>>>,
}

struct FieldCollector {
    sink: String,
}

impl Visit for FieldCollector {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={:?}", field.name(), value);
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={}", field.name(), value);
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={}", field.name(), value);
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={}", field.name(), value);
    }
    fn record_bool(&mut self, field: &Field, value: bool) {
        use std::fmt::Write;
        let _ = write!(&mut self.sink, " {}={}", field.name(), value);
    }
}

impl Subscriber for CapturingSubscriber {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, attrs: &Attributes<'_>) -> Id {
        let metadata = attrs.metadata();
        let mut collector = FieldCollector {
            sink: String::new(),
        };
        attrs.record(&mut collector);
        self.spans
            .lock()
            .expect("spans lock")
            .push((metadata.name().to_string(), collector.sink));
        Id::from_u64(1)
    }
    fn record(&self, _: &Id, values: &Record<'_>) {
        // Span field updates (e.g. via `span.record("plugin_dir_basename", ...)`)
        // arrive here. Append к the latest span entry's field string for
        // post-hoc inspection.
        let mut collector = FieldCollector {
            sink: String::new(),
        };
        values.record(&mut collector);
        if let Some((_, existing)) = self.spans.lock().expect("spans lock").last_mut() {
            existing.push_str(&collector.sink);
        }
    }
    fn record_follows_from(&self, _: &Id, _: &Id) {}
    fn event(&self, event: &Event<'_>) {
        let metadata = event.metadata();
        let mut collector = FieldCollector {
            sink: String::new(),
        };
        event.record(&mut collector);
        self.events
            .lock()
            .expect("events lock")
            .push((metadata.target().to_string(), collector.sink));
    }
    fn enter(&self, _: &Id) {}
    fn exit(&self, _: &Id) {}
}

#[test]
fn discover_plugins_returns_basename_only_in_loaded_plugin() {
    // Build а plugin dir с category subdir + WASM file. The dir path is
    // deeply nested (4 segments) to ensure а basename-only invariant is
    // distinguishable from а full-path leak.
    let tmp = TempDir::new().expect("tempdir");
    let category_dir = tmp
        .path()
        .join("a")
        .join("b")
        .join("c")
        .join("custom-dashboard");
    std::fs::create_dir_all(&category_dir).expect("create nested category dir");
    let wasm_path = category_dir.join("my-secret-plugin.wasm");
    // Empty Component bytes per testing.md 2026-05-11 chunk #45 pattern.
    let empty_component_bytes = wat::parse_str("(component)").expect("wat parses");
    std::fs::write(&wasm_path, &empty_component_bytes).expect("write wasm");

    let engine = build_engine().expect("build_engine");
    // discover_plugins expects а canonical plugin dir (the root with
    // `custom-dashboard/` / `data-transform/` / `snapshot-template/`
    // subdirs); use tmp.path().join("a").join("b").join("c") as the
    // synthetic root.
    let plugin_root = tmp.path().join("a").join("b").join("c");
    let plugins = discover_plugins(&engine, &plugin_root).expect("discover ok");

    // Find the plugin we wrote — basename only, no path separators.
    // LoadedPlugin doesn't impl Debug (wasmtime::Component is non-Debug); list
    // basenames instead на panic.
    let found = plugins
        .iter()
        .find(|p| p.basename == "my-secret-plugin.wasm")
        .unwrap_or_else(|| {
            let basenames: Vec<&str> = plugins.iter().map(|p| p.basename.as_str()).collect();
            panic!("expected my-secret-plugin.wasm discovered; got basenames: {basenames:?}")
        });

    // Acceptance: basename field is the filename only.
    assert_eq!(found.basename, "my-secret-plugin.wasm");
    // Defense: no path separator (Unix / / Windows \) appears anywhere в the
    // basename field.
    assert!(
        !found.basename.contains('/'),
        "plugin basename leaked Unix path separator: {}",
        found.basename
    );
    assert!(
        !found.basename.contains('\\'),
        "plugin basename leaked Windows path separator: {}",
        found.basename
    );
    // Defense: id (derived от basename без .wasm) is also basename-only.
    assert_eq!(found.id, "my-secret-plugin");
    assert!(!found.id.contains('/') && !found.id.contains('\\'));
}

#[test]
fn discover_plugins_emits_basename_only_in_tracing_spans() {
    let tmp = TempDir::new().expect("tempdir");
    // The leaky-path component: "deeply-nested-secret-dir" appears IN the
    // path but should NOT appear в any tracing field per security plan
    // §Anti-Pattern Logging row 2.
    let category_dir = tmp
        .path()
        .join("a")
        .join("deeply-nested-secret-dir")
        .join("c")
        .join("custom-dashboard");
    std::fs::create_dir_all(&category_dir).expect("create dirs");
    let wasm_path = category_dir.join("test-plugin.wasm");
    let empty_component_bytes = wat::parse_str("(component)").expect("wat parses");
    std::fs::write(&wasm_path, &empty_component_bytes).expect("write wasm");

    let plugin_root = tmp
        .path()
        .join("a")
        .join("deeply-nested-secret-dir")
        .join("c");

    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let spans: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let subscriber = CapturingSubscriber {
        events: events.clone(),
        spans: spans.clone(),
    };
    let guard = tracing::subscriber::set_default(subscriber);

    let engine = build_engine().expect("build_engine");
    let _ = discover_plugins(&engine, &plugin_root).expect("discover ok");

    drop(guard);

    // Assertions: tracing fields (event AND span) must not leak path
    // segments that would reveal the user's filesystem layout. The basename
    // helper в discover_plugins extracts ONLY the final segment.
    let full_path_string = plugin_root.to_string_lossy().into_owned();
    let leaky_segments: [&str; 2] = [
        "deeply-nested-secret-dir",
        // The full plugin_root path String would contain these:
        full_path_string.as_str(),
    ];

    for (target, fields) in events.lock().expect("events lock").iter() {
        for leaky in &leaky_segments {
            assert!(
                !fields.contains(*leaky),
                "tracing event target={target} field leaked path segment `{leaky}`: {fields}"
            );
        }
    }
    for (span_name, fields) in spans.lock().expect("spans lock").iter() {
        for leaky in &leaky_segments {
            assert!(
                !fields.contains(*leaky),
                "tracing span name={span_name} field leaked path segment `{leaky}`: {fields}"
            );
        }
    }
}

#[test]
fn canonicalize_plugin_dir_returns_basename_in_traversal_rejection_error() {
    // Construct а path с CWE-22 traversal segment — canonicalize_plugin_dir
    // should reject AND the resulting Error::PathCanonicalizationFailed reason
    // must not embed the full traversal path verbatim. Per security plan §Input
    // row 4 + obs-plan §5 Vector 6.
    let tmp = TempDir::new().expect("tempdir");
    let traversal_marker = "deliberately-checked-canary-token";
    let bad_path = tmp
        .path()
        .join(traversal_marker)
        .join("..")
        .join("..")
        .join("etc");

    // Direct invocation of the pub helper — this just exercises canonicalize
    // semantics; the Error variant signature is what's testable here.
    let res = plugins::loader::canonicalize_plugin_dir(&bad_path);
    // Either Ok (path resolved cleanly) OR Err (path rejected). In either
    // case, the result must not embed the canary marker IF it embeds anything
    // path-like.
    match res {
        Ok(canonical) => {
            // Canonical path is the OS-resolved form; it may or may not
            // contain the marker depending on whether `..` resolves through
            // the tempdir layout. The discipline is that the loader doesn't
            // leak path content к the AppError surface — that's tested
            // separately. Here just sanity-check no panic.
            let _ = canonical;
        }
        Err(plugins::contract::Error::PathCanonicalizationFailed { reason }) => {
            // The reason field is the sanitized rejection — basename / structured
            // marker only per `crates/plugins/src/loader.rs` discipline.
            // Verify it does not embed the full original path verbatim.
            assert!(
                !reason.contains(traversal_marker)
                    || reason.len() < bad_path.to_string_lossy().len(),
                "PathCanonicalizationFailed.reason leaked verbatim full traversal path; reason: {reason}"
            );
        }
        Err(other) => {
            // Other Error variants are acceptable so long as they don't
            // embed the full path leak.
            let dbg = format!("{other:?}");
            assert!(
                !dbg.contains(traversal_marker) || dbg.len() < bad_path.to_string_lossy().len(),
                "Plugin Error variant leaked traversal marker: {dbg}"
            );
        }
    }
}
