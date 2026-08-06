//! Native, deterministic UV1 generation for composed static primitives.

use super::rust_scene::ComposedPrimitive;
use anyhow::{Context, Result, bail};
use bevy::math::Vec2;
use bevyout_xatlas::{MeshInput, Options};

#[cfg(test)]
#[path = "tests/lightmap_uv.rs"]
mod tests;

pub(crate) const LIGHTMAP_PADDING_TEXELS: u32 = 12;

pub(crate) fn unwrap_primitive(primitive: &mut ComposedPrimitive) -> Result<()> {
    if primitive.positions.is_empty() || primitive.indices.len() < 3 {
        bail!("cannot unwrap empty lightmap primitive {}", primitive.name);
    }
    if primitive.positions.len() != primitive.normals.len()
        || primitive.positions.len() != primitive.uvs.len()
        || primitive.positions.len() != primitive.colors.len()
        || primitive.positions.len() != primitive.transport_colors.len()
    {
        bail!(
            "lightmap primitive {} has mismatched vertex attributes",
            primitive.name
        );
    }

    let positions = primitive
        .positions
        .iter()
        .map(|value| value.to_array())
        .collect::<Vec<_>>();
    let normals = primitive
        .normals
        .iter()
        .map(|value| value.to_array())
        .collect::<Vec<_>>();
    let uvs = primitive
        .uvs
        .iter()
        .map(|value| value.to_array())
        .collect::<Vec<_>>();
    let generated = bevyout_xatlas::generate(
        MeshInput {
            positions: &positions,
            normals: Some(&normals),
            uvs: Some(&uvs),
            indices: &primitive.indices,
        },
        Options {
            resolution: 0,
            texels_per_unit: primitive.lightmap_texels_per_meter,
            padding: LIGHTMAP_PADDING_TEXELS,
            max_chart_size: 4096,
            block_align: true,
            fix_winding: true,
        },
    )
    .map_err(|error| anyhow::anyhow!("xatlas rejected {}: {error}", primitive.name))?;
    if generated.atlas_count != 1 {
        bail!(
            "lightmap primitive {} spans {} atlas pages",
            primitive.name,
            generated.atlas_count
        );
    }
    let width = generated.width.max(1) as f32;
    let height = generated.height.max(1) as f32;

    let mut positions = Vec::with_capacity(generated.vertices.len());
    let mut normals = Vec::with_capacity(generated.vertices.len());
    let mut uvs = Vec::with_capacity(generated.vertices.len());
    let mut colors = Vec::with_capacity(generated.vertices.len());
    let mut transport_colors = Vec::with_capacity(generated.vertices.len());
    let mut uv1 = Vec::with_capacity(generated.vertices.len());
    let mut uv1_chart_ids = Vec::with_capacity(generated.vertices.len());
    for vertex in generated.vertices {
        if vertex.atlas_index != 0 {
            bail!(
                "lightmap primitive {} emitted nonzero atlas index {}",
                primitive.name,
                vertex.atlas_index
            );
        }
        if vertex.chart_index < 0 {
            bail!(
                "lightmap primitive {} emitted invalid chart index {}",
                primitive.name,
                vertex.chart_index
            );
        }
        let source_index = vertex.xref as usize;
        positions.push(
            *primitive
                .positions
                .get(source_index)
                .context("xatlas position xref is out of range")?,
        );
        normals.push(
            *primitive
                .normals
                .get(source_index)
                .context("xatlas normal xref is out of range")?,
        );
        uvs.push(
            *primitive
                .uvs
                .get(source_index)
                .context("xatlas UV0 xref is out of range")?,
        );
        colors.push(
            *primitive
                .colors
                .get(source_index)
                .context("xatlas color xref is out of range")?,
        );
        transport_colors.push(
            *primitive
                .transport_colors
                .get(source_index)
                .context("xatlas transport-color xref is out of range")?,
        );
        uv1.push(Vec2::new(vertex.uv[0] / width, vertex.uv[1] / height));
        uv1_chart_ids.push(vertex.chart_index as u32);
    }

    primitive.positions = positions;
    primitive.normals = normals;
    primitive.uvs = uvs;
    primitive.colors = colors;
    primitive.transport_colors = transport_colors;
    primitive.indices = generated.indices;
    primitive.uv1 = uv1;
    primitive.uv1_chart_ids = uv1_chart_ids;
    primitive.lightmap_dimensions = [generated.width.max(1), generated.height.max(1)];
    primitive.lightmap_binding_id = Some(stable_binding_id(&primitive.primitive_key));
    Ok(())
}

fn stable_binding_id(key: &str) -> u32 {
    let hash = key.bytes().fold(0x811c_9dc5_u32, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(0x0100_0193)
    });
    hash.max(1)
}
