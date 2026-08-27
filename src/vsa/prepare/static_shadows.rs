use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use anyhow::{Context, Result, bail};
use bevy::math::{Mat4, Quat, Vec3};
use bvh::aabb::{Aabb, Bounded};
use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};
use bvh::bvh::Bvh;
use bvh::ray::Ray;
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;
use sha2::{Digest, Sha256};

use super::super::manifest::is_pickup_record_kind;
use super::{
    Diagnostic, PreparedLight, PreparedPhysicsClassification, PreparedPlacement,
    PreparedRuntimeMutability, PreparedSemantic, PreparedStaticPointShadowLight,
    PreparedStaticPointShadows, STATIC_POINT_SHADOW_REVISION,
};
use crate::cli::progress::ProgressReporter;
use crate::vsa::bake::{
    find_unified_ktx_tool, ktx_supports_input_file_lists, relative_asset_path, tail,
};

pub(crate) const STATIC_POINT_SHADOW_NEAR_Z: f32 = 0.1;
const RCLIGHTBOX01_BASE_FORM_ID: u32 = 0x0003_54E8;
const FACE_COUNT: usize = 6;
static SHADOW_TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug)]
struct ShadowTriangle {
    vertices: [Point3<f32>; 3],
    node_index: usize,
}

impl Bounded<f32, 3> for ShadowTriangle {
    fn aabb(&self) -> Aabb<f32, 3> {
        let mut min = self.vertices[0];
        let mut max = self.vertices[0];
        for vertex in &self.vertices[1..] {
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);
            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);
        }
        Aabb::with_bounds(min, max)
    }
}

impl BHShape<f32, 3> for ShadowTriangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

pub(crate) struct StaticShadowPrepareOptions<'a> {
    pub(crate) asset_root: &'a Path,
    pub(crate) scene_dir: &'a Path,
    pub(crate) resolution: u32,
    pub(crate) rebuild: bool,
    pub(crate) ktx: Option<PathBuf>,
}

pub(crate) fn prepare_static_point_shadows(
    options: StaticShadowPrepareOptions<'_>,
    placements: &[PreparedPlacement],
    lights: &[PreparedLight],
    diagnostics: &mut Vec<Diagnostic>,
    progress: Option<&ProgressReporter>,
    progress_phase: &str,
) -> Result<Option<PreparedStaticPointShadows>> {
    let casters = sorted_shadow_casters(placements);
    let prepared_lights = sorted_shadow_lights(lights)?;
    if casters.is_empty() || prepared_lights.is_empty() {
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!(
                "static point shadows omitted: {} caster placement(s), {} eligible light(s)",
                casters.len(),
                prepared_lights.len()
            ),
        });
        return Ok(None);
    }

    let fingerprint = shadow_fingerprint(
        options.asset_root,
        &casters,
        &prepared_lights,
        options.resolution,
        STATIC_POINT_SHADOW_NEAR_Z,
    )?;
    let shadows_dir = options.scene_dir.join("shadows");
    let output_path = shadows_dir.join(format!("{fingerprint}.ktx2"));
    let manifest_lights = prepared_lights
        .iter()
        .enumerate()
        .map(|(layer, light)| PreparedStaticPointShadowLight {
            reference_form_id: light.reference_form_id,
            layer: layer as u32,
            translation: light.translation,
            range: light.radius,
        })
        .collect::<Vec<_>>();
    let prepared = PreparedStaticPointShadows {
        revision: STATIC_POINT_SHADOW_REVISION.into(),
        source_fingerprint: fingerprint.clone(),
        asset_path: relative_asset_path(options.asset_root, &output_path)?,
        resolution: options.resolution,
        near_z: STATIC_POINT_SHADOW_NEAR_Z,
        lights: manifest_lights,
    };
    let face_total = (prepared.lights.len() * FACE_COUNT) as u64;
    if let Some(progress) = progress {
        progress.phase_started(progress_phase, Some(face_total));
    }
    if output_path.is_file() && !options.rebuild {
        let message = format!(
            "static point shadows: cache hit, {} light layer(s) at {}x{} -> {}",
            prepared.lights.len(),
            prepared.resolution,
            prepared.resolution,
            output_path.display()
        );
        println!("{message}");
        if let Some(progress) = progress {
            for _ in 0..face_total {
                progress.unit_completed(Some(face_total), Some(true));
            }
        }
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message,
        });
        return Ok(Some(prepared));
    }

    let started = Instant::now();
    fs::create_dir_all(&shadows_dir)?;
    let mut triangles = collect_world_triangles(options.asset_root, &casters)?;
    if triangles.is_empty() {
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: "static point shadows omitted: resolved caster GLBs contained no triangles"
                .into(),
        });
        return Ok(None);
    }
    println!(
        "static point shadows: building BVH for {} triangles and {} lights",
        triangles.len(),
        prepared_lights.len()
    );
    let bvh = Bvh::build_par(&mut triangles);
    let faces = trace_shadow_faces(
        &bvh,
        &triangles,
        &prepared_lights,
        options.resolution,
        STATIC_POINT_SHADOW_NEAR_Z,
        progress,
        face_total,
    );
    let ktx = find_unified_ktx_tool(options.ktx)?;
    write_ktx2(
        &ktx.path,
        &shadows_dir,
        &fingerprint,
        &output_path,
        options.resolution,
        &faces,
    )?;
    let elapsed = started.elapsed();
    let message = format!(
        "static point shadows: generated {} layers / {} faces from {} triangles in {:.2}s -> {}",
        prepared.lights.len(),
        faces.len(),
        triangles.len(),
        elapsed.as_secs_f64(),
        output_path.display()
    );
    println!("{message}");
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message,
    });
    Ok(Some(prepared))
}

fn sorted_shadow_casters(placements: &[PreparedPlacement]) -> Vec<&PreparedPlacement> {
    let mut casters = placements
        .iter()
        .filter(|placement| is_static_shadow_caster(placement))
        .collect::<Vec<_>>();
    casters.sort_by_key(|placement| (placement.reference_form_id, placement.base_form_id));
    casters
}

fn is_static_shadow_caster(placement: &PreparedPlacement) -> bool {
    placement.initially_enabled
        && placement.asset_path.is_some()
        // Movable bodies must not leave a baked silhouette behind when they
        // are pushed. Their current pose is handled by the runtime shadow
        // pass instead.
        && placement.physics_classification == PreparedPhysicsClassification::Static
        && placement.mutability == PreparedRuntimeMutability::Immutable
        && !matches!(
            placement.semantic,
            PreparedSemantic::Door(_)
                | PreparedSemantic::Pickup(_)
                | PreparedSemantic::Npc(_)
                | PreparedSemantic::Creature(_)
        )
        && !is_pickup_record_kind(&placement.base_kind)
        && placement.base_form_id != RCLIGHTBOX01_BASE_FORM_ID
        && crate::vsa::classify_fallout_overlay(
            placement.editor_id.as_deref(),
            placement.asset_path.as_deref(),
        ) == crate::vsa::FalloutOverlayKind::None
}

fn sorted_shadow_lights(lights: &[PreparedLight]) -> Result<Vec<&PreparedLight>> {
    let mut lights = lights
        .iter()
        .filter(|light| {
            light.radius.is_finite()
                && light.radius > STATIC_POINT_SHADOW_NEAR_Z
                && (light.kind.is_empty() || light.kind.eq_ignore_ascii_case("point"))
                && light.flags & 0x200 == 0
        })
        .collect::<Vec<_>>();
    lights.sort_by_key(|light| (light.reference_form_id, light.base_form_id));
    let mut form_ids = HashSet::new();
    for light in &lights {
        if light.reference_form_id == 0 || !form_ids.insert(light.reference_form_id) {
            bail!(
                "static point shadows require unique non-zero light reference FormIDs; found {:08x}",
                light.reference_form_id
            );
        }
    }
    Ok(lights)
}

fn shadow_fingerprint(
    asset_root: &Path,
    casters: &[&PreparedPlacement],
    lights: &[&PreparedLight],
    resolution: u32,
    near_z: f32,
) -> Result<String> {
    shadow_fingerprint_with_revision(
        STATIC_POINT_SHADOW_REVISION,
        asset_root,
        casters,
        lights,
        resolution,
        near_z,
    )
}

fn shadow_fingerprint_with_revision(
    revision: &str,
    asset_root: &Path,
    casters: &[&PreparedPlacement],
    lights: &[&PreparedLight],
    resolution: u32,
    near_z: f32,
) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(revision.as_bytes());
    hasher.update(resolution.to_le_bytes());
    hasher.update(near_z.to_le_bytes());
    let mut asset_hashes = HashMap::<&str, [u8; 32]>::new();
    for placement in casters {
        let asset_path = placement.asset_path.as_deref().expect("caster has asset");
        let digest = if let Some(digest) = asset_hashes.get(asset_path) {
            *digest
        } else {
            let path = resolve_asset_path(asset_root, asset_path);
            let bytes = fs::read(&path)
                .with_context(|| format!("could not read shadow caster GLB {}", path.display()))?;
            let digest: [u8; 32] = Sha256::digest(bytes).into();
            asset_hashes.insert(asset_path, digest);
            digest
        };
        hasher.update(placement.reference_form_id.to_le_bytes());
        hasher.update(placement.base_form_id.to_le_bytes());
        hasher.update(asset_path.as_bytes());
        hasher.update(digest);
        for value in placement
            .translation
            .iter()
            .chain(placement.rotation_xyzw.iter())
            .chain(std::iter::once(&placement.scale))
        {
            hasher.update(value.to_le_bytes());
        }
    }
    for light in lights {
        hasher.update(light.reference_form_id.to_le_bytes());
        hasher.update(light.base_form_id.to_le_bytes());
        for value in light
            .translation
            .iter()
            .chain(std::iter::once(&light.radius))
        {
            hasher.update(value.to_le_bytes());
        }
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn collect_world_triangles(
    asset_root: &Path,
    casters: &[&PreparedPlacement],
) -> Result<Vec<ShadowTriangle>> {
    let mut assets = HashMap::<String, Vec<[Vec3; 3]>>::new();
    let mut triangles = Vec::new();
    for placement in casters {
        let asset_path = placement.asset_path.as_deref().expect("caster has asset");
        if !assets.contains_key(asset_path) {
            let path = resolve_asset_path(asset_root, asset_path);
            assets.insert(asset_path.into(), load_glb_triangles(&path)?);
        }
        let placement_transform = Mat4::from_scale_rotation_translation(
            Vec3::splat(placement.scale),
            Quat::from_array(placement.rotation_xyzw),
            Vec3::from_array(placement.translation),
        );
        for local in assets.get(asset_path).expect("asset was loaded") {
            let world = local.map(|vertex| placement_transform.transform_point3(vertex));
            if triangle_is_valid(world) {
                triangles.push(ShadowTriangle {
                    vertices: world.map(point3),
                    node_index: 0,
                });
            }
        }
    }
    Ok(triangles)
}

fn resolve_asset_path(asset_root: &Path, relative: &str) -> PathBuf {
    asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR))
}

fn load_glb_triangles(path: &Path) -> Result<Vec<[Vec3; 3]>> {
    let gltf = gltf::Gltf::open(path)
        .with_context(|| format!("could not parse shadow caster GLB {}", path.display()))?;
    let mut buffers = Vec::new();
    for buffer in gltf.document.buffers() {
        let bytes = match buffer.source() {
            gltf::buffer::Source::Bin => gltf
                .blob
                .as_ref()
                .context("GLB references a BIN buffer but has no binary blob")?
                .clone(),
            gltf::buffer::Source::Uri(uri) if !uri.starts_with("data:") => {
                fs::read(path.parent().unwrap_or_else(|| Path::new(".")).join(uri))?
            }
            gltf::buffer::Source::Uri(_) => {
                bail!("data-URI GLB buffers are not supported for static shadow preparation")
            }
        };
        buffers.push(bytes);
    }
    let mut triangles = Vec::new();
    let scenes = gltf
        .document
        .default_scene()
        .into_iter()
        .chain(gltf.document.scenes())
        .take(1)
        .collect::<Vec<_>>();
    for scene in scenes {
        for node in scene.nodes() {
            collect_node_triangles(node, Mat4::IDENTITY, &buffers, &mut triangles)?;
        }
    }
    Ok(triangles)
}

fn collect_node_triangles(
    node: gltf::Node<'_>,
    parent_transform: Mat4,
    buffers: &[Vec<u8>],
    triangles: &mut Vec<[Vec3; 3]>,
) -> Result<()> {
    let transform = parent_transform * Mat4::from_cols_array_2d(&node.transform().matrix());
    if let Some(mesh) = node.mesh() {
        for primitive in mesh.primitives() {
            if primitive.mode() != gltf::mesh::Mode::Triangles {
                continue;
            }
            let reader = primitive.reader(|buffer| buffers.get(buffer.index()).map(Vec::as_slice));
            let positions = reader
                .read_positions()
                .context("GLB triangle primitive has no POSITION attribute")?
                .map(|position| transform.transform_point3(Vec3::from_array(position)))
                .collect::<Vec<_>>();
            let indices = reader.read_indices().map_or_else(
                || (0..positions.len() as u32).collect::<Vec<_>>(),
                |indices| indices.into_u32().collect::<Vec<_>>(),
            );
            for indices in indices.as_chunks::<3>().0 {
                let [a, b, c] = *indices;
                let triangle = [
                    *positions
                        .get(a as usize)
                        .context("GLB index exceeds POSITION data")?,
                    *positions
                        .get(b as usize)
                        .context("GLB index exceeds POSITION data")?,
                    *positions
                        .get(c as usize)
                        .context("GLB index exceeds POSITION data")?,
                ];
                if triangle_is_valid(triangle) {
                    triangles.push(triangle);
                }
            }
        }
    }
    for child in node.children() {
        collect_node_triangles(child, transform, buffers, triangles)?;
    }
    Ok(())
}

fn triangle_is_valid(vertices: [Vec3; 3]) -> bool {
    vertices.iter().all(|vertex| vertex.is_finite())
        && (vertices[1] - vertices[0])
            .cross(vertices[2] - vertices[0])
            .length_squared()
            > 1e-12
}

fn trace_shadow_faces(
    bvh: &Bvh<f32, 3>,
    triangles: &[ShadowTriangle],
    lights: &[&PreparedLight],
    resolution: u32,
    near_z: f32,
    progress: Option<&ProgressReporter>,
    total: u64,
) -> Vec<Vec<f32>> {
    (0..(total as usize))
        .into_par_iter()
        .map(|index| {
            let light = lights[index / FACE_COUNT];
            let face = index % FACE_COUNT;
            let origin = point3(Vec3::from_array(light.translation));
            let mut depths = Vec::with_capacity((resolution * resolution) as usize);
            for y in 0..resolution {
                for x in 0..resolution {
                    let direction = cubemap_texel_direction(face, x, y, resolution);
                    let ray = Ray::new(origin, vector3(direction));
                    let hit =
                        nearest_hit_distance(bvh, triangles, &ray, origin, direction, light.radius);
                    depths.push(if let Some(hit) = hit {
                        reverse_z_depth(direction, hit, near_z)
                    } else {
                        0.0
                    });
                }
            }
            if let Some(progress) = progress {
                progress.unit_completed(Some(total), None);
            }
            depths
        })
        .collect()
}

fn nearest_hit_distance(
    bvh: &Bvh<f32, 3>,
    triangles: &[ShadowTriangle],
    ray: &Ray<f32, 3>,
    origin: Point3<f32>,
    direction: Vec3,
    max_distance: f32,
) -> Option<f32> {
    let hit = bvh
        .traverse_iterator(ray, triangles)
        .filter_map(|triangle| ray_triangle_distance(origin, direction, triangle))
        .filter(|distance| *distance <= max_distance)
        .fold(f32::INFINITY, f32::min);
    hit.is_finite().then_some(hit)
}

fn cubemap_texel_direction(face: usize, x: u32, y: u32, resolution: u32) -> Vec3 {
    let s = 2.0 * (x as f32 + 0.5) / resolution as f32 - 1.0;
    let t = 2.0 * (y as f32 + 0.5) / resolution as f32 - 1.0;
    let ktx = match face {
        0 => Vec3::new(1.0, -t, -s),
        1 => Vec3::new(-1.0, -t, s),
        2 => Vec3::new(s, 1.0, t),
        3 => Vec3::new(s, -1.0, -t),
        4 => Vec3::new(s, -t, 1.0),
        5 => Vec3::new(-s, -t, -1.0),
        _ => unreachable!("cubemap face must be in 0..6"),
    };
    Vec3::new(ktx.x, ktx.y, -ktx.z).normalize()
}

fn ray_triangle_distance(
    origin: Point3<f32>,
    direction: Vec3,
    triangle: &ShadowTriangle,
) -> Option<f32> {
    let a = vec3(triangle.vertices[0]);
    let b = vec3(triangle.vertices[1]);
    let c = vec3(triangle.vertices[2]);
    let edge1 = b - a;
    let edge2 = c - a;
    let p = direction.cross(edge2);
    let determinant = edge1.dot(p);
    if determinant.abs() < 1e-7 {
        return None;
    }
    let inverse = determinant.recip();
    let offset = vec3(origin) - a;
    let u = offset.dot(p) * inverse;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = offset.cross(edge1);
    let v = direction.dot(q) * inverse;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = edge2.dot(q) * inverse;
    (distance > 1e-5).then_some(distance)
}

fn reverse_z_depth(direction: Vec3, distance: f32, near_z: f32) -> f32 {
    let major_axis = distance * direction.abs().max_element();
    (near_z / major_axis.max(near_z)).clamp(0.0, 1.0)
}

fn write_ktx2(
    ktx: &Path,
    shadows_dir: &Path,
    fingerprint: &str,
    output_path: &Path,
    resolution: u32,
    faces: &[Vec<f32>],
) -> Result<()> {
    let temporary_dir = shadows_dir.join(format!(".tmp-{fingerprint}-{}", std::process::id()));
    if temporary_dir.exists() {
        fs::remove_dir_all(&temporary_dir)?;
    }
    fs::create_dir_all(&temporary_dir)?;
    let temporary_output = temporary_dir.join("point-shadows.ktx2");
    let raw_dir = std::env::temp_dir().join(format!(
        "bo-s-{}-{}",
        std::process::id(),
        SHADOW_TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&raw_dir)?;
    let mut raw_paths = Vec::with_capacity(faces.len());
    for (index, face) in faces.iter().enumerate() {
        let path = raw_dir.join(format!("face-{index:04}.raw"));
        let mut bytes = Vec::with_capacity(face.len() * size_of::<f32>());
        for value in face {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        fs::write(&path, bytes)?;
        raw_paths.push(path);
    }
    let arguments = if ktx_supports_input_file_lists(ktx) {
        let input_list = temporary_dir.join("raw-files.txt");
        let mut listing = String::new();
        for path in &raw_paths {
            writeln!(
                listing,
                "{}",
                path.to_str().context("KTX input path is not UTF-8")?
            )?;
        }
        fs::write(&input_list, listing)?;
        ktx_create_arguments_with_input(
            resolution,
            faces.len() / FACE_COUNT,
            OsString::from(format!("@{}", input_list.display())),
            &temporary_output,
        )
    } else {
        ktx_create_arguments(
            resolution,
            faces.len() / FACE_COUNT,
            &raw_paths,
            &temporary_output,
        )
    };
    let mut command = Command::new(ktx);
    command.args(arguments);
    let output = command.output().context("failed to start KTX-Software")?;
    if !output.status.success() {
        bail!(
            "KTX-Software failed with {}:\n{}\n{}\nraw shadow faces were kept at {}",
            output.status,
            tail(&output.stdout),
            tail(&output.stderr),
            raw_dir.display()
        );
    }
    let validation = Command::new(ktx)
        .arg("validate")
        .arg(&temporary_output)
        .output()
        .context("failed to validate prepared shadow KTX2")?;
    if !validation.status.success() {
        bail!(
            "KTX validation failed with {}:\n{}\n{}\nartifact was kept at {}",
            validation.status,
            tail(&validation.stdout),
            tail(&validation.stderr),
            temporary_output.display()
        );
    }
    atomic_replace(&temporary_output, output_path)?;
    fs::remove_dir_all(&temporary_dir)?;
    fs::remove_dir_all(&raw_dir)?;
    Ok(())
}

fn ktx_create_arguments(
    resolution: u32,
    layers: usize,
    raw_paths: &[PathBuf],
    output_path: &Path,
) -> Vec<OsString> {
    let mut arguments = ktx_create_arguments_prefix(resolution, layers);
    arguments.extend(raw_paths.iter().map(|path| path.as_os_str().to_owned()));
    arguments.push(output_path.as_os_str().to_owned());
    arguments
}

fn ktx_create_arguments_with_input(
    resolution: u32,
    layers: usize,
    input: OsString,
    output_path: &Path,
) -> Vec<OsString> {
    let mut arguments = ktx_create_arguments_prefix(resolution, layers);
    arguments.push(input);
    arguments.push(output_path.as_os_str().to_owned());
    arguments
}

fn ktx_create_arguments_prefix(resolution: u32, layers: usize) -> Vec<OsString> {
    [
        "create".into(),
        "--raw".into(),
        "--format".into(),
        "D32_SFLOAT".into(),
        "--width".into(),
        resolution.to_string().into(),
        "--height".into(),
        resolution.to_string().into(),
        "--layers".into(),
        layers.to_string().into(),
        "--cubemap".into(),
        "--assign-tf".into(),
        "linear".into(),
        "--assign-texcoord-origin".into(),
        "top-left".into(),
        "--zstd".into(),
        "3".into(),
    ]
    .into_iter()
    .collect()
}

#[cfg(windows)]
fn atomic_replace(source: &Path, destination: &Path) -> Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    let source_display = source.display().to_string();
    let destination_display = destination.display().to_string();
    let source = source
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let destination = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let result = unsafe {
        MoveFileExW(
            source.as_ptr(),
            destination.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if result == 0 {
        return Err(std::io::Error::last_os_error()).with_context(|| {
            format!(
                "could not atomically replace {} with {}",
                destination_display, source_display
            )
        });
    }
    Ok(())
}

#[cfg(not(windows))]
fn atomic_replace(source: &Path, destination: &Path) -> Result<()> {
    fs::rename(source, destination).with_context(|| {
        format!(
            "could not atomically replace {} with {}",
            destination.display(),
            source.display()
        )
    })
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
#[path = "tests/static_shadows.rs"]
mod tests;
