use std::collections::{HashMap, HashSet};

use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

use crate::vsa::{PreparedAudioClip, PreparedFootstepSet};

const LISTENER_EAR_GAP_METERS: f32 = 0.2;
const FOOTSTEP_GAIN_DB: f32 = -6.0;
pub(crate) const PICKUP_CONTAINER_GAIN_DB: f32 = 3.0;

/// Requests a transient sound by its resolved ESM4 FormID.
///
/// A position enables Bevy's spatial stereo panning unless the source record
/// explicitly marks the clip as 2D. Missing and unstaged clips are silent.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub(crate) struct PlaySound {
    pub(crate) form_id: u32,
    pub(crate) position: Option<Vec3>,
    pub(crate) gain_db: f32,
}

#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub(crate) struct PlayFootstep {
    pub(crate) surface: String,
    pub(crate) right: bool,
    pub(crate) variant: usize,
}

#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub(crate) struct PlayLanding {
    pub(crate) surface: String,
    pub(crate) variant: usize,
    pub(crate) hard: bool,
}

impl PlaySound {
    pub(crate) fn at(form_id: u32, position: Vec3) -> Self {
        Self::with_gain(form_id, position, 0.0)
    }

    pub(crate) fn pickup_at(form_id: u32, position: Vec3) -> Self {
        Self::with_gain(form_id, position, PICKUP_CONTAINER_GAIN_DB)
    }

    pub(crate) fn container_at(form_id: u32, position: Vec3) -> Self {
        Self::with_gain(form_id, position, PICKUP_CONTAINER_GAIN_DB)
    }

    fn with_gain(form_id: u32, position: Vec3, gain_db: f32) -> Self {
        Self {
            form_id,
            position: Some(position),
            gain_db,
        }
    }
}

#[derive(Resource, Default)]
struct AudioClipCatalog {
    clips: HashMap<u32, PreparedAudioClip>,
}

#[derive(Resource, Default)]
struct FootstepCatalog {
    sets: HashMap<String, PreparedFootstepSet>,
    hard_landing_clips: Vec<String>,
}

#[derive(Resource, Default)]
struct ReportedMissingClips(HashSet<u32>);

#[derive(Resource, Default)]
struct ReportedMissingFootsteps(HashSet<String>);

#[derive(Component)]
struct CellAmbientLoop;

#[derive(Component)]
struct PlacementLoop {
    #[allow(dead_code)]
    reference_form_id: u32,
}

pub(crate) struct ViewerAudioPlugin;

impl Plugin for ViewerAudioPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    app.init_resource::<AudioClipCatalog>()
        .init_resource::<FootstepCatalog>()
        .init_resource::<ReportedMissingClips>()
        .init_resource::<ReportedMissingFootsteps>()
        .add_message::<PlaySound>()
        .add_message::<PlayFootstep>()
        .add_message::<PlayLanding>()
        .add_systems(Startup, build_catalog_and_spawn_loops)
        .add_systems(
            Update,
            (
                ensure_single_listener,
                play_requested_sounds,
                play_requested_footsteps,
                play_requested_landings,
            ),
        );
}

fn build_catalog_and_spawn_loops(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<crate::viewer::LoadedSceneManifest>,
    mut catalog: ResMut<AudioClipCatalog>,
    mut footstep_catalog: ResMut<FootstepCatalog>,
    mut reported_missing: ResMut<ReportedMissingClips>,
) {
    catalog.clips.clear();
    for clip in &manifest.audio_clips {
        if catalog.clips.insert(clip.form_id, clip.clone()).is_some() {
            warn!(
                "audio FormID {:08x} appears more than once; using the last prepared clip",
                clip.form_id
            );
        }
    }
    footstep_catalog.sets = manifest
        .footstep_sets
        .iter()
        .cloned()
        .map(|set| (set.surface.clone(), set))
        .collect();
    footstep_catalog.hard_landing_clips = manifest.hard_landing_clips.clone();

    for form_id in &manifest.cell_audio.ambient_loop_sound_form_ids {
        if let Some(entity) = spawn_catalog_sound(
            &mut commands,
            &asset_server,
            &catalog,
            &mut reported_missing,
            *form_id,
            None,
            PlaybackMode::Loop,
            true,
            0.0,
        ) {
            commands.entity(entity).insert(CellAmbientLoop);
        }
    }

    for placement in manifest
        .placements
        .iter()
        .filter(|placement| placement.initially_enabled)
    {
        let Some(form_id) = placement.audio.loop_sound_form_id else {
            continue;
        };
        if let Some(entity) = spawn_catalog_sound(
            &mut commands,
            &asset_server,
            &catalog,
            &mut reported_missing,
            form_id,
            Some(Vec3::from_array(placement.translation)),
            PlaybackMode::Loop,
            false,
            0.0,
        ) {
            commands.entity(entity).insert(PlacementLoop {
                reference_form_id: placement.reference_form_id,
            });
        }
    }
}

/// Keeps the one Bevy spatial listener on the active 3D camera. The component
/// stays correct when the viewer reparents that camera for FPS mode.
fn ensure_single_listener(
    mut commands: Commands,
    cameras: Query<(Entity, &Camera), With<Camera3d>>,
    listeners: Query<Entity, With<SpatialListener>>,
) {
    let active_camera = cameras
        .iter()
        .find_map(|(entity, camera)| camera.is_active.then_some(entity));

    for listener in &listeners {
        if Some(listener) != active_camera {
            commands.entity(listener).remove::<SpatialListener>();
        }
    }

    if let Some(active_camera) = active_camera
        && listeners.get(active_camera).is_err()
    {
        commands
            .entity(active_camera)
            .insert(SpatialListener::new(LISTENER_EAR_GAP_METERS));
    }
}

fn play_requested_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<AudioClipCatalog>,
    mut reported_missing: ResMut<ReportedMissingClips>,
    mut requests: MessageReader<PlaySound>,
) {
    for request in requests.read() {
        let spawned = spawn_catalog_sound(
            &mut commands,
            &asset_server,
            &catalog,
            &mut reported_missing,
            request.form_id,
            request.position,
            PlaybackMode::Despawn,
            request.position.is_none(),
            request.gain_db,
        );
        if spawned.is_some() {
            info!(
                "sound playback: form={:08x} spatial={} gain_db={} started",
                request.form_id,
                request.position.is_some(),
                request.gain_db
            );
        }
    }
}

fn play_requested_footsteps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<FootstepCatalog>,
    mut reported_missing: ResMut<ReportedMissingFootsteps>,
    mut requests: MessageReader<PlayFootstep>,
) {
    for request in requests.read() {
        let set = catalog
            .sets
            .get(&request.surface)
            .or_else(|| catalog.sets.get("concrete"));
        let Some(set) = set else {
            if reported_missing.0.insert(request.surface.clone()) {
                warn!("no prepared footstep set for surface {}", request.surface);
            }
            continue;
        };
        let clips = footstep_clips(set, request.right);
        spawn_footstep_clip(
            &mut commands,
            &asset_server,
            clips,
            request.variant,
            &request.surface,
            FOOTSTEP_GAIN_DB,
            &mut reported_missing,
        );
    }
}

fn play_requested_landings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<FootstepCatalog>,
    mut reported_missing: ResMut<ReportedMissingFootsteps>,
    mut requests: MessageReader<PlayLanding>,
) {
    for request in requests.read() {
        let set = catalog
            .sets
            .get(&request.surface)
            .or_else(|| catalog.sets.get("concrete"));
        let Some(set) = set else {
            if reported_missing.0.insert(request.surface.clone()) {
                warn!("no prepared landing set for surface {}", request.surface);
            }
            continue;
        };
        let clips = if request.hard && !catalog.hard_landing_clips.is_empty() {
            &catalog.hard_landing_clips
        } else {
            &set.land
        };
        spawn_footstep_clip(
            &mut commands,
            &asset_server,
            clips,
            request.variant,
            if request.hard {
                "hard_landing"
            } else {
                &request.surface
            },
            FOOTSTEP_GAIN_DB,
            &mut reported_missing,
        );
    }
}

fn spawn_footstep_clip(
    commands: &mut Commands,
    asset_server: &AssetServer,
    clips: &[String],
    variant: usize,
    missing_key: &str,
    gain_db: f32,
    reported_missing: &mut ResMut<ReportedMissingFootsteps>,
) {
    let Some(asset_path) = select_clip_path(clips, variant) else {
        if reported_missing.0.insert(missing_key.to_string()) {
            warn!("prepared footstep set {missing_key} has no usable clips");
        }
        return;
    };
    commands.spawn((
        AudioPlayer::new(asset_server.load(asset_path.to_owned())),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            spatial: false,
            volume: Volume::Decibels(gain_db),
            ..default()
        },
    ));
}

fn footstep_clips(set: &PreparedFootstepSet, right: bool) -> &[String] {
    if right { &set.right } else { &set.left }
}

fn select_clip_path(clips: &[String], variant: usize) -> Option<&str> {
    clips
        .get(variant % clips.len().max(1))
        .map(String::as_str)
        .filter(|path| !path.is_empty())
}

#[allow(clippy::too_many_arguments)]
fn spawn_catalog_sound(
    commands: &mut Commands,
    asset_server: &AssetServer,
    catalog: &AudioClipCatalog,
    reported_missing: &mut ReportedMissingClips,
    form_id: u32,
    position: Option<Vec3>,
    mode: PlaybackMode,
    force_non_spatial: bool,
    gain_db: f32,
) -> Option<Entity> {
    let Some(clip) = catalog.clips.get(&form_id) else {
        report_missing_once(
            reported_missing,
            form_id,
            "is absent from the prepared catalog",
        );
        return None;
    };
    let Some(asset_path) = clip.asset_path.as_ref() else {
        report_missing_once(reported_missing, form_id, "has no staged asset");
        return None;
    };

    let settings =
        playback_settings_with_gain(clip, mode, position.is_some(), force_non_spatial, gain_db);
    let transform = Transform::from_translation(position.unwrap_or(Vec3::ZERO));
    Some(
        commands
            .spawn((
                AudioPlayer::new(asset_server.load(asset_path.clone())),
                settings,
                transform,
            ))
            .id(),
    )
}

fn report_missing_once(reported: &mut ReportedMissingClips, form_id: u32, reason: &str) {
    if reported.0.insert(form_id) {
        warn!("sound {:08x} {reason}; playback skipped", form_id);
    }
}

fn playback_settings(
    clip: &PreparedAudioClip,
    mode: PlaybackMode,
    has_position: bool,
    force_non_spatial: bool,
) -> PlaybackSettings {
    playback_settings_with_gain(clip, mode, has_position, force_non_spatial, 0.0)
}

fn playback_settings_with_gain(
    clip: &PreparedAudioClip,
    mode: PlaybackMode,
    has_position: bool,
    force_non_spatial: bool,
    gain_db: f32,
) -> PlaybackSettings {
    PlaybackSettings {
        mode,
        // Fallout stores this as a positive attenuation magnitude in 1/100 dB.
        volume: Volume::Decibels(gain_db - (clip.static_attenuation_hundredths_db as f32) / 100.0),
        spatial: has_position && !clip.is_2d && !force_non_spatial,
        ..default()
    }
}

/// Issue #52: switches the cell ambient loop(s) over to whatever
/// `PreparedSceneManifest` is currently the active resource (already
/// repointed to the destination cell by the caller). A plain despawn of the
/// previous loop(s) followed by spawning the destination's -- no
/// crossfade -- matching F52.2's "plain stop/start is acceptable". Also
/// rebuilds the audio clip and footstep catalogs from the new manifest,
/// since each prepared manifest carries only the clips its own cell needs.
/// Per-placement loop sounds (e.g. idle machinery) are not started here;
/// see this issue's final report for why that is deliberately deferred.
pub(crate) fn rebuild_ambient_for_active_cell(world: &mut World) {
    let (ambient_form_ids, audio_clips, footstep_sets, hard_landing_clips) = {
        let manifest = world.resource::<crate::viewer::LoadedSceneManifest>();
        (
            manifest.cell_audio.ambient_loop_sound_form_ids.clone(),
            manifest.audio_clips.clone(),
            manifest.footstep_sets.clone(),
            manifest.hard_landing_clips.clone(),
        )
    };

    let mut previous = world.query_filtered::<Entity, With<CellAmbientLoop>>();
    let previous_entities: Vec<Entity> = previous.iter(world).collect();
    for entity in previous_entities {
        world.despawn(entity);
    }

    {
        let mut catalog = world.resource_mut::<AudioClipCatalog>();
        catalog.clips.clear();
        for clip in audio_clips {
            let form_id = clip.form_id;
            if catalog.clips.insert(form_id, clip).is_some() {
                warn!(
                    "audio FormID {form_id:08x} appears more than once; using the last prepared clip"
                );
            }
        }
    }
    {
        let mut footstep_catalog = world.resource_mut::<FootstepCatalog>();
        footstep_catalog.sets = footstep_sets
            .into_iter()
            .map(|set| (set.surface.clone(), set))
            .collect();
        footstep_catalog.hard_landing_clips = hard_landing_clips;
    }

    let asset_server = world.resource::<AssetServer>().clone();
    for form_id in ambient_form_ids {
        let clip = world
            .resource::<AudioClipCatalog>()
            .clips
            .get(&form_id)
            .cloned();
        let Some(clip) = clip else {
            let mut reported = world.resource_mut::<ReportedMissingClips>();
            report_missing_once(
                &mut reported,
                form_id,
                "is absent from the prepared catalog",
            );
            continue;
        };
        let Some(asset_path) = clip.asset_path.clone() else {
            let mut reported = world.resource_mut::<ReportedMissingClips>();
            report_missing_once(&mut reported, form_id, "has no staged asset");
            continue;
        };
        let settings = playback_settings(&clip, PlaybackMode::Loop, false, true);
        world.spawn((
            AudioPlayer::new(asset_server.load(asset_path)),
            settings,
            Transform::default(),
            CellAmbientLoop,
        ));
    }
}

#[cfg(test)]
#[path = "audio/tests/mod.rs"]
mod tests;
