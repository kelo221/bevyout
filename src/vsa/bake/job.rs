use super::*;

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
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) color_rgba: [f32; 4],
    pub(crate) radius: f32,
    pub(crate) intensity_lumens: f32,
    pub(crate) kind: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BlenderBakeResult {
    pub(crate) irradiance: Option<BlenderIrradianceResult>,
    pub(crate) batching: BlenderBatchingStats,
    pub(crate) placement_contribution: BlenderPlacementContribution,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BlenderPlacementContribution {
    pub(crate) expected_placements: usize,
    pub(crate) contributed_placements: usize,
    pub(crate) reference_form_ids: Vec<u32>,
    pub(crate) post_batch_verified: bool,
    pub(crate) placements: Vec<BlenderPlacementGeometry>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BlenderPlacementGeometry {
    pub(crate) reference_form_id: u32,
    pub(crate) visual_meshes: usize,
    pub(crate) vertices: usize,
    pub(crate) triangles: usize,
    pub(crate) world_bounds_min: [f32; 3],
    pub(crate) world_bounds_max: [f32; 3],
}

#[derive(Debug, Deserialize)]
pub(crate) struct BlenderIrradianceResult {
    pub(crate) blend_path: String,
    pub(crate) resolution: [u32; 3],
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: [f32; 3],
}

#[derive(Debug, Deserialize)]
pub(crate) struct BlenderBatchingStats {
    pub(crate) chunk_size_meters: f32,
    pub(crate) visual_objects_before: usize,
    pub(crate) visual_objects_after: usize,
    pub(crate) render_primitives_before: usize,
    pub(crate) render_primitives_after: usize,
    pub(crate) materials_before: usize,
    pub(crate) materials_after: usize,
    pub(crate) batches_created: usize,
    pub(crate) largest_batch: usize,
    pub(crate) excluded_collision: usize,
    pub(crate) excluded_large: usize,
    pub(crate) excluded_non_static: usize,
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
