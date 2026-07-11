use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use std::collections::HashMap;
use std::fs;

use super::assets::{
    convert_staged_textures, find_blender, load_archives, resolve_asset, run_blender_batch,
    stage_textures,
};
use super::manifest::{Diagnostic, PreparedLight, PreparedPlacement, PreparedSceneManifest};
use super::paths::{
    absolutize, fingerprint, is_editor_marker, normalize_asset_path, parse_form_id,
    placement_transform,
};
use super::plugin::{RECORD_DELETED, RECORD_DISABLED, parse_plugin};
use crate::cli::PrepareArgs;

pub(crate) fn prepare(args: PrepareArgs) -> Result<()> {
    let root = fs::canonicalize(&args.game_root).context("game root does not exist")?;
    let plugin_path = if args.plugin.is_absolute() {
        args.plugin.clone()
    } else {
        root.join("Data").join(&args.plugin)
    };
    let plugin_path = fs::canonicalize(&plugin_path).context("plugin does not exist")?;
    let cell_id = parse_form_id(&args.cell)?;
    let data_root = root.join("Data");
    let cache_dir = absolutize(&args.cache_dir)?;
    let staging_dir = cache_dir.join("staging");
    let assets_dir = cache_dir.join("assets");
    let scene_dir = cache_dir.join("scenes").join(format!("{cell_id:08x}"));
    fs::create_dir_all(&staging_dir)?;
    fs::create_dir_all(&assets_dir)?;
    fs::create_dir_all(&scene_dir)?;

    let bytes = fs::read(&plugin_path).context("failed to read plugin")?;
    let source_fingerprint = fingerprint(&bytes);
    let mut diagnostics = Vec::new();
    let mut validator = esplugin::Plugin::new(esplugin::GameId::Fallout3, &plugin_path);
    if let Err(error) = validator.parse_file(esplugin::ParseOptions::header_only()) {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!("esplugin validation failed: {error}"),
        });
    }
    let mut parsed = parse_plugin(&bytes, cell_id).context("failed to parse Fallout plugin")?;
    let cell = parsed.cell.take().context("requested cell was not found")?;
    if !cell.interior {
        bail!("cell {cell_id:08x} is not an interior cell; LAND support is not part of this slice")
    }

    let archives = load_archives(&data_root, &mut diagnostics)?;
    let blender = find_blender(args.blender)?;
    let mut jobs = Vec::new();
    let mut placements = Vec::new();
    let mut lights = Vec::new();
    let mut seen_models = HashMap::<String, String>::new();

    for reference in parsed.references.drain(..) {
        if reference.flags & (RECORD_DELETED | RECORD_DISABLED) != 0 {
            continue;
        }
        let transform = placement_transform(&reference);
        let Some(base) = parsed.bases.get(&reference.base_form_id) else {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "missing base record {:08x} for reference {:08x}",
                    reference.base_form_id, reference.form_id
                ),
            });
            placements.push(PreparedPlacement {
                reference_form_id: reference.form_id,
                base_form_id: reference.base_form_id,
                asset_path: None,
                translation: transform.0,
                rotation_xyzw: transform.1,
                scale: transform.2,
                error: Some("missing base record".into()),
            });
            continue;
        };
        if base.kind == "LIGH" {
            lights.push(PreparedLight {
                translation: transform.0,
                color_rgba: [1.0, 0.78, 0.55, 1.0],
                radius: 5.0,
            });
        }
        let Some(model) = base.model.as_ref() else {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "reference {:08x} ({}) has no MODL",
                    reference.form_id, base.kind
                ),
            });
            continue;
        };
        let normalized_model = normalize_asset_path(model);
        if is_editor_marker(&normalized_model) {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!("skipping non-rendering editor marker {normalized_model}"),
            });
            continue;
        }
        let Some(nif_bytes) = resolve_asset(&data_root, &archives, &normalized_model)? else {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "missing model {normalized_model} for reference {:08x}",
                    reference.form_id
                ),
            });
            placements.push(PreparedPlacement {
                reference_form_id: reference.form_id,
                base_form_id: reference.base_form_id,
                asset_path: None,
                translation: transform.0,
                rotation_xyzw: transform.1,
                scale: transform.2,
                error: Some(format!("missing model {normalized_model}")),
            });
            continue;
        };
        let model_hash = fingerprint(&nif_bytes);
        let asset_name = format!("{model_hash}.glb");
        let asset_path = format!("assets/{asset_name}");
        if !seen_models.contains_key(&normalized_model) {
            let staging_nif = staging_dir.join(&normalized_model);
            if let Some(parent) = staging_nif.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&staging_nif, &nif_bytes)?;
            stage_textures(
                &nif_bytes,
                data_root.as_path(),
                &archives,
                staging_dir.as_path(),
                &mut diagnostics,
            )?;
            let output = assets_dir.join(&asset_name);
            if args.force || !output.exists() {
                jobs.push((staging_nif, output, normalized_model.clone()));
            }
            seen_models.insert(normalized_model.clone(), asset_path.clone());
        }
        placements.push(PreparedPlacement {
            reference_form_id: reference.form_id,
            base_form_id: reference.base_form_id,
            asset_path: Some(asset_path),
            translation: transform.0,
            rotation_xyzw: transform.1,
            scale: transform.2,
            error: None,
        });
    }

    convert_staged_textures(&staging_dir, &mut diagnostics)?;
    if !jobs.is_empty() {
        run_blender_batch(&blender, &jobs, &data_root, &staging_dir)
            .context("headless Blender conversion failed")?;
    }
    let failures = placements.iter().filter(|p| p.asset_path.is_none()).count();
    if args.strict && failures > 0 {
        bail!("strict preparation failed with {failures} unresolved placements")
    }
    if placements.iter().all(|p| p.asset_path.is_none()) && lights.is_empty() {
        bail!("no renderable assets were found in cell {cell_id:08x}")
    }

    let manifest = PreparedSceneManifest {
        schema_version: 1,
        asset_root: cache_dir.to_string_lossy().to_string(),
        source_plugin: plugin_path.to_string_lossy().to_string(),
        source_fingerprint,
        cell,
        placements,
        lights,
        diagnostics,
    };
    let manifest_path = scene_dir.join("scene.ron");
    fs::write(
        &manifest_path,
        to_string_pretty(&manifest, PrettyConfig::default())?,
    )?;
    println!(
        "prepared {} placements ({} unresolved) -> {}",
        manifest.placements.len(),
        failures,
        manifest_path.display()
    );
    Ok(())
}
