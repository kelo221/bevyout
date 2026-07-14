//! Converted asset cache validation.

use super::*;

fn read_glb_document(path: &Path) -> Result<(Vec<u8>, serde_json::Value, usize)> {
    let bytes = fs::read(path)?;
    if bytes.len() < 20 || &bytes[0..4] != b"glTF" {
        bail!("invalid GLB header")
    }
    let json_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let json_start: usize = 20;
    let json_end = json_start
        .checked_add(json_length)
        .context("GLB JSON chunk length overflows")?;
    if json_end > bytes.len() {
        bail!("GLB JSON chunk extends beyond file")
    }
    let document: serde_json::Value = serde_json::from_slice(&bytes[json_start..json_end])?;
    Ok((bytes, document, json_end))
}

pub(crate) fn validate_glb_images(path: &Path) -> Result<()> {
    let (bytes, document, json_end) = read_glb_document(path)?;
    let views = document
        .get("bufferViews")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let images = document
        .get("images")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let binary_start = json_end
        .checked_add(8)
        .context("GLB binary chunk header overflows")?;
    for image in images {
        let Some(view_index) = image.get("bufferView").and_then(serde_json::Value::as_u64) else {
            continue;
        };
        let Some(view) = views.get(view_index as usize) else {
            bail!("image references missing bufferView")
        };
        let offset = view
            .get("byteOffset")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0) as usize;
        let length = view
            .get("byteLength")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0) as usize;
        let data_start = binary_start
            .checked_add(offset)
            .context("image buffer offset overflows")?;
        let end = data_start
            .checked_add(length)
            .context("image buffer length overflows")?;
        if end > bytes.len() {
            bail!("image bufferView extends beyond GLB")
        }
        let data = &bytes[data_start..end];
        if data.starts_with(b"\x89PNG\r\n\x1a\n") && data.len() >= 24 {
            let width = u32::from_be_bytes(data[16..20].try_into().unwrap());
            let height = u32::from_be_bytes(data[20..24].try_into().unwrap());
            if width <= 1 || height <= 1 {
                let name = image
                    .get("name")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("unnamed");
                bail!("image {name} is a 1x1 placeholder")
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GlbVisualAudit {
    pub(crate) renderable_primitives: usize,
    pub(crate) renderable_vertices: usize,
    pub(crate) renderable_triangles: usize,
    pub(crate) source_render_meshes: Option<usize>,
    pub(crate) source_render_vertices: Option<usize>,
    pub(crate) source_render_triangles: Option<usize>,
    pub(crate) spatial_audit_version: Option<usize>,
    pub(crate) expected_spatial_corrections: Option<usize>,
    pub(crate) verified_spatial_corrections: Option<usize>,
    pub(crate) expected_collision_corrections: Option<usize>,
    pub(crate) verified_collision_corrections: Option<usize>,
    pub(crate) source_model: Option<String>,
    pub(crate) root_transform_policy: Option<String>,
    pub(crate) record_zero_non_identity: bool,
}

pub(crate) fn audit_glb_visuals(path: &Path) -> Result<GlbVisualAudit> {
    let (_, document, _) = read_glb_document(path)?;
    let accessors = document
        .get("accessors")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let meshes = document
        .get("meshes")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut renderable_primitives = 0;
    let mut renderable_vertices = 0;
    let mut renderable_triangles = 0;
    let mut source_render_meshes = None;
    let mut source_render_vertices = None;
    let mut source_render_triangles = None;
    let mut spatial_audit_version = None;
    let mut expected_spatial_corrections = None;
    let mut verified_spatial_corrections = None;
    let mut expected_collision_corrections = None;
    let mut verified_collision_corrections = None;
    let mut source_model = None;
    let mut root_transform_policy = None;
    let mut record_zero_non_identity = false;

    for node in document
        .get("nodes")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
    {
        let extras = node.get("extras");
        if source_model.is_none() {
            source_model = extras
                .and_then(|value| value.get("bevyout_source_model"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
        }
        if root_transform_policy.is_none() {
            root_transform_policy = extras
                .and_then(|value| value.get("bevyout_root_transform_policy"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
        }
        if source_render_meshes.is_none() {
            source_render_meshes = extras
                .and_then(|value| value.get("bevyout_source_render_meshes"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok());
            source_render_vertices = extras
                .and_then(|value| value.get("bevyout_source_render_vertices"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok());
            source_render_triangles = extras
                .and_then(|value| value.get("bevyout_source_render_triangles"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok());
            spatial_audit_version = extras
                .and_then(|value| value.get("bevyout_spatial_audit_version"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok());
            expected_spatial_corrections = extras
                .and_then(|value| value.get("bevyout_expected_spatial_corrections"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok());
            verified_spatial_corrections = extras
                .and_then(|value| value.get("bevyout_verified_spatial_corrections"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok());
            expected_collision_corrections = extras
                .and_then(|value| value.get("bevyout_expected_collision_corrections"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok());
            verified_collision_corrections = extras
                .and_then(|value| value.get("bevyout_verified_collision_corrections"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok());
        }
        record_zero_non_identity |= extras
            .and_then(|value| value.get("bevyout_record_zero_non_identity"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);

        if extras
            .and_then(|value| value.get("bevyout_collision"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            continue;
        }
        let Some(mesh_index) = node.get("mesh").and_then(serde_json::Value::as_u64) else {
            continue;
        };
        let Some(mesh) = meshes.get(mesh_index as usize) else {
            continue;
        };
        for primitive in mesh
            .get("primitives")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
        {
            let Some(position_index) = primitive
                .get("attributes")
                .and_then(|value| value.get("POSITION"))
                .and_then(serde_json::Value::as_u64)
            else {
                continue;
            };
            let vertex_count = accessors
                .get(position_index as usize)
                .and_then(|accessor| accessor.get("count"))
                .and_then(serde_json::Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
                .unwrap_or(0);
            if vertex_count > 0 {
                renderable_primitives += 1;
                renderable_vertices += vertex_count;
                let element_count = primitive
                    .get("indices")
                    .and_then(serde_json::Value::as_u64)
                    .and_then(|index| accessors.get(index as usize))
                    .and_then(|accessor| accessor.get("count"))
                    .and_then(serde_json::Value::as_u64)
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or(vertex_count);
                if primitive
                    .get("mode")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(4)
                    == 4
                {
                    renderable_triangles += element_count / 3;
                }
            }
        }
    }

    Ok(GlbVisualAudit {
        renderable_primitives,
        renderable_vertices,
        renderable_triangles,
        source_render_meshes,
        source_render_vertices,
        source_render_triangles,
        spatial_audit_version,
        expected_spatial_corrections,
        verified_spatial_corrections,
        expected_collision_corrections,
        verified_collision_corrections,
        source_model,
        root_transform_policy,
        record_zero_non_identity,
    })
}

pub(crate) fn validate_asset_cache_pair(glb: &Path, physics: &Path) -> Result<()> {
    validate_glb_images(glb)
        .with_context(|| format!("cached GLB is invalid: {}", glb.display()))?;
    read_physics_asset(physics)
        .with_context(|| format!("cached physics sidecar is invalid: {}", physics.display()))?;
    Ok(())
}

pub(crate) fn blender_jobs_json(jobs: &[BlenderAssetJob]) -> String {
    serde_json::Value::Array(
        jobs.iter()
            .map(|job| {
                serde_json::json!({
                    "input": job.input.to_string_lossy(),
                    "output": job.output.to_string_lossy(),
                    "physics_output": job.physics_output.to_string_lossy(),
                    "model": job.model,
                    "conversion": job.conversion.profile_tag(),
                    "root_transform_policy": job.root_transform_policy.tag(),
                })
            })
            .collect(),
    )
    .to_string()
}
