//! Prepared-scene spawning and lighting presentation.

use std::sync::Arc;

use super::controls::{AmbientScale, FogStrength, LightingScale};
use super::world::{ResidentCell, ResidentCells, ResidentState};
use super::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_prepared_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
    manifest: Res<PreparedSceneManifest>,
    lighting: Res<LightingScale>,
    ambient_scale: Res<AmbientScale>,
    fog_strength: Res<FogStrength>,
    mut references: ResMut<crate::console::RefRegistry>,
    mut resident_cells: ResMut<ResidentCells>,
) {
    let focus = scene_focus(&manifest);
    let initial_camera_position =
        transition_camera_position(&manifest).unwrap_or_else(|| focus + Vec3::new(0.0, 4.0, 12.0));
    let initial_camera_transform =
        Transform::from_translation(initial_camera_position).looking_at(focus, Vec3::Y);
    let (initial_yaw, initial_pitch, _) = initial_camera_transform.rotation.to_euler(EulerRot::YXZ);
    let cell_lighting = effective_lighting(&manifest.cell);
    let (color_grading, auto_exposure) =
        camera_post_processing(manifest.cell.image_space.as_ref(), &mut compensation_curves);
    let mut camera = commands.spawn((
        Camera3d::default(),
        DepthPrepass,
        OcclusionCulling,
        Bloom::NATURAL,
        Tonemapping::TonyMcMapface,
        Exposure { ev100: 12.0 },
        color_grading,
        initial_camera_transform,
        FlyCamera {
            yaw: initial_yaw,
            pitch: initial_pitch,
            speed: 8.0,
        },
    ));
    if let Some(auto_exposure) = auto_exposure {
        camera.insert(auto_exposure);
        if let Some(image_space) = manifest.cell.image_space.as_ref() {
            info!(
                "applying ImageSpace {:08x} ({}) eye_adapt_speed={:.3} target_lum={:.3}",
                image_space.form_id,
                image_space.editor_id.as_deref().unwrap_or("<unnamed>"),
                image_space.eye_adapt_speed,
                image_space.hdr_target_lum,
            );
        }
    } else {
        warn!(
            "{} has no resolved ImageSpace; retaining fixed viewer post-processing",
            cell_label(&manifest.cell)
        );
    }
    if let Some(fog) = distance_fog(&cell_lighting, fog_strength.0) {
        camera.insert(fog);
    }
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(
            cell_lighting.ambient_rgba[0],
            cell_lighting.ambient_rgba[1],
            cell_lighting.ambient_rgba[2],
        ),
        brightness: 25.0 * lighting.0 * ambient_scale.0,
        affects_lightmapped_meshes: true,
    });
    let directional_luminance = cell_lighting.directional_rgba[0]
        + cell_lighting.directional_rgba[1]
        + cell_lighting.directional_rgba[2];
    if directional_luminance > f32::EPSILON
        && cell_lighting.directional_rgba[..3]
            .iter()
            .all(|channel| channel.is_finite())
    {
        let base_illuminance = CELL_DIRECTIONAL_ILLUMINANCE;
        commands.spawn((
            DirectionalLight {
                color: Color::srgb(
                    cell_lighting.directional_rgba[0],
                    cell_lighting.directional_rgba[1],
                    cell_lighting.directional_rgba[2],
                ),
                illuminance: scaled_directional_illuminance(base_illuminance, lighting.0, false),
                affects_lightmapped_mesh_diffuse: true,
                shadow_maps_enabled: false,
                ..default()
            },
            CellDirectionalLight { base_illuminance },
            Transform::from_rotation(Quat::from_array(cell_lighting.directional_rotation_xyzw())),
        ));
    }
    if let Some(bake) = &manifest.bake {
        commands.spawn(WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(bake.scene_path.clone())),
        ));
        if let Some(volume) = &bake.irradiance_volume {
            commands.spawn((
                LightProbe::default(),
                IrradianceVolume {
                    voxels: asset_server.load(volume.asset_path.clone()),
                    intensity: volume.intensity,
                    affects_lightmapped_meshes: true,
                },
                Transform {
                    translation: Vec3::from_array(volume.translation),
                    rotation: Quat::from_xyzw(
                        volume.rotation_xyzw[0],
                        volume.rotation_xyzw[1],
                        volume.rotation_xyzw[2],
                        volume.rotation_xyzw[3],
                    ),
                    scale: Vec3::from_array(volume.scale),
                },
            ));
            info!(
                "loading baked scene {} with irradiance volume {} at {:?}",
                bake.scene_path, volume.asset_path, volume.resolution
            );
        } else {
            warn!(
                "baked scene {} has no irradiance volume; run `bake {}`",
                bake.scene_path,
                cell_label(&manifest.cell)
            );
        }
    }

    // Issue #51: the startup cell's placements and per-cell point lights are
    // parented under a per-cell root entity, visible and refs-registered,
    // and recorded as an already-`Ready` resident so the predictive
    // neighbor preloader's bookkeeping covers it too.
    let root = commands
        .spawn((Transform::default(), Visibility::Visible))
        .id();
    let content = spawn_cell_content(
        &mut commands,
        &asset_server,
        &manifest,
        root,
        lighting.0,
        Some(&mut references),
    );
    resident_cells.0.insert(
        manifest.cell.form_id,
        ResidentCell {
            root,
            state: ResidentState::Ready,
            manifest: Arc::new((*manifest).clone()),
            scene_handles: content.scene_handles,
            placement_count: content.placement_count,
        },
    );

    info!(
        "loaded {} with {} placements, {} diagnostics; camera focus {:?}",
        cell_label(&manifest.cell),
        manifest.placements.len(),
        manifest.diagnostics.len(),
        focus,
    );
    info!(
        "controls: V toggles FPS player/free camera, Tab opens Pip-Boy, Esc pauses and releases cursor, left click captures cursor"
    );
}

/// Content spawned for one cell by `spawn_cell_content`: the scene handles
/// to watch for load completion, and how many placements were spawned.
///
/// `Handle<WorldAsset>`, not `Handle<Scene>` -- `bevy_gltf`'s scene loader
/// targets `bevy_world_serialization`'s `WorldAsset` in this Bevy version
/// (`Scene` itself is now a trait), matching `WorldAssetRoot`'s own
/// `Handle<WorldAsset>` field.
pub(crate) struct SpawnedCellContent {
    pub(crate) scene_handles: Vec<Handle<WorldAsset>>,
    pub(crate) placement_count: usize,
}

/// Spawns one cell's placements and per-cell point lights, parented under
/// `root` (issue #51's per-cell root, shared by the startup cell and every
/// preloaded neighbor). `references` is `None` for preloaded (hidden,
/// not-yet-active) cells: they must not become console-selectable or
/// interactable until issue #52 activates them.
pub(crate) fn spawn_cell_content(
    commands: &mut Commands,
    asset_server: &AssetServer,
    manifest: &PreparedSceneManifest,
    root: Entity,
    lighting_scale: f32,
    mut references: Option<&mut crate::console::RefRegistry>,
) -> SpawnedCellContent {
    for light in &manifest.lights {
        if !light.initially_enabled {
            continue;
        }
        commands.spawn((
            PointLight {
                intensity: light.radius * light.radius * 2.0 * lighting_scale,
                range: light.radius,
                color: Color::srgb(
                    light.color_rgba[0],
                    light.color_rgba[1],
                    light.color_rgba[2],
                ),
                affects_lightmapped_mesh_diffuse: true,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(Vec3::from_array(light.translation)),
            ChildOf(root),
        ));
    }

    // Baked cells still spawn placements individually (for interaction),
    // just excluding whichever placements the bake already folded into the
    // combined static mesh (`is_bake_static`); non-baked cells spawn every
    // enabled placement.
    let exclude_bake_static = manifest.bake.is_some();
    let mut scene_handles = Vec::new();
    let mut placement_count = 0;
    for placement in manifest
        .placements
        .iter()
        .filter(|placement| placement.initially_enabled)
        .filter(|placement| !exclude_bake_static || !is_bake_static(placement))
    {
        let Some(path) = placement.asset_path.as_ref() else {
            continue;
        };
        let handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone()));
        let entity = commands
            .spawn((
                WorldAssetRoot(handle.clone()),
                interaction::PlacementRoot::new(placement.clone()),
                Transform {
                    translation: Vec3::from_array(placement.translation),
                    rotation: Quat::from_xyzw(
                        placement.rotation_xyzw[0],
                        placement.rotation_xyzw[1],
                        placement.rotation_xyzw[2],
                        placement.rotation_xyzw[3],
                    ),
                    scale: Vec3::splat(placement.scale),
                },
                ChildOf(root),
            ))
            .id();
        if let Some(references) = references.as_deref_mut() {
            references.register(
                entity,
                placement.reference_form_id,
                placement.editor_id.as_deref(),
            );
        }
        scene_handles.push(handle);
        placement_count += 1;
    }

    SpawnedCellContent {
        scene_handles,
        placement_count,
    }
}

pub(crate) fn effective_lighting(cell: &CellInfo) -> PreparedCellLighting {
    cell.effective_lighting
        .clone()
        .unwrap_or_else(|| PreparedCellLighting {
            ambient_rgba: cell.ambient_rgba,
            directional_rgba: cell.directional_rgba,
            ..default()
        })
}

pub(crate) fn distance_fog(lighting: &PreparedCellLighting, strength: f32) -> Option<DistanceFog> {
    let values = [
        lighting.fog_near,
        lighting.fog_far,
        lighting.fog_clip_distance,
        lighting.fog_power,
        lighting.directional_fade,
    ];
    if values.iter().any(|value| !value.is_finite())
        || lighting
            .fog_rgba
            .iter()
            .chain(lighting.directional_rgba.iter())
            .any(|value| !value.is_finite())
        || lighting.fog_far <= 0.0
        || lighting.fog_far <= lighting.fog_near
    {
        return None;
    }
    let start = lighting.fog_near.max(0.0) * FO3_SCALE;
    let mut end = lighting.fog_far.max(0.0) * FO3_SCALE;
    if lighting.fog_clip_distance > 0.0 {
        end = end.min(lighting.fog_clip_distance * FO3_SCALE);
    }
    if !start.is_finite() || !end.is_finite() || end <= start {
        return None;
    }
    if !strength.is_finite() || strength < 0.0 {
        return None;
    }
    let strength = strength.clamp(0.0, 1.0);
    let directional_fade = lighting.directional_fade.clamp(0.0, 1.0);
    Some(DistanceFog {
        color: Color::srgba(
            lighting.fog_rgba[0],
            lighting.fog_rgba[1],
            lighting.fog_rgba[2],
            strength,
        ),
        directional_light_color: Color::srgba(
            lighting.directional_rgba[0] * directional_fade,
            lighting.directional_rgba[1] * directional_fade,
            lighting.directional_rgba[2] * directional_fade,
            strength,
        ),
        directional_light_exponent: lighting.fog_power.max(1.0),
        falloff: FogFalloff::Linear { start, end },
    })
}

pub(crate) fn apply_fog_strength(
    fog_strength: Res<FogStrength>,
    manifest: Res<PreparedSceneManifest>,
    mut cameras: Query<&mut DistanceFog, With<Camera3d>>,
) {
    if !fog_strength.is_changed() {
        return;
    }
    let lighting = effective_lighting(&manifest.cell);
    let Some(fog) = distance_fog(&lighting, fog_strength.0) else {
        return;
    };
    for mut camera_fog in &mut cameras {
        *camera_fog = fog.clone();
    }
}

pub(crate) fn scaled_directional_illuminance(
    base_illuminance: f32,
    lighting_scale: f32,
    disabled: bool,
) -> f32 {
    if disabled {
        0.0
    } else {
        base_illuminance * lighting_scale / DEFAULT_LIGHTING_SCALE
    }
}

#[derive(Component)]
pub(crate) struct CellDirectionalLight {
    pub(crate) base_illuminance: f32,
}

pub(crate) fn camera_post_processing(
    image_space: Option<&ImageSpaceInfo>,
    compensation_curves: &mut Assets<AutoExposureCompensationCurve>,
) -> (ColorGrading, Option<AutoExposure>) {
    let Some(image_space) = image_space else {
        return (ColorGrading::default(), None);
    };

    let flags = image_space.flags;
    let mut color_grading = ColorGrading::default();
    if flags & 0x08 != 0 {
        color_grading.global.exposure = image_space.brightness.max(0.0001).log2();
    }
    if flags & 0x01 != 0 {
        color_grading.global.post_saturation = image_space.cinematic_saturation.max(0.0);
    }
    if flags & 0x02 != 0 {
        let contrast = image_space.cinematic_contrast.max(0.0);
        color_grading.shadows.contrast = contrast;
        color_grading.midtones.contrast = contrast;
        color_grading.highlights.contrast = contrast;
    }
    if flags & 0x04 != 0
        && let Some((temperature, tint)) = image_space_tint_to_white_balance(
            image_space.cinematic_brightness_tint_rgb,
            image_space.cinematic_brightness_tint_value,
        )
    {
        color_grading.global.temperature = temperature;
        color_grading.global.tint = tint;
    }

    let target_lum = image_space.hdr_target_lum.max(0.001);
    let compensation = target_lum.log2();
    let compensation_curve = compensation_curves.add(
        AutoExposureCompensationCurve::from_curve(LinearSpline::new([
            vec2(-8.0, compensation),
            vec2(8.0, compensation),
        ]))
        .expect("flat auto-exposure compensation curve is valid"),
    );
    let speed = image_space_eye_adaptation_speed(image_space.eye_adapt_speed);
    let auto_exposure = AutoExposure {
        speed_brighten: speed,
        speed_darken: speed,
        compensation_curve,
        ..default()
    };
    (color_grading, Some(auto_exposure))
}

pub(crate) fn image_space_eye_adaptation_speed(value: f32) -> f32 {
    0.5 + (1.0 - value.clamp(0.0, 1.0)) * 7.5
}

pub(crate) fn image_space_tint_to_white_balance(
    rgb: [f32; 3],
    strength: f32,
) -> Option<(f32, f32)> {
    let [r, g, b] = rgb;
    let x = 0.412_456_4 * r + 0.357_576_1 * g + 0.180_437_5 * b;
    let y = 0.212_672_9 * r + 0.715_152_2 * g + 0.072_175 * b;
    let z = 0.019_333_9 * r + 0.119_192 * g + 0.950_304_1 * b;
    let sum = x + y + z;
    if sum <= f32::EPSILON || !sum.is_finite() {
        return None;
    }
    let target_x = x / sum;
    let target_y = y / sum;
    let strength = strength.max(0.0);
    Some((
        (0.3127 - target_x) * strength,
        (target_y - 0.3290) * strength,
    ))
}

pub(crate) type GlowCardMeshQuery<'w> = (Entity, &'w GltfMeshName);

pub(crate) fn configure_glow_cards(
    mut commands: Commands,
    meshes: Query<GlowCardMeshQuery<'_>, (With<Mesh3d>, Without<GlowCard>)>,
    mut inspected: Local<HashSet<Entity>>,
    mut last_mesh_count: Local<Option<usize>>,
) {
    let mesh_count = meshes.iter().count();
    if *last_mesh_count == Some(mesh_count) {
        return;
    }
    *last_mesh_count = Some(mesh_count);
    for (entity, name) in &meshes {
        if !inspected.insert(entity) {
            continue;
        }
        if !is_glow_card_mesh_name(&name.0) {
            continue;
        }
        // Converted assets promote the physical bulb to an emissive material
        // and no longer export this hint card. Keep this fallback for older
        // cached GLBs so they cannot reintroduce the large flat billboard.
        commands
            .entity(entity)
            .insert((Visibility::Hidden, GlowCard));
    }
}

#[derive(Component)]
pub(crate) struct GlowCard;

pub(crate) fn is_glow_card_mesh_name(name: &str) -> bool {
    name.to_ascii_lowercase().starts_with("lightglow")
}

pub(crate) fn scene_focus(manifest: &PreparedSceneManifest) -> Vec3 {
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    let mut found = false;
    for placement in &manifest.placements {
        if placement.asset_path.is_none() {
            continue;
        }
        let position = Vec3::from_array(placement.translation);
        minimum = minimum.min(position);
        maximum = maximum.max(position);
        found = true;
    }
    if found {
        (minimum + maximum) * 0.5
    } else {
        Vec3::ZERO
    }
}

pub(crate) fn transition_camera_position(manifest: &PreparedSceneManifest) -> Option<Vec3> {
    manifest
        .placements
        .iter()
        .filter(|placement| placement.initially_enabled)
        .find_map(|placement| {
            matches!(
                &placement.semantic,
                PreparedSemantic::Door(door) if door.destination.is_some()
            )
            .then_some(Vec3::from_array(placement.translation) + Vec3::Y * player::EYE_HEIGHT)
        })
}
