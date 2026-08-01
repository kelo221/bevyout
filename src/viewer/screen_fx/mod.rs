//! Runtime ImageSpace/IMAD screen feedback feature.

use anyhow::{Context, Result, bail};
use bevy::core_pipeline::prepass::MotionVectorPrepass;
use bevy::post_process::dof::DepthOfField;
use bevy::post_process::dof::DepthOfFieldMode;
use bevy::post_process::effect_stack::{ChromaticAberration, LensDistortion, Vignette};
use bevy::post_process::motion_blur::MotionBlur;
use bevy::prelude::*;
use bevy::render::view::ColorGrading;
use bevyout_core::image_space::{
    IMAGE_SPACE_MODIFIER_CATALOG_REVISION, ImageSpaceModifier, ImageSpaceModifierCurveOperation,
    ImageSpaceModifierProperty, PreparedImageSpaceModifierCatalog,
};
use ron::de::from_bytes;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::app_state::AppState;
use crate::vsa::{CellInfo, ImageSpaceInfo, PreparedSceneManifest, fingerprint};

pub(crate) mod policy;

#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct ScreenFxCatalog {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    pub(crate) modifiers: BTreeMap<u32, ImageSpaceModifier>,
}

pub(crate) fn load_catalog_for_manifest(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> Result<ScreenFxCatalog> {
    let Some(relative) = manifest.image_space_modifier_catalog_path.as_deref() else {
        return Ok(ScreenFxCatalog::default());
    };
    let path = asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    let bytes = fs::read(&path)
        .with_context(|| format!("reading ImageSpace modifier catalog {}", path.display()))?;
    if manifest.image_space_modifier_catalog_hash.as_deref() != Some(fingerprint(&bytes).as_str()) {
        bail!("ImageSpace modifier catalog hash does not match scene manifest");
    }
    let catalog: PreparedImageSpaceModifierCatalog = from_bytes(&bytes)
        .with_context(|| format!("invalid ImageSpace modifier catalog {}", path.display()))?;
    if manifest.image_space_modifier_catalog_revision.as_deref() != Some(catalog.revision.as_str())
    {
        bail!("ImageSpace modifier catalog revision does not match scene manifest");
    }
    if catalog.revision != IMAGE_SPACE_MODIFIER_CATALOG_REVISION {
        bail!(
            "ImageSpace modifier catalog revision {} is stale, expected {IMAGE_SPACE_MODIFIER_CATALOG_REVISION}; run `prepare` again",
            catalog.revision
        );
    }
    if catalog.source_fingerprint != manifest.source_fingerprint {
        bail!(
            "ImageSpace modifier catalog fingerprint {} does not match scene {}",
            catalog.source_fingerprint,
            manifest.source_fingerprint
        );
    }
    Ok(ScreenFxCatalog {
        revision: catalog.revision,
        source_fingerprint: catalog.source_fingerprint,
        modifiers: catalog
            .modifiers
            .into_iter()
            .map(|modifier| (modifier.form_id, modifier))
            .collect(),
    })
}

#[derive(Message, Clone, Debug)]
pub(crate) struct ScreenFxRequested(pub(crate) policy::ScreenFxRequest);

impl ScreenFxRequested {
    pub(crate) fn weapon_hit() -> Self {
        let modifier_form_id = 0xffff_ff01;
        let definition = policy::ScreenFxDefinition {
            modifier_form_id,
            duration_ms: 450,
            static_values: policy::ScreenFxValues {
                blood: 0.72,
                double_vision: 0.18,
                motion_blur: 0.12,
                radial_blur: 0.08,
                ..policy::ScreenFxValues::neutral()
            },
            curves: vec![policy::ScreenFxCurve {
                property: policy::ScreenFxProperty::Blood,
                operation: policy::ScreenFxCurveOperation::Additive,
                keyframes: vec![
                    policy::ScreenFxKeyframe {
                        time_ms: 0,
                        value: 1.0,
                    },
                    policy::ScreenFxKeyframe {
                        time_ms: 450,
                        value: 0.0,
                    },
                ],
            }],
            color_keyframes: Vec::new(),
            fade_color_keyframes: Vec::new(),
        };
        let mut start = policy::ScreenFxStart::new(
            policy::ScreenFxSource::WeaponHit,
            modifier_form_id,
            100,
            // The adapter fills this with the policy clock when the message
            // is consumed.  This keeps the request independent of Bevy time.
            u64::MAX,
            definition,
        );
        start.intensity = 1.0;
        Self(policy::ScreenFxRequest::Start(start))
    }
}

#[derive(Resource, Debug)]
pub(crate) struct ScreenFxRuntime {
    pub(crate) policy: policy::ScreenFxPolicy,
    pub(crate) last_output: policy::ScreenFxValues,
    pub(crate) active_cell_form_id: Option<u32>,
    pub(crate) overlay_entity: Option<Entity>,
    pub(crate) last_camera_mode: Option<super::player::CameraMode>,
}

impl Default for ScreenFxRuntime {
    fn default() -> Self {
        let policy = policy::ScreenFxPolicy::default();
        Self {
            last_output: policy.snapshot(),
            policy,
            active_cell_form_id: None,
            overlay_entity: None,
            last_camera_mode: None,
        }
    }
}

pub(crate) struct ScreenFxPlugin;

impl Plugin for ScreenFxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenFxRuntime>()
            .add_message::<ScreenFxRequested>()
            .add_systems(OnEnter(AppState::InGame), spawn_screen_fx_overlay)
            .add_systems(OnExit(AppState::InGame), clear_screen_fx_on_exit)
            .add_systems(
                Update,
                consume_requests
                    .in_set(super::plugins::ViewerSet::Interaction)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                observe_camera_mode
                    .in_set(super::plugins::ViewerSet::Interaction)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                advance_policy
                    .in_set(super::plugins::ViewerSet::WorldSync)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                apply_screen_fx
                    .in_set(super::plugins::ViewerSet::Ui)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
struct ScreenFxOverlay;

fn spawn_screen_fx_overlay(mut commands: Commands, mut runtime: ResMut<ScreenFxRuntime>) {
    let entity = commands
        .spawn((
            ScreenFxOverlay,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                ..default()
            },
            BackgroundColor(Color::NONE),
            GlobalZIndex(900),
        ))
        .id();
    runtime.overlay_entity = Some(entity);
}

fn clear_screen_fx_on_exit(
    mut commands: Commands,
    mut runtime: ResMut<ScreenFxRuntime>,
    overlays: Query<Entity, With<ScreenFxOverlay>>,
) {
    runtime.policy.apply(policy::ScreenFxRequest::Clear {
        reason: policy::ScreenFxClearReason::Teardown,
    });
    runtime.active_cell_form_id = None;
    runtime.last_camera_mode = None;
    runtime.overlay_entity = None;
    for entity in &overlays {
        commands.entity(entity).despawn();
    }
}

fn consume_requests(
    mut requests: MessageReader<ScreenFxRequested>,
    mut runtime: ResMut<ScreenFxRuntime>,
) {
    let now_ms = runtime.policy.now_ms();
    for request in requests.read() {
        let mut request = request.0.clone();
        if let policy::ScreenFxRequest::Start(start) = &mut request
            && start.start_ms == u64::MAX
        {
            start.start_ms = now_ms;
        }
        runtime.policy.apply(request);
    }
}

fn observe_camera_mode(
    mode: Option<Res<super::player::CameraModeState>>,
    mut runtime: ResMut<ScreenFxRuntime>,
) {
    let Some(mode) = mode else {
        return;
    };
    if runtime.last_camera_mode != Some(mode.mode) {
        if runtime.last_camera_mode.is_some() {
            runtime.policy.apply(policy::ScreenFxRequest::Clear {
                reason: policy::ScreenFxClearReason::CameraMode,
            });
        }
        runtime.last_camera_mode = Some(mode.mode);
    }
}

fn advance_policy(
    time: Res<Time>,
    manifest: Res<crate::viewer::LoadedSceneManifest>,
    mut runtime: ResMut<ScreenFxRuntime>,
) {
    if runtime.active_cell_form_id != Some(manifest.cell.form_id) {
        runtime.policy.apply(policy::ScreenFxRequest::Clear {
            reason: policy::ScreenFxClearReason::CellTransition,
        });
        runtime.policy.set_base(base_values_from_image_space(
            manifest.cell.image_space.as_ref(),
        ));
        runtime.active_cell_form_id = Some(manifest.cell.form_id);
    }
    let delta_ms = (time.delta_secs_f64() * 1_000.0).round().max(0.0) as u64;
    let now_ms = runtime.policy.now_ms().saturating_add(delta_ms);
    runtime.policy.advance_to(now_ms);
}

#[allow(clippy::type_complexity)]
fn apply_screen_fx(
    mut commands: Commands,
    mut runtime: ResMut<ScreenFxRuntime>,
    mut cameras: Query<
        (
            Entity,
            &Camera,
            &mut ColorGrading,
            Option<&mut MotionBlur>,
            &mut ChromaticAberration,
            &mut LensDistortion,
            &mut Vignette,
            Option<&mut DepthOfField>,
        ),
        With<Camera3d>,
    >,
    mut overlays: Query<&mut BackgroundColor, With<ScreenFxOverlay>>,
) {
    let output = runtime.policy.snapshot();
    runtime.last_output = output;
    for (
        entity,
        camera,
        mut grading,
        motion_blur,
        mut chromatic_aberration,
        mut lens_distortion,
        mut vignette,
        depth_of_field,
    ) in &mut cameras
    {
        if !camera.is_active {
            continue;
        }
        apply_color_grading(&mut grading, output);
        if output.motion_blur > 0.001 {
            if let Some(mut motion_blur) = motion_blur {
                motion_blur.shutter_angle = output.motion_blur.clamp(0.0, 1.0);
                motion_blur.samples = 2;
            } else {
                commands.entity(entity).insert(MotionBlur {
                    shutter_angle: output.motion_blur.clamp(0.0, 1.0),
                    samples: 2,
                });
            }
        } else if motion_blur.is_some() {
            commands
                .entity(entity)
                .remove::<MotionBlur>()
                .remove::<MotionVectorPrepass>();
        }
        chromatic_aberration.intensity = (output.double_vision * 0.12).clamp(0.0, 0.4);
        chromatic_aberration.max_samples = if chromatic_aberration.intensity > 0.001 {
            16
        } else {
            8
        };
        lens_distortion.intensity = (output.radial_blur * 0.22).clamp(-1.0, 1.0);
        vignette.intensity = output.fade.clamp(0.0, 1.0);
        vignette.color = Color::srgba(output.tint[0], output.tint[1], output.tint[2], 1.0);
        lens_distortion.center = Vec2::new(
            output.radial_center[0].clamp(0.0, 1.0),
            output.radial_center[1].clamp(0.0, 1.0),
        );
        if output.depth_of_field > 0.001 {
            if let Some(mut depth_of_field) = depth_of_field {
                depth_of_field.aperture_f_stops =
                    (1.0 / (output.depth_of_field * 8.0)).clamp(0.05, 32.0);
                if output.depth_of_field_distance > 0.001 {
                    depth_of_field.focal_distance = output.depth_of_field_distance;
                }
                if output.depth_of_field_range > 0.001 {
                    depth_of_field.max_depth = output
                        .depth_of_field_range
                        .max(depth_of_field.focal_distance + 0.01);
                }
            } else {
                commands.entity(entity).insert(DepthOfField {
                    mode: DepthOfFieldMode::Gaussian,
                    focal_distance: if output.depth_of_field_distance > 0.001 {
                        output.depth_of_field_distance
                    } else {
                        10.0
                    },
                    aperture_f_stops: (1.0 / (output.depth_of_field * 8.0)).clamp(0.05, 32.0),
                    max_depth: if output.depth_of_field_range > 0.001 {
                        output.depth_of_field_range
                    } else {
                        1000.0
                    },
                    ..default()
                });
            }
        } else if depth_of_field.is_some() {
            commands.entity(entity).remove::<DepthOfField>();
        }
    }
    let blood = output.blood.clamp(0.0, 1.0);
    for mut background in &mut overlays {
        background.0 = Color::srgba(0.75, 0.0, 0.0, blood * 0.42);
    }
}

fn apply_color_grading(grading: &mut ColorGrading, values: policy::ScreenFxValues) {
    grading.global.exposure = values.brightness.max(0.000_1).log2();
    grading.global.post_saturation = values.saturation.max(0.0);
    let contrast = values.contrast.max(0.01);
    let gamma = contrast.recip();
    let gain = 0.5_f32.powf((1.0 - contrast) / contrast);
    grading.shadows.gamma = gamma;
    grading.midtones.gamma = gamma;
    grading.highlights.gamma = gamma;
    grading.shadows.gain = gain;
    grading.midtones.gain = gain;
    grading.highlights.gain = gain;
}

pub(crate) fn refresh_base(world: &mut World, cell: &CellInfo) {
    let Some(mut runtime) = world.get_resource_mut::<ScreenFxRuntime>() else {
        return;
    };
    runtime.policy.apply(policy::ScreenFxRequest::Clear {
        reason: policy::ScreenFxClearReason::CellTransition,
    });
    runtime
        .policy
        .set_base(base_values_from_image_space(cell.image_space.as_ref()));
    runtime.active_cell_form_id = Some(cell.form_id);
}

pub(crate) fn base_values_from_image_space(
    image_space: Option<&ImageSpaceInfo>,
) -> policy::ScreenFxValues {
    let Some(image_space) = image_space else {
        return policy::ScreenFxValues::neutral();
    };
    let mut values = policy::ScreenFxValues::neutral();
    if image_space.flags & 0x08 != 0 {
        values.brightness = image_space.cinematic_brightness.max(0.000_1);
    }
    if image_space.flags & 0x01 != 0 {
        values.saturation = image_space.cinematic_saturation.max(0.0);
    }
    if image_space.flags & 0x02 != 0 {
        values.contrast = image_space.cinematic_contrast.max(0.01);
    }
    if image_space.flags & 0x04 != 0 {
        values.tint = [
            image_space.cinematic_brightness_tint_rgb[0],
            image_space.cinematic_brightness_tint_rgb[1],
            image_space.cinematic_brightness_tint_rgb[2],
            image_space.cinematic_brightness_tint_value.clamp(0.0, 1.0),
        ];
    }
    values
}

pub(crate) fn definition_from_modifier(
    modifier: &ImageSpaceModifier,
) -> policy::ScreenFxDefinition {
    policy::ScreenFxDefinition {
        modifier_form_id: modifier.form_id,
        duration_ms: modifier.duration_ms,
        static_values: policy::ScreenFxValues {
            brightness: modifier.static_values.brightness,
            saturation: modifier.static_values.saturation,
            contrast: modifier.static_values.contrast,
            blur: modifier.static_values.blur,
            double_vision: modifier.static_values.double_vision,
            tint: modifier.static_values.tint_rgba,
            fade: modifier.static_values.fade,
            radial_blur: modifier.static_values.radial_blur,
            radial_ramp_up: modifier.static_values.radial_ramp_up,
            radial_start: modifier.static_values.radial_start,
            radial_ramp_down: modifier.static_values.radial_ramp_down,
            radial_down_start: modifier.static_values.radial_down_start,
            depth_of_field: modifier.static_values.depth_of_field_strength,
            depth_of_field_distance: modifier.static_values.depth_of_field_distance,
            depth_of_field_range: modifier.static_values.depth_of_field_range,
            motion_blur: modifier.static_values.motion_blur,
            radial_center: modifier.static_values.radial_center,
            ..policy::ScreenFxValues::neutral()
        },
        curves: modifier
            .curves
            .iter()
            .map(|curve| policy::ScreenFxCurve {
                property: match curve.property {
                    ImageSpaceModifierProperty::Blur => policy::ScreenFxProperty::Blur,
                    ImageSpaceModifierProperty::DoubleVision => {
                        policy::ScreenFxProperty::DoubleVision
                    }
                    ImageSpaceModifierProperty::Brightness => policy::ScreenFxProperty::Brightness,
                    ImageSpaceModifierProperty::Saturation => policy::ScreenFxProperty::Saturation,
                    ImageSpaceModifierProperty::Contrast => policy::ScreenFxProperty::Contrast,
                    ImageSpaceModifierProperty::Fade => policy::ScreenFxProperty::Fade,
                    ImageSpaceModifierProperty::RadialBlur => policy::ScreenFxProperty::RadialBlur,
                    ImageSpaceModifierProperty::RadialCenterX => {
                        policy::ScreenFxProperty::RadialCenterX
                    }
                    ImageSpaceModifierProperty::RadialCenterY => {
                        policy::ScreenFxProperty::RadialCenterY
                    }
                    ImageSpaceModifierProperty::RadialRampUp => {
                        policy::ScreenFxProperty::RadialRampUp
                    }
                    ImageSpaceModifierProperty::RadialStart => {
                        policy::ScreenFxProperty::RadialStart
                    }
                    ImageSpaceModifierProperty::RadialRampDown => {
                        policy::ScreenFxProperty::RadialRampDown
                    }
                    ImageSpaceModifierProperty::RadialDownStart => {
                        policy::ScreenFxProperty::RadialDownStart
                    }
                    ImageSpaceModifierProperty::DepthOfFieldStrength => {
                        policy::ScreenFxProperty::DepthOfField
                    }
                    ImageSpaceModifierProperty::DepthOfFieldDistance => {
                        policy::ScreenFxProperty::DepthOfFieldDistance
                    }
                    ImageSpaceModifierProperty::DepthOfFieldRange => {
                        policy::ScreenFxProperty::DepthOfFieldRange
                    }
                    ImageSpaceModifierProperty::MotionBlur => policy::ScreenFxProperty::MotionBlur,
                },
                operation: match curve.operation {
                    ImageSpaceModifierCurveOperation::Additive => {
                        policy::ScreenFxCurveOperation::Additive
                    }
                    ImageSpaceModifierCurveOperation::Multiplier => {
                        policy::ScreenFxCurveOperation::Multiplier
                    }
                    ImageSpaceModifierCurveOperation::Set => policy::ScreenFxCurveOperation::Set,
                },
                keyframes: curve
                    .keyframes
                    .iter()
                    .map(|keyframe| policy::ScreenFxKeyframe {
                        time_ms: keyframe.time_ms,
                        value: keyframe.value,
                    })
                    .collect(),
            })
            .collect(),
        color_keyframes: modifier
            .color_keyframes
            .iter()
            .map(|keyframe| policy::ScreenFxColorKeyframe {
                time_ms: keyframe.time_ms,
                rgba: keyframe.rgba,
            })
            .collect(),
        fade_color_keyframes: modifier
            .fade_color_keyframes
            .iter()
            .map(|keyframe| policy::ScreenFxColorKeyframe {
                time_ms: keyframe.time_ms,
                rgba: keyframe.rgba,
            })
            .collect(),
    }
}

pub(crate) fn status_json(
    runtime: &ScreenFxRuntime,
    catalog: &ScreenFxCatalog,
) -> serde_json::Value {
    json!({
        "schema": "bevyout.m5.screen_fx",
        "schema_version": 1,
        "active_cell": runtime.active_cell_form_id,
        "active_modifiers": runtime.policy.active_ids().collect::<Vec<_>>(),
        "active_count": runtime.policy.active_len(),
        "time_ms": runtime.policy.now_ms(),
        "base": {
            "brightness": runtime.policy.base().brightness,
            "saturation": runtime.policy.base().saturation,
            "contrast": runtime.policy.base().contrast,
        },
        "settings": {
            "overall_intensity": runtime.policy.settings().overall_intensity,
            "screen_blood": runtime.policy.settings().screen_blood,
            "flashes": runtime.policy.settings().flashes,
            "motion_and_distortion": runtime.policy.settings().motion_and_distortion,
        },
        "values": {
            "brightness": runtime.last_output.brightness,
            "saturation": runtime.last_output.saturation,
            "contrast": runtime.last_output.contrast,
            "blur": runtime.last_output.blur,
            "double_vision": runtime.last_output.double_vision,
            "radial_blur": runtime.last_output.radial_blur,
            "radial_ramp_up": runtime.last_output.radial_ramp_up,
            "radial_start": runtime.last_output.radial_start,
            "radial_ramp_down": runtime.last_output.radial_ramp_down,
            "radial_down_start": runtime.last_output.radial_down_start,
            "motion_blur": runtime.last_output.motion_blur,
            "fade": runtime.last_output.fade,
            "blood": runtime.last_output.blood,
            "depth_of_field": runtime.last_output.depth_of_field,
            "depth_of_field_distance": runtime.last_output.depth_of_field_distance,
            "depth_of_field_range": runtime.last_output.depth_of_field_range,
        },
        "catalog_revision": catalog.revision,
        "catalog_source_fingerprint": catalog.source_fingerprint,
        "catalog_records": catalog.modifiers.len(),
    })
}
