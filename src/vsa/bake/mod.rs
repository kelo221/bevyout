use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

mod backend;
mod batch;
mod cache;
mod denoise;
mod environment;
mod gltf_extension_policy;
mod job;
mod ktx2;
mod lightmap;
mod lightmap_uv;
mod plan;
mod policy;
pub(crate) mod rust_irradiance;
pub(crate) mod rust_scene;
mod tools;
mod transport;

pub(crate) use backend::validate_backend;
pub(crate) use batch::*;
pub(crate) use job::*;
pub(crate) use plan::*;
pub(crate) use tools::*;

use crate::cli::progress::ProgressReporter;
use crate::cli::{BakeArgs, LightmapBackendPreference};
use environment::EnvironmentMap;

use super::assets::SUPPORTED_PREPARED_CONVERTER_REVISIONS;
use super::manifest::{
    CURRENT_BAKE_REVISION, CURRENT_MANIFEST_SCHEMA_VERSION, PreparedBakeSettings,
    PreparedCellLighting, PreparedIrradianceVolume, PreparedLightmapAtlas, PreparedLightmapBinding,
    PreparedLightmapFormat, PreparedLightmapVarianceFormat, PreparedLightmapVariancePage,
    PreparedPhysicsClassification, PreparedPlacement, PreparedRuntimeMutability,
    PreparedSceneManifest, PreparedSemantic, cell_label, ensure_prepared_manifest_compatible_any,
    is_pickup_record_kind,
};
use super::physics::PHYSICS_ASSET_SCHEMA_VERSION;
use super::scenes::resolve_cached_manifest;

pub fn bake(args: BakeArgs) -> Result<()> {
    let progress = ProgressReporter::new(args.progress.mode);
    validate_backend(args.lightmap_backend)?;
    if args.lightmap_min_samples > args.lightmap_max_samples {
        bail!(
            "lightmap minimum samples {} exceeds maximum {}",
            args.lightmap_min_samples,
            args.lightmap_max_samples
        );
    }
    validate_lightmap_density_overrides(&args.lightmap_density_overrides)?;
    // Batch mode (issue #62): `--all-interiors`/`--retry-failed` walk the
    // prepared cell catalogue through the resumable bake job manifest; a
    // single selector/--manifest keeps the original single-cell path, which
    // batch mode reuses per cell via `bake_manifest`.
    if args.all_interiors || args.retry_failed {
        return bake_batch(args, &progress);
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
    progress.started(bake_operation_label(args.lightmap_backend), None);
    let result = bake_manifest(&args, &manifest_path, &progress);
    progress.finished(result.is_ok());
    result
}

fn bake_operation_label(backend: LightmapBackendPreference) -> &'static str {
    match backend {
        LightmapBackendPreference::Solari => "Solari bake",
        LightmapBackendPreference::Auto | LightmapBackendPreference::Cpu => "CPU bake",
    }
}

pub(crate) fn validate_lightmap_density_overrides(
    overrides: &[crate::cli::LightmapDensityOverrideArg],
) -> Result<()> {
    let mut form_ids = BTreeSet::new();
    for density_override in overrides {
        if !form_ids.insert(density_override.reference_form_id) {
            bail!(
                "duplicate lightmap density override for FormID {:08x}",
                density_override.reference_form_id
            );
        }
    }
    Ok(())
}

fn validate_lightmap_density_override_targets(
    manifest: &PreparedSceneManifest,
    overrides: &[crate::cli::LightmapDensityOverrideArg],
) -> Result<()> {
    let reference_form_ids = manifest
        .placements
        .iter()
        .map(|placement| placement.reference_form_id)
        .collect::<BTreeSet<_>>();
    for density_override in overrides {
        if !reference_form_ids.contains(&density_override.reference_form_id) {
            bail!(
                "lightmap density override targets unknown FormID {:08x}",
                density_override.reference_form_id
            );
        }
    }
    Ok(())
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
        irradiance_raw: output_dir.join("irradiance.raw"),
        job_file: output_dir.join("job.json"),
        asset_root,
        output_dir,
    })
}

/// Assembles the deterministic Rust bake job for one prepared cell. Pure over its inputs:
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
    let lightmap_texels_per_meter = args
        .lightmap_texels_per_meter
        .unwrap_or_else(|| default_lightmap_texels_per_meter(args.lightmap_backend));
    let lightmap_tile_size = args
        .lightmap_tile_size
        .unwrap_or_else(|| default_lightmap_tile_size(args.lightmap_backend));
    let static_batch_chunk_meters = args
        .static_batch_chunk_meters
        .unwrap_or_else(|| default_static_batch_chunk_meters(args.lightmap_backend));
    BakeJob {
        asset_root: job_path(&outputs.asset_root),
        output_scene: job_path(&outputs.output_scene),
        irradiance_spacing_meters: args.irradiance_spacing_meters,
        irradiance_samples: args.irradiance_samples,
        lightmap_min_samples: args.lightmap_min_samples,
        lightmap_max_samples: args.lightmap_max_samples,
        lightmap_variance_threshold: args.lightmap_variance_threshold,
        lightmap_bounces: args.lightmap_bounces,
        lightmap_texels_per_meter,
        lightmap_density_overrides: args
            .lightmap_density_overrides
            .iter()
            .map(|override_value| {
                (
                    override_value.reference_form_id,
                    override_value.texels_per_meter,
                )
            })
            .collect::<BTreeMap<_, _>>(),
        lightmap_denoise_iterations: args.lightmap_denoise_iterations,
        lightmap_tile_size,
        lightmap_backend: args.lightmap_backend.as_str().into(),
        static_batch_chunk_meters,
        ambient_rgba: cell_lighting.ambient_rgba,
        lightmap_environment_map: args.lightmap_environment_map.as_ref().map(|path| {
            let resolved = if path.is_absolute() {
                path.clone()
            } else {
                outputs.asset_root.join(path)
            };
            job_path(&resolved)
        }),
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
                intensity_lumens: authored_point_light_intensity(
                    light.radius,
                    light.intensity_lumens,
                ),
                kind: if light.kind.is_empty() {
                    "point".to_owned()
                } else {
                    light.kind.clone()
                },
                flags: light.flags,
                spot_fov_radians: light.spot_fov_radians,
                spot_falloff_exponent: light.spot_falloff_exponent,
            })
            .collect(),
    }
}

fn default_lightmap_texels_per_meter(backend: LightmapBackendPreference) -> f32 {
    match backend {
        LightmapBackendPreference::Solari => 4.0,
        LightmapBackendPreference::Auto | LightmapBackendPreference::Cpu => 16.0,
    }
}

fn default_lightmap_tile_size(backend: LightmapBackendPreference) -> u32 {
    match backend {
        LightmapBackendPreference::Solari => 512,
        LightmapBackendPreference::Auto | LightmapBackendPreference::Cpu => 128,
    }
}

fn default_static_batch_chunk_meters(backend: LightmapBackendPreference) -> f32 {
    match backend {
        LightmapBackendPreference::Solari => 32.0,
        LightmapBackendPreference::Auto | LightmapBackendPreference::Cpu => 64.0,
    }
}

/// The job-parameter fingerprint recorded as `PreparedBake.source_fingerprint`
/// after a successful bake, and recomputed by the batch skip check to decide
/// whether a recorded bake is still valid (F62.1): the prepared manifest's
/// own `source_fingerprint` plus the serialized Rust bake job, so both a
/// re-prepared cell and changed bake parameters invalidate a recorded bake.
pub(crate) fn bake_job_fingerprint(
    manifest: &PreparedSceneManifest,
    job: &BakeJob,
) -> Result<String> {
    let mut fingerprint = Sha256::new();
    fingerprint.update(manifest.source_fingerprint.as_bytes());
    fingerprint.update(serde_json::to_vec(job)?);
    update_asset_fingerprints(&mut fingerprint, job)?;
    Ok(format!("{:x}", fingerprint.finalize()))
}

/// Base fingerprint for raw transport accumulation tiles. Denoising and the
/// complete light list are deliberately excluded: denoising is a later stage,
/// while each primitive adds only the point/spot lights whose ranges intersect
/// its bounds to its tile fingerprint. This lets an unrelated light change
/// preserve unaffected primitive pages.
pub(crate) fn lightmap_accumulation_fingerprint(
    manifest: &PreparedSceneManifest,
    job: &BakeJob,
) -> Result<String> {
    let mut job_value = serde_json::to_value(job)?;
    if let Some(object) = job_value.as_object_mut() {
        object.remove("lightmap_denoise_iterations");
        object.remove("lights");
    }
    let mut fingerprint = Sha256::new();
    fingerprint.update(b"lightmap-accumulation-v2");
    let transport_revision = CURRENT_BAKE_REVISION.replace("-feature-guided-atrous-denoise-v1", "");
    fingerprint.update(transport_revision.as_bytes());
    if job.lightmap_backend.eq_ignore_ascii_case("solari") {
        fingerprint.update(b"solari-bevy-0.19-bounded-transport");
        fingerprint.update(Sha256::digest(include_bytes!("backend/solari_bake.wgsl")));
    } else {
        fingerprint.update(b"transport-cpu");
    }
    fingerprint.update(manifest.source_fingerprint.as_bytes());
    fingerprint.update(serde_json::to_vec(&job_value)?);
    update_asset_fingerprints(&mut fingerprint, job)?;
    Ok(format!("{:x}", fingerprint.finalize()))
}

fn integrator_revision_for_job(job: &BakeJob) -> String {
    if job.lightmap_backend.eq_ignore_ascii_case("solari") {
        let shader_revision = format!(
            "{:x}",
            Sha256::digest(include_bytes!("backend/solari_bake.wgsl"))
        );
        format!(
            "solari-bevy-0.19-ray-query-bounded-cosine-hit-emission-env-direct-only-{}-min{}-max{}-variance-{:.6}-shader-{}",
            job.lightmap_bounces,
            job.lightmap_min_samples,
            job.lightmap_max_samples,
            job.lightmap_variance_threshold,
            shader_revision,
        )
    } else {
        format!(
            "transport-cpu-direct-surface-v7-area-emissive-mis-rr-adaptive-bounces-{}-min{}-max{}-variance-{:.6}-environment-map-hdr-v1",
            job.lightmap_bounces,
            job.lightmap_min_samples,
            job.lightmap_max_samples,
            job.lightmap_variance_threshold
        )
    }
}

fn update_asset_fingerprints(fingerprint: &mut Sha256, job: &BakeJob) -> Result<()> {
    let asset_paths = job
        .placements
        .iter()
        .map(|placement| placement.asset_path.as_str())
        .collect::<BTreeSet<_>>();
    fingerprint.update(b"lightmap-assets-v1");
    for asset_path in asset_paths {
        let path =
            Path::new(&job.asset_root).join(asset_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let bytes = fs::read(&path).with_context(|| {
            format!(
                "could not read bake asset for fingerprint {}",
                path.display()
            )
        })?;
        fingerprint.update(asset_path.as_bytes());
        fingerprint.update(Sha256::digest(bytes));
    }
    if let Some(environment_map_path) = &job.lightmap_environment_map {
        let path = Path::new(environment_map_path);
        let bytes = fs::read(path).with_context(|| {
            format!(
                "could not read lightmap environment map for fingerprint {}",
                path.display()
            )
        })?;
        fingerprint.update(b"lightmap-environment-map-v1");
        fingerprint.update(environment_map_path.as_bytes());
        fingerprint.update(Sha256::digest(bytes));
    }
    Ok(())
}

/// Bakes one prepared scene manifest in place: the whole original
/// single-cell `bake` path after selector resolution, reused verbatim by
/// each batch cell.
pub(crate) fn bake_manifest(
    args: &BakeArgs,
    manifest_path: &Path,
    progress: &ProgressReporter,
) -> Result<()> {
    let mut manifest = load_prepared_manifest(manifest_path)?;
    progress.message(format!("cell {}", cell_label(&manifest.cell)));
    validate_lightmap_density_override_targets(&manifest, &args.lightmap_density_overrides)?;

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
        irradiance_raw,
        job_file,
    } = &outputs;
    let mut job = build_bake_job(&manifest, args, &outputs);
    exclude_animated_static_assets(asset_root, &mut job)?;
    let environment_map = job
        .lightmap_environment_map
        .as_deref()
        .map(|path| EnvironmentMap::load(Path::new(path)))
        .transpose()?;
    fs::write(job_file, serde_json::to_vec_pretty(&job)?)?;

    if output_scene.exists() {
        fs::remove_file(output_scene)?;
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
    progress.phase_started("scene composition", None);
    let mut rust_scene = rust_scene::compose_scene_with_lightmap_density(
        asset_root,
        &job.placements,
        job.static_batch_chunk_meters,
        job.lightmap_texels_per_meter,
        &job.lightmap_density_overrides,
    )?;
    let composed_elapsed = started.elapsed();
    println!(
        "Rust bake: composed {} primitives in {:.2}s",
        rust_scene.primitives.len(),
        composed_elapsed.as_secs_f64()
    );
    lightmap::validate_page_dimensions(&rust_scene, lightmap::LIGHTMAP_ATLAS_PAGE_SIZE)?;
    let lightmap_raw_dir = output_dir.join("lightmaps");
    let accumulation_fingerprint = lightmap_accumulation_fingerprint(&manifest, &job)?;
    let mut lightmap_cache = cache::TileCache::open(
        &output_dir.join("lightmap-accumulation"),
        &accumulation_fingerprint,
        args.lightmap_force_retrace,
    )?;
    let directional = rust_irradiance::DirectionalBakeLight {
        color_rgba: job.cell_directional_rgba,
        rotation_xyzw: job.cell_directional_rotation_xyzw,
        illuminance: job.cell_directional_illuminance,
    };
    let sampling = lightmap::LightmapSamplingSettings {
        min_samples: job.lightmap_min_samples,
        max_samples: job.lightmap_max_samples,
        variance_threshold: job.lightmap_variance_threshold,
    };
    let denoise = lightmap::LightmapDenoiseSettings {
        iterations: job.lightmap_denoise_iterations,
    };
    let debug = lightmap::LightmapDebugSettings {
        uv: args.lightmap_debug_uv,
        samples: args.lightmap_debug_samples,
        variance: args.lightmap_debug_variance,
    };
    let lightmap_bake = match args.lightmap_backend {
        LightmapBackendPreference::Solari => {
            progress.set_backend("Solari");
            progress.set_sampling(Some(job.lightmap_max_samples), Some(job.lightmap_bounces));
            progress.phase_started("lightmap tile dispatch/readback", None);
            #[cfg(feature = "lightmap-gpu-solari")]
            {
                lightmap::bake_direct_pages_solari_bounded(
                    &rust_scene,
                    &job.lights,
                    &directional,
                    job.ambient_rgba,
                    environment_map.as_ref(),
                    transport::sampling::seed_from_fingerprint(&manifest.source_fingerprint),
                    &lightmap_raw_dir,
                    sampling,
                    denoise,
                    job.lightmap_bounces,
                    job.lightmap_tile_size,
                    &accumulation_fingerprint,
                    debug,
                    &mut lightmap_cache,
                    Some(progress),
                )?
            }
            #[cfg(not(feature = "lightmap-gpu-solari"))]
            {
                bail!(
                    "lightmap Solari backend is not available in this build; use --bake-backend cpu"
                )
            }
        }
        LightmapBackendPreference::Auto | LightmapBackendPreference::Cpu => {
            progress.set_backend("CPU");
            progress.set_sampling(Some(job.lightmap_max_samples), Some(job.lightmap_bounces));
            progress.phase_started("lightmap primitive/tile transport", None);
            lightmap::bake_direct_pages(
                &rust_scene,
                &job.lights,
                &directional,
                job.ambient_rgba,
                environment_map.as_ref(),
                transport::sampling::seed_from_fingerprint(&manifest.source_fingerprint),
                &lightmap_raw_dir,
                sampling,
                denoise,
                job.lightmap_bounces,
                job.lightmap_tile_size,
                &accumulation_fingerprint,
                debug,
                &mut lightmap_cache,
                Some(progress),
            )?
        }
    };
    let lightmap_sampling = lightmap_bake.sampling;
    let lightmap_pages = lightmap_bake.pages;
    progress.phase_started("UV/page packing", Some(lightmap_pages.len() as u64));
    let (lightmap_pages, lightmap_atlases) = lightmap::pack_lightmap_pages(
        lightmap_pages,
        &lightmap_raw_dir,
        lightmap::LIGHTMAP_ATLAS_PAGE_SIZE,
    )?;
    for _ in &lightmap_pages {
        progress.unit_completed(Some(lightmap_pages.len() as u64), None);
    }
    println!(
        "surface lightmaps: packed {} primitive pages into {} atlases; adaptive samples {}..{} mean {:.2} max relative variance {:.6}",
        lightmap_pages.len(),
        lightmap_atlases.len(),
        lightmap_sampling.min_samples,
        lightmap_sampling.max_samples,
        if lightmap_sampling.sampled_texels == 0 {
            0.0
        } else {
            lightmap_sampling.total_samples as f64 / lightmap_sampling.sampled_texels as f64
        },
        lightmap_sampling.max_relative_variance,
    );
    let cache_stats = lightmap_cache.stats();
    println!(
        "surface lightmap cache: {} tile hits, {} misses, {} writes",
        cache_stats.hits, cache_stats.misses, cache_stats.writes
    );
    rust_scene.write_glb(&temporary_scene)?;
    progress.phase_started("irradiance-volume probes", None);
    let irradiance = rust_irradiance::bake_irradiance(
        &rust_scene,
        &job.lights,
        &rust_irradiance::DirectionalBakeLight {
            color_rgba: job.cell_directional_rgba,
            rotation_xyzw: job.cell_directional_rotation_xyzw,
            illuminance: job.cell_directional_illuminance,
        },
        job.ambient_rgba,
        environment_map.as_ref(),
        transport::sampling::seed_from_fingerprint(&manifest.source_fingerprint),
        args.irradiance_spacing_meters,
        args.irradiance_samples,
        irradiance_raw,
    )?;
    ktx2::write_rgb9e5_volume(
        &irradiance.raw_slices,
        &temporary_ktx,
        irradiance.resolution[0],
        2 * irradiance.resolution[1],
        3 * irradiance.resolution[2],
    )?;
    let ktx_validation = ::ktx2::Reader::new(
        fs::read(&temporary_ktx).context("failed to read in-tree irradiance KTX2")?,
    )
    .map_err(|error| {
        anyhow::anyhow!(
            "in-tree irradiance KTX2 validation failed for {}: {error}",
            temporary_ktx.display()
        )
    })?;
    if ktx_validation.header().format != Some(::ktx2::Format::E5B9G9R9_UFLOAT_PACK32)
        || ktx_validation.header().pixel_width != irradiance.resolution[0]
        || ktx_validation.header().pixel_height != 2 * irradiance.resolution[1]
        || ktx_validation.header().pixel_depth != 3 * irradiance.resolution[2]
        || ktx_validation.header().level_count != 1
        || ktx_validation.header().supercompression_scheme.is_some()
    {
        bail!(
            "in-tree irradiance KTX2 has unexpected metadata for {}",
            temporary_ktx.display()
        );
    }
    let mut lightmaps = Vec::with_capacity(lightmap_atlases.len());
    progress.phase_started("atlas encoding", Some(lightmap_atlases.len() as u64));
    let covered_texels = lightmap_pages
        .iter()
        .map(|page| page.covered_texels)
        .sum::<usize>();
    for atlas in &lightmap_atlases {
        let output = atlas.raw_path.with_extension("ktx2");
        let temporary = atlas.raw_path.with_file_name(format!(
            "{}.rust.tmp.ktx2",
            atlas
                .raw_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("lightmap")
        ));
        let _ = fs::remove_file(&temporary);
        let _ = fs::remove_file(&output);
        ktx2::write_rgba16f(&atlas.raw_path, &temporary, atlas.width, atlas.height)?;
        let validation = ::ktx2::Reader::new(
            fs::read(&temporary).context("failed to read in-tree surface lightmap KTX2")?,
        )
        .map_err(|error| {
            anyhow::anyhow!(
                "surface lightmap atlas KTX validation failed for {}: {error}",
                temporary.display()
            )
        })?;
        if validation.header().format != Some(::ktx2::Format::R16G16B16A16_SFLOAT)
            || validation.header().pixel_width != atlas.width
            || validation.header().pixel_height != atlas.height
            || validation.header().level_count != 1
            || validation.header().supercompression_scheme.is_some()
        {
            bail!(
                "in-tree surface lightmap KTX2 has unexpected metadata for {}",
                temporary.display()
            );
        }
        replace_output(&temporary, &output)?;
        progress.unit_completed(Some(lightmap_atlases.len() as u64), None);
        lightmaps.push(PreparedLightmapAtlas {
            asset_path: relative_asset_path(asset_root, &output)?,
            width: atlas.width,
            height: atlas.height,
            format: PreparedLightmapFormat::Rgba16Float,
            content_hash: atlas.content_hash.clone(),
        });
    }
    let lightmap_variance_pages = lightmap_pages
        .iter()
        .map(|page| {
            let primitive = rust_scene
                .primitives
                .get(page.primitive_index)
                .context("surface lightmap variance page references missing primitive")?;
            let output = lightmap_raw_dir.join(format!(
                "lightmap-variance-{:04}.r32f.raw",
                page.primitive_index
            ));
            let bytes = fs::read(&output).with_context(|| {
                format!(
                    "could not read surface lightmap variance page {}",
                    output.display()
                )
            })?;
            let expected = (page.width as usize)
                .checked_mul(page.height as usize)
                .and_then(|texels| texels.checked_mul(std::mem::size_of::<f32>()))
                .context("lightmap variance page byte size overflowed")?;
            if bytes.len() != expected {
                bail!(
                    "lightmap variance page {} has {} bytes, expected {}",
                    page.primitive_index,
                    bytes.len(),
                    expected
                );
            }
            Ok(PreparedLightmapVariancePage {
                primitive_key: primitive.primitive_key.clone(),
                asset_path: relative_asset_path(asset_root, &output)?,
                width: page.width,
                height: page.height,
                format: PreparedLightmapVarianceFormat::R32FloatRaw,
                content_hash: format!("{:x}", Sha256::digest(&bytes)),
                covered_texels: page
                    .covered_texels
                    .try_into()
                    .context("lightmap variance covered texel count exceeds u32")?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    if !args.keep_intermediate {
        progress.phase_started("surface-lightmap intermediate cleanup", None);
        for page in &lightmap_pages {
            let _ = fs::remove_file(&page.raw_path);
        }
        for atlas in &lightmap_atlases {
            let _ = fs::remove_file(&atlas.raw_path);
        }
    }
    let dilated_texels = lightmap_pages
        .iter()
        .map(|page| page.dilated_texels)
        .sum::<usize>();
    println!(
        "surface lightmaps: encoded {} atlases, {} covered texels, {} dilated texels",
        lightmaps.len(),
        covered_texels,
        dilated_texels
    );
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
    println!(
        "local thickness: generated {} {}x{} UV maps",
        batching.translucency_maps,
        batching.translucency_resolution,
        batching.translucency_resolution,
    );
    replace_output(&temporary_scene, output_scene)?;
    replace_output(&temporary_ktx, &ktx2_path)?;
    progress.phase_started("manifest publication", None);
    let scene_path = relative_asset_path(asset_root, output_scene)?;
    let irradiance_path = relative_asset_path(asset_root, &ktx2_path)?;
    let source_fingerprint = bake_job_fingerprint(&manifest, &job)?;
    let lightmap_bindings =
        build_lightmap_bindings(&rust_scene, &lightmap_pages, &lightmap_atlases)?;
    manifest.schema_version = CURRENT_MANIFEST_SCHEMA_VERSION;
    manifest.bake = Some(super::manifest::PreparedBake {
        bake_revision: Some(CURRENT_BAKE_REVISION.into()),
        source_fingerprint,
        scene_path,
        lightmaps,
        lightmap_variance_pages,
        lightmap_bindings,
        bake_settings: PreparedBakeSettings {
            integrator_revision: integrator_revision_for_job(&job),
            xatlas_revision: "vendored-xatlas-rs-v2-source-v1".into(),
            uv_layout_fingerprint: CURRENT_BAKE_REVISION.into(),
            material_fingerprint: CURRENT_BAKE_REVISION.into(),
            light_fingerprint: CURRENT_BAKE_REVISION.into(),
            sample_count: job.lightmap_max_samples,
            bounce_count: job.lightmap_bounces,
            denoiser_revision: format!(
                "feature-guided-atrous-v1-iterations-{}",
                job.lightmap_denoise_iterations
            ),
            encoder_revision: "ktx2-rgba16f-rgb9e5-uncompressed-v1".into(),
        },
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
        progress.phase_started("irradiance intermediate cleanup", None);
        for raw_slice in &irradiance.raw_slices {
            let _ = fs::remove_file(raw_slice);
        }
        let _ = fs::remove_file(irradiance_raw);
        let _ = fs::remove_file(job_file);
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

fn build_lightmap_bindings(
    rust_scene: &rust_scene::RustBakeScene,
    lightmap_pages: &[lightmap::LightmapPage],
    lightmap_atlases: &[lightmap::LightmapAtlas],
) -> Result<Vec<PreparedLightmapBinding>> {
    rust_scene
        .primitives
        .iter()
        .enumerate()
        .map(|(primitive_index, primitive)| {
            let page = lightmap_pages
                .get(primitive_index)
                .context("surface lightmap page order does not match composed primitives")?;
            if page.primitive_index != primitive_index {
                bail!(
                    "surface lightmap page {} is bound to primitive {}",
                    primitive_index,
                    page.primitive_index
                );
            }
            let atlas = lightmap_atlases
                .get(page.atlas_index)
                .context("surface lightmap page references missing atlas")?;
            let [offset_x, offset_y] = page.atlas_offset;
            let uv_rect = [
                offset_x as f32 / atlas.width as f32,
                offset_y as f32 / atlas.height as f32,
                (offset_x + page.width) as f32 / atlas.width as f32,
                (offset_y + page.height) as f32 / atlas.height as f32,
            ];
            Ok(PreparedLightmapBinding {
                binding_id: primitive
                    .lightmap_binding_id
                    .context("lightmap primitive has no generated binding ID")?,
                primitive_key: primitive.primitive_key.clone(),
                atlas_index: page
                    .atlas_index
                    .try_into()
                    .context("surface lightmap atlas index exceeds u16")?,
                uv_rect,
                texels_per_meter: primitive.lightmap_texels_per_meter,
            })
        })
        .collect()
}

pub(crate) fn is_bake_static(placement: &PreparedPlacement) -> bool {
    placement.physics_classification == PreparedPhysicsClassification::Static
        && placement.mutability == PreparedRuntimeMutability::Immutable
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
