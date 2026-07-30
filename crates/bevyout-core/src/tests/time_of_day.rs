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
