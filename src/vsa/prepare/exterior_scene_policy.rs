//! Pure storage policy for compact exterior scene roots.
//!
//! The full exterior package is already published under `worldspaces/`.
//! Embedding it again in `scenes/<cell>/scene.ron` duplicates cell data, and
//! embedding content-set diagnostics in every scene multiplies a single
//! diagnostic report by the exterior cell count.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExteriorSceneStoragePlan {
    pub(crate) package_path: String,
    pub(crate) embed_package: bool,
    pub(crate) embed_content_diagnostics: bool,
}

pub(crate) fn exterior_scene_storage_plan(
    worldspace_form_id: u32,
    cell_form_id: u32,
) -> ExteriorSceneStoragePlan {
    ExteriorSceneStoragePlan {
        package_path: format!("worldspaces/{worldspace_form_id:08x}/cells/{cell_form_id:08x}.ron"),
        embed_package: false,
        embed_content_diagnostics: false,
    }
}
