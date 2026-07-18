//! Prepared-manifest and baked-scene compatibility checks.

use super::*;

pub(crate) fn ensure_prepared_manifest_compatible(
    manifest: &PreparedSceneManifest,
    expected_converter_revision: &str,
    expected_physics_schema: u32,
) -> Result<()> {
    let mut issues = Vec::new();
    if manifest.schema_version != CURRENT_MANIFEST_SCHEMA_VERSION {
        let direction = if manifest.schema_version < CURRENT_MANIFEST_SCHEMA_VERSION {
            "older than"
        } else {
            "newer than"
        };
        issues.push(format!(
            "manifest schema {} is {direction} current schema {}",
            manifest.schema_version, CURRENT_MANIFEST_SCHEMA_VERSION
        ));
    }
    if manifest.prepare_revision.as_deref() != Some(CURRENT_PREPARE_REVISION) {
        issues.push(format!(
            "prepare revision is {:?}, expected {CURRENT_PREPARE_REVISION}",
            manifest.prepare_revision
        ));
    }
    if manifest.converter_revision.as_deref() != Some(expected_converter_revision) {
        issues.push(format!(
            "converter revision is {:?}, expected {expected_converter_revision}",
            manifest.converter_revision
        ));
    }
    if manifest.physics_schema_version != Some(expected_physics_schema) {
        issues.push(format!(
            "physics schema is {:?}, expected {expected_physics_schema}",
            manifest.physics_schema_version
        ));
    }
    fail_for_compatibility_issues(manifest, issues)
}

pub(crate) fn ensure_prepared_manifest_compatible_any(
    manifest: &PreparedSceneManifest,
    expected_converter_revisions: &[&str],
    expected_physics_schema: u32,
) -> Result<()> {
    let Some(actual) = manifest.converter_revision.as_deref() else {
        return ensure_prepared_manifest_compatible(
            manifest,
            expected_converter_revisions
                .first()
                .copied()
                .unwrap_or("<none>"),
            expected_physics_schema,
        );
    };
    let expected = expected_converter_revisions
        .iter()
        .copied()
        .find(|revision| *revision == actual)
        .unwrap_or_else(|| {
            expected_converter_revisions
                .first()
                .copied()
                .unwrap_or("<none>")
        });
    ensure_prepared_manifest_compatible(manifest, expected, expected_physics_schema)
}

pub(crate) fn ensure_baked_scene_compatible(manifest: &PreparedSceneManifest) -> Result<()> {
    let mut issues = Vec::new();
    if let Some(bake) = manifest.bake.as_ref()
        && bake.bake_revision.as_deref() != Some(CURRENT_BAKE_REVISION)
    {
        issues.push(format!(
            "bake revision is {:?}, expected {CURRENT_BAKE_REVISION}",
            bake.bake_revision
        ));
    }
    fail_for_compatibility_issues(manifest, issues)
}

pub(crate) fn fail_for_compatibility_issues(
    manifest: &PreparedSceneManifest,
    issues: Vec<String>,
) -> Result<()> {
    if issues.is_empty() {
        return Ok(());
    }
    let details = issues
        .into_iter()
        .map(|issue| format!("  - {issue}"))
        .collect::<Vec<_>>()
        .join("\n");
    bail!(
        "prepared scene {} is incompatible with this build:\n{}\nrun `prepare` again, then `bake` if a baked scene is required",
        cell_label(&manifest.cell),
        details
    )
}
