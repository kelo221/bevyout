use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};

use super::{
    Diagnostic, PreparedCellLighting, PreparedLight, PreparedPlacement, PreparedReflectionProbe,
    PreparedReflectionProbeSet, REFLECTION_PROBE_REVISION, ReflectionProbeLayout,
};
use crate::vsa::bake::{
    JobLight, JobPlacement, cell_directional_illuminance, find_unified_ktx_tool, is_bake_static,
    rust_irradiance::{DirectionalBakeLight, trace_reflection_cubemap},
    rust_scene,
};

pub(crate) const REFLECTION_PROBE_FACE_RESOLUTION: u32 = 64;
const REFLECTION_PROBE_DIFFUSE_RESOLUTION: u32 = 16;
const STATIC_BATCH_CHUNK_METERS: f32 = 64.0;

pub(crate) struct ReflectionProbePrepareOptions<'a> {
    pub(crate) asset_root: &'a Path,
    pub(crate) scene_dir: &'a Path,
    pub(crate) rebuild: bool,
    pub(crate) ktx: Option<PathBuf>,
}

pub(crate) fn prepare_reflection_probes(
    options: ReflectionProbePrepareOptions<'_>,
    layouts: &[ReflectionProbeLayout],
    placements: &[PreparedPlacement],
    lights: &[PreparedLight],
    lighting: &PreparedCellLighting,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Option<PreparedReflectionProbeSet>> {
    if layouts.is_empty() {
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: "reflection probes omitted: no reliable walkable interior regions".into(),
        });
        return Ok(None);
    }
    let job_placements = placements
        .iter()
        .filter(|placement| placement.initially_enabled && is_bake_static(placement))
        .filter_map(|placement| {
            Some(JobPlacement {
                reference_form_id: placement.reference_form_id,
                asset_path: placement.asset_path.clone()?,
                ao_mode: placement.ao_mode.clone(),
                batchable_static: true,
                translation: placement.translation,
                rotation_xyzw: placement.rotation_xyzw,
                scale: placement.scale,
            })
        })
        .collect::<Vec<_>>();
    let job_placements = exclude_animated_static_placements(job_placements, |asset_path| {
        rust_scene::asset_contains_animation(options.asset_root, asset_path)
    })?;
    if job_placements.is_empty() {
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: "reflection probes omitted: no eligible static visual placements".into(),
        });
        return Ok(None);
    }
    let job_lights = lights
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
            kind: light.kind.clone(),
        })
        .collect::<Vec<_>>();
    let fingerprint = reflection_fingerprint(
        options.asset_root,
        layouts,
        &job_placements,
        &job_lights,
        lighting,
    )?;
    let output_dir = options
        .scene_dir
        .join("reflection-probes")
        .join(&fingerprint);
    let paths = (0..layouts.len())
        .map(|index| {
            (
                output_dir.join(format!("probe-{index:02}-diffuse.ktx2")),
                output_dir.join(format!("probe-{index:02}-specular.ktx2")),
            )
        })
        .collect::<Vec<_>>();
    let prepared = PreparedReflectionProbeSet {
        revision: REFLECTION_PROBE_REVISION.into(),
        source_fingerprint: fingerprint.clone(),
        face_resolution: REFLECTION_PROBE_FACE_RESOLUTION,
        probes: layouts
            .iter()
            .zip(&paths)
            .map(|(layout, (diffuse, specular))| {
                Ok(PreparedReflectionProbe {
                    capture_translation: layout.capture_translation,
                    influence_half_extents: layout.influence_half_extents,
                    parallax_half_extents: layout.parallax_half_extents,
                    falloff: layout.falloff,
                    diffuse_asset_path: relative_asset_path(options.asset_root, diffuse)?,
                    specular_asset_path: relative_asset_path(options.asset_root, specular)?,
                })
            })
            .collect::<Result<Vec<_>>>()?,
    };
    if !options.rebuild
        && paths
            .iter()
            .all(|(diffuse, specular)| diffuse.is_file() && specular.is_file())
    {
        let message = format!(
            "reflection probes: cache hit, {} probe(s) at {}x{}",
            prepared.probes.len(),
            prepared.face_resolution,
            prepared.face_resolution
        );
        println!("{message}");
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message,
        });
        return Ok(Some(prepared));
    }

    let started = Instant::now();
    fs::create_dir_all(&output_dir)?;
    let ktx = find_unified_ktx_tool(options.ktx)?;
    println!(
        "reflection probes: composing {} static placements for {} capture(s)",
        job_placements.len(),
        layouts.len()
    );
    let scene = rust_scene::compose_scene(
        options.asset_root,
        &job_placements,
        STATIC_BATCH_CHUNK_METERS,
    )?;
    let directional = DirectionalBakeLight {
        color_rgba: lighting.directional_rgba,
        rotation_xyzw: lighting.directional_rotation_xyzw(),
        illuminance: cell_directional_illuminance(lighting),
    };
    for (index, (layout, (diffuse_path, specular_path))) in layouts.iter().zip(&paths).enumerate() {
        println!(
            "reflection probes: tracing {}/{} at {:?}",
            index + 1,
            layouts.len(),
            layout.capture_translation
        );
        let faces = trace_reflection_cubemap(
            &scene,
            &job_lights,
            &directional,
            layout.capture_translation,
            REFLECTION_PROBE_FACE_RESOLUTION,
        )?;
        let specular_levels = box_filter_mip_chain(&faces, REFLECTION_PROBE_FACE_RESOLUTION);
        write_cubemap_ktx(
            &ktx.path,
            &output_dir,
            &format!("probe-{index:02}-specular"),
            specular_path,
            REFLECTION_PROBE_FACE_RESOLUTION,
            &specular_levels,
        )?;
        // Prepared irradiance remains authoritative for diffuse GI. A black
        // diffuse cube lets Bevy use this probe for specular reflections
        // without replacing irradiance with discontinuous local diffuse light.
        let diffuse_faces = vec![
            vec![
                0_u32;
                (REFLECTION_PROBE_DIFFUSE_RESOLUTION * REFLECTION_PROBE_DIFFUSE_RESOLUTION)
                    as usize
            ];
            6
        ];
        write_cubemap_ktx(
            &ktx.path,
            &output_dir,
            &format!("probe-{index:02}-diffuse"),
            diffuse_path,
            REFLECTION_PROBE_DIFFUSE_RESOLUTION,
            &[diffuse_faces],
        )?;
    }
    let bytes = paths
        .iter()
        .flat_map(|(diffuse, specular)| [diffuse, specular])
        .filter_map(|path| path.metadata().ok())
        .map(|metadata| metadata.len())
        .sum::<u64>();
    let message = format!(
        "reflection probes: generated {} probe(s), {} cubemap files, {} bytes in {:.2}s",
        layouts.len(),
        paths.len() * 2,
        bytes,
        started.elapsed().as_secs_f64()
    );
    println!("{message}");
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message,
    });
    Ok(Some(prepared))
}

fn exclude_animated_static_placements(
    placements: Vec<JobPlacement>,
    mut asset_contains_animation: impl FnMut(&str) -> Result<bool>,
) -> Result<Vec<JobPlacement>> {
    let mut animation_by_asset = HashMap::<String, bool>::new();
    let mut retained = Vec::with_capacity(placements.len());
    for placement in placements {
        let animated = if let Some(animated) = animation_by_asset.get(&placement.asset_path) {
            *animated
        } else {
            let animated = asset_contains_animation(&placement.asset_path)?;
            animation_by_asset.insert(placement.asset_path.clone(), animated);
            animated
        };
        if animated {
            println!(
                "reflection probes: skipping animated static placement {:08x} ({})",
                placement.reference_form_id, placement.asset_path
            );
        } else {
            retained.push(placement);
        }
    }
    Ok(retained)
}

fn reflection_fingerprint(
    asset_root: &Path,
    layouts: &[ReflectionProbeLayout],
    placements: &[JobPlacement],
    lights: &[JobLight],
    lighting: &PreparedCellLighting,
) -> Result<String> {
    let mut hash = Sha256::new();
    hash.update(REFLECTION_PROBE_REVISION.as_bytes());
    hash.update(REFLECTION_PROBE_FACE_RESOLUTION.to_le_bytes());
    for layout in layouts {
        for value in layout
            .capture_translation
            .into_iter()
            .chain(layout.influence_half_extents)
            .chain(layout.parallax_half_extents)
            .chain(layout.falloff)
        {
            hash.update(value.to_le_bytes());
        }
    }
    for placement in placements {
        hash.update(placement.reference_form_id.to_le_bytes());
        hash.update(placement.asset_path.as_bytes());
        for value in placement
            .translation
            .into_iter()
            .chain(placement.rotation_xyzw)
            .chain([placement.scale])
        {
            hash.update(value.to_le_bytes());
        }
        let path = asset_root.join(
            placement
                .asset_path
                .replace('/', std::path::MAIN_SEPARATOR_STR),
        );
        hash.update(
            fs::read(&path).with_context(|| format!("could not fingerprint {}", path.display()))?,
        );
    }
    for light in lights {
        for value in light
            .translation
            .into_iter()
            .chain(light.rotation_xyzw)
            .chain(light.color_rgba)
            .chain([light.radius, light.intensity_lumens])
        {
            hash.update(value.to_le_bytes());
        }
        hash.update(light.kind.as_bytes());
    }
    for value in lighting
        .ambient_rgba
        .into_iter()
        .chain(lighting.directional_rgba)
        .chain(lighting.directional_rotation_xyzw())
    {
        hash.update(value.to_le_bytes());
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn box_filter_mip_chain(base: &[Vec<u32>], resolution: u32) -> Vec<Vec<Vec<u32>>> {
    let mut levels = vec![base.to_vec()];
    let mut current_resolution = resolution;
    while current_resolution > 1 {
        let next_resolution = current_resolution / 2;
        let next = levels
            .last()
            .expect("base level exists")
            .iter()
            .map(|face| downsample_face(face, current_resolution, next_resolution))
            .collect();
        levels.push(next);
        current_resolution = next_resolution;
    }
    levels
}

fn downsample_face(face: &[u32], source_resolution: u32, target_resolution: u32) -> Vec<u32> {
    let mut output = Vec::with_capacity((target_resolution * target_resolution) as usize);
    for y in 0..target_resolution {
        for x in 0..target_resolution {
            let source_x = x * 2;
            let source_y = y * 2;
            let samples = [
                face[(source_y * source_resolution + source_x) as usize],
                face[(source_y * source_resolution + source_x + 1) as usize],
                face[((source_y + 1) * source_resolution + source_x) as usize],
                face[((source_y + 1) * source_resolution + source_x + 1) as usize],
            ];
            output.push(average_rgb9e5(samples));
        }
    }
    output
}

fn average_rgb9e5(words: [u32; 4]) -> u32 {
    let mut sum = [0.0_f32; 3];
    for word in words {
        let value = unpack_rgb9e5(word);
        for axis in 0..3 {
            sum[axis] += value[axis];
        }
    }
    pack_rgb9e5(sum.map(|value| value * 0.25))
}

fn unpack_rgb9e5(word: u32) -> [f32; 3] {
    let exponent = ((word >> 27) & 0x1f) as i32;
    let scale = 2.0_f32.powi(exponent - 24);
    [
        (word & 0x1ff) as f32 * scale,
        ((word >> 9) & 0x1ff) as f32 * scale,
        ((word >> 18) & 0x1ff) as f32 * scale,
    ]
}

fn pack_rgb9e5(color: [f32; 3]) -> u32 {
    let maximum = color
        .into_iter()
        .filter(|value| value.is_finite())
        .fold(0.0_f32, f32::max)
        .clamp(0.0, 65_408.0);
    if maximum <= 0.0 {
        return 0;
    }
    let mut exponent = (maximum.log2().floor() as i32 + 16).clamp(1, 31);
    let mut scale = 2.0_f32.powi(exponent - 24);
    if (maximum / scale).round() > 511.0 && exponent < 31 {
        exponent += 1;
        scale *= 2.0;
    }
    let channel = |value: f32| -> u32 { ((value.max(0.0) / scale).round() as u32).min(511) };
    channel(color[0])
        | (channel(color[1]) << 9)
        | (channel(color[2]) << 18)
        | ((exponent as u32) << 27)
}

fn write_cubemap_ktx(
    ktx: &Path,
    output_dir: &Path,
    stem: &str,
    output_path: &Path,
    resolution: u32,
    levels: &[Vec<Vec<u32>>],
) -> Result<()> {
    let temporary = output_dir.join(format!(".tmp-{stem}-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary)?;
    }
    fs::create_dir_all(&temporary)?;
    let mut inputs = Vec::new();
    for (level, faces) in levels.iter().enumerate() {
        for (face, pixels) in faces.iter().enumerate() {
            let path = temporary.join(format!("level-{level:02}-face-{face:02}.raw"));
            let mut bytes = Vec::with_capacity(pixels.len() * size_of::<u32>());
            for pixel in pixels {
                bytes.extend_from_slice(&pixel.to_le_bytes());
            }
            fs::write(&path, bytes)?;
            inputs.push(path);
        }
    }
    let input_list = temporary.join("raw-files.txt");
    let mut listing = String::new();
    for input in &inputs {
        writeln!(
            listing,
            "{}",
            input
                .to_str()
                .context("reflection-probe path is not UTF-8")?
        )?;
    }
    fs::write(&input_list, listing)?;
    let temporary_output = temporary.join(format!("{stem}.ktx2"));
    let output = Command::new(ktx)
        .args([
            "create",
            "--raw",
            "--format",
            "E5B9G9R9_UFLOAT_PACK32",
            "--width",
            &resolution.to_string(),
            "--height",
            &resolution.to_string(),
            "--levels",
            &levels.len().to_string(),
            "--cubemap",
            "--assign-tf",
            "linear",
            "--assign-texcoord-origin",
            "top-left",
            "--zstd",
            "3",
            &format!("@{}", input_list.display()),
        ])
        .arg(&temporary_output)
        .output()
        .context("failed to start KTX-Software for reflection probe")?;
    if !output.status.success() {
        bail!(
            "KTX-Software failed with {}:\n{}\n{}\nreflection inputs kept at {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            temporary.display()
        );
    }
    let validation = Command::new(ktx)
        .arg("validate")
        .arg(&temporary_output)
        .output()
        .context("failed to validate reflection-probe KTX2")?;
    if !validation.status.success() {
        bail!(
            "reflection-probe KTX validation failed with {}:\n{}",
            validation.status,
            String::from_utf8_lossy(&validation.stderr)
        );
    }
    if output_path.exists() {
        fs::remove_file(output_path)?;
    }
    fs::rename(&temporary_output, output_path)?;
    fs::remove_dir_all(&temporary)?;
    Ok(())
}

fn relative_asset_path(asset_root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(asset_root)
        .with_context(|| {
            format!(
                "{} is outside asset root {}",
                path.display(),
                asset_root.display()
            )
        })?
        .to_string_lossy()
        .replace('\\', "/"))
}

#[cfg(test)]
#[path = "tests/reflection_probes.rs"]
mod tests;
