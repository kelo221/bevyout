//! Bevy 0.19 Solari scene/proxy and bake-dispatch boundary.
//!
//! The CPU bake frontend remains authoritative. This module owns only the
//! optional render-world adapter: UV-texel/light buffers, a custom compute
//! dispatch over those texels, and an asynchronous readback. It deliberately
//! does not install Solari's realtime ReSTIR plugin.

#![allow(
    dead_code,
    reason = "the opt-in adapter is not part of the default CPU bake path"
)]

use super::super::JobLight;
use super::super::environment::EnvironmentMap;
use super::super::rust_scene::{AlphaMode, ComposedPrimitive, TransportMaterial};
use anyhow::{Context, Result, bail};
use bevy::app::{App, Plugin, SubApps};
use bevy::asset::{
    AssetServer, Assets, Handle, RenderAssetUsages, embedded_asset, load_embedded_asset,
};
use bevy::color::LinearRgba;
use bevy::ecs::resource::Resource;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::{Commands, Res, ResMut};
use bevy::image::{Image, ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
use bevy::material::AlphaMode as BevyAlphaMode;
use bevy::math::Vec3;
use bevy::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Color, DefaultPlugins, Mesh3d, PluginGroup, Transform, default};
use bevy::render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, Buffer,
    BufferDescriptor, BufferInitDescriptor, BufferUsages, CachedComputePipelineId,
    ComputePassDescriptor, ComputePipelineDescriptor, Extent3d, MapMode, PipelineCache,
    ShaderStages, TextureDimension, TextureFormat, binding_types,
};
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy::solari::scene::RaytracingSceneBindings;
use bevy::window::{ExitCondition, WindowPlugin};
use bevy::winit::WinitPlugin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub(crate) fn required_wgpu_features() -> bevy::render::settings::WgpuFeatures {
    bevy::solari::SolariPlugins::required_wgpu_features()
}

/// Installs only Solari's BLAS/TLAS and scene-binding infrastructure. The
/// realtime ReSTIR `SolariLightingPlugin` is intentionally not installed by
/// the future bake app.
pub(crate) fn add_scene_plugin(app: &mut bevy::app::App) {
    app.add_plugins(bevy::solari::scene::RaytracingScenePlugin);
}

/// Adds the bake-only render adapter. Callers must also add
/// [`add_scene_plugin`] so group 0 is populated by Solari's BLAS/TLAS scene
/// systems.
pub(crate) fn add_bake_plugin(app: &mut App) {
    app.add_plugins(SolariBakePlugin);
}

pub(crate) fn raytracing_mesh_component(
    mesh: Handle<Mesh>,
) -> bevy::solari::prelude::RaytracingMesh3d {
    bevy::solari::prelude::RaytracingMesh3d(mesh)
}

/// Builds the Solari-compatible bake proxy for one composed primitive.
///
/// The final GLB remains the authority for raster lightmap rendering and may
/// contain UV1. The proxy intentionally contains only Solari's supported
/// POSITION/NORMAL/UV0/TANGENT attributes and uses an independent mesh asset.
pub(crate) fn build_proxy_mesh(primitive: &ComposedPrimitive) -> Result<Mesh> {
    let vertex_count = primitive.positions.len();
    if vertex_count == 0
        || primitive.normals.len() != vertex_count
        || primitive.uvs.len() != vertex_count
    {
        bail!("Solari proxy attributes have mismatched vertex counts");
    }
    if primitive.indices.is_empty() || !primitive.indices.len().is_multiple_of(3) {
        bail!("Solari proxy requires a non-empty triangle-list index buffer");
    }
    if primitive
        .indices
        .iter()
        .any(|index| *index as usize >= vertex_count)
    {
        bail!("Solari proxy index buffer references a missing vertex");
    }

    let positions = primitive
        .positions
        .iter()
        .map(|position| position.to_array())
        .collect::<Vec<_>>();
    let normals = primitive
        .normals
        .iter()
        .map(|normal| normal.normalize_or_zero().to_array())
        .collect::<Vec<_>>();
    let uvs = primitive
        .uvs
        .iter()
        .map(|uv| uv.to_array())
        .collect::<Vec<_>>();
    let tangents = primitive
        .normals
        .iter()
        .map(|normal| tangent_from_normal(*normal))
        .collect::<Vec<_>>();

    Ok(Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
    .with_inserted_indices(Indices::U32(primitive.indices.clone())))
}

fn tangent_from_normal(normal: Vec3) -> [f32; 4] {
    let normal = normal.normalize_or_zero();
    let reference = if normal.y.abs() < 0.9 {
        Vec3::Y
    } else {
        Vec3::X
    };
    let tangent = normal.cross(reference).normalize_or_zero();
    [tangent.x, tangent.y, tangent.z, 1.0]
}

fn build_alpha_scene(
    primitives: &[ComposedPrimitive],
    materials: &[TransportMaterial],
) -> Result<SolariBakeAlphaScene> {
    let mut records = Vec::with_capacity(primitives.len());
    let mut texels = Vec::new();
    let mut vertex_records = Vec::with_capacity(primitives.len());
    let mut vertex_colors = Vec::new();
    let mut vertex_indices = Vec::new();
    for primitive in primitives {
        if primitive.transport_colors.len() != primitive.positions.len() {
            bail!(
                "Solari transport-color count {} does not match vertex count {} for {}",
                primitive.transport_colors.len(),
                primitive.positions.len(),
                primitive.name
            );
        }
        let color_offset = u32::try_from(vertex_colors.len())
            .context("Solari vertex-color side table exceeded u32 indexing")?;
        let index_offset = u32::try_from(vertex_indices.len())
            .context("Solari vertex-index side table exceeded u32 indexing")?;
        vertex_colors.extend(
            primitive
                .transport_colors
                .iter()
                .map(|color| color.to_array()),
        );
        vertex_indices.extend(&primitive.indices);
        vertex_records.push(SolariBakeVertexRecord {
            color_offset,
            index_offset,
            vertex_count: u32::try_from(primitive.transport_colors.len())
                .context("Solari primitive vertex count exceeded u32 indexing")?,
            index_count: u32::try_from(primitive.indices.len())
                .context("Solari primitive index count exceeded u32 indexing")?,
        });

        let material = materials.get(primitive.material).with_context(|| {
            format!(
                "Solari alpha material index {} is invalid for {}",
                primitive.material, primitive.name
            )
        })?;
        let mode = match material.alpha_mode {
            AlphaMode::Opaque => 0,
            AlphaMode::Mask => 1,
            AlphaMode::Blend => {
                bail!(
                    "Solari bake prototype supports alpha-mask materials but not blended transport"
                );
            }
        };
        let mut data_offset = 0;
        let mut width = 0;
        let mut height = 0;
        let mut wrap = [0.0, 0.0];
        if mode == 1
            && let Some(texture) = material.base_color_texture.as_ref()
        {
            let image = texture.image();
            width = image.width();
            height = image.height();
            data_offset = u32::try_from(texels.len())
                .context("Solari alpha texture data exceeded u32 indexing")?;
            wrap = texture.wrap_codes().map(|code| code as f32);
            texels.extend(
                image
                    .as_raw()
                    .chunks_exact(4)
                    .map(|pixel| f32::from(pixel[3]) / 255.0),
            );
        }
        let cutoff = if material.alpha_cutoff.is_finite() {
            material.alpha_cutoff.clamp(0.0, 1.0)
        } else {
            0.5
        };
        records.push(SolariBakeAlphaRecord {
            data_offset_width_height_mode: [data_offset, width, height, mode],
            base_alpha_cutoff_wrap: [material.base_color_factor.w, cutoff, wrap[0], wrap[1]],
        });
    }
    Ok(SolariBakeAlphaScene {
        records: Arc::new(records),
        texels: Arc::new(texels),
        vertex_records: Arc::new(vertex_records),
        vertex_colors: Arc::new(vertex_colors),
        vertex_indices: Arc::new(vertex_indices),
    })
}

const SOLARI_BAKE_WORKGROUP_SIZE: u32 = 64;
pub(crate) const SOLARI_BAKE_MAX_BOUNCES: u32 = 4;

/// One UV-space texel/surfel consumed by the optional GPU backend. Position
/// and normal are reconstructed by the shared CPU UV rasterizer; no UV1 data
/// is sent to the Solari proxy or shader.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SolariBakeTexel {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
}

/// A backend-neutral point/spot-light record. The color is already in the
/// shared CPU bake's linear/intensity convention before it crosses this
/// boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SolariBakeLight {
    pub(crate) position: [f32; 3],
    pub(crate) color: [f32; 3],
    pub(crate) range: f32,
    pub(crate) direction: [f32; 3],
    pub(crate) outer_cosine: f32,
    pub(crate) inner_cosine: f32,
    pub(crate) falloff_exponent: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SolariBakeDirectionalLight {
    pub(crate) direction: [f32; 3],
    pub(crate) color: [f32; 3],
    pub(crate) illuminance: f32,
}

/// Alpha data that cannot be recovered from Solari's `Material` structure.
/// The records are ordered exactly like the session's proxy primitives; the
/// bake scene contains no other ray-traced instances, so Solari's instance
/// index is the stable lookup key for this side table.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SolariBakeAlphaRecord {
    pub(crate) data_offset_width_height_mode: [u32; 4],
    pub(crate) base_alpha_cutoff_wrap: [f32; 4],
}

/// Per-proxy offsets into the flattened transport-color and index side tables.
/// Solari's resolved hit omits the primitive's vertex attributes, so the bake
/// shader uses the raw ray's instance/primitive/barycentric data to recover
/// the CPU transport color at the hit.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SolariBakeVertexRecord {
    pub(crate) color_offset: u32,
    pub(crate) index_offset: u32,
    pub(crate) vertex_count: u32,
    pub(crate) index_count: u32,
}

#[derive(Clone, Debug, Default)]
struct SolariBakeAlphaScene {
    records: Arc<Vec<SolariBakeAlphaRecord>>,
    texels: Arc<Vec<f32>>,
    vertex_records: Arc<Vec<SolariBakeVertexRecord>>,
    vertex_colors: Arc<Vec<[f32; 4]>>,
    vertex_indices: Arc<Vec<u32>>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SolariBakeEnvironment {
    width: u32,
    height: u32,
    pixels: Arc<Vec<[f32; 3]>>,
    importance_cdf: Arc<Vec<f32>>,
    constant: bool,
}

impl SolariBakeEnvironment {
    fn from_map(environment: &EnvironmentMap) -> Self {
        let (width, height, pixels, importance_cdf) = environment.solari_data();
        Self {
            width,
            height,
            pixels,
            importance_cdf,
            constant: environment.constant_radiance().is_some(),
        }
    }
}

pub(crate) type SolariBakeReadback = Arc<Mutex<Option<Result<Vec<[f32; 4]>, String>>>>;

/// Main-world request shared with the render-world adapter. `revision` is
/// explicit so extraction can be repeated every frame without rebuilding GPU
/// buffers unless the bake frontend publishes a new request.
#[derive(Clone, Debug, Resource)]
pub(crate) struct SolariBakeRequest {
    pub(crate) texels: Vec<SolariBakeTexel>,
    pub(crate) lights: Vec<SolariBakeLight>,
    pub(crate) sample_count: u32,
    pub(crate) bounce_count: u32,
    pub(crate) ambient: [f32; 3],
    pub(crate) directional: SolariBakeDirectionalLight,
    pub(crate) revision: u64,
    pub(crate) scene_seed: u32,
    pub(crate) alpha_records: Arc<Vec<SolariBakeAlphaRecord>>,
    pub(crate) alpha_texels: Arc<Vec<f32>>,
    pub(crate) vertex_records: Arc<Vec<SolariBakeVertexRecord>>,
    pub(crate) vertex_colors: Arc<Vec<[f32; 4]>>,
    pub(crate) vertex_indices: Arc<Vec<u32>>,
    pub(crate) environment: Option<SolariBakeEnvironment>,
    pub(crate) readback: SolariBakeReadback,
}

impl SolariBakeRequest {
    pub(crate) fn new(
        texels: Vec<SolariBakeTexel>,
        lights: Vec<SolariBakeLight>,
        sample_count: u32,
        revision: u64,
    ) -> (Self, SolariBakeReadback) {
        let readback = Arc::new(Mutex::new(None));
        (
            Self {
                texels,
                lights,
                sample_count: sample_count.max(1),
                bounce_count: 0,
                ambient: [0.0; 3],
                directional: SolariBakeDirectionalLight::default(),
                revision,
                scene_seed: revision as u32,
                alpha_records: Arc::new(Vec::new()),
                alpha_texels: Arc::new(Vec::new()),
                vertex_records: Arc::new(Vec::new()),
                vertex_colors: Arc::new(Vec::new()),
                vertex_indices: Arc::new(Vec::new()),
                environment: None,
                readback: readback.clone(),
            },
            readback,
        )
    }
}

/// Runs one headless Solari bake session and returns direct-light irradiance
/// for the supplied UV-space surfels. The session deliberately owns no output
/// or atlas state; callers can feed this result into the existing lightmap
/// frontend and cache format.
#[allow(clippy::too_many_arguments)]
pub(crate) fn bake_direct_texels(
    primitives: &[ComposedPrimitive],
    materials: &[TransportMaterial],
    texels: Vec<SolariBakeTexel>,
    lights: &[JobLight],
    ambient: [f32; 3],
    directional: SolariBakeDirectionalLight,
    sample_count: u32,
    revision: u64,
) -> Result<Vec<[f32; 4]>> {
    bake_direct_texels_with_environment(
        primitives,
        materials,
        texels,
        lights,
        ambient,
        directional,
        None,
        sample_count,
        revision,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn bake_direct_texels_with_environment(
    primitives: &[ComposedPrimitive],
    materials: &[TransportMaterial],
    texels: Vec<SolariBakeTexel>,
    lights: &[JobLight],
    ambient: [f32; 3],
    directional: SolariBakeDirectionalLight,
    environment: Option<&EnvironmentMap>,
    sample_count: u32,
    revision: u64,
) -> Result<Vec<[f32; 4]>> {
    let mut session = SolariBakeSession::new(primitives, materials)?;
    session.bake_texels_with_environment(
        texels,
        lights,
        ambient,
        directional,
        sample_count,
        0,
        environment,
        revision,
        revision as u32,
    )
}

/// Reusable headless scene session for bounded Solari bake dispatches. The
/// BLAS/TLAS and proxy meshes are built once; each tile replaces only the
/// texel/light/output buffers and performs one readback.
pub(crate) struct SolariBakeSession {
    sub_apps: SubApps,
    alpha_scene: SolariBakeAlphaScene,
}

impl SolariBakeSession {
    pub(crate) fn new(
        primitives: &[ComposedPrimitive],
        materials: &[TransportMaterial],
    ) -> Result<Self> {
        let alpha_scene = build_alpha_scene(primitives, materials)?;
        let proxy_meshes = primitives
            .iter()
            .map(|primitive| {
                let material = materials.get(primitive.material).with_context(|| {
                    format!(
                        "Solari proxy material index {} is invalid for {}",
                        primitive.material, primitive.name
                    )
                })?;
                Ok(SolariProxy {
                    mesh: build_proxy_mesh(primitive)?,
                    material: material.clone(),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let mut app = App::new();
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                .set(bevy::render::RenderPlugin {
                    synchronous_pipeline_compilation: true,
                    ..default()
                })
                .disable::<WinitPlugin>(),
        );
        add_scene_plugin(&mut app);
        add_bake_plugin(&mut app);
        app.insert_resource(SolariProxyMeshes(proxy_meshes))
            .add_systems(bevy::app::Startup, spawn_solari_proxies);

        // Match Bevy's externally-driven headless renderer lifecycle: finish
        // the plugin graph once, then pump SubApps manually so the render
        // world can submit and complete asynchronous readbacks without a
        // window.
        app.finish();
        app.cleanup();
        let sub_apps = std::mem::take(app.sub_apps_mut());
        let features = sub_apps.main.world().resource::<RenderDevice>().features();
        let required = required_wgpu_features();
        if !features.contains(required) {
            bail!(
                "Solari bake adapter is unavailable on the selected GPU; missing features: {:?}",
                required.difference(features)
            );
        }
        Ok(Self {
            sub_apps,
            alpha_scene,
        })
    }

    pub(crate) fn bake_direct_texels(
        &mut self,
        texels: Vec<SolariBakeTexel>,
        lights: &[JobLight],
        ambient: [f32; 3],
        directional: SolariBakeDirectionalLight,
        sample_count: u32,
        revision: u64,
    ) -> Result<Vec<[f32; 4]>> {
        self.bake_texels(
            texels,
            lights,
            ambient,
            directional,
            sample_count,
            0,
            revision,
            revision as u32,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn bake_texels(
        &mut self,
        texels: Vec<SolariBakeTexel>,
        lights: &[JobLight],
        ambient: [f32; 3],
        directional: SolariBakeDirectionalLight,
        sample_count: u32,
        bounce_count: u32,
        revision: u64,
        scene_seed: u32,
    ) -> Result<Vec<[f32; 4]>> {
        self.bake_texels_with_environment(
            texels,
            lights,
            ambient,
            directional,
            sample_count,
            bounce_count,
            None,
            revision,
            scene_seed,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn bake_texels_with_environment(
        &mut self,
        texels: Vec<SolariBakeTexel>,
        lights: &[JobLight],
        ambient: [f32; 3],
        directional: SolariBakeDirectionalLight,
        sample_count: u32,
        bounce_count: u32,
        environment: Option<&EnvironmentMap>,
        revision: u64,
        scene_seed: u32,
    ) -> Result<Vec<[f32; 4]>> {
        if texels.is_empty() {
            return Ok(Vec::new());
        }
        if bounce_count > SOLARI_BAKE_MAX_BOUNCES {
            bail!(
                "Solari bake prototype supports at most {SOLARI_BAKE_MAX_BOUNCES} diffuse bounces"
            );
        }
        let lights = point_lights(lights)?;
        let (mut request, readback) =
            SolariBakeRequest::new(texels, lights, sample_count, revision);
        request.bounce_count = bounce_count;
        request.scene_seed = scene_seed;
        request.alpha_records = self.alpha_scene.records.clone();
        request.alpha_texels = self.alpha_scene.texels.clone();
        request.vertex_records = self.alpha_scene.vertex_records.clone();
        request.vertex_colors = self.alpha_scene.vertex_colors.clone();
        request.vertex_indices = self.alpha_scene.vertex_indices.clone();
        request.environment = environment.map(SolariBakeEnvironment::from_map);
        request.ambient = ambient;
        request.directional = directional;
        self.sub_apps.main.world_mut().insert_resource(request);

        for _ in 0..600 {
            self.sub_apps.update();
            self.sub_apps
                .main
                .world()
                .resource::<RenderDevice>()
                .wgpu_device()
                .poll(bevy::render::render_resource::PollType::Wait {
                    submission_index: None,
                    timeout: Some(Duration::from_millis(50)),
                })
                .context("waiting for Solari bake GPU work")?;
            if let Some(result) = readback
                .lock()
                .map_err(|_| anyhow::anyhow!("Solari bake readback lock was poisoned"))?
                .take()
            {
                return result.map_err(anyhow::Error::msg);
            }
        }
        bail!("Solari bake session timed out while waiting for GPU readback")
    }
}

fn point_lights(lights: &[JobLight]) -> Result<Vec<SolariBakeLight>> {
    lights
        .iter()
        .map(|light| {
            let is_spot = light.kind.eq_ignore_ascii_case("spot") || light.flags & 0x200 != 0;
            if !light.kind.is_empty()
                && !light.kind.eq_ignore_ascii_case("point")
                && !light.kind.eq_ignore_ascii_case("spot")
            {
                bail!(
                    "Solari bake prototype supports only point and spot lights; encountered {}",
                    light.kind
                );
            }
            let intensity = bevyout_core::lighting::point_light_intensity(
                light.radius,
                light.intensity_lumens,
                bevyout_core::lighting::DEFAULT_LIGHTING_SCALE,
            ) / (4.0 * std::f32::consts::PI);
            let color = bevyout_core::lighting::srgb_to_linear_rgb([
                light.color_rgba[0],
                light.color_rgba[1],
                light.color_rgba[2],
            ])
            .map(|channel| channel * intensity);
            if !color.iter().all(|channel| channel.is_finite()) {
                bail!("Solari bake light has non-finite radiance");
            }
            let (direction, outer_cosine, inner_cosine, falloff_exponent) = if is_spot
                && light.spot_fov_radians.is_finite()
                && light.spot_fov_radians > f32::EPSILON
            {
                let axis = (bevy::math::Quat::from_array(light.rotation_xyzw) * Vec3::NEG_Z)
                    .normalize_or_zero();
                if axis == Vec3::ZERO {
                    return Ok(SolariBakeLight {
                        position: light.translation,
                        color,
                        range: light.radius,
                        direction: [0.0, 0.0, -1.0],
                        outer_cosine: 1.0,
                        inner_cosine: 1.0,
                        falloff_exponent: 0.0,
                    });
                }
                let outer_angle =
                    (light.spot_fov_radians * 0.5).clamp(f32::EPSILON, std::f32::consts::FRAC_PI_2);
                (
                    axis.to_array(),
                    outer_angle.cos(),
                    (outer_angle * 0.8).cos(),
                    light.spot_falloff_exponent.max(0.0),
                )
            } else {
                // The CPU policy treats a missing/invalid cone as a point
                // light for backwards-compatible old LIGH records.
                ([0.0, 0.0, -1.0], -1.0, -1.0, 0.0)
            };
            Ok(SolariBakeLight {
                position: light.translation,
                color,
                range: light.radius,
                direction,
                outer_cosine,
                inner_cosine,
                falloff_exponent,
            })
        })
        .collect()
}

struct SolariProxy {
    mesh: Mesh,
    material: TransportMaterial,
}

#[derive(Resource)]
struct SolariProxyMeshes(Vec<SolariProxy>);

fn spawn_solari_proxies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut proxies: ResMut<SolariProxyMeshes>,
) {
    for proxy in proxies.0.drain(..) {
        let mesh_handle = meshes.add(proxy.mesh);
        let material_handle = materials.add(solari_material(&proxy.material, &mut images));
        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle),
            bevy::solari::prelude::RaytracingMesh3d(mesh_handle),
            Transform::IDENTITY,
        ));
    }
}

fn solari_material(source: &TransportMaterial, images: &mut Assets<Image>) -> StandardMaterial {
    let base_color =
        bevyout_core::lighting::srgb_to_linear_rgb(source.base_color_factor.truncate().to_array());
    let emissive = bevyout_core::lighting::srgb_to_linear_rgb(source.emissive_factor.to_array())
        .map(|channel| channel * bevyout_core::lighting::EMISSION_SCALE);
    let mut material = StandardMaterial {
        base_color: Color::linear_rgba(
            base_color[0],
            base_color[1],
            base_color[2],
            source.base_color_factor.w,
        ),
        emissive: LinearRgba::new(emissive[0], emissive[1], emissive[2], 1.0),
        metallic: source.metallic_factor.clamp(0.0, 1.0),
        double_sided: source.double_sided,
        cull_mode: (!source.double_sided).then_some(bevy::render::render_resource::Face::Back),
        alpha_mode: match source.alpha_mode {
            AlphaMode::Opaque => BevyAlphaMode::Opaque,
            AlphaMode::Mask => BevyAlphaMode::Mask(source.alpha_cutoff),
            AlphaMode::Blend => BevyAlphaMode::Blend,
        },
        ..default()
    };
    material.base_color_texture = source
        .base_color_texture
        .as_ref()
        .map(|texture| images.add(solari_image(texture.image(), texture.wrap_codes())));
    material.emissive_texture = source
        .emissive_texture
        .as_ref()
        .map(|texture| images.add(solari_image(texture.image(), texture.wrap_codes())));
    material
}

fn solari_image(image: &image::RgbaImage, wrap_codes: [u32; 2]) -> Image {
    let mut image = Image::new(
        Extent3d {
            width: image.width().max(1),
            height: image.height().max(1),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image.as_raw().clone(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    let mut sampler = ImageSamplerDescriptor::linear();
    let address_mode = |code| match code {
        1 => ImageAddressMode::Repeat,
        2 => ImageAddressMode::MirrorRepeat,
        _ => ImageAddressMode::ClampToEdge,
    };
    sampler.address_mode_u = address_mode(wrap_codes[0]);
    sampler.address_mode_v = address_mode(wrap_codes[1]);
    image.sampler = ImageSampler::Descriptor(sampler);
    image
}

#[derive(Resource)]
struct SolariBakePipelines {
    bind_group_layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
}

#[derive(Resource)]
struct SolariBakeGpuResources {
    revision: u64,
    texel_count: u32,
    output_size: u64,
    texel_buffer: Buffer,
    light_buffer: Buffer,
    params_buffer: Buffer,
    directional_buffer: Buffer,
    ambient_buffer: Buffer,
    alpha_material_buffer: Buffer,
    alpha_texel_buffer: Buffer,
    vertex_record_buffer: Buffer,
    vertex_color_buffer: Buffer,
    vertex_index_buffer: Buffer,
    environment_buffer: Buffer,
    environment_cdf_buffer: Buffer,
    output_buffer: Buffer,
    staging_buffer: Buffer,
    bind_group: Option<BindGroup>,
    submitted: bool,
    readback: SolariBakeReadback,
}

/// Render plugin for an offline, UV-texel Solari dispatch. It uses only
/// `RaytracingSceneBindings` from Solari and never installs `SolariLightingPlugin`.
pub(crate) struct SolariBakePlugin;

impl Plugin for SolariBakePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "solari_bake.wgsl");
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        let Some(render_device) = render_app.world().get_resource::<RenderDevice>() else {
            return;
        };
        if !render_device
            .features()
            .contains(bevy::solari::SolariPlugins::required_wgpu_features())
        {
            bevy::log::warn!(
                "Solari bake adapter disabled; GPU lacks required features: {:?}",
                bevy::solari::SolariPlugins::required_wgpu_features()
                    .difference(render_device.features())
            );
            return;
        }
        render_app
            .add_systems(RenderStartup, init_solari_bake_pipelines)
            .add_systems(ExtractSchedule, extract_solari_bake_request)
            .add_systems(
                Render,
                prepare_solari_bake_buffers.in_set(RenderSystems::PrepareResources),
            )
            .add_systems(Render, dispatch_solari_bake.in_set(RenderSystems::Render));
    }
}

fn init_solari_bake_pipelines(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    scene_bindings: Option<Res<RaytracingSceneBindings>>,
    asset_server: Res<AssetServer>,
) {
    let Some(scene_bindings) = scene_bindings else {
        return;
    };
    let bind_group_layout = BindGroupLayoutDescriptor::new(
        "solari_bake_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
                binding_types::storage_buffer_read_only_sized(false, None),
            ),
        ),
    );
    let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("solari_bake_pipeline".into()),
        layout: vec![
            scene_bindings.bind_group_layout.clone(),
            bind_group_layout.clone(),
        ],
        shader: load_embedded_asset!(asset_server.as_ref(), "solari_bake.wgsl"),
        ..Default::default()
    });
    commands.insert_resource(SolariBakePipelines {
        bind_group_layout,
        pipeline,
    });
}

fn extract_solari_bake_request(
    request: Extract<Option<Res<SolariBakeRequest>>>,
    mut commands: Commands,
) {
    if let Some(request) = request.as_ref().map(|request| (*request).clone()) {
        commands.insert_resource(request);
    }
}

fn prepare_solari_bake_buffers(
    request: Option<Res<SolariBakeRequest>>,
    gpu: Option<Res<SolariBakeGpuResources>>,
    render_device: Res<RenderDevice>,
    mut commands: Commands,
) {
    let Some(request) = request else {
        return;
    };
    if request.texels.is_empty() || gpu.is_some_and(|gpu| gpu.revision == request.revision) {
        return;
    }

    let texel_words = request
        .texels
        .iter()
        .flat_map(|texel| {
            [
                texel.position[0],
                texel.position[1],
                texel.position[2],
                0.0,
                texel.normal[0],
                texel.normal[1],
                texel.normal[2],
                0.0,
            ]
        })
        .collect::<Vec<_>>();
    let light_words = request
        .lights
        .iter()
        .flat_map(|light| {
            [
                light.position[0],
                light.position[1],
                light.position[2],
                light.range,
                light.color[0],
                light.color[1],
                light.color[2],
                0.0,
                light.direction[0],
                light.direction[1],
                light.direction[2],
                light.outer_cosine,
                light.inner_cosine,
                light.falloff_exponent,
                0.0,
                0.0,
            ]
        })
        .collect::<Vec<_>>();
    let texel_bytes = f32_bytes(&texel_words);
    let light_bytes = f32_bytes(&light_words);
    let (environment_width, environment_height, environment_words, environment_enabled) = request
        .environment
        .as_ref()
        .map_or((1, 1, vec![0.0; 4], 0), |environment| {
            (
                environment.width,
                environment.height,
                environment
                    .pixels
                    .iter()
                    .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], 0.0])
                    .collect::<Vec<_>>(),
                1,
            )
        });
    let environment_cdf_words = request.environment.as_ref().map_or_else(
        || vec![0.0],
        |environment| (*environment.importance_cdf).clone(),
    );
    let environment_constant = request
        .environment
        .as_ref()
        .is_some_and(|environment| environment.constant);
    let params = [
        request.texels.len() as u32,
        request.lights.len() as u32,
        request.sample_count,
        request.bounce_count,
        request.scene_seed,
        environment_width,
        environment_height,
        environment_enabled,
        environment_cdf_words.len() as u32,
        u32::from(environment_constant),
        request.vertex_records.len() as u32,
    ];
    let params_bytes = u32_bytes(&params);
    let directional_words = [
        request.directional.direction[0],
        request.directional.direction[1],
        request.directional.direction[2],
        request.directional.illuminance,
        request.directional.color[0],
        request.directional.color[1],
        request.directional.color[2],
        0.0,
    ];
    let ambient_words = [
        request.ambient[0],
        request.ambient[1],
        request.ambient[2],
        0.0,
    ];
    let directional_bytes = f32_bytes(&directional_words);
    let ambient_bytes = f32_bytes(&ambient_words);
    let alpha_material_bytes = request
        .alpha_records
        .iter()
        .flat_map(|record| {
            u32_bytes(&record.data_offset_width_height_mode)
                .into_iter()
                .chain(f32_bytes(&record.base_alpha_cutoff_wrap))
        })
        .collect::<Vec<_>>();
    let alpha_texel_bytes = f32_bytes(&request.alpha_texels);
    let vertex_record_bytes = request
        .vertex_records
        .iter()
        .flat_map(|record| {
            u32_bytes(&[
                record.color_offset,
                record.index_offset,
                record.vertex_count,
                record.index_count,
            ])
        })
        .collect::<Vec<_>>();
    let vertex_color_words = request
        .vertex_colors
        .iter()
        .flat_map(|color| color.iter().copied())
        .collect::<Vec<_>>();
    let vertex_color_bytes = f32_bytes(&vertex_color_words);
    let vertex_index_bytes = u32_bytes(&request.vertex_indices);
    let environment_bytes = f32_bytes(&environment_words);
    let environment_cdf_bytes = f32_bytes(&environment_cdf_words);
    let output_size = (request.texels.len() as u64).saturating_mul(16).max(16);
    let texel_bytes = nonempty_bytes(texel_bytes);
    let mut light_bytes = nonempty_bytes(light_bytes);
    // The shader declares an array of 64-byte BakeLight records. Even when
    // light_count is zero, wgpu validates the binding against that minimum
    // structured element size rather than the logical element count.
    light_bytes.resize(light_bytes.len().max(64), 0);
    let params_bytes = nonempty_bytes(params_bytes);
    let directional_bytes = nonempty_bytes(directional_bytes);
    let ambient_bytes = nonempty_bytes(ambient_bytes);
    // The shader declares structured arrays even for an all-opaque scene.
    // Keep every storage binding valid on adapters that validate the minimum
    // array element size rather than the logical element count.
    let mut alpha_material_bytes = nonempty_bytes(alpha_material_bytes);
    alpha_material_bytes.resize(alpha_material_bytes.len().max(32), 0);
    let mut alpha_texel_bytes = nonempty_bytes(alpha_texel_bytes);
    alpha_texel_bytes.resize(alpha_texel_bytes.len().max(16), 0);
    let mut vertex_record_bytes = nonempty_bytes(vertex_record_bytes);
    vertex_record_bytes.resize(vertex_record_bytes.len().max(16), 0);
    let mut vertex_color_bytes = nonempty_bytes(vertex_color_bytes);
    vertex_color_bytes.resize(vertex_color_bytes.len().max(16), 0);
    let mut vertex_index_bytes = nonempty_bytes(vertex_index_bytes);
    vertex_index_bytes.resize(vertex_index_bytes.len().max(16), 0);
    let environment_bytes = nonempty_bytes(environment_bytes);
    let mut environment_cdf_bytes = nonempty_bytes(environment_cdf_bytes);
    environment_cdf_bytes.resize(environment_cdf_bytes.len().max(16), 0);
    let texel_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake texels"),
        contents: &texel_bytes,
        usage: BufferUsages::STORAGE,
    });
    let light_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake lights"),
        contents: &light_bytes,
        usage: BufferUsages::STORAGE,
    });
    let params_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake parameters"),
        contents: &params_bytes,
        usage: BufferUsages::STORAGE,
    });
    let directional_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake directional light"),
        contents: &directional_bytes,
        usage: BufferUsages::STORAGE,
    });
    let ambient_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake ambient"),
        contents: &ambient_bytes,
        usage: BufferUsages::STORAGE,
    });
    let alpha_material_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake alpha materials"),
        contents: &alpha_material_bytes,
        usage: BufferUsages::STORAGE,
    });
    let alpha_texel_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake alpha texels"),
        contents: &alpha_texel_bytes,
        usage: BufferUsages::STORAGE,
    });
    let vertex_record_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake vertex records"),
        contents: &vertex_record_bytes,
        usage: BufferUsages::STORAGE,
    });
    let vertex_color_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake vertex colors"),
        contents: &vertex_color_bytes,
        usage: BufferUsages::STORAGE,
    });
    let vertex_index_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake vertex indices"),
        contents: &vertex_index_bytes,
        usage: BufferUsages::STORAGE,
    });
    let environment_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake environment"),
        contents: &environment_bytes,
        usage: BufferUsages::STORAGE,
    });
    let environment_cdf_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("solari bake environment importance cdf"),
        contents: &environment_cdf_bytes,
        usage: BufferUsages::STORAGE,
    });
    let output_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("solari bake output"),
        size: output_size,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let staging_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("solari bake readback"),
        size: output_size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    commands.insert_resource(SolariBakeGpuResources {
        revision: request.revision,
        texel_count: request.texels.len() as u32,
        output_size,
        texel_buffer,
        light_buffer,
        params_buffer,
        directional_buffer,
        ambient_buffer,
        alpha_material_buffer,
        alpha_texel_buffer,
        vertex_record_buffer,
        vertex_color_buffer,
        vertex_index_buffer,
        environment_buffer,
        environment_cdf_buffer,
        output_buffer,
        staging_buffer,
        bind_group: None,
        submitted: false,
        readback: request.readback.clone(),
    });
}

fn dispatch_solari_bake(
    mut ctx: RenderContext,
    pipelines: Option<Res<SolariBakePipelines>>,
    pipeline_cache: Res<PipelineCache>,
    scene_bindings: Option<Res<RaytracingSceneBindings>>,
    render_device: Res<RenderDevice>,
    mut gpu: Option<ResMut<SolariBakeGpuResources>>,
) {
    let (Some(pipelines), Some(scene_bindings), Some(mut gpu)) =
        (pipelines, scene_bindings, gpu.take())
    else {
        return;
    };
    if gpu.submitted {
        return;
    }
    let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipelines.pipeline) else {
        return;
    };
    let Some(scene_bind_group) = scene_bindings.bind_group.as_ref() else {
        return;
    };
    if gpu.bind_group.is_none() {
        let bind_group = render_device.create_bind_group(
            "solari_bake_bind_group",
            &pipeline_cache.get_bind_group_layout(&pipelines.bind_group_layout),
            &BindGroupEntries::sequential((
                gpu.texel_buffer.as_entire_binding(),
                gpu.light_buffer.as_entire_binding(),
                gpu.output_buffer.as_entire_binding(),
                gpu.params_buffer.as_entire_binding(),
                gpu.directional_buffer.as_entire_binding(),
                gpu.ambient_buffer.as_entire_binding(),
                gpu.alpha_material_buffer.as_entire_binding(),
                gpu.alpha_texel_buffer.as_entire_binding(),
                gpu.vertex_record_buffer.as_entire_binding(),
                gpu.vertex_color_buffer.as_entire_binding(),
                gpu.vertex_index_buffer.as_entire_binding(),
                gpu.environment_buffer.as_entire_binding(),
                gpu.environment_cdf_buffer.as_entire_binding(),
            )),
        );
        gpu.bind_group = Some(bind_group);
    }
    let Some(bind_group) = gpu.bind_group.as_ref() else {
        return;
    };
    let command_encoder = ctx.command_encoder();
    {
        let mut pass = command_encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("solari bake"),
            timestamp_writes: None,
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, scene_bind_group, &[]);
        pass.set_bind_group(1, bind_group, &[]);
        pass.dispatch_workgroups(gpu.texel_count.div_ceil(SOLARI_BAKE_WORKGROUP_SIZE), 1, 1);
    }
    command_encoder.copy_buffer_to_buffer(
        &gpu.output_buffer,
        0,
        &gpu.staging_buffer,
        0,
        gpu.output_size,
    );
    let staging_buffer = gpu.staging_buffer.clone();
    let callback_buffer = staging_buffer.clone();
    let readback = gpu.readback.clone();
    let texel_count = gpu.texel_count;
    command_encoder.map_buffer_on_submit(&staging_buffer, MapMode::Read, .., move |result| {
        let outcome = result
            .map_err(|error| format!("Solari bake readback failed: {error:?}"))
            .and_then(|_| decode_readback(&callback_buffer, texel_count));
        if let Ok(mut slot) = readback.lock() {
            *slot = Some(outcome);
        }
        callback_buffer.unmap();
    });
    gpu.submitted = true;
}

fn decode_readback(buffer: &Buffer, texel_count: u32) -> Result<Vec<[f32; 4]>, String> {
    let bytes = buffer.slice(..).get_mapped_range();
    let expected = texel_count as usize * 16;
    if bytes.len() < expected {
        return Err(format!(
            "Solari bake readback returned {} bytes, expected {expected}",
            bytes.len()
        ));
    }
    let values = bytes[..expected]
        .chunks_exact(16)
        .map(|chunk| {
            std::array::from_fn(|channel| {
                let offset = channel * 4;
                f32::from_le_bytes([
                    chunk[offset],
                    chunk[offset + 1],
                    chunk[offset + 2],
                    chunk[offset + 3],
                ])
            })
        })
        .collect();
    drop(bytes);
    Ok(values)
}

fn f32_bytes(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn u32_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn nonempty_bytes(mut bytes: Vec<u8>) -> Vec<u8> {
    if bytes.is_empty() {
        bytes.resize(16, 0);
    }
    bytes
}

#[cfg(test)]
#[path = "../tests/solari.rs"]
mod tests;
