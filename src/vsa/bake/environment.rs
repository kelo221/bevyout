//! Authored environment-radiance input for CPU light transport.
//!
//! The bake contract intentionally accepts an equirectangular `.hdr` file,
//! rather than reusing a runtime reflection-probe cube. Reflection probes are
//! view-dependent scene captures and are not a stable world-radiance source.

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub(crate) struct EnvironmentMap {
    width: u32,
    height: u32,
    pixels: Arc<Vec<[f32; 3]>>,
    importance_cdf: Arc<Vec<f32>>,
    importance_total: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct EnvironmentImportanceSample {
    pub(crate) direction: [f32; 3],
    pub(crate) radiance: [f32; 3],
    pub(crate) pdf_solid_angle: f32,
}

impl EnvironmentMap {
    pub(crate) fn load(path: &Path) -> Result<Self> {
        let is_hdr = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("hdr"));
        if !is_hdr {
            bail!(
                "lightmap environment map must be an equirectangular .hdr file: {}",
                path.display()
            );
        }
        let image = image::open(path)
            .with_context(|| {
                format!(
                    "could not decode lightmap environment map {}",
                    path.display()
                )
            })?
            .into_rgb32f();
        let (width, height) = image.dimensions();
        if width != height.saturating_mul(2) {
            bail!(
                "lightmap environment map must be 2:1 equirectangular, got {}x{}",
                width,
                height
            );
        }
        let pixels = image.pixels().map(|pixel| pixel.0).collect::<Vec<_>>();
        Self::from_pixels(width, height, pixels)
    }

    pub(crate) fn from_pixels(width: u32, height: u32, pixels: Vec<[f32; 3]>) -> Result<Self> {
        let expected = (width as usize)
            .checked_mul(height as usize)
            .context("environment map dimensions overflowed")?;
        if width == 0 || height == 0 || pixels.len() != expected {
            bail!(
                "environment map dimensions {}x{} do not match {} pixels",
                width,
                height,
                pixels.len()
            );
        }
        if pixels
            .iter()
            .flat_map(|pixel| pixel.iter())
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            bail!("environment map contains invalid radiance values");
        }
        let mut importance_cdf = Vec::with_capacity(pixels.len());
        let mut importance_total = 0.0;
        for (index, pixel) in pixels.iter().enumerate() {
            let y = index / width as usize;
            importance_total += Self::importance_weight(*pixel, y as u32, height);
            importance_cdf.push(importance_total);
        }
        if !importance_total.is_finite() {
            bail!("environment map importance weights overflowed");
        }
        Ok(Self {
            width,
            height,
            pixels: Arc::new(pixels),
            importance_cdf: Arc::new(importance_cdf),
            importance_total,
        })
    }

    pub(crate) fn sample(&self, direction: [f32; 3]) -> [f32; 3] {
        let Some((u, v)) = Self::uv_from_direction(direction) else {
            return [0.0; 3];
        };
        let x = u * self.width as f32 - 0.5;
        let y = v * self.height as f32 - 0.5;
        let x0 = x.floor() as i64;
        let y0 = y.floor() as i64;
        let tx = x - x.floor();
        let ty = y - y.floor();
        let x1 = x0 + 1;
        let y1 = y0 + 1;
        let p00 = self.pixel(x0, y0);
        let p10 = self.pixel(x1, y0);
        let p01 = self.pixel(x0, y1);
        let p11 = self.pixel(x1, y1);
        std::array::from_fn(|channel| {
            let top = p00[channel] * (1.0 - tx) + p10[channel] * tx;
            let bottom = p01[channel] * (1.0 - tx) + p11[channel] * tx;
            top * (1.0 - ty) + bottom * ty
        })
    }

    /// Samples the authored map with a luminance-times-solid-angle
    /// distribution. The returned PDF is with respect to solid angle, which
    /// makes it usable directly in the diffuse/environment MIS estimator.
    pub(crate) fn sample_importance(&self, u0: f32, u1: f32) -> EnvironmentImportanceSample {
        if self.importance_total <= f32::EPSILON {
            let y = 1.0 - 2.0 * u0.clamp(0.0, 1.0);
            let phi = std::f32::consts::TAU * u1.rem_euclid(1.0);
            let radius = (1.0 - y * y).max(0.0).sqrt();
            let direction = [radius * phi.cos(), y, radius * phi.sin()];
            return EnvironmentImportanceSample {
                direction,
                radiance: self.sample(direction),
                pdf_solid_angle: 1.0 / (4.0 * std::f32::consts::PI),
            };
        }

        let target = u0.clamp(0.0, 1.0 - f32::EPSILON) * self.importance_total;
        let mut low = 0usize;
        let mut high = self.importance_cdf.len();
        while low < high {
            let middle = low + (high - low) / 2;
            if self.importance_cdf[middle] > target {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        let index = low.min(self.importance_cdf.len() - 1);
        let x = index % self.width as usize;
        let y = index / self.width as usize;
        let u = (x as f32 + u1.rem_euclid(1.0)) / self.width as f32;
        let v = (y as f32 + u0.rem_euclid(1.0)) / self.height as f32;
        let direction = Self::direction_from_uv(u, v);
        EnvironmentImportanceSample {
            direction,
            radiance: self.sample(direction),
            pdf_solid_angle: self.pdf_solid_angle(direction),
        }
    }

    pub(crate) fn pdf_solid_angle(&self, direction: [f32; 3]) -> f32 {
        if self.importance_total <= f32::EPSILON {
            return 1.0 / (4.0 * std::f32::consts::PI);
        }
        let Some((u, v)) = Self::uv_from_direction(direction) else {
            return 0.0;
        };
        let x = ((u * self.width as f32).floor() as u32).min(self.width - 1);
        let y = ((v * self.height as f32).floor() as u32).min(self.height - 1);
        let index = (y * self.width + x) as usize;
        let probability =
            Self::importance_weight(self.pixels[index], y, self.height) / self.importance_total;
        let solid_angle = self.pixel_solid_angle(y);
        if probability <= 0.0 || solid_angle <= 0.0 {
            0.0
        } else {
            probability / solid_angle
        }
    }

    pub(crate) fn constant_radiance(&self) -> Option<[f32; 3]> {
        let first = *self.pixels.first()?;
        self.pixels
            .iter()
            .all(|pixel| *pixel == first)
            .then_some(first)
    }

    #[cfg(feature = "lightmap-gpu-solari")]
    pub(crate) fn solari_data(&self) -> (u32, u32, Arc<Vec<[f32; 3]>>, Arc<Vec<f32>>) {
        (
            self.width,
            self.height,
            self.pixels.clone(),
            self.importance_cdf.clone(),
        )
    }

    fn importance_weight(pixel: [f32; 3], y: u32, height: u32) -> f32 {
        let luminance = 0.2126 * pixel[0] + 0.7152 * pixel[1] + 0.0722 * pixel[2];
        let theta = std::f32::consts::PI * (y as f32 + 0.5) / height as f32;
        luminance.max(0.0) * theta.sin().max(0.0)
    }

    fn pixel_solid_angle(&self, y: u32) -> f32 {
        let theta0 = std::f32::consts::PI * y as f32 / self.height as f32;
        let theta1 = std::f32::consts::PI * (y + 1) as f32 / self.height as f32;
        std::f32::consts::TAU / self.width as f32 * (theta0.cos() - theta1.cos()).max(0.0)
    }

    fn uv_from_direction(direction: [f32; 3]) -> Option<(f32, f32)> {
        let length = direction
            .iter()
            .map(|component| component * component)
            .sum::<f32>()
            .sqrt();
        if !length.is_finite() || length <= f32::EPSILON {
            return None;
        }
        let direction = direction.map(|component| component / length);
        Some((
            (0.5 + direction[2].atan2(direction[0]) / std::f32::consts::TAU).rem_euclid(1.0),
            direction[1].clamp(-1.0, 1.0).acos() / std::f32::consts::PI,
        ))
    }

    fn direction_from_uv(u: f32, v: f32) -> [f32; 3] {
        let theta = v.clamp(0.0, 1.0) * std::f32::consts::PI;
        let phi = (u - 0.5) * std::f32::consts::TAU;
        let radius = theta.sin();
        [radius * phi.cos(), theta.cos(), radius * phi.sin()]
    }

    fn pixel(&self, x: i64, y: i64) -> [f32; 3] {
        let x = x.rem_euclid(i64::from(self.width)) as u32;
        let y = y.clamp(0, i64::from(self.height - 1)) as u32;
        self.pixels[(y * self.width + x) as usize]
    }
}

#[cfg(test)]
#[path = "tests/environment.rs"]
mod tests;
