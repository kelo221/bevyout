//! Converted asset cache validation.

use super::*;

pub(crate) fn validate_glb_images(path: &Path) -> Result<()> {
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
        if data.starts_with(b"\\x89PNG\\r\\n\\x1a\\n") && data.len() >= 24 {
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

pub(crate) fn validate_asset_cache_pair(glb: &Path, physics: &Path) -> Result<()> {
    validate_glb_images(glb)
        .with_context(|| format!("cached GLB is invalid: {}", glb.display()))?;
    read_physics_asset(physics)
        .with_context(|| format!("cached physics sidecar is invalid: {}", physics.display()))?;
    Ok(())
}

pub(crate) fn blender_jobs_json(jobs: &[BlenderAssetJob]) -> String {
    let mut out = String::from("[");
    for (index, job) in jobs.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"input\":\"{}\",\"output\":\"{}\",\"physics_output\":\"{}\",\"model\":\"{}\",\"conversion\":\"{}\"}}",
            json_escape(&job.input.to_string_lossy()),
            json_escape(&job.output.to_string_lossy()),
            json_escape(&job.physics_output.to_string_lossy()),
            json_escape(&job.model),
            job.conversion.profile_tag(),
        ));
    }
    out.push(']');
    out
}

fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
