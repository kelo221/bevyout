use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli::{BakeArgs, BakeQuality};

use super::assets::{NIF_CONVERTER_REVISION, find_blender};
use super::irradiance::export_rgb9e5_atlas;
use super::manifest::{
    CURRENT_BAKE_REVISION, CURRENT_MANIFEST_SCHEMA_VERSION, PreparedCellLighting,
    PreparedIrradianceVolume, PreparedPhysicsClassification, PreparedPlacement,
    PreparedSceneManifest, PreparedSemantic, cell_label, ensure_prepared_manifest_compatible,
};
use super::physics::PHYSICS_ASSET_SCHEMA_VERSION;
use super::scenes::resolve_cached_manifest;

const CELL_DIRECTIONAL_ILLUMINANCE: f32 = 10_000.0;

fn cell_directional_illuminance(lighting: &PreparedCellLighting) -> f32 {
    let luminance =
        lighting.directional_rgba[0] + lighting.directional_rgba[1] + lighting.directional_rgba[2];
    if !luminance.is_finite() || luminance <= f32::EPSILON {
        0.0
    } else {
        CELL_DIRECTIONAL_ILLUMINANCE
    }
}

#[derive(Debug, Serialize)]
struct BakeJob {
    asset_root: String,
    output_scene: String,
    preview_output: String,
    result_json: String,
    irradiance_blend: String,
    irradiance_spacing_meters: f32,
    irradiance_samples: u32,
    preview_only: bool,
    static_batch_chunk_meters: f32,
    emission_scale: f32,
    ambient_rgba: [f32; 4],
    cell_directional_rgba: [f32; 4],
    cell_directional_rotation_xyzw: [f32; 4],
    cell_directional_illuminance: f32,
    placements: Vec<JobPlacement>,
    lights: Vec<JobLight>,
}

#[derive(Debug, Serialize)]
struct JobPlacement {
    reference_form_id: u32,
    asset_path: String,
    ao_mode: String,
    batchable_static: bool,
    translation: [f32; 3],
    rotation_xyzw: [f32; 4],
    scale: f32,
}

#[derive(Debug, Serialize)]
struct JobLight {
    translation: [f32; 3],
    rotation_xyzw: [f32; 4],
    color_rgba: [f32; 4],
    radius: f32,
    intensity_lumens: f32,
    kind: String,
}

#[derive(Debug, Deserialize)]
struct BlenderBakeResult {
    irradiance: Option<BlenderIrradianceResult>,
    batching: BlenderBatchingStats,
}

#[derive(Debug, Deserialize)]
struct BlenderIrradianceResult {
    blend_path: String,
    resolution: [u32; 3],
    translation: [f32; 3],
    rotation_xyzw: [f32; 4],
    scale: [f32; 3],
}

#[derive(Debug, Deserialize)]
struct BlenderBatchingStats {
    chunk_size_meters: f32,
    visual_objects_before: usize,
    visual_objects_after: usize,
    render_primitives_before: usize,
    render_primitives_after: usize,
    materials_before: usize,
    materials_after: usize,
    batches_created: usize,
    largest_batch: usize,
    excluded_collision: usize,
    excluded_large: usize,
    excluded_non_static: usize,
}

#[derive(Debug, Clone, Copy)]
enum KtxToolKind {
    LegacyToktx,
    UnifiedKtx,
}

#[derive(Debug, Clone)]
struct KtxTool {
    path: PathBuf,
    kind: KtxToolKind,
}

pub(crate) fn bake(args: BakeArgs) -> Result<()> {
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

    let blender = if matches!(args.quality, BakeQuality::Irradiance) {
        find_irradiance_blender(args.irradiance_blender.as_deref(), args.blender.as_deref())?
    } else {
        find_blender(args.blender.clone())?
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
    let result_json = output_dir.join("result.json");
    let irradiance_blend = output_dir.join("irradiance.blend");
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
        result_json: blender_path(&result_json),
        irradiance_blend: blender_path(&irradiance_blend),
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
    fs::write(&job_file, serde_json::to_vec_pretty(&job)?)?;

    let script_file = output_dir.join("blender_bake.py");
    fs::write(&script_file, include_str!("blender_bake.py"))?;
    if matches!(args.quality, BakeQuality::Preview) {
        let _ = fs::remove_file(&preview_output);
    } else {
        for path in [&output_scene, &irradiance_blend, &result_json] {
            let _ = fs::remove_file(path);
        }
    }
    let blender_status = Command::new(&blender)
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
        bail!("Blender bake failed with {blender_status}");
    }
    if matches!(args.quality, BakeQuality::Preview) {
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
    if !output_scene.exists() || !irradiance_blend.exists() || !result_json.exists() {
        bail!(
            "Blender reported success but did not create the expected bake outputs in {}",
            output_dir.display()
        );
    }

    let ktx_tool = ktx_tool.expect("irradiance bake always resolves a KTX tool");
    let bake_result: BlenderBakeResult =
        serde_json::from_slice(&fs::read(&result_json)?).context("invalid Blender bake result")?;
    let irradiance = bake_result
        .irradiance
        .as_ref()
        .context("Blender produced no irradiance volume metadata")?;
    let exported = export_rgb9e5_atlas(
        Path::new(&irradiance.blend_path),
        &irradiance_raw,
        irradiance.resolution,
    )?;
    let ktx2_path = output_dir.join("irradiance.ktx2");
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
                .arg(exported.resolution[0].to_string())
                .arg("--height")
                .arg((2 * exported.resolution[1]).to_string())
                .arg("--depth")
                .arg((3 * exported.resolution[2]).to_string())
                .arg("--assign-tf")
                .arg("linear")
                .arg("--assign-texcoord-origin")
                .arg("top-left-front")
                .arg("--zstd")
                .arg("3");
        }
    }
    for raw_slice in &exported.raw_slices {
        ktx_command.arg(raw_slice);
    }
    let ktx_output = ktx_command
        .arg(&ktx2_path)
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

    let batching = &bake_result.batching;
    println!(
        "static batching ({:.1} m chunks): objects {} -> {}, primitives {} -> {}, materials {} -> {}; {} batches (largest {}), excluded {} collision / {} large / {} non-static",
        batching.chunk_size_meters,
        batching.visual_objects_before,
        batching.visual_objects_after,
        batching.render_primitives_before,
        batching.render_primitives_after,
        batching.materials_before,
        batching.materials_after,
        batching.batches_created,
        batching.largest_batch,
        batching.excluded_collision,
        batching.excluded_large,
        batching.excluded_non_static,
    );
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
            resolution: exported.resolution,
            intensity: 1.0,
        }),
    });
    fs::write(
        &manifest_path,
        to_string_pretty(&manifest, PrettyConfig::default())?,
    )?;
    if !args.keep_intermediate {
        let _ = fs::remove_file(&job_file);
        let _ = fs::remove_file(&script_file);
        let _ = fs::remove_file(&result_json);
        for raw_slice in &exported.raw_slices {
            let _ = fs::remove_file(raw_slice);
        }
        let _ = fs::remove_file(&irradiance_raw);
        let _ = fs::remove_file(&irradiance_blend);
    }
    println!(
        "baked {}: irradiance volume {:?} -> {}",
        cell_label(&manifest.cell),
        exported.resolution,
        ktx2_path.display()
    );
    Ok(())
}

pub(crate) fn is_bake_static(placement: &PreparedPlacement) -> bool {
    if placement.physics_classification == PreparedPhysicsClassification::Dynamic {
        return false;
    }
    !matches!(
        placement.semantic,
        PreparedSemantic::Pickup(_)
            | PreparedSemantic::Container
            | PreparedSemantic::Door(_)
            | PreparedSemantic::Activator
    )
}

fn is_batchable_static(placement: &PreparedPlacement) -> bool {
    placement.physics_classification != PreparedPhysicsClassification::Dynamic
        && matches!(placement.semantic, PreparedSemantic::Static)
}

fn find_ktx_tool(explicit: Option<PathBuf>) -> Result<Option<KtxTool>> {
    if let Some(path) = explicit {
        if path.exists() {
            return Ok(Some(KtxTool {
                kind: ktx_tool_kind(&path),
                path,
            }));
        }
        bail!("KTX executable does not exist: {}", path.display());
    }
    for (command, kind) in [
        ("toktx", KtxToolKind::LegacyToktx),
        ("ktx", KtxToolKind::UnifiedKtx),
    ] {
        if Command::new(command)
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success())
        {
            return Ok(Some(KtxTool {
                path: PathBuf::from(command),
                kind,
            }));
        }
    }
    for path in [
        PathBuf::from(r"C:\Program Files\KTX-Software\bin\toktx.exe"),
        PathBuf::from(r"C:\Program Files\KTX-Software\bin\ktx.exe"),
    ] {
        if path.exists() {
            return Ok(Some(KtxTool {
                kind: ktx_tool_kind(&path),
                path,
            }));
        }
    }
    Ok(None)
}

fn find_irradiance_ktx_tool(explicit: Option<PathBuf>) -> Result<KtxTool> {
    if let Some(path) = explicit {
        let tool = KtxTool {
            kind: ktx_tool_kind(&path),
            path,
        };
        if !matches!(tool.kind, KtxToolKind::UnifiedKtx) {
            bail!(
                "irradiance volume export requires the unified KTX executable (ktx.exe), not legacy toktx.exe"
            );
        }
        if !tool.path.exists() {
            bail!("KTX executable does not exist: {}", tool.path.display());
        }
        return Ok(tool);
    }
    for path in [
        PathBuf::from("ktx"),
        PathBuf::from(r"C:\Program Files\KTX-Software\bin\ktx.exe"),
    ] {
        if (path.is_absolute() && path.exists())
            || (!path.is_absolute()
                && Command::new(&path)
                    .arg("--version")
                    .output()
                    .is_ok_and(|output| output.status.success()))
        {
            return Ok(KtxTool {
                kind: KtxToolKind::UnifiedKtx,
                path,
            });
        }
    }
    let Some(tool) = find_ktx_tool(None)? else {
        bail!(
            "KTX-Software was not found; install the unified ktx.exe or pass --toktx with its path"
        );
    };
    if !matches!(tool.kind, KtxToolKind::UnifiedKtx) {
        bail!(
            "irradiance volume export requires the unified KTX executable (ktx.exe), not legacy toktx.exe"
        );
    }
    Ok(tool)
}

fn find_irradiance_blender(
    irradiance_explicit: Option<&Path>,
    general_explicit: Option<&Path>,
) -> Result<PathBuf> {
    if let Some(path) = irradiance_explicit {
        validate_blender_45(path)?;
        return Ok(path.to_path_buf());
    }
    if let Some(path) = general_explicit {
        if !path.exists() {
            bail!("Blender executable does not exist: {}", path.display());
        }
        if validate_blender_45(path).is_ok() {
            return Ok(path.to_path_buf());
        }
        let preferred =
            PathBuf::from(r"C:\Program Files\Blender Foundation\Blender 4.5\blender.exe");
        if preferred.exists() {
            validate_blender_45(&preferred)?;
            println!(
                "configured Blender {} is not 4.5 LTS; using {} for irradiance baking",
                path.display(),
                preferred.display()
            );
            return Ok(preferred);
        }
        validate_blender_45(path)?;
        unreachable!("validated Blender path must return above")
    }
    let preferred = PathBuf::from(r"C:\Program Files\Blender Foundation\Blender 4.5\blender.exe");
    if preferred.exists() {
        validate_blender_45(&preferred)?;
        return Ok(preferred);
    }
    let blender = find_blender(None)?;
    validate_blender_45(&blender)?;
    Ok(blender)
}

fn validate_blender_45(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("Blender executable does not exist: {}", path.display());
    }
    let output = Command::new(path)
        .arg("--version")
        .output()
        .with_context(|| format!("could not run Blender at {}", path.display()))?;
    let version = String::from_utf8_lossy(&output.stdout);
    if !output.status.success()
        || !version
            .lines()
            .next()
            .is_some_and(|line| line.contains("4.5"))
    {
        bail!(
            "irradiance-volume baking requires Blender 4.5 LTS; {} reported {}",
            path.display(),
            version.lines().next().unwrap_or("an unknown version")
        );
    }
    Ok(())
}

fn ktx_tool_kind(path: &Path) -> KtxToolKind {
    if path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .is_some_and(|stem| stem.eq_ignore_ascii_case("ktx"))
    {
        KtxToolKind::UnifiedKtx
    } else {
        KtxToolKind::LegacyToktx
    }
}

fn relative_asset_path(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)
        .with_context(|| {
            format!(
                "{} is outside asset root {}",
                path.display(),
                root.display()
            )
        })?
        .to_string_lossy()
        .replace('\\', "/"))
}

fn blender_path(path: &Path) -> String {
    let value = path.to_string_lossy();
    value.strip_prefix(r"\\?\").unwrap_or(&value).to_owned()
}

fn tail(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let lines = text.lines().collect::<Vec<_>>();
    lines
        .iter()
        .rev()
        .take(40)
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bake_job_emits_resolved_cell_directional_light() {
        let lighting = PreparedCellLighting {
            directional_rgba: [0.5, 0.5, 0.5, 1.0],
            directional_fade: 2.0,
            ..Default::default()
        };
        assert_eq!(cell_directional_illuminance(&lighting), 10_000.0);
        assert_eq!(
            cell_directional_illuminance(&PreparedCellLighting::default()),
            0.0
        );
    }

    #[test]
    fn only_static_semantics_are_batchable_without_changing_bake_inclusion() {
        fn placement(semantic: PreparedSemantic) -> PreparedPlacement {
            PreparedPlacement {
                reference_form_id: 1,
                base_form_id: 2,
                asset_path: Some("assets/test.glb".into()),
                translation: [0.0; 3],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                scale: 1.0,
                error: None,
                physics_asset_path: None,
                physics_source: None,
                physics_classification: PreparedPhysicsClassification::Static,
                reference_kind: "REFR".into(),
                base_kind: "STAT".into(),
                editor_id: None,
                display_name: None,
                count: 1,
                semantic,
                initially_enabled: true,
                enable_parent: None,
                owner_form_id: None,
                owner_faction_rank: None,
                inventory: Vec::new(),
                audio: Default::default(),
                ao_mode: "ao-none".into(),
            }
        }

        let static_placement = placement(PreparedSemantic::Static);
        assert!(is_bake_static(&static_placement));
        assert!(is_batchable_static(&static_placement));

        let mut dynamic_placement = placement(PreparedSemantic::Static);
        dynamic_placement.physics_classification = PreparedPhysicsClassification::Dynamic;
        assert!(!is_bake_static(&dynamic_placement));
        assert!(!is_batchable_static(&dynamic_placement));

        for semantic in [
            PreparedSemantic::Furniture,
            PreparedSemantic::Npc(super::super::manifest::PreparedActor {
                base_template_form_id: None,
            }),
            PreparedSemantic::Creature(super::super::manifest::PreparedActor {
                base_template_form_id: None,
            }),
            PreparedSemantic::Unsupported,
        ] {
            let placement = placement(semantic);
            assert!(is_bake_static(&placement));
            assert!(!is_batchable_static(&placement));
        }
    }
}
