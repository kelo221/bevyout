//! Production native FO3/FNV NIF-to-GLB conversion.
//!
//! This owns its CLI input resolution, focused NIF conversion, texture
//! resolution, reports, and output writes. The production preparation path no
//! longer selects Blender. Remaining limitations are deliberately reported by
//! the native converter: VWD/distant geometry generation and NIF blocks that
//! are unsupported or lossy. Fallout's segmented terrain LOD shape carries a
//! segment table that is not needed for its render geometry; the converter
//! normalizes that table under the explicit lossy policy so the vanilla LOD
//! assets remain usable. Exterior packages own terrain, distant references,
//! and navigation; the converter is not invoked by the runtime viewer.

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
    MetallicMaterialTable, RootTransformPolicy, fallout_specular_texture_path,
    flip_directx_normal_y_texel, load_archives, patch_glb_material_policy,
    perceptual_roughness_from_glossiness, resolve_asset,
};
use super::physics::{
    PHYSICS_ASSET_SCHEMA_VERSION, PreparedPhysicsAsset, PreparedPhysicsBody, PreparedPhysicsJoint,
    PreparedPhysicsJointSource, PreparedPhysicsShape, PreparedPhysicsSource,
    validate_physics_asset,
};

#[derive(Debug, Serialize)]
struct ConversionReport {
    converter: String,
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

pub(crate) const NATIVE_NIF_REPORT_REVISION: &str = "nifty-fo3-native-v10-normal-y-v1-specular-normal-alpha-v1-fallout-shader-semantics-v1-emissive-quarter-cap-v1-shader-emission-gate-v2-physical-effect-bulb-v1-effect-emission-control-v1-light-card-promotion-v1-env-light-emission-v1-17f5769-skin-anim-xyzw-v1-audio-cues-v1-havok-joints-v1-com-frame-v1";

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
    /// Deterministic in-memory images synthesized during actor assembly.
    /// They are merged after ordinary Data/BSA resolution and then take the
    /// same embedded-GLB -> UASTC KTX2 path as authored textures.
    pub(crate) extra_textures: BTreeMap<String, Vec<u8>>,
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

#[derive(Debug)]
struct NifLayout {
    block_type_names: Vec<String>,
    block_type_indices: Vec<u16>,
    block_sizes: Vec<u32>,
    block_sizes_start: usize,
    block_sizes_end: usize,
    payload_start: usize,
    payload_end: usize,
}

/// Strip only the Bethesda segment table appended to FO3's
/// `BSSegmentedTriShape` payload. Nifty already parses the shared
/// `NiTriShape` geometry prefix; the table is a render-irrelevant partition
/// hint used by the original renderer. Keeping this normalization here makes
/// the compatibility policy explicit and leaves the pinned dependency
/// untouched.
fn normalize_segmented_shape_tables(bytes: &[u8]) -> Result<(Option<Vec<u8>>, Vec<usize>)> {
    let Some(layout) = parse_nif_layout(bytes) else {
        return Ok((None, Vec::new()));
    };
    let mut new_sizes = layout.block_sizes.clone();
    let mut normalized_blocks = Vec::new();
    let mut payload_offset = layout.payload_start;
    for (index, size) in layout.block_sizes.iter().enumerate() {
        let size = usize::try_from(*size).context("NIF block size does not fit usize")?;
        let end = payload_offset
            .checked_add(size)
            .context("NIF block payload offset overflows")?;
        let type_index = usize::from(layout.block_type_indices[index]);
        if layout
            .block_type_names
            .get(type_index)
            .is_some_and(|name| name == "BSSegmentedTriShape")
            && let Some(suffix_len) = segmented_shape_suffix_len(&bytes[payload_offset..end])
        {
            new_sizes[index] = u32::try_from(size - suffix_len)
                .context("normalized NIF block size does not fit u32")?;
            normalized_blocks.push(index);
        }
        payload_offset = end;
    }
    if normalized_blocks.is_empty() {
        return Ok((None, normalized_blocks));
    }

    let removed_bytes = layout
        .block_sizes
        .iter()
        .zip(&new_sizes)
        .map(|(old, new)| usize::try_from(old.saturating_sub(*new)).unwrap_or(0))
        .sum::<usize>();
    let mut normalized = Vec::with_capacity(bytes.len().saturating_sub(removed_bytes));
    normalized.extend_from_slice(&bytes[..layout.block_sizes_start]);
    for size in &new_sizes {
        normalized.extend_from_slice(&size.to_le_bytes());
    }
    normalized.extend_from_slice(&bytes[layout.block_sizes_end..layout.payload_start]);
    let mut payload_offset = layout.payload_start;
    for (index, size) in layout.block_sizes.iter().enumerate() {
        let old_size = usize::try_from(*size).context("NIF block size does not fit usize")?;
        let new_size = usize::try_from(new_sizes[index])
            .context("normalized NIF block size does not fit usize")?;
        normalized.extend_from_slice(&bytes[payload_offset..payload_offset + new_size]);
        payload_offset += old_size;
    }
    normalized.extend_from_slice(&bytes[layout.payload_end..]);
    Ok((Some(normalized), normalized_blocks))
}

fn parse_nif_layout(bytes: &[u8]) -> Option<NifLayout> {
    const HEADER: &[u8] = b"Gamebryo File Format, Version 20.2.0.7";
    let line_end = bytes.iter().position(|byte| *byte == b'\n')?;
    if bytes.get(..line_end)? != HEADER {
        return None;
    }
    let mut offset = line_end + 1;
    if read_u32(bytes, &mut offset)? != nif::fo3::FILE_VERSION
        || read_u8(bytes, &mut offset)? != 1
        || read_u32(bytes, &mut offset)? != nif::fo3::USER_VERSION
    {
        return None;
    }
    let block_count = usize::try_from(read_u32(bytes, &mut offset)?).ok()?;
    if block_count > 1_000_000 {
        return None;
    }
    let _bethesda_version = read_u32(bytes, &mut offset)?;
    for _ in 0..3 {
        skip_short_string(bytes, &mut offset)?;
    }
    let type_count = usize::from(read_u16(bytes, &mut offset)?);
    let mut block_type_names = Vec::with_capacity(type_count);
    for _ in 0..type_count {
        block_type_names.push(read_sized_string(bytes, &mut offset)?);
    }
    let mut block_type_indices = Vec::with_capacity(block_count);
    for _ in 0..block_count {
        block_type_indices.push(read_u16(bytes, &mut offset)?);
    }
    let block_sizes_start = offset;
    let mut block_sizes = Vec::with_capacity(block_count);
    let mut payload_length = 0usize;
    for _ in 0..block_count {
        let size = read_u32(bytes, &mut offset)?;
        payload_length = payload_length.checked_add(usize::try_from(size).ok()?)?;
        block_sizes.push(size);
    }
    let block_sizes_end = offset;
    let string_count = usize::try_from(read_u32(bytes, &mut offset)?).ok()?;
    let _maximum_string_length = read_u32(bytes, &mut offset)?;
    for _ in 0..string_count {
        skip_sized_string(bytes, &mut offset)?;
    }
    let group_count = usize::try_from(read_u32(bytes, &mut offset)?).ok()?;
    let groups_bytes = group_count.checked_mul(4)?;
    offset = offset.checked_add(groups_bytes)?;
    if offset > bytes.len() {
        return None;
    }
    let payload_start = offset;
    let payload_end = payload_start.checked_add(payload_length)?;
    if payload_end.checked_add(4)? > bytes.len() {
        return None;
    }
    let root_count = u32::from_le_bytes(bytes[payload_end..payload_end + 4].try_into().ok()?);
    let footer_end = payload_end
        .checked_add(4)?
        .checked_add(usize::try_from(root_count).ok()?.checked_mul(4)?)?;
    (footer_end == bytes.len()).then_some(NifLayout {
        block_type_names,
        block_type_indices,
        block_sizes,
        block_sizes_start,
        block_sizes_end,
        payload_start,
        payload_end,
    })
}

fn segmented_shape_suffix_len(payload: &[u8]) -> Option<usize> {
    let mut best = None;
    for start in 80..payload.len().saturating_sub(12) {
        let count = u32::from_le_bytes(payload.get(start..start + 4)?.try_into().ok()?);
        if count == 0 || count > 1_000_000 {
            continue;
        }
        let suffix_len = 4usize.checked_add(usize::try_from(count).ok()?.checked_mul(9)?)?;
        if suffix_len == payload.len() - start
            && best.is_none_or(|(best_count, _)| count > best_count)
        {
            best = Some((count, suffix_len));
        }
    }
    best.map(|(_, suffix_len)| suffix_len)
}

fn read_u8(bytes: &[u8], offset: &mut usize) -> Option<u8> {
    let value = *bytes.get(*offset)?;
    *offset += 1;
    Some(value)
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let value = u16::from_le_bytes(bytes.get(*offset..end)?.try_into().ok()?);
    *offset = end;
    Some(value)
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let value = u32::from_le_bytes(bytes.get(*offset..end)?.try_into().ok()?);
    *offset = end;
    Some(value)
}

fn skip_short_string(bytes: &[u8], offset: &mut usize) -> Option<()> {
    let length = usize::from(read_u8(bytes, offset)?);
    let end = offset.checked_add(length)?;
    bytes.get(*offset..end)?;
    *offset = end;
    Some(())
}

fn skip_sized_string(bytes: &[u8], offset: &mut usize) -> Option<()> {
    let length = usize::try_from(read_u32(bytes, offset)?).ok()?;
    let end = offset.checked_add(length)?;
    bytes.get(*offset..end)?;
    *offset = end;
    Some(())
}

fn read_sized_string(bytes: &[u8], offset: &mut usize) -> Option<String> {
    let start = *offset;
    skip_sized_string(bytes, offset)?;
    Some(String::from_utf8_lossy(bytes.get(start + 4..*offset)?).into_owned())
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
    let (normalized_bytes, normalized_blocks) =
        normalize_segmented_shape_tables(request.nif_bytes)?;
    if !normalized_blocks.is_empty() {
        if !request.allow_lossy {
            bail!(
                "strict conversion rejected {} BSSegmentedTriShape segment table(s) (pass --allow-lossy to omit render-irrelevant segment metadata)",
                normalized_blocks.len()
            );
        }
        lines.push(format!(
            "nif-convert: normalized BSSegmentedTriShape blocks={} (segment metadata omitted)",
            normalized_blocks.len()
        ));
    }
    let parse_bytes = normalized_bytes.as_deref().unwrap_or(request.nif_bytes);
    let document = nif::fo3::parse(parse_bytes).context("parsing FO3/FNV NIF 20.2.0.7")?;
    lines.push(format!(
        "nif-convert: parsed version=20.2.0.7 bethesda={} blocks={}",
        document.header.bethesda.version,
        document.blocks.len()
    ));
    let mut scene = nif::fo3::extract_scene(&document).context("extracting NIF scene")?;
    apply_native_material_policy(&document, &mut scene)?;
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
    let removed_lod_skirt_triangles = apply_conversion_mode(&mut scene, request.conversion);
    if removed_lod_skirt_triangles > 0 {
        lines.push(format!(
            "nif-convert: removed worldspace LOD skirt triangles={removed_lod_skirt_triangles}"
        ));
    }
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
    output.bytes = patch_glb_material_policy(
        &output.bytes,
        &MetallicMaterialTable::built_in().map_err(anyhow::Error::msg)?,
    )?;
    output.bytes = super::assets::transcode_glb_images_to_ktx2(&output.bytes)
        .context("transcoding embedded GLB textures to KTX2")?;
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
    let lossy_scene_issues = scene.issues.len() + normalized_blocks.len();
    let report_bytes = if request.report.is_some() {
        let report = ConversionReport {
            converter: super::assets::material_policy_identity(NATIVE_NIF_REPORT_REVISION),
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
                .chain(normalized_blocks.iter().map(|block| ReportIssue {
                    block: *block,
                    block_type: "BSSegmentedTriShape".into(),
                    message: "render-irrelevant segment metadata omitted by native compatibility normalizer".into(),
                }))
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
    textures.extend(request.extra_textures);
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
    output.bytes = patch_glb_material_policy(
        &output.bytes,
        &MetallicMaterialTable::built_in().map_err(anyhow::Error::msg)?,
    )?;
    output.bytes = super::assets::transcode_glb_images_to_ktx2(&output.bytes)
        .context("transcoding native actor GLB textures to KTX2")?;

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

pub(crate) fn apply_native_material_policy(
    document: &nif::fo3::Document,
    scene: &mut nif::fo3::Scene,
) -> Result<()> {
    apply_fallout_specular_texture_policy(&mut scene.materials);
    for material in &mut scene.materials {
        material.roughness = perceptual_roughness_from_glossiness(None);
    }
    for node in &scene.nodes {
        let Some(material_index) = node.mesh.as_ref().and_then(|mesh| mesh.material) else {
            continue;
        };
        let geometry = match document.decode_block(node.source_block).with_context(|| {
            format!("decoding geometry block {} for material", node.source_block)
        })? {
            nif::fo3::TypedBlock::Geometry(geometry) => geometry,
            _ => continue,
        };
        let mut glossiness = None;
        for property in geometry
            .base
            .properties
            .into_iter()
            .filter(|index| *index >= 0)
        {
            if let nif::fo3::TypedBlock::MaterialProperty(material) = document
                .decode_block(property as usize)
                .with_context(|| format!("decoding material property block {property}"))?
            {
                glossiness = Some(material.glossiness);
                break;
            }
        }
        let material = scene.materials.get_mut(material_index).with_context(|| {
            format!("scene mesh references invalid material index {material_index}")
        })?;
        material.roughness = perceptual_roughness_from_glossiness(glossiness);
    }
    Ok(())
}

fn apply_fallout_specular_texture_policy(materials: &mut [nif::fo3::SceneMaterial]) {
    for material in materials {
        let features = nif::fo3::FalloutShaderFeatures::from_flags(
            material.shader_type,
            material.shader_flags_1,
            material.shader_flags_2,
        );
        material.specular_texture =
            fallout_specular_texture_path(features.specular, material.normal_texture.as_deref());
    }
}

/// Builds a distinct glTF image for every normal-map source. A same-source
/// specular slot follows the converted image because its alpha is unchanged;
/// this preserves Fallout's authored specular-strength payload. Diffuse and
/// glow slots retain the original image. The material slot, not the filename,
/// is the authority in the native converter.
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

fn apply_conversion_mode(scene: &mut nif::fo3::Scene, mode: NifConversionMode) -> usize {
    match mode {
        NifConversionMode::Preserve => 0,
        NifConversionMode::QuickAo => {
            for mesh in scene.nodes.iter_mut().filter_map(|node| node.mesh.as_mut()) {
                for color in &mut mesh.colors {
                    let ao =
                        (color[0] * 0.2126 + color[1] * 0.7152 + color[2] * 0.0722).clamp(0.0, 1.0);
                    *color = [ao, ao, ao, color[3]];
                }
            }
            0
        }
        NifConversionMode::WorldspaceLod => strip_worldspace_lod_skirts(scene),
    }
}

/// FO3's worldspace LOD tiles carry deep border skirts so the original
/// renderer can hide seams in its horizon pass. In the prepared Bevy scene
/// those skirts are visible as walls and cut-out panels because there is no
/// equivalent terrain-horizon clip. Remove only vertical/degenerate faces
/// (zero source-XY area) and triangles whose source-Z span exceeds a
/// conservative 2,048 plugin units; ordinary sloped terrain facets remain
/// intact.
fn strip_worldspace_lod_skirts(scene: &mut nif::fo3::Scene) -> usize {
    const MAX_TERRAIN_FACET_HEIGHT: f32 = 2_048.0;
    let mut removed = 0usize;
    for node in &mut scene.nodes {
        let Some(mesh) = node.mesh.as_mut() else {
            continue;
        };
        let mut retained = Vec::with_capacity(mesh.indices.len());
        for triangle in mesh.indices.chunks(3) {
            if triangle.len() != 3 {
                retained.extend_from_slice(triangle);
                continue;
            }
            let &[a_index, b_index, c_index] = triangle else {
                unreachable!("triangle length checked above");
            };
            let [a, b, c] = [
                mesh.positions[a_index as usize],
                mesh.positions[b_index as usize],
                mesh.positions[c_index as usize],
            ];
            let horizontal_area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            let minimum = a[2].min(b[2]).min(c[2]);
            let maximum = a[2].max(b[2]).max(c[2]);
            if horizontal_area.abs() <= 1.0 || maximum - minimum > MAX_TERRAIN_FACET_HEIGHT {
                removed += 1;
            } else {
                retained.extend_from_slice(triangle);
            }
        }
        mesh.indices = retained;
        if mesh.indices.is_empty() {
            let mesh = node.mesh.take().expect("mesh was present above");
            scene.statistics.source_meshes = scene.statistics.source_meshes.saturating_sub(1);
            scene.statistics.source_vertices = scene
                .statistics
                .source_vertices
                .saturating_sub(mesh.positions.len());
        }
    }
    scene.statistics.source_triangles = scene.statistics.source_triangles.saturating_sub(removed);
    removed
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
        NifConversionMode::WorldspaceLod => "worldspace-lod",
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
#[path = "tests/mod.rs"]
mod tests;
