//! Resolver probe for the engine boot record's leaf (`app.boot.engine`).
//!
//! Lives under `pulse-app/tests/` because `[lib] test = false` means a
//! src-level `mod tests` in `pulse-app` compiles and never runs.
//!
//! The emit site (`pulse_app::engine_boot::emit_boot_record`) is the authority
//! for this field list, so the field names are captured from it rather than
//! restated. No bare `app` key is registered, so without the EXACT leaf
//! `for_target` resolves nothing and every field is redacted.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::ThreadId;

use pulse_app::engine_boot::{Program, SeatKind, TARGET_BOOT_ENGINE, emit_boot_record};
use pulse_app::observability::AllowList;

const TARGET: &str = "app.boot.engine";
const FIELDS: [&str; 3] = ["program", "interpretation", "reason"];

#[test]
fn the_emit_site_target_is_the_registered_one() {
    assert_eq!(TARGET_BOOT_ENGINE, TARGET);
}

#[test]
fn boot_engine_resolves_to_an_exact_leaf_with_exactly_its_fields() {
    let al = AllowList::production();
    let set = al
        .for_target(TARGET)
        .expect("without its own leaf every field is redacted");
    let leaf: BTreeSet<&str> = set.iter().copied().collect();
    let expected: BTreeSet<&str> = FIELDS.into_iter().collect();
    assert_eq!(
        leaf, expected,
        "the leaf must equal {{program, interpretation, reason}}"
    );
}

#[test]
fn boot_engine_has_no_bare_app_fallback() {
    // The discriminator: deleting the exact leaf leaves the target resolving
    // to nothing, so the pin above cannot pass through a fallback set.
    assert!(AllowList::production().for_target("app").is_none());
}

/// One captured event: `(target, field name -> value, level, emitting thread)`.
type Captured = Arc<Mutex<Vec<(String, BTreeMap<String, String>, tracing::Level, ThreadId)>>>;

/// Process-global, not the thread-local `with_default`: under parallel
/// libtest the thread-local form races the callsite interest cache.
fn global_capture() -> Captured {
    static CAPTURE: OnceLock<Captured> = OnceLock::new();
    Arc::clone(CAPTURE.get_or_init(|| {
        let events: Captured = Arc::new(Mutex::new(Vec::new()));
        tracing::subscriber::set_global_default(CapturingSubscriber {
            events: Arc::clone(&events),
        })
        .expect("the capture is the binary's only global subscriber");
        events
    }))
}

struct CapturingSubscriber {
    events: Captured,
}

impl tracing::Subscriber for CapturingSubscriber {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }
    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
    fn event(&self, event: &tracing::Event<'_>) {
        struct Fields(BTreeMap<String, String>);
        impl tracing::field::Visit for Fields {
            fn record_str(&mut self, f: &tracing::field::Field, value: &str) {
                if f.name() != "message" {
                    self.0.insert(f.name().to_string(), value.to_string());
                }
            }
            fn record_debug(&mut self, f: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                if f.name() != "message" {
                    self.0.insert(f.name().to_string(), format!("{value:?}"));
                }
            }
        }
        let mut fields = Fields(BTreeMap::new());
        event.record(&mut fields);
        self.events.lock().expect("lock").push((
            event.metadata().target().to_string(),
            fields.0,
            *event.metadata().level(),
            std::thread::current().id(),
        ));
    }
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}
}

fn emitted(program: Program, seat: Option<SeatKind>) -> (BTreeMap<String, String>, tracing::Level) {
    let events = global_capture();
    let me = std::thread::current().id();
    let before = events.lock().expect("lock").len();
    emit_boot_record(program, seat);
    let mine: Vec<_> = events
        .lock()
        .expect("lock")
        .iter()
        .skip(before)
        .filter(|(target, _, _, thread)| target == TARGET && *thread == me)
        .map(|(_, fields, level, _)| (fields.clone(), *level))
        .collect();
    assert_eq!(mine.len(), 1, "one record per emit: {mine:?}");
    mine.into_iter().next().expect("one record")
}

#[test]
fn every_program_and_seat_emits_exactly_the_leaf_fields_at_its_level() {
    let leaf: BTreeSet<String> = AllowList::production()
        .for_target(TARGET)
        .expect("the leaf resolves")
        .iter()
        .map(|f| f.to_string())
        .collect();
    let info = tracing::Level::INFO;
    let warn = tracing::Level::WARN;
    for (program, seat, level, labels) in [
        (
            Program::Window,
            Some(SeatKind::Model),
            info,
            ["window", "model", "window_runs_model"],
        ),
        (
            Program::Window,
            Some(SeatKind::Deterministic),
            info,
            ["window", "deterministic", "deterministic_gate_set"],
        ),
        (
            Program::Console,
            Some(SeatKind::Deterministic),
            info,
            ["console", "deterministic", "deterministic_gate_set"],
        ),
        (
            Program::Console,
            None,
            warn,
            ["console", "none", "deterministic_gate_unset"],
        ),
    ] {
        let (fields, got) = emitted(program, seat);
        let names: BTreeSet<String> = fields.keys().cloned().collect();
        assert_eq!(
            names, leaf,
            "{program:?} / {seat:?}: emitted fields must equal the leaf"
        );
        assert_eq!(
            got, level,
            "{program:?} / {seat:?}: WARN for an empty seat only"
        );
        assert_eq!(
            [
                fields["program"].as_str(),
                fields["interpretation"].as_str(),
                fields["reason"].as_str(),
            ],
            labels,
            "{program:?} / {seat:?}: the three closed labels",
        );
    }
}
