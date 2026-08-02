use super::*;

fn scalar_keys() -> ColorKeyframes {
    ColorKeyframes {
        sunrise: [1.0; 4],
        day: [2.0; 4],
        sunset: [3.0; 4],
        night: [0.0; 4],
    }
}

#[test]
fn full_day_at_timescale_1440_takes_one_real_minute() {
    assert_eq!(advance_game_hour(12.0, 1440.0, 60.0), 12.0);
    assert_eq!(advance_game_hour(12.0, 1440.0, 30.0), 0.0);
}

#[test]
fn authored_keys_are_reached_at_window_midpoints() {
    let timings = DayNightTimings::default();
    assert_eq!(interpolate_keyframes(scalar_keys(), timings, 6.0), [1.0; 4]);
    assert_eq!(
        interpolate_keyframes(scalar_keys(), timings, 18.0),
        [3.0; 4]
    );
}

#[test]
fn preview_weather_prefers_clear_then_lowest_form_id() {
    let candidates = vec![
        (1, Some("Cloudy".into())),
        (50, Some("wastelandclear".into())),
        (2, Some("Rain".into())),
    ];
    assert_eq!(select_preview_weather(&candidates), Some(50));
    assert_eq!(select_preview_weather(&candidates[..1]), Some(1));
}

#[test]
fn invalid_clock_inputs_resolve_to_finite_deterministic_hours() {
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(normalize_hour(value), 0.0);
    }
    for inputs in [
        (f32::NAN, 1.0, 1.0),
        (f32::INFINITY, 1.0, 1.0),
        (12.0, f32::INFINITY, 1.0),
        (12.0, 1.0, f32::NAN),
    ] {
        let first = advance_game_hour(inputs.0, inputs.1, inputs.2);
        let second = advance_game_hour(inputs.0, inputs.1, inputs.2);
        assert!(first.is_finite());
        assert_eq!(first, second);
    }
}

#[test]
fn keyframe_interpolation_is_finite_and_reproducible_for_invalid_inputs() {
    let keyframes = ColorKeyframes {
        sunrise: [f32::NAN, 0.1, 0.2, 1.0],
        day: [f32::INFINITY, 0.3, 0.4, 1.0],
        sunset: [f32::NEG_INFINITY, 0.5, 0.6, 1.0],
        night: [0.0, 0.7, 0.8, 1.0],
    };
    let timings = DayNightTimings {
        sunrise_begin_hour: f32::NAN,
        sunrise_end_hour: f32::INFINITY,
        sunset_begin_hour: 17.0,
        sunset_end_hour: f32::NEG_INFINITY,
    };

    let first = interpolate_keyframes(keyframes, timings, f32::NAN);
    let second = interpolate_keyframes(keyframes, timings, f32::NAN);
    assert!(first.iter().all(|channel| channel.is_finite()));
    assert_eq!(first, second);
}
