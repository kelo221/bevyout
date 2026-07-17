//! Prepared-scene spawning and lighting presentation.

use std::sync::Arc;

use super::controls::{AmbientScale, FogStrength, LightingScale};
use super::lighting_policy::{
    PreparedCellFogPlan, PreparedLightPlanInput, PreparedLightVolumetricPlan,
    plan_prepared_light_migration,
};
use super::world::{ResidentCell, ResidentCells, ResidentState};
use super::*;
use crate::vsa::placement_never_casts_shadow;
use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::light::NotShadowCaster;
use bevy::post_process::bloom::{BloomCompositeMode, BloomPrefilter};
use bevy::render::render_resource::TextureFormat;

#[derive(Component)]
pub(crate) struct BakedStaticSceneRoot;

#[derive(Component)]
pub(crate) struct PreparedPointShadowReceiverRoot;

/// Marks meshes under the launch cell as prepared-shadow receivers once their
/// async GLB scene instances appear. Combined baked geometry is also removed
/// from Bevy's per-frame shadow pass; interactive/moving placements keep
/// casting there, so the shader can combine both sources.
pub(crate) fn mark_prepared_shadow_meshes(
    mut commands: Commands,
    roots: Query<(Entity, Has<BakedStaticSceneRoot>), With<PreparedPointShadowReceiverRoot>>,
    placement_roots: Query<&interaction::PlacementRoot>,
    parents: Query<&ChildOf>,
    meshes: Query<Entity, Added<Mesh3d>>,
) {
    let roots = roots.iter().collect::<HashMap<_, _>>();
    for mesh in &meshes {
        let mut ancestor = mesh;
        let mut root_kind = None;
        let mut never_casts_shadow = false;
        loop {
            if let Ok(placement_root) = placement_roots.get(ancestor) {
                never_casts_shadow |= placement_never_casts_shadow(placement_root.placement());
            }
            if let Some(is_baked_static) = roots.get(&ancestor) {
                root_kind = Some(*is_baked_static);
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        let mut entity = commands.entity(mesh);
        if root_kind.is_some() {
            entity.insert(bevy::pbr::BakedPointShadowReceiver);
        }
        if root_kind == Some(true) || never_casts_shadow {
            entity.insert(NotShadowCaster);
        }
    }
}

pub(super) fn fallout_bloom() -> Bloom {
    Bloom {
        intensity: 0.05,
        prefilter: BloomPrefilter {
            // Keep cell fog below bloom's HDR prefilter while preserving the
            // deliberately hot emissive fixtures above it.
            threshold: 1.2,
            threshold_softness: 0.2,
        },
        composite_mode: BloomCompositeMode::Additive,
        ..Bloom::OLD_SCHOOL
    }
}

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
        Projection::Perspective(default_perspective_projection()),
        HorizontalFov::default(),
        ShadowFilteringMethod::Hardware2x2,
        DepthPrepass,
        DeferredPrepass,
        DynamicLightingView,
        OcclusionCulling,
        fallout_bloom(),
        Tonemapping::AcesFitted,
        Exposure { ev100: 12.0 },
        color_grading,
        initial_camera_transform,
        FlyCamera {
            yaw: initial_yaw,
            pitch: initial_pitch,
            speed: 8.0,
        },
    ));
    camera.insert((Hdr, Msaa::Off));
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
    let mut prepared_shadow_records = HashMap::new();
    let prepared_shadow_runtime = if let Some(shadows) = manifest.static_point_shadows.as_ref() {
        prepared_shadow_records.extend(
            shadows
                .lights
                .iter()
                .map(|light| (light.reference_form_id, light)),
        );
        let artifact_path = PathBuf::from(&manifest.asset_root).join(
            shadows
                .asset_path
                .replace('/', std::path::MAIN_SEPARATOR_STR),
        );
        let depth_data = (|| -> Result<Arc<[u8]>> {
            let bytes = fs::read(&artifact_path).with_context(|| {
                format!(
                    "could not read prepared point-shadow artifact {}",
                    artifact_path.display()
                )
            })?;
            let image = Image::from_buffer(
                &bytes,
                ImageType::Extension("ktx2"),
                CompressedImageFormats::NONE,
                false,
                ImageSampler::Default,
                RenderAssetUsages::MAIN_WORLD,
            )
            .with_context(|| {
                format!(
                    "could not decode prepared point-shadow artifact {}",
                    artifact_path.display()
                )
            })?;
            let expected_layers = shadows.lights.len() as u32 * 6;
            if image.texture_descriptor.format != TextureFormat::Depth32Float
                || image.texture_descriptor.size.width != shadows.resolution
                || image.texture_descriptor.size.height != shadows.resolution
                || image.texture_descriptor.size.depth_or_array_layers != expected_layers
            {
                anyhow::bail!(
                    "prepared point-shadow artifact {} does not match D32_SFLOAT {}x{} with {} array layers",
                    artifact_path.display(),
                    shadows.resolution,
                    shadows.resolution,
                    expected_layers,
                );
            }
            image
                .data
                .map(Arc::from)
                .context("prepared point-shadow artifact decoded without depth data")
        })();
        let (depth_data, load_error) = match depth_data {
            Ok(data) => (Some(data), None),
            Err(error) => {
                error!("{error:#}");
                (None, Some(format!("{error:#}")))
            }
        };
        commands.insert_resource(BakedPointShadowMap {
            data: depth_data.clone(),
            fingerprint: Some(shadows.source_fingerprint.clone()),
            resolution: shadows.resolution,
            layers: shadows.lights.len() as u32,
        });
        PreparedPointShadowRuntime {
            revision: Some(shadows.revision.clone()),
            fingerprint: Some(shadows.source_fingerprint.clone()),
            asset_path: Some(shadows.asset_path.clone()),
            resolution: shadows.resolution,
            near_z: shadows.near_z,
            layers: shadows.lights.len() as u32,
            attached_lights: 0,
            cpu_loaded: depth_data.is_some(),
            load_error,
        }
    } else {
        commands.insert_resource(BakedPointShadowMap::default());
        PreparedPointShadowRuntime::default()
    };
    let prepared_shadow_available = prepared_shadow_runtime.cpu_loaded;
    let mut attached_shadow_lights = 0_u32;
    if let Some(bake) = &manifest.bake {
        let mut root = commands.spawn((
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(bake.scene_path.clone())),
            ),
            BakedStaticSceneRoot,
        ));
        if prepared_shadow_available {
            root.insert(PreparedPointShadowReceiverRoot);
        }
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
    // neighbor preloader's bookkeeping covers it too. The startup cell's
    // lights are spawned here rather than through `spawn_cell_lights` so
    // prepared static point shadows (#53) can attach their baked slots;
    // preloaded neighbor cells keep plain lights — the baked shadow map is
    // a single global resource holding only this manifest's artifact.
    let mut root_entity = commands.spawn((Transform::default(), Visibility::Visible));
    if prepared_shadow_available {
        root_entity.insert(PreparedPointShadowReceiverRoot);
    }
    let root = root_entity.id();
    let light_by_reference = manifest
        .lights
        .iter()
        .map(|light| (light.reference_form_id, light))
        .collect::<HashMap<_, _>>();
    let migration = plan_prepared_light_migration(
        manifest.lights.iter().map(|light| PreparedLightPlanInput {
            reference_form_id: light.reference_form_id,
            initially_enabled: light.initially_enabled,
            radius: light.radius,
            prepared_shadow_layer: prepared_shadow_available
                .then(|| prepared_shadow_records.get(&light.reference_form_id))
                .flatten()
                .map(|shadow| shadow.layer),
        }),
        lighting.0,
        prepared_cell_fog_plan(&cell_lighting, fog_strength.0),
    );
    let point_shadow_defaults = PointLight::default();
    let prepared_normal_bias = if prepared_shadow_runtime.resolution > 0 {
        point_shadow_defaults.shadow_normal_bias * 2.0 / prepared_shadow_runtime.resolution as f32
            * std::f32::consts::SQRT_2
    } else {
        0.0
    };
    let mut realtime_shadow_proxy = None;
    for source in migration.sources {
        let light = light_by_reference[&source.reference_form_id];
        let prepared_shadow = source.prepared_shadow_layer.map(|cubemap_index| {
            attached_shadow_lights += 1;
            DynamicLightPreparedShadow {
                cubemap_index,
                depth_bias: point_shadow_defaults.shadow_depth_bias,
                normal_bias: prepared_normal_bias,
                near_z: prepared_shadow_runtime.near_z,
            }
        });
        let transform = Transform::from_translation(Vec3::from_array(light.translation));
        let dynamic_light = commands
            .spawn((
                Name::new(format!(
                    "Prepared DynamicLight {:08x}",
                    source.reference_form_id
                )),
                DynamicLight::with_effect(source.intensity, DynamicLightEffect::Steady)
                    .with_color(Color::srgb(
                        light.color_rgba[0],
                        light.color_rgba[1],
                        light.color_rgba[2],
                    ))
                    .with_radius(source.radius)
                    .with_volumetric(dynamic_volumetric_parameters(source.volumetric))
                    .with_shadows(prepared_shadow.is_some() || source.realtime_shadow_proxy),
                DynamicLightPreparedSource {
                    reference_form_id: source.reference_form_id,
                    baked_shadow: prepared_shadow,
                    volumetric_intensity_at_full_strength: source
                        .volumetric
                        .map_or(0.0, |volumetric| volumetric.intensity_at_full_strength),
                },
                transform,
                Visibility::Visible,
                ChildOf(root),
            ))
            .id();
        if source.realtime_shadow_proxy {
            let proxy = commands
                .spawn((
                    Name::new(format!(
                        "Prepared DynamicLight realtime shadow proxy {:08x}",
                        source.reference_form_id
                    )),
                    DynamicLightShadowProxy::shadow_only_point_light(source.radius),
                    DynamicLightShadowProxy::realtime(dynamic_light),
                    transform,
                    Visibility::Visible,
                    ChildOf(root),
                ))
                .id();
            realtime_shadow_proxy = Some(proxy);
        }
    }
    commands.insert_resource(RealtimeShadowLight(realtime_shadow_proxy));
    commands.insert_resource(PreparedPointShadowRuntime {
        attached_lights: attached_shadow_lights,
        ..prepared_shadow_runtime
    });
    let (content, _next) = spawn_cell_placements_chunk(
        &mut commands,
        &asset_server,
        &manifest,
        root,
        Some(&mut references),
        0,
        usize::MAX,
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
        "controls: Tab opens Pip-Boy, ` (backquote) opens the console, Esc pauses and releases cursor, left click captures cursor"
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

/// Spawns a cell's initially-enabled point lights under `root` (issue #51's
/// per-cell root). Used for preloaded neighbor cells only — the startup
/// cell's lights are spawned inline in `spawn_prepared_scene` so prepared
/// static point shadows (#53) can attach; the global baked shadow map only
/// covers the startup manifest.
pub(crate) fn spawn_cell_lights(
    commands: &mut Commands,
    manifest: &PreparedSceneManifest,
    root: Entity,
    lighting_scale: f32,
) {
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
}

/// Spawns up to `max_entries` raw `manifest.placements` entries starting at
/// index `start`, returning what was spawned and the next raw index (equal
/// to `manifest.placements.len()` once the cell is fully spawned). The
/// preloader drains large cells through this a bounded chunk per frame so a
/// background preload never spawns a thousand entities in one frame spike;
/// callers wanting everything at once pass `usize::MAX`.
pub(crate) fn spawn_cell_placements_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    manifest: &PreparedSceneManifest,
    root: Entity,
    mut references: Option<&mut crate::console::RefRegistry>,
    start: usize,
    max_entries: usize,
) -> (SpawnedCellContent, usize) {
    // Baked cells still spawn placements individually (for interaction),
    // just excluding whichever placements the bake already folded into the
    // combined static mesh (`is_bake_static`); non-baked cells spawn every
    // enabled placement.
    let exclude_bake_static = manifest.bake.is_some();
    let mut scene_handles = Vec::new();
    let mut placement_count = 0;
    let end = start
        .saturating_add(max_entries)
        .min(manifest.placements.len());
    for placement in manifest.placements[start..end]
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

    (
        SpawnedCellContent {
            scene_handles,
            placement_count,
        },
        end,
    )
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
    let (start, end) = cell_fog_range_metres(lighting)?;
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

fn cell_fog_range_metres(lighting: &PreparedCellLighting) -> Option<(f32, f32)> {
    let start = lighting.fog_near.max(0.0) * FO3_SCALE;
    let mut end = lighting.fog_far.max(0.0) * FO3_SCALE;
    if lighting.fog_clip_distance > 0.0 {
        end = end.min(lighting.fog_clip_distance * FO3_SCALE);
    }
    (start.is_finite() && end.is_finite() && end > start).then_some((start, end))
}

fn prepared_cell_fog_plan(
    lighting: &PreparedCellLighting,
    strength: f32,
) -> Option<PreparedCellFogPlan> {
    if lighting.fog_rgba.iter().any(|channel| !channel.is_finite())
        || !lighting.fog_power.is_finite()
        || !strength.is_finite()
        || strength < 0.0
    {
        return None;
    }
    let (start_metres, end_metres) = cell_fog_range_metres(lighting)?;
    Some(PreparedCellFogPlan {
        start_metres,
        end_metres,
        power: lighting.fog_power,
        strength: strength.clamp(0.0, 1.0),
    })
}

fn dynamic_volumetric_parameters(
    plan: Option<PreparedLightVolumetricPlan>,
) -> DynamicLightVolumetricParameters {
    plan.map_or_else(DynamicLightVolumetricParameters::default, |plan| {
        DynamicLightVolumetricParameters {
            volumetric_type: DynamicLightVolumetricType::Sphere,
            radius: plan.radius,
            thickness: plan.thickness,
            intensity: plan.intensity,
            visibility: plan.visibility,
        }
    })
}

pub(crate) fn apply_fog_strength(
    fog_strength: Res<FogStrength>,
    manifest: Res<PreparedSceneManifest>,
    mut cameras: Query<&mut DistanceFog, With<Camera3d>>,
    mut prepared_lights: Query<(&DynamicLightPreparedSource, &mut DynamicLight)>,
) {
    if !fog_strength.is_changed() {
        return;
    }
    let strength = if fog_strength.0.is_finite() {
        fog_strength.0.clamp(0.0, 1.0)
    } else {
        0.0
    };
    for (source, mut light) in &mut prepared_lights {
        light.config.volumetric.intensity = source.volumetric_intensity_at_full_strength * strength;
    }
    let lighting = effective_lighting(&manifest.cell);
    let Some(fog) = distance_fog(&lighting, fog_strength.0) else {
        return;
    };
    for mut camera_fog in &mut cameras {
        *camera_fog = fog.clone();
    }
}

/// Issue #52: re-derives ambient light, camera fog, and the directional
/// light from whatever `PreparedSceneManifest` is currently the active
/// resource (already repointed to the destination cell by the caller),
/// applying them directly to the existing camera/light entities and the
/// `GlobalAmbientLight` resource rather than spawning new ones.
/// `apply_fog_strength` only reacts to `FogStrength` changing, so a cell
/// swap needs its own explicit refresh to pick up the new cell's lighting.
/// Camera post-processing (color grading/auto-exposure/bloom, which follow
/// the cell's `ImageSpace`) are deliberately left as-is; see this issue's
/// final report.
pub(crate) fn refresh_environment_for_active_cell(world: &mut World) {
    let cell = world.resource::<PreparedSceneManifest>().cell.clone();
    let lighting_scale = world.resource::<LightingScale>().0;
    let ambient_scale = world.resource::<AmbientScale>().0;
    let fog_strength = world.resource::<FogStrength>().0;
    let cell_lighting = effective_lighting(&cell);

    world.insert_resource(GlobalAmbientLight {
        color: Color::srgb(
            cell_lighting.ambient_rgba[0],
            cell_lighting.ambient_rgba[1],
            cell_lighting.ambient_rgba[2],
        ),
        brightness: 25.0 * lighting_scale * ambient_scale,
        affects_lightmapped_meshes: true,
    });

    if let Some(fog) = distance_fog(&cell_lighting, fog_strength) {
        let mut cameras = world.query_filtered::<&mut DistanceFog, With<Camera3d>>();
        for mut camera_fog in cameras.iter_mut(world) {
            *camera_fog = fog.clone();
        }
    }

    let directional_luminance = cell_lighting.directional_rgba[0]
        + cell_lighting.directional_rgba[1]
        + cell_lighting.directional_rgba[2];
    let disabled = directional_luminance <= f32::EPSILON
        || !cell_lighting.directional_rgba[..3]
            .iter()
            .all(|channel| channel.is_finite());
    let base_illuminance = CELL_DIRECTIONAL_ILLUMINANCE;
    let mut directional_lights = world.query::<(
        &mut DirectionalLight,
        &mut CellDirectionalLight,
        &mut Transform,
    )>();
    for (mut light, mut cell_light, mut transform) in directional_lights.iter_mut(world) {
        light.color = Color::srgb(
            cell_lighting.directional_rgba[0],
            cell_lighting.directional_rgba[1],
            cell_lighting.directional_rgba[2],
        );
        light.illuminance =
            scaled_directional_illuminance(base_illuminance, lighting_scale, disabled);
        cell_light.base_illuminance = base_illuminance;
        transform.rotation = Quat::from_array(cell_lighting.directional_rotation_xyzw());
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
    let speed = image_space_eye_adaptation_speed(image_space.eye_adapt_speed) * 2.0;
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
