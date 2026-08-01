use super::*;

#[derive(Debug, Clone)]
pub(crate) struct PreparedVisualAsset {
    pub(crate) model_path: String,
    pub(crate) asset_path: String,
    pub(crate) root_transform_policy: RootTransformPolicy,
}

pub(crate) fn audit_prepared_visuals(
    cache_dir: &Path,
    assets: &[PreparedVisualAsset],
    placements: &[PreparedPlacement],
) -> Result<Vec<PreparedVisualIssue>> {
    let mut issues = Vec::new();
    for asset in assets {
        let path = cache_dir.join(asset.asset_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let audit = audit_glb_visuals(&path)
            .with_context(|| format!("could not audit converted GLB {}", path.display()))?;
        let expected_policy = asset.root_transform_policy.tag();
        if audit.root_transform_policy.as_deref() != Some(expected_policy) {
            bail!(
                "converted GLB {} reports root policy {:?}, expected {expected_policy}",
                path.display(),
                audit.root_transform_policy
            );
        }
        if audit.source_model.as_deref() != Some(asset.model_path.as_str()) {
            bail!(
                "converted GLB {} reports source model {:?}, expected {}",
                path.display(),
                audit.source_model,
                asset.model_path
            );
        }
        let source_meshes = audit.source_render_meshes.with_context(|| {
            format!("converted GLB {} has no source mesh count", path.display())
        })?;
        let source_vertices = audit.source_render_vertices.with_context(|| {
            format!(
                "converted GLB {} has no source vertex count",
                path.display()
            )
        })?;
        let source_triangles = audit.source_render_triangles.with_context(|| {
            format!(
                "converted GLB {} has no source triangle count",
                path.display()
            )
        })?;
        let spatial_audit_version = audit.spatial_audit_version.with_context(|| {
            format!(
                "converted GLB {} has no spatial audit version",
                path.display()
            )
        })?;
        let expected_spatial_corrections =
            audit.expected_spatial_corrections.with_context(|| {
                format!(
                    "converted GLB {} has no expected spatial correction count",
                    path.display()
                )
            })?;
        let verified_spatial_corrections =
            audit.verified_spatial_corrections.with_context(|| {
                format!(
                    "converted GLB {} has no verified spatial correction count",
                    path.display()
                )
            })?;
        let expected_collision_corrections =
            audit.expected_collision_corrections.with_context(|| {
                format!(
                    "converted GLB {} has no expected collision correction count",
                    path.display()
                )
            })?;
        let verified_collision_corrections =
            audit.verified_collision_corrections.with_context(|| {
                format!(
                    "converted GLB {} has no verified collision correction count",
                    path.display()
                )
            })?;
        if spatial_audit_version != 1
            || verified_spatial_corrections != expected_spatial_corrections
            || verified_collision_corrections != expected_collision_corrections
        {
            issues.push(issue_for_asset(
                "visual_spatial_mismatch",
                "error",
                asset,
                placements,
                format!(
                    "{} spatial audit version {spatial_audit_version} verified {verified_spatial_corrections} of {expected_spatial_corrections} visual and {verified_collision_corrections} of {expected_collision_corrections} collision correction(s)",
                    asset.model_path
                ),
            ));
        }
        // glTF may split or deduplicate vertices at attribute seams, so vertex
        // totals are diagnostic only. Triangle totals remain stable across the
        // native NIF -> glTF conversion and detect dropped geometry.
        if audit.renderable_triangles != source_triangles {
            issues.push(issue_for_asset(
                "visual_topology_mismatch",
                "error",
                asset,
                placements,
                format!(
                    "{} retained {source_meshes} source mesh(es), {source_vertices} vertices, and {source_triangles} triangles but exported {} primitive(s), {} vertices, and {} triangles",
                    asset.model_path,
                    audit.renderable_primitives,
                    audit.renderable_vertices,
                    audit.renderable_triangles,
                ),
            ));
        }

        if audit.renderable_primitives == 0 {
            issues.push(issue_for_asset(
                "no_renderable_primitives",
                "error",
                asset,
                placements,
                format!(
                    "{} produced no non-collision mesh primitive with vertex positions",
                    asset.model_path
                ),
            ));
        }
        if audit.record_zero_non_identity && asset.root_transform_policy.requires_review() {
            issues.push(issue_for_asset(
                "unreviewed_root_transform",
                "warning",
                asset,
                placements,
                format!(
                    "{} preserves a non-identity NIF record-zero transform without a reviewed compatibility policy",
                    asset.model_path
                ),
            ));
        }
    }
    issues.extend(actor_fallback_visual_issues(placements));
    issues.sort_by(|left, right| {
        (&left.model_path, &left.code, &left.message).cmp(&(
            &right.model_path,
            &right.code,
            &right.message,
        ))
    });
    Ok(issues)
}

fn actor_fallback_visual_issues(placements: &[PreparedPlacement]) -> Vec<PreparedVisualIssue> {
    let mut issues = Vec::new();
    for placement in placements {
        let assembly = match &placement.semantic {
            PreparedSemantic::Npc(actor) | PreparedSemantic::Creature(actor) => {
                actor.assembly.as_ref()
            }
            _ => None,
        };
        let Some(assembly) = assembly else {
            continue;
        };
        for reason in &assembly.fallback.reasons {
            let mut base_form_ids =
                vec![assembly.source_base_form_id, assembly.resolved_base_form_id];
            base_form_ids.sort_unstable();
            base_form_ids.dedup();
            issues.push(PreparedVisualIssue {
                code: format!("actor_{}", reason.code()),
                severity: "warning".into(),
                model_path: assembly
                    .skeleton_path
                    .clone()
                    .unwrap_or_else(|| "<actor-proxy>".into()),
                base_form_ids,
                reference_form_ids: vec![placement.reference_form_id],
                message: format!(
                    "actor source={:08x} resolved={:08x} reference={:08x} tier={} reason={} detail={reason:?}",
                    assembly.source_base_form_id,
                    assembly.resolved_base_form_id,
                    placement.reference_form_id,
                    assembly.fallback.level.label(),
                    reason.code(),
                ),
            });
        }
    }
    issues
}

fn issue_for_asset(
    code: &str,
    severity: &str,
    asset: &PreparedVisualAsset,
    placements: &[PreparedPlacement],
    message: String,
) -> PreparedVisualIssue {
    let mut base_form_ids = placements
        .iter()
        .filter(|placement| placement.asset_path.as_deref() == Some(asset.asset_path.as_str()))
        .map(|placement| placement.base_form_id)
        .collect::<Vec<_>>();
    let mut reference_form_ids = placements
        .iter()
        .filter(|placement| placement.asset_path.as_deref() == Some(asset.asset_path.as_str()))
        .map(|placement| placement.reference_form_id)
        .collect::<Vec<_>>();
    base_form_ids.sort_unstable();
    base_form_ids.dedup();
    reference_form_ids.sort_unstable();
    reference_form_ids.dedup();
    PreparedVisualIssue {
        code: code.into(),
        severity: severity.into(),
        model_path: asset.model_path.clone(),
        base_form_ids,
        reference_form_ids,
        message,
    }
}

pub(crate) fn format_visual_issue(issue: &PreparedVisualIssue) -> String {
    let bases = issue
        .base_form_ids
        .iter()
        .map(|form_id| format!("{form_id:08X}"))
        .collect::<Vec<_>>()
        .join(", ");
    let references = issue
        .reference_form_ids
        .iter()
        .map(|form_id| format!("{form_id:08X}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "WARNING [{}] {} (bases: {}; references: {})",
        issue.code, issue.message, bases, references
    )
}

pub(crate) fn enforce_strict_visual_completeness(
    strict: bool,
    unresolved_placements: usize,
    visual_issues: &[PreparedVisualIssue],
) -> Result<()> {
    if strict && (unresolved_placements > 0 || !visual_issues.is_empty()) {
        bail!(
            "strict preparation failed with {unresolved_placements} unresolved placement(s) and {} visual completeness issue(s)",
            visual_issues.len()
        )
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests/visual.rs"]
mod tests;
