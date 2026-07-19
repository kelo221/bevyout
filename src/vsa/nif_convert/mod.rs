//! Experimental native FO3/FNV NIF-to-GLB vertical slice.
//!
//! This owns its CLI input resolution, focused NIF conversion, texture
//! resolution, reports, and output writes. The established Blender conversion
//! route used by `prepare` intentionally remains unchanged.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use flate2::{Compression, GzBuilder};
use serde::Serialize;

use crate::cli::{NifConversionMode, NifConvertArgs};

use super::assets::{
    RootTransformPolicy, flip_directx_normal_y_texel, load_archives, resolve_asset,
};
use super::physics::{
    PHYSICS_ASSET_SCHEMA_VERSION, PreparedPhysicsAsset, PreparedPhysicsBody, PreparedPhysicsJoint,
    PreparedPhysicsJointSource, PreparedPhysicsShape, PreparedPhysicsSource,
    validate_physics_asset,
};

#[derive(Debug, Serialize)]
struct ConversionReport {
    converter: &'static str,
    source: String,
    output: String,
    conversion: &'static str,
    nif_version: &'static str,
    bethesda_version: u32,
    blocks: usize,
    meshes: usize,
    vertices: usize,
    triangles: usize,
    animations: usize,
    animation_channels: usize,
    animation_keyframes: usize,
    embedded_textures: usize,
    missing_textures: Vec<String>,
    lossy_issues: Vec<ReportIssue>,
    physics: &'static str,
    physics_output: Option<String>,
    physics_bodies: usize,
    physics_shapes: usize,
    physics_joints: usize,
}

#[derive(Debug, Serialize)]
struct ReportIssue {
    block: usize,
    block_type: String,
    message: String,
}

pub(crate) const NATIVE_NIF_REPORT_REVISION: &str = "nifty-fo3-native-v4-normal-y-v1-material-parity-skin-anim-xyzw-v1-audio-cues-v1-havok-joints-v1-com-frame-v1";

pub(crate) struct NifConversionRequest<'a> {
    pub(crate) source_name: &'a str,
    pub(crate) nif_bytes: &'a [u8],
    pub(crate) output: &'a Path,
    pub(crate) physics_output: Option<&'a Path>,
    pub(crate) report: Option<&'a Path>,
    pub(crate) conversion: NifConversionMode,
    pub(crate) root_transform_policy: RootTransformPolicy,
    pub(crate) allow_lossy: bool,
    pub(crate) force: bool,
    pub(crate) data_root: Option<&'a Path>,
    pub(crate) archives: &'a [super::bsa::BsaArchive],
}

#[derive(Debug)]
pub(crate) struct NifConversionResult {
    pub(crate) lines: Vec<String>,
    pub(crate) missing_textures: Vec<String>,
    pub(crate) lossy_scene_issues: usize,
}

pub(crate) struct ActorSceneConversionRequest<'a> {
    pub(crate) source_name: &'a str,
    pub(crate) scene: nif::fo3::Scene,
    pub(crate) skeleton_document: &'a nif::fo3::Document,
    pub(crate) output: &'a Path,
    pub(crate) physics_output: &'a Path,
    pub(crate) allow_lossy: bool,
    pub(crate) data_root: Option<&'a Path>,
    pub(crate) archives: &'a [super::bsa::BsaArchive],
}

pub fn nif_convert(args: NifConvertArgs) -> Result<()> {
    let data_root = resolve_data_root(args.game_root.as_deref())?;
    let mut archive_diagnostics = Vec::new();
    let archives = if let Some(data_root) = &data_root {
        load_archives(data_root, &mut archive_diagnostics)?
    } else {
        Vec::new()
    };
    for diagnostic in archive_diagnostics {
        println!(
            "nif-convert: {}: {}",
            diagnostic.severity, diagnostic.message
        );
    }

    let (source_name, nif_bytes) = if let Some(path) = &args.input {
        (
            path.file_name()
                .unwrap_or(path.as_os_str())
                .to_string_lossy()
                .into_owned(),
            fs::read(path).with_context(|| format!("reading NIF {}", path.display()))?,
        )
    } else {
        let asset = normalize_asset_path(args.asset.as_deref().expect("clap source group"));
        let data_root = data_root
            .as_deref()
            .context("--asset requires --game-root or [fallout3].game_root")?;
        let bytes = resolve_asset(data_root, &archives, &asset)?.with_context(|| {
            format!("NIF asset {asset} was not found in loose Data files or indexed BSAs")
        })?;
        (asset, bytes)
    };

    let result = convert_nif(NifConversionRequest {
        source_name: &source_name,
        nif_bytes: &nif_bytes,
        output: &args.output,
        physics_output: args.physics_output.as_deref(),
        report: args.report.as_deref(),
        conversion: args.conversion,
        root_transform_policy: RootTransformPolicy::PreserveReviewRequired,
        allow_lossy: args.allow_lossy,
        force: args.force,
        data_root: data_root.as_deref(),
        archives: &archives,
    })?;
    for line in result.lines {
        println!("{line}");
    }
    Ok(())
}

pub(crate) fn convert_nif(request: NifConversionRequest<'_>) -> Result<NifConversionResult> {
    ensure_output_available(request.output, request.force)?;
    if let Some(path) = request.report {
        ensure_output_available(path, request.force)?;
    }
    if let Some(path) = request.physics_output {
        ensure_output_available(path, request.force)?;
    }

    let mut lines = vec![format!("nif-convert: input {}", request.source_name)];
    let document = nif::fo3::parse(request.nif_bytes).context("parsing FO3/FNV NIF 20.2.0.7")?;
    lines.push(format!(
        "nif-convert: parsed version=20.2.0.7 bethesda={} blocks={}",
        document.header.bethesda.version,
        document.blocks.len()
    ));
    let mut scene = nif::fo3::extract_scene(&document).context("extracting NIF scene")?;
    let record_zero_non_identity = scene
        .nodes
        .iter()
        .find(|node| node.source_block == 0)
        .is_some_and(|node| !nif_transform_is_identity(node.transform));
    let corrected_root_transform = request.root_transform_policy
        == RootTransformPolicy::DiscardVerified
        && record_zero_non_identity;
    if request.root_transform_policy == RootTransformPolicy::DiscardVerified
        && let Some(root) = scene.nodes.iter_mut().find(|node| node.source_block == 0)
    {
        root.transform.translation = [0.0; 3];
        root.transform.rotation = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        root.transform.scale = 1.0;
    }
    apply_conversion_mode(&mut scene, request.conversion);
    lines.push(format!(
        "nif-convert: scene meshes={} vertices={} triangles={}",
        scene.statistics.source_meshes,
        scene.statistics.source_vertices,
        scene.statistics.source_triangles
    ));
    lines.push(format!(
        "nif-convert: animations={} channels={}",
        scene.animations.len(),
        scene
            .animations
            .iter()
            .map(|animation| animation.channels.len())
            .sum::<usize>()
    ));
    if !request.allow_lossy && !scene.issues.is_empty() {
        bail!(
            "strict conversion rejected {} lossy scene issue(s): {} (pass --allow-lossy to emit the supported subset)",
            scene.issues.len(),
            scene
                .issues
                .iter()
                .take(5)
                .map(|issue| format!(
                    "block {} {}: {}",
                    issue.source_block, issue.type_name, issue.message
                ))
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    if scene.statistics.source_meshes == 0 {
        bail!("NIF scene contains no renderable props-first meshes");
    }

    let mut textures = resolve_textures(&scene, request.data_root, request.archives)?;
    prepare_native_normal_textures(&mut scene.materials, &mut textures)?;
    let mut output = nif::fo3::encode_glb(
        &scene,
        &textures,
        &nif::fo3::GlbOptions {
            source_name: request.source_name.to_owned(),
            allow_missing_textures: request.allow_lossy,
        },
    )
    .context("encoding self-contained GLB")?;
    output.bytes = patch_native_glb_metadata(
        &output.bytes,
        request.root_transform_policy,
        record_zero_non_identity,
        usize::from(corrected_root_transform),
    )?;
    lines.push(format!(
        "nif-convert: textures embedded={} missing={}",
        textures.len().saturating_sub(output.missing_textures.len()),
        output.missing_textures.len()
    ));

    let (physics, physics_bytes, physics_bodies, physics_shapes, physics_joints) = if request
        .physics_output
        .is_some()
    {
        let physics_scene =
            nif::fo3::extract_physics(&document).context("extracting authored Havok collision")?;
        if !request.allow_lossy && !physics_scene.issues.is_empty() {
            bail!(
                "strict conversion rejected {} lossy physics issue(s): {} (pass --allow-lossy to emit the supported subset)",
                physics_scene.issues.len(),
                physics_scene
                    .issues
                    .iter()
                    .take(5)
                    .map(|issue| format!(
                        "block {} {}: {}",
                        issue.source_block, issue.type_name, issue.message
                    ))
                    .collect::<Vec<_>>()
                    .join("; ")
            );
        }
        let physics_bodies = physics_scene.bodies.len();
        let physics_shapes = physics_scene
            .bodies
            .iter()
            .map(|body| body.shapes.len())
            .sum();
        let physics_joints = physics_scene.joints.len();
        let bytes = encode_physics_sidecar(physics_scene)?;
        (
            if physics_bodies == 0 {
                "no-authored-collision"
            } else {
                "authored-havok"
            },
            Some(bytes),
            physics_bodies,
            physics_shapes,
            physics_joints,
        )
    } else {
        ("not-requested", None, 0, 0, 0)
    };

    let missing_textures = output.missing_textures.clone();
    let lossy_scene_issues = scene.issues.len();
    let report_bytes = if request.report.is_some() {
        let report = ConversionReport {
            converter: NATIVE_NIF_REPORT_REVISION,
            source: request.source_name.to_owned(),
            output: request.output.display().to_string(),
            conversion: conversion_name(request.conversion),
            nif_version: "20.2.0.7",
            bethesda_version: document.header.bethesda.version,
            blocks: document.blocks.len(),
            meshes: scene.statistics.source_meshes,
            vertices: scene.statistics.source_vertices,
            triangles: scene.statistics.source_triangles,
            animations: scene.animations.len(),
            animation_channels: scene
                .animations
                .iter()
                .map(|animation| animation.channels.len())
                .sum(),
            animation_keyframes: scene
                .animations
                .iter()
                .flat_map(|animation| animation.channels.iter())
                .map(|channel| {
                    channel.translations.len() + channel.rotations.len() + channel.scales.len()
                })
                .sum(),
            embedded_textures: textures.len().saturating_sub(output.missing_textures.len()),
            missing_textures: output.missing_textures,
            lossy_issues: scene
                .issues
                .into_iter()
                .map(|issue| ReportIssue {
                    block: issue.source_block,
                    block_type: issue.type_name,
                    message: issue.message,
                })
                .collect(),
            physics,
            physics_output: request
                .physics_output
                .map(|path| path.display().to_string()),
            physics_bodies,
            physics_shapes,
            physics_joints,
        };
        let mut bytes = serde_json::to_vec_pretty(&report)?;
        bytes.push(b'\n');
        Some(bytes)
    } else {
        None
    };

    atomic_write(request.output, &output.bytes, request.force)?;
    lines.push(format!(
        "nif-convert: wrote {} bytes -> {}",
        output.bytes.len(),
        request.output.display()
    ));
    if let (Some(path), Some(bytes)) = (request.physics_output, &physics_bytes) {
        atomic_write(path, bytes, request.force)?;
        lines.push(format!(
            "nif-convert: physics bodies={} shapes={} joints={} -> {}",
            physics_bodies,
            physics_shapes,
            physics_joints,
            path.display()
        ));
    }
    if let (Some(path), Some(bytes)) = (request.report, &report_bytes) {
        atomic_write(path, bytes, request.force)?;
        lines.push(format!("nif-convert: report -> {}", path.display()));
    }
    Ok(NifConversionResult {
        lines,
        missing_textures,
        lossy_scene_issues,
    })
}

pub(crate) fn convert_actor_scene(
    request: ActorSceneConversionRequest<'_>,
) -> Result<NifConversionResult> {
    ensure_output_available(request.output, true)?;
    ensure_output_available(request.physics_output, true)?;
    let mut scene = request.scene;
    scene.animations.clear();
    if !request.allow_lossy && !scene.issues.is_empty() {
        bail!(
            "strict actor conversion rejected {} lossy scene issue(s): {}",
            scene.issues.len(),
            scene
                .issues
                .iter()
                .take(5)
                .map(|issue| format!(
                    "block {} {}: {}",
                    issue.source_block, issue.type_name, issue.message
                ))
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    if !scene.has_visible_geometry() {
        bail!("actor assembly contains no visible geometry");
    }
    let mut textures = resolve_textures(&scene, request.data_root, request.archives)?;
    prepare_native_normal_textures(&mut scene.materials, &mut textures)?;
    let mut output = nif::fo3::encode_glb(
        &scene,
        &textures,
        &nif::fo3::GlbOptions {
            source_name: request.source_name.to_owned(),
            allow_missing_textures: request.allow_lossy,
        },
    )
    .context("encoding native actor GLB")?;
    output.bytes = patch_native_glb_metadata(
        &output.bytes,
        RootTransformPolicy::PreserveVerified,
        false,
        0,
    )?;

    let physics_scene = nif::fo3::extract_physics(request.skeleton_document)
        .context("extracting actor skeleton Havok collision")?;
    if !request.allow_lossy && !physics_scene.issues.is_empty() {
        bail!(
            "strict actor conversion rejected {} lossy physics issue(s)",
            physics_scene.issues.len()
        );
    }
    let physics_bodies = physics_scene.bodies.len();
    let physics_shapes = physics_scene
        .bodies
        .iter()
        .map(|body| body.shapes.len())
        .sum::<usize>();
    let physics_joints = physics_scene.joints.len();
    let physics_bytes = encode_physics_sidecar(physics_scene)?;
    atomic_write(request.output, &output.bytes, true)?;
    atomic_write(request.physics_output, &physics_bytes, true)?;

    Ok(NifConversionResult {
        lines: vec![format!(
            "nif-convert: actor meshes={} vertices={} triangles={} physics_bodies={} physics_shapes={} physics_joints={}",
            scene.statistics.source_meshes,
            scene.statistics.source_vertices,
            scene.statistics.source_triangles,
            physics_bodies,
            physics_shapes,
            physics_joints
        )],
        missing_textures: output.missing_textures,
        lossy_scene_issues: scene.issues.len(),
    })
}

fn encode_physics_sidecar(scene: nif::fo3::PhysicsScene) -> Result<Vec<u8>> {
    let nif::fo3::PhysicsScene { bodies, joints, .. } = scene;
    let asset = PreparedPhysicsAsset {
        schema_version: PHYSICS_ASSET_SCHEMA_VERSION,
        source: PreparedPhysicsSource::AuthoredHavok,
        bodies: bodies
            .into_iter()
            .map(|body| PreparedPhysicsBody {
                group_id: body.group_id,
                node: body.node,
                motion_type: body.motion_type,
                quality_type: body.quality_type,
                mass: body.mass,
                center_of_mass: body.center_of_mass,
                inertia: body.inertia,
                linear_velocity: body.linear_velocity,
                angular_velocity: body.angular_velocity,
                gravity_factor: body.gravity_factor,
                linear_damping: body.linear_damping,
                angular_damping: body.angular_damping,
                friction: body.friction,
                restitution: body.restitution,
                max_linear_velocity: body.max_linear_velocity,
                max_angular_velocity: body.max_angular_velocity,
                sleep_enabled: body.sleep_enabled,
                ccd_enabled: body.ccd_enabled,
                layer: body.layer,
                filter_flags: body.filter_flags,
                material: body.material,
                material_name: body.material_name,
                phantom: body.phantom,
                constrained: body.constrained,
                shapes: body.shapes.into_iter().map(convert_physics_shape).collect(),
            })
            .collect(),
        joints: joints.into_iter().map(convert_physics_joint).collect(),
    };
    validate_physics_asset(&asset)?;
    let json = serde_json::to_vec(&asset)?;
    let mut encoder = GzBuilder::new()
        .mtime(0)
        .write(Vec::new(), Compression::default());
    encoder.write_all(&json)?;
    encoder.finish().context("finishing physics sidecar gzip")
}

fn convert_physics_joint(joint: nif::fo3::PhysicsJoint) -> PreparedPhysicsJoint {
    PreparedPhysicsJoint {
        kind: joint.kind,
        body_a: joint.body_a,
        body_b: joint.body_b,
        anchor_a: joint.anchor_a,
        anchor_b: joint.anchor_b,
        frame_a_rotation_xyzw: joint.frame_a_rotation_xyzw,
        frame_b_rotation_xyzw: joint.frame_b_rotation_xyzw,
        lower_limit: joint.lower_limit,
        upper_limit: joint.upper_limit,
        cone_limit: joint.cone_limit,
        plane_lower_limit: joint.plane_lower_limit,
        plane_upper_limit: joint.plane_upper_limit,
        twist_lower_limit: joint.twist_lower_limit,
        twist_upper_limit: joint.twist_upper_limit,
        malleable_strength: joint.malleable_strength,
        source: PreparedPhysicsJointSource::Authored,
    }
}

fn convert_physics_shape(shape: nif::fo3::ConvertedPhysicsShape) -> PreparedPhysicsShape {
    match shape {
        nif::fo3::ConvertedPhysicsShape::Box {
            center,
            half_extents,
            rotation_xyzw,
        } => PreparedPhysicsShape::Box {
            center,
            half_extents,
            rotation_xyzw,
        },
        nif::fo3::ConvertedPhysicsShape::Sphere { center, radius } => {
            PreparedPhysicsShape::Sphere { center, radius }
        }
        nif::fo3::ConvertedPhysicsShape::Capsule {
            point1,
            point2,
            radius,
        } => PreparedPhysicsShape::Capsule {
            point1,
            point2,
            radius,
        },
        nif::fo3::ConvertedPhysicsShape::ConvexHull { points } => {
            PreparedPhysicsShape::ConvexHull { points }
        }
        nif::fo3::ConvertedPhysicsShape::TriangleMesh { vertices, indices } => {
            PreparedPhysicsShape::TriangleMesh { vertices, indices }
        }
    }
}

fn resolve_data_root(game_root: Option<&Path>) -> Result<Option<PathBuf>> {
    let Some(game_root) = game_root else {
        return Ok(None);
    };
    let game_root = fs::canonicalize(game_root)
        .with_context(|| format!("game root does not exist: {}", game_root.display()))?;
    let data = if game_root
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("Data"))
    {
        game_root
    } else {
        game_root.join("Data")
    };
    if !data.is_dir() {
        bail!("Fallout Data directory does not exist: {}", data.display());
    }
    Ok(Some(data))
}

fn normalize_asset_path(asset: &str) -> String {
    let normalized = asset
        .replace('\\', "/")
        .trim_start_matches('/')
        .to_ascii_lowercase();
    if normalized.starts_with("meshes/") {
        normalized
    } else {
        format!("meshes/{normalized}")
    }
}

fn resolve_textures(
    scene: &nif::fo3::Scene,
    data_root: Option<&Path>,
    archives: &[super::bsa::BsaArchive],
) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut referenced = scene
        .materials
        .iter()
        .flat_map(|material| {
            [
                material.diffuse_texture.as_ref(),
                material.normal_texture.as_ref(),
                material.glow_texture.as_ref(),
            ]
            .into_iter()
            .flatten()
        })
        .cloned()
        .collect::<Vec<_>>();
    referenced.sort();
    referenced.dedup();
    let mut textures = BTreeMap::new();
    let Some(data_root) = data_root else {
        return Ok(textures);
    };
    for path in referenced {
        if let Some(bytes) = resolve_asset(data_root, archives, &path)? {
            textures.insert(path, bytes);
        }
    }
    Ok(textures)
}

/// Builds a distinct glTF image for every normal-map source. A same-source
/// specular slot follows the converted image because its alpha is unchanged;
/// this preserves the viewer's shared normal/specular roughness proxy. Diffuse
/// and glow slots retain the original image. The material slot, not the
/// filename, is the authority in the native converter.
fn prepare_native_normal_textures(
    materials: &mut [nif::fo3::SceneMaterial],
    textures: &mut BTreeMap<String, Vec<u8>>,
) -> Result<()> {
    let normal_paths = materials
        .iter()
        .filter_map(|material| material.normal_texture.clone())
        .collect::<BTreeSet<_>>();

    for source_path in normal_paths {
        let Some(source_bytes) = textures.get(&source_path) else {
            continue;
        };
        let mut rgba = image::load_from_memory(source_bytes)
            .with_context(|| format!("decoding DirectX normal texture {source_path}"))?
            .to_rgba8();
        for pixel in rgba.pixels_mut() {
            flip_directx_normal_y_texel(&mut pixel.0);
        }

        let derived_path = format!("{source_path}#bevyout-normal-y-v1");
        let mut encoded = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(rgba)
            .write_to(&mut encoded, image::ImageFormat::Png)
            .with_context(|| format!("encoding Bevy normal texture {source_path}"))?;
        textures.insert(derived_path.clone(), encoded.into_inner());

        for material in materials
            .iter_mut()
            .filter(|material| material.normal_texture.as_deref() == Some(source_path.as_str()))
        {
            material.normal_texture = Some(derived_path.clone());
            if material.specular_texture.as_deref() == Some(source_path.as_str()) {
                material.specular_texture = Some(derived_path.clone());
            }
        }
    }
    Ok(())
}

fn apply_conversion_mode(scene: &mut nif::fo3::Scene, mode: NifConversionMode) {
    if mode != NifConversionMode::QuickAo {
        return;
    }
    for mesh in scene.nodes.iter_mut().filter_map(|node| node.mesh.as_mut()) {
        for color in &mut mesh.colors {
            let ao = (color[0] * 0.2126 + color[1] * 0.7152 + color[2] * 0.0722).clamp(0.0, 1.0);
            *color = [ao, ao, ao, color[3]];
        }
    }
}

fn nif_transform_is_identity(transform: nif::fo3::Transform) -> bool {
    const IDENTITY_ROTATION: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    transform.translation == [0.0; 3]
        && transform.rotation == IDENTITY_ROTATION
        && transform.scale == 1.0
}

fn patch_native_glb_metadata(
    bytes: &[u8],
    root_transform_policy: RootTransformPolicy,
    record_zero_non_identity: bool,
    spatial_corrections: usize,
) -> Result<Vec<u8>> {
    if bytes.len() < 20 || &bytes[0..4] != b"glTF" {
        bail!("native converter produced an invalid GLB header");
    }
    let json_length = usize::try_from(u32::from_le_bytes(bytes[12..16].try_into().unwrap()))
        .context("GLB JSON length exceeds the addressable range")?;
    if &bytes[16..20] != b"JSON" || 20 + json_length > bytes.len() {
        bail!("native converter produced an invalid GLB JSON chunk");
    }
    let mut document: serde_json::Value = serde_json::from_slice(&bytes[20..20 + json_length])
        .context("decoding native GLB JSON metadata")?;
    let nodes = document
        .get_mut("nodes")
        .and_then(serde_json::Value::as_array_mut)
        .context("native GLB has no node array")?;
    let carrier = nodes
        .iter_mut()
        .find(|node| {
            node.get("extras")
                .and_then(|extras| extras.get("bevyout_native_nif_converter"))
                .and_then(serde_json::Value::as_bool)
                == Some(true)
        })
        .context("native GLB has no metadata carrier node")?;
    let extras = carrier
        .get_mut("extras")
        .and_then(serde_json::Value::as_object_mut)
        .context("native GLB metadata carrier has no extras object")?;
    extras.insert(
        "bevyout_root_transform_policy".into(),
        root_transform_policy.tag().into(),
    );
    extras.insert("bevyout_spatial_audit_version".into(), 1.into());
    extras.insert(
        "bevyout_expected_spatial_corrections".into(),
        spatial_corrections.into(),
    );
    extras.insert(
        "bevyout_verified_spatial_corrections".into(),
        spatial_corrections.into(),
    );
    extras.insert("bevyout_expected_collision_corrections".into(), 0.into());
    extras.insert("bevyout_verified_collision_corrections".into(), 0.into());
    extras.insert(
        "bevyout_record_zero_non_identity".into(),
        record_zero_non_identity.into(),
    );

    let mut json = serde_json::to_vec(&document)?;
    while json.len() % 4 != 0 {
        json.push(b' ');
    }
    let json_length = u32::try_from(json.len()).context("native GLB JSON exceeds u32")?;
    let remaining_offset =
        20 + usize::try_from(u32::from_le_bytes(bytes[12..16].try_into().unwrap()))?;
    let total_length = 20_usize
        .checked_add(json.len())
        .and_then(|length| length.checked_add(bytes.len() - remaining_offset))
        .context("native GLB length overflow")?;
    let mut patched = Vec::with_capacity(total_length);
    patched.extend_from_slice(&bytes[0..8]);
    patched.extend_from_slice(&u32::try_from(total_length)?.to_le_bytes());
    patched.extend_from_slice(&json_length.to_le_bytes());
    patched.extend_from_slice(b"JSON");
    patched.extend_from_slice(&json);
    patched.extend_from_slice(&bytes[remaining_offset..]);
    gltf::Gltf::from_slice(&patched).context("validating native GLB metadata patch")?;
    Ok(patched)
}

fn conversion_name(mode: NifConversionMode) -> &'static str {
    match mode {
        NifConversionMode::Preserve => "preserve",
        NifConversionMode::QuickAo => "quick-ao",
    }
}

fn ensure_output_available(path: &Path, force: bool) -> Result<()> {
    if path.exists() && !force {
        bail!(
            "output already exists: {} (pass --force to replace it)",
            path.display()
        );
    }
    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8], force: bool) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating output directory {}", parent.display()))?;
    }
    let temporary = path.with_extension(format!(
        "{}.tmp-{}",
        path.extension()
            .and_then(|value| value.to_str())
            .unwrap_or("output"),
        std::process::id()
    ));
    fs::write(&temporary, bytes)
        .with_context(|| format!("writing temporary output {}", temporary.display()))?;
    if force && path.exists() {
        fs::remove_file(path).with_context(|| format!("replacing {}", path.display()))?;
    }
    fs::rename(&temporary, path)
        .with_context(|| format!("publishing output {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_normal_conversion_separates_normal_from_shared_specular_source() {
        let source_path = "textures/shared_payload.dds".to_string();
        let mut source = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            1,
            1,
            image::Rgba([12, 34, 56, 78]),
        ))
        .write_to(&mut source, image::ImageFormat::Png)
        .unwrap();
        let source = source.into_inner();
        let mut textures = BTreeMap::from([(source_path.clone(), source.clone())]);
        let mut materials = vec![nif::fo3::SceneMaterial {
            name: "shared normal/specular".into(),
            base_color: [1.0; 4],
            emissive: [0.0; 3],
            emissive_multiplier: 1.0,
            roughness: 0.5,
            alpha_mode: nif::fo3::SceneAlphaMode::Opaque,
            alpha_cutoff: None,
            double_sided: false,
            unlit: false,
            diffuse_texture: Some(source_path.clone()),
            normal_texture: Some(source_path.clone()),
            specular_texture: Some(source_path.clone()),
            glow_texture: None,
        }];

        prepare_native_normal_textures(&mut materials, &mut textures).unwrap();

        let derived_path = materials[0]
            .normal_texture
            .as_deref()
            .expect("normal path is retained");
        assert_ne!(derived_path, source_path);
        assert_eq!(
            materials[0].diffuse_texture.as_deref(),
            Some(source_path.as_str())
        );
        assert_eq!(materials[0].specular_texture.as_deref(), Some(derived_path));
        assert_eq!(textures.get(&source_path), Some(&source));
        let converted = image::load_from_memory(
            textures
                .get(derived_path)
                .expect("derived normal image was inserted"),
        )
        .unwrap()
        .to_rgba8();
        assert_eq!(converted.get_pixel(0, 0).0, [12, 221, 56, 78]);
    }

    #[test]
    fn asset_paths_are_data_relative_and_portable() {
        assert_eq!(
            normalize_asset_path("Clutter\\Desk.NIF"),
            "meshes/clutter/desk.nif"
        );
        assert_eq!(
            normalize_asset_path("meshes/Ammo/10mm.nif"),
            "meshes/ammo/10mm.nif"
        );
    }

    #[test]
    fn existing_outputs_require_force() {
        let path = std::env::temp_dir().join(format!(
            "bevyout-nif-convert-existing-{}.glb",
            std::process::id()
        ));
        fs::write(&path, b"old").unwrap();
        assert!(ensure_output_available(&path, false).is_err());
        assert!(ensure_output_available(&path, true).is_ok());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn reusable_conversion_rejects_malformed_nif_without_publishing_outputs() {
        let root = std::env::temp_dir().join(format!(
            "bevyout-nif-convert-malformed-{}",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();
        let output = root.join("malformed.glb");
        let physics = root.join("malformed.physics.json.gz");
        let error = convert_nif(NifConversionRequest {
            source_name: "malformed.nif",
            nif_bytes: b"not a nif",
            output: &output,
            physics_output: Some(&physics),
            report: None,
            conversion: NifConversionMode::Preserve,
            root_transform_policy: RootTransformPolicy::PreserveReviewRequired,
            allow_lossy: true,
            force: true,
            data_root: Some(&root),
            archives: &[],
        })
        .unwrap_err();
        assert!(error.to_string().contains("parsing FO3/FNV NIF"));
        assert!(!output.exists());
        assert!(!physics.exists());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn native_joint_preserves_authored_frames_limits_and_strength() {
        let source = nif::fo3::PhysicsJoint {
            source_block: 42,
            kind: "spherical".into(),
            body_a: 3,
            body_b: 7,
            anchor_a: [1.0, 2.0, 3.0],
            anchor_b: [1.0, 2.0, 3.0],
            frame_a_rotation_xyzw: [0.0, 0.0, 0.70710677, 0.70710677],
            frame_b_rotation_xyzw: [0.0, 0.0, 0.70710677, 0.70710677],
            lower_limit: None,
            upper_limit: None,
            cone_limit: Some(1.2),
            plane_lower_limit: Some(-0.4),
            plane_upper_limit: Some(0.5),
            twist_lower_limit: Some(-0.7),
            twist_upper_limit: Some(0.8),
            malleable_strength: Some(0.9),
        };

        let converted = convert_physics_joint(source);

        assert_eq!(converted.kind, "spherical");
        assert_eq!((converted.body_a, converted.body_b), (3, 7));
        assert_eq!(converted.frame_a_rotation_xyzw[3], 0.70710677);
        assert_eq!(converted.cone_limit, Some(1.2));
        assert_eq!(converted.twist_upper_limit, Some(0.8));
        assert_eq!(converted.malleable_strength, Some(0.9));
        assert_eq!(converted.source, PreparedPhysicsJointSource::Authored);
    }
}
