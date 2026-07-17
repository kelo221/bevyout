use super::super::dynamic_lighting::DEFAULT_BOUNCE_MULTIPLIER;
use super::JobLight;
use super::policy::{
    AMBIENT_CUBE_FACE_COUNT, atlas_dimensions, primary_ray_count, volume_resolution,
};
use super::rust_scene::{AlphaMode, RustBakeScene, TransportMaterial};
use anyhow::{Context, Result, bail};
use bevy::math::{Quat, Vec2, Vec3, Vec4};
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
const DEFAULT_LIGHTING_SCALE: f32 = 128.0;
const RAY_EPSILON: f32 = 0.002;
const MAX_TRANSPARENT_LAYERS: usize = 8;
const EMISSION_SCALE: f32 = 0.01;

#[derive(Clone, Debug)]
struct IrradianceTriangle {
    vertices: [Point3<f32>; 3],
    normals: [Vec3; 3],
    uvs: [Vec2; 3],
    colors: [Vec4; 3],
    material: usize,
    node_index: usize,
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
    pub(crate) bounce_multiplier: f32,
}

pub(crate) fn bake_irradiance(
    scene: &RustBakeScene,
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
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
    let minimum = scene.bounds.minimum - Vec3::splat(spacing);
    let maximum = scene.bounds.maximum + Vec3::splat(spacing);
    let scale = maximum - minimum;
    let resolution = volume_resolution(scale.to_array(), spacing);
    let translation = (minimum + maximum) * 0.5;
    let probe_count = resolution.iter().product::<u32>() as usize;
    debug_assert_eq!(FACE_DIRECTIONS.len(), AMBIENT_CUBE_FACE_COUNT);
    let primary_rays = primary_ray_count(resolution, samples);
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

fn collect_triangles(scene: &RustBakeScene) -> Result<Vec<IrradianceTriangle>> {
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

#[allow(clippy::too_many_arguments)]
fn sample_face(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    origin: Vec3,
    face_normal: Vec3,
    probe_index: usize,
    samples: u32,
    max_distance: f32,
) -> Vec3 {
    let mut sum = Vec3::ZERO;
    for sample in 0..samples {
        let direction = cosine_hemisphere_direction(face_normal, probe_index, sample, samples);
        sum += trace_radiance(
            bvh,
            triangles,
            materials,
            lights,
            directional,
            origin,
            direction,
            max_distance,
        );
    }
    sum * (std::f32::consts::PI / samples.max(1) as f32)
}

#[allow(clippy::too_many_arguments)]
fn trace_radiance(
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
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
            break;
        };
        let material = &materials[hit.triangle.material];
        let uv = interpolate_vec2(hit.triangle.uvs, hit.barycentric);
        let color = interpolate_vec4(hit.triangle.colors, hit.barycentric);
        let sampled = material
            .base_color_texture
            .as_ref()
            .map_or(Vec4::ONE, |texture| texture.sample(uv));
        let alpha = (material.base_color_factor.w * sampled.w * color.w).clamp(0.0, 1.0);
        if material.alpha_mode == AlphaMode::Mask && alpha < material.alpha_cutoff {
            let advance = hit.distance + RAY_EPSILON;
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
            origin + direction * hit.distance,
            direction,
            hit,
            uv,
            sampled,
            color,
        );
        radiance += surface * throughput * opacity;
        throughput *= 1.0 - opacity;
        if throughput <= 0.001 || opacity >= 1.0 {
            break;
        }
        let advance = hit.distance + RAY_EPSILON;
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
    position: Vec3,
    incoming_ray: Vec3,
    hit: SurfaceHit<'_>,
    uv: Vec2,
    sampled_base: Vec4,
    vertex_color: Vec4,
) -> Vec3 {
    let material = &materials[hit.triangle.material];
    let mut normal = interpolate_vec3(hit.triangle.normals, hit.barycentric).normalize_or_zero();
    if normal.dot(-incoming_ray) < 0.0 {
        if !material.double_sided {
            return Vec3::ZERO;
        }
        normal = -normal;
    }
    let base = srgb_to_linear_vec3(material.base_color_factor.truncate())
        * srgb_to_linear_vec3(sampled_base.truncate())
        * srgb_to_linear_vec3(vertex_color.truncate());
    let diffuse = base * (1.0 - material.metallic_factor).clamp(0.0, 1.0);
    let mut irradiance = Vec3::ZERO;
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
        let visibility = trace_visibility(
            bvh,
            triangles,
            materials,
            position + normal * RAY_EPSILON,
            direction,
            distance - RAY_EPSILON,
        );
        if visibility <= 0.0 {
            continue;
        }
        let factor = distance_squared / range_squared.max(0.0001);
        let smooth = (1.0 - factor * factor).clamp(0.0, 1.0).powi(2);
        let attenuation = smooth / distance_squared.max(0.0001);
        let intensity = light.radius * light.radius * 2.0 * DEFAULT_LIGHTING_SCALE
            / (4.0 * std::f32::consts::PI);
        irradiance += srgb_to_linear_vec3(Vec3::from_array([
            light.color_rgba[0],
            light.color_rgba[1],
            light.color_rgba[2],
        ])) * intensity
            * attenuation
            * n_dot_l
            * visibility
            * effective_bounce_multiplier(light.bounce_multiplier);
    }
    if directional.illuminance > 0.0 {
        let direction = Quat::from_array(directional.rotation_xyzw) * Vec3::Z;
        let n_dot_l = normal.dot(direction).max(0.0);
        if n_dot_l > 0.0 {
            let visibility = trace_visibility(
                bvh,
                triangles,
                materials,
                position + normal * RAY_EPSILON,
                direction,
                f32::INFINITY,
            );
            irradiance += srgb_to_linear_vec3(Vec3::from_array([
                directional.color_rgba[0],
                directional.color_rgba[1],
                directional.color_rgba[2],
            ])) * directional.illuminance
                * n_dot_l
                * visibility
                * effective_bounce_multiplier(directional.bounce_multiplier);
        }
    }
    let emissive_sample = material
        .emissive_texture
        .as_ref()
        .map_or(Vec4::ONE, |texture| texture.sample(uv));
    let emissive = srgb_to_linear_vec3(material.emissive_factor)
        * srgb_to_linear_vec3(emissive_sample.truncate())
        * EMISSION_SCALE;
    diffuse * irradiance / std::f32::consts::PI + emissive
}

fn effective_bounce_multiplier(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        DEFAULT_BOUNCE_MULTIPLIER
    }
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
        let sampled = material
            .base_color_texture
            .as_ref()
            .map_or(Vec4::ONE, |texture| texture.sample(uv));
        let alpha = (material.base_color_factor.w * sampled.w * color.w).clamp(0.0, 1.0);
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
        let advance = hit.distance + RAY_EPSILON;
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
        .filter(|hit| hit.distance > RAY_EPSILON && hit.distance < max_distance)
        .min_by(|left, right| left.distance.total_cmp(&right.distance))
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

fn cosine_hemisphere_direction(normal: Vec3, probe: usize, sample: u32, count: u32) -> Vec3 {
    let u = (sample as f32 + 0.5) / count.max(1) as f32;
    let scramble = hash_u32((probe as u32).wrapping_mul(0x9e37_79b9));
    let v = radical_inverse(sample ^ scramble);
    let radius = u.sqrt();
    let angle = std::f32::consts::TAU * v;
    let local = Vec3::new(radius * angle.cos(), radius * angle.sin(), (1.0 - u).sqrt());
    let tangent = if normal.z.abs() < 0.999 {
        normal.cross(Vec3::Z).normalize()
    } else {
        normal.cross(Vec3::Y).normalize()
    };
    let bitangent = normal.cross(tangent);
    (tangent * local.x + bitangent * local.y + normal * local.z).normalize()
}

fn radical_inverse(bits: u32) -> f32 {
    bits.reverse_bits() as f32 * 2.328_306_4e-10
}

fn hash_u32(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^ (value >> 16)
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

fn interpolate_vec3(values: [Vec3; 3], weights: [f32; 3]) -> Vec3 {
    values[0] * weights[0] + values[1] * weights[1] + values[2] * weights[2]
}

fn interpolate_vec4(values: [Vec4; 3], weights: [f32; 3]) -> Vec4 {
    values[0] * weights[0] + values[1] * weights[1] + values[2] * weights[2]
}

fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn srgb_to_linear_vec3(value: Vec3) -> Vec3 {
    Vec3::new(
        srgb_to_linear(value.x),
        srgb_to_linear(value.y),
        srgb_to_linear(value.z),
    )
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
mod tests {
    use super::*;

    #[test]
    fn hemisphere_samples_are_deterministic_and_inside_the_face() {
        let first = (0..64)
            .map(|sample| cosine_hemisphere_direction(Vec3::Y, 12, sample, 64))
            .collect::<Vec<_>>();
        let second = (0..64)
            .map(|sample| cosine_hemisphere_direction(Vec3::Y, 12, sample, 64))
            .collect::<Vec<_>>();
        assert_eq!(first, second);
        assert!(first.iter().all(|direction| direction.dot(Vec3::Y) >= 0.0));
    }

    #[test]
    fn rgb9e5_black_is_zero_and_bright_values_survive() {
        assert_eq!(pack_rgb9e5(Vec3::ZERO), 0);
        assert_ne!(pack_rgb9e5(Vec3::new(1.0, 2.0, 3.0)), 0);
    }

    #[test]
    fn probe_grid_uses_bevy_xyz_axis_order() {
        let resolution = [3, 2, 4];
        assert_eq!(
            probe_position(0, resolution, Vec3::ZERO, Vec3::ONE),
            Vec3::ZERO
        );
        assert_eq!(
            probe_position(23, resolution, Vec3::ZERO, Vec3::ONE),
            Vec3::ONE
        );
    }

    #[test]
    fn one_bounce_diffuse_is_lit_and_respects_occlusion() {
        fn horizontal_triangle(height: f32) -> IrradianceTriangle {
            IrradianceTriangle {
                vertices: [
                    Point3::new(-2.0, height, -2.0),
                    Point3::new(0.0, height, 2.0),
                    Point3::new(2.0, height, -2.0),
                ],
                normals: [Vec3::Y; 3],
                uvs: [Vec2::ZERO; 3],
                colors: [Vec4::ONE; 3],
                material: 0,
                node_index: 0,
            }
        }

        fn sampled_radiance(mut triangles: Vec<IrradianceTriangle>) -> Vec3 {
            let bvh = Bvh::build(&mut triangles);
            let materials = [TransportMaterial {
                metallic_factor: 0.0,
                ..Default::default()
            }];
            let lights = [JobLight {
                reference_form_id: 1,
                translation: [0.0, 1.0, 0.0],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                color_rgba: [1.0; 4],
                radius: 4.0,
                intensity_lumens: 0.0,
                kind: "point".into(),
                bounce_multiplier: DEFAULT_BOUNCE_MULTIPLIER,
                shadow_mode: crate::vsa::dynamic_lighting::DynamicLightShadowMode::RaytracedShadows,
                illumination_mode:
                    crate::vsa::dynamic_lighting::DynamicLightIlluminationMode::SingleBounce,
                transparency_mode:
                    crate::vsa::dynamic_lighting::DynamicLightTransparencyMode::Disabled,
            }];
            trace_radiance(
                &bvh,
                &triangles,
                &materials,
                &lights,
                &DirectionalBakeLight {
                    color_rgba: [0.0; 4],
                    rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                    illuminance: 0.0,
                    bounce_multiplier: DEFAULT_BOUNCE_MULTIPLIER,
                },
                Vec3::new(0.0, 0.25, 0.0),
                Vec3::NEG_Y,
                10.0,
            )
        }

        let visible = sampled_radiance(vec![horizontal_triangle(0.0)]);
        let occluded = sampled_radiance(vec![horizontal_triangle(0.0), horizontal_triangle(0.5)]);
        assert!(visible.max_element() > 0.0, "expected a lit diffuse bounce");
        assert_eq!(occluded, Vec3::ZERO);
    }
}
