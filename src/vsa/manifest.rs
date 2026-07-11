use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub(crate) struct PreparedSceneManifest {
    pub(crate) schema_version: u32,
    pub(crate) asset_root: String,
    pub(crate) source_plugin: String,
    pub(crate) source_fingerprint: String,
    pub(crate) cell: CellInfo,
    pub(crate) placements: Vec<PreparedPlacement>,
    pub(crate) lights: Vec<PreparedLight>,
    pub(crate) diagnostics: Vec<Diagnostic>,
    #[serde(default)]
    pub(crate) bake: Option<PreparedBake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedBake {
    pub(crate) source_fingerprint: String,
    pub(crate) scene_path: String,
    pub(crate) lightmaps: Vec<PreparedLightmapPage>,
    pub(crate) bindings: Vec<PreparedLightmapBinding>,
    pub(crate) lightmap_exposure: f32,
    /// Quick/direct bakes still need the runtime ambient and point lights to
    /// provide the missing indirect fill. Final bakes can disable them on
    /// lightmapped meshes because they contain the complete transport.
    #[serde(default)]
    pub(crate) runtime_lighting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedLightmapPage {
    pub(crate) asset_path: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedLightmapBinding {
    pub(crate) mesh_name: String,
    pub(crate) page: usize,
    pub(crate) uv_rect: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CellInfo {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) interior: bool,
    pub(crate) ambient_rgba: [f32; 4],
    pub(crate) directional_rgba: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedPlacement {
    pub(crate) reference_form_id: u32,
    pub(crate) base_form_id: u32,
    pub(crate) asset_path: Option<String>,
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: f32,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedLight {
    #[serde(default)]
    pub(crate) reference_form_id: u32,
    #[serde(default)]
    pub(crate) base_form_id: u32,
    pub(crate) translation: [f32; 3],
    #[serde(default = "default_rotation")]
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) color_rgba: [f32; 4],
    pub(crate) radius: f32,
    #[serde(default)]
    pub(crate) intensity_lumens: f32,
    #[serde(default)]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) flags: u32,
}

fn default_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Diagnostic {
    pub(crate) severity: String,
    pub(crate) message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_one_manifests_remain_readable_without_bake_metadata() {
        let text = r#"(
            schema_version: 1,
            asset_root: "cache",
            source_plugin: "Fallout3.esm",
            source_fingerprint: "fingerprint",
            cell: (
                form_id: 1,
                editor_id: None,
                name: None,
                interior: true,
                ambient_rgba: (0.0, 0.0, 0.0, 0.0),
                directional_rgba: (0.0, 0.0, 0.0, 0.0),
            ),
            placements: [],
            lights: [],
            diagnostics: [],
        )"#;
        let manifest: PreparedSceneManifest = ron::de::from_str(text).unwrap();
        assert_eq!(manifest.schema_version, 1);
        assert!(manifest.bake.is_none());
    }
}
