//! Experimental native FO3/FNV NIF-to-GLB vertical slice.
//!
//! This owns its CLI input resolution, focused NIF conversion, texture
//! resolution, reports, and output writes. The established Blender conversion
//! route used by `prepare` intentionally remains unchanged.

use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use flate2::{Compression, GzBuilder};
use serde::Serialize;

use crate::cli::{NifConversionMode, NifConvertArgs};

use super::assets::{load_archives, resolve_asset};
use super::physics::{
    PHYSICS_ASSET_SCHEMA_VERSION, PreparedPhysicsAsset, PreparedPhysicsBody, PreparedPhysicsShape,
    PreparedPhysicsSource, validate_physics_asset,
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
}

#[derive(Debug, Serialize)]
struct ReportIssue {
    block: usize,
    block_type: String,
    message: String,
}

pub fn nif_convert(args: NifConvertArgs) -> Result<()> {
    ensure_output_available(&args.output, args.force)?;
    if let Some(path) = &args.report {
        ensure_output_available(path, args.force)?;
    }
    if let Some(path) = &args.physics_output {
        ensure_output_available(path, args.force)?;
    }

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

    println!("nif-convert: input {source_name}");
    let document = nif::fo3::parse(&nif_bytes).context("parsing FO3/FNV NIF 20.2.0.7")?;
    println!(
        "nif-convert: parsed version=20.2.0.7 bethesda={} blocks={}",
        document.header.bethesda.version,
        document.blocks.len()
    );
    let mut scene = nif::fo3::extract_scene(&document).context("extracting NIF scene")?;
    apply_conversion_mode(&mut scene, args.conversion);
    println!(
        "nif-convert: scene meshes={} vertices={} triangles={}",
        scene.statistics.source_meshes,
        scene.statistics.source_vertices,
        scene.statistics.source_triangles
    );
    println!(
        "nif-convert: animations={} channels={}",
        scene.animations.len(),
        scene
            .animations
            .iter()
            .map(|animation| animation.channels.len())
            .sum::<usize>()
    );
    if !args.allow_lossy && !scene.issues.is_empty() {
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

    let textures = resolve_textures(&scene, data_root.as_deref(), &archives)?;
    let output = nif::fo3::encode_glb(
        &scene,
        &textures,
        &nif::fo3::GlbOptions {
            source_name: source_name.clone(),
            allow_missing_textures: args.allow_lossy,
        },
    )
    .context("encoding self-contained GLB")?;
    println!(
        "nif-convert: textures embedded={} missing={}",
        textures.len().saturating_sub(output.missing_textures.len()),
        output.missing_textures.len()
    );

    let (physics, physics_bytes, physics_bodies, physics_shapes) = if args.physics_output.is_some()
    {
        let physics_scene =
            nif::fo3::extract_physics(&document).context("extracting authored Havok collision")?;
        if !args.allow_lossy && !physics_scene.issues.is_empty() {
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
        )
    } else {
        ("not-requested", None, 0, 0)
    };

    let report_bytes = if args.report.is_some() {
        let report = ConversionReport {
            converter: "nifty-fo3-native-v1",
            source: source_name,
            output: args.output.display().to_string(),
            conversion: conversion_name(args.conversion),
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
            physics_output: args
                .physics_output
                .as_ref()
                .map(|path| path.display().to_string()),
            physics_bodies,
            physics_shapes,
        };
        let mut bytes = serde_json::to_vec_pretty(&report)?;
        bytes.push(b'\n');
        Some(bytes)
    } else {
        None
    };

    atomic_write(&args.output, &output.bytes, args.force)?;
    println!(
        "nif-convert: wrote {} bytes -> {}",
        output.bytes.len(),
        args.output.display()
    );
    if let (Some(path), Some(bytes)) = (&args.physics_output, &physics_bytes) {
        atomic_write(path, bytes, args.force)?;
        println!(
            "nif-convert: physics bodies={} shapes={} -> {}",
            physics_bodies,
            physics_shapes,
            path.display()
        );
    }
    if let (Some(path), Some(bytes)) = (&args.report, &report_bytes) {
        atomic_write(path, bytes, args.force)?;
        println!("nif-convert: report -> {}", path.display());
    }
    Ok(())
}

fn encode_physics_sidecar(scene: nif::fo3::PhysicsScene) -> Result<Vec<u8>> {
    let asset = PreparedPhysicsAsset {
        schema_version: PHYSICS_ASSET_SCHEMA_VERSION,
        source: PreparedPhysicsSource::AuthoredHavok,
        bodies: scene
            .bodies
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
        // Ragdoll constraints are deliberately deferred; ordinary props preserve whether a body
        // is constrained so the runtime never misclassifies it as a free dynamic object.
        joints: Vec::new(),
    };
    validate_physics_asset(&asset)?;
    let json = serde_json::to_vec(&asset)?;
    let mut encoder = GzBuilder::new()
        .mtime(0)
        .write(Vec::new(), Compression::default());
    encoder.write_all(&json)?;
    encoder.finish().context("finishing physics sidecar gzip")
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
}
