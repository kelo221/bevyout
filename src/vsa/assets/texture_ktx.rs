//! Shared KTX2 texture encoding and embedded-GLB rewriting.

use super::*;
use std::ffi::OsString;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) const KTX2_IDENTIFIER: &[u8; 12] = b"\xABKTX 20\xBB\r\n\x1A\n";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextureColorSpace {
    Srgb,
    Linear,
}

pub(crate) fn find_texture_ktx_tool() -> Result<PathBuf> {
    for path in [
        PathBuf::from("ktx"),
        PathBuf::from(r"C:\Program Files\KTX-Software\bin\ktx.exe"),
    ] {
        if (path.is_absolute() && path.exists())
            || (!path.is_absolute()
                && Command::new(&path)
                    .arg("--version")
                    .output()
                    .is_ok_and(|output| output.status.success()))
        {
            return Ok(path);
        }
    }
    bail!("KTX-Software was not found; texture preparation requires the unified ktx executable")
}

pub(crate) fn ktx_create_arguments(
    input: &Path,
    output: &Path,
    color_space: TextureColorSpace,
) -> Vec<OsString> {
    let format = match color_space {
        TextureColorSpace::Srgb => "R8G8B8A8_SRGB",
        TextureColorSpace::Linear => "R8G8B8A8_UNORM",
    };
    [
        "create",
        "--format",
        format,
        "--encode",
        "uastc",
        "--zstd",
        "9",
        "--generate-mipmap",
    ]
    .into_iter()
    .map(OsString::from)
    .chain([input.as_os_str().to_owned(), output.as_os_str().to_owned()])
    .collect()
}

pub(crate) fn encode_texture_to_ktx2(
    source_bytes: &[u8],
    color_space: TextureColorSpace,
) -> Result<Vec<u8>> {
    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);
    let tool = find_texture_ktx_tool()?;
    let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!("bevyout-ktx-{}-{sequence}", std::process::id()));
    fs::create_dir_all(&root)?;
    let input = root.join("input.png");
    let output = root.join("output.ktx2");
    let result = (|| {
        let image = image::load_from_memory(source_bytes).context("decoding source texture")?;
        image
            .save_with_format(&input, image::ImageFormat::Png)
            .context("writing temporary KTX source image")?;
        let command = Command::new(&tool)
            .args(ktx_create_arguments(&input, &output, color_space))
            .output()
            .context("failed to start KTX-Software")?;
        if !command.status.success() {
            bail!(
                "KTX-Software failed with {}:\n{}\n{}",
                command.status,
                String::from_utf8_lossy(&command.stdout).trim(),
                String::from_utf8_lossy(&command.stderr).trim()
            );
        }
        let bytes = fs::read(&output).context("reading encoded KTX2 texture")?;
        validate_ktx2_payload(&bytes)?;
        Ok(bytes)
    })();
    let _ = fs::remove_dir_all(&root);
    result
}

pub(crate) fn validate_ktx2_payload(bytes: &[u8]) -> Result<(u32, u32)> {
    if bytes.len() < 48 || !bytes.starts_with(KTX2_IDENTIFIER) {
        bail!("invalid KTX2 identifier or truncated header");
    }
    let width = u32::from_le_bytes(bytes[20..24].try_into().unwrap());
    let height = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
    let levels = u32::from_le_bytes(bytes[40..44].try_into().unwrap());
    if width == 0 || height == 0 || levels == 0 {
        bail!("KTX2 texture has invalid dimensions or no mip levels");
    }
    if width <= 1 || height <= 1 {
        bail!("KTX2 texture is a 1x1 placeholder");
    }
    Ok((width, height))
}

pub(crate) fn transcode_glb_images_to_ktx2(bytes: &[u8]) -> Result<Vec<u8>> {
    if bytes.len() < 28 || &bytes[0..4] != b"glTF" {
        bail!("KTX2 transcode received an invalid GLB header");
    }
    let json_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let json_end = 20usize
        .checked_add(json_length)
        .context("GLB JSON length overflow")?;
    if json_end + 8 > bytes.len() || &bytes[16..20] != b"JSON" {
        bail!("KTX2 transcode received an invalid GLB JSON chunk");
    }
    let binary_length =
        u32::from_le_bytes(bytes[json_end..json_end + 4].try_into().unwrap()) as usize;
    if &bytes[json_end + 4..json_end + 8] != b"BIN\0" {
        bail!("KTX2 transcode received a GLB without a BIN chunk");
    }
    let binary_start = json_end + 8;
    if binary_start + binary_length > bytes.len() {
        bail!("GLB BIN chunk extends beyond the file");
    }

    let mut document: serde_json::Value =
        serde_json::from_slice(&bytes[20..json_end]).context("decoding GLB JSON")?;
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
    if images.is_empty() {
        return Ok(bytes.to_vec());
    }

    let mut srgb_images = HashSet::new();
    let texture_sources = document
        .get("textures")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut mark_texture = |value: Option<&serde_json::Value>| {
        let Some(index) = value
            .and_then(|texture| texture.get("index"))
            .and_then(serde_json::Value::as_u64)
        else {
            return;
        };
        if let Some(source) = texture_sources
            .get(index as usize)
            .and_then(|texture| texture.get("source"))
            .and_then(serde_json::Value::as_u64)
        {
            srgb_images.insert(source as usize);
        }
    };
    for material in document
        .get("materials")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
    {
        mark_texture(
            material
                .get("pbrMetallicRoughness")
                .and_then(|pbr| pbr.get("baseColorTexture")),
        );
        mark_texture(material.get("emissiveTexture"));
    }

    let binary = &bytes[binary_start..binary_start + binary_length];
    let mut replacements = std::collections::HashMap::new();
    for (image_index, image) in images.iter().enumerate() {
        let view_index = image
            .get("bufferView")
            .and_then(serde_json::Value::as_u64)
            .with_context(|| format!("embedded image {image_index} has no bufferView"))?
            as usize;
        let view = views
            .get(view_index)
            .with_context(|| format!("embedded image {image_index} has an invalid bufferView"))?;
        let offset = view
            .get("byteOffset")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0) as usize;
        let length = view
            .get("byteLength")
            .and_then(serde_json::Value::as_u64)
            .context("embedded image bufferView has no byteLength")? as usize;
        let source = binary
            .get(offset..offset + length)
            .context("embedded image extends beyond the GLB BIN chunk")?;
        let color_space = if srgb_images.contains(&image_index) {
            TextureColorSpace::Srgb
        } else {
            TextureColorSpace::Linear
        };
        replacements.insert(view_index, encode_texture_to_ktx2(source, color_space)?);
    }

    let mut rebuilt_binary = Vec::new();
    let document_views = document
        .get_mut("bufferViews")
        .and_then(serde_json::Value::as_array_mut)
        .context("GLB has no mutable bufferViews array")?;
    for (index, view) in document_views.iter_mut().enumerate() {
        while !rebuilt_binary.len().is_multiple_of(4) {
            rebuilt_binary.push(0);
        }
        let old = &views[index];
        let offset = old
            .get("byteOffset")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0) as usize;
        let length = old
            .get("byteLength")
            .and_then(serde_json::Value::as_u64)
            .context("bufferView has no byteLength")? as usize;
        let payload = replacements
            .get(&index)
            .map(Vec::as_slice)
            .unwrap_or_else(|| &binary[offset..offset + length]);
        view["byteOffset"] = serde_json::Value::from(rebuilt_binary.len());
        view["byteLength"] = serde_json::Value::from(payload.len());
        rebuilt_binary.extend_from_slice(payload);
    }
    while !rebuilt_binary.len().is_multiple_of(4) {
        rebuilt_binary.push(0);
    }
    document["buffers"][0]["byteLength"] = serde_json::Value::from(rebuilt_binary.len());
    if let Some(document_images) = document
        .get_mut("images")
        .and_then(serde_json::Value::as_array_mut)
    {
        for image in document_images {
            image["mimeType"] = serde_json::Value::from("image/ktx2");
        }
    }

    let mut json = serde_json::to_vec(&document)?;
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let total_length = 20 + json.len() + 8 + rebuilt_binary.len();
    let mut result = Vec::with_capacity(total_length);
    result.extend_from_slice(b"glTF");
    result.extend_from_slice(&2u32.to_le_bytes());
    result.extend_from_slice(&u32::try_from(total_length)?.to_le_bytes());
    result.extend_from_slice(&u32::try_from(json.len())?.to_le_bytes());
    result.extend_from_slice(b"JSON");
    result.extend_from_slice(&json);
    result.extend_from_slice(&u32::try_from(rebuilt_binary.len())?.to_le_bytes());
    result.extend_from_slice(b"BIN\0");
    result.extend_from_slice(&rebuilt_binary);
    gltf::Gltf::from_slice(&result).context("validating KTX2-rewritten GLB")?;
    Ok(result)
}

#[cfg(test)]
#[path = "tests/texture_ktx.rs"]
mod tests;
