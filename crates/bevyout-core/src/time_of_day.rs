//! Pure Fallout time-of-day clock and weather-color interpolation policy.

use serde::{Deserialize, Serialize};

pub const HOURS_PER_DAY: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DayNightTimings {
    pub sunrise_begin_hour: f32,
    pub sunrise_end_hour: f32,
    pub sunset_begin_hour: f32,
    pub sunset_end_hour: f32,
}

impl Default for DayNightTimings {
    fn default() -> Self {
        Self {
            sunrise_begin_hour: 5.0,
            sunrise_end_hour: 7.0,
            sunset_begin_hour: 17.0,
            sunset_end_hour: 19.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorKeyframes {
    pub sunrise: [f32; 4],
    pub day: [f32; 4],
    pub sunset: [f32; 4],
    pub night: [f32; 4],
}

impl Default for ColorKeyframes {
    fn default() -> Self {
        Self {
            sunrise: [0.0, 0.0, 0.0, 1.0],
            day: [0.0, 0.0, 0.0, 1.0],
            sunset: [0.0, 0.0, 0.0, 1.0],
            night: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

pub fn normalize_hour(hour: f32) -> f32 {
    if !hour.is_finite() {
        return 0.0;
    }
    hour.rem_euclid(HOURS_PER_DAY)
}

pub fn advance_game_hour(current_hour: f32, timescale: f32, real_delta_seconds: f32) -> f32 {
    let timescale = finite_nonnegative(timescale);
    let real_delta_seconds = finite_nonnegative(real_delta_seconds);
    normalize_hour(normalize_hour(current_hour) + timescale * real_delta_seconds / 3600.0)
}

pub fn uses_dynamic_lighting(
    interior: bool,
    behave_like_exterior: bool,
    preview_override: bool,
) -> bool {
    !interior || behave_like_exterior || preview_override
}

pub fn select_preview_weather(candidates: &[(u32, Option<String>)]) -> Option<u32> {
    candidates
        .iter()
        .filter(|(_, editor_id)| {
            editor_id
                .as_deref()
                .is_some_and(|value| value.eq_ignore_ascii_case("WastelandClear"))
        })
        .map(|(form_id, _)| *form_id)
        .min()
        .or_else(|| candidates.iter().map(|(form_id, _)| *form_id).min())
}

pub fn interpolate_keyframes(
    keyframes: ColorKeyframes,
    timings: DayNightTimings,
    hour: f32,
) -> [f32; 4] {
    let keyframes = finite_keyframes(keyframes);
    let hour = normalize_hour(hour);
    if let Some(progress) =
        cyclic_window_progress(hour, timings.sunrise_begin_hour, timings.sunrise_end_hour)
    {
        return midpoint_lerp(keyframes.night, keyframes.sunrise, keyframes.day, progress);
    }
    if let Some(progress) =
        cyclic_window_progress(hour, timings.sunset_begin_hour, timings.sunset_end_hour)
    {
        return midpoint_lerp(keyframes.day, keyframes.sunset, keyframes.night, progress);
    }

    if cyclic_window_progress(hour, timings.sunrise_end_hour, timings.sunset_begin_hour).is_some() {
        keyframes.day
    } else {
        keyframes.night
    }
}

fn cyclic_window_progress(hour: f32, begin: f32, end: f32) -> Option<f32> {
    let begin = normalize_hour(begin);
    let duration = (normalize_hour(end) - begin).rem_euclid(HOURS_PER_DAY);
    if duration <= f32::EPSILON {
        return None;
    }
    let elapsed = (hour - begin).rem_euclid(HOURS_PER_DAY);
    (elapsed <= duration).then_some((elapsed / duration).clamp(0.0, 1.0))
}

fn midpoint_lerp(begin: [f32; 4], midpoint: [f32; 4], end: [f32; 4], progress: f32) -> [f32; 4] {
    if progress <= 0.5 {
        lerp_rgba(begin, midpoint, progress * 2.0)
    } else {
        lerp_rgba(midpoint, end, (progress - 0.5) * 2.0)
    }
}

fn lerp_rgba(from: [f32; 4], to: [f32; 4], amount: f32) -> [f32; 4] {
    let amount = if amount.is_finite() {
        amount.clamp(0.0, 1.0)
    } else {
        0.0
    };
    std::array::from_fn(|index| {
        finite_or_zero(from[index])
            + (finite_or_zero(to[index]) - finite_or_zero(from[index])) * amount
    })
}

fn finite_nonnegative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

fn finite_color(color: [f32; 4]) -> [f32; 4] {
    std::array::from_fn(|index| finite_or_zero(color[index]))
}

fn finite_keyframes(keyframes: ColorKeyframes) -> ColorKeyframes {
    ColorKeyframes {
        sunrise: finite_color(keyframes.sunrise),
        day: finite_color(keyframes.day),
        sunset: finite_color(keyframes.sunset),
        night: finite_color(keyframes.night),
    }
}

#[cfg(test)]
#[path = "tests/time_of_day.rs"]
mod tests;
