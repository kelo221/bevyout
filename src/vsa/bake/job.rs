use super::*;
use bevyout_core::lighting::{
    AUTHORED_LIGHTING_SCALE, cell_directional_illuminance as resolve_cell_directional_illuminance,
    point_light_intensity,
};
use std::collections::BTreeMap;

pub(crate) fn cell_directional_illuminance(lighting: &PreparedCellLighting) -> f32 {
    resolve_cell_directional_illuminance(lighting.directional_rgba)
}

pub(crate) fn authored_point_light_intensity(radius: f32, intensity_lumens: f32) -> f32 {
    point_light_intensity(radius, intensity_lumens, AUTHORED_LIGHTING_SCALE)
}

#[derive(Debug, Serialize)]
pub(crate) struct BakeJob {
    pub(crate) asset_root: String,
    pub(crate) output_scene: String,
    pub(crate) irradiance_spacing_meters: f32,
    pub(crate) irradiance_samples: u32,
    pub(crate) lightmap_min_samples: u32,
    pub(crate) lightmap_max_samples: u32,
    pub(crate) lightmap_variance_threshold: f32,
    pub(crate) lightmap_bounces: u32,
    pub(crate) lightmap_texels_per_meter: f32,
    pub(crate) lightmap_density_overrides: BTreeMap<u32, f32>,
    pub(crate) lightmap_denoise_iterations: u32,
    pub(crate) lightmap_tile_size: u32,
    pub(crate) lightmap_backend: String,
    pub(crate) static_batch_chunk_meters: f32,
    pub(crate) ambient_rgba: [f32; 4],
    pub(crate) lightmap_environment_map: Option<String>,
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
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) color_rgba: [f32; 4],
    pub(crate) radius: f32,
    pub(crate) intensity_lumens: f32,
    pub(crate) kind: String,
    pub(crate) flags: u32,
    pub(crate) spot_fov_radians: f32,
    pub(crate) spot_falloff_exponent: f32,
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
