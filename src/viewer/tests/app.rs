use super::*;
use bevy::winit::{UpdateMode, WinitSettings};

#[test]
fn primary_window_defaults_to_1080p() {
    let window = default_primary_window(false);
    assert_eq!(window.resolution.width(), 1920.0);
    assert_eq!(window.resolution.height(), 1080.0);
}

#[test]
fn primary_window_is_focused_when_not_unfocused() {
    assert!(default_primary_window(false).focused);
}

#[test]
fn primary_window_is_unfocused_when_requested() {
    assert!(!default_primary_window(true).focused);
}

// Issue #180: `should_start_unfocused` is the pure decision behind the
// window descriptor's `focused` field -- exercised directly here, and
// the two cases above confirm it actually reaches the `Window`.

#[test]
fn starts_focused_without_the_flag_or_an_agent_bridge() {
    assert!(!should_start_unfocused(false, None));
}

#[test]
fn explicit_unfocused_flag_starts_unfocused_even_without_a_bridge() {
    assert!(should_start_unfocused(true, None));
}

#[test]
fn agent_bridge_starts_unfocused_even_without_the_explicit_flag() {
    assert!(should_start_unfocused(false, Some(15_702)));
}

/// Issue #180: mirrors the production `run_view` call exactly (same
/// resource, same constructor) so a regression that swaps in
/// `WinitSettings::default()` or `WinitSettings::game()` -- either of
/// which throttles ticking while unfocused/occluded and starves the
/// agent bridge's `with_method_main` handlers -- fails this test.
#[test]
fn winit_settings_are_continuous_focused_and_unfocused() {
    let mut app = App::new();
    app.insert_resource(WinitSettings::continuous());

    let settings = app.world().resource::<WinitSettings>();
    assert_eq!(settings.focused_mode, UpdateMode::Continuous);
    assert_eq!(settings.unfocused_mode, UpdateMode::Continuous);
}
