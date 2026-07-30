//! Fallout game clock, climate lighting, procedural sky, and diagnostic HUD.

use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
};
use bevyout_core::manifest::{CellInfo, PreparedDayNightProfile};
use bevyout_core::time_of_day::{
    advance_game_hour, interpolate_keyframes, normalize_hour, uses_dynamic_lighting,
};

use super::controls::{AmbientScale, LightingScale};
use super::scene::{
    CellDirectionalLight, refresh_environment_for_active_cell, scaled_directional_illuminance,
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
            .init_resource::<DayNightRuntime>()
            .add_systems(Startup, spawn_day_night_text)
            .add_systems(
                Update,
                (advance_clock, apply_day_night_environment)
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
            if profile.is_some() {
                "PREVIEW"
            } else {
                "STATIC"
            },
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
    let preview = world.resource::<DayNightPreview>().0;
    let hour = world.resource::<GameClock>().hour;
    let profile = profile_for_cell(&cell, preview).0.cloned();
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
        affects_lightmapped_meshes: true,
    });
    {
        let mut directional_lights =
            world.query::<(&mut DirectionalLight, &CellDirectionalLight)>();
        let disabled = sunlight[..3].iter().copied().sum::<f32>() <= f32::EPSILON
            || !sunlight[..3].iter().all(|channel| channel.is_finite());
        for (mut light, cell_light) in directional_lights.iter_mut(world) {
            light.color = Color::srgb(sunlight[0], sunlight[1], sunlight[2]);
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
        let upper = interpolate_keyframes(profile.sky_upper, profile.timings, hour);
        let lower = interpolate_keyframes(profile.sky_lower, profile.timings, hour);
        update_skybox(world, upper, lower);
    }

    let mut runtime = world.resource_mut::<DayNightRuntime>();
    runtime.last_cell_form_id = Some(cell.form_id);
    runtime.last_preview = preview;
    runtime.dynamic_applied = true;
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
    let scale = if day_luminance > f32::EPSILON {
        authored_luminance / day_luminance
    } else {
        0.0
    };
    [
        weather[0] * scale,
        weather[1] * scale,
        weather[2] * scale,
        weather[3],
    ]
}

fn relative_luminance(color: [f32; 4]) -> f32 {
    color[0] * 0.2126 + color[1] * 0.7152 + color[2] * 0.0722
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
mod tests {
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
}
