// Unit coverage for the first-close signpost decision seam (P-063): the
// "still running in the tray" notification fires only on the FIRST close per
// session AND only when notifications are enabled. The notification body is a
// static string passed solely to the notification builder (never to a tracing
// macro), so the "no notification body in logs" security criterion holds
// by-construction — there is no log path that carries it.

use pulse_app::window::should_show_close_signpost;

#[test]
fn shows_on_first_close_when_notifications_enabled() {
    assert!(should_show_close_signpost(true, false));
}

#[test]
fn suppressed_after_first_close() {
    assert!(!should_show_close_signpost(true, true));
}

#[test]
fn suppressed_when_notifications_disabled() {
    assert!(!should_show_close_signpost(false, false));
}

#[test]
fn suppressed_when_disabled_and_already_shown() {
    assert!(!should_show_close_signpost(false, true));
}
