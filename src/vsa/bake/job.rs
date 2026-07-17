use super::*;
use crate::vsa::dynamic_lighting::{
    DynamicLightIlluminationMode, DynamicLightShadowMode, DynamicLightTransparencyMode,
};

const CELL_DIRECTIONAL_ILLUMINANCE: f32 = 10_000.0;

pub(crate) fn cell_directional_illuminance(lighting: &PreparedCellLighting) -> f32 {
    let luminance =
        lighting.directional_rgba[0] + lighting.directional_rgba[1] + lighting.directional_rgba[2];
    if !luminance.is_finite() || luminance <= f32::EPSILON {
        0.0
    } else {
        CELL_DIRECTIONAL_ILLUMINANCE
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct BakeJob {
    pub(crate) asset_root: String,
    pub(crate) output_scene: String,
    pub(crate) preview_output: String,
    pub(crate) result_json: String,
    pub(crate) irradiance_blend: String,
    pub(crate) irradiance_spacing_meters: f32,
    pub(crate) irradiance_samples: u32,
    pub(crate) dynamic_lighting_texels_per_meter: u32,
    pub(crate) dynamic_lighting_max_lightmap_size: u32,
    pub(crate) dynamic_lighting_bounce_samples: u32,
    pub(crate) dynamic_lighting_bounce_compression: u8,
    pub(crate) preview_only: bool,
    pub(crate) static_batch_chunk_meters: f32,
    pub(crate) emission_scale: f32,
    pub(crate) ambient_rgba: [f32; 4],
    pub(crate) cell_directional_rgba: [f32; 4],
    pub(crate) cell_directional_rotation_xyzw: [f32; 4],
    pub(crate) cell_directional_illuminance: f32,
    pub(crate) placements: Vec<JobPlacement>,
    pub(crate) lights: Vec<JobLight>,
}

#[derive(Debug, Serialize)]
pub(crate) struct JobPlacement {
    pub(crate) reference_form_id: u32,
    pub(crate) asset_path: String,
    pub(crate) ao_mode: String,
    pub(crate) batchable_static: bool,
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: f32,
}

#[derive(Debug, Serialize)]
pub(crate) struct JobLight {
    pub(crate) reference_form_id: u32,
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) color_rgba: [f32; 4],
    pub(crate) radius: f32,
    pub(crate) intensity_lumens: f32,
    pub(crate) kind: String,
    /// Default-on indirect contribution for this light's irradiance bake.
    pub(crate) bounce_multiplier: f32,
    pub(crate) shadow_mode: DynamicLightShadowMode,
    pub(crate) illumination_mode: DynamicLightIlluminationMode,
    pub(crate) transparency_mode: DynamicLightTransparencyMode,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum KtxToolKind {
    LegacyToktx,
    UnifiedKtx,
}

#[derive(Debug, Clone)]
pub(crate) struct KtxTool {
    pub(crate) path: PathBuf,
    pub(crate) kind: KtxToolKind,
}
