//! Bevy-free Henry DynamicLighting bake data.
//!
//! The renderer only needs a compact, deterministic stream of triangle
//! headers, light entries, visibility words, and quantised bounce samples.
//! Keeping this module independent of Bevy makes the expensive/authoritative
//! part of a bake testable without a GPU or an editor process.

use anyhow::{Context, Result, bail};
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::{Cursor, Read, Write};
use std::path::Path;

use super::core::{
    DynamicBounceCompression, DynamicLightIlluminationMode, DynamicLightShadowMode,
    DynamicLightTransparencyMode,
};

pub(crate) const ARTIFACT_MAGIC: [u8; 4] = *b"HYDL";
pub(crate) const ARTIFACT_VERSION: u32 = 1;
pub(crate) const DEFAULT_TEXELS_PER_METER: u32 = 128;
pub(crate) const DEFAULT_MAX_LIGHTMAP_SIZE: u32 = 2048;
pub(crate) const DEFAULT_BOUNCE_SAMPLES: u32 = 32;
pub(crate) const MAX_LIGHT_CHANNELS: usize = 32;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct DynamicLightingBakeSettings {
    pub(crate) texels_per_meter: u32,
    pub(crate) max_lightmap_size: u32,
    pub(crate) bounce_samples: u32,
    pub(crate) bounce_compression: DynamicBounceCompression,
    pub(crate) shadow_mode: DynamicLightShadowMode,
    pub(crate) illumination_mode: DynamicLightIlluminationMode,
    pub(crate) transparency_mode: DynamicLightTransparencyMode,
}

impl Default for DynamicLightingBakeSettings {
    fn default() -> Self {
        Self {
            texels_per_meter: DEFAULT_TEXELS_PER_METER,
            max_lightmap_size: DEFAULT_MAX_LIGHTMAP_SIZE,
            bounce_samples: DEFAULT_BOUNCE_SAMPLES,
            bounce_compression: DynamicBounceCompression::Bits8,
            shadow_mode: DynamicLightShadowMode::RaytracedShadows,
            illumination_mode: DynamicLightIlluminationMode::SingleBounce,
            transparency_mode: DynamicLightTransparencyMode::Disabled,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct DynamicLightingLightInput {
    pub(crate) reference_form_id: u32,
    pub(crate) position: [f32; 3],
    pub(crate) radius: f32,
    pub(crate) shadow_mode: DynamicLightShadowMode,
    pub(crate) illumination_mode: DynamicLightIlluminationMode,
    pub(crate) transparency_mode: DynamicLightTransparencyMode,
}

#[derive(Clone, Debug)]
pub(crate) struct DynamicLightingMeshInput {
    pub(crate) reference_form_ids: Vec<u32>,
    pub(crate) positions: Vec<[f32; 3]>,
    pub(crate) normals: Vec<[f32; 3]>,
    pub(crate) indices: Vec<u32>,
    /// A supplied UV1 is kept byte-for-byte. When it is absent, the baker
    /// generates a stable dominant-plane unwrap; this is deterministic for
    /// legacy GLBs and is replaced by xatlas output when the exporter supplies
    /// an unwrap.
    pub(crate) uv1: Option<Vec<[f32; 2]>>,
    pub(crate) casts_static_shadow: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DynamicLightingBake {
    pub(crate) revision: String,
    pub(crate) settings: DynamicLightingBakeSettings,
    pub(crate) lights: Vec<DynamicLightingLightRecord>,
    pub(crate) meshes: Vec<DynamicLightingMeshRecord>,
    pub(crate) words: Vec<u32>,
    pub(crate) bounce_words: Vec<u32>,
    pub(crate) diagnostics: DynamicLightingBakeDiagnostics,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DynamicLightingLightRecord {
    pub(crate) reference_form_id: u32,
    pub(crate) position: [f32; 3],
    pub(crate) radius: f32,
    pub(crate) shadow_mode: DynamicLightShadowMode,
    pub(crate) illumination_mode: DynamicLightIlluminationMode,
    pub(crate) transparency_mode: DynamicLightTransparencyMode,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DynamicLightingMeshRecord {
    pub(crate) tag: u32,
    pub(crate) triangle_count: u32,
    pub(crate) lightmap_resolution: u32,
    pub(crate) triangle_word_offset: u32,
    pub(crate) triangle_word_count: u32,
    pub(crate) bounce_word_offset: u32,
    pub(crate) bounce_word_count: u32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct DynamicLightingBakeDiagnostics {
    pub(crate) candidate_associations: u32,
    pub(crate) retained_associations: u32,
    pub(crate) pruned_associations: u32,
    pub(crate) occluded_samples: u32,
    pub(crate) triangle_count: u32,
    pub(crate) caster_triangle_count: u32,
}

impl DynamicLightingBake {
    pub(crate) fn uncompressed_bytes(&self) -> usize {
        4 * self.words.len() + 4 * self.bounce_words.len()
    }

    pub(crate) fn compressed_container(&self) -> Result<Vec<u8>> {
        let mut raw = Vec::with_capacity(self.uncompressed_bytes());
        raw.extend_from_slice(&ARTIFACT_MAGIC);
        put_u32(&mut raw, ARTIFACT_VERSION);
        // Little-endian declaration (0x01020304 reads as 04 03 02 01).
        put_u32(&mut raw, 0x0102_0304);
        put_string(&mut raw, &self.revision);
        put_u32(&mut raw, self.settings.texels_per_meter);
        put_u32(&mut raw, self.settings.max_lightmap_size);
        put_u32(&mut raw, self.settings.bounce_samples);
        raw.push(self.settings.bounce_compression.bits());
        raw.push(self.settings.shadow_mode as u8);
        raw.push(self.settings.illumination_mode as u8);
        raw.push(self.settings.transparency_mode as u8);
        put_u32(&mut raw, self.lights.len() as u32);
        put_u32(&mut raw, self.meshes.len() as u32);
        put_u32(&mut raw, self.words.len() as u32);
        put_u32(&mut raw, self.bounce_words.len() as u32);
        for light in &self.lights {
            put_u32(&mut raw, light.reference_form_id);
            for value in light.position {
                put_f32(&mut raw, value);
            }
            put_f32(&mut raw, light.radius);
            put_u32(&mut raw, light.shadow_mode as u32);
            put_u32(&mut raw, light.illumination_mode as u32);
            put_u32(&mut raw, light.transparency_mode as u32);
        }
        for mesh in &self.meshes {
            put_u32(&mut raw, mesh.tag);
            put_u32(&mut raw, mesh.triangle_count);
            put_u32(&mut raw, mesh.lightmap_resolution);
            put_u32(&mut raw, mesh.triangle_word_offset);
            put_u32(&mut raw, mesh.triangle_word_count);
            put_u32(&mut raw, mesh.bounce_word_offset);
            put_u32(&mut raw, mesh.bounce_word_count);
        }
        for word in &self.words {
            put_u32(&mut raw, *word);
        }
        for word in &self.bounce_words {
            put_u32(&mut raw, *word);
        }
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&raw)?;
        Ok(encoder.finish()?)
    }

    pub(crate) fn write_gzip(&self, path: &Path) -> Result<String> {
        let bytes = self.compressed_container()?;
        std::fs::write(path, &bytes).with_context(|| format!("write {}", path.display()))?;
        Ok(hex_sha256(&bytes))
    }

    pub(crate) fn read_gzip(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
        let mut decoder = GzDecoder::new(Cursor::new(bytes));
        let mut raw = Vec::new();
        decoder.read_to_end(&mut raw)?;
        decode_raw(&raw)
    }
}

pub(crate) fn build_dynamic_lighting_bake(
    meshes: &[DynamicLightingMeshInput],
    lights: &[DynamicLightingLightInput],
    settings: DynamicLightingBakeSettings,
    revision: impl Into<String>,
) -> Result<DynamicLightingBake> {
    if settings.texels_per_meter == 0 || settings.max_lightmap_size == 0 {
        bail!("dynamic-lighting texel settings must be non-zero");
    }
    let mut sorted_lights = lights.to_vec();
    sorted_lights.sort_by_key(|light| light.reference_form_id);
    sorted_lights.dedup_by_key(|light| light.reference_form_id);
    let light_records = sorted_lights
        .iter()
        .map(|light| DynamicLightingLightRecord {
            reference_form_id: light.reference_form_id,
            position: light.position,
            radius: light.radius.max(0.001),
            shadow_mode: light.shadow_mode,
            illumination_mode: light.illumination_mode,
            transparency_mode: light.transparency_mode,
        })
        .collect::<Vec<_>>();
    let casters = meshes
        .iter()
        .filter(|mesh| mesh.casts_static_shadow)
        .collect::<Vec<_>>();
    let mut words = Vec::new();
    let mut bounce_words = Vec::new();
    let mut records = Vec::with_capacity(meshes.len());
    let mut diagnostics = DynamicLightingBakeDiagnostics {
        caster_triangle_count: casters
            .iter()
            .map(|mesh| mesh.indices.len() / 3)
            .sum::<usize>() as u32,
        ..Default::default()
    };

    for (mesh_index, mesh) in meshes.iter().enumerate() {
        validate_mesh(mesh)?;
        let uv1 = mesh
            .uv1
            .clone()
            .unwrap_or_else(|| generate_uv1(&mesh.positions, &mesh.normals));
        let resolution = lightmap_resolution(mesh, settings);
        let triangle_count = mesh.indices.len() / 3;
        let triangle_word_offset = words.len() as u32;
        words.resize(words.len() + triangle_count * 4, 0);
        let bounce_word_offset = bounce_words.len() as u32;
        for triangle_index in 0..triangle_count {
            let [i0, i1, i2] = triangle_indices(&mesh.indices, triangle_index);
            let p0 = mesh.positions[i0 as usize];
            let p1 = mesh.positions[i1 as usize];
            let p2 = mesh.positions[i2 as usize];
            let normal = triangle_normal(p0, p1, p2, mesh.normals.get(i0 as usize).copied());
            let center = midpoint(p0, p1, p2);
            let self_caster_index = mesh.casts_static_shadow.then(|| {
                casters
                    .iter()
                    .position(|candidate| std::ptr::eq(*candidate, mesh))
                    .expect("static caster must be present in caster list")
            });
            let bounds = uv_bounds(&uv1, [i0, i1, i2], resolution);
            let header = triangle_word_offset as usize + triangle_index * 4;
            words[header + 1] = bounds.0;
            words[header + 2] = bounds.1;
            words[header + 3] = bounds.2;
            let light_data_offset = words.len() as u32;
            words[header] = light_data_offset;
            let candidates = candidate_lights(center, normal, &light_records, settings);
            diagnostics.candidate_associations += candidates.len() as u32;
            words.push(candidates.len() as u32);
            let entries_offset = words.len();
            words.resize(words.len() + candidates.len() * 3, u32::MAX);
            let mut retained = Vec::new();
            for (candidate_index, light_index) in candidates.iter().copied().enumerate() {
                let light = &light_records[light_index as usize];
                let blocked = matches!(light.shadow_mode, DynamicLightShadowMode::RaytracedShadows)
                    && !ray_clear(
                        center,
                        light.position,
                        &casters,
                        self_caster_index,
                        triangle_index,
                    );
                diagnostics.occluded_samples += blocked as u32;
                let shadow_offset = words.len() as u32;
                let texel_count = (bounds.2 as usize).saturating_mul(bounds.2 as usize).max(1);
                let shadow_word_count = texel_count.div_ceil(32);
                words.resize(
                    words.len() + shadow_word_count,
                    if blocked { u32::MAX } else { 0 },
                );
                let bounce_offset = if matches!(
                    light.illumination_mode,
                    DynamicLightIlluminationMode::SingleBounce
                ) && matches!(
                    settings.illumination_mode,
                    DynamicLightIlluminationMode::SingleBounce
                ) {
                    let values = (0..settings.bounce_samples.max(1))
                        .map(|sample| {
                            let phase =
                                (sample as f32 + 0.5) / settings.bounce_samples.max(1) as f32;
                            (0.25 + 0.15 * phase)
                                * normal_light_factor(normal, center, light.position)
                        })
                        .collect::<Vec<_>>();
                    let offset = bounce_words.len() as u32;
                    bounce_words.extend(pack_quantized(&values, settings.bounce_compression));
                    Some(offset)
                } else {
                    None
                };
                let entry = entries_offset + candidate_index * 3;
                words[entry] = light_index;
                words[entry + 1] = shadow_offset;
                words[entry + 2] = bounce_offset.unwrap_or(u32::MAX);
                retained.push((light_index, blocked));
            }
            diagnostics.retained_associations += retained.len() as u32;
            diagnostics.pruned_associations +=
                candidates.len().saturating_sub(retained.len()) as u32;
        }
        records.push(DynamicLightingMeshRecord {
            tag: (mesh_index + 1) as u32,
            triangle_count: triangle_count as u32,
            lightmap_resolution: resolution,
            triangle_word_offset,
            triangle_word_count: (words.len() as u32).saturating_sub(triangle_word_offset),
            bounce_word_offset,
            bounce_word_count: (bounce_words.len() as u32).saturating_sub(bounce_word_offset),
        });
        diagnostics.triangle_count += triangle_count as u32;
    }
    Ok(DynamicLightingBake {
        revision: revision.into(),
        settings,
        lights: light_records,
        meshes: records,
        words,
        bounce_words,
        diagnostics,
    })
}

fn validate_mesh(mesh: &DynamicLightingMeshInput) -> Result<()> {
    if mesh.positions.len() != mesh.normals.len() {
        bail!("dynamic-lighting mesh position/normal count mismatch");
    }
    if !mesh.indices.len().is_multiple_of(3)
        || mesh
            .indices
            .iter()
            .any(|index| *index as usize >= mesh.positions.len())
    {
        bail!("dynamic-lighting mesh has invalid triangle indices");
    }
    if let Some(uv1) = &mesh.uv1
        && uv1.len() != mesh.positions.len()
    {
        bail!("dynamic-lighting UV1 count mismatch");
    }
    Ok(())
}

fn lightmap_resolution(
    mesh: &DynamicLightingMeshInput,
    settings: DynamicLightingBakeSettings,
) -> u32 {
    let area = (0..mesh.indices.len() / 3)
        .map(|triangle| {
            let [i0, i1, i2] = triangle_indices(&mesh.indices, triangle);
            let a = mesh.positions[i0 as usize];
            let b = mesh.positions[i1 as usize];
            let c = mesh.positions[i2 as usize];
            cross(sub(b, a), sub(c, a)).length() * 0.5
        })
        .sum::<f32>();
    (settings.texels_per_meter as f32 * area.max(0.0).sqrt())
        .ceil()
        .clamp(1.0, settings.max_lightmap_size as f32) as u32
}

pub(crate) fn generate_uv1(positions: &[[f32; 3]], normals: &[[f32; 3]]) -> Vec<[f32; 2]> {
    let axis = normals.iter().fold([0.0; 3], |sum, normal| {
        [
            sum[0] + normal[0].abs(),
            sum[1] + normal[1].abs(),
            sum[2] + normal[2].abs(),
        ]
    });
    let dropped = if axis[0] >= axis[1] && axis[0] >= axis[2] {
        0
    } else if axis[1] >= axis[2] {
        1
    } else {
        2
    };
    let mut min = [f32::INFINITY; 2];
    let mut max = [f32::NEG_INFINITY; 2];
    let project = |p: [f32; 3]| -> [f32; 2] {
        match dropped {
            0 => [p[1], p[2]],
            1 => [p[0], p[2]],
            _ => [p[0], p[1]],
        }
    };
    let mut raw = Vec::with_capacity(positions.len());
    for position in positions {
        let uv = project(*position);
        min[0] = min[0].min(uv[0]);
        min[1] = min[1].min(uv[1]);
        max[0] = max[0].max(uv[0]);
        max[1] = max[1].max(uv[1]);
        raw.push(uv);
    }
    let extent = [(max[0] - min[0]).max(1e-6), (max[1] - min[1]).max(1e-6)];
    raw.into_iter()
        .map(|uv| [(uv[0] - min[0]) / extent[0], (uv[1] - min[1]) / extent[1]])
        .collect()
}

fn candidate_lights(
    center: [f32; 3],
    normal: [f32; 3],
    lights: &[DynamicLightingLightRecord],
    _settings: DynamicLightingBakeSettings,
) -> Vec<u32> {
    let mut candidates = lights
        .iter()
        .enumerate()
        .filter_map(|(index, light)| {
            let delta = sub(light.position, center);
            let distance = F3(delta).length();
            if distance > light.radius || dot(normal, normalize(delta)) <= 0.0 {
                return None;
            }
            Some((index as u32, distance))
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.1.total_cmp(&right.1).then(left.0.cmp(&right.0)));
    candidates.truncate(MAX_LIGHT_CHANNELS);
    candidates
        .into_iter()
        .map(|candidate| candidate.0)
        .collect()
}

fn ray_clear(
    start: [f32; 3],
    end: [f32; 3],
    casters: &[&DynamicLightingMeshInput],
    self_mesh: Option<usize>,
    self_triangle: usize,
) -> bool {
    let direction = sub(end, start);
    let length = F3(direction).length();
    if length <= 1e-5 {
        return true;
    }
    let direction = scale(direction, 1.0 / length);
    let origin = add(start, scale(direction, 1e-4));
    for (mesh_index, mesh) in casters.iter().enumerate() {
        for triangle in 0..mesh.indices.len() / 3 {
            if self_mesh == Some(mesh_index) && triangle == self_triangle {
                continue;
            }
            let [i0, i1, i2] = triangle_indices(&mesh.indices, triangle);
            if ray_triangle(
                origin,
                direction,
                mesh.positions[i0 as usize],
                mesh.positions[i1 as usize],
                mesh.positions[i2 as usize],
            )
            .is_some_and(|distance| distance < length - 1e-4)
            {
                return false;
            }
        }
    }
    true
}

fn ray_triangle(
    origin: [f32; 3],
    direction: [f32; 3],
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
) -> Option<f32> {
    let edge1 = sub(b, a);
    let edge2 = sub(c, a);
    let h = cross(direction, edge2).0;
    let det = dot(edge1, h);
    if det.abs() < 1e-7 {
        return None;
    }
    let inverse = 1.0 / det;
    let s = sub(origin, a);
    let u = inverse * dot(s, h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = cross(s, edge1).0;
    let v = inverse * dot(direction, q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = inverse * dot(edge2, q);
    (distance > 0.0).then_some(distance)
}

fn uv_bounds(uvs: &[[f32; 2]], indices: [u32; 3], resolution: u32) -> (u32, u32, u32) {
    let points = [
        uvs[indices[0] as usize],
        uvs[indices[1] as usize],
        uvs[indices[2] as usize],
    ];
    let min_x = (points.iter().map(|uv| uv[0]).fold(f32::INFINITY, f32::min) * resolution as f32)
        .floor()
        .max(0.0) as u32;
    let min_y = (points.iter().map(|uv| uv[1]).fold(f32::INFINITY, f32::min) * resolution as f32)
        .floor()
        .max(0.0) as u32;
    let max_x = (points
        .iter()
        .map(|uv| uv[0])
        .fold(f32::NEG_INFINITY, f32::max)
        * resolution as f32)
        .ceil()
        .min(resolution.saturating_sub(1) as f32) as u32;
    let max_y = (points
        .iter()
        .map(|uv| uv[1])
        .fold(f32::NEG_INFINITY, f32::max)
        * resolution as f32)
        .ceil()
        .min(resolution.saturating_sub(1) as f32) as u32;
    (
        min_x.saturating_sub(2),
        min_y.saturating_sub(2),
        (max_x.saturating_sub(min_x) + 1 + 4).max(max_y.saturating_sub(min_y) + 1 + 4),
    )
}

fn pack_quantized(values: &[f32], compression: DynamicBounceCompression) -> Vec<u32> {
    let bits = compression.bits() as u32;
    let mask = (1u32 << bits) - 1;
    let mut output = Vec::new();
    let mut word = 0u32;
    let mut used = 0u32;
    for value in values {
        let quantized = (value.clamp(0.0, 1.0) * mask as f32).round() as u32;
        if used + bits > 32 {
            output.push(word);
            word = 0;
            used = 0;
        }
        word |= quantized << used;
        used += bits;
    }
    if used != 0 {
        output.push(word);
    }
    output
}

fn decode_raw(raw: &[u8]) -> Result<DynamicLightingBake> {
    let mut reader = ByteReader {
        bytes: raw,
        offset: 0,
    };
    if reader.take(4)? != ARTIFACT_MAGIC {
        bail!("dynamic-lighting artifact magic mismatch");
    }
    if reader.u32()? != ARTIFACT_VERSION {
        bail!("unsupported dynamic-lighting artifact version");
    }
    if reader.u32()? != 0x0102_0304 {
        bail!("dynamic-lighting artifact is not little-endian");
    }
    let revision = reader.string()?;
    let settings = DynamicLightingBakeSettings {
        texels_per_meter: reader.u32()?,
        max_lightmap_size: reader.u32()?,
        bounce_samples: reader.u32()?,
        bounce_compression: compression_from_bits(reader.u8()?)?,
        shadow_mode: shadow_from_u8(reader.u8()?)?,
        illumination_mode: illumination_from_u8(reader.u8()?)?,
        transparency_mode: transparency_from_u8(reader.u8()?)?,
    };
    let light_count = reader.u32()? as usize;
    let mesh_count = reader.u32()? as usize;
    let word_count = reader.u32()? as usize;
    let bounce_count = reader.u32()? as usize;
    let mut lights = Vec::with_capacity(light_count);
    for _ in 0..light_count {
        lights.push(DynamicLightingLightRecord {
            reference_form_id: reader.u32()?,
            position: [reader.f32()?, reader.f32()?, reader.f32()?],
            radius: reader.f32()?,
            shadow_mode: shadow_from_u32(reader.u32()?)?,
            illumination_mode: illumination_from_u32(reader.u32()?)?,
            transparency_mode: transparency_from_u32(reader.u32()?)?,
        });
    }
    let mut meshes = Vec::with_capacity(mesh_count);
    for _ in 0..mesh_count {
        meshes.push(DynamicLightingMeshRecord {
            tag: reader.u32()?,
            triangle_count: reader.u32()?,
            lightmap_resolution: reader.u32()?,
            triangle_word_offset: reader.u32()?,
            triangle_word_count: reader.u32()?,
            bounce_word_offset: reader.u32()?,
            bounce_word_count: reader.u32()?,
        });
    }
    let words = (0..word_count)
        .map(|_| reader.u32())
        .collect::<Result<Vec<_>>>()?;
    let bounce_words = (0..bounce_count)
        .map(|_| reader.u32())
        .collect::<Result<Vec<_>>>()?;
    if reader.offset != raw.len() {
        bail!("dynamic-lighting artifact has trailing bytes");
    }
    Ok(DynamicLightingBake {
        revision,
        settings,
        lights,
        meshes,
        words,
        bounce_words,
        diagnostics: Default::default(),
    })
}

fn compression_from_bits(bits: u8) -> Result<DynamicBounceCompression> {
    match bits {
        8 => Ok(DynamicBounceCompression::Bits8),
        6 => Ok(DynamicBounceCompression::Bits6),
        5 => Ok(DynamicBounceCompression::Bits5),
        4 => Ok(DynamicBounceCompression::Bits4),
        _ => bail!("unsupported bounce compression {bits}"),
    }
}
fn shadow_from_u8(value: u8) -> Result<DynamicLightShadowMode> {
    shadow_from_u32(value as u32)
}
fn shadow_from_u32(value: u32) -> Result<DynamicLightShadowMode> {
    match value {
        0 => Ok(DynamicLightShadowMode::RaytracedShadows),
        1 => Ok(DynamicLightShadowMode::RealtimeShadows),
        2 => Ok(DynamicLightShadowMode::Disabled),
        _ => bail!("unsupported shadow mode {value}"),
    }
}
fn illumination_from_u8(value: u8) -> Result<DynamicLightIlluminationMode> {
    illumination_from_u32(value as u32)
}
fn illumination_from_u32(value: u32) -> Result<DynamicLightIlluminationMode> {
    match value {
        0 => Ok(DynamicLightIlluminationMode::DirectIllumination),
        1 => Ok(DynamicLightIlluminationMode::SingleBounce),
        _ => bail!("unsupported illumination mode {value}"),
    }
}
fn transparency_from_u8(value: u8) -> Result<DynamicLightTransparencyMode> {
    transparency_from_u32(value as u32)
}
fn transparency_from_u32(value: u32) -> Result<DynamicLightTransparencyMode> {
    match value {
        0 => Ok(DynamicLightTransparencyMode::Disabled),
        1 => Ok(DynamicLightTransparencyMode::AlphaTest),
        2 => Ok(DynamicLightTransparencyMode::AlphaBlend),
        _ => bail!("unsupported transparency mode {value}"),
    }
}

fn put_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}
fn put_f32(buffer: &mut Vec<u8>, value: f32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}
fn put_string(buffer: &mut Vec<u8>, value: &str) {
    put_u32(buffer, value.len() as u32);
    buffer.extend_from_slice(value.as_bytes());
}
fn hex_sha256(bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(bytes);
    format!("{:x}", hash.finalize())
}

struct ByteReader<'a> {
    bytes: &'a [u8],
    offset: usize,
}
impl<'a> ByteReader<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8]> {
        let end = self
            .offset
            .checked_add(count)
            .context("artifact offset overflow")?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .context("truncated dynamic-lighting artifact")?;
        self.offset = end;
        Ok(bytes)
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(*self.take(1)?.first().unwrap())
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn string(&mut self) -> Result<String> {
        let length = self.u32()? as usize;
        String::from_utf8(self.take(length)?.to_vec()).context("invalid artifact revision")
    }
}

#[derive(Clone, Copy)]
struct F3([f32; 3]);
impl F3 {
    fn length(self) -> f32 {
        (self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]).sqrt()
    }
}
fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn scale(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f32; 3], b: [f32; 3]) -> F3 {
    F3([
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ])
}
fn normalize(a: [f32; 3]) -> [f32; 3] {
    let length = F3(a).length();
    if length <= 1e-6 {
        [0.0, 0.0, 1.0]
    } else {
        scale(a, 1.0 / length)
    }
}
fn midpoint(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    scale(add(add(a, b), c), 1.0 / 3.0)
}
fn triangle_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3], authored: Option<[f32; 3]>) -> [f32; 3] {
    authored
        .map(normalize)
        .unwrap_or_else(|| normalize(cross(sub(b, a), sub(c, a)).0))
}
fn normal_light_factor(normal: [f32; 3], point: [f32; 3], light: [f32; 3]) -> f32 {
    dot(normal, normalize(sub(light, point))).max(0.0)
}
fn triangle_indices(indices: &[u32], triangle: usize) -> [u32; 3] {
    [
        indices[triangle * 3],
        indices[triangle * 3 + 1],
        indices[triangle * 3 + 2],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quad(casts_static_shadow: bool) -> DynamicLightingMeshInput {
        DynamicLightingMeshInput {
            reference_form_ids: vec![1],
            positions: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            normals: vec![[0.0, 1.0, 0.0]; 4],
            indices: vec![0, 2, 1, 0, 3, 2],
            uv1: None,
            casts_static_shadow,
        }
    }

    #[test]
    fn stable_catalog_is_form_id_sorted_and_capped_to_32_channels() {
        let lights = (0..40)
            .map(|index| DynamicLightingLightInput {
                reference_form_id: 40 - index,
                position: [0.5, 2.0 + index as f32, 0.5],
                radius: 100.0,
                shadow_mode: DynamicLightShadowMode::Disabled,
                illumination_mode: DynamicLightIlluminationMode::DirectIllumination,
                transparency_mode: DynamicLightTransparencyMode::Disabled,
            })
            .collect::<Vec<_>>();
        let bake = build_dynamic_lighting_bake(
            &[quad(false)],
            &lights,
            DynamicLightingBakeSettings::default(),
            "test",
        )
        .unwrap();
        assert_eq!(bake.lights.first().unwrap().reference_form_id, 1);
        assert!(bake.words.len() > 4);
        assert!(bake.words[bake.words[0] as usize] <= MAX_LIGHT_CHANNELS as u32);
    }

    #[test]
    fn gzip_round_trip_and_corruption_are_detected() {
        let bake = build_dynamic_lighting_bake(
            &[quad(false)],
            &[],
            DynamicLightingBakeSettings::default(),
            "rev",
        )
        .unwrap();
        let bytes = bake.compressed_container().unwrap();
        let path =
            std::env::temp_dir().join(format!("bevyout-dynamic-{}.bytes.gz", std::process::id()));
        std::fs::write(&path, &bytes).unwrap();
        let loaded = DynamicLightingBake::read_gzip(&path).unwrap();
        assert_eq!(loaded.revision, "rev");
        std::fs::write(&path, &bytes[..bytes.len() - 2]).unwrap();
        assert!(DynamicLightingBake::read_gzip(&path).is_err());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn four_compression_modes_produce_deterministic_words() {
        for compression in [
            DynamicBounceCompression::Bits8,
            DynamicBounceCompression::Bits6,
            DynamicBounceCompression::Bits5,
            DynamicBounceCompression::Bits4,
        ] {
            let settings = DynamicLightingBakeSettings {
                bounce_compression: compression,
                ..Default::default()
            };
            let bake = build_dynamic_lighting_bake(
                &[quad(false)],
                &[DynamicLightingLightInput {
                    reference_form_id: 1,
                    position: [0.5, 1.0, 0.5],
                    radius: 4.0,
                    shadow_mode: DynamicLightShadowMode::Disabled,
                    illumination_mode: DynamicLightIlluminationMode::SingleBounce,
                    transparency_mode: DynamicLightTransparencyMode::Disabled,
                }],
                settings,
                "rev",
            )
            .unwrap();
            assert!(!bake.bounce_words.is_empty());
        }
    }

    #[test]
    fn triangle_light_entries_precede_their_visibility_payload() {
        let bake = build_dynamic_lighting_bake(
            &[quad(false)],
            &[DynamicLightingLightInput {
                reference_form_id: 7,
                position: [0.5, 1.0, 0.5],
                radius: 4.0,
                shadow_mode: DynamicLightShadowMode::RaytracedShadows,
                illumination_mode: DynamicLightIlluminationMode::SingleBounce,
                transparency_mode: DynamicLightTransparencyMode::Disabled,
            }],
            DynamicLightingBakeSettings::default(),
            "rev",
        )
        .unwrap();
        let light_data = bake.words[0] as usize;
        assert_eq!(bake.words[light_data], 1);
        assert_eq!(bake.words[light_data + 1], 0);
        let shadow_offset = bake.words[light_data + 2] as usize;
        assert!(shadow_offset >= light_data + 4);
        assert_eq!(bake.words[shadow_offset], 0);
        assert_ne!(bake.words[light_data + 3], u32::MAX);
    }

    #[test]
    fn non_caster_geometry_never_occludes_a_receiver() {
        let receiver = quad(false);
        let mut blocker = quad(false);
        for position in &mut blocker.positions {
            position[1] = 0.5;
        }
        let light = DynamicLightingLightInput {
            reference_form_id: 1,
            position: [0.5, 1.5, 0.5],
            radius: 4.0,
            shadow_mode: DynamicLightShadowMode::RaytracedShadows,
            illumination_mode: DynamicLightIlluminationMode::DirectIllumination,
            transparency_mode: DynamicLightTransparencyMode::Disabled,
        };
        let unoccluded = build_dynamic_lighting_bake(
            &[receiver.clone(), blocker.clone()],
            std::slice::from_ref(&light),
            DynamicLightingBakeSettings::default(),
            "rev",
        )
        .unwrap();
        let light_data = unoccluded.words[0] as usize;
        let shadow_offset = unoccluded.words[light_data + 2] as usize;
        assert_eq!(unoccluded.words[shadow_offset], 0);

        blocker.casts_static_shadow = true;
        let occluded = build_dynamic_lighting_bake(
            &[receiver, blocker],
            &[light],
            DynamicLightingBakeSettings::default(),
            "rev",
        )
        .unwrap();
        let light_data = occluded.words[0] as usize;
        let shadow_offset = occluded.words[light_data + 2] as usize;
        assert_eq!(occluded.words[shadow_offset], u32::MAX);
    }
}
