use super::*;
use bevyout_core::manifest::PreparedDayNightProfileSource;
use bevyout_core::time_of_day::{ColorKeyframes, DayNightTimings};

fn profile() -> PreparedDayNightProfile {
    PreparedDayNightProfile {
        climate_form_id: Some(1),
        climate_editor_id: Some("Climate".into()),
        weather_form_id: 2,
        weather_editor_id: Some("Clear".into()),
        timings: DayNightTimings::default(),
        sky_upper: ColorKeyframes::default(),
        sky_lower: ColorKeyframes::default(),
        ambient: ColorKeyframes::default(),
        sunlight: ColorKeyframes::default(),
        source: PreparedDayNightProfileSource::Authoritative,
    }
}

fn cell(behave_like_exterior: bool) -> CellInfo {
    CellInfo {
        form_id: 1,
        editor_id: None,
        name: None,
        interior: true,
        behave_like_exterior,
        ambient_rgba: [0.0; 4],
        directional_rgba: [0.0; 4],
        image_space_form_id: None,
        image_space: None,
        lighting_template_form_id: None,
        lighting_template_flags: 0,
        lighting_template: None,
        raw_lighting: None,
        effective_lighting: None,
        water_form_id: None,
        water_height: None,
        grid: None,
        worldspace_form_id: None,
        day_night_profile: Some(profile()),
        day_night_preview_profile: Some(profile()),
    }
}

#[test]
fn procedural_sky_reaches_authored_upper_and_lower_colors() {
    let pixels = sky_pixels([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(&pixels[0..4], &[255, 0, 0, 255]);
    let last_side_row = ((SKY_SIZE - 1) * SKY_SIZE * 4) as usize;
    assert_eq!(&pixels[last_side_row..last_side_row + 4], &[0, 0, 255, 255]);
}

#[test]
fn cycle_seconds_round_trip_through_timescale() {
    let clock = GameClock {
        hour: 12.0,
        timescale: 1440.0,
    };
    assert_eq!(clock.cycle_seconds(), Some(60.0));
}

#[test]
fn set_hour_normalizes_twenty_four_to_midnight() {
    let mut clock = GameClock::default();
    clock.set_hour(24.0);
    assert_eq!(clock.hour, 0.0);
}

#[test]
fn virtual_pause_stops_only_clock_advancement() {
    let mut clock = GameClock {
        hour: 12.0,
        timescale: 1440.0,
    };
    advance_clock_by(&mut clock, 30.0, true);
    assert_eq!(clock.hour, 12.0);
    advance_clock_by(&mut clock, 30.0, false);
    assert_eq!(clock.hour, 0.0);
}

#[test]
fn preview_preserves_authored_day_luminance_and_weather_variation() {
    let authored = [0.11, 0.12, 0.17, 0.0];
    let weather_day = [0.25, 0.53, 0.62, 0.0];
    let weather_night = [0.28, 0.35, 0.43, 0.0];
    let day = preview_weather_color(weather_day, authored, weather_day);
    let night = preview_weather_color(weather_night, authored, weather_day);

    assert!((relative_luminance(day) - relative_luminance(authored)).abs() < 1e-5);
    assert!(relative_luminance(night) < relative_luminance(day));
}

#[test]
fn preview_does_not_invent_directional_strength_for_a_cell_without_it() {
    let sunlight = preview_weather_color(
        [1.0, 0.8, 0.6, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 0.8, 0.6, 0.0],
    );
    assert_eq!(sunlight[..3], [0.0; 3]);
}

#[test]
fn ordinary_interior_restores_static_when_preview_is_disabled() {
    let ordinary = cell(false);
    assert_eq!(profile_for_cell(&ordinary, false).1, "STATIC");
    assert!(profile_for_cell(&ordinary, false).0.is_none());
    assert_eq!(profile_for_cell(&ordinary, true).1, "PREVIEW");
    assert!(profile_for_cell(&ordinary, true).0.is_some());

    let exterior_like = cell(true);
    assert_eq!(profile_for_cell(&exterior_like, false).1, "FAITHFUL");
    assert!(profile_for_cell(&exterior_like, false).0.is_some());
}

#[test]
fn diagnostic_clock_uses_the_existing_top_right_stack() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_day_night_text);
    app.update();
    let world = app.world_mut();
    let mut query =
        world.query_filtered::<(&Node, Option<&console::DiagnosticUi>), With<DayNightText>>();
    let (node, marker) = query.single(world).unwrap();
    assert_eq!(node.top, px(80));
    assert_eq!(node.right, px(10));
    assert!(marker.is_some());
}
