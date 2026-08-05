//! CPU direct-diffuse surface lightmaps.
//!
//! Rasterization first writes one deterministic intermediate page per composed
//! primitive. The pages are then chart-dilated and packed into atlas images,
//! keeping the binding contract explicit while applying feature-guided
//! denoising before chart-safe dilation.

use super::JobLight;
#[cfg(feature = "lightmap-gpu-solari")]
use super::backend::solari::{
    SOLARI_BAKE_MAX_BOUNCES, SolariBakeDirectionalLight, SolariBakeSession, SolariBakeTexel,
};
use super::cache::{TileCache, TileKey, TileRecord};
use super::denoise::{DenoiseFeature, denoise};
use super::environment::EnvironmentMap;
use super::rust_irradiance::{
    DirectionalBakeLight, EmissiveSampler, IrradianceTriangle, collect_triangles,
    surface_irradiance_with_emissive_and_environment,
};
use super::rust_scene::RustBakeScene;
use super::transport::adaptive::AdaptiveEstimator;
use super::transport::material::sample_material;
use crate::cli::progress::ProgressReporter;
use anyhow::{Context, Result, bail};
use bevy::math::{Vec2, Vec3, Vec4};
use bvh::bounding_hierarchy::BoundingHierarchy;
use bvh::bvh::Bvh;
use half::f16;
use image::{Rgba, RgbaImage};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) struct LightmapPage {
    pub(crate) primitive_index: usize,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) raw_path: PathBuf,
    pub(crate) covered_texels: usize,
    pub(crate) dilated_texels: usize,
    pub(crate) atlas_index: usize,
    pub(crate) atlas_offset: [u32; 2],
}

pub(crate) struct LightmapAtlas {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) raw_path: PathBuf,
    pub(crate) content_hash: String,
}

pub(crate) const LIGHTMAP_ATLAS_PAGE_SIZE: u32 = 4096;
const LIGHTMAP_ATLAS_GUTTER_TEXELS: u32 = 2;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct LightmapSamplingSettings {
    pub(crate) min_samples: u32,
    pub(crate) max_samples: u32,
    pub(crate) variance_threshold: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct LightmapDenoiseSettings {
    pub(crate) iterations: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct LightmapDebugSettings {
    pub(crate) uv: bool,
    pub(crate) samples: bool,
    pub(crate) variance: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct LightmapSamplingSummary {
    pub(crate) sampled_texels: usize,
    pub(crate) total_samples: u64,
    pub(crate) min_samples: u32,
    pub(crate) max_samples: u32,
    pub(crate) max_relative_variance: f32,
}

impl LightmapSamplingSummary {
    fn record(&mut self, statistics: &AdaptiveEstimator) {
        let sample_count = statistics.sample_count();
        self.sampled_texels += 1;
        self.total_samples += u64::from(sample_count);
        if self.min_samples == 0 || sample_count < self.min_samples {
            self.min_samples = sample_count;
        }
        self.max_samples = self.max_samples.max(sample_count);
        self.max_relative_variance = self
            .max_relative_variance
            .max(statistics.relative_variance());
    }

    fn merge(&mut self, other: Self) {
        if other.sampled_texels == 0 {
            return;
        }
        if self.min_samples == 0 || other.min_samples < self.min_samples {
            self.min_samples = other.min_samples;
        }
        self.sampled_texels += other.sampled_texels;
        self.total_samples += other.total_samples;
        self.max_samples = self.max_samples.max(other.max_samples);
        self.max_relative_variance = self.max_relative_variance.max(other.max_relative_variance);
    }
}

pub(crate) struct LightmapBakeResult {
    pub(crate) pages: Vec<LightmapPage>,
    pub(crate) sampling: LightmapSamplingSummary,
}

fn primitive_tile_fingerprint(
    scene_fingerprint: &str,
    primitive_index: usize,
    primitive: &super::rust_scene::ComposedPrimitive,
    lights: &[JobLight],
) -> String {
    let mut light_signatures = lights
        .iter()
        .filter(|light| light_affects_primitive(light, primitive))
        .map(|light| {
            let mut signature = Sha256::new();
            signature.update(b"lightmap-light-v1");
            signature.update(light.kind.as_bytes());
            for value in light
                .translation
                .iter()
                .chain(light.rotation_xyzw.iter())
                .chain(light.color_rgba.iter())
                .chain(
                    [
                        light.radius,
                        light.intensity_lumens,
                        light.spot_fov_radians,
                        light.spot_falloff_exponent,
                    ]
                    .iter(),
                )
            {
                signature.update(value.to_le_bytes());
            }
            signature.update(light.flags.to_le_bytes());
            let digest: [u8; 32] = signature.finalize().into();
            digest
        })
        .collect::<Vec<_>>();
    light_signatures.sort_unstable();

    let mut fingerprint = Sha256::new();
    fingerprint.update(b"lightmap-primitive-transport-v1");
    fingerprint.update(scene_fingerprint.as_bytes());
    fingerprint.update(primitive_index.to_le_bytes());
    fingerprint.update(primitive.primitive_key.as_bytes());
    for signature in light_signatures {
        fingerprint.update(signature);
    }
    format!("{:x}", fingerprint.finalize())
}

fn light_affects_primitive(
    light: &JobLight,
    primitive: &super::rust_scene::ComposedPrimitive,
) -> bool {
    if light.kind != "point" && light.kind != "spot" {
        return true;
    }
    let radius = light.radius;
    if !radius.is_finite() || radius <= 0.0 {
        return true;
    }
    let Some(first) = primitive.positions.first() else {
        return true;
    };
    let mut minimum = *first;
    let mut maximum = *first;
    for position in primitive.positions.iter().skip(1) {
        minimum.x = minimum.x.min(position.x);
        minimum.y = minimum.y.min(position.y);
        minimum.z = minimum.z.min(position.z);
        maximum.x = maximum.x.max(position.x);
        maximum.y = maximum.y.max(position.y);
        maximum.z = maximum.z.max(position.z);
    }
    let position = Vec3::from_array(light.translation);
    if !position.is_finite() {
        return true;
    }
    let closest = position.clamp(minimum, maximum);
    position.distance_squared(closest) <= radius * radius + 1.0e-4
}

#[cfg(feature = "lightmap-gpu-solari")]
fn solari_sample_count(settings: LightmapSamplingSettings) -> Result<u32> {
    if settings.min_samples == 0 || settings.max_samples == 0 {
        bail!("Solari bake requires at least one lightmap sample");
    }
    if settings.min_samples != settings.max_samples {
        bail!(
            "Solari bake currently requires fixed sampling; set --lightmap-min-samples and --lightmap-max-samples to the same value"
        );
    }
    if !settings.variance_threshold.is_finite() || settings.variance_threshold > 0.0 {
        bail!(
            "Solari bake does not support adaptive variance stopping yet; set --lightmap-variance-threshold 0"
        );
    }
    Ok(settings.max_samples)
}

/// Rasterizes every composed primitive's generated UV1 and evaluates the
/// shared direct-light transport at covered texel centers. The output stores
/// incident irradiance divided by PI, without receiver albedo.
#[allow(clippy::too_many_arguments)]
pub(crate) fn bake_direct_pages(
    scene: &RustBakeScene,
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_rgba: [f32; 4],
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    output_dir: &Path,
    sampling: LightmapSamplingSettings,
    denoise_settings: LightmapDenoiseSettings,
    bounce_count: u32,
    tile_size: u32,
    cache_fingerprint: &str,
    debug: LightmapDebugSettings,
    cache: &mut TileCache,
    progress: Option<&ProgressReporter>,
) -> Result<LightmapBakeResult> {
    let mut triangles = collect_triangles(scene)?;
    if triangles.is_empty() {
        bail!("surface lightmap bake scene contains no triangles");
    }
    let bvh = Bvh::build_par(&mut triangles);
    let emissive_sampler = EmissiveSampler::new(&triangles, &scene.materials);
    let ambient_irradiance = Vec3::from_array(bevyout_core::lighting::ambient_irradiance(
        ambient_rgba,
        bevyout_core::lighting::DEFAULT_LIGHTING_SCALE,
        bevyout_core::lighting::DEFAULT_AMBIENT_SCALE,
    ));
    fs::create_dir_all(output_dir)?;

    let mut pages = Vec::with_capacity(scene.primitives.len());
    let mut total_covered = 0;
    let mut total_dilated = 0;
    let mut sampling_summary = LightmapSamplingSummary::default();
    for (primitive_index, primitive) in scene.primitives.iter().enumerate() {
        let [width, height] = primitive.lightmap_dimensions;
        if width == 0 || height == 0 {
            bail!(
                "lightmap primitive {} has no generated dimensions",
                primitive.name
            );
        }
        if primitive.positions.len() != primitive.uv1.len()
            || primitive.positions.len() != primitive.uvs.len()
            || primitive.positions.len() != primitive.normals.len()
            || primitive.positions.len() != primitive.colors.len()
            || primitive.positions.len() != primitive.uv1_chart_ids.len()
        {
            bail!(
                "lightmap primitive {} has mismatched raster attributes",
                primitive.name
            );
        }
        let tile_fingerprint =
            primitive_tile_fingerprint(cache_fingerprint, primitive_index, primitive, lights);
        let (pixels, covered_texels, dilated_texels, primitive_sampling) = rasterize_primitive(
            primitive,
            primitive_index,
            width,
            height,
            &bvh,
            &triangles,
            &scene.materials,
            lights,
            directional,
            &emissive_sampler,
            ambient_irradiance,
            environment_map,
            scene_seed,
            sampling,
            denoise_settings,
            bounce_count,
            tile_size,
            &tile_fingerprint,
            output_dir,
            debug,
            cache,
            progress,
        )?;
        let bytes = encode_rgba16f(&pixels);
        let raw_path = output_dir.join(format!("lightmap-{primitive_index:04}.rgba16f.raw"));
        fs::write(&raw_path, &bytes)
            .with_context(|| format!("could not write {}", raw_path.display()))?;
        total_covered += covered_texels;
        total_dilated += dilated_texels;
        sampling_summary.merge(primitive_sampling);
        pages.push(LightmapPage {
            primitive_index,
            width,
            height,
            raw_path,
            covered_texels,
            dilated_texels,
            atlas_index: usize::MAX,
            atlas_offset: [0, 0],
        });
    }
    println!(
        "surface lightmaps: rasterized {} pages, {} covered texels, {} dilated texels",
        pages.len(),
        total_covered,
        total_dilated
    );
    Ok(LightmapBakeResult {
        pages,
        sampling: sampling_summary,
    })
}

/// Runs the feature-gated Solari direct-light prototype through the same page
/// publication path as the CPU backend. The prototype is intentionally narrow:
/// opaque, alpha-mask, and blended receivers, point/spot lights, ambient/directional
/// input, bounded authored-environment and emissive-mesh transport, and at
/// most four secondary diffuse bounces. Unsupported inputs fail explicitly
/// so an explicit GPU request cannot silently change the bake meaning.
#[cfg(feature = "lightmap-gpu-solari")]
#[allow(clippy::too_many_arguments)]
#[allow(
    dead_code,
    reason = "kept as a diagnostic reference for the bounded adapter"
)]
pub(crate) fn bake_direct_pages_solari(
    scene: &RustBakeScene,
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_rgba: [f32; 4],
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    output_dir: &Path,
    sampling: LightmapSamplingSettings,
    denoise_settings: LightmapDenoiseSettings,
    bounce_count: u32,
    _tile_size: u32,
    _cache_fingerprint: &str,
    debug: LightmapDebugSettings,
    _cache: &mut TileCache,
) -> Result<LightmapBakeResult> {
    if bounce_count > SOLARI_BAKE_MAX_BOUNCES {
        bail!(
            "Solari bake prototype supports at most {SOLARI_BAKE_MAX_BOUNCES} diffuse bounces; use --lightmap-bounces 0..={SOLARI_BAKE_MAX_BOUNCES}"
        );
    }
    let sample_count = solari_sample_count(sampling)?;
    let ambient = bevyout_core::lighting::ambient_irradiance(
        ambient_rgba,
        bevyout_core::lighting::DEFAULT_LIGHTING_SCALE,
        bevyout_core::lighting::DEFAULT_AMBIENT_SCALE,
    );
    for material in &scene.materials {
        if material.translucency_strength > f32::EPSILON {
            bail!(
                "Solari bake prototype does not support translucent transport yet; use --bake-backend cpu"
            );
        }
    }

    fs::create_dir_all(output_dir)?;
    let mut texels = Vec::new();
    let mut pages = Vec::with_capacity(scene.primitives.len());
    for (primitive_index, primitive) in scene.primitives.iter().enumerate() {
        let [width, height] = primitive.lightmap_dimensions;
        if width == 0 || height == 0 {
            bail!(
                "lightmap primitive {} has no generated dimensions",
                primitive.name
            );
        }
        let page = collect_solari_page(primitive, primitive_index, width, height, &mut texels)?;
        pages.push(page);
    }
    if texels.is_empty() {
        bail!("Solari bake scene contains no covered UV texels");
    }
    let directional_direction =
        (bevy::math::Quat::from_array(directional.rotation_xyzw) * Vec3::Z).normalize_or_zero();
    let directional = SolariBakeDirectionalLight {
        direction: directional_direction.to_array(),
        color: bevyout_core::lighting::srgb_to_linear_rgb([
            directional.color_rgba[0],
            directional.color_rgba[1],
            directional.color_rgba[2],
        ]),
        illuminance: directional.illuminance,
    };
    let results = super::backend::solari::bake_direct_texels_with_environment(
        &scene.primitives,
        &scene.materials,
        texels.clone(),
        lights,
        ambient,
        directional,
        environment_map,
        sample_count,
        scene_seed,
    )?;
    if results.len() != texels.len() {
        bail!(
            "Solari bake returned {} texels, expected {}",
            results.len(),
            texels.len()
        );
    }

    let mut output_pages = Vec::with_capacity(pages.len());
    let mut summary = LightmapSamplingSummary::default();
    for mut page in pages {
        let pixel_count = (page.width as usize) * (page.height as usize);
        let mut pixels = vec![Vec3::ZERO; pixel_count];
        let mut features = vec![DenoiseFeature::default(); pixel_count];
        for (pixel_index, sample_indices) in page.pixel_samples.iter().enumerate() {
            if sample_indices.is_empty() {
                continue;
            }
            let mut irradiance = Vec3::ZERO;
            let mut position = Vec3::ZERO;
            let mut normal = Vec3::ZERO;
            for sample_index in sample_indices {
                let value = results[*sample_index];
                irradiance += Vec3::from_array([value[0], value[1], value[2]]);
                position += Vec3::from_array(texels[*sample_index].position);
                normal += Vec3::from_array(texels[*sample_index].normal);
            }
            let surfel_count = sample_indices.len() as f32;
            pixels[pixel_index] = (irradiance / surfel_count / std::f32::consts::PI)
                .max(Vec3::ZERO)
                .min(Vec3::splat(65_504.0));
            features[pixel_index] = DenoiseFeature {
                position: (position / surfel_count).to_array(),
                normal: (normal / surfel_count).normalize_or_zero().to_array(),
                material_id: page.material_id,
                relative_variance: 0.0,
                coverage: 1.0,
                sample_count,
            };
            summary.sampled_texels += 1;
            summary.total_samples += sample_indices.len() as u64 * u64::from(sample_count);
            summary.min_samples = if summary.min_samples == 0 {
                sample_count
            } else {
                summary.min_samples.min(sample_count)
            };
            summary.max_samples = summary.max_samples.max(sample_count);
        }
        write_debug_images(
            output_dir,
            page.primitive_index,
            page.width,
            page.height,
            &page.chart_owners,
            &features,
            debug,
        )?;
        if denoise_settings.iterations > 0 {
            let mut denoised = pixels
                .iter()
                .map(|pixel| pixel.to_array())
                .collect::<Vec<_>>();
            denoise(
                &mut denoised,
                &page.chart_owners,
                &features,
                page.width as usize,
                page.height as usize,
                denoise_settings.iterations,
            )
            .map_err(|error| anyhow::anyhow!("lightmap denoising failed: {error:?}"))?;
            for (pixel, denoised) in pixels.iter_mut().zip(denoised) {
                *pixel = Vec3::from_array(denoised);
            }
        }
        write_variance_output(
            output_dir,
            page.primitive_index,
            &page.chart_owners,
            &features,
        )?;
        let dilated_texels = dilate_chart_aware(
            &mut pixels,
            &mut page.chart_owners,
            super::lightmap_uv::LIGHTMAP_PADDING_TEXELS as usize,
            page.width as usize,
            page.height as usize,
        );
        let bytes = encode_rgba16f(&pixels);
        let raw_path = output_dir.join(format!("lightmap-{:04}.rgba16f.raw", page.primitive_index));
        fs::write(&raw_path, &bytes)
            .with_context(|| format!("could not write {}", raw_path.display()))?;
        output_pages.push(LightmapPage {
            primitive_index: page.primitive_index,
            width: page.width,
            height: page.height,
            raw_path,
            covered_texels: page
                .chart_owners
                .iter()
                .filter(|owner| owner.is_some())
                .count(),
            dilated_texels,
            atlas_index: usize::MAX,
            atlas_offset: [0, 0],
        });
    }
    Ok(LightmapBakeResult {
        pages: output_pages,
        sampling: summary,
    })
}

#[cfg(feature = "lightmap-gpu-solari")]
#[allow(clippy::too_many_arguments)]
pub(crate) fn bake_direct_pages_solari_bounded(
    scene: &RustBakeScene,
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    ambient_rgba: [f32; 4],
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    output_dir: &Path,
    sampling: LightmapSamplingSettings,
    denoise_settings: LightmapDenoiseSettings,
    bounce_count: u32,
    tile_size: u32,
    cache_fingerprint: &str,
    debug: LightmapDebugSettings,
    cache: &mut TileCache,
    progress: Option<&ProgressReporter>,
) -> Result<LightmapBakeResult> {
    if bounce_count > SOLARI_BAKE_MAX_BOUNCES {
        bail!(
            "Solari bake prototype supports at most {SOLARI_BAKE_MAX_BOUNCES} diffuse bounces; use --lightmap-bounces 0..={SOLARI_BAKE_MAX_BOUNCES}"
        );
    }
    let sample_count = solari_sample_count(sampling)?;
    let ambient = bevyout_core::lighting::ambient_irradiance(
        ambient_rgba,
        bevyout_core::lighting::DEFAULT_LIGHTING_SCALE,
        bevyout_core::lighting::DEFAULT_AMBIENT_SCALE,
    );
    for material in &scene.materials {
        if material.translucency_strength > f32::EPSILON {
            bail!(
                "Solari bake prototype does not support translucent transport yet; use --bake-backend cpu"
            );
        }
    }

    fs::create_dir_all(output_dir)?;
    let tile_size = usize::try_from(tile_size.max(1)).context("lightmap tile size overflowed")?;
    let directional = SolariBakeDirectionalLight {
        direction: (bevy::math::Quat::from_array(directional.rotation_xyzw) * Vec3::Z)
            .normalize_or_zero()
            .to_array(),
        color: bevyout_core::lighting::srgb_to_linear_rgb([
            directional.color_rgba[0],
            directional.color_rgba[1],
            directional.color_rgba[2],
        ]),
        illuminance: directional.illuminance,
    };
    let mut session =
        SolariBakeSession::new(&scene.primitives, &scene.materials, scene_seed as u32)?;
    let mut revision = scene_seed;
    let mut output_pages = Vec::with_capacity(scene.primitives.len());
    let mut summary = LightmapSamplingSummary::default();

    for (primitive_index, primitive) in scene.primitives.iter().enumerate() {
        let [width, height] = primitive.lightmap_dimensions;
        if width == 0 || height == 0 {
            bail!(
                "lightmap primitive {} has no generated dimensions",
                primitive.name
            );
        }
        scene
            .materials
            .get(primitive.material)
            .context("lightmap primitive material index is invalid")?;
        let page_len = (width as usize)
            .checked_mul(height as usize)
            .context("Solari lightmap page dimensions overflowed")?;
        let mut pixels = vec![Vec3::ZERO; page_len];
        let mut chart_owners = vec![None; page_len];
        let mut features = vec![DenoiseFeature::default(); page_len];
        let tile_count_x = (width as usize).div_ceil(tile_size);
        let tile_count_y = (height as usize).div_ceil(tile_size);
        let tile_total = (tile_count_x * tile_count_y) as u64;
        if let Some(progress) = progress {
            progress.phase_started(
                format!("primitive {} Solari tiles", primitive_index + 1),
                Some(tile_total),
            );
        }

        for tile_y in 0..tile_count_y {
            let tile_start_y = tile_y * tile_size;
            let tile_height = tile_size.min(height as usize - tile_start_y);
            for tile_x in 0..tile_count_x {
                let tile_start_x = tile_x * tile_size;
                let tile_width = tile_size.min(width as usize - tile_start_x);
                let key = TileKey {
                    primitive: primitive_index,
                    tile_x: tile_x as u32,
                    tile_y: tile_y as u32,
                };
                if let Some(record) = cache.read(key, cache_fingerprint)? {
                    let decoded =
                        decode_tile_payload(&record, tile_width as u32, tile_height as u32)?;
                    copy_solari_tile_to_page(
                        &decoded,
                        tile_start_x,
                        tile_start_y,
                        width as usize,
                        &mut pixels,
                        &mut chart_owners,
                        &mut features,
                        primitive,
                    )?;
                    summary.merge(decoded.summary);
                    if let Some(progress) = progress {
                        progress.unit_completed(Some(tile_total), Some(true));
                    }
                    continue;
                }

                let tile = collect_solari_tile(
                    primitive,
                    primitive_index,
                    width,
                    height,
                    tile_start_x,
                    tile_start_y,
                    tile_width,
                    tile_height,
                )?;
                let results = session.bake_texels_with_environment(
                    tile.texels.clone(),
                    lights,
                    ambient,
                    directional,
                    sample_count,
                    bounce_count,
                    environment_map,
                    revision,
                    scene_seed as u32,
                )?;
                revision = revision.wrapping_add(1);
                if results.len() != tile.texels.len() {
                    bail!(
                        "Solari returned {} texels for primitive {} tile {}, expected {}",
                        results.len(),
                        primitive_index,
                        key.tile_x,
                        tile.texels.len()
                    );
                }

                let local_len = tile_width * tile_height;
                let mut tile_pixels = vec![Vec3::ZERO; local_len];
                let tile_owners = tile.chart_owners;
                let mut tile_features = vec![DenoiseFeature::default(); local_len];
                let mut tile_summary = LightmapSamplingSummary::default();
                for (local_index, sample_indices) in tile.pixel_samples.iter().enumerate() {
                    if sample_indices.is_empty() {
                        continue;
                    }
                    let mut irradiance = Vec3::ZERO;
                    let mut position = Vec3::ZERO;
                    let mut normal = Vec3::ZERO;
                    for sample_index in sample_indices {
                        let value = results[*sample_index];
                        irradiance += Vec3::from_array([value[0], value[1], value[2]]);
                        position += Vec3::from_array(tile.texels[*sample_index].position);
                        normal += Vec3::from_array(tile.texels[*sample_index].normal);
                    }
                    let surfel_count = sample_indices.len() as f32;
                    tile_pixels[local_index] = (irradiance / surfel_count / std::f32::consts::PI)
                        .max(Vec3::ZERO)
                        .min(Vec3::splat(65_504.0));
                    tile_features[local_index] = DenoiseFeature {
                        position: (position / surfel_count).to_array(),
                        normal: (normal / surfel_count).normalize_or_zero().to_array(),
                        material_id: primitive.material as u32,
                        relative_variance: 0.0,
                        coverage: 1.0,
                        sample_count,
                    };
                    tile_summary.sampled_texels += 1;
                    tile_summary.total_samples +=
                        sample_indices.len() as u64 * u64::from(sample_count);
                }
                if tile_summary.sampled_texels != 0 {
                    tile_summary.min_samples = sample_count;
                    tile_summary.max_samples = sample_count;
                }
                let decoded = DecodedTilePayload {
                    width: tile_width,
                    height: tile_height,
                    pixels: tile_pixels,
                    owners: tile_owners,
                    features: tile_features,
                    summary: tile_summary,
                };
                copy_solari_tile_to_page(
                    &decoded,
                    tile_start_x,
                    tile_start_y,
                    width as usize,
                    &mut pixels,
                    &mut chart_owners,
                    &mut features,
                    primitive,
                )?;
                let payload = encode_tile_payload(
                    tile_width as u32,
                    tile_height as u32,
                    &decoded.pixels,
                    &decoded.owners,
                    &decoded.features,
                    tile_summary,
                )?;
                cache.write(
                    key,
                    cache_fingerprint,
                    tile_width as u32,
                    tile_height as u32,
                    &payload,
                )?;
                if let Some(progress) = progress {
                    progress.unit_completed(Some(tile_total), Some(false));
                }
                summary.merge(tile_summary);
            }
        }

        if let Some(progress) = progress {
            progress.phase_started(
                format!(
                    "primitive {} Solari denoise and dilation",
                    primitive_index + 1
                ),
                None,
            );
        }
        let covered_texels = chart_owners.iter().filter(|owner| owner.is_some()).count();
        write_debug_images(
            output_dir,
            primitive_index,
            width,
            height,
            &chart_owners,
            &features,
            debug,
        )?;
        if denoise_settings.iterations > 0 {
            let mut denoised = pixels
                .iter()
                .map(|pixel| pixel.to_array())
                .collect::<Vec<_>>();
            denoise(
                &mut denoised,
                &chart_owners,
                &features,
                width as usize,
                height as usize,
                denoise_settings.iterations,
            )
            .map_err(|error| anyhow::anyhow!("lightmap denoising failed: {error:?}"))?;
            for (pixel, denoised) in pixels.iter_mut().zip(denoised) {
                *pixel = Vec3::from_array(denoised);
            }
        }
        write_variance_output(output_dir, primitive_index, &chart_owners, &features)?;
        let dilated_texels = dilate_chart_aware(
            &mut pixels,
            &mut chart_owners,
            super::lightmap_uv::LIGHTMAP_PADDING_TEXELS as usize,
            width as usize,
            height as usize,
        );
        let raw_path = output_dir.join(format!("lightmap-{primitive_index:04}.rgba16f.raw"));
        fs::write(&raw_path, encode_rgba16f(&pixels))
            .with_context(|| format!("could not write {}", raw_path.display()))?;
        output_pages.push(LightmapPage {
            primitive_index,
            width,
            height,
            raw_path,
            covered_texels,
            dilated_texels,
            atlas_index: usize::MAX,
            atlas_offset: [0, 0],
        });
    }
    Ok(LightmapBakeResult {
        pages: output_pages,
        sampling: summary,
    })
}

#[cfg(feature = "lightmap-gpu-solari")]
#[allow(clippy::too_many_arguments)]
fn copy_solari_tile_to_page(
    tile: &DecodedTilePayload,
    tile_start_x: usize,
    tile_start_y: usize,
    page_width: usize,
    page_pixels: &mut [Vec3],
    page_owners: &mut [Option<u32>],
    page_features: &mut [DenoiseFeature],
    primitive: &super::rust_scene::ComposedPrimitive,
) -> Result<()> {
    for local_y in 0..tile.height {
        for local_x in 0..tile.width {
            let local_index = local_y * tile.width + local_x;
            let Some(owner) = tile.owners[local_index] else {
                continue;
            };
            let page_index = (tile_start_y + local_y) * page_width + tile_start_x + local_x;
            if let Some(existing) = page_owners[page_index]
                && existing != owner
            {
                bail!(
                    "lightmap charts {} and {} overlap in {}",
                    existing,
                    owner,
                    primitive.name
                );
            }
            page_pixels[page_index] = tile.pixels[local_index];
            page_owners[page_index] = Some(owner);
            page_features[page_index] = tile.features[local_index];
        }
    }
    Ok(())
}

#[cfg(feature = "lightmap-gpu-solari")]
struct SolariPageWork {
    primitive_index: usize,
    width: u32,
    height: u32,
    material_id: u32,
    pixel_samples: Vec<Vec<usize>>,
    chart_owners: Vec<Option<u32>>,
}

#[cfg(feature = "lightmap-gpu-solari")]
struct SolariTileWork {
    texels: Vec<SolariBakeTexel>,
    pixel_samples: Vec<Vec<usize>>,
    chart_owners: Vec<Option<u32>>,
}

#[cfg(feature = "lightmap-gpu-solari")]
#[allow(clippy::too_many_arguments)]
fn collect_solari_tile(
    primitive: &super::rust_scene::ComposedPrimitive,
    primitive_index: usize,
    page_width: u32,
    page_height: u32,
    tile_start_x: usize,
    tile_start_y: usize,
    tile_width: usize,
    tile_height: usize,
) -> Result<SolariTileWork> {
    let mut texels = Vec::new();
    let mut pixel_samples = vec![Vec::new(); tile_width * tile_height];
    let mut chart_owners = vec![None; tile_width * tile_height];
    for indices in primitive.indices.chunks_exact(3) {
        let [a, b, c] = [
            indices[0] as usize,
            indices[1] as usize,
            indices[2] as usize,
        ];
        let position = [
            *primitive
                .positions
                .get(a)
                .context("Solari lightmap position index is invalid")?,
            *primitive
                .positions
                .get(b)
                .context("Solari lightmap position index is invalid")?,
            *primitive
                .positions
                .get(c)
                .context("Solari lightmap position index is invalid")?,
        ];
        let normal = [
            *primitive
                .normals
                .get(a)
                .context("Solari lightmap normal index is invalid")?,
            *primitive
                .normals
                .get(b)
                .context("Solari lightmap normal index is invalid")?,
            *primitive
                .normals
                .get(c)
                .context("Solari lightmap normal index is invalid")?,
        ];
        let uv1 = [
            *primitive
                .uv1
                .get(a)
                .context("Solari lightmap UV1 index is invalid")?,
            *primitive
                .uv1
                .get(b)
                .context("Solari lightmap UV1 index is invalid")?,
            *primitive
                .uv1
                .get(c)
                .context("Solari lightmap UV1 index is invalid")?,
        ];
        let chart_ids = [
            *primitive
                .uv1_chart_ids
                .get(a)
                .context("Solari lightmap chart index is invalid")?,
            *primitive
                .uv1_chart_ids
                .get(b)
                .context("Solari lightmap chart index is invalid")?,
            *primitive
                .uv1_chart_ids
                .get(c)
                .context("Solari lightmap chart index is invalid")?,
        ];
        if chart_ids[0] != chart_ids[1] || chart_ids[0] != chart_ids[2] {
            bail!(
                "lightmap triangle {} in {} crosses xatlas charts",
                indices[0] / 3,
                primitive.name
            );
        }
        let chart_id = chart_ids[0];
        let min = uv1[0].min(uv1[1]).min(uv1[2]) * Vec2::new(page_width as f32, page_height as f32);
        let max = uv1[0].max(uv1[1]).max(uv1[2]) * Vec2::new(page_width as f32, page_height as f32);
        let min_x = (min.x.floor() as i32).max(tile_start_x as i32);
        let max_x = (max.x.ceil() as i32).min((tile_start_x + tile_width) as i32 - 1);
        let min_y = (min.y.floor() as i32).max(tile_start_y as i32);
        let max_y = (max.y.ceil() as i32).min((tile_start_y + tile_height) as i32 - 1);
        if min_x > max_x || min_y > max_y {
            continue;
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let local_index =
                    (y as usize - tile_start_y) * tile_width + (x as usize - tile_start_x);
                let (sample_weights, sample_count) =
                    pixel_sample_weights(x as u32, y as u32, page_width, page_height, uv1);
                for weights in sample_weights.into_iter().take(sample_count).flatten() {
                    let sampled_normal = interpolate_vec3(normal, weights).normalize_or_zero();
                    if sampled_normal == Vec3::ZERO {
                        continue;
                    }
                    if let Some(owner) = chart_owners[local_index]
                        && owner != chart_id
                    {
                        bail!(
                            "lightmap charts {} and {} overlap at texel ({x}, {y}) in {}",
                            owner,
                            chart_id,
                            primitive.name
                        );
                    }
                    chart_owners[local_index] = Some(chart_id);
                    let sample_index = pixel_samples[local_index].len();
                    let global_index = y as usize * page_width as usize + x as usize;
                    let spatial_index = primitive_index
                        .wrapping_mul(page_width as usize * page_height as usize)
                        .wrapping_add(global_index.wrapping_mul(4))
                        .wrapping_add(sample_index) as u32;
                    texels.push(SolariBakeTexel {
                        position: interpolate_vec3(position, weights).to_array(),
                        normal: sampled_normal.to_array(),
                        spatial_index,
                    });
                    pixel_samples[local_index].push(texels.len() - 1);
                }
            }
        }
    }
    Ok(SolariTileWork {
        texels,
        pixel_samples,
        chart_owners,
    })
}

#[cfg(feature = "lightmap-gpu-solari")]
fn collect_solari_page(
    primitive: &super::rust_scene::ComposedPrimitive,
    primitive_index: usize,
    width: u32,
    height: u32,
    texels: &mut Vec<SolariBakeTexel>,
) -> Result<SolariPageWork> {
    let pixel_count = (width as usize)
        .checked_mul(height as usize)
        .context("Solari lightmap page dimensions overflowed")?;
    let mut pixel_samples = vec![Vec::new(); pixel_count];
    let mut chart_owners = vec![None; pixel_count];
    for indices in primitive.indices.chunks_exact(3) {
        let [a, b, c] = [
            indices[0] as usize,
            indices[1] as usize,
            indices[2] as usize,
        ];
        let position = [
            *primitive
                .positions
                .get(a)
                .context("Solari lightmap position index is invalid")?,
            *primitive
                .positions
                .get(b)
                .context("Solari lightmap position index is invalid")?,
            *primitive
                .positions
                .get(c)
                .context("Solari lightmap position index is invalid")?,
        ];
        let normal = [
            *primitive
                .normals
                .get(a)
                .context("Solari lightmap normal index is invalid")?,
            *primitive
                .normals
                .get(b)
                .context("Solari lightmap normal index is invalid")?,
            *primitive
                .normals
                .get(c)
                .context("Solari lightmap normal index is invalid")?,
        ];
        let uv1 = [
            *primitive
                .uv1
                .get(a)
                .context("Solari lightmap UV1 index is invalid")?,
            *primitive
                .uv1
                .get(b)
                .context("Solari lightmap UV1 index is invalid")?,
            *primitive
                .uv1
                .get(c)
                .context("Solari lightmap UV1 index is invalid")?,
        ];
        let chart_ids = [
            *primitive
                .uv1_chart_ids
                .get(a)
                .context("Solari lightmap chart index is invalid")?,
            *primitive
                .uv1_chart_ids
                .get(b)
                .context("Solari lightmap chart index is invalid")?,
            *primitive
                .uv1_chart_ids
                .get(c)
                .context("Solari lightmap chart index is invalid")?,
        ];
        if chart_ids[0] != chart_ids[1] || chart_ids[0] != chart_ids[2] {
            bail!(
                "lightmap triangle {} in {} crosses xatlas charts",
                indices[0] / 3,
                primitive.name
            );
        }
        let chart_id = chart_ids[0];
        let min = uv1[0].min(uv1[1]).min(uv1[2]) * Vec2::new(width as f32, height as f32);
        let max = uv1[0].max(uv1[1]).max(uv1[2]) * Vec2::new(width as f32, height as f32);
        let min_x = (min.x.floor() as i32).max(0);
        let max_x = (max.x.ceil() as i32).min(width as i32 - 1);
        let min_y = (min.y.floor() as i32).max(0);
        let max_y = (max.y.ceil() as i32).min(height as i32 - 1);
        if min_x > max_x || min_y > max_y {
            continue;
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let index = y as usize * width as usize + x as usize;
                let (sample_weights, sample_count) =
                    pixel_sample_weights(x as u32, y as u32, width, height, uv1);
                for weights in sample_weights.into_iter().take(sample_count).flatten() {
                    let sampled_normal = interpolate_vec3(normal, weights).normalize_or_zero();
                    if sampled_normal == Vec3::ZERO {
                        continue;
                    }
                    if let Some(owner) = chart_owners[index]
                        && owner != chart_id
                    {
                        bail!(
                            "lightmap charts {} and {} overlap at texel ({x}, {y}) in {}",
                            owner,
                            chart_id,
                            primitive.name
                        );
                    }
                    chart_owners[index] = Some(chart_id);
                    let sample_index = pixel_samples[index].len();
                    let spatial_index = primitive_index
                        .wrapping_mul(pixel_count)
                        .wrapping_add(index.wrapping_mul(4))
                        .wrapping_add(sample_index) as u32;
                    texels.push(SolariBakeTexel {
                        position: interpolate_vec3(position, weights).to_array(),
                        normal: sampled_normal.to_array(),
                        spatial_index,
                    });
                    pixel_samples[index].push(texels.len() - 1);
                }
            }
        }
    }
    Ok(SolariPageWork {
        primitive_index,
        width,
        height,
        material_id: primitive.material as u32,
        pixel_samples,
        chart_owners,
    })
}

struct AtlasLayout {
    placements: Vec<(usize, u32, u32)>,
    cursor_x: u32,
    cursor_y: u32,
    row_height: u32,
    width: u32,
    height: u32,
}

/// Reject pages that cannot be represented by the one-primitive/one-binding
/// atlas contract before the backend starts tracing them. A later geometry
/// split could remove this restriction, but slicing a finished page would
/// invalidate UV1, cache identities, variance artifacts, and runtime bindings.
pub(crate) fn validate_page_dimensions(
    scene: &super::rust_scene::RustBakeScene,
    max_size: u32,
) -> Result<()> {
    if max_size <= LIGHTMAP_ATLAS_GUTTER_TEXELS * 2 {
        bail!("lightmap atlas page size must exceed its gutter");
    }
    let usable_size = max_size - LIGHTMAP_ATLAS_GUTTER_TEXELS * 2;
    for primitive in &scene.primitives {
        let [width, height] = primitive.lightmap_dimensions;
        if width <= usable_size && height <= usable_size {
            continue;
        }
        let required = width.max(height).max(1) as f32;
        let density_limit = primitive.lightmap_texels_per_meter * usable_size as f32 / required;
        bail!(
            "lightmap primitive {} requires {}x{} atlas texels, exceeding {}; reduce --lightmap-texels-per-meter to <= {:.2} (current {:.2})",
            primitive.name,
            width.saturating_add(LIGHTMAP_ATLAS_GUTTER_TEXELS * 2),
            height.saturating_add(LIGHTMAP_ATLAS_GUTTER_TEXELS * 2),
            max_size,
            density_limit,
            primitive.lightmap_texels_per_meter,
        );
    }
    Ok(())
}

/// Packs the deterministic per-primitive pages into atlas images while
/// preserving each primitive's local UV1. The two-texel gutter copies edge
/// texels so bilinear sampling cannot immediately read a neighboring tile.
pub(crate) fn pack_lightmap_pages(
    mut pages: Vec<LightmapPage>,
    output_dir: &Path,
    max_size: u32,
) -> Result<(Vec<LightmapPage>, Vec<LightmapAtlas>)> {
    if max_size == 0 {
        bail!("lightmap atlas page size must be nonzero");
    }
    let gutter = LIGHTMAP_ATLAS_GUTTER_TEXELS;
    let mut order = (0..pages.len()).collect::<Vec<_>>();
    order.sort_by_key(|index| {
        (
            std::cmp::Reverse(pages[*index].height),
            std::cmp::Reverse(pages[*index].width),
            pages[*index].primitive_index,
        )
    });
    let mut layouts = Vec::<AtlasLayout>::new();
    for page_index in order {
        let page = &pages[page_index];
        let tile_width = page
            .width
            .checked_add(gutter * 2)
            .context("lightmap atlas tile width overflowed")?;
        let tile_height = page
            .height
            .checked_add(gutter * 2)
            .context("lightmap atlas tile height overflowed")?;
        if tile_width > max_size || tile_height > max_size {
            bail!(
                "lightmap primitive {} requires {}x{} atlas texels, exceeding {}",
                page.primitive_index,
                tile_width,
                tile_height,
                max_size
            );
        }
        let mut placed = false;
        for layout in &mut layouts {
            let (x, y) = if layout.cursor_x + tile_width > max_size {
                (0, layout.cursor_y + layout.row_height)
            } else {
                (layout.cursor_x, layout.cursor_y)
            };
            if y + tile_height > max_size {
                continue;
            }
            layout.placements.push((page_index, x, y));
            layout.cursor_x = x + tile_width;
            layout.cursor_y = y;
            layout.row_height = if x == 0 {
                tile_height
            } else {
                layout.row_height.max(tile_height)
            };
            layout.width = layout.width.max(x + tile_width);
            layout.height = layout.height.max(y + tile_height);
            placed = true;
            break;
        }
        if !placed {
            layouts.push(AtlasLayout {
                placements: vec![(page_index, 0, 0)],
                cursor_x: tile_width,
                cursor_y: 0,
                row_height: tile_height,
                width: tile_width,
                height: tile_height,
            });
        }
    }

    let mut atlases = Vec::with_capacity(layouts.len());
    for (atlas_index, layout) in layouts.into_iter().enumerate() {
        let width = layout.width.max(1);
        let height = layout.height.max(1);
        let byte_len = (width as usize)
            .checked_mul(height as usize)
            .and_then(|pixels| pixels.checked_mul(8))
            .context("lightmap atlas byte size overflowed")?;
        let mut bytes = vec![0_u8; byte_len];
        for (page_index, x, y) in layout.placements {
            let page = &pages[page_index];
            let source = fs::read(&page.raw_path).with_context(|| {
                format!(
                    "could not read {} for atlas packing",
                    page.raw_path.display()
                )
            })?;
            let expected = (page.width as usize)
                .checked_mul(page.height as usize)
                .and_then(|pixels| pixels.checked_mul(8))
                .context("lightmap page byte size overflowed")?;
            if source.len() != expected {
                bail!(
                    "lightmap page {} has {} bytes, expected {}",
                    page.primitive_index,
                    source.len(),
                    expected
                );
            }
            let tile_width = page.width + gutter * 2;
            let tile_height = page.height + gutter * 2;
            for tile_y in 0..tile_height {
                let source_y = tile_y.saturating_sub(gutter).min(page.height - 1);
                for tile_x in 0..tile_width {
                    let source_x = tile_x.saturating_sub(gutter).min(page.width - 1);
                    let source_offset = ((source_y * page.width + source_x) as usize) * 8;
                    let destination_x = x + tile_x;
                    let destination_y = y + tile_y;
                    let destination_offset = ((destination_y * width + destination_x) as usize) * 8;
                    bytes[destination_offset..destination_offset + 8]
                        .copy_from_slice(&source[source_offset..source_offset + 8]);
                }
            }
            pages[page_index].atlas_index = atlas_index;
            pages[page_index].atlas_offset = [x + gutter, y + gutter];
        }
        let content_hash = format!("{:x}", Sha256::digest(&bytes));
        let raw_path = output_dir.join(format!("lightmap-atlas-{atlas_index:04}.rgba16f.raw"));
        fs::write(&raw_path, &bytes)
            .with_context(|| format!("could not write {}", raw_path.display()))?;
        atlases.push(LightmapAtlas {
            width,
            height,
            raw_path,
            content_hash,
        });
    }
    Ok((pages, atlases))
}

#[allow(clippy::too_many_arguments)]
fn rasterize_primitive(
    primitive: &super::rust_scene::ComposedPrimitive,
    primitive_index: usize,
    width: u32,
    height: u32,
    bvh: &Bvh<f32, 3>,
    triangles: &[IrradianceTriangle],
    materials: &[super::rust_scene::TransportMaterial],
    lights: &[JobLight],
    directional: &DirectionalBakeLight,
    emitters: &EmissiveSampler,
    ambient_irradiance: Vec3,
    environment_map: Option<&EnvironmentMap>,
    scene_seed: u64,
    sampling: LightmapSamplingSettings,
    denoise_settings: LightmapDenoiseSettings,
    bounce_count: u32,
    tile_size: u32,
    tile_fingerprint: &str,
    debug_output_dir: &Path,
    debug: LightmapDebugSettings,
    cache: &mut TileCache,
    progress: Option<&ProgressReporter>,
) -> Result<(Vec<Vec3>, usize, usize, LightmapSamplingSummary)> {
    let mut pixels = vec![Vec3::ZERO; (width as usize) * (height as usize)];
    let mut chart_owners = vec![None; pixels.len()];
    let mut features = vec![DenoiseFeature::default(); pixels.len()];
    let material = materials
        .get(primitive.material)
        .context("lightmap primitive material index is invalid")?;
    let mut sampling_summary = LightmapSamplingSummary::default();
    let tile_size = usize::try_from(tile_size.max(1)).context("lightmap tile size overflowed")?;
    let tile_count_x = (width as usize).div_ceil(tile_size);
    let tile_count_y = (height as usize).div_ceil(tile_size);
    let tile_total = (tile_count_x * tile_count_y) as u64;
    if let Some(progress) = progress {
        progress.phase_started(
            format!("primitive {} tiles", primitive_index + 1),
            Some(tile_total),
        );
    }

    for tile_y in 0..tile_count_y {
        let tile_start_y = tile_y * tile_size;
        let tile_end_y = (tile_start_y + tile_size).min(height as usize);
        let tile_height = tile_end_y - tile_start_y;
        for tile_x in 0..tile_count_x {
            let tile_start_x = tile_x * tile_size;
            let tile_end_x = (tile_start_x + tile_size).min(width as usize);
            let tile_width = tile_end_x - tile_start_x;
            let key = TileKey {
                primitive: primitive_index,
                tile_x: tile_x as u32,
                tile_y: tile_y as u32,
            };
            if let Some(record) = cache.read(key, tile_fingerprint)? {
                let decoded = decode_tile_payload(&record, tile_width as u32, tile_height as u32)?;
                for local_y in 0..tile_height {
                    for local_x in 0..tile_width {
                        let local_index = local_y * tile_width + local_x;
                        let destination =
                            (tile_start_y + local_y) * width as usize + tile_start_x + local_x;
                        pixels[destination] = decoded.pixels[local_index];
                        chart_owners[destination] = decoded.owners[local_index];
                        features[destination] = decoded.features[local_index];
                    }
                }
                sampling_summary.merge(decoded.summary);
                if let Some(progress) = progress {
                    progress.unit_completed(Some(tile_total), Some(true));
                }
                continue;
            }
            let mut tile_sampling_summary = LightmapSamplingSummary::default();

            for indices in primitive.indices.chunks_exact(3) {
                let [a, b, c] = [
                    indices[0] as usize,
                    indices[1] as usize,
                    indices[2] as usize,
                ];
                let position = [
                    *primitive
                        .positions
                        .get(a)
                        .context("lightmap position index is invalid")?,
                    *primitive
                        .positions
                        .get(b)
                        .context("lightmap position index is invalid")?,
                    *primitive
                        .positions
                        .get(c)
                        .context("lightmap position index is invalid")?,
                ];
                let normal = [
                    *primitive
                        .normals
                        .get(a)
                        .context("lightmap normal index is invalid")?,
                    *primitive
                        .normals
                        .get(b)
                        .context("lightmap normal index is invalid")?,
                    *primitive
                        .normals
                        .get(c)
                        .context("lightmap normal index is invalid")?,
                ];
                let uv1 = [
                    *primitive
                        .uv1
                        .get(a)
                        .context("lightmap UV1 index is invalid")?,
                    *primitive
                        .uv1
                        .get(b)
                        .context("lightmap UV1 index is invalid")?,
                    *primitive
                        .uv1
                        .get(c)
                        .context("lightmap UV1 index is invalid")?,
                ];
                let uv0 = [
                    *primitive
                        .uvs
                        .get(a)
                        .context("lightmap UV0 index is invalid")?,
                    *primitive
                        .uvs
                        .get(b)
                        .context("lightmap UV0 index is invalid")?,
                    *primitive
                        .uvs
                        .get(c)
                        .context("lightmap UV0 index is invalid")?,
                ];
                let colors = [
                    *primitive
                        .colors
                        .get(a)
                        .context("lightmap color index is invalid")?,
                    *primitive
                        .colors
                        .get(b)
                        .context("lightmap color index is invalid")?,
                    *primitive
                        .colors
                        .get(c)
                        .context("lightmap color index is invalid")?,
                ];
                let chart_ids = [
                    *primitive
                        .uv1_chart_ids
                        .get(a)
                        .context("lightmap chart index is invalid")?,
                    *primitive
                        .uv1_chart_ids
                        .get(b)
                        .context("lightmap chart index is invalid")?,
                    *primitive
                        .uv1_chart_ids
                        .get(c)
                        .context("lightmap chart index is invalid")?,
                ];
                if chart_ids[0] != chart_ids[1] || chart_ids[0] != chart_ids[2] {
                    bail!(
                        "lightmap triangle {} in {} crosses xatlas charts",
                        indices[0] / 3,
                        primitive.name
                    );
                }
                let chart_id = chart_ids[0];
                let min = uv1[0].min(uv1[1]).min(uv1[2]) * Vec2::new(width as f32, height as f32);
                let max = uv1[0].max(uv1[1]).max(uv1[2]) * Vec2::new(width as f32, height as f32);
                let min_x = (min.x.floor() as i32).max(tile_start_x as i32);
                let max_x = (max.x.ceil() as i32).min(tile_end_x as i32 - 1);
                let min_y = (min.y.floor() as i32).max(tile_start_y as i32);
                let max_y = (max.y.ceil() as i32).min(tile_end_y as i32 - 1);
                if min_x > max_x || min_y > max_y {
                    continue;
                }
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        let index = y as usize * width as usize + x as usize;
                        let (sample_weights, sample_count) =
                            pixel_sample_weights(x as u32, y as u32, width, height, uv1);
                        let mut sum = Vec3::ZERO;
                        let mut feature_position_sum = Vec3::ZERO;
                        let mut feature_normal_sum = Vec3::ZERO;
                        let mut feature_relative_variance = 0.0_f32;
                        let mut feature_sample_count = 0_u64;
                        let mut valid_samples = 0_u32;
                        for (sample_index, weights) in
                            sample_weights.into_iter().take(sample_count).enumerate()
                        {
                            let Some(weights) = weights else {
                                continue;
                            };
                            let sampled_uv = interpolate_vec2(uv0, weights);
                            let sampled_color = interpolate_vec4(colors, weights);
                            let alpha = sample_material(material, sampled_uv, sampled_color).alpha;
                            if material.alpha_mode == super::rust_scene::AlphaMode::Mask
                                && alpha < material.alpha_cutoff
                            {
                                continue;
                            }
                            let sampled_position = interpolate_vec3(position, weights);
                            let sampled_normal =
                                interpolate_vec3(normal, weights).normalize_or_zero();
                            if sampled_normal == Vec3::ZERO {
                                continue;
                            }
                            let mut statistics = AdaptiveEstimator::new(
                                sampling.min_samples,
                                sampling.max_samples,
                                sampling.variance_threshold,
                            );
                            let spatial_index = primitive_index
                                .wrapping_mul(pixels.len())
                                .wrapping_add(index.wrapping_mul(4))
                                .wrapping_add(sample_index);
                            for adaptive_sample in 0..sampling.max_samples {
                                let irradiance = surface_irradiance_with_emissive_and_environment(
                                    bvh,
                                    triangles,
                                    materials,
                                    lights,
                                    directional,
                                    emitters,
                                    ambient_irradiance,
                                    environment_map,
                                    scene_seed,
                                    spatial_index
                                        .wrapping_mul(0x9e37_79b9)
                                        .wrapping_add(adaptive_sample as usize),
                                    1,
                                    bounce_count,
                                    sampled_position,
                                    sampled_normal,
                                );
                                let sample = (irradiance / std::f32::consts::PI)
                                    .max(Vec3::ZERO)
                                    .min(Vec3::splat(65_504.0));
                                statistics.add(sample.to_array());
                                if statistics.should_stop() {
                                    break;
                                }
                            }
                            sum += Vec3::from_array(statistics.mean());
                            feature_position_sum += sampled_position;
                            feature_normal_sum += sampled_normal;
                            feature_sample_count += u64::from(statistics.sample_count());
                            let relative_variance = statistics.relative_variance();
                            feature_relative_variance =
                                feature_relative_variance.max(if relative_variance.is_finite() {
                                    relative_variance
                                } else {
                                    1.0e6
                                });
                            tile_sampling_summary.record(&statistics);
                            valid_samples += 1;
                        }
                        if valid_samples == 0 {
                            continue;
                        }
                        let value = sum / valid_samples as f32;
                        let feature = DenoiseFeature {
                            position: (feature_position_sum / valid_samples as f32).to_array(),
                            normal: (feature_normal_sum / valid_samples as f32)
                                .normalize_or_zero()
                                .to_array(),
                            material_id: primitive.material as u32,
                            relative_variance: feature_relative_variance,
                            coverage: valid_samples as f32 / sample_count as f32,
                            sample_count: feature_sample_count
                                .div_ceil(u64::from(valid_samples))
                                .try_into()
                                .unwrap_or(u32::MAX),
                        };
                        match chart_owners[index] {
                            None => {
                                pixels[index] = value;
                                features[index] = feature;
                                chart_owners[index] = Some(chart_id);
                            }
                            Some(owner) if owner == chart_id => {
                                pixels[index] = value;
                                features[index] = feature;
                            }
                            Some(owner) => {
                                bail!(
                                    "lightmap charts {} and {} overlap at texel ({x}, {y}) in {}",
                                    owner,
                                    chart_id,
                                    primitive.name
                                );
                            }
                        }
                    }
                }
            }
            let mut tile_pixels = Vec::with_capacity(tile_width * tile_height);
            let mut tile_owners = Vec::with_capacity(tile_width * tile_height);
            let mut tile_features = Vec::with_capacity(tile_width * tile_height);
            for local_y in 0..tile_height {
                for local_x in 0..tile_width {
                    let index = (tile_start_y + local_y) * width as usize + tile_start_x + local_x;
                    tile_pixels.push(pixels[index]);
                    tile_owners.push(chart_owners[index]);
                    tile_features.push(features[index]);
                }
            }
            sampling_summary.merge(tile_sampling_summary);
            let payload = encode_tile_payload(
                tile_width as u32,
                tile_height as u32,
                &tile_pixels,
                &tile_owners,
                &tile_features,
                tile_sampling_summary,
            )?;
            cache.write(
                key,
                tile_fingerprint,
                tile_width as u32,
                tile_height as u32,
                &payload,
            )?;
            if let Some(progress) = progress {
                progress.unit_completed(Some(tile_total), Some(false));
            }
        }
    }
    let covered_texels = chart_owners.iter().filter(|owner| owner.is_some()).count();
    write_debug_images(
        debug_output_dir,
        primitive_index,
        width,
        height,
        &chart_owners,
        &features,
        debug,
    )?;
    write_variance_output(debug_output_dir, primitive_index, &chart_owners, &features)?;
    if let Some(progress) = progress {
        progress.phase_started(
            format!("primitive {} denoise and dilation", primitive_index + 1),
            None,
        );
    }
    if denoise_settings.iterations > 0 {
        let mut denoised = pixels
            .iter()
            .map(|pixel| pixel.to_array())
            .collect::<Vec<_>>();
        denoise(
            &mut denoised,
            &chart_owners,
            &features,
            width as usize,
            height as usize,
            denoise_settings.iterations,
        )
        .map_err(|error| anyhow::anyhow!("lightmap denoising failed: {error:?}"))?;
        for (pixel, denoised) in pixels.iter_mut().zip(denoised) {
            *pixel = Vec3::from_array(denoised);
        }
    }
    let dilated_texels = dilate_chart_aware(
        &mut pixels,
        &mut chart_owners,
        super::lightmap_uv::LIGHTMAP_PADDING_TEXELS as usize,
        width as usize,
        height as usize,
    );
    Ok((pixels, covered_texels, dilated_texels, sampling_summary))
}

fn write_debug_images(
    output_dir: &Path,
    primitive_index: usize,
    width: u32,
    height: u32,
    chart_owners: &[Option<u32>],
    features: &[DenoiseFeature],
    settings: LightmapDebugSettings,
) -> Result<()> {
    if !settings.uv && !settings.samples && !settings.variance {
        return Ok(());
    }
    let debug_dir = output_dir.join("debug");
    fs::create_dir_all(&debug_dir)?;
    let expected = (width as usize)
        .checked_mul(height as usize)
        .context("lightmap debug dimensions overflowed")?;
    if chart_owners.len() != expected || features.len() != expected {
        bail!("lightmap debug buffers have mismatched lengths");
    }

    let max_samples = features
        .iter()
        .zip(chart_owners)
        .filter_map(|(feature, owner)| owner.map(|_| feature.sample_count))
        .max()
        .unwrap_or(1)
        .max(1) as f32;
    let max_variance = features
        .iter()
        .zip(chart_owners)
        .filter_map(|(feature, owner)| {
            owner.and_then(|_| {
                feature
                    .relative_variance
                    .is_finite()
                    .then_some(feature.relative_variance)
            })
        })
        .fold(0.0_f32, f32::max);

    let mut uv_image = settings.uv.then(|| RgbaImage::new(width, height));
    let mut samples_image = settings.samples.then(|| RgbaImage::new(width, height));
    let mut variance_image = settings.variance.then(|| RgbaImage::new(width, height));
    for y in 0..height {
        for x in 0..width {
            let index = y as usize * width as usize + x as usize;
            let owner = chart_owners[index];
            let feature = features[index];
            if let Some(image) = uv_image.as_mut() {
                let pixel = owner.map(chart_debug_color).unwrap_or(Rgba([0, 0, 0, 0]));
                image.put_pixel(x, y, pixel);
            }
            if let Some(image) = samples_image.as_mut() {
                let value = owner
                    .map(|_| feature.sample_count as f32 / max_samples)
                    .unwrap_or(0.0);
                let channel = debug_channel(value);
                image.put_pixel(
                    x,
                    y,
                    Rgba([channel, channel, channel, owner.map(|_| 255).unwrap_or(0)]),
                );
            }
            if let Some(image) = variance_image.as_mut() {
                let value = if owner.is_some() && max_variance > 0.0 {
                    (feature.relative_variance.max(0.0) / max_variance).sqrt()
                } else {
                    0.0
                };
                let channel = debug_channel(value);
                image.put_pixel(
                    x,
                    y,
                    Rgba([channel, channel, channel, owner.map(|_| 255).unwrap_or(0)]),
                );
            }
        }
    }
    if let Some(image) = uv_image {
        image
            .save(debug_dir.join(format!("lightmap-debug-uv-{primitive_index:04}.png")))
            .context("writing lightmap UV debug image")?;
    }
    if let Some(image) = samples_image {
        image
            .save(debug_dir.join(format!("lightmap-debug-samples-{primitive_index:04}.png")))
            .context("writing lightmap sample debug image")?;
    }
    if let Some(image) = variance_image {
        image
            .save(debug_dir.join(format!("lightmap-debug-variance-{primitive_index:04}.png")))
            .context("writing lightmap variance debug image")?;
    }
    Ok(())
}

/// Persists the raw pre-denoise relative variance for offline bake inspection.
/// Covered texels contain the adaptive estimator's variance of the mean;
/// uncovered texels are NaN so consumers cannot mistake padding for a zero
/// variance estimate. The dimensions match the corresponding primitive page.
fn write_variance_output(
    output_dir: &Path,
    primitive_index: usize,
    chart_owners: &[Option<u32>],
    features: &[DenoiseFeature],
) -> Result<()> {
    if chart_owners.len() != features.len() {
        bail!("lightmap variance buffers have mismatched lengths");
    }
    let mut bytes = Vec::with_capacity(features.len() * std::mem::size_of::<f32>());
    for (owner, feature) in chart_owners.iter().zip(features) {
        let variance = owner.map_or(f32::NAN, |_| feature.relative_variance);
        bytes.extend_from_slice(&variance.to_le_bytes());
    }
    let path = output_dir.join(format!("lightmap-variance-{primitive_index:04}.r32f.raw"));
    fs::write(&path, bytes).with_context(|| format!("could not write {}", path.display()))?;
    Ok(())
}

fn chart_debug_color(chart_id: u32) -> Rgba<u8> {
    let mut hash = chart_id.wrapping_mul(0x9e37_79b9).rotate_left(13);
    hash ^= hash >> 16;
    Rgba([
        (hash & 0xff) as u8,
        ((hash >> 8) & 0xff) as u8,
        ((hash >> 16) & 0xff) as u8,
        255,
    ])
}

fn debug_channel(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

const TILE_PAYLOAD_MAGIC: &[u8; 8] = b"BVOLMP01";
const TILE_PAYLOAD_VERSION: u32 = 2;

fn encode_tile_payload(
    width: u32,
    height: u32,
    pixels: &[Vec3],
    owners: &[Option<u32>],
    features: &[DenoiseFeature],
    summary: LightmapSamplingSummary,
) -> Result<Vec<u8>> {
    let count = (width as usize)
        .checked_mul(height as usize)
        .context("lightmap tile dimensions overflowed")?;
    if pixels.len() != count || owners.len() != count || features.len() != count {
        bail!("lightmap tile payload buffers have mismatched lengths");
    }
    let valid_count = owners.iter().filter(|owner| owner.is_some()).count();
    let mut bytes = Vec::with_capacity(64 + valid_count * 60);
    bytes.extend_from_slice(TILE_PAYLOAD_MAGIC);
    bytes.extend_from_slice(&TILE_PAYLOAD_VERSION.to_le_bytes());
    bytes.extend_from_slice(&width.to_le_bytes());
    bytes.extend_from_slice(&height.to_le_bytes());
    bytes.extend_from_slice(&(summary.sampled_texels as u64).to_le_bytes());
    bytes.extend_from_slice(&summary.total_samples.to_le_bytes());
    bytes.extend_from_slice(&summary.min_samples.to_le_bytes());
    bytes.extend_from_slice(&summary.max_samples.to_le_bytes());
    bytes.extend_from_slice(&summary.max_relative_variance.to_le_bytes());
    bytes.extend_from_slice(
        &u32::try_from(valid_count)
            .context("lightmap tile valid texel count exceeds u32")?
            .to_le_bytes(),
    );
    for index in 0..count {
        let Some(owner) = owners[index] else {
            continue;
        };
        bytes.extend_from_slice(
            &u32::try_from(index)
                .context("lightmap tile texel index exceeds u32")?
                .to_le_bytes(),
        );
        for channel in pixels[index].to_array() {
            bytes.extend_from_slice(&channel.to_le_bytes());
        }
        bytes.extend_from_slice(&owner.to_le_bytes());
        for channel in features[index].position {
            bytes.extend_from_slice(&channel.to_le_bytes());
        }
        for channel in features[index].normal {
            bytes.extend_from_slice(&channel.to_le_bytes());
        }
        bytes.extend_from_slice(&features[index].material_id.to_le_bytes());
        bytes.extend_from_slice(&features[index].relative_variance.to_le_bytes());
        bytes.extend_from_slice(&features[index].coverage.to_le_bytes());
        bytes.extend_from_slice(&features[index].sample_count.to_le_bytes());
    }
    Ok(bytes)
}

fn decode_tile_payload(
    record: &TileRecord,
    expected_width: u32,
    expected_height: u32,
) -> Result<DecodedTilePayload> {
    if record.width != expected_width || record.height != expected_height {
        bail!("lightmap cache tile dimensions do not match the current page");
    }
    let bytes = &record.payload;
    let mut offset = 0;
    if take_payload(bytes, &mut offset, TILE_PAYLOAD_MAGIC.len())? != TILE_PAYLOAD_MAGIC {
        bail!("lightmap tile payload has an invalid header");
    }
    if read_payload_u32(bytes, &mut offset)? != TILE_PAYLOAD_VERSION {
        bail!("unsupported lightmap tile payload version");
    }
    let width = read_payload_u32(bytes, &mut offset)?;
    let height = read_payload_u32(bytes, &mut offset)?;
    if width != expected_width || height != expected_height {
        bail!("lightmap tile payload dimensions do not match the tile record");
    }
    let sampled_texels = read_payload_u64(bytes, &mut offset)? as usize;
    let total_samples = read_payload_u64(bytes, &mut offset)?;
    let min_samples = read_payload_u32(bytes, &mut offset)?;
    let max_samples = read_payload_u32(bytes, &mut offset)?;
    let max_relative_variance = read_payload_f32(bytes, &mut offset)?;
    let valid_count = read_payload_u32(bytes, &mut offset)? as usize;
    let count = (width as usize)
        .checked_mul(height as usize)
        .context("lightmap tile dimensions overflowed")?;
    let mut pixels = vec![Vec3::ZERO; count];
    let mut owners = vec![None; count];
    let mut features = vec![DenoiseFeature::default(); count];
    for _ in 0..valid_count {
        let index = read_payload_u32(bytes, &mut offset)? as usize;
        if index >= count || owners[index].is_some() {
            bail!("lightmap tile payload contains an invalid texel index");
        }
        let pixel = [
            read_payload_f32(bytes, &mut offset)?,
            read_payload_f32(bytes, &mut offset)?,
            read_payload_f32(bytes, &mut offset)?,
        ];
        let owner = read_payload_u32(bytes, &mut offset)?;
        let position = [
            read_payload_f32(bytes, &mut offset)?,
            read_payload_f32(bytes, &mut offset)?,
            read_payload_f32(bytes, &mut offset)?,
        ];
        let normal = [
            read_payload_f32(bytes, &mut offset)?,
            read_payload_f32(bytes, &mut offset)?,
            read_payload_f32(bytes, &mut offset)?,
        ];
        let material_id = read_payload_u32(bytes, &mut offset)?;
        let relative_variance = read_payload_f32(bytes, &mut offset)?;
        let coverage = read_payload_f32(bytes, &mut offset)?;
        let sample_count = read_payload_u32(bytes, &mut offset)?;
        if owner == u32::MAX {
            bail!("lightmap tile payload contains an invalid chart owner");
        }
        pixels[index] = Vec3::from_array(pixel);
        owners[index] = Some(owner);
        features[index] = DenoiseFeature {
            position,
            normal,
            material_id,
            relative_variance,
            coverage,
            sample_count,
        };
    }
    if offset != bytes.len() {
        bail!("lightmap tile payload has trailing bytes");
    }
    Ok(DecodedTilePayload {
        #[cfg(feature = "lightmap-gpu-solari")]
        width: width as usize,
        #[cfg(feature = "lightmap-gpu-solari")]
        height: height as usize,
        pixels,
        owners,
        features,
        summary: LightmapSamplingSummary {
            sampled_texels,
            total_samples,
            min_samples,
            max_samples,
            max_relative_variance,
        },
    })
}

struct DecodedTilePayload {
    #[cfg(feature = "lightmap-gpu-solari")]
    width: usize,
    #[cfg(feature = "lightmap-gpu-solari")]
    height: usize,
    pixels: Vec<Vec3>,
    owners: Vec<Option<u32>>,
    features: Vec<DenoiseFeature>,
    summary: LightmapSamplingSummary,
}

fn take_payload<'a>(bytes: &'a [u8], offset: &mut usize, length: usize) -> Result<&'a [u8]> {
    let end = offset
        .checked_add(length)
        .context("lightmap tile payload length overflowed")?;
    let value = bytes
        .get(*offset..end)
        .context("lightmap tile payload is truncated")?;
    *offset = end;
    Ok(value)
}

fn read_payload_u32(bytes: &[u8], offset: &mut usize) -> Result<u32> {
    Ok(u32::from_le_bytes(
        take_payload(bytes, offset, 4)?
            .try_into()
            .expect("u32 slice length"),
    ))
}

fn read_payload_u64(bytes: &[u8], offset: &mut usize) -> Result<u64> {
    Ok(u64::from_le_bytes(
        take_payload(bytes, offset, 8)?
            .try_into()
            .expect("u64 slice length"),
    ))
}

fn read_payload_f32(bytes: &[u8], offset: &mut usize) -> Result<f32> {
    Ok(f32::from_le_bytes(
        take_payload(bytes, offset, 4)?
            .try_into()
            .expect("f32 slice length"),
    ))
}

/// Use one center sample for fully covered pixels and four quarter-pixel
/// samples only when the triangle crosses the pixel boundary. This keeps the
/// expensive transport work focused on UV edges while giving coverage a
/// deterministic 2x2 resolve.
fn pixel_sample_weights(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    triangle: [Vec2; 3],
) -> ([Option<[f32; 3]>; 4], usize) {
    let center = Vec2::new(
        (x as f32 + 0.5) / width as f32,
        (y as f32 + 0.5) / height as f32,
    );
    let center_weights = barycentric(center, triangle);
    let offsets = [
        Vec2::new(0.25, 0.25),
        Vec2::new(0.75, 0.25),
        Vec2::new(0.25, 0.75),
        Vec2::new(0.75, 0.75),
    ];
    let mut samples = [None; 4];
    let mut all_subsamples_covered = true;
    for (index, offset) in offsets.into_iter().enumerate() {
        samples[index] = barycentric(
            Vec2::new(
                (x as f32 + offset.x) / width as f32,
                (y as f32 + offset.y) / height as f32,
            ),
            triangle,
        );
        all_subsamples_covered &= samples[index].is_some();
    }
    if center_weights.is_some() && all_subsamples_covered {
        return ([center_weights, None, None, None], 1);
    }
    (samples, 4)
}

fn dilate_chart_aware(
    pixels: &mut [Vec3],
    chart_owners: &mut [Option<u32>],
    max_radius: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut dilated = 0;
    for _ in 0..max_radius {
        let mut candidates = Vec::new();
        let mut candidate_charts = vec![None; chart_owners.len()];
        for index in 0..chart_owners.len() {
            if chart_owners[index].is_some() {
                continue;
            }
            let x = index % width;
            let y = index / width;
            let mut source = None;
            let mut chart = None;
            let mut mixed = false;
            for offset_y in -1_i32..=1 {
                for offset_x in -1_i32..=1 {
                    if offset_x == 0 && offset_y == 0 {
                        continue;
                    }
                    let neighbor_x = x as i32 + offset_x;
                    let neighbor_y = y as i32 + offset_y;
                    if neighbor_x < 0
                        || neighbor_y < 0
                        || neighbor_x >= width as i32
                        || neighbor_y >= height as i32
                    {
                        continue;
                    }
                    let neighbor = neighbor_y as usize * width + neighbor_x as usize;
                    let Some(neighbor_chart) = chart_owners[neighbor] else {
                        continue;
                    };
                    if chart.is_some_and(|existing| existing != neighbor_chart) {
                        mixed = true;
                        break;
                    }
                    chart = Some(neighbor_chart);
                    source = Some(pixels[neighbor]);
                }
                if mixed {
                    break;
                }
            }
            if mixed {
                continue;
            }
            let (Some(chart), Some(value)) = (chart, source) else {
                continue;
            };
            candidate_charts[index] = Some(chart);
            candidates.push((index, chart, value));
        }
        let assignments = candidates
            .into_iter()
            .filter(|(index, chart, _)| {
                let x = index % width;
                let y = index / width;
                for offset_y in -1_i32..=1 {
                    for offset_x in -1_i32..=1 {
                        if offset_x == 0 && offset_y == 0 {
                            continue;
                        }
                        let neighbor_x = x as i32 + offset_x;
                        let neighbor_y = y as i32 + offset_y;
                        if neighbor_x < 0
                            || neighbor_y < 0
                            || neighbor_x >= width as i32
                            || neighbor_y >= height as i32
                        {
                            continue;
                        }
                        let neighbor = neighbor_y as usize * width + neighbor_x as usize;
                        if candidate_charts[neighbor].is_some_and(|other| other != *chart) {
                            return false;
                        }
                    }
                }
                true
            })
            .collect::<Vec<_>>();
        if assignments.is_empty() {
            break;
        }
        for (index, chart, value) in assignments {
            chart_owners[index] = Some(chart);
            pixels[index] = value;
            dilated += 1;
        }
    }
    dilated
}

fn barycentric(point: Vec2, triangle: [Vec2; 3]) -> Option<[f32; 3]> {
    let denominator = (triangle[1].y - triangle[2].y) * (triangle[0].x - triangle[2].x)
        + (triangle[2].x - triangle[1].x) * (triangle[0].y - triangle[2].y);
    if denominator.abs() <= f32::EPSILON {
        return None;
    }
    let w0 = ((triangle[1].y - triangle[2].y) * (point.x - triangle[2].x)
        + (triangle[2].x - triangle[1].x) * (point.y - triangle[2].y))
        / denominator;
    let w1 = ((triangle[2].y - triangle[0].y) * (point.x - triangle[2].x)
        + (triangle[0].x - triangle[2].x) * (point.y - triangle[2].y))
        / denominator;
    let w2 = 1.0 - w0 - w1;
    (w0 >= -1e-5 && w1 >= -1e-5 && w2 >= -1e-5).then_some([w0, w1, w2])
}

fn interpolate_vec2(values: [Vec2; 3], weights: [f32; 3]) -> Vec2 {
    values[0] * weights[0] + values[1] * weights[1] + values[2] * weights[2]
}

fn interpolate_vec3(values: [Vec3; 3], weights: [f32; 3]) -> Vec3 {
    values[0] * weights[0] + values[1] * weights[1] + values[2] * weights[2]
}

fn interpolate_vec4(values: [Vec4; 3], weights: [f32; 3]) -> Vec4 {
    values[0] * weights[0] + values[1] * weights[1] + values[2] * weights[2]
}

fn encode_rgba16f(pixels: &[Vec3]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(pixels.len() * 8);
    for pixel in pixels {
        for channel in [pixel.x, pixel.y, pixel.z, 1.0] {
            bytes.extend_from_slice(&f16::from_f32(channel).to_le_bytes());
        }
    }
    bytes
}

#[cfg(test)]
#[path = "tests/lightmap.rs"]
mod tests;
