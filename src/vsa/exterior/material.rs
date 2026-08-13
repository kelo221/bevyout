//! Native LAND material preparation.
//!
//! Fallout terrain is authored as sparse quadrant layer weights, not as one
//! diffuse image.  The runtime deliberately does not know how to open ESM/BSA
//! assets, so this module resolves the LTEX sources during `prepare` and
//! writes one deterministic, mip-friendly cell albedo PNG. Missing LTEX
//! overlays are diagnosed and skipped; zero/base-less quadrants use Fallout
//! 3's authored default wasteland dirt before falling back to LAND vertex
//! colours.

use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use std::collections::{BTreeSet, HashMap};
use std::io::Cursor;
use std::path::Path;

use bevyout_core::manifest::exterior::PreparedTerrain;

use super::super::assets::{flip_directx_normal_y_texel, resolve_asset};
use super::super::bsa::BsaArchive;
use super::super::cache_store::{
    CandidateObject, FsPreparedObjectStore, PreparedObjectKind, PreparedObjectStore,
    PreparedRecipeInputs, normalize_source_path,
};
use super::super::manifest::Diagnostic;
use super::super::openmw_esm4::{
    LandTextureAssignment, LandTextureWeight, LandscapeTextureRecord, TextureSetRecord,
};
use super::super::paths::fingerprint;

const OUTPUT_SIZE: u32 = 1024;
// OpenMW's ESM4 terrain path uses six texture tiles per quadrant, hence twelve
// tiles across one 4096-unit cell. Keep the bake self-contained, but retain
// the source's per-quadrant BTXT/ATXT/VTXT semantics while doing so.
const TEXTURE_TILES_PER_CELL: f32 = 12.0;
const TERRAIN_MATERIAL_REVISION: &str =
    "terrain-material-v7-quadrant-layers-12-tiles-shared-payloads";
const TERRAIN_IMAGE_FORMAT_REVISION: &str = "png-rgba8-v1";

struct TerrainLayerSource {
    diffuse: Option<RgbaImage>,
    normal: Option<RgbaImage>,
}

struct LayerPlan {
    form_id: u32,
    layer: u16,
    base: bool,
    weights: Vec<f32>,
}

struct MaterialSample {
    color: [f32; 3],
    normal: [f32; 3],
    normal_present: bool,
    specular: f32,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare_terrain_albedo(
    terrain: &mut PreparedTerrain,
    assignments: &[LandTextureAssignment],
    landscape_textures: &HashMap<u32, LandscapeTextureRecord>,
    texture_sets: &HashMap<u32, TextureSetRecord>,
    data_root: &Path,
    archives: &[BsaArchive],
    cache_dir: &Path,
    source_fingerprint: &str,
    cell_form_id: u32,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    if !terrain.is_well_formed() {
        return Ok(());
    }
    let mut layer_ids = BTreeSet::new();
    layer_ids.extend(
        terrain
            .texture_layers
            .iter()
            .copied()
            .filter(|form_id| *form_id != 0),
    );
    layer_ids.extend(
        assignments
            .iter()
            .map(|assignment| assignment.form_id)
            .filter(|form_id| *form_id != 0),
    );
    let mut source_images = HashMap::with_capacity(layer_ids.len());
    for form_id in layer_ids {
        source_images.insert(
            form_id,
            resolve_layer_source(
                form_id,
                landscape_textures,
                texture_sets,
                data_root,
                archives,
                diagnostics,
            )?,
        );
    }

    let mut quadrant_layers: [Vec<LayerPlan>; 4] = std::array::from_fn(|_| Vec::new());
    for assignment in assignments {
        let quadrant = usize::from(assignment.quadrant);
        if quadrant < quadrant_layers.len() {
            quadrant_layers[quadrant].push(LayerPlan {
                form_id: assignment.form_id,
                layer: assignment.layer,
                base: assignment.base,
                weights: dense_assignment_weights(&assignment.weights),
            });
        }
    }
    let needs_default_texture = assignments
        .iter()
        .any(|assignment| assignment.base && assignment.form_id == 0)
        || (0..quadrant_layers.len())
            .any(|quadrant| !quadrant_layers[quadrant].iter().any(|layer| layer.base));
    if needs_default_texture {
        source_images.insert(
            0,
            resolve_default_layer_source(data_root, archives, diagnostics)?,
        );
    }
    for layers in &mut quadrant_layers {
        layers.sort_by_key(|layer| (!layer.base, layer.layer, layer.form_id));
    }
    for (quadrant, layers) in quadrant_layers.iter().enumerate() {
        let layers = layers
            .iter()
            .map(|layer| {
                format!(
                    "{:08x}:{}{}",
                    layer.form_id,
                    layer.layer,
                    if layer.base { "B" } else { "A" }
                )
            })
            .collect::<Vec<_>>();
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!(
                "LAND material quadrant {quadrant} layers [{}]",
                layers.join(",")
            ),
        });
    }

    let mut output = RgbaImage::new(OUTPUT_SIZE, OUTPUT_SIZE);
    let has_normal_layers = source_images.values().any(|source| source.normal.is_some());
    let mut normal_output = RgbaImage::new(OUTPUT_SIZE, OUTPUT_SIZE);
    for y in 0..OUTPUT_SIZE {
        for x in 0..OUTPUT_SIZE {
            let u = x as f32 / (OUTPUT_SIZE - 1) as f32;
            let v = y as f32 / (OUTPUT_SIZE - 1) as f32;
            let vertex_color =
                sample_grid_color(&terrain.colors, terrain.width, terrain.height, u, v);
            let (quadrant, local_u, local_v) = quadrant_coordinates(u, v);
            let sample = if quadrant_layers[quadrant].is_empty() {
                sample_legacy_blend(terrain, &source_images, vertex_color, u, v)
            } else {
                sample_quadrant_layers(
                    &quadrant_layers[quadrant],
                    &source_images,
                    vertex_color,
                    u,
                    v,
                    local_u,
                    local_v,
                )
            };
            let mut color = sample.color;
            // VCLR is authored ambient/tint data. Keep it as a gentle baked
            // modulation so a missing/zero channel cannot turn an otherwise
            // valid terrain layer into a black hole under PBR lighting.
            for component in 0..3 {
                color[component] *= 0.55 + vertex_color[component] * 0.45;
            }
            output.put_pixel(
                x,
                y,
                Rgba([
                    (color[0].clamp(0.0, 1.0) * 255.0).round() as u8,
                    (color[1].clamp(0.0, 1.0) * 255.0).round() as u8,
                    (color[2].clamp(0.0, 1.0) * 255.0).round() as u8,
                    255,
                ]),
            );
            if has_normal_layers {
                normal_output.put_pixel(
                    x,
                    y,
                    Rgba([
                        ((sample.normal[0] * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0).round() as u8,
                        ((sample.normal[1] * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0).round() as u8,
                        ((sample.normal[2] * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0).round() as u8,
                        (sample.specular.clamp(0.0, 1.0) * 255.0).round() as u8,
                    ]),
                );
            }
        }
    }

    let store = FsPreparedObjectStore::open(cache_dir)?;
    terrain.albedo_asset_path = Some(publish_terrain_image(
        &store,
        source_fingerprint,
        cell_form_id,
        "albedo",
        output,
    )?);
    terrain.normal_asset_path = if has_normal_layers {
        Some(publish_terrain_image(
            &store,
            source_fingerprint,
            cell_form_id,
            "normal-specular",
            normal_output,
        )?)
    } else {
        None
    };
    Ok(())
}

fn publish_terrain_image(
    store: &FsPreparedObjectStore,
    source_fingerprint: &str,
    cell_form_id: u32,
    role: &str,
    image: RgbaImage,
) -> Result<String> {
    let mut encoded = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image)
        .write_to(&mut encoded, ImageFormat::Png)
        .context("encoding prepared terrain PNG")?;
    let recipe = PreparedRecipeInputs {
        recipe_version: 1,
        kind: PreparedObjectKind::Texture,
        source_identity: normalize_source_path(&format!(
            "generated/terrain/{cell_form_id:08x}-{role}.png"
        ))?,
        input_hashes: vec![fingerprint(source_fingerprint.as_bytes())],
        converter_revision: TERRAIN_MATERIAL_REVISION.into(),
        format_policy_revision: TERRAIN_IMAGE_FORMAT_REVISION.into(),
        canonical_settings: Vec::new(),
    };
    let object = store.publish(
        &recipe,
        CandidateObject::from_bytes(PreparedObjectKind::Texture, "png", encoded.into_inner()),
    )?;
    Ok(store.object_asset_path(&object))
}

fn resolve_layer_source(
    form_id: u32,
    landscape_textures: &HashMap<u32, LandscapeTextureRecord>,
    texture_sets: &HashMap<u32, TextureSetRecord>,
    data_root: &Path,
    archives: &[BsaArchive],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<TerrainLayerSource> {
    let Some(texture) = landscape_textures.get(&form_id) else {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!("LAND texture {form_id:08x} has no LTEX record"),
        });
        return Ok(TerrainLayerSource {
            diffuse: None,
            normal: None,
        });
    };
    let texture_set = texture
        .texture_set_form_id
        .and_then(|texture_set_form_id| texture_sets.get(&texture_set_form_id));
    let diffuse_path = texture_set
        .and_then(|texture_set| texture_set.diffuse_path.as_deref())
        .or(texture.diffuse_path.as_deref());
    let diffuse = if let Some(path) = diffuse_path {
        let bytes = resolve_asset(data_root, archives, path)
            .with_context(|| format!("reading LAND texture {path}"))?;
        match bytes {
            Some(bytes) => match image::load_from_memory(&bytes) {
                Ok(image) => {
                    let image = image.to_rgba8();
                    diagnostics.push(Diagnostic {
                        severity: "info".into(),
                        message: format!(
                            "LAND texture {form_id:08x} uses {path} ({}x{})",
                            image.width(),
                            image.height()
                        ),
                    });
                    Some(image)
                }
                Err(error) => {
                    diagnostics.push(Diagnostic {
                        severity: "warning".into(),
                        message: format!(
                            "could not decode LAND texture {path} for LTEX {form_id:08x}: {error}"
                        ),
                    });
                    None
                }
            },
            None => {
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!("missing LAND texture {path} for LTEX {form_id:08x}"),
                });
                None
            }
        }
    } else {
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!(
                "LAND texture {form_id:08x} ({}) has no diffuse source",
                texture.editor_id.as_deref().unwrap_or("<unnamed>")
            ),
        });
        None
    };

    let normal = texture_set
        .and_then(|texture_set| texture_set.normal_path.as_deref())
        .map(|normal_path| -> Result<Option<RgbaImage>> {
            let bytes = resolve_asset(data_root, archives, normal_path)
                .with_context(|| format!("reading LAND normal texture {normal_path}"))?;
            let Some(bytes) = bytes else {
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!(
                        "missing LAND normal texture {normal_path} for LTEX {form_id:08x}"
                    ),
                });
                return Ok(None);
            };
            let mut image = image::load_from_memory(&bytes)
                .with_context(|| {
                    format!("decoding LAND normal texture {normal_path} for LTEX {form_id:08x}")
                })?
                .to_rgba8();
            for pixel in image.pixels_mut() {
                flip_directx_normal_y_texel(&mut pixel.0);
            }
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "LAND texture {form_id:08x} uses normal {normal_path} ({}x{})",
                    image.width(),
                    image.height()
                ),
            });
            Ok(Some(image))
        })
        .transpose()?
        .flatten();

    Ok(TerrainLayerSource { diffuse, normal })
}

fn resolve_default_layer_source(
    data_root: &Path,
    archives: &[BsaArchive],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<TerrainLayerSource> {
    let diffuse = resolve_default_texture(
        data_root,
        archives,
        "textures/landscape/dirtwasteland01.dds",
        false,
        diagnostics,
    )?;
    let normal = resolve_default_texture(
        data_root,
        archives,
        "textures/landscape/dirtwasteland01_n.dds",
        true,
        diagnostics,
    )?;
    Ok(TerrainLayerSource { diffuse, normal })
}

fn resolve_default_texture(
    data_root: &Path,
    archives: &[BsaArchive],
    path: &str,
    normal_map: bool,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Option<RgbaImage>> {
    let bytes = resolve_asset(data_root, archives, path)
        .with_context(|| format!("reading default LAND texture {path}"))?;
    let Some(bytes) = bytes else {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!("missing default LAND texture {path}"),
        });
        return Ok(None);
    };
    let mut image = match image::load_from_memory(&bytes) {
        Ok(image) => image.to_rgba8(),
        Err(error) => {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!("could not decode default LAND texture {path}: {error}"),
            });
            return Ok(None);
        }
    };
    if normal_map {
        for pixel in image.pixels_mut() {
            flip_directx_normal_y_texel(&mut pixel.0);
        }
    }
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: format!(
            "LAND default texture uses {path} ({}x{})",
            image.width(),
            image.height()
        ),
    });
    Ok(Some(image))
}

fn sample_quadrant_layers(
    layers: &[LayerPlan],
    source_images: &HashMap<u32, TerrainLayerSource>,
    fallback_color: [f32; 3],
    u: f32,
    v: f32,
    local_u: f32,
    local_v: f32,
) -> MaterialSample {
    let mut sample = MaterialSample {
        color: fallback_color,
        normal: [0.0, 1.0, 0.0],
        normal_present: false,
        specular: 0.0,
    };
    let default_source = source_images.get(&0);
    let base_form_id = layers
        .iter()
        .find(|layer| layer.base)
        .map_or(0, |layer| layer.form_id);
    let source = source_images.get(&base_form_id);
    if let Some(image) = source
        .and_then(|source| source.diffuse.as_ref())
        .or_else(|| default_source.and_then(|source| source.diffuse.as_ref()))
    {
        sample.color = sample_tiled(image, u, v, TEXTURE_TILES_PER_CELL);
    }
    if let Some(image) = source
        .and_then(|source| source.normal.as_ref())
        .or_else(|| default_source.and_then(|source| source.normal.as_ref()))
    {
        let texel = sample_tiled_rgba(image, u, v, TEXTURE_TILES_PER_CELL);
        sample.normal = tangent_from_texel(texel);
        sample.normal_present = true;
        sample.specular = texel[3];
    }
    for layer in layers.iter().filter(|layer| !layer.base) {
        let opacity = sample_assignment_weight(&layer.weights, local_u, local_v);
        if opacity <= f32::EPSILON {
            continue;
        }
        let Some(source) = source_images.get(&layer.form_id) else {
            continue;
        };
        if let Some(image) = source.diffuse.as_ref() {
            blend_rgb(
                &mut sample.color,
                sample_tiled(image, u, v, TEXTURE_TILES_PER_CELL),
                opacity,
            );
        }
        if let Some(image) = source.normal.as_ref() {
            let texel = sample_tiled_rgba(image, u, v, TEXTURE_TILES_PER_CELL);
            blend_rgb(&mut sample.normal, tangent_from_texel(texel), opacity);
            sample.normal = normalize_vector(sample.normal);
            sample.normal_present = true;
            sample.specular = sample.specular * (1.0 - opacity) + texel[3] * opacity;
        }
    }
    sample
}

fn sample_legacy_blend(
    terrain: &PreparedTerrain,
    source_images: &HashMap<u32, TerrainLayerSource>,
    fallback_color: [f32; 3],
    u: f32,
    v: f32,
) -> MaterialSample {
    let weights = sample_grid_rgba(&terrain.blend_weights, terrain.width, terrain.height, u, v);
    let total = weights.iter().sum::<f32>();
    if total <= f32::EPSILON {
        return MaterialSample {
            color: fallback_color,
            normal: [0.0, 1.0, 0.0],
            normal_present: false,
            specular: 0.0,
        };
    }
    let mut sample = MaterialSample {
        color: [0.0; 3],
        normal: [0.0; 3],
        normal_present: false,
        specular: 0.0,
    };
    let mut normal_weight = 0.0;
    for (channel, form_id) in terrain.texture_layers.iter().take(4).enumerate() {
        let weight = weights[channel] / total;
        let color = source_images
            .get(form_id)
            .and_then(|source| source.diffuse.as_ref())
            .map_or(fallback_color, |image| {
                sample_tiled(image, u, v, TEXTURE_TILES_PER_CELL)
            });
        for (component, value) in color.into_iter().enumerate() {
            sample.color[component] += value * weight;
        }
        if let Some(image) = source_images
            .get(form_id)
            .and_then(|source| source.normal.as_ref())
        {
            let texel = sample_tiled_rgba(image, u, v, TEXTURE_TILES_PER_CELL);
            let tangent = tangent_from_texel(texel);
            for (component, value) in tangent.into_iter().enumerate() {
                sample.normal[component] += value * weight;
            }
            normal_weight += weight;
            sample.specular += texel[3] * weight;
            sample.normal_present = true;
        }
    }
    if sample.normal_present && normal_weight > f32::EPSILON {
        sample.normal = normalize_vector(sample.normal);
    } else {
        sample.normal = [0.0, 1.0, 0.0];
    }
    sample
}

fn dense_assignment_weights(weights: &[LandTextureWeight]) -> Vec<f32> {
    let mut dense = vec![0.0; 17 * 17];
    for weight in weights {
        let index = usize::from(weight.position);
        if index < dense.len() {
            dense[index] = weight.opacity.clamp(0.0, 1.0);
        }
    }
    dense
}

fn sample_assignment_weight(weights: &[f32], u: f32, v: f32) -> f32 {
    if weights.len() != 17 * 17 {
        return 0.0;
    }
    let x = u.clamp(0.0, 1.0) * 16.0;
    let y = v.clamp(0.0, 1.0) * 16.0;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(16);
    let y1 = (y0 + 1).min(16);
    let tx = x.fract();
    let ty = y.fract();
    let at = |x: usize, y: usize| weights[y * 17 + x];
    let top = at(x0, y0) * (1.0 - tx) + at(x1, y0) * tx;
    let bottom = at(x0, y1) * (1.0 - tx) + at(x1, y1) * tx;
    (top * (1.0 - ty) + bottom * ty).clamp(0.0, 1.0)
}

fn quadrant_coordinates(u: f32, v: f32) -> (usize, f32, f32) {
    let x = u.clamp(0.0, 1.0) * 32.0;
    let y = v.clamp(0.0, 1.0) * 32.0;
    let right = x > 16.0;
    let top = y > 16.0;
    let local_u = (if right { x - 16.0 } else { x }) / 16.0;
    let local_v = (if top { y - 16.0 } else { y }) / 16.0;
    let quadrant = (if right { 1 } else { 0 }) + if top { 2 } else { 0 };
    (quadrant, local_u, local_v)
}

fn blend_rgb(destination: &mut [f32; 3], source: [f32; 3], opacity: f32) {
    for component in 0..3 {
        destination[component] =
            destination[component] * (1.0 - opacity) + source[component] * opacity;
    }
}

fn tangent_from_texel(texel: [f32; 4]) -> [f32; 3] {
    normalize_vector([
        texel[0] * 2.0 - 1.0,
        texel[1] * 2.0 - 1.0,
        texel[2] * 2.0 - 1.0,
    ])
}

fn sample_grid_rgba(values: &[[u8; 4]], width: u16, height: u16, u: f32, v: f32) -> [f32; 4] {
    let sample = |x: usize, y: usize| {
        values
            .get(y * usize::from(width) + x)
            .copied()
            .unwrap_or([255, 0, 0, 0])
    };
    let x = u.clamp(0.0, 1.0) * f32::from(width.saturating_sub(1));
    let y = v.clamp(0.0, 1.0) * f32::from(height.saturating_sub(1));
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(usize::from(width.saturating_sub(1)));
    let y1 = (y0 + 1).min(usize::from(height.saturating_sub(1)));
    let tx = x.fract();
    let ty = y.fract();
    let a = sample(x0, y0);
    let b = sample(x1, y0);
    let c = sample(x0, y1);
    let d = sample(x1, y1);
    std::array::from_fn(|channel| {
        let top = f32::from(a[channel]) * (1.0 - tx) + f32::from(b[channel]) * tx;
        let bottom = f32::from(c[channel]) * (1.0 - tx) + f32::from(d[channel]) * tx;
        (top * (1.0 - ty) + bottom * ty) / 255.0
    })
}

fn sample_grid_color(values: &[[u8; 4]], width: u16, height: u16, u: f32, v: f32) -> [f32; 3] {
    let rgba = sample_grid_rgba(values, width, height, u, v);
    [rgba[0], rgba[1], rgba[2]]
}

fn sample_tiled(image: &RgbaImage, u: f32, v: f32, tiles: f32) -> [f32; 3] {
    let pixel = sample_tiled_rgba(image, u, v, tiles);
    [pixel[0], pixel[1], pixel[2]]
}

fn sample_tiled_rgba(image: &RgbaImage, u: f32, v: f32, tiles: f32) -> [f32; 4] {
    let x = (u * tiles).fract() * image.width() as f32;
    let y = (v * tiles).fract() * image.height() as f32;
    let pixel = image.get_pixel(
        (x.floor() as u32).min(image.width().saturating_sub(1)),
        (y.floor() as u32).min(image.height().saturating_sub(1)),
    );
    [
        f32::from(pixel[0]) / 255.0,
        f32::from(pixel[1]) / 255.0,
        f32::from(pixel[2]) / 255.0,
        f32::from(pixel[3]) / 255.0,
    ]
}

fn normalize_vector(value: [f32; 3]) -> [f32; 3] {
    let length = (value[0] * value[0] + value[1] * value[1] + value[2] * value[2]).sqrt();
    if length.is_finite() && length > f32::EPSILON {
        [value[0] / length, value[1] / length, value[2] / length]
    } else {
        [0.0, 1.0, 0.0]
    }
}
