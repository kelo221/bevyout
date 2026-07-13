//! Prepared navmesh staging.

use super::*;

pub(crate) fn stage_navmeshes(
    scene_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
    navmeshes: &[crate::vsa::openmw_esm4::NavMeshRecord],
) -> Result<Vec<PreparedNavMeshSource>> {
    if navmeshes.is_empty() {
        return Ok(Vec::new());
    }
    let navmesh_dir = scene_dir.join("navmesh");
    fs::create_dir_all(&navmesh_dir)?;
    navmeshes
        .iter()
        .map(|navmesh| {
            let filename = format!("{:08x}.navm.bin", navmesh.form_id);
            fs::write(navmesh_dir.join(&filename), &navmesh.payload)?;
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "retained FO3 NAVM {:08x} source metadata; runtime navigation is not decoded",
                    navmesh.form_id
                ),
            });
            Ok(PreparedNavMeshSource {
                form_id: navmesh.form_id,
                record_flags: navmesh.flags,
                version: navmesh.version,
                asset_path: format!(
                    "scenes/{}/navmesh/{filename}",
                    scene_dir
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or_default()
                ),
                chunks: navmesh
                    .chunks
                    .iter()
                    .map(|chunk| PreparedNavMeshChunk {
                        signature: chunk.signature.clone(),
                        byte_len: chunk.byte_len,
                    })
                    .collect(),
            })
        })
        .collect()
}
