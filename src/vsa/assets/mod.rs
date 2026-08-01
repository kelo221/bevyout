use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

mod actor_animation;
mod archives;
mod cache;
mod material_glb;
mod material_policy;
pub(crate) mod texture_ktx;
mod textures;

pub(crate) use actor_animation::*;
pub(crate) use archives::*;
pub(crate) use cache::*;
pub(crate) use material_glb::*;
pub(crate) use material_policy::*;
pub(crate) use texture_ktx::*;
pub(crate) use textures::*;

use super::bsa::BsaArchive;
use super::manifest::Diagnostic;
use super::paths::{fingerprint, normalize_asset_path};
use super::physics::read_physics_asset;

/// Native static conversion cache identity.
pub(crate) const NATIVE_NIF_CONVERTER_REVISION: &str = "nifty-fo3-native-v10-normal-y-v1-specular-normal-alpha-v1-fallout-shader-semantics-v1-emissive-quarter-cap-v1-shader-emission-gate-v2-physical-effect-bulb-v1-effect-emission-control-v1-light-card-promotion-v1-env-light-emission-v1-17f5769-pbr-material-v3-workers-v2-anim-xyzw-v1-audio-cues-v1-havok-joints-v1-com-frame-v1-ktx2-uastc-v1-segmented-trishape-v1";

/// Native actor assembly cache identity. Keep this separate from static NIFs
/// so skin-binding fixes rebuild actors without invalidating the world.
pub(crate) const NATIVE_ACTOR_CONVERTER_REVISION: &str = "nifty-fo3-native-actor-assembly-v13-normal-y-v1-specular-normal-alpha-v1-pbr-material-v3-selective-head-anims-ktx2-uastc-v1-0dfd052";

/// Legacy prepared-scene revision retained only so old manifests can produce a
/// clear stale-cache result; new preparation records the native revision below.
pub(crate) const PREPARED_CONVERTER_REVISION: &str = "niftools-blender52-visual-audit-havok-anim-audio-emission-actors-v36-fallout-shader-semantics-v1-emissive-quarter-cap-v1-shader-emission-gate-v2-physical-effect-bulb-v1-effect-emission-control-v1-environment-light-emission-v1-emission-authority-v2-pbr-material-v3-ktx2-uastc-v1+pynifly-v32-normal-y-v1-pbr-material-v3-actor-bindpose-v22-eyes-creature-primary-fallback-ktx2-uastc-v1+day-night-profile-v1";

pub(crate) const NATIVE_PREPARED_CONVERTER_REVISION: &str = "nifty-fo3-native-v10-normal-y-v1-specular-normal-alpha-v1-fallout-shader-semantics-v1-emissive-quarter-cap-v1-shader-emission-gate-v2-physical-effect-bulb-v1-effect-emission-control-v1-light-card-promotion-v1-env-light-emission-v1-17f5769-pbr-material-v3-workers-v2-anim-xyzw-v1-audio-cues-v1-havok-joints-v1-com-frame-v1-ktx2-uastc-v1-segmented-trishape-v1+actor-assembly-v13-normal-y-v1-specular-normal-alpha-v1-pbr-material-v3-selective-head-anims-ktx2-uastc-v1-17f5769+day-night-profile-v1";

pub(crate) const SUPPORTED_PREPARED_CONVERTER_REVISIONS: &[&str] = &[
    PREPARED_CONVERTER_REVISION,
    NATIVE_PREPARED_CONVERTER_REVISION,
];

pub(crate) fn material_policy_identity(base_revision: &str) -> String {
    material_policy_identity_with_csv(base_revision, METALLIC_MATERIALS_CSV)
}

pub(crate) fn material_policy_identity_with_csv(base_revision: &str, csv: &str) -> String {
    let mut identity = Vec::new();
    identity.extend_from_slice(MATERIAL_POLICY_REVISION.as_bytes());
    identity.push(0);
    identity.extend_from_slice(csv.as_bytes());
    let digest = fingerprint(&identity);
    format!("{base_revision}+material-policy-{}", &digest[..16])
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct ActorAssemblyDescriptor {
    pub(crate) skeleton: String,
    pub(crate) visual_inputs: Vec<String>,
    /// Race body inputs remain in the import batch as underwear/skin
    /// fallbacks. Blender removes a covered part only after the selected worn
    /// apparel produced visible weighted geometry.
    #[serde(default)]
    pub(crate) body_parts: Vec<ActorBodyPartInput>,
    #[serde(default)]
    pub(crate) apparel: Vec<ActorApparelInput>,
    /// Visual inputs whose independent NIF roots belong to the animated head.
    #[serde(default)]
    pub(crate) head_parts: Vec<String>,
    /// Head visuals authored directly in the `HeadAnims` frame. FO3 hair
    /// roots omit the compensating rotation carried by eyes and mouth parts.
    #[serde(default)]
    pub(crate) head_anim_parts: Vec<String>,
    /// Staged source NIFs for the race's left/right eye geometry.
    #[serde(default)]
    pub(crate) eye_geometry: Vec<String>,
    /// EYES diffuse texture, relative to the staging data root.
    #[serde(default)]
    pub(crate) eye_texture: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct ActorBodyPartInput {
    pub(crate) path: String,
    pub(crate) index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct ActorApparelInput {
    pub(crate) path: String,
    pub(crate) form_id: u32,
    pub(crate) biped_slot_mask: u32,
}

/// Apparel is part of the baked actor appearance. Weapons are runtime
/// attachments and must not become permanently skinned body geometry.
pub(crate) fn actor_visual_gear_kind(kind: &str) -> bool {
    kind.eq_ignore_ascii_case("ARMO")
}

/// Canonical actor input contract shared by preparation and executable specs.
/// The explicit skeleton is never displaced by sorting. It is also retained as
/// a visual input because creature skeleton NIFs may contain render geometry.
pub(crate) fn canonical_actor_assembly(
    skeleton: Option<String>,
    visual_inputs: Vec<String>,
) -> Option<ActorAssemblyDescriptor> {
    let mut visual_inputs = visual_inputs
        .into_iter()
        .map(|path| normalize_asset_path(&path))
        .filter(|path| !path.is_empty())
        .collect::<Vec<_>>();
    visual_inputs.sort();
    visual_inputs.dedup();

    let skeleton = skeleton
        .map(|path| normalize_asset_path(&path))
        .filter(|path| !path.is_empty())
        .or_else(|| {
            visual_inputs.iter().find_map(|path| {
                let normalized = path.replace('\\', "/");
                let mut components = normalized.rsplit('/');
                let file_stem = components.next()?.rsplit_once('.')?.0;
                let parent = components.next()?;
                file_stem.eq_ignore_ascii_case(parent).then(|| path.clone())
            })
        })
        .or_else(|| visual_inputs.first().cloned())?;
    visual_inputs.retain(|path| path != &skeleton);
    visual_inputs.insert(0, skeleton.clone());
    Some(ActorAssemblyDescriptor {
        skeleton,
        visual_inputs,
        body_parts: Vec::new(),
        apparel: Vec::new(),
        head_parts: Vec::new(),
        head_anim_parts: Vec::new(),
        eye_geometry: Vec::new(),
        eye_texture: None,
    })
}

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
    WorldspaceLod,
}

impl AssetConversion {
    pub(crate) fn profile_tag(self) -> &'static str {
        match self {
            Self::Preserve => "ao-none",
            Self::QuickAo => "ao-quick-v1",
            Self::WorldspaceLod => "lod-skirts-v1",
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
pub(crate) struct AssetJob {
    pub(crate) kind: AssetJobKind,
    pub(crate) input: PathBuf,
    pub(crate) output: PathBuf,
    pub(crate) physics_output: PathBuf,
    pub(crate) model: String,
    pub(crate) conversion: AssetConversion,
    pub(crate) root_transform_policy: RootTransformPolicy,
}

/// Returns the retained legacy Blender preview/reference script for tests and
/// explicitly requested offline comparison tooling. Production preparation is
/// native-only and never calls this function.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn legacy_blender_preview_script() -> &'static str {
    include_str!("blender_script.py")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssetJobKind {
    StaticNif,
    ActorAssembly,
}

pub(crate) fn content_addressed_glb_name(converter_revision: &str, nif_bytes: &[u8]) -> String {
    let converter_revision = material_policy_identity(converter_revision);
    let mut cache_key = converter_revision.as_bytes().to_vec();
    cache_key.push(0);
    cache_key.extend_from_slice(nif_bytes);
    format!("{}.glb", fingerprint(&cache_key))
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
