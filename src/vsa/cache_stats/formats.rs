use super::{
    model::{GlbFileStats, GlbSummary, TextureReport},
    scan::ScannedFile,
};
use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

const GLB_MAGIC: &[u8; 4] = b"glTF";
const GLB_JSON_CHUNK: u32 = 0x4e4f_534a;
const GLB_BINARY_CHUNK: u32 = 0x004e_4942;

#[derive(Debug, Clone, Copy, Default)]
struct BufferView {
    offset: u64,
    length: u64,
}

pub(super) fn inspect_formats(
    files: &[ScannedFile],
) -> (
    Vec<Option<GlbFileStats>>,
    GlbSummary,
    Vec<TextureReport>,
    Vec<String>,
) {
    let mut per_file = vec![None; files.len()];
    let mut summary = GlbSummary::default();
    let mut textures = Vec::new();
    let mut diagnostics = Vec::new();

    for (index, file) in files.iter().enumerate() {
        let lower = file.relative_path.to_ascii_lowercase();
        if lower.ends_with(".glb") {
            summary.file_count += 1;
            summary.logical_bytes = summary.logical_bytes.saturating_add(file.logical_bytes);
            match inspect_glb(file) {
                Ok((stats, mut embedded, mut warnings)) => {
                    add_glb_stats(&mut summary, &stats);
                    textures.append(&mut embedded);
                    diagnostics.append(&mut warnings);
                    per_file[index] = Some(stats);
                }
                Err(error) => {
                    summary.parse_failures += 1;
                    diagnostics.push(format!("GLB {}: {error:#}", file.relative_path));
                }
            }
        } else if lower.ends_with(".ktx2") {
            match File::open(&file.absolute_path).and_then(|mut source| {
                inspect_ktx(
                    &mut source,
                    0,
                    file.logical_bytes,
                    file.relative_path.clone(),
                    "external".into(),
                    infer_texture_role(&file.relative_path).into(),
                )
                .map_err(std::io::Error::other)
            }) {
                Ok(texture) => textures.push(texture),
                Err(error) => diagnostics.push(format!("KTX2 {}: {error}", file.relative_path)),
            }
        }
    }
    textures.sort_by(|left, right| left.location.cmp(&right.location));
    diagnostics.sort();
    (per_file, summary, textures, diagnostics)
}

fn add_glb_stats(summary: &mut GlbSummary, stats: &GlbFileStats) {
    summary.json_bytes = summary.json_bytes.saturating_add(stats.json_bytes);
    summary.binary_bytes = summary.binary_bytes.saturating_add(stats.binary_bytes);
    summary.geometry_bytes = summary.geometry_bytes.saturating_add(stats.geometry_bytes);
    summary.animation_bytes = summary
        .animation_bytes
        .saturating_add(stats.animation_bytes);
    summary.embedded_image_bytes = summary
        .embedded_image_bytes
        .saturating_add(stats.embedded_image_bytes);
    summary.embedded_ktx2_bytes = summary
        .embedded_ktx2_bytes
        .saturating_add(stats.embedded_ktx2_bytes);
    summary.other_buffer_bytes = summary
        .other_buffer_bytes
        .saturating_add(stats.other_buffer_bytes);
    summary.padding_bytes = summary.padding_bytes.saturating_add(stats.padding_bytes);
}

fn inspect_glb(scanned: &ScannedFile) -> Result<(GlbFileStats, Vec<TextureReport>, Vec<String>)> {
    let mut file = File::open(&scanned.absolute_path)
        .with_context(|| format!("could not open {}", scanned.absolute_path.display()))?;
    let mut header = [0u8; 12];
    file.read_exact(&mut header)?;
    if &header[..4] != GLB_MAGIC {
        bail!("invalid GLB magic");
    }
    let version = u32::from_le_bytes(header[4..8].try_into().unwrap());
    if version != 2 {
        bail!("unsupported GLB version {version}");
    }
    let declared_length = u64::from(u32::from_le_bytes(header[8..12].try_into().unwrap()));
    if declared_length > scanned.logical_bytes {
        bail!(
            "declared length {declared_length} exceeds file length {}",
            scanned.logical_bytes
        );
    }

    let mut cursor = 12u64;
    let mut json_bytes = None;
    let mut json_chunk_bytes = 0u64;
    let mut binary_bytes = 0u64;
    let mut first_binary_offset = None;
    while cursor.saturating_add(8) <= declared_length {
        file.seek(SeekFrom::Start(cursor))?;
        let mut chunk_header = [0u8; 8];
        file.read_exact(&mut chunk_header)?;
        let length = u64::from(u32::from_le_bytes(chunk_header[..4].try_into().unwrap()));
        let kind = u32::from_le_bytes(chunk_header[4..8].try_into().unwrap());
        let data_offset = cursor + 8;
        let end = data_offset
            .checked_add(length)
            .ok_or_else(|| anyhow!("GLB chunk length overflow"))?;
        if end > declared_length {
            bail!("GLB chunk exceeds declared file length");
        }
        match kind {
            GLB_JSON_CHUNK if json_bytes.is_none() => {
                let length = usize::try_from(length).context("GLB JSON chunk is too large")?;
                let mut bytes = vec![0u8; length];
                file.read_exact(&mut bytes)?;
                json_chunk_bytes = bytes.len() as u64;
                json_bytes = Some(bytes);
            }
            GLB_BINARY_CHUNK => {
                first_binary_offset.get_or_insert(data_offset);
                binary_bytes = binary_bytes.saturating_add(length);
            }
            _ => {}
        }
        cursor = end;
    }

    let json_bytes = json_bytes.ok_or_else(|| anyhow!("GLB has no JSON chunk"))?;
    let json: Value = serde_json::from_slice(&json_bytes).context("invalid GLB JSON")?;
    let views = parse_buffer_views(&json);
    let image_views = image_buffer_views(&json);
    let animation_views = animation_buffer_views(&json);
    let geometry_views = geometry_buffer_views(&json);

    let mut stats = GlbFileStats {
        json_bytes: json_chunk_bytes,
        binary_bytes,
        padding_bytes: scanned.logical_bytes.saturating_sub(declared_length),
        ..Default::default()
    };
    for (index, view) in views.iter().enumerate() {
        if image_views.contains(&index) {
            stats.embedded_image_bytes = stats.embedded_image_bytes.saturating_add(view.length);
        } else if animation_views.contains(&index) {
            stats.animation_bytes = stats.animation_bytes.saturating_add(view.length);
        } else if geometry_views.contains(&index) {
            stats.geometry_bytes = stats.geometry_bytes.saturating_add(view.length);
        }
    }
    stats.other_buffer_bytes = binary_bytes.saturating_sub(
        stats
            .embedded_image_bytes
            .saturating_add(stats.animation_bytes)
            .saturating_add(stats.geometry_bytes),
    );

    let mut textures = Vec::new();
    let mut diagnostics = Vec::new();
    let image_roles = material_image_roles(&json);
    if let Some(binary_offset) = first_binary_offset {
        for (image_index, image) in json
            .get("images")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            let Some(view_index) = image.get("bufferView").and_then(Value::as_u64) else {
                continue;
            };
            let Some(view) = views.get(view_index as usize) else {
                continue;
            };
            let mime = image
                .get("mimeType")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if mime != "image/ktx2" {
                continue;
            }
            stats.embedded_ktx2_bytes = stats.embedded_ktx2_bytes.saturating_add(view.length);
            let role = image_roles
                .get(&image_index)
                .map(|roles| roles.iter().copied().collect::<Vec<_>>().join("+"))
                .unwrap_or_else(|| {
                    image
                        .get("name")
                        .and_then(Value::as_str)
                        .map(infer_texture_role)
                        .unwrap_or("other")
                        .to_string()
                });
            let location = format!("{}#image/{image_index}", scanned.relative_path);
            match inspect_ktx(
                &mut file,
                binary_offset.saturating_add(view.offset),
                view.length,
                location,
                scanned.relative_path.clone(),
                role,
            ) {
                Ok(texture) => textures.push(texture),
                Err(error) => diagnostics.push(format!(
                    "embedded KTX2 {} image {}: {error:#}",
                    scanned.relative_path, image_index
                )),
            }
        }
    }
    Ok((stats, textures, diagnostics))
}

fn parse_buffer_views(json: &Value) -> Vec<BufferView> {
    json.get("bufferViews")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|view| BufferView {
            offset: view.get("byteOffset").and_then(Value::as_u64).unwrap_or(0),
            length: view.get("byteLength").and_then(Value::as_u64).unwrap_or(0),
        })
        .collect()
}

fn image_buffer_views(json: &Value) -> BTreeSet<usize> {
    json.get("images")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|image| {
            image
                .get("bufferView")?
                .as_u64()
                .map(|value| value as usize)
        })
        .collect()
}

fn accessor_buffer_view(json: &Value, accessor_index: u64) -> Option<usize> {
    json.get("accessors")?
        .as_array()?
        .get(accessor_index as usize)?
        .get("bufferView")?
        .as_u64()
        .map(|value| value as usize)
}

fn add_accessor_view(json: &Value, value: Option<&Value>, output: &mut BTreeSet<usize>) {
    if let Some(view) = value
        .and_then(Value::as_u64)
        .and_then(|index| accessor_buffer_view(json, index))
    {
        output.insert(view);
    }
}

fn animation_buffer_views(json: &Value) -> BTreeSet<usize> {
    let mut output = BTreeSet::new();
    for animation in json
        .get("animations")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        for sampler in animation
            .get("samplers")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            add_accessor_view(json, sampler.get("input"), &mut output);
            add_accessor_view(json, sampler.get("output"), &mut output);
        }
    }
    output
}

fn geometry_buffer_views(json: &Value) -> BTreeSet<usize> {
    let mut output = BTreeSet::new();
    for mesh in json
        .get("meshes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        for primitive in mesh
            .get("primitives")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            add_accessor_view(json, primitive.get("indices"), &mut output);
            for accessor in primitive
                .get("attributes")
                .and_then(Value::as_object)
                .into_iter()
                .flat_map(|attributes| attributes.values())
            {
                add_accessor_view(json, Some(accessor), &mut output);
            }
            for target in primitive
                .get("targets")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                for accessor in target
                    .as_object()
                    .into_iter()
                    .flat_map(|attributes| attributes.values())
                {
                    add_accessor_view(json, Some(accessor), &mut output);
                }
            }
        }
    }
    for skin in json
        .get("skins")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        add_accessor_view(json, skin.get("inverseBindMatrices"), &mut output);
    }
    output
}

fn material_image_roles(json: &Value) -> BTreeMap<usize, BTreeSet<&'static str>> {
    let mut roles = BTreeMap::<usize, BTreeSet<&'static str>>::new();
    for material in json
        .get("materials")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let pbr = material.get("pbrMetallicRoughness");
        add_texture_role(
            json,
            pbr.and_then(|value| value.get("baseColorTexture")),
            "base_color",
            &mut roles,
        );
        add_texture_role(
            json,
            pbr.and_then(|value| value.get("metallicRoughnessTexture")),
            "mask",
            &mut roles,
        );
        add_texture_role(
            json,
            material.get("normalTexture"),
            "normal_specular",
            &mut roles,
        );
        add_texture_role(json, material.get("occlusionTexture"), "mask", &mut roles);
        add_texture_role(
            json,
            material.get("emissiveTexture"),
            "emissive",
            &mut roles,
        );
        if let Some(extensions) = material.get("extensions").and_then(Value::as_object) {
            for (name, extension) in extensions {
                let role = if name.contains("specular") || name.contains("clearcoat") {
                    "mask"
                } else {
                    "other"
                };
                if let Some(values) = extension.as_object() {
                    for (field, value) in values {
                        if field.to_ascii_lowercase().contains("texture") {
                            add_texture_role(json, Some(value), role, &mut roles);
                        }
                    }
                }
            }
        }
    }
    roles
}

fn add_texture_role(
    json: &Value,
    texture_info: Option<&Value>,
    role: &'static str,
    output: &mut BTreeMap<usize, BTreeSet<&'static str>>,
) {
    let Some(texture_index) = texture_info
        .and_then(|value| value.get("index").or(Some(value)))
        .and_then(Value::as_u64)
    else {
        return;
    };
    let Some(texture) = json
        .get("textures")
        .and_then(Value::as_array)
        .and_then(|textures| textures.get(texture_index as usize))
    else {
        return;
    };
    let source = texture
        .get("extensions")
        .and_then(|extensions| extensions.get("KHR_texture_basisu"))
        .and_then(|basis| basis.get("source"))
        .and_then(Value::as_u64)
        .or_else(|| texture.get("source").and_then(Value::as_u64));
    if let Some(source) = source {
        output.entry(source as usize).or_default().insert(role);
    }
}

fn inspect_ktx(
    file: &mut File,
    base_offset: u64,
    encoded_bytes: u64,
    location: String,
    container: String,
    role: String,
) -> Result<TextureReport> {
    if encoded_bytes < 80 {
        bail!("payload is shorter than the KTX2 header");
    }
    file.seek(SeekFrom::Start(base_offset))?;
    let mut header = [0u8; 80];
    file.read_exact(&mut header)?;
    if header[..12] != ktx2::MAGIC {
        bail!("invalid KTX2 magic");
    }
    let u32_at = |offset: usize| u32::from_le_bytes(header[offset..offset + 4].try_into().unwrap());
    let vk_format = u32_at(12);
    let width = u32_at(20);
    let height = u32_at(24).max(1);
    let depth = u32_at(28).max(1);
    let layer_count = u32_at(32).max(1);
    let face_count = u32_at(36).max(1);
    let mip_count = u32_at(40).max(1);
    let supercompression_value = u32_at(44);
    let dfd_offset = u64::from(u32_at(48));
    let dfd_length = u64::from(u32_at(52));

    let mut channel_count = 0u32;
    let mut color_model = "unknown".to_string();
    let mut color_space = "unknown".to_string();
    if (12..=64 * 1024).contains(&dfd_length)
        && dfd_offset.saturating_add(dfd_length) <= encoded_bytes
    {
        file.seek(SeekFrom::Start(base_offset.saturating_add(dfd_offset)))?;
        let mut dfd = vec![0u8; dfd_length as usize];
        file.read_exact(&mut dfd)?;
        let block_size = usize::from(u16::from_le_bytes([dfd[10], dfd[11]]));
        if block_size >= 8 && 4usize.saturating_add(block_size) <= dfd.len() {
            let basic = &dfd[12..4 + block_size];
            if let Ok(basic) = ktx2::dfd::Basic::parse(basic) {
                channel_count = basic.sample_information.len() as u32;
                color_model = basic
                    .color_model
                    .map(|value| format!("{value:?}"))
                    .unwrap_or_else(|| "unknown".into());
                color_space = basic
                    .transfer_function
                    .map(|value| format!("{value:?}"))
                    .unwrap_or_else(|| "unknown".into());
            }
        }
    }
    let supercompression = match supercompression_value {
        0 => "none".to_string(),
        1 => "basis-lz".to_string(),
        2 => "zstd".to_string(),
        3 => "zlib".to_string(),
        value => format!("unknown-{value}"),
    };
    if color_model == "unknown" && vk_format != 0 {
        color_model = format!("vk-format-{vk_format}");
    }

    Ok(TextureReport {
        location,
        container,
        role,
        encoded_bytes,
        width,
        height,
        depth,
        layer_count,
        face_count,
        mip_count,
        channel_count,
        vk_format,
        supercompression,
        color_model,
        color_space,
    })
}

fn infer_texture_role(path: &str) -> &'static str {
    let lower = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or(path)
        .to_ascii_lowercase();
    if lower.contains("normal") || lower.contains("_n") || lower.contains("ddn") {
        "normal_specular"
    } else if lower.contains("spec") || lower.contains("mask") || lower.contains("gloss") {
        "mask"
    } else if lower.contains("emit") || lower.contains("glow") {
        "emissive"
    } else if lower.contains("lightmap") {
        "lightmap"
    } else if lower.contains("interface") || lower.contains("ui") {
        "ui"
    } else {
        "base_color_or_other"
    }
}
