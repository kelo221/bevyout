use super::JobLight;
use super::environment::EnvironmentMap;
use super::policy::{
    AMBIENT_CUBE_FACE_COUNT, atlas_dimensions, primary_ray_count, volume_resolution,
};
use super::rust_scene::{AlphaMode, RustBakeScene, TransportMaterial};
use super::transport::{
    material::{MaterialSample, sample_material},
    sampling::{cosine_hemisphere_direction, sample_seed, sample_uniform_1d},
};
use anyhow::{Context, Result, bail};
use bevy::math::{Quat, Vec2, Vec3, Vec4};
use bevyout_core::lighting::{
    DEFAULT_AMBIENT_SCALE, DEFAULT_LIGHTING_SCALE, ambient_irradiance, point_light_intensity,
    srgb_to_linear_rgb,
};
use bvh::aabb::{Aabb, Bounded};
use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};
use bvh::bvh::Bvh;
use bvh::ray::Ray;
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const FACE_DIRECTIONS: [Vec3; 6] = [
    Vec3::X,
    Vec3::Y,
    Vec3::Z,
    Vec3::NEG_X,
    Vec3::NEG_Y,
    Vec3::NEG_Z,
];
const RAY_EPSILON: f32 = 0.002;
const MAX_RAY_OFFSET: f32 = 0.05;
const MAX_TRANSPARENT_LAYERS: usize = 8;
const RUSSIAN_ROULETTE_START_BOUNCE: usize = 3;
const MIN_RUSSIAN_ROULETTE_SURVIVAL: f32 = 0.05;
const MAX_RUSSIAN_ROULETTE_SURVIVAL: f32 = 0.95;
const VOLUME_INDIRECT_SAMPLE_COUNT: u32 = 4;
const VOLUME_BOUNCE_COUNT: u32 = 2;
const ENVIRONMENT_IRRADIANCE_SAMPLE_COUNT: u32 = 16;

#[derive(Clone, Debug)]
pub(crate) struct IrradianceTriangle {
    pub(crate) vertices: [Point3<f32>; 3],
    pub(crate) normals: [Vec3; 3],
    pub(crate) uvs: [Vec2; 3],
    pub(crate) colors: [Vec4; 3],
    pub(crate) material: usize,
    pub(crate) node_index: usize,
}

impl Bounded<f32, 3> for IrradianceTriangle {
    fn aabb(&self) -> Aabb<f32, 3> {
        let mut minimum = self.vertices[0];
        let mut maximum = self.vertices[0];
        for vertex in &self.vertices[1..] {
            minimum.x = minimum.x.min(vertex.x);
            minimum.y = minimum.y.min(vertex.y);
            minimum.z = minimum.z.min(vertex.z);
            maximum.x = maximum.x.max(vertex.x);
            maximum.y = maximum.y.max(vertex.y);
            maximum.z = maximum.z.max(vertex.z);
        }
        Aabb::with_bounds(minimum, maximum)
    }
}

impl BHShape<f32, 3> for IrradianceTriangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

#[derive(Clone, Copy)]
struct SurfaceHit<'a> {
    triangle: &'a IrradianceTriangle,
    distance: f32,
    barycentric: [f32; 3],
}

#[derive(Debug)]
pub(crate) struct RustIrradianceResult {
    pub(crate) resolution: [u32; 3],
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: [f32; 3],
    pub(crate) raw_slices: Vec<PathBuf>,
    pub(crate) primary_rays: usize,
    pub(crate) nonzero_voxels: usize,
    pub(crate) maximum: f32,
}

pub(crate) struct DirectionalBakeLight {
    pub(crate) color_rgba: [f32; 4],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) illuminance: f32,
}

#[derive(Clone, Copy, Debug)]
struct EmissiveEntry {
    triangle_index: usize,
    weight: f32,
    cumulative_weight: f32,
}

/// A deterministic area-light distribution built once per composed scene.
/// Entries are selected by emitted-power weight; the sampled point is then
/// uniform over that triangle, making the estimator an explicit next-event
/// sample rather than relying on a cosine path to hit small emitters.
#[derive(Clone, Debug, Default)]
pub(crate) struct EmissiveSampler {
    entries: Vec<EmissiveEntry>,
    total_weight: f32,
    selection_pmf_by_triangle: Vec<f32>,
}

impl EmissiveSampler {
    pub(crate) fn new(triangles: &[IrradianceTriangle], materials: &[TransportMaterial]) -> Self {
        let mut entries = Vec::new();
        let mut total_weight = 0.0_f32;
        for (triangle_index, triangle) in triangles.iter().enumerate() {
            let Some(material) = materials.get(triangle.material) else {
                continue;
            };
            let area = triangle_area(triangle);
            if area <= 0.0 || !area.is_finite() {
                continue;
            }
            let center = [1.0 / 3.0; 3];
            let sample = sample_material(
                material,
                interpolate_vec2(triangle.uvs, center),
                interpolate_vec4(triangle.colors, center),
            );
            let potential = sample
                .emissive
                .max_element()
                .max(material.emissive_factor.max_element())
                .max(if material.emissive_texture.is_some() {
                    1.0e-6
                } else {
                    0.0
                });
            let weight = (area * potential.max(0.0)).max(0.0);
            if weight <= 0.0 || !weight.is_finite() {
                continue;
            }
            total_weight += weight;
            entries.push(EmissiveEntry {
                triangle_index,
                weight,
                cumulative_weight: 0.0,
            });
        }
        if !total_weight.is_finite() {
            return Self::default();
        }
        let mut cumulative_weight = 0.0;
        let mut selection_pmf_by_triangle = vec![0.0; triangles.len()];
        for entry in &mut entries {
            cumulative_weight += entry.weight;
            entry.cumulative_weight = cumulative_weight;
            selection_pmf_by_triangle[entry.triangle_index] = entry.weight / total_weight;
        }
        Self {
            entries,
            total_weight,
            selection_pmf_by_triangle,
        }
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty() || self.total_weight <= 0.0
    }

    fn select(&self, sample: f32) -> &EmissiveEntry {
        let target = sample.clamp(0.0, 1.0) * self.total_weight;
        let index = self
            .entries
            .partition_point(|entry| entry.cumulative_weight <= target)
            .min(self.entries.len() - 1);
        &self.entries[index]
    }

    fn selection_probability(&self, triangle_index: usize) -> f32 {
        self.selection_pmf_by_triangle
            .get(triangle_index)
            .copied()
            .unwrap_or(0.0)
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn bake_irradiance(
    scene: &RustBakeScene,
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_rgba: [f32; 4],
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    spacing: f32,
    samples: u32,
    raw_path: &Path,
) -> Result<RustIrradianceResult> {
    let mut triangles = collect_triangles(scene)?;
    if triangles.is_empty() {
        bail!("Rust irradiance bake scene contains no triangles");
    }
    println!(
        "Rust irradiance: building BVH for {} triangles and {} point lights",
        triangles.len(),
        lights.len()
    );
    let bvh = Bvh::build_par(&mut triangles);
    let emissive_sampler = EmissiveSampler::new(&triangles, &scene.materials);
    let minimum = scene.bounds.minimum - Vec3::splat(spacing);
    let maximum = scene.bounds.maximum + Vec3::splat(spacing);
    let scale = maximum - minimum;
    let resolution = volume_resolution(scale.to_array(), spacing);
    let translation = (minimum + maximum) * 0.5;
    let probe_count = resolution.iter().product::<u32>() as usize;
    debug_assert_eq!(FACE_DIRECTIONS.len(), AMBIENT_CUBE_FACE_COUNT);
    let primary_rays = primary_ray_count(resolution, samples);
    let ambient_irradiance = Vec3::from_array(ambient_irradiance(
        ambient_rgba,
        DEFAULT_LIGHTING_SCALE,
        DEFAULT_AMBIENT_SCALE,
    ));
    println!(
        "Rust irradiance: bounds center={:?} scale={:?} resolution={:?}, {} primary rays",
        translation.to_array(),
        scale.to_array(),
        resolution,
        primary_rays
    );
    let completed = AtomicUsize::new(0);
    let max_distance = scale.length() * 2.0 + spacing;
    let values = (0..probe_count)
        .into_par_iter()
        .map(|probe_index| {
            let position = probe_position(probe_index, resolution, minimum, maximum);
            let faces = FACE_DIRECTIONS.map(|normal| {
                sample_face(
                    &bvh,
                    &triangles,
                    &scene.materials,
                    lights,
                    directional,
                    Some(&emissive_sampler),
                    ambient_irradiance,
                    environment_map,
                    scene_seed,
                    position,
                    normal,
                    probe_index,
                    samples,
                    max_distance,
                )
            });
            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            if done == probe_count || done.is_multiple_of(64) {
                println!("Rust irradiance: traced probes {done}/{probe_count}");
            }
            faces
        })
        .collect::<Vec<_>>();
    let mut nonzero_voxels = 0;
    let mut maximum_value = 0.0_f32;
    for faces in &values {
        for color in faces {
            maximum_value = maximum_value.max(color.max_element());
            if color.max_element() > 0.0 {
                nonzero_voxels += 1;
            }
        }
    }
    if (!lights.is_empty() || directional.illuminance > 0.0) && nonzero_voxels == 0 {
        bail!("Rust irradiance bake produced an all-zero volume for a lit scene");
    }
    let raw_slices = write_atlas(raw_path, resolution, &values)?;
    Ok(RustIrradianceResult {
        resolution,
        translation: translation.to_array(),
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: scale.to_array(),
        raw_slices,
        primary_rays,
        nonzero_voxels,
        maximum: maximum_value,
    })
}

/// Traces a Bevy/KTX-oriented HDR reflection cubemap from a prepared position.
/// Four deterministic sub-pixel samples keep the low-resolution 64px capture
/// stable without turning prepare into a runtime cubemap render pass.
pub(crate) fn trace_reflection_cubemap(
    scene: &RustBakeScene,
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_rgba: [f32; 4],
    environment_map: Option<&EnvironmentMap>,
    position: [f32; 3],
    resolution: u32,
) -> Result<Vec<Vec<u32>>> {
    let mut triangles = collect_triangles(scene)?;
    if triangles.is_empty() {
        bail!("Rust reflection capture scene contains no triangles");
    }
    let bvh = Bvh::build_par(&mut triangles);
    let emissive_sampler = EmissiveSampler::new(&triangles, &scene.materials);
    let origin = Vec3::from_array(position);
    let ambient_irradiance = Vec3::from_array(ambient_irradiance(
        ambient_rgba,
        DEFAULT_LIGHTING_SCALE,
        DEFAULT_AMBIENT_SCALE,
    ));
    let max_distance = scene.bounds.extent().length() * 2.0 + 1.0;
    let sample_offsets = [
        (0.25_f32, 0.25_f32),
        (0.75, 0.25),
        (0.25, 0.75),
        (0.75, 0.75),
    ];
    let faces = (0..6)
        .into_par_iter()
        .map(|face| {
            let mut pixels = Vec::with_capacity((resolution * resolution) as usize);
            for y in 0..resolution {
                for x in 0..resolution {
                    let mut radiance = Vec3::ZERO;
                    for (offset_x, offset_y) in sample_offsets {
                        let u = ((x as f32 + offset_x) / resolution as f32) * 2.0 - 1.0;
                        let v = ((y as f32 + offset_y) / resolution as f32) * 2.0 - 1.0;
                        let direction = reflection_cubemap_direction(face, u, v);
                        radiance += trace_radiance_with_emissive_and_environment(
                            &bvh,
                            &triangles,
                            &scene.materials,
                            lights,
                            directional,
                            &emissive_sampler,
                            ambient_irradiance,
                            environment_map,
                            origin,
                            direction,
                            max_distance,
                        );
                    }
                    pixels.push(pack_rgb9e5(radiance / sample_offsets.len() as f32));
                }
            }
            pixels
        })
        .collect();
    Ok(faces)
}

fn reflection_cubemap_direction(face: usize, u: f32, v: f32) -> Vec3 {
    let ktx = match face {
        0 => Vec3::new(1.0, -v, -u),
        1 => Vec3::new(-1.0, -v, u),
        2 => Vec3::new(u, 1.0, v),
        3 => Vec3::new(u, -1.0, -v),
        4 => Vec3::new(u, -v, 1.0),
        5 => Vec3::new(-u, -v, -1.0),
        _ => unreachable!("cubemap face must be in 0..6"),
    };
    Vec3::new(ktx.x, ktx.y, -ktx.z).normalize()
}

pub(crate) fn collect_triangles(scene: &RustBakeScene) -> Result<Vec<IrradianceTriangle>> {
    let mut triangles = Vec::new();
    for primitive in &scene.primitives {
        for indices in primitive.indices.chunks_exact(3) {
            let [a, b, c] = [
                indices[0] as usize,
                indices[1] as usize,
                indices[2] as usize,
            ];
            let positions = [
                *primitive
                    .positions
                    .get(a)
                    .context("triangle position index is invalid")?,
                *primitive
                    .positions
                    .get(b)
                    .context("triangle position index is invalid")?,
                *primitive
                    .positions
                    .get(c)
                    .context("triangle position index is invalid")?,
            ];
            if !positions.iter().all(|position| position.is_finite())
                || (positions[1] - positions[0])
                    .cross(positions[2] - positions[0])
                    .length_squared()
                    <= 1e-12
            {
                continue;
            }
            triangles.push(IrradianceTriangle {
                vertices: positions.map(point3),
                normals: [
                    primitive.normals[a],
                    primitive.normals[b],
                    primitive.normals[c],
                ],
                uvs: [primitive.uvs[a], primitive.uvs[b], primitive.uvs[c]],
                colors: [
                    primitive.transport_colors[a],
                    primitive.transport_colors[b],
                    primitive.transport_colors[c],
                ],
                material: primitive.material,
                node_index: 0,
            });
        }
    }
    Ok(triangles)
}

fn triangle_area(triangle: &IrradianceTriangle) -> f32 {
    let edge_a = triangle.vertices[1] - triangle.vertices[0];
    let edge_b = triangle.vertices[2] - triangle.vertices[0];
    edge_a.cross(&edge_b).norm() * 0.5
}

fn sample_triangle_barycentric(u: f32, v: f32) -> [f32; 3] {
    let root = u.clamp(0.0, 1.0).sqrt();
    [
        1.0 - root,
        root * (1.0 - v.clamp(0.0, 1.0)),
        root * v.clamp(0.0, 1.0),
    ]
}

#[allow(clippy::too_many_arguments)]
fn sample_face(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: Option<&EmissiveSampler>,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    origin: Vec3,
    face_normal: Vec3,
    probe_index: usize,
    samples: u32,
    max_distance: f32,
) -> Vec3 {
    let mut sum = Vec3::ZERO;
    for sample in 0..samples {
        let direction = cosine_hemisphere_direction(
            face_normal,
            sample_seed(scene_seed, probe_index, sample),
            sample,
            samples,
        );
        sum += trace_radiance_with_bounces(
            bvh,
            triangles,
            materials,
            lights,
            directional,
            emitters,
            ambient_irradiance,
            environment_map,
            scene_seed,
            probe_index
                .wrapping_mul(samples as usize)
                .wrapping_add(sample as usize),
            VOLUME_INDIRECT_SAMPLE_COUNT,
            VOLUME_BOUNCE_COUNT,
            0,
            origin,
            direction,
            max_distance,
        );
    }
    sum * (std::f32::consts::PI / samples.max(1) as f32)
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn trace_radiance(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_irradiance: Vec3,
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
) -> Vec3 {
    trace_radiance_with_bounces(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        None,
        ambient_irradiance,
        None,
        0,
        0,
        0,
        0,
        0,
        origin,
        direction,
        max_distance,
    )
}

#[allow(clippy::too_many_arguments)]
#[cfg(test)]
fn trace_radiance_with_emissive(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: &EmissiveSampler,
    ambient_irradiance: Vec3,
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
) -> Vec3 {
    trace_radiance_with_emissive_and_environment(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        emitters,
        ambient_irradiance,
        None,
        origin,
        direction,
        max_distance,
    )
}

#[allow(clippy::too_many_arguments)]
fn trace_radiance_with_emissive_and_environment(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: &EmissiveSampler,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
) -> Vec3 {
    trace_radiance_with_bounces(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        Some(emitters),
        ambient_irradiance,
        environment_map,
        0,
        0,
        0,
        0,
        0,
        origin,
        direction,
        max_distance,
    )
}

#[allow(clippy::too_many_arguments)]
fn trace_radiance_with_bounces(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: Option<&EmissiveSampler>,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    spatial_index: usize,
    sample_count: u32,
    bounce_count: u32,
    path_depth: usize,
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
) -> Vec3 {
    let mut origin = origin;
    let mut remaining = max_distance;
    let mut throughput = 1.0_f32;
    let mut radiance = Vec3::ZERO;
    for _ in 0..MAX_TRANSPARENT_LAYERS {
        let Some(hit) = nearest_hit(bvh, triangles, origin, direction, remaining) else {
            radiance += escaped_radiance(environment_map, direction, ambient_irradiance);
            break;
        };
        let material = &materials[hit.triangle.material];
        let uv = interpolate_vec2(hit.triangle.uvs, hit.barycentric);
        let color = interpolate_vec4(hit.triangle.colors, hit.barycentric);
        let sample = sample_material(material, uv, color);
        let alpha = sample.alpha;
        if material.alpha_mode == AlphaMode::Mask && alpha < material.alpha_cutoff {
            let hit_position = origin + direction * hit.distance;
            let advance = hit.distance + ray_epsilon_for_triangle(hit_position, hit.triangle);
            origin += direction * advance;
            remaining -= advance;
            continue;
        }
        let opacity = if material.alpha_mode == AlphaMode::Blend {
            alpha
        } else {
            1.0
        };
        let surface = surface_radiance(
            bvh,
            triangles,
            materials,
            lights,
            directional,
            emitters,
            ambient_irradiance,
            environment_map,
            scene_seed,
            spatial_index,
            sample_count,
            bounce_count,
            path_depth,
            origin + direction * hit.distance,
            direction,
            hit,
            sample,
        );
        radiance += surface * throughput * opacity;
        throughput *= 1.0 - opacity;
        if throughput <= 0.001 || opacity >= 1.0 {
            break;
        }
        let hit_position = origin + direction * hit.distance;
        let advance = hit.distance + ray_epsilon_for_triangle(hit_position, hit.triangle);
        origin += direction * advance;
        remaining -= advance;
    }
    radiance
}

#[allow(clippy::too_many_arguments)]
fn surface_radiance(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: Option<&EmissiveSampler>,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    spatial_index: usize,
    sample_count: u32,
    bounce_count: u32,
    path_depth: usize,
    position: Vec3,
    incoming_ray: Vec3,
    hit: SurfaceHit<'_>,
    sample: MaterialSample,
) -> Vec3 {
    let material = &materials[hit.triangle.material];
    let mut normal = interpolate_vec3(hit.triangle.normals, hit.barycentric).normalize_or_zero();
    if normal == Vec3::ZERO {
        return Vec3::ZERO;
    }
    if normal.dot(-incoming_ray) < 0.0 {
        if !material.double_sided {
            return Vec3::ZERO;
        }
        normal = -normal;
    }
    let irradiance = surface_irradiance_with_bounces_at_depth(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        emitters,
        ambient_irradiance,
        environment_map,
        scene_seed,
        spatial_index,
        sample_count,
        bounce_count,
        path_depth,
        position,
        normal,
    );
    irradiance * sample.base_color / std::f32::consts::PI + sample.emissive
}

/// Returns incident diffuse irradiance without applying the receiver albedo.
/// Surface lightmaps store this value divided by PI so the runtime material
/// remains authoritative for base color and texture variation.
#[allow(clippy::too_many_arguments)]
#[cfg(test)]
pub(crate) fn direct_irradiance(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_irradiance: Vec3,
    position: Vec3,
    normal: Vec3,
) -> Vec3 {
    direct_irradiance_with_environment(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        ambient_irradiance,
        None,
        position,
        normal,
    )
}

#[allow(clippy::too_many_arguments)]
fn direct_irradiance_with_environment(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    position: Vec3,
    normal: Vec3,
) -> Vec3 {
    let mut irradiance = ambient_irradiance;
    if let Some(environment_map) = environment_map {
        irradiance +=
            environment_irradiance(bvh, triangles, materials, environment_map, normal, position);
    }
    for light in lights {
        let to_light = Vec3::from_array(light.translation) - position;
        let distance_squared = to_light.length_squared();
        let range_squared = light.radius * light.radius;
        if distance_squared <= 1e-6 || distance_squared >= range_squared {
            continue;
        }
        let distance = distance_squared.sqrt();
        let direction = to_light / distance;
        let n_dot_l = normal.dot(direction).max(0.0);
        if n_dot_l <= 0.0 {
            continue;
        }
        let angular_factor = spot_angular_factor(light, -direction);
        if angular_factor <= 0.0 {
            continue;
        }
        let offset = ray_epsilon(position);
        let shadow_origin = position + normal * offset;
        let shadow_distance = (to_light - normal * offset).length();
        let visibility = trace_visibility(
            bvh,
            triangles,
            materials,
            shadow_origin,
            direction,
            shadow_distance,
        );
        if visibility <= 0.0 {
            continue;
        }
        let factor = distance_squared / range_squared.max(0.0001);
        let smooth = (1.0 - factor * factor).clamp(0.0, 1.0).powi(2);
        let attenuation = smooth / distance_squared.max(0.0001);
        let intensity =
            point_light_intensity(light.radius, light.intensity_lumens, DEFAULT_LIGHTING_SCALE)
                / (4.0 * std::f32::consts::PI);
        irradiance += Vec3::from_array(srgb_to_linear_rgb([
            light.color_rgba[0],
            light.color_rgba[1],
            light.color_rgba[2],
        ])) * intensity
            * attenuation
            * n_dot_l
            * angular_factor
            * visibility;
    }
    if directional.illuminance > 0.0 {
        let direction = Quat::from_array(directional.rotation_xyzw) * Vec3::Z;
        let n_dot_l = normal.dot(direction).max(0.0);
        if n_dot_l > 0.0 {
            let offset = ray_epsilon(position);
            let visibility = trace_visibility(
                bvh,
                triangles,
                materials,
                position + normal * offset,
                direction,
                f32::INFINITY,
            );
            irradiance += Vec3::from_array(srgb_to_linear_rgb([
                directional.color_rgba[0],
                directional.color_rgba[1],
                directional.color_rgba[2],
            ])) * directional.illuminance
                * n_dot_l
                * visibility;
        }
    }
    irradiance
}

fn escaped_radiance(
    environment_map: Option<&EnvironmentMap>,
    direction: Vec3,
    ambient_irradiance: Vec3,
) -> Vec3 {
    environment_map.map_or(ambient_irradiance / std::f32::consts::PI, |map| {
        Vec3::from_array(map.sample(direction.to_array()))
            + ambient_irradiance / std::f32::consts::PI
    })
}

#[allow(clippy::too_many_arguments)]
fn environment_irradiance(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    environment_map: &EnvironmentMap,
    normal: Vec3,
    position: Vec3,
) -> Vec3 {
    let normal = normal.normalize_or_zero();
    if normal == Vec3::ZERO {
        return Vec3::ZERO;
    }
    // An empty scene cannot occlude the environment. Keep the exact constant
    // result for this common/reference case; non-constant maps and populated
    // scenes use the MIS estimator below.
    if triangles.is_empty()
        && let Some(radiance) = environment_map.constant_radiance()
    {
        return Vec3::from_array(radiance) * std::f32::consts::PI;
    }
    let seed = u64::from(position.x.to_bits())
        ^ (u64::from(position.y.to_bits()) << 21)
        ^ (u64::from(position.z.to_bits()) << 42);
    let spatial_index = seed as usize;
    let offset = ray_epsilon(position);
    let shadow_origin = position + normal * offset;
    let mut cosine_sum = Vec3::ZERO;
    let mut environment_sum = Vec3::ZERO;
    for sample_index in 0..ENVIRONMENT_IRRADIANCE_SAMPLE_COUNT {
        // BSDF strategy: cosine-weighted hemisphere sample.
        let cosine_direction = cosine_hemisphere_direction(
            normal,
            seed as u32,
            sample_index,
            ENVIRONMENT_IRRADIANCE_SAMPLE_COUNT,
        );
        let cosine = normal.dot(cosine_direction).max(0.0);
        let cosine_pdf = cosine / std::f32::consts::PI;
        if cosine_pdf > 0.0 {
            let visibility = trace_visibility(
                bvh,
                triangles,
                materials,
                shadow_origin,
                cosine_direction,
                f32::INFINITY,
            );
            let environment_pdf = environment_map.pdf_solid_angle(cosine_direction.to_array());
            let weight = power_heuristic(cosine_pdf, environment_pdf);
            cosine_sum += Vec3::from_array(environment_map.sample(cosine_direction.to_array()))
                * (cosine * visibility * weight / cosine_pdf);
        }

        // Environment strategy: sample the HDR map's luminance/solid-angle
        // distribution and combine it with the cosine PDF using MIS.
        let environment_sample = environment_map.sample_importance(
            sample_uniform_1d(seed ^ 0x4f1b_2d39_8a6e_70c5, spatial_index, sample_index),
            sample_uniform_1d(seed ^ 0xb7e1_5a94_32c8_f06d, spatial_index, sample_index),
        );
        let environment_direction = Vec3::from_array(environment_sample.direction);
        let environment_cosine = normal.dot(environment_direction).max(0.0);
        let environment_pdf = environment_sample.pdf_solid_angle;
        if environment_cosine > 0.0 && environment_pdf > 0.0 {
            let visibility = trace_visibility(
                bvh,
                triangles,
                materials,
                shadow_origin,
                environment_direction,
                f32::INFINITY,
            );
            let cosine_pdf = environment_cosine / std::f32::consts::PI;
            let weight = power_heuristic(environment_pdf, cosine_pdf);
            environment_sum += Vec3::from_array(environment_sample.radiance)
                * (environment_cosine * visibility * weight / environment_pdf);
        }
    }
    (cosine_sum + environment_sum) / ENVIRONMENT_IRRADIANCE_SAMPLE_COUNT as f32
}

/// Evaluates the authored LIGH spotlight cone for a direction from the light
/// toward the receiver. Fallout stores FOV as a full cone angle in degrees;
/// preparation converts it to radians, while the runtime Bevy contract uses
/// the same forward (-Z) transform convention. A missing/invalid cone falls
/// back to point-light behavior so old 12-byte LIGH DATA remains compatible.
pub(crate) fn spot_angular_factor(light: &JobLight, direction_from_light: Vec3) -> f32 {
    let is_spot = light.kind.eq_ignore_ascii_case("spot") || light.flags & 0x200 != 0;
    let fov = light.spot_fov_radians;
    if !is_spot || !fov.is_finite() || fov <= f32::EPSILON {
        return 1.0;
    }
    let axis = (Quat::from_array(light.rotation_xyzw) * Vec3::NEG_Z).normalize_or_zero();
    let direction_from_light = direction_from_light.normalize_or_zero();
    if axis == Vec3::ZERO || direction_from_light == Vec3::ZERO {
        return 0.0;
    }
    let outer_angle = (fov * 0.5).clamp(f32::EPSILON, std::f32::consts::FRAC_PI_2);
    let cosine = axis.dot(direction_from_light);
    let outer_cosine = outer_angle.cos();
    if cosine <= outer_cosine {
        return 0.0;
    }
    let inner_cosine = (outer_angle * 0.8).cos();
    let blend = ((cosine - outer_cosine) / (inner_cosine - outer_cosine)).clamp(0.0, 1.0);
    if light.spot_falloff_exponent > 0.0 {
        blend.powf(light.spot_falloff_exponent)
    } else {
        1.0
    }
}

/// Adds deterministic cosine-weighted diffuse transport to the direct
/// incident irradiance. `bounce_count` counts secondary surfaces reached from
/// the receiver; a value of one evaluates direct radiance at the first hit,
/// while larger values recurse through additional diffuse surfaces.
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn surface_irradiance_with_bounces(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_irradiance: Vec3,
    scene_seed: u64,
    spatial_index: usize,
    sample_count: u32,
    bounce_count: u32,
    position: Vec3,
    normal: Vec3,
) -> Vec3 {
    surface_irradiance_with_bounces_at_depth(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        None,
        ambient_irradiance,
        None,
        scene_seed,
        spatial_index,
        sample_count,
        bounce_count,
        0,
        position,
        normal,
    )
}

#[allow(clippy::too_many_arguments)]
#[cfg(test)]
pub(crate) fn surface_irradiance_with_emissive(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: &EmissiveSampler,
    ambient_irradiance: Vec3,
    scene_seed: u64,
    spatial_index: usize,
    sample_count: u32,
    bounce_count: u32,
    position: Vec3,
    normal: Vec3,
) -> Vec3 {
    surface_irradiance_with_emissive_and_environment(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        emitters,
        ambient_irradiance,
        None,
        scene_seed,
        spatial_index,
        sample_count,
        bounce_count,
        position,
        normal,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn surface_irradiance_with_emissive_and_environment(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: &EmissiveSampler,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    spatial_index: usize,
    sample_count: u32,
    bounce_count: u32,
    position: Vec3,
    normal: Vec3,
) -> Vec3 {
    surface_irradiance_with_bounces_at_depth(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        Some(emitters),
        ambient_irradiance,
        environment_map,
        scene_seed,
        spatial_index,
        sample_count,
        bounce_count,
        0,
        position,
        normal,
    )
}

#[allow(clippy::too_many_arguments)]
fn surface_irradiance_with_bounces_at_depth(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: Option<&EmissiveSampler>,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    spatial_index: usize,
    sample_count: u32,
    bounce_count: u32,
    path_depth: usize,
    position: Vec3,
    normal: Vec3,
) -> Vec3 {
    let direct = direct_irradiance_with_environment(
        bvh,
        triangles,
        materials,
        lights,
        directional,
        ambient_irradiance,
        environment_map,
        position,
        normal,
    );
    let emissive = if bounce_count == 0 {
        Vec3::ZERO
    } else {
        emitters.map_or(Vec3::ZERO, |emitters| {
            sample_emissive_irradiance(
                bvh,
                triangles,
                materials,
                emitters,
                scene_seed,
                spatial_index,
                path_depth,
                position,
                normal,
            )
            .map_or(Vec3::ZERO, |sample| {
                sample.irradiance * power_heuristic(sample.pdf_solid_angle, sample.bsdf_pdf)
            })
        })
    };
    direct
        + emissive
        + sample_indirect_irradiance(
            bvh,
            triangles,
            materials,
            lights,
            directional,
            emitters,
            ambient_irradiance,
            environment_map,
            scene_seed,
            spatial_index,
            sample_count,
            bounce_count,
            path_depth,
            position,
            normal,
        )
}

#[allow(clippy::too_many_arguments)]
fn sample_indirect_irradiance(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: Option<&EmissiveSampler>,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    spatial_index: usize,
    sample_count: u32,
    bounce_count: u32,
    path_depth: usize,
    position: Vec3,
    normal: Vec3,
) -> Vec3 {
    if sample_count == 0 || bounce_count == 0 {
        return Vec3::ZERO;
    }
    let mut radiance = Vec3::ZERO;
    for sample_index in 0..sample_count {
        let direction = cosine_hemisphere_direction(
            normal,
            sample_seed(scene_seed, spatial_index, sample_index),
            sample_index,
            sample_count,
        );
        let bsdf_pdf = normal.dot(direction).max(0.0) / std::f32::consts::PI;
        radiance += trace_indirect_path(
            bvh,
            triangles,
            materials,
            lights,
            directional,
            emitters,
            ambient_irradiance,
            environment_map,
            scene_seed,
            spatial_index
                .wrapping_mul(0x9e37_79b9)
                .wrapping_add(sample_index as usize),
            sample_count,
            bounce_count - 1,
            path_depth,
            bsdf_pdf,
            position + normal * ray_epsilon(position),
            direction,
            f32::INFINITY,
        );
    }
    radiance * (std::f32::consts::PI / sample_count as f32)
}

#[allow(clippy::too_many_arguments)]
fn trace_indirect_path(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: Option<&EmissiveSampler>,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    spatial_index: usize,
    sample_count: u32,
    bounce_count: u32,
    path_depth: usize,
    bsdf_pdf: f32,
    mut origin: Vec3,
    direction: Vec3,
    mut remaining: f32,
) -> Vec3 {
    let mut throughput = 1.0_f32;
    let mut radiance = Vec3::ZERO;
    for _ in 0..MAX_TRANSPARENT_LAYERS {
        let Some(hit) = nearest_hit(bvh, triangles, origin, direction, remaining) else {
            // The previous surface already evaluated the authored environment
            // through direct_irradiance_with_environment. Adding an escaped
            // environment sample here would count that transport vertex twice.
            break;
        };
        let material = &materials[hit.triangle.material];
        let uv = interpolate_vec2(hit.triangle.uvs, hit.barycentric);
        let color = interpolate_vec4(hit.triangle.colors, hit.barycentric);
        let sample = sample_material(material, uv, color);
        if material.alpha_mode == AlphaMode::Mask && sample.alpha < material.alpha_cutoff {
            let hit_position = origin + direction * hit.distance;
            let advance = hit.distance + ray_epsilon_for_triangle(hit_position, hit.triangle);
            origin += direction * advance;
            remaining -= advance;
            continue;
        }
        let opacity = if material.alpha_mode == AlphaMode::Blend {
            sample.alpha
        } else {
            1.0
        };
        let position = origin + direction * hit.distance;
        let mut normal =
            interpolate_vec3(hit.triangle.normals, hit.barycentric).normalize_or_zero();
        if normal == Vec3::ZERO {
            return radiance;
        }
        if normal.dot(-direction) < 0.0 {
            if !material.double_sided {
                return radiance;
            }
            normal = -normal;
        }
        let direct = direct_irradiance_with_environment(
            bvh,
            triangles,
            materials,
            lights,
            directional,
            ambient_irradiance,
            environment_map,
            position,
            normal,
        );
        let direct = direct
            + emitters.map_or(Vec3::ZERO, |emitters| {
                sample_emissive_irradiance(
                    bvh,
                    triangles,
                    materials,
                    emitters,
                    scene_seed,
                    spatial_index,
                    path_depth,
                    position,
                    normal,
                )
                .map_or(Vec3::ZERO, |sample| {
                    sample.irradiance * power_heuristic(sample.pdf_solid_angle, sample.bsdf_pdf)
                })
            });
        let indirect = if bounce_count == 0 {
            Vec3::ZERO
        } else {
            let survival = russian_roulette_survival(path_depth, sample.base_color, opacity);
            let roulette =
                sample_uniform_1d(scene_seed, spatial_index ^ 0xa511_e9b3, path_depth as u32);
            if roulette > survival {
                Vec3::ZERO
            } else {
                sample_indirect_irradiance(
                    bvh,
                    triangles,
                    materials,
                    lights,
                    directional,
                    emitters,
                    ambient_irradiance,
                    environment_map,
                    scene_seed,
                    spatial_index,
                    sample_count,
                    bounce_count,
                    path_depth + 1,
                    position,
                    normal,
                ) / survival
            }
        };
        let emission_weight = if sample.emissive.max_element() > 0.0 {
            emitters
                .and_then(|emitters| {
                    emissive_pdf_for_hit(
                        emitters,
                        triangles,
                        materials,
                        hit.triangle,
                        hit.barycentric,
                        origin,
                        direction,
                    )
                })
                .map_or(1.0, |nee_pdf| power_heuristic(bsdf_pdf, nee_pdf))
        } else {
            1.0
        };
        let surface = (direct + indirect) * sample.base_color / std::f32::consts::PI
            + sample.emissive * emission_weight;
        radiance += surface * throughput * opacity;
        throughput *= 1.0 - opacity;
        if throughput <= 0.001 || opacity >= 1.0 {
            break;
        }
        let hit_position = origin + direction * hit.distance;
        let advance = hit.distance + ray_epsilon_for_triangle(hit_position, hit.triangle);
        origin += direction * advance;
        remaining -= advance;
    }
    radiance
}

fn russian_roulette_survival(path_depth: usize, base_color: Vec3, opacity: f32) -> f32 {
    if path_depth < RUSSIAN_ROULETTE_START_BOUNCE {
        return 1.0;
    }
    (base_color
        .max_element()
        .clamp(MIN_RUSSIAN_ROULETTE_SURVIVAL, MAX_RUSSIAN_ROULETTE_SURVIVAL)
        * opacity.clamp(0.0, 1.0))
    .clamp(MIN_RUSSIAN_ROULETTE_SURVIVAL, MAX_RUSSIAN_ROULETTE_SURVIVAL)
}

#[derive(Clone, Copy, Debug)]
struct EmissiveLightSample {
    irradiance: Vec3,
    pdf_solid_angle: f32,
    bsdf_pdf: f32,
}

#[allow(clippy::too_many_arguments)]
fn sample_emissive_irradiance(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    emitters: &EmissiveSampler,
    scene_seed: u64,
    spatial_index: usize,
    path_depth: usize,
    position: Vec3,
    normal: Vec3,
) -> Option<EmissiveLightSample> {
    if emitters.is_empty() {
        return None;
    }
    let entry = emitters.select(sample_uniform_1d(
        scene_seed,
        spatial_index ^ 0x63d8_35f1,
        path_depth as u32,
    ));
    let triangle = triangles.get(entry.triangle_index)?;
    let material = materials.get(triangle.material)?;
    let barycentric = sample_triangle_barycentric(
        sample_uniform_1d(
            scene_seed,
            spatial_index ^ 0x9e37_79b9,
            path_depth as u32 + 1,
        ),
        sample_uniform_1d(
            scene_seed,
            spatial_index ^ 0xd1b5_4a32,
            path_depth as u32 + 2,
        ),
    );
    let emitter_position = vec3(interpolate_point3(triangle.vertices, barycentric));
    let emitter_normal = interpolate_vec3(triangle.normals, barycentric).normalize_or_zero();
    if emitter_normal == Vec3::ZERO {
        return None;
    }
    let sample = sample_material(
        material,
        interpolate_vec2(triangle.uvs, barycentric),
        interpolate_vec4(triangle.colors, barycentric),
    );
    if material.alpha_mode == AlphaMode::Mask && sample.alpha < material.alpha_cutoff {
        return None;
    }
    let opacity = if material.alpha_mode == AlphaMode::Blend {
        sample.alpha
    } else {
        1.0
    };
    let emission = sample.emissive * opacity;
    if emission.max_element() <= 0.0 || !emission.is_finite() {
        return None;
    }
    let to_emitter = emitter_position - position;
    let distance_squared = to_emitter.length_squared();
    if distance_squared <= 1.0e-6 || !distance_squared.is_finite() {
        return None;
    }
    let distance = distance_squared.sqrt();
    let direction = to_emitter / distance;
    let receiver_cosine = normal.dot(direction).max(0.0);
    if receiver_cosine <= 0.0 {
        return None;
    }
    let emitter_cosine = if material.double_sided {
        emitter_normal.dot(-direction).abs()
    } else {
        emitter_normal.dot(-direction).max(0.0)
    };
    if emitter_cosine <= 0.0 {
        return None;
    }
    let offset = ray_epsilon(position);
    let shadow_distance = (to_emitter - normal * offset).length();
    let visibility = trace_visibility(
        bvh,
        triangles,
        materials,
        position + normal * offset,
        direction,
        (shadow_distance - ray_epsilon(emitter_position)).max(0.0),
    );
    if visibility <= 0.0 {
        return None;
    }
    let selection_probability = emitters.selection_probability(entry.triangle_index);
    if selection_probability <= 0.0 || !selection_probability.is_finite() {
        return None;
    }
    let area = triangle_area(triangle);
    let pdf_solid_angle = selection_probability * distance_squared / (area * emitter_cosine);
    if area <= 0.0 || !pdf_solid_angle.is_finite() || pdf_solid_angle <= 0.0 {
        return None;
    }
    Some(EmissiveLightSample {
        irradiance: emission * receiver_cosine * visibility / pdf_solid_angle,
        pdf_solid_angle,
        bsdf_pdf: receiver_cosine / std::f32::consts::PI,
    })
}

fn emissive_pdf_for_hit(
    emitters: &EmissiveSampler,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    triangle: &IrradianceTriangle,
    barycentric: [f32; 3],
    origin: Vec3,
    direction: Vec3,
) -> Option<f32> {
    let triangle_index = triangles
        .iter()
        .position(|candidate| std::ptr::eq(candidate, triangle))?;
    let selection_probability = emitters.selection_probability(triangle_index);
    if selection_probability <= 0.0 || !selection_probability.is_finite() {
        return None;
    }
    let material = materials.get(triangle.material)?;
    let sample = sample_material(
        material,
        interpolate_vec2(triangle.uvs, barycentric),
        interpolate_vec4(triangle.colors, barycentric),
    );
    if material.alpha_mode == AlphaMode::Mask && sample.alpha < material.alpha_cutoff {
        return None;
    }
    if sample.emissive.max_element() <= 0.0 || !sample.emissive.is_finite() {
        return None;
    }
    let emitter_normal = interpolate_vec3(triangle.normals, barycentric).normalize_or_zero();
    if emitter_normal == Vec3::ZERO {
        return None;
    }
    let emitter_cosine = if material.double_sided {
        emitter_normal.dot(-direction).abs()
    } else {
        emitter_normal.dot(-direction).max(0.0)
    };
    if emitter_cosine <= 0.0 {
        return None;
    }
    let emitter_position = vec3(interpolate_point3(triangle.vertices, barycentric));
    let distance_squared = emitter_position.distance_squared(origin);
    let area = triangle_area(triangle);
    let pdf = selection_probability * distance_squared / (area * emitter_cosine);
    (pdf.is_finite() && pdf > 0.0).then_some(pdf)
}

fn power_heuristic(pdf_a: f32, pdf_b: f32) -> f32 {
    let a_squared = pdf_a * pdf_a;
    let b_squared = pdf_b * pdf_b;
    if !a_squared.is_finite() || !b_squared.is_finite() {
        return 0.0;
    }
    a_squared / (a_squared + b_squared).max(f32::MIN_POSITIVE)
}

fn trace_visibility(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    mut origin: Vec3,
    direction: Vec3,
    mut max_distance: f32,
) -> f32 {
    let mut visibility = 1.0;
    for _ in 0..MAX_TRANSPARENT_LAYERS {
        let Some(hit) = nearest_hit(bvh, triangles, origin, direction, max_distance) else {
            break;
        };
        let material = &materials[hit.triangle.material];
        let uv = interpolate_vec2(hit.triangle.uvs, hit.barycentric);
        let color = interpolate_vec4(hit.triangle.colors, hit.barycentric);
        let alpha = sample_material(material, uv, color).alpha;
        match material.alpha_mode {
            AlphaMode::Opaque => return 0.0,
            AlphaMode::Mask if alpha >= material.alpha_cutoff => return 0.0,
            AlphaMode::Mask => {}
            AlphaMode::Blend => {
                visibility *= 1.0 - alpha;
                if visibility <= 0.001 {
                    return 0.0;
                }
            }
        }
        let hit_position = origin + direction * hit.distance;
        let advance = hit.distance + ray_epsilon_for_triangle(hit_position, hit.triangle);
        origin += direction * advance;
        max_distance -= advance;
    }
    visibility
}

fn nearest_hit<'a>(
    bvh: &Bvh<f32, 3>,
    triangles: &'a [IrradianceTriangle],
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
) -> Option<SurfaceHit<'a>> {
    let ray = Ray::new(point3(origin), vector3(direction));
    bvh.traverse_iterator(&ray, triangles)
        .filter_map(|triangle| intersect_triangle(origin, direction, triangle))
        .filter(|hit| {
            hit.distance > ray_epsilon_for_triangle(origin, hit.triangle)
                && hit.distance < max_distance
        })
        .min_by(|left, right| left.distance.total_cmp(&right.distance))
}

/// Keep ray-origin offsets above the local f32 coordinate error without
/// allowing large world coordinates to turn a contact offset into a visible
/// gap. The minimum preserves the existing small-scene behavior; the cap keeps
/// the offset below the scale of ordinary Fallout geometry.
fn ray_epsilon(position: Vec3) -> f32 {
    let magnitude = position.abs().max_element();
    if !magnitude.is_finite() {
        return RAY_EPSILON;
    }
    let coordinate_error = 16.0 * f32::EPSILON * magnitude.max(1.0);
    RAY_EPSILON.max(coordinate_error).min(MAX_RAY_OFFSET)
}

fn ray_epsilon_for_triangle(position: Vec3, triangle: &IrradianceTriangle) -> f32 {
    let edges = [
        vec3(triangle.vertices[1]) - vec3(triangle.vertices[0]),
        vec3(triangle.vertices[2]) - vec3(triangle.vertices[1]),
        vec3(triangle.vertices[0]) - vec3(triangle.vertices[2]),
    ];
    let geometry_error = edges.into_iter().map(Vec3::length).fold(0.0, f32::max) * 1.0e-4;
    ray_epsilon(position)
        .max(geometry_error)
        .min(MAX_RAY_OFFSET)
}

fn intersect_triangle<'a>(
    origin: Vec3,
    direction: Vec3,
    triangle: &'a IrradianceTriangle,
) -> Option<SurfaceHit<'a>> {
    let v0 = vec3(triangle.vertices[0]);
    let v1 = vec3(triangle.vertices[1]);
    let v2 = vec3(triangle.vertices[2]);
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let p = direction.cross(edge2);
    let determinant = edge1.dot(p);
    if determinant.abs() <= 1e-8 {
        return None;
    }
    let inverse = determinant.recip();
    let t = origin - v0;
    let u = t.dot(p) * inverse;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = t.cross(edge1);
    let v = direction.dot(q) * inverse;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = edge2.dot(q) * inverse;
    (distance > 0.0).then_some(SurfaceHit {
        triangle,
        distance,
        barycentric: [1.0 - u - v, u, v],
    })
}

fn probe_position(index: usize, resolution: [u32; 3], minimum: Vec3, maximum: Vec3) -> Vec3 {
    let [rx, ry, rz] = resolution;
    let x = index as u32 % rx;
    let y = (index as u32 / rx) % ry;
    let z = index as u32 / (rx * ry);
    let fraction = Vec3::new(
        x as f32 / (rx - 1) as f32,
        y as f32 / (ry - 1) as f32,
        z as f32 / (rz - 1) as f32,
    );
    minimum + (maximum - minimum) * fraction
}

fn write_atlas(
    raw_path: &Path,
    resolution: [u32; 3],
    values: &[[Vec3; 6]],
) -> Result<Vec<PathBuf>> {
    let [rx, ry, rz] = resolution;
    let [atlas_width, atlas_height, atlas_depth] = atlas_dimensions(resolution);
    let width = atlas_width as usize;
    let height = atlas_height as usize;
    let depth = atlas_depth as usize;
    let mut words = vec![vec![0_u32; width * height]; depth];
    for z in 0..rz as usize {
        for y in 0..ry as usize {
            for x in 0..rx as usize {
                let sample = z * ry as usize * rx as usize + y * rx as usize + x;
                for (side, color) in values[sample].iter().enumerate() {
                    let (side_y, side_z) = match side {
                        0 => (y + ry as usize, z),
                        1 => (y + ry as usize, z + rz as usize),
                        2 => (y + ry as usize, z + 2 * rz as usize),
                        3 => (y, z),
                        4 => (y, z + rz as usize),
                        5 => (y, z + 2 * rz as usize),
                        _ => unreachable!(),
                    };
                    words[side_z][side_y * width + x] = pack_rgb9e5(*color);
                }
            }
        }
    }
    let stem = raw_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("irradiance");
    let extension = raw_path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("raw");
    let mut paths = Vec::with_capacity(depth);
    for (slice, words) in words.into_iter().enumerate() {
        let path = raw_path.with_file_name(format!("{stem}_{slice:04}.{extension}"));
        let mut file = fs::File::create(&path)
            .with_context(|| format!("could not create {}", path.display()))?;
        for word in words {
            file.write_all(&word.to_le_bytes())?;
        }
        paths.push(path);
    }
    Ok(paths)
}

fn pack_rgb9e5(color: Vec3) -> u32 {
    let color = color.clamp(Vec3::ZERO, Vec3::splat(65_408.0));
    let maximum = color.max_element();
    if maximum <= f32::MIN_POSITIVE {
        return 0;
    }
    let mut exponent = (maximum.log2().floor() as i32 + 16).clamp(1, 31);
    let mut scale = 2.0_f32.powi(9 - (exponent - 15));
    let mut mantissas = color.to_array().map(|value| (value * scale).round() as u32);
    if mantissas.iter().any(|value| *value > 511) && exponent < 31 {
        exponent += 1;
        scale = 2.0_f32.powi(9 - (exponent - 15));
        mantissas = color
            .to_array()
            .map(|value| (value * scale).round().min(511.0) as u32);
    }
    mantissas[0] | (mantissas[1] << 9) | (mantissas[2] << 18) | ((exponent as u32) << 27)
}

fn interpolate_vec2(values: [Vec2; 3], weights: [f32; 3]) -> Vec2 {
    values[0] * weights[0] + values[1] * weights[1] + values[2] * weights[2]
}

fn interpolate_point3(values: [Point3<f32>; 3], weights: [f32; 3]) -> Point3<f32> {
    Point3::from(
        values[0].coords * weights[0]
            + values[1].coords * weights[1]
            + values[2].coords * weights[2],
    )
}

fn interpolate_vec3(values: [Vec3; 3], weights: [f32; 3]) -> Vec3 {
    values[0] * weights[0] + values[1] * weights[1] + values[2] * weights[2]
}

fn interpolate_vec4(values: [Vec4; 3], weights: [f32; 3]) -> Vec4 {
    values[0] * weights[0] + values[1] * weights[1] + values[2] * weights[2]
}

fn point3(value: Vec3) -> Point3<f32> {
    Point3::new(value.x, value.y, value.z)
}

fn vector3(value: Vec3) -> Vector3<f32> {
    Vector3::new(value.x, value.y, value.z)
}

fn vec3(value: Point3<f32>) -> Vec3 {
    Vec3::new(value.x, value.y, value.z)
}

#[cfg(test)]
#[path = "tests/rust_irradiance.rs"]
mod tests;
