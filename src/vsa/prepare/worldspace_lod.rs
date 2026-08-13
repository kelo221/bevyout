//! Native preparation of Fallout's worldspace terrain LOD tiles.
//!
//! FO3 ships the LOD geometry as `meshes/landscape/lod/<worldspace>` NIFs,
//! alongside matching diffuse/normal DDS files. They are presentation-only:
//! cell LAND remains the gameplay surface, while these tiles fill the
//! horizon after the streamed cell ring ends.

use super::*;
use bevyout_core::manifest::exterior::{
    ExteriorWorldspaceIndex, ExteriorWorldspaceLodAsset, GridCoordinate,
};
use std::collections::BTreeSet;

const WORLDSPACE_LOD_CONVERTER_TAG: &str = "worldspace-lod-v4-vertical-skirt-trim-empty-mesh";

#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare_worldspace_lod(
    index: &ExteriorWorldspaceIndex,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    assets_dir: &Path,
    rebuild_assets: bool,
    requested_workers: Option<usize>,
    strict: bool,
    output: &mut Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Vec<ExteriorWorldspaceLodAsset>> {
    let sources = lod_sources(index, archives);
    if sources.is_empty() {
        let message = format!(
            "worldspace LOD: no vanilla assets found for {:08x} ({})",
            index.worldspace_form_id,
            index.editor_id.as_deref().unwrap_or("unnamed")
        );
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: message.clone(),
        });
        output.push(message);
        return Ok(Vec::new());
    }
    let source_count = sources.len();

    let mut descriptors = Vec::with_capacity(source_count);
    let mut jobs = Vec::new();
    let mut job_descriptors = Vec::new();
    let mut reused = 0usize;
    for (source, level, grid, blocks) in sources {
        let Some(nif_bytes) = resolve_asset(data_root, archives, &source)? else {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!("worldspace LOD source is missing: {source}"),
            });
            continue;
        };
        let converter_revision = format!(
            "{NATIVE_NIF_CONVERTER_REVISION}-{WORLDSPACE_LOD_CONVERTER_TAG}-level{level}-{}",
            if blocks { "blocks" } else { "terrain" }
        );
        let asset_name = content_addressed_glb_name(&converter_revision, &nif_bytes);
        let output_path = assets_dir.join(&asset_name);
        let physics_name = physics_sidecar_name(&asset_name);
        let physics_path = assets_dir.join(&physics_name);
        let descriptor = ExteriorWorldspaceLodAsset {
            asset_path: format!("assets/{asset_name}"),
            level,
            grid,
            blocks,
        };
        let cache_valid = output_path.is_file()
            && physics_path.is_file()
            && validate_asset_cache_pair(&output_path, &physics_path).is_ok();
        match asset_cache_decision(output_path.is_file(), cache_valid, rebuild_assets) {
            AssetCacheDecision::Reuse => {
                reused += 1;
                descriptors.push(descriptor);
            }
            AssetCacheDecision::BuildMissing
            | AssetCacheDecision::RebuildInvalid
            | AssetCacheDecision::RebuildRequested => {
                let staging_path = staging_dir
                    .join("worldspace-lod")
                    .join(format!("{:08x}", index.worldspace_form_id))
                    .join(source.replace('/', std::path::MAIN_SEPARATOR_STR));
                if let Some(parent) = staging_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&staging_path, nif_bytes)?;
                jobs.push(AssetJob {
                    kind: AssetJobKind::StaticNif,
                    input: staging_path,
                    output: output_path,
                    physics_output: physics_path,
                    model: source,
                    conversion: if blocks {
                        // `blocks/` contains persistent landmark geometry,
                        // not terrain skirts. Keep its authored faces intact.
                        AssetConversion::Preserve
                    } else {
                        AssetConversion::WorldspaceLod
                    },
                    root_transform_policy: RootTransformPolicy::PreserveVerified,
                });
                job_descriptors.push(descriptor);
            }
        }
    }

    let mut converted = 0usize;
    let mut failed = 0usize;
    if !jobs.is_empty() {
        let cache_dir = assets_dir
            .parent()
            .context("worldspace LOD assets directory has no cache parent")?;
        let batch = run_native_batch(
            &jobs,
            data_root,
            archives,
            cache_dir,
            requested_workers,
            strict,
        )
        .context("native worldspace LOD conversion failed")?;
        let summary = batch.summary().line();
        output.push(summary.clone());
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: summary,
        });
        batch.enforce_strict(strict)?;
        for outcome in batch.outcomes {
            let Some(descriptor) = job_descriptors.get(outcome.index).cloned() else {
                continue;
            };
            if outcome.status == NativeJobStatus::Converted {
                converted += 1;
                descriptors.push(descriptor);
            } else {
                failed += 1;
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!(
                        "worldspace LOD conversion failed for {}: {}",
                        outcome.model,
                        outcome.error.as_deref().unwrap_or("unknown error")
                    ),
                });
            }
        }
    }

    descriptors.sort_by_key(|asset| {
        (
            asset.level,
            asset.grid,
            asset.blocks,
            asset.asset_path.clone(),
        )
    });
    let summary = worldspace_lod_summary(
        index.editor_id.as_deref().unwrap_or("unnamed"),
        index.worldspace_form_id,
        source_count,
        reused,
        converted,
        failed,
        descriptors.len(),
    );
    output.push(summary.clone());
    diagnostics.push(Diagnostic {
        severity: if failed == 0 { "info" } else { "warning" }.into(),
        message: summary,
    });
    Ok(descriptors)
}

fn worldspace_lod_summary(
    name: &str,
    form_id: u32,
    sources: usize,
    reused: usize,
    converted: usize,
    failed: usize,
    assets: usize,
) -> String {
    format!(
        "worldspace LOD {name} ({form_id:08x}): sources={sources} reused={reused} converted={converted} failed={failed} assets={assets}"
    )
}

fn lod_sources(
    index: &ExteriorWorldspaceIndex,
    archives: &[crate::vsa::bsa::BsaArchive],
) -> Vec<(String, u8, GridCoordinate, bool)> {
    let tokens = [index.editor_id.as_deref(), index.name.as_deref()];
    for token in tokens.into_iter().flatten() {
        let token = token.trim().to_ascii_lowercase();
        let prefix = format!("meshes/landscape/lod/{token}/");
        let mut paths = BTreeSet::new();
        for archive in archives {
            paths.extend(
                archive
                    .paths_with_extension("nif")
                    .into_iter()
                    .filter(|path| path.starts_with(&prefix)),
            );
        }
        let mut sources = paths
            .into_iter()
            .filter_map(|path| parse_lod_source(&path, &prefix))
            .collect::<Vec<_>>();
        if !sources.is_empty() {
            sources.sort_by(|left, right| left.0.cmp(&right.0));
            return sources;
        }
    }
    Vec::new()
}

fn parse_lod_source(path: &str, prefix: &str) -> Option<(String, u8, GridCoordinate, bool)> {
    let relative = path.strip_prefix(prefix)?;
    let file_name = relative.rsplit('/').next()?;
    let stem = file_name.strip_suffix(".nif")?;
    let fields = stem.split('.').collect::<Vec<_>>();
    if fields.len() != 4 {
        return None;
    }
    let level = fields[1].strip_prefix("level")?.parse::<u8>().ok()?;
    if !matches!(level, 4 | 8 | 16 | 32) {
        return None;
    }
    let x = fields[2].strip_prefix('x')?.parse::<i32>().ok()?;
    let y = fields[3].strip_prefix('y')?.parse::<i32>().ok()?;
    Some((
        path.to_owned(),
        level,
        GridCoordinate::new(x, y),
        relative.starts_with("blocks/"),
    ))
}

#[cfg(test)]
#[path = "worldspace_lod_tests.rs"]
mod tests;
