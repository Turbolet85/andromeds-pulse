//! Resolver probe for the launch render posture leaf (`app.boot.render.posture`).
//!
//! Lives under `pulse-app/tests/` because `[lib] test = false` means a
//! src-level `mod tests` in `pulse-app` compiles and never runs.
//!
//! The emit site (`pulse_app::render_posture::emit_posture`) is the authority
//! for this field list, so the field names are captured from it rather than
//! restated. No bare `app` key is registered, so without the EXACT leaf
//! `for_target` resolves nothing and every field is redacted.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::ThreadId;

use pulse_app::observability::AllowList;
use pulse_app::render_posture::{RenderPosture, emit_posture};

const TARGET: &str = "app.boot.render.posture";
const FIELDS: [&str; 2] = ["posture", "lever"];

#[test]
fn render_posture_resolves_to_an_exact_leaf_with_exactly_its_fields() {
    let al = AllowList::production();
    let set = al
        .for_target(TARGET)
        .expect("without its own leaf every field is redacted");
    let leaf: BTreeSet<&str> = set.iter().copied().collect();
    let expected: BTreeSet<&str> = FIELDS.into_iter().collect();
    assert_eq!(leaf, expected, "the leaf must equal {{posture, lever}}");
}

#[test]
fn render_posture_has_no_bare_app_fallback() {
    // The discriminator: deleting the exact leaf leaves the target resolving
    // to nothing, so the pin above cannot pass through a fallback set.
    assert!(AllowList::production().for_target("app").is_none());
}

/// One captured event: `(target, field names, level, emitting thread)`.
type Captured = Arc<Mutex<Vec<(String, BTreeSet<String>, tracing::Level, ThreadId)>>>;

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
        struct Names(BTreeSet<String>);
        impl tracing::field::Visit for Names {
            fn record_debug(&mut self, f: &tracing::field::Field, _: &dyn std::fmt::Debug) {
                if f.name() != "message" {
                    self.0.insert(f.name().to_string());
                }
            }
        }
        let mut names = Names(BTreeSet::new());
        event.record(&mut names);
        self.events.lock().expect("lock").push((
            event.metadata().target().to_string(),
            names.0,
            *event.metadata().level(),
            std::thread::current().id(),
        ));
    }
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}
}

fn emitted(posture: RenderPosture) -> (BTreeSet<String>, tracing::Level) {
    let events = global_capture();
    let me = std::thread::current().id();
    let before = events.lock().expect("lock").len();
    emit_posture(posture);
    let mine: Vec<_> = events
        .lock()
        .expect("lock")
        .iter()
        .skip(before)
        .filter(|(target, _, _, thread)| target == TARGET && *thread == me)
        .map(|(_, names, level, _)| (names.clone(), *level))
        .collect();
    assert_eq!(mine.len(), 1, "one record per emit: {mine:?}");
    mine.into_iter().next().expect("one record")
}

#[test]
fn every_posture_emits_exactly_the_leaf_fields_at_its_level() {
    let leaf: BTreeSet<String> = AllowList::production()
        .for_target(TARGET)
        .expect("the leaf resolves")
        .iter()
        .map(|f| f.to_string())
        .collect();
    for (posture, level) in [
        (RenderPosture::Applied, tracing::Level::INFO),
        (RenderPosture::PresetHonoured, tracing::Level::WARN),
        (RenderPosture::NotApplicable, tracing::Level::INFO),
    ] {
        let (names, got) = emitted(posture);
        assert_eq!(
            names, leaf,
            "{posture:?}: emitted fields must equal the leaf"
        );
        assert_eq!(got, level, "{posture:?}: WARN for preset_honoured only");
    }
}
