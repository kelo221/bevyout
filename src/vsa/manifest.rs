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
    #[serde(default)]
    pub(crate) image_space_form_id: Option<u32>,
    #[serde(default)]
    pub(crate) image_space: Option<ImageSpaceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct ImageSpaceInfo {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) eye_adapt_speed: f32,
    pub(crate) hdr_blur_radius: f32,
    pub(crate) hdr_blur_passes: f32,
    pub(crate) hdr_emissive_multiplier: f32,
    pub(crate) hdr_target_lum: f32,
    pub(crate) hdr_upper_lum_clamp: f32,
    pub(crate) hdr_bright_scale: f32,
    pub(crate) hdr_bright_clamp: f32,
    pub(crate) hdr_lum_ramp_no_tex: f32,
    pub(crate) hdr_lum_ramp_min: f32,
    pub(crate) hdr_lum_ramp_max: f32,
    pub(crate) hdr_sunlight_dimmer: f32,
    pub(crate) hdr_grass_dimmer: f32,
    pub(crate) hdr_tree_dimmer: f32,
    pub(crate) hdr_skin_dimmer: f32,
    pub(crate) bloom_blur_radius: f32,
    pub(crate) bloom_alpha_mult_interior: f32,
    pub(crate) bloom_alpha_mult_exterior: f32,
    pub(crate) get_hit_blur_radius: f32,
    pub(crate) get_hit_blur_damping_constant: f32,
    pub(crate) get_hit_damping_constant: f32,
    pub(crate) night_eye_tint_rgb: [f32; 3],
    pub(crate) brightness: f32,
    pub(crate) cinematic_saturation: f32,
    pub(crate) cinematic_contrast_avg_lum: f32,
    pub(crate) cinematic_contrast: f32,
    pub(crate) cinematic_brightness_tint_rgb: [f32; 3],
    pub(crate) cinematic_brightness_tint_value: f32,
    pub(crate) flags: u8,
}

impl Default for ImageSpaceInfo {
    fn default() -> Self {
        Self {
            form_id: 0,
            editor_id: None,
            eye_adapt_speed: 0.5,
            hdr_blur_radius: 7.0,
            hdr_blur_passes: 1.0,
            hdr_emissive_multiplier: 1.0,
            hdr_target_lum: 1.0,
            hdr_upper_lum_clamp: 1.0,
            hdr_bright_scale: 1.0,
            hdr_bright_clamp: 0.225,
            hdr_lum_ramp_no_tex: 1.0,
            hdr_lum_ramp_min: 0.0,
            hdr_lum_ramp_max: 0.0,
            hdr_sunlight_dimmer: 1.0,
            hdr_grass_dimmer: 1.0,
            hdr_tree_dimmer: 1.0,
            hdr_skin_dimmer: 1.0,
            bloom_blur_radius: 0.0,
            bloom_alpha_mult_interior: 1.0,
            bloom_alpha_mult_exterior: 1.0,
            get_hit_blur_radius: 0.0,
            get_hit_blur_damping_constant: 0.0,
            get_hit_damping_constant: 0.0,
            night_eye_tint_rgb: [0.0, 0.0, 0.0],
            brightness: 1.0,
            cinematic_saturation: 1.0,
            cinematic_contrast_avg_lum: 0.5,
            cinematic_contrast: 1.0,
            cinematic_brightness_tint_rgb: [1.0, 1.0, 1.0],
            cinematic_brightness_tint_value: 0.0,
            flags: 0,
        }
    }
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
