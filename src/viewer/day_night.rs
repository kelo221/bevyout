//! Fallout game clock, climate lighting, procedural sky, and diagnostic HUD.

use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
};
use bevyout_core::manifest::exterior::{PreparedWeatherProfile, resolve_prepared_weather_profile};
use bevyout_core::manifest::{CellInfo, PreparedDayNightProfile, PreparedDayNightProfileSource};
use bevyout_core::time_of_day::{
    advance_game_hour, interpolate_keyframes, normalize_hour, uses_dynamic_lighting,
};
use serde::Serialize;

use super::controls::{AmbientScale, LightingScale};
use super::scene::{
    CellDirectionalLight, refresh_environment_for_active_cell, runtime_lightmapped_diffuse_enabled,
    scaled_directional_illuminance,
};
use super::{LoadedSceneManifest, console, plugins::ViewerSet};

const SKY_SIZE: u32 = 32;
const SKY_UPDATE_INTERVAL_SECONDS: f32 = 0.1;
const SKY_BRIGHTNESS: f32 = 250.0;

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub(crate) struct GameClock {
    pub(crate) hour: f32,
    pub(crate) timescale: f32,
}

impl Default for GameClock {
    fn default() -> Self {
        Self {
            hour: 12.0,
            timescale: 0.0,
        }
    }
}

impl GameClock {
    pub(crate) fn set_hour(&mut self, hour: f32) {
        self.hour = normalize_hour(hour);
    }

    pub(crate) fn cycle_seconds(self) -> Option<f32> {
        (self.timescale > 0.0).then_some(86_400.0 / self.timescale)
    }
}

#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct DayNightPreview(pub(crate) bool);

#[derive(Resource, Clone, Copy, Debug, PartialEq, Serialize)]
pub(crate) struct WeatherTransition {
    pub(crate) source_weather_form_id: Option<u32>,
    pub(crate) target_weather_form_id: Option<u32>,
    pub(crate) elapsed_seconds: f32,
    pub(crate) duration_seconds: f32,
}

impl Default for WeatherTransition {
    fn default() -> Self {
        Self {
            source_weather_form_id: None,
            target_weather_form_id: None,
            elapsed_seconds: 0.0,
            duration_seconds: 0.0,
        }
    }
}

#[derive(Resource, Default)]
struct DayNightRuntime {
    last_cell_form_id: Option<u32>,
    last_preview: bool,
    dynamic_applied: bool,
    sky_elapsed: f32,
    sky_image: Option<Handle<Image>>,
}

#[derive(Component)]
pub(crate) struct DayNightText;

pub(crate) struct DayNightPlugin {
    pub(crate) cycle_seconds: Option<f32>,
}

impl Plugin for DayNightPlugin {
    fn build(&self, app: &mut App) {
        let clock = self
            .cycle_seconds
            .map_or_else(GameClock::default, |seconds| GameClock {
                hour: 12.0,
                timescale: 86_400.0 / seconds,
            });
        app.insert_resource(clock)
            .insert_resource(DayNightPreview(self.cycle_seconds.is_some()))
            .init_resource::<WeatherTransition>()
            .init_resource::<DayNightRuntime>()
            .add_systems(Startup, spawn_day_night_text)
            .add_systems(
                Update,
                (
                    advance_clock,
                    advance_weather_transition,
                    apply_day_night_environment,
                )
                    .chain()
                    .in_set(ViewerSet::WorldSync),
            )
            .add_systems(Update, update_day_night_text.in_set(ViewerSet::Ui));
    }
}

fn advance_clock(
    real_time: Res<Time<Real>>,
    virtual_time: Res<Time<Virtual>>,
    mut clock: ResMut<GameClock>,
) {
    advance_clock_by(&mut clock, real_time.delta_secs(), virtual_time.is_paused());
}

fn advance_clock_by(clock: &mut GameClock, real_delta_seconds: f32, virtual_time_paused: bool) {
    if !virtual_time_paused && clock.timescale > 0.0 {
        clock.hour = advance_game_hour(clock.hour, clock.timescale, real_delta_seconds);
    }
}

fn advance_weather_transition(time: Res<Time<Real>>, mut transition: ResMut<WeatherTransition>) {
    if transition.target_weather_form_id.is_none() {
        return;
    }

    let duration = transition.duration_seconds;
    if !duration.is_finite() || duration <= 0.0 {
        if duration.is_nan() || duration <= 0.0 {
            transition.elapsed_seconds = 0.0;
            transition.source_weather_form_id = transition.target_weather_form_id;
        }
        return;
    }

    let elapsed = if transition.elapsed_seconds.is_finite() {
        transition.elapsed_seconds.max(0.0)
    } else if transition.elapsed_seconds.is_sign_positive() {
        duration
    } else {
        0.0
    };
    let delta = if time.delta_secs().is_finite() {
        time.delta_secs().max(0.0)
    } else {
        0.0
    };
    transition.elapsed_seconds = (elapsed + delta).min(duration);
    if transition.elapsed_seconds >= duration {
        transition.source_weather_form_id = transition.target_weather_form_id;
    }
}

pub(crate) fn profile_for_cell(
    cell: &CellInfo,
    preview: bool,
) -> (Option<&PreparedDayNightProfile>, &'static str) {
    if preview {
        let profile = cell
            .day_night_profile
            .as_ref()
            .or(cell.day_night_preview_profile.as_ref());
        (
            profile,
            profile.map_or("STATIC", |profile| match profile.source {
                PreparedDayNightProfileSource::Authoritative => "PREVIEW_AUTHORITATIVE",
                PreparedDayNightProfileSource::PreviewFallback => "PREVIEW_FALLBACK",
            }),
        )
    } else if uses_dynamic_lighting(cell.interior, cell.behave_like_exterior, false) {
        let profile = cell.day_night_profile.as_ref();
        (
            profile,
            if profile.is_some() {
                "FAITHFUL"
            } else {
                "STATIC"
            },
        )
    } else {
        (None, "STATIC")
    }
}

pub(crate) fn apply_day_night_environment(world: &mut World) {
    let Some(manifest) = world.get_resource::<LoadedSceneManifest>() else {
        return;
    };
    let cell = manifest.cell.clone();
    let lightmapped_diffuse_enabled = runtime_lightmapped_diffuse_enabled(manifest.bake.as_ref());
    let preview = world.resource::<DayNightPreview>().0;
    let hour = world.resource::<GameClock>().hour;
    let profile = profile_for_cell(&cell, preview).0.cloned();
    let weather_transition = *world.resource::<WeatherTransition>();
    let cell_changed = world.resource::<DayNightRuntime>().last_cell_form_id != Some(cell.form_id);
    let preview_changed = world.resource::<DayNightRuntime>().last_preview != preview;

    let Some(profile) = profile else {
        let restore = {
            let runtime = world.resource::<DayNightRuntime>();
            runtime.dynamic_applied || cell_changed || preview_changed
        };
        if restore {
            refresh_environment_for_active_cell(world);
            let cameras = {
                let mut query = world.query_filtered::<Entity, With<Camera3d>>();
                query.iter(world).collect::<Vec<_>>()
            };
            for camera in cameras {
                world.entity_mut(camera).remove::<Skybox>();
            }
        }
        let mut runtime = world.resource_mut::<DayNightRuntime>();
        runtime.last_cell_form_id = Some(cell.form_id);
        runtime.last_preview = preview;
        runtime.dynamic_applied = false;
        runtime.sky_elapsed = 0.0;
        return;
    };

    let mut ambient = interpolate_keyframes(profile.ambient, profile.timings, hour);
    let mut sunlight = interpolate_keyframes(profile.sunlight, profile.timings, hour);
    let mut sky_upper = interpolate_keyframes(profile.sky_upper, profile.timings, hour);
    let mut sky_lower = interpolate_keyframes(profile.sky_lower, profile.timings, hour);
    apply_weather_transition(
        &mut ambient,
        &mut sunlight,
        &mut sky_upper,
        &mut sky_lower,
        weather_transition,
        hour,
        weather_transition
            .source_weather_form_id
            .and_then(|form_id| weather_profile_for_world(world, form_id)),
        weather_transition
            .target_weather_form_id
            .and_then(|form_id| weather_profile_for_world(world, form_id)),
    );
    if preview {
        // Debug preview must retain the authored interior's mood. Weather
        // supplies hue and its own relative day/night variation, while the
        // weather's authored day luminance is normalized to the cell's
        // effective ambient/directional luminance.
        ambient = preview_weather_color(ambient, cell.ambient_rgba, profile.ambient.day);
        sunlight = preview_weather_color(sunlight, cell.directional_rgba, profile.sunlight.day);
    }
    let lighting_scale = world.resource::<LightingScale>().0;
    let ambient_scale = world.resource::<AmbientScale>().0;
    world.insert_resource(GlobalAmbientLight {
        color: Color::srgb(ambient[0], ambient[1], ambient[2]),
        brightness: 25.0 * lighting_scale * ambient_scale,
        affects_lightmapped_meshes: lightmapped_diffuse_enabled,
    });
    {
        let mut directional_lights =
            world.query::<(&mut DirectionalLight, &CellDirectionalLight)>();
        let disabled = sunlight[..3].iter().copied().sum::<f32>() <= f32::EPSILON
            || !sunlight[..3].iter().all(|channel| channel.is_finite());
        for (mut light, cell_light) in directional_lights.iter_mut(world) {
            light.color = Color::srgb(sunlight[0], sunlight[1], sunlight[2]);
            light.affects_lightmapped_mesh_diffuse = lightmapped_diffuse_enabled;
            light.illuminance = scaled_directional_illuminance(
                cell_light.base_illuminance,
                lighting_scale,
                disabled,
            );
        }
    }

    let real_delta = world.resource::<Time<Real>>().delta_secs();
    let update_sky = {
        let mut runtime = world.resource_mut::<DayNightRuntime>();
        runtime.sky_elapsed += real_delta;
        let update = cell_changed
            || preview_changed
            || !runtime.dynamic_applied
            || runtime.sky_elapsed >= SKY_UPDATE_INTERVAL_SECONDS;
        if update {
            runtime.sky_elapsed = 0.0;
        }
        update
    };
    if update_sky {
        update_skybox(world, sky_upper, sky_lower);
    }

    let mut runtime = world.resource_mut::<DayNightRuntime>();
    runtime.last_cell_form_id = Some(cell.form_id);
    runtime.last_preview = preview;
    runtime.dynamic_applied = true;
}

#[allow(clippy::too_many_arguments)]
fn apply_weather_transition(
    ambient: &mut [f32; 4],
    sunlight: &mut [f32; 4],
    sky_upper: &mut [f32; 4],
    sky_lower: &mut [f32; 4],
    transition: WeatherTransition,
    hour: f32,
    source_profile: Option<PreparedWeatherProfile>,
    target_profile: Option<PreparedWeatherProfile>,
) {
    let Some(target) = transition.target_weather_form_id else {
        return;
    };
    let progress =
        weather_transition_progress(transition.elapsed_seconds, transition.duration_seconds);
    let Some(target_profile) = target_profile else {
        let tint = weather_tint(target);
        for color in [
            &mut *ambient,
            &mut *sunlight,
            &mut *sky_upper,
            &mut *sky_lower,
        ] {
            for channel in 0..3 {
                color[channel] = finite_or_zero(color[channel]) * (1.0 - progress)
                    + finite_or_zero(tint[channel]) * progress;
            }
        }
        sanitize_colors(ambient, sunlight, sky_upper, sky_lower);
        return;
    };
    let source_profile = source_profile.as_ref();
    let source_ambient = source_profile
        .map(|profile| interpolate_weather(profile, hour, |profile| profile.ambient))
        .unwrap_or(*ambient);
    let source_sunlight = source_profile
        .map(|profile| interpolate_weather(profile, hour, |profile| profile.sunlight))
        .unwrap_or(*sunlight);
    let source_sky_upper = source_profile
        .map(|profile| interpolate_weather(profile, hour, |profile| profile.sky_upper))
        .unwrap_or(*sky_upper);
    let source_sky_lower = source_profile
        .map(|profile| interpolate_weather(profile, hour, |profile| profile.sky_lower))
        .unwrap_or(*sky_lower);
    let target_ambient = interpolate_weather(&target_profile, hour, |profile| profile.ambient);
    let target_sunlight = interpolate_weather(&target_profile, hour, |profile| profile.sunlight);
    let target_sky_upper = interpolate_weather(&target_profile, hour, |profile| profile.sky_upper);
    let target_sky_lower = interpolate_weather(&target_profile, hour, |profile| profile.sky_lower);
    for (destination, source, target) in [
        (&mut *ambient, source_ambient, target_ambient),
        (&mut *sunlight, source_sunlight, target_sunlight),
        (&mut *sky_upper, source_sky_upper, target_sky_upper),
        (&mut *sky_lower, source_sky_lower, target_sky_lower),
    ] {
        for channel in 0..4 {
            destination[channel] = finite_or_zero(source[channel]) * (1.0 - progress)
                + finite_or_zero(target[channel]) * progress;
        }
    }
    sanitize_colors(ambient, sunlight, sky_upper, sky_lower);
}

fn weather_transition_progress(elapsed_seconds: f32, duration_seconds: f32) -> f32 {
    if duration_seconds.is_nan() || duration_seconds <= 0.0 {
        return 1.0;
    }
    if duration_seconds.is_infinite() {
        return 0.0;
    }
    if elapsed_seconds.is_nan() || elapsed_seconds <= 0.0 {
        return 0.0;
    }
    if elapsed_seconds.is_infinite() {
        return 1.0;
    }
    (elapsed_seconds / duration_seconds).clamp(0.0, 1.0)
}

fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

fn sanitize_colors(
    ambient: &mut [f32; 4],
    sunlight: &mut [f32; 4],
    sky_upper: &mut [f32; 4],
    sky_lower: &mut [f32; 4],
) {
    for color in [ambient, sunlight, sky_upper, sky_lower] {
        for channel in color {
            *channel = finite_or_zero(*channel);
        }
    }
}

fn interpolate_weather(
    profile: &PreparedWeatherProfile,
    hour: f32,
    select: impl Fn(&PreparedWeatherProfile) -> bevyout_core::time_of_day::ColorKeyframes,
) -> [f32; 4] {
    bevyout_core::time_of_day::interpolate_keyframes(select(profile), profile.timings, hour)
}

fn weather_profile_for_world(world: &World, form_id: u32) -> Option<PreparedWeatherProfile> {
    let stream_state = world.get_resource::<super::world::exterior::ExteriorStreamState>();
    let streamed = stream_state
        .and_then(|state| state.cells.get(&state.current_grid))
        .and_then(|cell| cell.package.as_ref())
        .map(|package| &package.environment);
    let environment = streamed.or_else(|| {
        world
            .get_resource::<LoadedSceneManifest>()
            .and_then(|manifest| manifest.exterior.as_ref())
            .map(|package| &package.environment)
    })?;
    let catalog = stream_state
        .and_then(|state| state.index.as_ref())
        .map(|index| index.weather_profiles.as_slice())
        .unwrap_or_default();
    resolve_prepared_weather_profile(environment, catalog, form_id)
}

fn weather_tint(form_id: u32) -> [f32; 4] {
    // A hand-authored or stale manifest can still request a weather record
    // that was not included in its package. Keep that failure deterministic
    // and visible instead of silently retaining the source weather.
    let seed = form_id.rotate_left(11) ^ 0x9e37_79b9;
    let variation = (seed & 0xff) as f32 / 255.0;
    [
        0.16 + variation * 0.12,
        0.20 + variation * 0.13,
        0.22 + variation * 0.16,
        1.0,
    ]
}

fn update_skybox(world: &mut World, upper: [f32; 4], lower: [f32; 4]) {
    let pixels = sky_pixels(upper, lower);
    let handle = world.resource::<DayNightRuntime>().sky_image.clone();
    let handle = if let Some(handle) = handle {
        if let Some(mut image) = world.resource_mut::<Assets<Image>>().get_mut(&handle) {
            image.data = Some(pixels);
        }
        handle
    } else {
        let mut image = Image::new(
            Extent3d {
                width: SKY_SIZE,
                height: SKY_SIZE * 6,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
        image
            .reinterpret_stacked_2d_as_array(6)
            .expect("procedural sky is six equally sized cubemap faces");
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
        image.sampler = ImageSampler::linear();
        let handle = world.resource_mut::<Assets<Image>>().add(image);
        world.resource_mut::<DayNightRuntime>().sky_image = Some(handle.clone());
        handle
    };

    let cameras = {
        let mut query = world.query_filtered::<Entity, With<Camera3d>>();
        query.iter(world).collect::<Vec<_>>()
    };
    for camera in cameras {
        world.entity_mut(camera).insert(Skybox {
            image: Some(handle.clone()),
            brightness: SKY_BRIGHTNESS,
            ..default()
        });
    }
}

fn sky_pixels(upper: [f32; 4], lower: [f32; 4]) -> Vec<u8> {
    let mut pixels = vec![0_u8; (SKY_SIZE * SKY_SIZE * 6 * 4) as usize];
    for face in 0..6_u32 {
        for y in 0..SKY_SIZE {
            let amount = match face {
                2 => 0.0,
                3 => 1.0,
                _ => y as f32 / (SKY_SIZE - 1) as f32,
            };
            let color = std::array::from_fn::<_, 4, _>(|channel| {
                upper[channel] + (lower[channel] - upper[channel]) * amount
            });
            for x in 0..SKY_SIZE {
                let offset = (((face * SKY_SIZE + y) * SKY_SIZE + x) * 4) as usize;
                pixels[offset..offset + 4].copy_from_slice(&rgba8(color));
            }
        }
    }
    pixels
}

fn rgba8(color: [f32; 4]) -> [u8; 4] {
    std::array::from_fn(|channel| (color[channel].clamp(0.0, 1.0) * 255.0).round() as u8)
}

fn preview_weather_color(
    weather: [f32; 4],
    authored_cell: [f32; 4],
    weather_day: [f32; 4],
) -> [f32; 4] {
    let day_luminance = relative_luminance(weather_day);
    let authored_luminance = relative_luminance(authored_cell);
    let scale = if day_luminance.is_finite()
        && authored_luminance.is_finite()
        && day_luminance > f32::EPSILON
    {
        finite_or_zero(authored_luminance / day_luminance).max(0.0)
    } else {
        0.0
    };
    [0, 1, 2, 3].map(|channel| {
        if channel == 3 {
            finite_or_zero(weather[channel])
        } else {
            finite_or_zero(weather[channel] * scale)
        }
    })
}

fn relative_luminance(color: [f32; 4]) -> f32 {
    finite_or_zero(color[0]) * 0.2126
        + finite_or_zero(color[1]) * 0.7152
        + finite_or_zero(color[2]) * 0.0722
}

fn spawn_day_night_text(mut commands: Commands) {
    commands.spawn((
        Text::new("TIME 12:00 | x0 | <none> | STATIC"),
        DayNightText,
        console::DiagnosticUi,
        TextLayout::justify(Justify::Right),
        Node {
            position_type: PositionType::Absolute,
            top: px(80),
            right: px(10),
            ..default()
        },
    ));
}

fn update_day_night_text(
    clock: Res<GameClock>,
    preview: Res<DayNightPreview>,
    manifest: Option<Res<LoadedSceneManifest>>,
    mut text: Single<&mut Text, With<DayNightText>>,
) {
    let total_minutes = (normalize_hour(clock.hour) * 60.0).floor() as u32 % (24 * 60);
    let hour = total_minutes / 60;
    let minute = total_minutes % 60;
    let (weather, mode) = manifest
        .as_deref()
        .map_or(("<none>", "STATIC"), |manifest| {
            let (active, mode) = profile_for_cell(&manifest.cell, preview.0);
            let weather = active
                .or(manifest.cell.day_night_profile.as_ref())
                .or(manifest.cell.day_night_preview_profile.as_ref())
                .and_then(|profile| profile.weather_editor_id.as_deref())
                .unwrap_or("<none>");
            (weather, mode)
        });
    text.0 = format!(
        "TIME {hour:02}:{minute:02} | {} | {weather} | {mode}",
        timescale_label(clock.timescale)
    );
}

fn timescale_label(timescale: f32) -> String {
    if (timescale - timescale.round()).abs() < 0.01 {
        format!("x{:.0}", timescale)
    } else {
        format!("x{timescale:.1}")
    }
}

#[cfg(test)]
#[path = "tests/day_night.rs"]
mod tests;
