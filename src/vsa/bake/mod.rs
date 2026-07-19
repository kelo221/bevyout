use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

mod batch;
mod gltf_extension_policy;
mod job;
mod plan;
mod policy;
mod rust_irradiance;
mod rust_scene;
mod tools;

pub(crate) use batch::*;
pub(crate) use job::*;
pub(crate) use plan::*;
pub(crate) use tools::*;

use crate::cli::{BakeArgs, BakeQuality};

use super::assets::{SUPPORTED_PREPARED_CONVERTER_REVISIONS, find_blender};
use super::manifest::{
    CURRENT_BAKE_REVISION, CURRENT_MANIFEST_SCHEMA_VERSION, PreparedCellLighting,
    PreparedIrradianceVolume, PreparedPhysicsClassification, PreparedPlacement,
    PreparedSceneManifest, PreparedSemantic, cell_label, ensure_prepared_manifest_compatible_any,
    is_pickup_record_kind,
};
use super::physics::PHYSICS_ASSET_SCHEMA_VERSION;
use super::scenes::resolve_cached_manifest;

pub fn bake(args: BakeArgs) -> Result<()> {
    // Batch mode (issue #62): `--all-interiors`/`--retry-failed` walk the
    // prepared cell catalogue through the resumable bake job manifest; a
    // single selector/--manifest keeps the original single-cell path, which
    // batch mode reuses per cell via `bake_manifest`.
    if args.all_interiors || args.retry_failed {
        return bake_batch(args);
    }
    let manifest_path = match (args.selector.as_deref(), args.manifest.as_deref()) {
        (Some(_), Some(_)) => {
            bail!("choose either a GECK EditorID/FormID selector or --manifest, not both")
        }
        (Some(selector), None) => resolve_cached_manifest(
            args.cache_dir
                .as_deref()
                .unwrap_or_else(|| Path::new(".bevyout/cache")),
            selector,
        )?,
        (None, Some(manifest)) => fs::canonicalize(manifest).context("manifest does not exist")?,
        (None, None) => bail!(
            "provide a GECK EditorID/FormID selector or --manifest; run `bevyout bake --help` for usage"
        ),
    };
    bake_manifest(&args, &manifest_path)
}

/// Reads and validates the prepared scene manifest a bake starts from:
/// parseable, compatible converter/physics revisions, and at least one
/// renderable placement. Shared by the single-cell path, each batch cell,
/// and the batch validity check (`batch::recorded_bake_validity`).
pub(crate) fn load_prepared_manifest(manifest_path: &Path) -> Result<PreparedSceneManifest> {
    let text = fs::read_to_string(manifest_path).with_context(|| {
        format!(
            "could not read scene manifest {}; run prepare before bake",
            manifest_path.display()
        )
    })?;
    let manifest: PreparedSceneManifest =
        ron::de::from_str(&text).context("invalid scene manifest; run prepare before bake")?;
    ensure_prepared_manifest_compatible_any(
        &manifest,
        SUPPORTED_PREPARED_CONVERTER_REVISIONS,
        PHYSICS_ASSET_SCHEMA_VERSION,
    )?;
    if manifest
        .placements
        .iter()
        .all(|placement| placement.asset_path.is_none())
    {
        bail!("scene manifest contains no renderable placements");
    }
    Ok(manifest)
}

/// The canonical asset root and every output path a bake derives from it.
/// Building these performs no writes (only the asset-root canonicalization
/// reads the filesystem), so the batch validity check can rebuild a cell's
/// job -- and therefore its job fingerprint -- without touching outputs.
pub(crate) struct BakeOutputs {
    pub(crate) asset_root: PathBuf,
    pub(crate) output_dir: PathBuf,
    pub(crate) output_scene: PathBuf,
    pub(crate) preview_output: PathBuf,
    pub(crate) result_json: PathBuf,
    pub(crate) irradiance_blend: PathBuf,
    pub(crate) irradiance_raw: PathBuf,
    pub(crate) job_file: PathBuf,
}

pub(crate) fn bake_outputs(manifest: &PreparedSceneManifest) -> Result<BakeOutputs> {
    let asset_root = fs::canonicalize(&manifest.asset_root)
        .with_context(|| format!("asset root does not exist: {}", manifest.asset_root))?;
    let output_dir = asset_root
        .join("scenes")
        .join(format!("{:08x}", manifest.cell.form_id))
        .join("baked");
    Ok(BakeOutputs {
        output_scene: output_dir.join("scene.glb"),
        preview_output: output_dir.join("preview.png"),
        result_json: output_dir.join("result.json"),
        irradiance_blend: output_dir.join("irradiance.blend"),
        irradiance_raw: output_dir.join("irradiance.raw"),
        job_file: output_dir.join("job.json"),
        asset_root,
        output_dir,
    })
}

/// Assembles the Blender job for one prepared cell. Pure over its inputs:
/// the same manifest, bake arguments, and output paths always produce the
/// same job, which is what makes `bake_job_fingerprint` a meaningful
/// validity key for the batch skip check (F62.1).
pub(crate) fn build_bake_job(
    manifest: &PreparedSceneManifest,
    args: &BakeArgs,
    outputs: &BakeOutputs,
) -> BakeJob {
    let cell_lighting =
        manifest
            .cell
            .effective_lighting
            .clone()
            .unwrap_or_else(|| PreparedCellLighting {
                ambient_rgba: manifest.cell.ambient_rgba,
                directional_rgba: manifest.cell.directional_rgba,
                ..Default::default()
            });
    BakeJob {
        asset_root: blender_path(&outputs.asset_root),
        output_scene: blender_path(&outputs.output_scene),
        preview_output: blender_path(&outputs.preview_output),
        result_json: blender_path(&outputs.result_json),
        irradiance_blend: blender_path(&outputs.irradiance_blend),
        irradiance_spacing_meters: args.irradiance_spacing_meters,
        irradiance_samples: args.irradiance_samples,
        preview_only: matches!(args.quality, BakeQuality::Preview),
        static_batch_chunk_meters: args.static_batch_chunk_meters,
        // Runtime glow maps are intentionally much brighter than their physical
        // bake contribution so they remain visible under Bloom in the viewer.
        emission_scale: 0.01,
        ambient_rgba: cell_lighting.ambient_rgba,
        cell_directional_rgba: cell_lighting.directional_rgba,
        cell_directional_rotation_xyzw: cell_lighting.directional_rotation_xyzw(),
        cell_directional_illuminance: cell_directional_illuminance(&cell_lighting),
        placements: manifest
            .placements
            .iter()
            .filter(|placement| placement.initially_enabled && is_bake_static(placement))
            .filter_map(|placement| {
                Some(JobPlacement {
                    reference_form_id: placement.reference_form_id,
                    asset_path: placement.asset_path.clone()?,
                    ao_mode: placement.ao_mode.clone(),
                    batchable_static: is_batchable_static(placement),
                    translation: placement.translation,
                    rotation_xyzw: placement.rotation_xyzw,
                    scale: placement.scale,
                })
            })
            .collect(),
        lights: manifest
            .lights
            .iter()
            .filter(|light| light.initially_enabled)
            .map(|light| JobLight {
                translation: light.translation,
                rotation_xyzw: light.rotation_xyzw,
                color_rgba: light.color_rgba,
                radius: light.radius.max(0.01),
                intensity_lumens: if light.intensity_lumens > 0.0 {
                    light.intensity_lumens
                } else {
                    light.radius * light.radius * 2.0 * 8192.0
                },
                kind: if light.kind.is_empty() {
                    "point".to_owned()
                } else {
                    light.kind.clone()
                },
            })
            .collect(),
    }
}

/// The job-parameter fingerprint recorded as `PreparedBake.source_fingerprint`
/// after a successful bake, and recomputed by the batch skip check to decide
/// whether a recorded bake is still valid (F62.1): the prepared manifest's
/// own `source_fingerprint` plus the serialized Blender job, so both a
/// re-prepared cell and changed bake parameters invalidate a recorded bake.
pub(crate) fn bake_job_fingerprint(
    manifest: &PreparedSceneManifest,
    job: &BakeJob,
) -> Result<String> {
    let mut fingerprint = Sha256::new();
    fingerprint.update(manifest.source_fingerprint.as_bytes());
    fingerprint.update(serde_json::to_vec(job)?);
    Ok(format!("{:x}", fingerprint.finalize()))
}

/// Bakes one prepared scene manifest in place: the whole original
/// single-cell `bake` path after selector resolution, reused verbatim by
/// each batch cell.
pub(crate) fn bake_manifest(args: &BakeArgs, manifest_path: &Path) -> Result<()> {
    let mut manifest = load_prepared_manifest(manifest_path)?;

    let blender = if matches!(args.quality, BakeQuality::Preview) {
        Some(find_blender(args.blender.clone())?)
    } else {
        None
    };
    let ktx_tool = if matches!(args.quality, BakeQuality::Irradiance) {
        Some(find_irradiance_ktx_tool(args.toktx.clone())?)
    } else {
        None
    };
    let outputs = bake_outputs(&manifest)?;
    // Calling bake is itself the user's request to regenerate this output.
    // Keep reading the legacy --force flag for existing scripts, but do not
    // require a second confirmation before replacing known bake artifacts.
    let _legacy_force = args.force;
    fs::create_dir_all(&outputs.output_dir)?;

    let BakeOutputs {
        asset_root,
        output_dir,
        output_scene,
        preview_output,
        result_json,
        irradiance_blend,
        irradiance_raw,
        job_file,
    } = &outputs;
    let mut job = build_bake_job(&manifest, args, &outputs);
    exclude_animated_static_assets(asset_root, &mut job)?;
    fs::write(job_file, serde_json::to_vec_pretty(&job)?)?;

    let script_file = output_dir.join("blender_bake.py");
    fs::write(&script_file, include_str!("../blender_bake.py"))?;
    if matches!(args.quality, BakeQuality::Preview) {
        let _ = fs::remove_file(preview_output);
    } else {
        for path in [output_scene, irradiance_blend, result_json] {
            let _ = fs::remove_file(path);
        }
    }
    if matches!(args.quality, BakeQuality::Preview) {
        let blender_status = Command::new(blender.expect("preview resolves Blender"))
            .arg("--background")
            .arg("--factory-startup")
            .arg("--python")
            .arg(&script_file)
            .arg("--")
            .arg(job_file)
            .current_dir(asset_root)
            .status()
            .context("failed to start Blender")?;
        if !blender_status.success() {
            bail!("Blender bake failed with {blender_status}");
        }
        if !preview_output.exists() {
            bail!(
                "Blender reported success but did not create the preview image {}",
                preview_output.display()
            );
        }
        if !args.keep_intermediate {
            let _ = fs::remove_file(job_file);
            let _ = fs::remove_file(&script_file);
        }
        println!("Eevee preview rendered -> {}", preview_output.display());
        return Ok(());
    }

    let started = Instant::now();
    let temporary_scene = output_dir.join("scene.rust.tmp.glb");
    let ktx2_path = output_dir.join("irradiance.ktx2");
    let temporary_ktx = output_dir.join("irradiance.rust.tmp.ktx2");
    for path in [&temporary_scene, &temporary_ktx] {
        let _ = fs::remove_file(path);
    }
    println!(
        "Rust bake: composing {} static placements",
        job.placements.len()
    );
    let mut rust_scene =
        rust_scene::compose_scene(asset_root, &job.placements, args.static_batch_chunk_meters)?;
    let composed_elapsed = started.elapsed();
    println!(
        "Rust bake: composed {} primitives in {:.2}s",
        rust_scene.primitives.len(),
        composed_elapsed.as_secs_f64()
    );
    rust_scene.write_glb(&temporary_scene)?;
    let irradiance = rust_irradiance::bake_irradiance(
        &rust_scene,
        &job.lights,
        &rust_irradiance::DirectionalBakeLight {
            color_rgba: job.cell_directional_rgba,
            rotation_xyzw: job.cell_directional_rotation_xyzw,
            illuminance: job.cell_directional_illuminance,
        },
        args.irradiance_spacing_meters,
        args.irradiance_samples,
        irradiance_raw,
    )?;
    let ktx_tool = ktx_tool.expect("irradiance bake always resolves a KTX tool");
    let mut ktx_command = Command::new(&ktx_tool.path);
    match ktx_tool.kind {
        KtxToolKind::LegacyToktx => {
            bail!(
                "irradiance volume export requires the unified KTX executable (ktx.exe), not legacy toktx.exe"
            );
        }
        KtxToolKind::UnifiedKtx => {
            ktx_command
                .arg("create")
                .arg("--raw")
                .arg("--format")
                .arg("E5B9G9R9_UFLOAT_PACK32")
                .arg("--width")
                .arg(irradiance.resolution[0].to_string())
                .arg("--height")
                .arg((2 * irradiance.resolution[1]).to_string())
                .arg("--depth")
                .arg((3 * irradiance.resolution[2]).to_string())
                .arg("--assign-tf")
                .arg("linear")
                .arg("--assign-texcoord-origin")
                .arg("top-left-front")
                .arg("--zstd")
                .arg("3");
        }
    }
    for raw_slice in &irradiance.raw_slices {
        ktx_command.arg(raw_slice);
    }
    let ktx_output = ktx_command
        .arg(&temporary_ktx)
        .output()
        .context("failed to start KTX-Software")?;
    if !ktx_output.status.success() {
        if !args.keep_intermediate {
            bail!(
                "KTX-Software failed with {}:\n{}\n{}\nraw irradiance data was kept at {}",
                ktx_output.status,
                tail(&ktx_output.stdout),
                tail(&ktx_output.stderr),
                irradiance_raw.display()
            );
        }
        bail!(
            "KTX-Software failed with {}:\n{}\n{}\nraw irradiance data was kept at {}",
            ktx_output.status,
            tail(&ktx_output.stdout),
            tail(&ktx_output.stderr),
            irradiance_raw.display()
        );
    }

    let ktx_validation = Command::new(&ktx_tool.path)
        .arg("info")
        .arg(&temporary_ktx)
        .output()
        .context("failed to validate Rust irradiance KTX2")?;
    if !ktx_validation.status.success() {
        bail!(
            "KTX validation failed with {}:\n{}",
            ktx_validation.status,
            tail(&ktx_validation.stderr)
        );
    }
    let batching = &rust_scene.batching;
    println!(
        "static batching ({:.1} m chunks): objects {} -> {}, primitives {} -> {}, materials {} -> {}; {} batches (largest {}), excluded {} large",
        batching.chunk_size_meters,
        batching.visual_objects_before,
        batching.visual_objects_after,
        batching.render_primitives_before,
        batching.render_primitives_after,
        batching.materials_before,
        batching.materials_after,
        batching.batches_created,
        batching.largest_batch,
        batching.excluded_large,
    );
    println!(
        "seam stitch: matched {} boundary edges, adjusted {} vertices, max correction {:.3} mm",
        batching.seam_edges_matched,
        batching.seam_vertices_adjusted,
        batching.seam_max_correction_meters * 1000.0,
    );
    replace_output(&temporary_scene, output_scene)?;
    replace_output(&temporary_ktx, &ktx2_path)?;
    let scene_path = relative_asset_path(asset_root, output_scene)?;
    let irradiance_path = relative_asset_path(asset_root, &ktx2_path)?;
    let source_fingerprint = bake_job_fingerprint(&manifest, &job)?;
    manifest.schema_version = CURRENT_MANIFEST_SCHEMA_VERSION;
    manifest.bake = Some(super::manifest::PreparedBake {
        bake_revision: Some(CURRENT_BAKE_REVISION.into()),
        source_fingerprint,
        scene_path,
        irradiance_volume: Some(PreparedIrradianceVolume {
            asset_path: irradiance_path,
            translation: irradiance.translation,
            rotation_xyzw: irradiance.rotation_xyzw,
            scale: irradiance.scale,
            resolution: irradiance.resolution,
            intensity: 1.0,
        }),
    });
    fs::write(
        manifest_path,
        to_string_pretty(&manifest, PrettyConfig::default())?,
    )?;
    if !args.keep_intermediate {
        for raw_slice in &irradiance.raw_slices {
            let _ = fs::remove_file(raw_slice);
        }
        let _ = fs::remove_file(irradiance_raw);
        let _ = fs::remove_file(job_file);
        let _ = fs::remove_file(&script_file);
        let _ = fs::remove_file(result_json);
        let _ = fs::remove_file(irradiance_blend);
    }
    println!(
        "baked {} in {:.2}s: Rust irradiance {:?}, {} nonzero face voxels, max {:.3}, {} primary rays -> {}",
        cell_label(&manifest.cell),
        started.elapsed().as_secs_f64(),
        irradiance.resolution,
        irradiance.nonzero_voxels,
        irradiance.maximum,
        irradiance.primary_rays,
        ktx2_path.display()
    );
    Ok(())
}

fn replace_output(temporary: &Path, final_path: &Path) -> Result<()> {
    if !temporary.is_file() {
        bail!(
            "expected bake output was not created: {}",
            temporary.display()
        );
    }
    if final_path.exists() {
        fs::remove_file(final_path)
            .with_context(|| format!("could not replace {}", final_path.display()))?;
    }
    fs::rename(temporary, final_path).with_context(|| {
        format!(
            "could not publish {} as {}",
            temporary.display(),
            final_path.display()
        )
    })
}

pub(crate) fn is_bake_static(placement: &PreparedPlacement) -> bool {
    placement.physics_classification != PreparedPhysicsClassification::Dynamic
        && matches!(placement.semantic, PreparedSemantic::Static)
        && !is_pickup_record_kind(&placement.base_kind)
}

fn exclude_animated_static_assets(asset_root: &Path, job: &mut BakeJob) -> Result<()> {
    let mut animation_by_asset = HashMap::<String, bool>::new();
    let mut retained = Vec::with_capacity(job.placements.len());
    for placement in job.placements.drain(..) {
        let animated = if let Some(animated) = animation_by_asset.get(&placement.asset_path) {
            *animated
        } else {
            let animated = rust_scene::asset_contains_animation(asset_root, &placement.asset_path)?;
            animation_by_asset.insert(placement.asset_path.clone(), animated);
            animated
        };
        if animated {
            println!(
                "Rust bake: skipping animated static placement {:08x} ({})",
                placement.reference_form_id, placement.asset_path
            );
        } else {
            retained.push(placement);
        }
    }
    job.placements = retained;
    Ok(())
}

fn is_batchable_static(placement: &PreparedPlacement) -> bool {
    is_bake_static(placement)
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
