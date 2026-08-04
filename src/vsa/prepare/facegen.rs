//! Bounded native FaceGen EGM/EGT preparation.
//!
//! This module is deliberately separate from the core coefficient contract:
//! the core knows only serialized `FGGS`/`FGGA`/`FGTS` values, while this side
//! owns file-backed morph bases, mesh deformation, and image synthesis.

use std::io::Cursor;

use bevyout_core::facegen::{
    FaceGenAssetKind, FaceGenCoefficients, FaceGenComponent, FaceGenDiagnostic,
};

const EGM_MAGIC: &[u8; 8] = b"FREGM002";
const EGT_MAGIC: &[u8; 8] = b"FREGT003";
const TRI_MAGIC: &[u8; 8] = b"FRTRI003";
const EGM_HEADER_BYTES: usize = 8 + 4 + 4 + 4 + 4 + 40;
const EGT_HEADER_BYTES: usize = 8 + 4 + 4 + 4 + 4 + 4 + 36;
const TRI_HEADER_BYTES: usize = 8 + (10 * 4) + 16;
const MAX_FACEGEN_VERTICES: usize = 200_000;
const MAX_FACEGEN_MODES: usize = 256;
const MAX_FACEGEN_TEXTURE_PIXELS: usize = 16_777_216;
const MAX_TRIANGLES: usize = 500_000;

#[derive(Debug, Clone)]
pub(crate) struct GeometryMorph {
    pub(crate) vertex_count: usize,
    pub(crate) symmetric_count: usize,
    pub(crate) asymmetric_count: usize,
    pub(crate) modes: Vec<Vec<[f32; 3]>>,
}

#[derive(Debug, Clone)]
pub(crate) struct TextureMorph {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) symmetric_count: usize,
    pub(crate) modes: Vec<Vec<[f32; 3]>>,
}

/// The vanilla NIF stores the first `base_vertex_count` vertices from its
/// FaceGen TRI.  The remaining TRI static-morph vertices are still present in
/// EGM's basis, so `combined_vertex_count` is the EGM vertex count while the
/// base count is the number to deform in the selected NIF.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TriLayout {
    pub(crate) base_vertex_count: usize,
    pub(crate) combined_vertex_count: usize,
    pub(crate) triangle_count: usize,
    pub(crate) texture_coordinate_count: usize,
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let end = self.offset.checked_add(count)?;
        let bytes = self.bytes.get(self.offset..end)?;
        self.offset = end;
        Some(bytes)
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn f32(&mut self) -> Option<f32> {
        Some(f32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }
}

fn unsupported(asset: FaceGenAssetKind, reason: impl Into<String>) -> FaceGenDiagnostic {
    FaceGenDiagnostic::UnsupportedAsset {
        asset,
        reason: reason.into(),
    }
}

fn truncated(asset: FaceGenAssetKind) -> FaceGenDiagnostic {
    unsupported(asset, "truncated or structurally incomplete file")
}

fn checked_count(
    asset: FaceGenAssetKind,
    value: u32,
    maximum: usize,
    label: &str,
) -> Result<usize, FaceGenDiagnostic> {
    let value = usize::try_from(value)
        .map_err(|_| unsupported(asset, format!("{label} overflows usize")))?;
    if value > maximum {
        return Err(unsupported(
            asset,
            format!("{label} {value} exceeds bounded maximum {maximum}"),
        ));
    }
    Ok(value)
}

pub(crate) fn parse_geometry_morph(bytes: &[u8]) -> Result<GeometryMorph, FaceGenDiagnostic> {
    let asset = FaceGenAssetKind::GeometryMorph;
    if bytes.len() < EGM_HEADER_BYTES {
        return Err(truncated(asset));
    }
    let mut reader = Reader::new(bytes);
    if reader.take(8) != Some(EGM_MAGIC.as_slice()) {
        return Err(unsupported(asset, "expected FREGM002 magic"));
    }
    let vertex_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_VERTICES,
        "vertex count",
    )?;
    let symmetric_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_MODES,
        "symmetric mode count",
    )?;
    let asymmetric_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_MODES,
        "asymmetric mode count",
    )?;
    let _basis_version = reader.u32().ok_or_else(|| truncated(asset))?;
    reader.take(40).ok_or_else(|| truncated(asset))?;

    if symmetric_count != FaceGenComponent::GeometrySymmetric.expected_coefficients()
        || asymmetric_count != FaceGenComponent::GeometryAsymmetric.expected_coefficients()
    {
        return Err(unsupported(
            asset,
            format!(
                "expected 50 symmetric and 30 asymmetric modes, got {symmetric_count} and {asymmetric_count}"
            ),
        ));
    }
    let mode_count = symmetric_count
        .checked_add(asymmetric_count)
        .ok_or_else(|| unsupported(asset, "mode count overflow"))?;
    let per_mode = 4usize
        .checked_add(
            vertex_count
                .checked_mul(6)
                .ok_or_else(|| unsupported(asset, "vertex payload size overflow"))?,
        )
        .ok_or_else(|| unsupported(asset, "mode payload size overflow"))?;
    let expected = EGM_HEADER_BYTES
        .checked_add(
            per_mode
                .checked_mul(mode_count)
                .ok_or_else(|| unsupported(asset, "file size overflow"))?,
        )
        .ok_or_else(|| unsupported(asset, "file size overflow"))?;
    if bytes.len() != expected {
        return Err(unsupported(
            asset,
            format!("expected {expected} bytes, got {}", bytes.len()),
        ));
    }

    let mut modes = Vec::with_capacity(mode_count);
    for _ in 0..mode_count {
        let scale = reader.f32().ok_or_else(|| truncated(asset))?;
        if !scale.is_finite() {
            return Err(unsupported(asset, "non-finite morph scale"));
        }
        let mut mode = Vec::with_capacity(vertex_count);
        for _ in 0..vertex_count {
            let x = i16::from_le_bytes(
                reader
                    .take(2)
                    .ok_or_else(|| truncated(asset))?
                    .try_into()
                    .unwrap(),
            );
            let y = i16::from_le_bytes(
                reader
                    .take(2)
                    .ok_or_else(|| truncated(asset))?
                    .try_into()
                    .unwrap(),
            );
            let z = i16::from_le_bytes(
                reader
                    .take(2)
                    .ok_or_else(|| truncated(asset))?
                    .try_into()
                    .unwrap(),
            );
            let delta = [
                f32::from(x) * scale,
                f32::from(y) * scale,
                f32::from(z) * scale,
            ];
            if delta.iter().any(|value| !value.is_finite()) {
                return Err(unsupported(asset, "non-finite geometry delta"));
            }
            mode.push(delta);
        }
        modes.push(mode);
    }
    debug_assert_eq!(reader.remaining(), 0);
    Ok(GeometryMorph {
        vertex_count,
        symmetric_count,
        asymmetric_count,
        modes,
    })
}

pub(crate) fn parse_texture_morph(bytes: &[u8]) -> Result<TextureMorph, FaceGenDiagnostic> {
    let asset = FaceGenAssetKind::TextureMorph;
    if bytes.len() < EGT_HEADER_BYTES {
        return Err(truncated(asset));
    }
    let mut reader = Reader::new(bytes);
    if reader.take(8) != Some(EGT_MAGIC.as_slice()) {
        return Err(unsupported(asset, "expected FREGT003 magic"));
    }
    let height = reader.u32().ok_or_else(|| truncated(asset))?;
    let width = reader.u32().ok_or_else(|| truncated(asset))?;
    let symmetric_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_MODES,
        "symmetric mode count",
    )?;
    let asymmetric_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_MODES,
        "asymmetric mode count",
    )?;
    let _basis_version = reader.u32().ok_or_else(|| truncated(asset))?;
    reader.take(36).ok_or_else(|| truncated(asset))?;
    let width_usize = checked_count(asset, width, 16_384, "texture width")?;
    let height_usize = checked_count(asset, height, 16_384, "texture height")?;
    let pixels = width_usize
        .checked_mul(height_usize)
        .ok_or_else(|| unsupported(asset, "texture pixel count overflow"))?;
    if pixels == 0 || pixels > MAX_FACEGEN_TEXTURE_PIXELS {
        return Err(unsupported(
            asset,
            "texture dimensions are outside the bounded range",
        ));
    }
    if symmetric_count != FaceGenComponent::TextureSymmetric.expected_coefficients()
        || asymmetric_count != 0
    {
        return Err(unsupported(
            asset,
            format!(
                "expected 50 symmetric and 0 asymmetric modes, got {symmetric_count} and {asymmetric_count}"
            ),
        ));
    }
    let mode_count = symmetric_count;
    let per_mode = 4usize
        .checked_add(
            pixels
                .checked_mul(3)
                .ok_or_else(|| unsupported(asset, "texture mode size overflow"))?,
        )
        .ok_or_else(|| unsupported(asset, "texture mode size overflow"))?;
    let expected = EGT_HEADER_BYTES
        .checked_add(
            per_mode
                .checked_mul(mode_count)
                .ok_or_else(|| unsupported(asset, "file size overflow"))?,
        )
        .ok_or_else(|| unsupported(asset, "file size overflow"))?;
    if bytes.len() != expected {
        return Err(unsupported(
            asset,
            format!("expected {expected} bytes, got {}", bytes.len()),
        ));
    }

    let mut modes = Vec::with_capacity(mode_count);
    for _ in 0..mode_count {
        let scale = reader.f32().ok_or_else(|| truncated(asset))?;
        if !scale.is_finite() {
            return Err(unsupported(asset, "non-finite texture morph scale"));
        }
        let mut mode = vec![[0.0; 3]; pixels];
        for channel in 0..3 {
            for pixel in &mut mode {
                let value = i8::from_le_bytes([reader.take(1).ok_or_else(|| truncated(asset))?[0]]);
                pixel[channel] = f32::from(value) * scale;
                if !pixel[channel].is_finite() {
                    return Err(unsupported(asset, "non-finite texture delta"));
                }
            }
        }
        modes.push(mode);
    }
    debug_assert_eq!(reader.remaining(), 0);
    Ok(TextureMorph {
        width,
        height,
        symmetric_count,
        modes,
    })
}

/// Read the bounded structural header of a FaceGen TRI companion.  Static
/// NPC reconstruction needs its base/combined vertex and UV counts; the
/// expression morph payload is intentionally not interpreted here.  The
/// declared vertex, triangle, and quad blocks must nevertheless fit inside
/// the file before the header is accepted.
pub(crate) fn parse_tri_layout(bytes: &[u8]) -> Result<TriLayout, FaceGenDiagnostic> {
    let asset = FaceGenAssetKind::TriMorph;
    if bytes.len() < TRI_HEADER_BYTES {
        return Err(truncated(asset));
    }
    let mut reader = Reader::new(bytes);
    if reader.take(8) != Some(TRI_MAGIC.as_slice()) {
        return Err(unsupported(asset, "expected FRTRI003 magic"));
    }
    let base_vertex_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_VERTICES,
        "base vertex count",
    )?;
    if base_vertex_count == 0 {
        return Err(unsupported(asset, "base vertex count must be positive"));
    }
    let triangle_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_TRIANGLES,
        "triangle count",
    )?;
    let quad_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_TRIANGLES,
        "quad count",
    )?;
    let _labelled_vertex_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_VERTICES,
        "labelled vertex count",
    )?;
    let _labelled_surface_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_VERTICES,
        "labelled surface count",
    )?;
    let texture_coordinate_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_VERTICES,
        "texture coordinate count",
    )?;
    let _extension = reader.u32().ok_or_else(|| truncated(asset))?;
    let difference_morph_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_MODES,
        "difference morph count",
    )?;
    let static_morph_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_MODES,
        "static morph count",
    )?;
    let static_vertex_count = checked_count(
        asset,
        reader.u32().ok_or_else(|| truncated(asset))?,
        MAX_FACEGEN_VERTICES,
        "static morph vertex count",
    )?;
    reader.take(16).ok_or_else(|| truncated(asset))?;

    let combined_vertex_count = base_vertex_count
        .checked_add(static_vertex_count)
        .ok_or_else(|| unsupported(asset, "combined vertex count overflow"))?;
    let vertex_bytes = combined_vertex_count
        .checked_mul(12)
        .ok_or_else(|| unsupported(asset, "vertex payload size overflow"))?;
    let triangle_bytes = triangle_count
        .checked_mul(12)
        .ok_or_else(|| unsupported(asset, "triangle payload size overflow"))?;
    let quad_bytes = quad_count
        .checked_mul(16)
        .ok_or_else(|| unsupported(asset, "quad payload size overflow"))?;
    let minimum = TRI_HEADER_BYTES
        .checked_add(vertex_bytes)
        .and_then(|value| value.checked_add(triangle_bytes))
        .and_then(|value| value.checked_add(quad_bytes))
        .ok_or_else(|| unsupported(asset, "TRI payload size overflow"))?;
    if bytes.len() < minimum {
        return Err(unsupported(
            asset,
            format!(
                "TRI declares at least {minimum} bytes but has {}",
                bytes.len()
            ),
        ));
    }

    // These counts are needed only for the compatibility check.  Reading the
    // full expression section would make static actor preparation responsible
    // for the unrelated animation-morph grammar.
    let _ = (difference_morph_count, static_morph_count);
    Ok(TriLayout {
        base_vertex_count,
        combined_vertex_count,
        triangle_count,
        texture_coordinate_count,
    })
}

fn normalize(vector: [f32; 3], fallback: [f32; 3]) -> [f32; 3] {
    let length = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    if length.is_finite() && length > 1.0e-6 {
        [vector[0] / length, vector[1] / length, vector[2] / length]
    } else {
        fallback
    }
}

pub(crate) fn texture_dimensions_compatible(
    base_width: u32,
    base_height: u32,
    morph_width: u32,
    morph_height: u32,
) -> bool {
    morph_width > 0
        && morph_height > 0
        && base_width >= morph_width
        && base_height >= morph_height
        && base_width.is_multiple_of(morph_width)
        && base_height.is_multiple_of(morph_height)
}

fn sample_texture_delta(
    mode: &[[f32; 3]],
    morph_width: u32,
    morph_height: u32,
    output_x: u32,
    output_y: u32,
    output_width: u32,
    output_height: u32,
) -> [f32; 3] {
    // Fallout's native texture coordinates use a bottom-origin V axis while
    // decoded image rows and the FaceGen EGT raster are addressed from the
    // top.  Mirror the output row before sampling so a FaceGen delta lands on
    // the same facial feature as the base diffuse instead of being vertically
    // inverted.
    let output_y = output_height - 1 - output_y;
    if morph_width == output_width && morph_height == output_height {
        let index = (output_y * output_width + output_x) as usize;
        return mode[index];
    }

    let x = ((output_x as f32 + 0.5) * morph_width as f32 / output_width as f32) - 0.5;
    let y = ((output_y as f32 + 0.5) * morph_height as f32 / output_height as f32) - 0.5;
    let x0 = x.floor().clamp(0.0, morph_width.saturating_sub(1) as f32) as u32;
    let y0 = y.floor().clamp(0.0, morph_height.saturating_sub(1) as f32) as u32;
    let x1 = (x0 + 1).min(morph_width.saturating_sub(1));
    let y1 = (y0 + 1).min(morph_height.saturating_sub(1));
    let x_weight = (x - x0 as f32).clamp(0.0, 1.0);
    let y_weight = (y - y0 as f32).clamp(0.0, 1.0);
    let index = |x: u32, y: u32| (y * morph_width + x) as usize;
    let top_left = mode[index(x0, y0)];
    let top_right = mode[index(x1, y0)];
    let bottom_left = mode[index(x0, y1)];
    let bottom_right = mode[index(x1, y1)];
    let mut result = [0.0; 3];
    for channel in 0..3 {
        let top = top_left[channel] * (1.0 - x_weight) + top_right[channel] * x_weight;
        let bottom = bottom_left[channel] * (1.0 - x_weight) + bottom_right[channel] * x_weight;
        result[channel] = top * (1.0 - y_weight) + bottom * y_weight;
    }
    result
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn subtract(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn add_scaled(target: &mut [f32; 3], value: [f32; 3], scale: f32) {
    target[0] += value[0] * scale;
    target[1] += value[1] * scale;
    target[2] += value[2] * scale;
}

fn apply_mesh_morph(
    mesh: &mut nif::fo3::SceneMesh,
    morph: &GeometryMorph,
    coefficients: &FaceGenCoefficients,
    offset: usize,
) -> Result<(), FaceGenDiagnostic> {
    let old_normals = mesh.normals.clone();
    let old_tangents = mesh.tangents.clone();
    let old_positions = mesh.positions.clone();
    for (local_index, position) in mesh.positions.iter_mut().enumerate() {
        let vertex = offset + local_index;
        let mut delta = [0.0; 3];
        for (mode, coefficient) in morph
            .modes
            .iter()
            .take(morph.symmetric_count)
            .zip(coefficients.geometry_symmetric.iter().copied())
        {
            add_scaled(&mut delta, mode[vertex], coefficient);
        }
        for (mode, coefficient) in morph
            .modes
            .iter()
            .skip(morph.symmetric_count)
            .take(morph.asymmetric_count)
            .zip(coefficients.geometry_asymmetric.iter().copied())
        {
            add_scaled(&mut delta, mode[vertex], coefficient);
        }
        let updated = [
            position[0] + delta[0],
            position[1] + delta[1],
            position[2] + delta[2],
        ];
        if updated.iter().any(|value| !value.is_finite()) {
            return Err(unsupported(
                FaceGenAssetKind::GeometryMorph,
                format!("deformed vertex {vertex} is non-finite"),
            ));
        }
        *position = updated;
    }
    recompute_mesh_basis(mesh, &old_normals, &old_tangents, &old_positions)?;
    Ok(())
}

fn recompute_mesh_basis(
    mesh: &mut nif::fo3::SceneMesh,
    old_normals: &[[f32; 3]],
    old_tangents: &[[f32; 4]],
    _old_positions: &[[f32; 3]],
) -> Result<(), FaceGenDiagnostic> {
    if mesh.tex_coords.len() != mesh.positions.len() {
        return Err(unsupported(
            FaceGenAssetKind::GeometryMorph,
            "head mesh UV count does not match vertex count",
        ));
    }
    let vertex_count = mesh.positions.len();
    let mut normals = vec![[0.0; 3]; vertex_count];
    let mut tangents = vec![[0.0; 3]; vertex_count];
    let mut bitangents = vec![[0.0; 3]; vertex_count];
    for triangle in mesh.indices.chunks_exact(3) {
        let i0 = usize::from(triangle[0]);
        let i1 = usize::from(triangle[1]);
        let i2 = usize::from(triangle[2]);
        if i0 >= vertex_count || i1 >= vertex_count || i2 >= vertex_count {
            return Err(unsupported(
                FaceGenAssetKind::GeometryMorph,
                "head mesh index is outside the deformed vertex range",
            ));
        }
        let edge1 = subtract(mesh.positions[i1], mesh.positions[i0]);
        let edge2 = subtract(mesh.positions[i2], mesh.positions[i0]);
        let face_normal = cross(edge1, edge2);
        for index in [i0, i1, i2] {
            add_scaled(&mut normals[index], face_normal, 1.0);
        }

        let uv1 = [
            mesh.tex_coords[i1][0] - mesh.tex_coords[i0][0],
            mesh.tex_coords[i1][1] - mesh.tex_coords[i0][1],
        ];
        let uv2 = [
            mesh.tex_coords[i2][0] - mesh.tex_coords[i0][0],
            mesh.tex_coords[i2][1] - mesh.tex_coords[i0][1],
        ];
        let determinant = uv1[0] * uv2[1] - uv2[0] * uv1[1];
        if determinant.abs() <= 1.0e-8 || !determinant.is_finite() {
            continue;
        }
        let inverse = 1.0 / determinant;
        let tangent = [
            (edge1[0] * uv2[1] - edge2[0] * uv1[1]) * inverse,
            (edge1[1] * uv2[1] - edge2[1] * uv1[1]) * inverse,
            (edge1[2] * uv2[1] - edge2[2] * uv1[1]) * inverse,
        ];
        let bitangent = [
            (edge2[0] * uv1[0] - edge1[0] * uv2[0]) * inverse,
            (edge2[1] * uv1[0] - edge1[1] * uv2[0]) * inverse,
            (edge2[2] * uv1[0] - edge1[2] * uv2[0]) * inverse,
        ];
        for index in [i0, i1, i2] {
            add_scaled(&mut tangents[index], tangent, 1.0);
            add_scaled(&mut bitangents[index], bitangent, 1.0);
        }
    }

    mesh.normals.resize(vertex_count, [0.0; 3]);
    mesh.tangents.resize(vertex_count, [0.0; 4]);
    for index in 0..vertex_count {
        let fallback_normal = old_normals.get(index).copied().unwrap_or([0.0, 1.0, 0.0]);
        let normal = normalize(normals[index], normalize(fallback_normal, [0.0, 1.0, 0.0]));
        let fallback_tangent = old_tangents
            .get(index)
            .copied()
            .unwrap_or([1.0, 0.0, 0.0, 1.0]);
        let projected = subtract(
            tangents[index],
            [
                normal[0] * dot(normal, tangents[index]),
                normal[1] * dot(normal, tangents[index]),
                normal[2] * dot(normal, tangents[index]),
            ],
        );
        let tangent = normalize(
            projected,
            normalize(
                [
                    fallback_tangent[0],
                    fallback_tangent[1],
                    fallback_tangent[2],
                ],
                [1.0, 0.0, 0.0],
            ),
        );
        let handedness: f32 = if dot(cross(normal, tangent), bitangents[index]) < 0.0 {
            -1.0
        } else {
            1.0
        };
        if normal.iter().any(|value| !value.is_finite())
            || tangent.iter().any(|value| !value.is_finite())
            || !handedness.is_finite()
        {
            return Err(unsupported(
                FaceGenAssetKind::GeometryMorph,
                format!("deformed vertex basis {index} is non-finite"),
            ));
        }
        mesh.normals[index] = normal;
        mesh.tangents[index] = [tangent[0], tangent[1], tangent[2], handedness];
    }
    Ok(())
}

pub(crate) fn apply_geometry_morph(
    scene: &mut nif::fo3::Scene,
    morph: &GeometryMorph,
    coefficients: &FaceGenCoefficients,
    base_vertex_count: usize,
) -> Result<(), FaceGenDiagnostic> {
    let actual_vertices = scene
        .nodes
        .iter()
        .filter_map(|node| node.mesh.as_ref())
        .map(|mesh| mesh.positions.len())
        .sum::<usize>();
    if base_vertex_count != actual_vertices {
        return Err(FaceGenDiagnostic::GeometryBaseVertexCountMismatch {
            expected: base_vertex_count,
            actual: actual_vertices,
        });
    }
    if morph.vertex_count < base_vertex_count {
        return Err(FaceGenDiagnostic::GeometryVertexCountMismatch {
            expected: base_vertex_count,
            actual: morph.vertex_count,
        });
    }
    let mut offset = 0;
    for node in &mut scene.nodes {
        let Some(mesh) = node.mesh.as_mut() else {
            continue;
        };
        apply_mesh_morph(mesh, morph, coefficients, offset)?;
        offset += mesh.positions.len();
    }
    Ok(())
}

pub(crate) fn synthesize_head_diffuse(
    base_bytes: &[u8],
    morph: &TextureMorph,
    coefficients: &FaceGenCoefficients,
) -> Result<Vec<u8>, FaceGenDiagnostic> {
    let image = image::load_from_memory(base_bytes).map_err(|error| {
        unsupported(
            FaceGenAssetKind::TextureMorph,
            format!("head diffuse texture could not be decoded: {error}"),
        )
    })?;
    let mut rgba = image.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    if !texture_dimensions_compatible(width, height, morph.width, morph.height) {
        return Err(FaceGenDiagnostic::TextureDimensionsMismatch {
            expected_width: morph.width,
            expected_height: morph.height,
            actual_width: width,
            actual_height: height,
        });
    }
    let morph_pixel_count = usize::try_from(morph.width)
        .ok()
        .and_then(|width| {
            usize::try_from(morph.height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .ok_or_else(|| {
            unsupported(
                FaceGenAssetKind::TextureMorph,
                "texture morph dimensions overflow",
            )
        })?;
    if morph
        .modes
        .iter()
        .any(|mode| mode.len() != morph_pixel_count)
    {
        return Err(unsupported(
            FaceGenAssetKind::TextureMorph,
            "texture morph mode pixel count does not match its dimensions",
        ));
    }
    for (pixel_index, pixel) in rgba.pixels_mut().enumerate() {
        let output_x = pixel_index as u32 % width;
        let output_y = pixel_index as u32 / width;
        let mut rgb = [
            f32::from(pixel[0]),
            f32::from(pixel[1]),
            f32::from(pixel[2]),
        ];
        for (mode, coefficient) in morph
            .modes
            .iter()
            .take(morph.symmetric_count)
            .zip(coefficients.texture_symmetric.iter().copied())
        {
            let delta = sample_texture_delta(
                mode,
                morph.width,
                morph.height,
                output_x,
                output_y,
                width,
                height,
            );
            for channel in 0..3 {
                rgb[channel] += delta[channel] * coefficient;
            }
        }
        if rgb.iter().any(|value| !value.is_finite()) {
            return Err(unsupported(
                FaceGenAssetKind::TextureMorph,
                format!("synthesized pixel {pixel_index} is non-finite"),
            ));
        }
        pixel[0] = rgb[0].round().clamp(0.0, 255.0) as u8;
        pixel[1] = rgb[1].round().clamp(0.0, 255.0) as u8;
        pixel[2] = rgb[2].round().clamp(0.0, 255.0) as u8;
    }
    let mut encoded = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(rgba)
        .write_to(&mut encoded, image::ImageFormat::Png)
        .map_err(|error| unsupported(FaceGenAssetKind::TextureMorph, error.to_string()))?;
    Ok(encoded.into_inner())
}

pub(crate) fn facegen_texture_key(source_path: &str, coefficients: &FaceGenCoefficients) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in source_path.bytes().chain(
        coefficients
            .texture_symmetric
            .iter()
            .flat_map(|value| value.to_le_bytes()),
    ) {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("__bevyout_facegen/{hash:016x}.png")
}

#[cfg(test)]
#[path = "tests/facegen.rs"]
mod tests;
