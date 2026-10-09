// Unit coverage for the close-to-tray decision seams (P-063): the "still
// running in the tray" notification fires EVERY time the app goes fully to the
// tray (the deliberate widget-close), gated only on the notification
// preference — no first-close latch. The notification body is a static string
// passed solely to the notification builder (never to a tracing macro), so the
// "no notification body in logs" security criterion holds by-construction.

use pulse_app::window::{close_sends_app_to_tray, should_show_close_signpost};

#[test]
fn fires_every_time_when_notifications_enabled() {
    assert!(should_show_close_signpost(true));
}

#[test]
fn suppressed_when_notifications_disabled() {
    assert!(!should_show_close_signpost(false));
}

#[test]
fn close_sends_app_to_tray_only_for_the_widget() {
    // Widget (primary) close → app to tray (hide all + signpost); dashboard
    // (secondary) close → collapse to widget; an unexpected label is secondary.
    assert!(close_sends_app_to_tray("compact-widget"));
    assert!(!close_sends_app_to_tray("main"));
    assert!(!close_sends_app_to_tray("unknown"));
}
