//! Prepared-scene spawning and lighting presentation.

use std::sync::Arc;

use super::controls::{
    AmbientScale, AuthorizedEmissionMaterials, FogStrength, ImageSpaceBloomOverrides,
    LightingScale, VolumetricFogMultiplier, image_space_emission_multiplier,
};
use super::world::{ResidentCell, ResidentCells, ResidentState};
use super::*;
use crate::vsa::PreparedPlacement;
use bevy::asset::RenderAssetUsages;
use bevy::gltf::{GltfExtras, GltfMaterialExtras};
use bevy::image::{CompressedImageFormats, Image, ImageSampler, ImageType};
use bevy::light::{
    EnvironmentMapLight, FogVolume, LightProbe, NotShadowCaster, ParallaxCorrection, VolumetricFog,
    VolumetricLight,
};
use bevy::math::Rect;
use bevy::pbr::{Lightmap, MeshMaterial3d, StandardMaterial};
use bevy::post_process::bloom::{BloomCompositeMode, BloomPrefilter};
use bevy::post_process::effect_stack::{ChromaticAberration, LensDistortion, Vignette};
use bevy::render::render_resource::TextureFormat;
use bevyout_core::manifest::PreparedBake;
use serde::Deserialize;

#[derive(Component)]
pub(crate) struct BakedStaticSceneRoot;

#[derive(Component)]
pub(crate) struct PreparedPointShadowReceiverRoot;

#[derive(Component)]
pub(crate) struct FalloutMaterialConfigured;

#[derive(Component)]
pub(crate) struct FalloutSurfaceConfigured;

#[derive(Component)]
pub(crate) struct CellVolumetricFog;

#[derive(Component)]
pub(crate) struct PreparedReflectionProbe;

#[derive(Resource)]
pub(crate) struct PreparedLightmap {
    pub(crate) bindings: HashMap<u32, PreparedLightmapBinding>,
}

#[derive(Component)]
pub(crate) struct PreparedLightmapAttached;

pub(crate) struct PreparedLightmapBinding {
    pub(crate) image: Handle<Image>,
    pub(crate) uv_rect: Rect,
}

/// Whether the shared runtime diffuse sources may still affect meshes in the
/// active prepared scene. A non-empty surface-lightmap set owns static diffuse
/// on the baked scene; dynamic and non-lightmapped meshes retain the sources
/// because Bevy applies this flag only to lightmapped meshes.
pub(crate) fn runtime_lightmapped_diffuse_enabled(bake: Option<&PreparedBake>) -> bool {
    bake.is_none_or(|bake| bake.lightmaps.is_empty())
}

pub(crate) const PREPARED_REFLECTION_PROBE_INTENSITY: f32 = 0.025;
pub(crate) const DEFAULT_REFLECTION_PROBE_STRENGTH: f32 = 100.0;

fn prepared_light_is_spot(light: &bevyout_core::manifest::PreparedLight) -> bool {
    let authored_spot = light.kind.eq_ignore_ascii_case("spot") || light.flags & 0x200 != 0;
    authored_spot && light.spot_fov_radians.is_finite() && light.spot_fov_radians > f32::EPSILON
}

fn prepared_spot_angles(light: &bevyout_core::manifest::PreparedLight) -> (f32, f32) {
    let outer =
        (light.spot_fov_radians * 0.5).clamp(f32::EPSILON, std::f32::consts::FRAC_PI_2 - 0.0001);
    ((outer * 0.8).min(outer), outer)
}

#[derive(Debug, Deserialize)]
struct FalloutMaterialExtra {
    #[serde(default)]
    shader_type: u32,
    #[serde(default)]
    shader_flags_1: u32,
    #[serde(default)]
    emission_authorized: Option<bool>,
    #[serde(default)]
    translucency_enabled: bool,
    #[serde(default)]
    translucency_strength: f32,
    #[serde(default)]
    local_thickness: Option<LocalThicknessExtra>,
}

#[derive(Debug, Deserialize)]
struct LocalThicknessExtra {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    strength: f32,
}

fn parse_fallout_material_extra(value: &str) -> Option<FalloutMaterialExtra> {
    let root: serde_json::Value = serde_json::from_str(value).ok()?;
    let nested = root.get("bevyout_fallout_material").cloned()?;
    match nested {
        serde_json::Value::Object(_) => serde_json::from_value(nested).ok(),
        serde_json::Value::String(serialized) => serde_json::from_str(&serialized).ok(),
        _ => None,
    }
}

const FALLOUT_SURFACE_STANDARD: u32 = 0;
const FALLOUT_SURFACE_HAIR: u32 = 1;
const FALLOUT_SURFACE_EYE: u32 = 2;
const FALLOUT_SURFACE_SKIN: u32 = 3;
const FALLOUT_SHADER_TYPE_SKIN_TINT: u32 = 5;
const FALLOUT_SHADER_TYPE_HAIR_TINT: u32 = 6;
const FALLOUT_SHADER_FLAG1_EYE_ENVIRONMENT_MAPPING: u32 = 1 << 17;
const FALLOUT_SHADER_FLAG1_HAIR_SOFT_LIGHTING: u32 = 1 << 18;
const FALLOUT_HAIR_ANISOTROPY_STRENGTH: f32 = 0.65;
const FALLOUT_HAIR_MIN_ROUGHNESS: f32 = 0.72;
const FALLOUT_HAIR_REFLECTANCE: f32 = 0.25;
const FALLOUT_SKIN_MIN_ROUGHNESS: f32 = 0.62;
const FALLOUT_SKIN_REFLECTANCE: f32 = 0.35;
const FALLOUT_EYE_CLEARCOAT: f32 = 1.0;
const FALLOUT_EYE_CLEARCOAT_ROUGHNESS: f32 = 0.04;

fn is_eye_surface_mesh_name(name: &str) -> bool {
    name.to_ascii_lowercase().contains("eye")
}

fn fallout_surface_kind(metadata: &FalloutMaterialExtra, mesh_name: Option<&str>) -> u32 {
    if metadata.shader_type == FALLOUT_SHADER_TYPE_HAIR_TINT
        || metadata.shader_flags_1 & FALLOUT_SHADER_FLAG1_HAIR_SOFT_LIGHTING != 0
    {
        return FALLOUT_SURFACE_HAIR;
    }

    if metadata.shader_type == FALLOUT_SHADER_TYPE_SKIN_TINT {
        return FALLOUT_SURFACE_SKIN;
    }

    // The eye-environment flag is also authored on some glasses materials.
    // Existing GLB mesh names provide the narrowest runtime discriminator;
    // when a name is unavailable, preserve the source flag as the fallback.
    if metadata.shader_flags_1 & FALLOUT_SHADER_FLAG1_EYE_ENVIRONMENT_MAPPING != 0
        && mesh_name.is_none_or(is_eye_surface_mesh_name)
    {
        return FALLOUT_SURFACE_EYE;
    }

    FALLOUT_SURFACE_STANDARD
}

/// Applies the runtime-only Fallout skin, hair, and eye variants to the existing
/// `StandardMaterial` handles. The GLB metadata remains the source of truth;
/// no scene hierarchy or prepared manifest data is changed here.
#[allow(clippy::type_complexity)]
pub(crate) fn configure_fallout_surface_materials(
    mut commands: Commands,
    extras: Query<
        (
            Entity,
            &MeshMaterial3d<StandardMaterial>,
            &GltfMaterialExtras,
            Option<&GltfMeshName>,
        ),
        (With<Mesh3d>, Without<FalloutSurfaceConfigured>),
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, material_handle, extras, mesh_name) in &extras {
        let Some(metadata) = parse_fallout_material_extra(&extras.value) else {
            commands.entity(entity).insert(FalloutSurfaceConfigured);
            continue;
        };
        let surface_kind = fallout_surface_kind(&metadata, mesh_name.map(|name| name.0.as_str()));
        let Some(mut material) = materials.get_mut(&material_handle.0) else {
            // The scene entity can arrive before its material asset. Leave it
            // unmarked so the next frame can configure it once loaded.
            continue;
        };

        match surface_kind {
            FALLOUT_SURFACE_HAIR => {
                material.fallout_surface_kind = FALLOUT_SURFACE_HAIR;
                // This also selects Bevy's tangent-capable PBR variant. The
                // shader falls back to GGX if the mesh has no tangent data.
                material.anisotropy_strength = FALLOUT_HAIR_ANISOTROPY_STRENGTH;
                material.perceptual_roughness = material
                    .perceptual_roughness
                    .max(FALLOUT_HAIR_MIN_ROUGHNESS);
                material.reflectance = material.reflectance.min(FALLOUT_HAIR_REFLECTANCE);
            }
            FALLOUT_SURFACE_EYE => {
                material.fallout_surface_kind = FALLOUT_SURFACE_EYE;
                material.clearcoat = FALLOUT_EYE_CLEARCOAT;
                material.clearcoat_perceptual_roughness = FALLOUT_EYE_CLEARCOAT_ROUGHNESS;
            }
            FALLOUT_SURFACE_SKIN => {
                material.fallout_surface_kind = FALLOUT_SURFACE_SKIN;
                material.perceptual_roughness = material
                    .perceptual_roughness
                    .max(FALLOUT_SKIN_MIN_ROUGHNESS);
                material.reflectance = material.reflectance.min(FALLOUT_SKIN_REFLECTANCE);
            }
            _ => {}
        }

        commands.entity(entity).insert(FalloutSurfaceConfigured);
    }
}

/// Registers the shader-authorized material handles used by the live
/// `setrender emission` control. GLB extras are the cross-backend contract;
/// materials without an explicit authorization marker are left untouched.
#[allow(clippy::type_complexity)]
pub(crate) fn configure_fallout_emission(
    extras: Query<
        (&MeshMaterial3d<StandardMaterial>, &GltfMaterialExtras),
        Or<(
            Changed<MeshMaterial3d<StandardMaterial>>,
            Changed<GltfMaterialExtras>,
        )>,
    >,
    mut authorized: ResMut<AuthorizedEmissionMaterials>,
) {
    for (material_handle, extras) in &extras {
        let Some(metadata) = parse_fallout_material_extra(&extras.value) else {
            continue;
        };
        let Some(emission_authorized) = metadata.emission_authorized else {
            continue;
        };
        authorized.set(material_handle.0.id(), emission_authorized);
    }
}

/// Connects the bake's local-thickness map to Bevy's cheap diffuse-transmission
/// lobe. Bevy loads `KHR_materials_volume` into `thickness_texture`, while the
/// Fallout extras carry the authored transmission strength and identify the
/// material as eligible for this bridge.
#[allow(clippy::type_complexity)]
pub(crate) fn configure_fallout_translucency(
    mut commands: Commands,
    extras: Query<
        (
            Entity,
            &MeshMaterial3d<StandardMaterial>,
            &GltfMaterialExtras,
        ),
        (With<Mesh3d>, Without<FalloutMaterialConfigured>),
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, material_handle, extras) in &extras {
        let Some(metadata) = parse_fallout_material_extra(&extras.value) else {
            // This entity has stable glTF extras, but they do not carry the
            // Fallout material contract. Mark it terminally inspected so
            // ordinary materials do not incur a JSON parse every frame.
            commands.entity(entity).insert(FalloutMaterialConfigured);
            continue;
        };
        let Some(local_thickness) = metadata.local_thickness else {
            // Fallout metadata without local-thickness data has no
            // translucency work to perform.
            commands.entity(entity).insert(FalloutMaterialConfigured);
            continue;
        };
        if !metadata.translucency_enabled || !local_thickness.enabled {
            commands.entity(entity).insert(FalloutMaterialConfigured);
            continue;
        }
        let strength = if local_thickness.strength.is_finite() && local_thickness.strength > 0.0 {
            local_thickness.strength
        } else {
            metadata.translucency_strength
        }
        .clamp(0.0, 1.0);
        let Some(mut material) = materials.get_mut(&material_handle.0) else {
            continue;
        };
        let Some(thickness_texture) = material.thickness_texture.clone() else {
            warn!(
                "Fallout translucency metadata has no loaded thickness texture on entity {:?}",
                entity
            );
            commands.entity(entity).insert(FalloutMaterialConfigured);
            continue;
        };
        material.diffuse_transmission = strength;
        material.diffuse_transmission_texture = Some(thickness_texture);
        commands.entity(entity).insert(FalloutMaterialConfigured);
        info!(
            "configured Fallout local translucency entity {:?} strength={:.3}",
            entity, strength
        );
    }
}

/// Marks meshes under the prepared scene and startup placement root as
/// receivers of the prepared cubemap. The combined static scene is removed
/// from Bevy's per-frame caster pass; individual physics placements remain
/// runtime casters so their shadows can land on prepared receivers.
pub(crate) fn mark_prepared_shadow_meshes(
    mut commands: Commands,
    roots: Query<(Entity, Has<BakedStaticSceneRoot>), With<PreparedPointShadowReceiverRoot>>,
    parents: Query<&ChildOf>,
    meshes: Query<Entity, Added<Mesh3d>>,
) {
    let roots = roots.iter().collect::<HashMap<_, _>>();
    for mesh in &meshes {
        let mut ancestor = mesh;
        let mut root_kind = None;
        loop {
            if let Some(is_baked_static) = roots.get(&ancestor) {
                root_kind = Some(*is_baked_static);
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        let Some(is_baked_static) = root_kind else {
            continue;
        };

        let mut entity = commands.entity(mesh);
        entity.insert(bevy::pbr::BakedPointShadowReceiver);
        if is_baked_static {
            entity.insert(NotShadowCaster);
        }
    }
}

fn diagnostic_lightmap_binding_id(extras: &GltfExtras) -> Option<u32> {
    let root: serde_json::Value = serde_json::from_str(&extras.value).ok()?;
    root.get("bevyout")?
        .get("lightmap_binding")?
        .as_u64()
        .and_then(|binding| u32::try_from(binding).ok())
}

/// Attaches prepared HDR surface lightmaps to generated static meshes. The
/// binding ID comes from primitive extras, so Bevy entity or mesh ordering is
/// never used to associate an atlas page with a primitive.
#[allow(clippy::type_complexity)]
pub(crate) fn attach_prepared_lightmaps(
    mut commands: Commands,
    prepared: Option<Res<PreparedLightmap>>,
    roots: Query<Entity, With<BakedStaticSceneRoot>>,
    parents: Query<&ChildOf>,
    meshes: Query<
        (Entity, Option<&GltfExtras>),
        (
            With<Mesh3d>,
            With<MeshMaterial3d<StandardMaterial>>,
            Without<Lightmap>,
            Without<PreparedLightmapAttached>,
        ),
    >,
) {
    let Some(prepared) = prepared else {
        return;
    };
    let roots = roots.iter().collect::<HashSet<_>>();
    for (entity, extras) in &meshes {
        let mut ancestor = entity;
        let mut under_baked_root = false;
        loop {
            if roots.contains(&ancestor) {
                under_baked_root = true;
                break;
            }
            let Ok(parent) = parents.get(ancestor) else {
                break;
            };
            ancestor = parent.parent();
        }
        if !under_baked_root {
            continue;
        }
        let Some(binding_id) = extras.and_then(diagnostic_lightmap_binding_id) else {
            continue;
        };
        let Some(binding) = prepared.bindings.get(&binding_id) else {
            warn!("GLB lightmap binding {binding_id} is absent from PreparedBake");
            continue;
        };
        commands.entity(entity).insert((
            Lightmap {
                image: binding.image.clone(),
                uv_rect: binding.uv_rect,
                bicubic_sampling: false,
            },
            PreparedLightmapAttached,
        ));
    }
}

const DEFAULT_BLOOM_INTENSITY: f32 = 0.2;
const DEFAULT_BLOOM_THRESHOLD: f32 = 0.05;
const DEFAULT_BLOOM_SOFTNESS: f32 = 0.2;
const IMAGE_SPACE_NEUTRAL_BRIGHT_CLAMP: f32 = 0.225;
const IMAGE_SPACE_BLOOM_RADIUS_RANGE: f32 = 8.0;
const VOLUMETRIC_FOG_VOLUME_SIZE: f32 = 4096.0;
const VOLUMETRIC_FOG_STEP_COUNT: u32 = 64;
const VOLUMETRIC_FOG_ABSORPTION: f32 = 0.3;
const VOLUMETRIC_FOG_SCATTERING: f32 = 0.3;
const VOLUMETRIC_FOG_MAX_DENSITY: f32 = 0.1;

pub(crate) fn image_space_bloom_values(
    image_space: Option<&ImageSpaceInfo>,
    interior: bool,
) -> (f32, f32, f32) {
    let Some(image_space) = image_space else {
        return (
            DEFAULT_BLOOM_INTENSITY,
            DEFAULT_BLOOM_THRESHOLD,
            DEFAULT_BLOOM_SOFTNESS,
        );
    };

    let authored_alpha = if interior {
        image_space.bloom_alpha_mult_interior
    } else {
        image_space.bloom_alpha_mult_exterior
    };
    let alpha = if authored_alpha.is_finite() {
        authored_alpha
    } else {
        1.0
    }
    .clamp(0.0, 1.0);
    let bright_scale = if image_space.hdr_bright_scale.is_finite() {
        image_space.hdr_bright_scale
    } else {
        1.0
    }
    .max(0.0);
    let bright_clamp = if image_space.hdr_bright_clamp.is_finite() {
        image_space.hdr_bright_clamp
    } else {
        DEFAULT_BLOOM_THRESHOLD
    }
    .max(DEFAULT_BLOOM_THRESHOLD);
    let blur_radius = if image_space.bloom_blur_radius.is_finite() {
        image_space.bloom_blur_radius
    } else {
        0.0
    }
    .max(0.0);

    // Keep the established viewer bloom as the neutral profile. ImageSpace
    // values modulate that profile instead of replacing it with raw Fallout
    // HDR magnitudes; this keeps the cell pipeline from undoing the tuned
    // `.2/.05/.2` defaults while still honoring authored bloom controls.
    let intensity = DEFAULT_BLOOM_INTENSITY * (alpha * bright_scale).clamp(0.0, 1.0);
    let threshold = DEFAULT_BLOOM_THRESHOLD
        * (bright_clamp / IMAGE_SPACE_NEUTRAL_BRIGHT_CLAMP).clamp(0.25, 4.0);
    let softness = (DEFAULT_BLOOM_SOFTNESS
        * (1.0 + (blur_radius / IMAGE_SPACE_BLOOM_RADIUS_RANGE).clamp(0.0, 1.0)))
    .clamp(0.0, 1.0);

    (intensity, threshold, softness)
}

pub(crate) fn fallout_bloom_for(
    image_space: Option<&ImageSpaceInfo>,
    interior: bool,
    overrides: &ImageSpaceBloomOverrides,
) -> Bloom {
    let (intensity, threshold, softness) = image_space_bloom_values(image_space, interior);
    Bloom {
        intensity: overrides.intensity.unwrap_or(intensity),
        prefilter: BloomPrefilter {
            threshold: overrides.threshold.unwrap_or(threshold),
            threshold_softness: overrides.softness.unwrap_or(softness),
        },
        composite_mode: BloomCompositeMode::Additive,
        ..Bloom::OLD_SCHOOL
    }
}

#[cfg(test)]
pub(super) fn fallout_bloom() -> Bloom {
    fallout_bloom_for(None, true, &ImageSpaceBloomOverrides::default())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_prepared_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
    manifest: Res<crate::viewer::LoadedSceneManifest>,
    image_space_bloom_overrides: Res<ImageSpaceBloomOverrides>,
    lighting: Res<LightingScale>,
    ambient_scale: Res<AmbientScale>,
    fog_strength: Res<FogStrength>,
    mut references: ResMut<crate::console::RefRegistry>,
    mut resident_cells: ResMut<ResidentCells>,
    saved_location: Option<Res<super::world::CurrentWorldLocation>>,
) {
    let focus = scene_focus(&manifest);
    let saved_location = saved_location.as_deref().and_then(|location| {
        location.0.as_ref().and_then(|location| match location {
            bevyout_core::manifest::exterior::WorldLocation::Exterior(location)
                if manifest.exterior.as_ref().is_some_and(|exterior| {
                    exterior.worldspace_form_id == location.worldspace_form_id
                }) =>
            {
                Some((location.position, location.rotation_xyzw))
            }
            bevyout_core::manifest::exterior::WorldLocation::Interior(location)
                if manifest.cell.form_id == location.cell_form_id =>
            {
                Some((location.position, location.rotation_xyzw))
            }
            _ => None,
        })
    });
    let initial_camera_transform = saved_location
        .map(|(position, rotation_xyzw)| {
            Transform::from_translation(
                Vec3::from_array(position)
                    + Vec3::Y
                        * (crate::viewer::player::EYE_HEIGHT
                            - crate::viewer::player::CAPSULE_HEIGHT * 0.5),
            )
            .with_rotation(Quat::from_xyzw(
                rotation_xyzw[0],
                rotation_xyzw[1],
                rotation_xyzw[2],
                rotation_xyzw[3],
            ))
        })
        .or_else(|| {
            transition_camera_position(&manifest)
                .map(|position| Transform::from_translation(position).looking_at(focus, Vec3::Y))
        })
        .unwrap_or_else(|| {
            Transform::from_translation(focus + Vec3::new(0.0, 4.0, 12.0))
                .looking_at(focus, Vec3::Y)
        });
    let (initial_yaw, initial_pitch, _) = initial_camera_transform.rotation.to_euler(EulerRot::YXZ);
    let cell_lighting = effective_lighting(&manifest.cell);
    let lightmapped_diffuse_enabled = runtime_lightmapped_diffuse_enabled(manifest.bake.as_ref());
    let (color_grading, auto_exposure) =
        camera_post_processing(manifest.cell.image_space.as_ref(), &mut compensation_curves);
    let mut camera = commands.spawn((
        Camera3d::default(),
        Projection::Perspective(default_perspective_projection()),
        HorizontalFov::default(),
        ShadowFilteringMethod::Hardware2x2,
        DepthPrepass,
        OcclusionCulling,
        fallout_bloom_for(
            manifest.cell.image_space.as_ref(),
            manifest.cell.interior,
            &image_space_bloom_overrides,
        ),
        Tonemapping::AgX,
        Exposure { ev100: 12.0 },
        color_grading,
        initial_camera_transform,
        FlyCamera {
            yaw: initial_yaw,
            pitch: initial_pitch,
            speed: 8.0,
        },
    ));
    camera.insert((
        ChromaticAberration {
            intensity: 0.0,
            ..default()
        },
        LensDistortion {
            intensity: 0.0,
            ..default()
        },
        Vignette {
            intensity: 0.0,
            ..default()
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
        affects_lightmapped_meshes: lightmapped_diffuse_enabled,
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
                affects_lightmapped_mesh_diffuse: lightmapped_diffuse_enabled,
                shadow_maps_enabled: false,
                ..default()
            },
            // Bevy's volumetric pass is activated by a volumetric light. This
            // directional light has no runtime shadow map, so it contributes
            // the cell's ambient fog without enabling another shadow pass.
            VolumetricLight,
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
        if !bake.lightmaps.is_empty() && !bake.lightmap_bindings.is_empty() {
            let atlas_images = bake
                .lightmaps
                .iter()
                .enumerate()
                .map(|(index, atlas)| {
                    (
                        u16::try_from(index).unwrap_or(u16::MAX),
                        asset_server.load::<Image>(atlas.asset_path.clone()),
                    )
                })
                .collect::<HashMap<_, _>>();
            let mut bindings = HashMap::new();
            for binding in &bake.lightmap_bindings {
                let Some(image) = atlas_images.get(&binding.atlas_index) else {
                    warn!(
                        "PreparedBake lightmap binding {} references missing atlas {}",
                        binding.binding_id, binding.atlas_index
                    );
                    continue;
                };
                bindings.insert(
                    binding.binding_id,
                    PreparedLightmapBinding {
                        image: image.clone(),
                        uv_rect: Rect::from_corners(
                            Vec2::new(binding.uv_rect[0], binding.uv_rect[1]),
                            Vec2::new(binding.uv_rect[2], binding.uv_rect[3]),
                        ),
                    },
                );
            }
            if !bindings.is_empty() {
                commands.insert_resource(PreparedLightmap { bindings });
            }
        }
        let mut baked_root = commands.spawn((
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(bake.scene_path.clone())),
            ),
            BakedStaticSceneRoot,
        ));
        if prepared_shadow_available {
            baked_root.insert(PreparedPointShadowReceiverRoot);
        }
        if let Some(volume) = &bake.irradiance_volume {
            commands.spawn((
                LightProbe::default(),
                IrradianceVolume {
                    voxels: asset_server.load(volume.asset_path.clone()),
                    intensity: volume.intensity,
                    affects_lightmapped_meshes: lightmapped_diffuse_enabled,
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
    spawn_cell_reflection_probes(&mut commands, &asset_server, &manifest, root);
    for light in &manifest.lights {
        if !light.initially_enabled {
            continue;
        }
        let is_spot = prepared_light_is_spot(light);
        let mut light_entity = commands.spawn((
            PreparedPointLightIntensity {
                radius: light.radius,
                intensity_lumens: light.intensity_lumens,
            },
            Transform::from_translation(Vec3::from_array(light.translation))
                .with_rotation(Quat::from_array(light.rotation_xyzw)),
            ChildOf(root),
        ));
        if is_spot {
            let (inner_angle, outer_angle) = prepared_spot_angles(light);
            light_entity.insert(SpotLight {
                intensity: point_light_intensity(light.radius, light.intensity_lumens, lighting.0),
                range: light.radius,
                color: Color::srgb(
                    light.color_rgba[0],
                    light.color_rgba[1],
                    light.color_rgba[2],
                ),
                inner_angle,
                outer_angle,
                affects_lightmapped_mesh_diffuse: lightmapped_diffuse_enabled,
                shadow_maps_enabled: false,
                ..default()
            });
        } else {
            light_entity.insert((
                PointLight {
                    intensity: point_light_intensity(
                        light.radius,
                        light.intensity_lumens,
                        lighting.0,
                    ),
                    range: light.radius,
                    color: Color::srgb(
                        light.color_rgba[0],
                        light.color_rgba[1],
                        light.color_rgba[2],
                    ),
                    affects_lightmapped_mesh_diffuse: lightmapped_diffuse_enabled,
                    shadow_maps_enabled: false,
                    ..default()
                },
                RealtimeShadowCandidate {
                    reference_form_id: light.reference_form_id,
                },
            ));
        }
        if !is_spot && let Some(shadow) = prepared_shadow_records.get(&light.reference_form_id) {
            light_entity.insert(BakedPointLightShadow {
                layer: shadow.layer,
                baked_translation: Vec3::from_array(shadow.translation),
                baked_range: shadow.range,
                near_z: manifest
                    .static_point_shadows
                    .as_ref()
                    .map_or(0.1, |artifact| artifact.near_z),
            });
            attached_shadow_lights += 1;
        }
    }
    commands.insert_resource(PreparedPointShadowRuntime {
        attached_lights: attached_shadow_lights,
        ..prepared_shadow_runtime
    });
    let (content, _next) = spawn_cell_placements_chunk(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
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
            manifest: Arc::new((**manifest).clone()),
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

pub(crate) fn spawn_cell_reflection_probes(
    commands: &mut Commands,
    asset_server: &AssetServer,
    manifest: &crate::vsa::PreparedSceneManifest,
    root: Entity,
) {
    let Some(probe_set) = manifest.reflection_probes.as_ref() else {
        return;
    };
    for probe in &probe_set.probes {
        let half_extents = Vec3::from_array(probe.influence_half_extents).max(Vec3::splat(0.01));
        let scale = half_extents * 2.0;
        let parallax_world = Vec3::from_array(probe.parallax_half_extents).max(Vec3::splat(0.01));
        let parallax_local = parallax_world / scale;
        commands.spawn((
            PreparedReflectionProbe,
            LightProbe {
                falloff: Vec3::from_array(probe.falloff),
            },
            EnvironmentMapLight {
                diffuse_map: asset_server.load(probe.diffuse_asset_path.clone()),
                specular_map: asset_server.load(probe.specular_asset_path.clone()),
                intensity: PREPARED_REFLECTION_PROBE_INTENSITY * DEFAULT_REFLECTION_PROBE_STRENGTH,
                affects_lightmapped_mesh_diffuse: false,
                ..default()
            },
            ParallaxCorrection::Custom(parallax_local),
            Transform::from_translation(Vec3::from_array(probe.capture_translation))
                .with_scale(scale),
            ChildOf(root),
        ));
    }
    info!(
        "reflection probes: loaded {} prepared probe(s) for {}",
        probe_set.probes.len(),
        cell_label(&manifest.cell)
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
    let lightmapped_diffuse_enabled = runtime_lightmapped_diffuse_enabled(manifest.bake.as_ref());
    for light in &manifest.lights {
        if !light.initially_enabled {
            continue;
        }
        let is_spot = prepared_light_is_spot(light);
        let mut light_entity = commands.spawn((
            PreparedPointLightIntensity {
                radius: light.radius,
                intensity_lumens: light.intensity_lumens,
            },
            ChildOf(root),
        ));
        if is_spot {
            let (inner_angle, outer_angle) = prepared_spot_angles(light);
            light_entity.insert((
                SpotLight {
                    intensity: point_light_intensity(
                        light.radius,
                        light.intensity_lumens,
                        lighting_scale,
                    ),
                    range: light.radius,
                    color: Color::srgb(
                        light.color_rgba[0],
                        light.color_rgba[1],
                        light.color_rgba[2],
                    ),
                    inner_angle,
                    outer_angle,
                    affects_lightmapped_mesh_diffuse: lightmapped_diffuse_enabled,
                    shadow_maps_enabled: false,
                    ..default()
                },
                Transform::from_translation(Vec3::from_array(light.translation))
                    .with_rotation(Quat::from_array(light.rotation_xyzw)),
            ));
        } else {
            light_entity.insert((
                PointLight {
                    intensity: point_light_intensity(
                        light.radius,
                        light.intensity_lumens,
                        lighting_scale,
                    ),
                    range: light.radius,
                    color: Color::srgb(
                        light.color_rgba[0],
                        light.color_rgba[1],
                        light.color_rgba[2],
                    ),
                    affects_lightmapped_mesh_diffuse: lightmapped_diffuse_enabled,
                    shadow_maps_enabled: false,
                    ..default()
                },
                Transform::from_translation(Vec3::from_array(light.translation))
                    .with_rotation(Quat::from_array(light.rotation_xyzw)),
            ));
        }
    }
}

/// Spawns up to `max_entries` raw `manifest.placements` entries starting at
/// index `start`, returning what was spawned and the next raw index (equal
/// to `manifest.placements.len()` once the cell is fully spawned). The
/// preloader drains large cells through this a bounded chunk per frame so a
/// background preload never spawns a thousand entities in one frame spike;
/// callers wanting everything at once pass `usize::MAX`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn spawn_cell_placements_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
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
        // A missing mesh must not erase a living actor's stable reference.
        // ActorPlugin will project its prepared identity and spawn the bounds
        // proxy selected by the fallback policy. Other asset-less placements
        // retain the pre-wave behavior and are skipped -- except a
        // source-authored dead actor (issue #120, `PreparedSemantic::Corpse`,
        // outside `is_actor_semantic`): it has no resolved GLB and no
        // ActorPlugin projection, but must still be present and
        // raycast-targetable, so it gets the placeholder body.
        if placement.asset_path.is_none() && !super::actor::is_actor_semantic(&placement.semantic) {
            if placement.semantic == PreparedSemantic::Corpse {
                let entity = spawn_corpse_placeholder(commands, meshes, materials, placement, root);
                register_placement_reference(references.as_deref_mut(), entity, placement);
                placement_count += 1;
            }
            continue;
        }
        let mut entity_commands = commands.spawn((
            interaction::PlacementRoot::new(placement.clone()),
            Transform {
                translation: Vec3::from_array(placement.translation),
                rotation: Quat::from_xyzw(
                    placement.rotation_xyzw[0],
                    placement.rotation_xyzw[1],
                    placement.rotation_xyzw[2],
                    placement.rotation_xyzw[3],
                ),
                scale: Vec3::splat(super::actor::placement_root_scale(placement)),
            },
            ChildOf(root),
        ));
        if let Some(path) = placement.asset_path.as_ref() {
            let handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone()));
            entity_commands.insert(WorldAssetRoot(handle.clone()));
            scene_handles.push(handle);
        }
        let entity = entity_commands.id();
        if let Some(references) = references.as_deref_mut() {
            references.register(
                entity,
                placement.reference_form_id,
                placement.editor_id.as_deref(),
            );
        }
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

fn register_placement_reference(
    references: Option<&mut crate::console::RefRegistry>,
    entity: Entity,
    placement: &PreparedPlacement,
) {
    if let Some(references) = references {
        references.register(
            entity,
            placement.reference_form_id,
            placement.editor_id.as_deref(),
        );
    }
}

/// Prone capsule roughly matching human proportions (radius plus cylinder
/// length totalling ~1.7 m).
const CORPSE_PLACEHOLDER_RADIUS: f32 = 0.25;
const CORPSE_PLACEHOLDER_CYLINDER_LENGTH: f32 = 1.2;

/// A source-authored dead actor (issue #120, #118's
/// `PreparedSemantic::Corpse`) has no resolved GLB. Until #106-#108 land
/// real skeleton+parts actor bodies, this gives the corpse a real `Mesh3d`
/// so it is present and raycast-targetable (`update_focused_placement` in
/// `interaction.rs` is `MeshRayCast`-only) through the exact same
/// `PlacementRoot`/`ChildOf` wiring every resolved-asset placement gets.
// ponytail: placeholder prone primitive until #106-#108 actor bodies
fn spawn_corpse_placeholder(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    placement: &PreparedPlacement,
    root: Entity,
) -> Entity {
    let mesh = meshes.add(Capsule3d::new(
        CORPSE_PLACEHOLDER_RADIUS,
        CORPSE_PLACEHOLDER_CYLINDER_LENGTH,
    ));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.18, 0.16),
        perceptual_roughness: 0.95,
        ..default()
    });
    let placement_rotation = Quat::from_xyzw(
        placement.rotation_xyzw[0],
        placement.rotation_xyzw[1],
        placement.rotation_xyzw[2],
        placement.rotation_xyzw[3],
    );
    // `Capsule3d` stands upright along local Y by default; rotating 90
    // degrees about X lays it down flat (prone) in local space first, then
    // the placement's own yaw carries that orientation to the authored
    // facing direction.
    let rotation = placement_rotation * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    // Lifted by one radius so the lying body's underside rests on the
    // ground plane instead of clipping through it -- placements carry a
    // feet/ground-level origin.
    let translation = Vec3::from_array(placement.translation) + Vec3::Y * CORPSE_PLACEHOLDER_RADIUS;
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform {
                translation,
                rotation,
                scale: Vec3::splat(placement.scale),
            },
            interaction::PlacementRoot::new(placement.clone()),
            ChildOf(root),
        ))
        .id()
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
    let (start, end) = fog_world_range(lighting)?;
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

fn fog_world_range(lighting: &PreparedCellLighting) -> Option<(f32, f32)> {
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
    Some((start, end))
}

/// Converts the existing Fallout fog level into a uniform volumetric density.
/// `FogStrength` is the live viewer multiplier and the cell supplies the fog
/// range/color. Density is chosen so the volume reaches approximately the same
/// opacity at the cell's fog end distance as the current distance-fog alpha.
pub(crate) fn volumetric_fog_density(
    lighting: &PreparedCellLighting,
    strength: f32,
    multiplier: f32,
) -> Option<f32> {
    let (_, end) = fog_world_range(lighting)?;
    if !strength.is_finite() || strength < 0.0 || !multiplier.is_finite() || multiplier < 0.0 {
        return None;
    }
    let opacity = (strength * multiplier).clamp(0.0, 1.0);
    if opacity <= 0.0 {
        return Some(0.0);
    }
    let density = if opacity >= 1.0 {
        // A fully opaque target has infinite optical depth mathematically;
        // use the bounded runtime density instead of rejecting the profile.
        VOLUMETRIC_FOG_MAX_DENSITY
    } else {
        let optical_depth = -(1.0 - opacity).ln();
        optical_depth / (end * (VOLUMETRIC_FOG_ABSORPTION + VOLUMETRIC_FOG_SCATTERING))
    };
    density
        .is_finite()
        .then_some(density.clamp(0.0, VOLUMETRIC_FOG_MAX_DENSITY))
}

pub(crate) fn volumetric_fog_profile(
    lighting: &PreparedCellLighting,
    strength: f32,
    multiplier: f32,
) -> Option<(VolumetricFog, FogVolume)> {
    let density = volumetric_fog_density(lighting, strength, multiplier)?;
    let fog_color = Color::srgb(
        lighting.fog_rgba[0],
        lighting.fog_rgba[1],
        lighting.fog_rgba[2],
    );
    let strength = (strength * multiplier).clamp(0.0, 1.0);
    Some((
        VolumetricFog {
            ambient_color: fog_color,
            ambient_intensity: strength,
            jitter: 0.0,
            step_count: VOLUMETRIC_FOG_STEP_COUNT,
        },
        FogVolume {
            fog_color,
            density_factor: density,
            absorption: VOLUMETRIC_FOG_ABSORPTION,
            scattering: VOLUMETRIC_FOG_SCATTERING,
            ..default()
        },
    ))
}

pub(crate) fn apply_fog_strength(
    fog_strength: Res<FogStrength>,
    manifest: Res<crate::viewer::LoadedSceneManifest>,
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

#[allow(clippy::type_complexity)]
pub(crate) fn apply_volumetric_fog(
    mut commands: Commands,
    fog_strength: Res<FogStrength>,
    fog_multiplier: Res<VolumetricFogMultiplier>,
    manifest: Res<crate::viewer::LoadedSceneManifest>,
    mut cameras: Query<(Entity, Option<&mut VolumetricFog>), With<Camera3d>>,
    mut volumes: Query<
        (Entity, Option<&mut FogVolume>, Option<&VolumetricLight>),
        With<CellVolumetricFog>,
    >,
) {
    if !fog_strength.is_changed() && !fog_multiplier.is_changed() && !manifest.is_changed() {
        return;
    }
    let lighting = effective_lighting(&manifest.cell);
    let profile = volumetric_fog_profile(&lighting, fog_strength.0, fog_multiplier.0);
    let Some((camera_entity, camera_fog)) = cameras.iter_mut().next() else {
        return;
    };

    if let Some((volumetric_fog, volume_fog)) = profile {
        if let Some(mut camera_fog) = camera_fog {
            *camera_fog = volumetric_fog;
        } else {
            commands.entity(camera_entity).insert(volumetric_fog);
        }

        let mut volume_exists = false;
        for (volume_entity, volume, volumetric_light) in &mut volumes {
            volume_exists = true;
            if volumetric_light.is_none() {
                commands.entity(volume_entity).insert(VolumetricLight);
            }
            if let Some(mut volume) = volume {
                *volume = volume_fog.clone();
            } else {
                commands.entity(volume_entity).insert(volume_fog.clone());
            }
        }
        if !volume_exists {
            commands.spawn((
                CellVolumetricFog,
                // Bevy 0.19 gates its volumetric post-process on the
                // presence of a `VolumetricLight`. Keep the gate with the
                // camera-following volume so ambient-only cells still fog;
                // actual directional lights are marked separately when they
                // exist.
                VolumetricLight,
                volume_fog,
                Transform::from_scale(Vec3::splat(VOLUMETRIC_FOG_VOLUME_SIZE)),
                ChildOf(camera_entity),
            ));
        }
    } else {
        commands.entity(camera_entity).remove::<VolumetricFog>();
        for (volume_entity, _, _) in &mut volumes {
            commands
                .entity(volume_entity)
                .remove::<(FogVolume, VolumetricLight)>();
        }
    }
}

/// Issue #52: re-derives ambient light, camera fog, and the directional
/// light from whatever `PreparedSceneManifest` is currently the active
/// resource (already repointed to the destination cell by the caller),
/// applying them directly to the existing camera/light entities and the
/// `GlobalAmbientLight` resource rather than spawning new ones.
/// `apply_fog_strength` only reacts to `FogStrength` changing, so a cell
/// swap needs its own explicit refresh to pick up the new cell's lighting.
/// Camera post-processing follows the active cell's ImageSpace. Explicit
/// bloom console overrides are layered on top so live tuning survives a cell
/// swap.
pub(crate) fn refresh_environment_for_active_cell(world: &mut World) {
    let cell = world
        .resource::<crate::viewer::LoadedSceneManifest>()
        .cell
        .clone();
    let lighting_scale = world.resource::<LightingScale>().0;
    let ambient_scale = world.resource::<AmbientScale>().0;
    let fog_strength = world.resource::<FogStrength>().0;
    let lightmapped_diffuse_enabled = runtime_lightmapped_diffuse_enabled(
        world
            .resource::<crate::viewer::LoadedSceneManifest>()
            .bake
            .as_ref(),
    );
    let cell_lighting = effective_lighting(&cell);
    world.insert_resource(image_space_emission_multiplier(cell.image_space.as_ref()));

    world.insert_resource(GlobalAmbientLight {
        color: Color::srgb(
            cell_lighting.ambient_rgba[0],
            cell_lighting.ambient_rgba[1],
            cell_lighting.ambient_rgba[2],
        ),
        brightness: 25.0 * lighting_scale * ambient_scale,
        affects_lightmapped_meshes: lightmapped_diffuse_enabled,
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
        light.affects_lightmapped_mesh_diffuse = lightmapped_diffuse_enabled;
        cell_light.base_illuminance = base_illuminance;
        transform.rotation = Quat::from_array(cell_lighting.directional_rotation_xyzw());
    }

    refresh_camera_post_processing(world, &cell);
}

pub(crate) fn refresh_camera_post_processing(world: &mut World, cell: &CellInfo) {
    if !world.contains_resource::<Assets<AutoExposureCompensationCurve>>() {
        crate::viewer::screen_fx::refresh_base(world, cell);
        return;
    }

    let (color_grading, auto_exposure) = {
        let mut compensation_curves = world.resource_mut::<Assets<AutoExposureCompensationCurve>>();
        camera_post_processing(cell.image_space.as_ref(), &mut compensation_curves)
    };
    let overrides = world
        .get_resource::<ImageSpaceBloomOverrides>()
        .copied()
        .unwrap_or_default();
    let bloom = fallout_bloom_for(cell.image_space.as_ref(), cell.interior, &overrides);
    let camera = {
        let mut cameras = world.query_filtered::<Entity, With<Camera3d>>();
        cameras.iter(world).next()
    };
    let Some(camera) = camera else {
        return;
    };

    let mut camera = world.entity_mut(camera);
    camera.insert((color_grading, bloom));
    if let Some(auto_exposure) = auto_exposure {
        camera.insert(auto_exposure);
    } else {
        camera.remove::<AutoExposure>();
    }
    crate::viewer::screen_fx::refresh_base(world, cell);
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
        color_grading.global.exposure = image_space.cinematic_brightness.max(0.0001).log2();
    }
    if flags & 0x01 != 0 {
        color_grading.global.post_saturation = image_space.cinematic_saturation.max(0.0);
    }
    if flags & 0x02 != 0 {
        let contrast = image_space.cinematic_contrast.max(0.01);
        let contrast_pivot = if image_space.cinematic_contrast_avg_lum.is_finite() {
            image_space.cinematic_contrast_avg_lum.clamp(0.001, 1.0)
        } else {
            0.18
        };
        // Bevy's linear contrast stage is hard-coded around 0.5 and runs
        // before auto-exposure. Fallout's authored average luminance is a
        // much lower pivot (commonly around 0.14), so feeding its multiplier
        // to Bevy directly clips almost the whole HDR interior below zero.
        // A power curve preserves positive HDR values and the authored pivot:
        //     output = pivot * (input / pivot)^contrast
        let gamma = contrast.recip();
        let gain = contrast_pivot.powf((1.0 - contrast) / contrast);
        color_grading.shadows.gamma = gamma;
        color_grading.midtones.gamma = gamma;
        color_grading.highlights.gamma = gamma;
        color_grading.shadows.gain = gain;
        color_grading.midtones.gain = gain;
        color_grading.highlights.gain = gain;
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
    // Fallout authors this value as the opacity of a cinematic color
    // overlay. Bevy exposes chromatic white-balance offsets instead, whose
    // perceptual response is much weaker for the same normalized value.
    // Calibrate the normalized overlay strength to the closest matching
    // white-balance response while keeping malformed values bounded.
    let strength = if strength.is_finite() {
        strength.clamp(0.0, 1.0) * 4.0
    } else {
        0.0
    };
    Some((
        (target_x - 0.3127) * strength,
        (0.3290 - target_y) * strength,
    ))
}

pub(crate) type GlowCardMeshQuery<'w> = (Entity, &'w GltfMeshName);

pub(crate) fn configure_glow_cards(
    mut commands: Commands,
    meshes: Query<GlowCardMeshQuery<'_>, (With<Mesh3d>, Without<GlowCardInspected>)>,
) {
    for (entity, name) in &meshes {
        // Mark every inspected mesh. Component markers despawn with their
        // entity, so unlike the old `Local<HashSet<Entity>>`/count-sentinel
        // pair this can neither leak stale entries nor skip a mesh spawned
        // while another despawned between frames (issue #270).
        commands.entity(entity).insert(GlowCardInspected);
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

/// Issue #270 (PERF wave 1): every mesh entity that has been glow-card
/// classified carries this marker, so the `configure_glow_cards` query
/// filters to not-yet-inspected entities without a per-frame count
/// sentinel or a `Local<HashSet<Entity>>` (markers despawn cleanly with
/// their entity, closing the remove+add count-coincidence blind spot).
#[derive(Component)]
pub(crate) struct GlowCardInspected;

// The naming decision itself lives in the Bevy-free `glow_card_policy`
// module so the executable spec can drive it verbatim (issue #270).
pub(crate) use super::glow_card_policy::is_glow_card_mesh_name;

pub(crate) fn scene_focus(manifest: &PreparedSceneManifest) -> Vec3 {
    if let Some(package) = manifest.exterior.as_ref() {
        if let Some(terrain) = package
            .terrain
            .as_ref()
            .filter(|terrain| terrain.is_well_formed())
        {
            let width = usize::from(terrain.width);
            let center = terrain.positions[(usize::from(terrain.height) / 2) * width + width / 2];
            return Vec3::from_array(center);
        }
        let span = bevyout_core::manifest::exterior::ExteriorCoordinatePolicy::default()
            .cell_span_metres() as f32;
        return Vec3::new(
            package.origin[0] + span * 0.5,
            package.origin[1],
            package.origin[2] - span * 0.5,
        );
    }
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

#[cfg(test)]
#[path = "tests/scene.rs"]
mod tests;
