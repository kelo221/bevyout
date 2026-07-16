use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

mod archives;
mod blender;
mod cache;
mod textures;

pub(crate) use archives::*;
pub(crate) use blender::*;
pub(crate) use cache::*;
use std::process::Command;
pub(crate) use textures::*;

use super::bsa::BsaArchive;
use super::manifest::Diagnostic;
use super::paths::{fingerprint, normalize_asset_path};
use super::physics::read_physics_asset;

/// Bump this whenever the embedded NIFTools conversion/filtering changes.
/// It is part of the content-addressed GLB name so stale conversions cannot
/// silently survive a converter fix.
pub(crate) const NIF_CONVERTER_REVISION: &str =
    "niftools-blender52-visual-audit-havok-anim-audio-emission-v24";

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AuthoredEmission {
    pub(crate) color: [f32; 3],
    pub(crate) strength: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum MaterialEmissionPolicy {
    None,
    Authored(AuthoredEmission),
    Explicit,
    Bulb,
    Glow,
}

pub(crate) fn authored_emission(color: [f32; 3], strength: f32) -> Option<AuthoredEmission> {
    let strength = if strength.is_finite() && strength >= 0.0 {
        strength
    } else {
        1.0
    };
    (color.iter().all(|channel| channel.is_finite()) && color.iter().any(|channel| *channel != 0.0))
        .then_some(AuthoredEmission { color, strength })
}

/// Mirrors the Blender-side authored-emission gate for std-only tests.
/// Zero-valued NIFTools colors are intentionally not exported as emission.
#[allow(dead_code)]
pub(crate) fn authored_emission_color(color: [f32; 3]) -> Option<[f32; 3]> {
    authored_emission(color, 1.0).map(|emission| emission.color)
}

#[allow(dead_code)]
pub(crate) fn material_emission_policy(
    color: [f32; 3],
    strength: f32,
    explicit: bool,
    bulb: bool,
    glow: bool,
) -> MaterialEmissionPolicy {
    if glow {
        MaterialEmissionPolicy::Glow
    } else if bulb {
        MaterialEmissionPolicy::Bulb
    } else if explicit {
        MaterialEmissionPolicy::Explicit
    } else {
        authored_emission(color, strength)
            .map(MaterialEmissionPolicy::Authored)
            .unwrap_or(MaterialEmissionPolicy::None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RootTransformPolicy {
    PreserveReviewRequired,
    PreserveVerified,
    DiscardVerified,
}

impl RootTransformPolicy {
    pub(crate) fn tag(self) -> &'static str {
        match self {
            Self::PreserveReviewRequired => "preserve_review_required",
            Self::PreserveVerified => "preserve_verified",
            Self::DiscardVerified => "discard_verified",
        }
    }

    pub(crate) fn requires_review(self) -> bool {
        matches!(self, Self::PreserveReviewRequired)
    }
}

pub(crate) fn normalized_model_policy_path(model: &str) -> String {
    let normalized = normalize_asset_path(model);
    normalized
        .strip_prefix("meshes/")
        .unwrap_or(&normalized)
        .to_owned()
}

pub(crate) fn root_transform_policy(model: &str) -> RootTransformPolicy {
    match normalized_model_policy_path(model).as_str() {
        "dungeons/vault/room/vrmwallscreen01.nif" => RootTransformPolicy::DiscardVerified,
        "dungeons/vault/room/vdnwallendcorinr01.nif"
        | "dungeons/vault/room/vdnwallendcoroutr01.nif" => RootTransformPolicy::PreserveVerified,
        _ => RootTransformPolicy::PreserveReviewRequired,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssetConversion {
    Preserve,
    QuickAo,
}

impl AssetConversion {
    pub(crate) fn profile_tag(self) -> &'static str {
        match self {
            Self::Preserve => "ao-none",
            Self::QuickAo => "ao-quick-v1",
        }
    }
}

pub(crate) fn asset_conversion(static_asset: bool) -> AssetConversion {
    if static_asset {
        AssetConversion::QuickAo
    } else {
        AssetConversion::Preserve
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BlenderAssetJob {
    pub(crate) input: PathBuf,
    pub(crate) output: PathBuf,
    pub(crate) physics_output: PathBuf,
    pub(crate) model: String,
    pub(crate) conversion: AssetConversion,
    pub(crate) root_transform_policy: RootTransformPolicy,
}

pub(crate) fn content_addressed_glb_name(converter_revision: &str, nif_bytes: &[u8]) -> String {
    let mut cache_key = converter_revision.as_bytes().to_vec();
    cache_key.push(0);
    cache_key.extend_from_slice(nif_bytes);
    format!("{}.glb", fingerprint(&cache_key))
}

pub(crate) fn find_blender(explicit: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = explicit {
        if path.exists() {
            return Ok(path);
        }
        bail!("Blender executable does not exist: {}", path.display());
    }
    let candidates = [
        PathBuf::from(r"C:\Program Files\Blender Foundation\Blender 5.2\blender.exe"),
        PathBuf::from(r"C:\Program Files\Blender Foundation\Blender 4.5\blender.exe"),
    ];
    candidates
        .into_iter()
        .find(|p| p.exists())
        .context("Blender was not found; pass --blender explicitly")
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
