use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

mod job;
mod policy;
mod rust_irradiance;
mod rust_scene;
mod tools;

pub(crate) use job::*;
pub(crate) use tools::*;

use crate::cli::{BakeArgs, BakeQuality};

use super::assets::{NIF_CONVERTER_REVISION, find_blender};
use super::manifest::{
    CURRENT_BAKE_REVISION, CURRENT_MANIFEST_SCHEMA_VERSION, PreparedCellLighting,
    PreparedIrradianceVolume, PreparedPhysicsClassification, PreparedPlacement,
    PreparedSceneManifest, PreparedSemantic, cell_label, ensure_prepared_manifest_compatible,
};
use super::physics::PHYSICS_ASSET_SCHEMA_VERSION;
use super::scenes::resolve_cached_manifest;

pub fn bake(args: BakeArgs) -> Result<()> {
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
    let text = fs::read_to_string(&manifest_path).context("could not read scene manifest")?;
    let mut manifest: PreparedSceneManifest =
        ron::de::from_str(&text).context("invalid scene manifest; run prepare before bake")?;
    ensure_prepared_manifest_compatible(
        &manifest,
        NIF_CONVERTER_REVISION,
        PHYSICS_ASSET_SCHEMA_VERSION,
    )?;
    if manifest
        .placements
        .iter()
        .all(|placement| placement.asset_path.is_none())
    {
        bail!("scene manifest contains no renderable placements");
    }

    let blender = if matches!(args.quality, BakeQuality::Preview) {
        Some(find_blender(args.blender.clone())?)
    } else {
        None
    };
    let ktx_tool = if matches!(args.quality, BakeQuality::Irradiance) {
        Some(find_irradiance_ktx_tool(args.toktx)?)
    } else {
        None
    };
    let asset_root = fs::canonicalize(&manifest.asset_root)
        .with_context(|| format!("asset root does not exist: {}", manifest.asset_root))?;
    let cell_dir = asset_root
        .join("scenes")
        .join(format!("{:08x}", manifest.cell.form_id));
    let output_dir = cell_dir.join("baked");
    // Calling bake is itself the user's request to regenerate this output.
    // Keep reading the legacy --force flag for existing scripts, but do not
    // require a second confirmation before replacing known bake artifacts.
    let _legacy_force = args.force;
    fs::create_dir_all(&output_dir)?;

    let output_scene = output_dir.join("scene.glb");
    let preview_output = output_dir.join("preview.png");
    let irradiance_raw = output_dir.join("irradiance.raw");
    let job_file = output_dir.join("job.json");
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
    let job = BakeJob {
        asset_root: blender_path(&asset_root),
        output_scene: blender_path(&output_scene),
        preview_output: blender_path(&preview_output),
        result_json: blender_path(&output_dir.join("result.json")),
        irradiance_blend: blender_path(&output_dir.join("irradiance.blend")),
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
    };
    if matches!(args.quality, BakeQuality::Preview) {
        fs::write(&job_file, serde_json::to_vec_pretty(&job)?)?;
        let script_file = output_dir.join("blender_bake.py");
        fs::write(&script_file, include_str!("../blender_bake.py"))?;
        let _ = fs::remove_file(&preview_output);
        let blender_status = Command::new(blender.expect("preview resolves Blender"))
            .arg("--background")
            .arg("--factory-startup")
            .arg("--python")
            .arg(&script_file)
            .arg("--")
            .arg(&job_file)
            .current_dir(&asset_root)
            .status()
            .context("failed to start Blender")?;
        if !blender_status.success() {
            bail!("Blender preview failed with {blender_status}");
        }
        if !preview_output.exists() {
            bail!(
                "Blender reported success but did not create the preview image {}",
                preview_output.display()
            );
        }
        if !args.keep_intermediate {
            let _ = fs::remove_file(&job_file);
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
        rust_scene::compose_scene(&asset_root, &job.placements, args.static_batch_chunk_meters)?;
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
        &irradiance_raw,
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
    replace_output(&temporary_scene, &output_scene)?;
    replace_output(&temporary_ktx, &ktx2_path)?;
    let scene_path = relative_asset_path(&asset_root, &output_scene)?;
    let irradiance_path = relative_asset_path(&asset_root, &ktx2_path)?;
    let mut fingerprint = Sha256::new();
    fingerprint.update(manifest.source_fingerprint.as_bytes());
    fingerprint.update(serde_json::to_vec(&job)?);
    let source_fingerprint = format!("{:x}", fingerprint.finalize());
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
        &manifest_path,
        to_string_pretty(&manifest, PrettyConfig::default())?,
    )?;
    if !args.keep_intermediate {
        for raw_slice in &irradiance.raw_slices {
            let _ = fs::remove_file(raw_slice);
        }
        let _ = fs::remove_file(&irradiance_raw);
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
}

fn is_batchable_static(placement: &PreparedPlacement) -> bool {
    is_bake_static(placement)
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
