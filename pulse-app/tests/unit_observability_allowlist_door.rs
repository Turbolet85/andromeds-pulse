//! Resolver probes for the door's three leaves (`app.boot.door`, `door.call`,
//! `door.refused`).
//!
//! Lives under `pulse-app/tests/` because `[lib] test = false` means a
//! src-level `mod tests` in `pulse-app` compiles and never runs.
//!
//! The emit sites (`pulse_app::door::emit_boot`, `emit_call`, `emit_refused`)
//! are the authority for the field lists, so the field names are captured from
//! them rather than restated. No bare `door` key and no bare `app` key is
//! registered, so a target without its own exact leaf resolves nothing and
//! every field of it is redacted.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::ThreadId;

use pulse_app::door::{
    CallRecord, EndpointState, Outcome, Peer, Refusal, ResolveWrite, TARGET_BOOT_DOOR,
    TARGET_DOOR_CALL, TARGET_DOOR_REFUSED, emit_boot, emit_call, emit_refused,
};
use pulse_app::observability::AllowList;

const BOOT: &str = "app.boot.door";
const CALL: &str = "door.call";
const REFUSED: &str = "door.refused";

const STATES: [EndpointState; 5] = [
    EndpointState::Listening,
    EndpointState::PathTooLong,
    EndpointState::InUse,
    EndpointState::Occupied,
    EndpointState::BindFailed,
];
const SWITCHES: [ResolveWrite; 3] = [ResolveWrite::On, ResolveWrite::Off, ResolveWrite::Rejected];
const OUTCOMES: [Outcome; 9] = [
    Outcome::Ok,
    Outcome::ToolError,
    Outcome::InvalidArguments,
    Outcome::UnknownTool,
    Outcome::UnknownMethod,
    Outcome::ParseError,
    Outcome::WriteOff,
    Outcome::TimedOut,
    Outcome::AnswerTooLarge,
];
const REFUSALS: [Refusal; 4] = [
    Refusal::OtherAccount,
    Refusal::TooManyConnections,
    Refusal::RequestTooLarge,
    Refusal::Idle,
];

fn leaf(target: &str) -> BTreeSet<String> {
    AllowList::production()
        .for_target(target)
        .unwrap_or_else(|| {
            panic!("`{target}` resolves: without its own leaf every field is redacted")
        })
        .iter()
        .map(|field| field.to_string())
        .collect()
}

#[test]
fn the_emit_site_targets_are_the_registered_ones() {
    assert_eq!(TARGET_BOOT_DOOR, BOOT);
    assert_eq!(TARGET_DOOR_CALL, CALL);
    assert_eq!(TARGET_DOOR_REFUSED, REFUSED);
}

#[test]
fn each_door_target_resolves_and_no_fallback_key_could_resolve_it() {
    let al = AllowList::production();
    for target in [BOOT, CALL, REFUSED] {
        assert!(
            al.for_target(target).is_some(),
            "`{target}` resolves to a leaf"
        );
    }
    // The discriminators: with no bare `door` and no bare `app` key, the three
    // resolve above through their own exact leaves and through nothing else,
    // so deleting a leaf leaves its target resolving to nothing.
    assert!(al.for_target("door").is_none());
    assert!(al.for_target("app").is_none());
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

/// The one record of `target` that `emit` wrote on this thread.
fn emitted(target: &str, emit: impl FnOnce()) -> (BTreeMap<String, String>, tracing::Level) {
    let events = global_capture();
    let me = std::thread::current().id();
    let before = events.lock().expect("lock").len();
    emit();
    let mine: Vec<_> = events
        .lock()
        .expect("lock")
        .iter()
        .skip(before)
        .filter(|(seen, _, _, thread)| seen == target && *thread == me)
        .map(|(_, fields, level, _)| (fields.clone(), *level))
        .collect();
    assert_eq!(mine.len(), 1, "one `{target}` record per emit: {mine:?}");
    mine.into_iter().next().expect("one record")
}

fn names(fields: &BTreeMap<String, String>) -> BTreeSet<String> {
    fields.keys().cloned().collect()
}

#[test]
fn the_boot_record_emits_exactly_its_leaf_and_is_info_only_when_listening_with_the_switch_read() {
    let leaf = leaf(BOOT);
    for state in STATES {
        for switch in SWITCHES {
            let (fields, level) = emitted(BOOT, || emit_boot(state, switch));
            assert_eq!(
                names(&fields),
                leaf,
                "{state:?} / {switch:?}: emitted fields must equal the leaf"
            );
            assert_eq!(fields["state"], state.label());
            assert_eq!(fields["resolve_write"], switch.label());
            let info = state == EndpointState::Listening && switch != ResolveWrite::Rejected;
            assert_eq!(
                level,
                if info {
                    tracing::Level::INFO
                } else {
                    tracing::Level::WARN
                },
                "{state:?} / {switch:?}"
            );
        }
    }
}

#[test]
fn the_call_record_emits_exactly_its_leaf_and_is_info_only_for_ok() {
    let leaf = leaf(CALL);
    for outcome in OUTCOMES {
        let record = CallRecord {
            method: "tools/call",
            tool: "query_traces",
            outcome,
            write: false,
            result_type: "paginated_traces",
            result_count: 3,
            answer_bytes: 512,
            duration_ms: 7,
        };
        let (fields, level) = emitted(CALL, || emit_call(record));
        assert_eq!(
            names(&fields),
            leaf,
            "{outcome:?}: emitted fields must equal the leaf"
        );
        assert_eq!(fields["outcome"], outcome.label());
        assert_eq!(fields["write"], "false");
        assert_eq!(fields["result_count"], "3");
        assert_eq!(
            level,
            if outcome == Outcome::Ok {
                tracing::Level::INFO
            } else {
                tracing::Level::WARN
            },
            "{outcome:?}"
        );
    }
}

#[test]
fn the_refusal_record_emits_exactly_its_leaf_at_warn() {
    let leaf = leaf(REFUSED);
    for reason in REFUSALS {
        for peer in [Peer::Owner, Peer::Other] {
            let (fields, level) = emitted(REFUSED, || emit_refused(reason, peer));
            assert_eq!(
                names(&fields),
                leaf,
                "{reason:?} / {peer:?}: emitted fields must equal the leaf"
            );
            assert_eq!(fields["reason"], reason.label());
            assert_eq!(fields["peer"], peer.label());
            assert_eq!(level, tracing::Level::WARN, "{reason:?} / {peer:?}");
        }
    }
}

#[test]
fn no_door_record_is_written_at_error_and_every_label_is_closed() {
    let closed = |label: &str| {
        !label.is_empty() && label.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
    };
    for label in STATES
        .map(EndpointState::label)
        .into_iter()
        .chain(SWITCHES.map(ResolveWrite::label))
        .chain(OUTCOMES.map(Outcome::label))
        .chain(REFUSALS.map(Refusal::label))
        .chain([Peer::Owner.label(), Peer::Other.label()])
    {
        assert!(closed(label), "`{label}` is a lower-case label");
    }
    let (_, worst_boot) = emitted(BOOT, || {
        emit_boot(EndpointState::BindFailed, ResolveWrite::Rejected)
    });
    assert_eq!(worst_boot, tracing::Level::WARN);
}
