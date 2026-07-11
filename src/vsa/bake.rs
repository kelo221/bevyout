use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli::{BakeArgs, BakeDevice, BakeQuality};

use super::assets::find_blender;
use super::manifest::{PreparedLightmapBinding, PreparedLightmapPage, PreparedSceneManifest};

#[derive(Debug, Serialize)]
struct BakeJob {
    asset_root: String,
    output_scene: String,
    output_exr: String,
    preview_output: String,
    result_json: String,
    page_size: u32,
    samples: u32,
    bounces: u32,
    gutter: u32,
    device: String,
    preview_only: bool,
    fast_gi: bool,
    indirect_clamp: f32,
    include_indirect: bool,
    denoise: bool,
    bake_all: bool,
    emission_scale: f32,
    ambient_rgba: [f32; 4],
    placements: Vec<JobPlacement>,
    lights: Vec<JobLight>,
}

#[derive(Debug, Serialize)]
struct JobPlacement {
    reference_form_id: u32,
    asset_path: String,
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
    bindings: Vec<PreparedLightmapBinding>,
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
    let manifest_path = fs::canonicalize(&args.manifest).context("manifest does not exist")?;
    let text = fs::read_to_string(&manifest_path).context("could not read scene manifest")?;
    let mut manifest: PreparedSceneManifest =
        ron::de::from_str(&text).context("invalid scene manifest; run prepare before bake")?;
    if manifest
        .placements
        .iter()
        .all(|placement| placement.asset_path.is_none())
    {
        bail!("scene manifest contains no renderable placements");
    }

    let blender = find_blender(args.blender)?;
    let ktx_tool = if matches!(args.quality, BakeQuality::Preview) {
        None
    } else {
        find_ktx_tool(args.toktx)?
    };
    if ktx_tool.is_none()
        && !args.keep_intermediate
        && !matches!(args.quality, BakeQuality::Preview)
    {
        bail!(
            "KTX-Software was not found; install it or pass --toktx with ktx.exe/toktx.exe. Use --keep-intermediate to produce an EXR-only bake"
        );
    }
    let asset_root = fs::canonicalize(&manifest.asset_root)
        .with_context(|| format!("asset root does not exist: {}", manifest.asset_root))?;
    let cell_dir = asset_root
        .join("scenes")
        .join(format!("{:08x}", manifest.cell.form_id));
    let output_dir = cell_dir.join("baked");
    if output_dir.exists() && !args.force {
        bail!(
            "bake output already exists: {}; pass --force to replace it",
            output_dir.display()
        );
    }
    fs::create_dir_all(&output_dir)?;

    let (
        page_size,
        samples,
        bounces,
        gutter,
        fast_gi,
        indirect_clamp,
        include_indirect,
        denoise,
        bake_all,
    ) = match args.quality {
        // Fast GI and a finite indirect clamp keep an interactive preview from
        // spending minutes on bright multi-bounce interiors.
        BakeQuality::Preview => (1024, 16, 1, 4, true, 2.0, true, true, true),
        // Quick is intentionally a direct-light bake. Indirect transport is
        // the dominant cost for a large Fallout cell and is reserved for
        // Final, where the additional time is expected.
        BakeQuality::Quick => (512, 4, 1, 2, false, 1.0, false, false, false),
        BakeQuality::Final => (4096, 512, 4, 8, false, 0.0, true, true, true),
    };
    let runtime_lighting = matches!(args.quality, BakeQuality::Quick);
    let output_scene = output_dir.join("scene.glb");
    let output_exr = output_dir.join("lightmap.exr");
    let preview_output = output_dir.join("preview.png");
    let result_json = output_dir.join("result.json");
    let job_file = output_dir.join("job.json");
    let job = BakeJob {
        asset_root: blender_path(&asset_root),
        output_scene: blender_path(&output_scene),
        output_exr: blender_path(&output_exr),
        preview_output: blender_path(&preview_output),
        result_json: blender_path(&result_json),
        page_size,
        samples,
        bounces,
        gutter,
        device: bake_device_name(args.device).to_owned(),
        preview_only: matches!(args.quality, BakeQuality::Preview),
        fast_gi,
        indirect_clamp,
        include_indirect,
        denoise,
        bake_all,
        // Runtime glow maps are intentionally much brighter than their physical
        // bake contribution so they remain visible under Bloom in the viewer.
        emission_scale: 0.01,
        ambient_rgba: manifest.cell.ambient_rgba,
        placements: manifest
            .placements
            .iter()
            .filter_map(|placement| {
                Some(JobPlacement {
                    reference_form_id: placement.reference_form_id,
                    asset_path: placement.asset_path.clone()?,
                    translation: placement.translation,
                    rotation_xyzw: placement.rotation_xyzw,
                    scale: placement.scale,
                })
            })
            .collect(),
        lights: manifest
            .lights
            .iter()
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
    if !output_scene.exists() || !output_exr.exists() || !result_json.exists() {
        bail!(
            "Blender reported success but did not create the expected bake outputs in {}",
            output_dir.display()
        );
    }

    let Some(ktx_tool) = ktx_tool else {
        if !args.keep_intermediate {
            bail!(
                "Blender bake completed to {} but KTX-Software is unavailable; pass --toktx with ktx.exe/toktx.exe. EXR was kept at {}",
                output_dir.display(),
                output_exr.display()
            );
        }
        println!(
            "Blender bake completed; KTX2 conversion skipped because KTX-Software is unavailable. EXR: {}",
            output_exr.display()
        );
        return Ok(());
    };

    let ktx2_path = output_dir.join("lightmap.ktx2");
    let mut ktx_command = Command::new(&ktx_tool.path);
    match ktx_tool.kind {
        KtxToolKind::LegacyToktx => {
            ktx_command
                .arg("--t2")
                .arg("--target_type")
                .arg("RGBA16F")
                .arg("--assign_oetf")
                .arg("linear")
                .arg("--zcmp")
                .arg("18");
        }
        KtxToolKind::UnifiedKtx => {
            ktx_command
                .arg("create")
                .arg("--format")
                .arg("R16G16B16A16_SFLOAT")
                .arg("--assign-tf")
                .arg("linear")
                .arg("--assign-texcoord-origin")
                .arg("top-left")
                .arg("--zstd")
                .arg("18");
        }
    }
    let ktx_output = ktx_command
        .arg(&output_exr)
        .arg(&ktx2_path)
        .output()
        .context("failed to start KTX-Software")?;
    if !ktx_output.status.success() {
        if !args.keep_intermediate {
            bail!(
                "KTX-Software failed with {}:\n{}\n{}\nEXR was kept at {}",
                ktx_output.status,
                tail(&ktx_output.stdout),
                tail(&ktx_output.stderr),
                output_exr.display()
            );
        }
        bail!(
            "KTX-Software failed with {}; EXR was kept at {}",
            ktx_output.status,
            output_exr.display()
        );
    }

    let bake_result: BlenderBakeResult =
        serde_json::from_slice(&fs::read(&result_json)?).context("invalid Blender bake result")?;
    if bake_result.bindings.is_empty() {
        bail!("Blender produced no lightmap bindings");
    }
    let scene_path = relative_asset_path(&asset_root, &output_scene)?;
    let lightmap_path = relative_asset_path(&asset_root, &ktx2_path)?;
    let mut fingerprint = Sha256::new();
    fingerprint.update(manifest.source_fingerprint.as_bytes());
    fingerprint.update(serde_json::to_vec(&job)?);
    let source_fingerprint = format!("{:x}", fingerprint.finalize());
    manifest.schema_version = 2;
    manifest.bake = Some(super::manifest::PreparedBake {
        source_fingerprint,
        scene_path,
        lightmaps: vec![PreparedLightmapPage {
            asset_path: lightmap_path,
            width: page_size,
            height: page_size,
        }],
        bindings: bake_result.bindings,
        // Blender's direct-bake values are HDR radiance, not display-referred
        // colors. Bevy's lightmap shader expects this calibration factor so
        // indoor Fallout surfaces remain visible under the camera exposure.
        lightmap_exposure: 250.0,
        runtime_lighting,
    });
    fs::write(
        &manifest_path,
        to_string_pretty(&manifest, PrettyConfig::default())?,
    )?;
    if !args.keep_intermediate {
        let _ = fs::remove_file(&job_file);
        let _ = fs::remove_file(&script_file);
        let _ = fs::remove_file(&result_json);
        let _ = fs::remove_file(&output_exr);
    }
    println!(
        "baked cell {:08x}: {} bindings -> {}",
        manifest.cell.form_id,
        manifest.bake.as_ref().map_or(0, |bake| bake.bindings.len()),
        ktx2_path.display()
    );
    Ok(())
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

fn bake_device_name(device: BakeDevice) -> &'static str {
    match device {
        BakeDevice::Cpu => "CPU",
        BakeDevice::Optix => "OPTIX",
        BakeDevice::Cuda => "CUDA",
        BakeDevice::Hip => "HIP",
    }
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
