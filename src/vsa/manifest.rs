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
    pub(crate) translation: [f32; 3],
    pub(crate) color_rgba: [f32; 4],
    pub(crate) radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Diagnostic {
    pub(crate) severity: String,
    pub(crate) message: String,
}
