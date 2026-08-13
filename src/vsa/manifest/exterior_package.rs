use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use bevyout_core::manifest::PreparedSceneManifest;
use bevyout_core::manifest::exterior::{EXTERIOR_CELL_PACKAGE_REVISION, ExteriorCellPackage};

use crate::vsa::prepare::exterior_scene_storage_plan;

/// Hydrates the compact exterior scene root from its canonical worldspace
/// package. Interior manifests and already-hydrated legacy manifests are
/// unchanged.
pub(crate) fn hydrate_exterior_package(manifest: &mut PreparedSceneManifest) -> Result<()> {
    if manifest.cell.interior || manifest.exterior.is_some() {
        return Ok(());
    }
    let worldspace_form_id = manifest.cell.worldspace_form_id.with_context(|| {
        format!(
            "exterior scene {:08x} has no worldspace identity",
            manifest.cell.form_id
        )
    })?;
    let plan = exterior_scene_storage_plan(worldspace_form_id, manifest.cell.form_id);
    let path = Path::new(&manifest.asset_root).join(
        plan.package_path
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    let bytes =
        fs::read(&path).with_context(|| format!("reading exterior package {}", path.display()))?;
    let package: ExteriorCellPackage = ron::de::from_bytes(&bytes)
        .with_context(|| format!("parsing exterior package {}", path.display()))?;
    validate_external_package(manifest, &package, &path)?;
    manifest.exterior = Some(package);
    Ok(())
}

fn validate_external_package(
    manifest: &PreparedSceneManifest,
    package: &ExteriorCellPackage,
    path: &Path,
) -> Result<()> {
    if package.revision != EXTERIOR_CELL_PACKAGE_REVISION {
        bail!(
            "exterior package {} has revision {}, expected {}",
            path.display(),
            package.revision,
            EXTERIOR_CELL_PACKAGE_REVISION
        );
    }
    if package.cell_form_id != manifest.cell.form_id {
        bail!(
            "exterior package {} belongs to cell {:08x}, expected {:08x}",
            path.display(),
            package.cell_form_id,
            manifest.cell.form_id
        );
    }
    if Some(package.worldspace_form_id) != manifest.cell.worldspace_form_id {
        bail!(
            "exterior package {} belongs to worldspace {:08x}, expected {:08x}",
            path.display(),
            package.worldspace_form_id,
            manifest.cell.worldspace_form_id.unwrap_or_default()
        );
    }
    if package.content_fingerprint != manifest.source_fingerprint {
        bail!(
            "exterior package {} content fingerprint does not match scene root",
            path.display()
        );
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests/exterior_package.rs"]
mod tests;
