//! Engine-independent actor animation catalog contracts and discovery policy.
//!
//! The application crate supplies a case-normalized view of loose/BSA assets
//! and later fills conversion metadata. Keeping path resolution, ordering,
//! diagnostics, and set reuse here makes them testable without Bevy or I/O.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PreparedActorAnimationKind {
    #[default]
    Npc,
    Creature,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PreparedActorAnimationClipStatus {
    #[default]
    Ready,
    Missing,
    Incompatible,
    Malformed,
    ConversionFailed,
    NotConverted,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PreparedActorAnimationLoopMode {
    Loop,
    Clamp,
    Reverse,
    Mixed,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PreparedActorAnimationRootMotionPolicy {
    #[default]
    Unknown,
    /// Keep every source controller channel, including the accumulation root,
    /// in the exported clip. Runtime extraction is a later policy decision.
    PreserveAuthored,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PreparedActorAnimationTextKey {
    pub time_seconds: f32,
    pub value: String,
}

impl PreparedActorAnimationClipStatus {
    #[must_use]
    pub const fn diagnostic_code(self) -> Option<&'static str> {
        match self {
            Self::Ready => None,
            Self::Missing => Some("missing_kf"),
            Self::Incompatible => Some("incompatible_kf"),
            Self::Malformed => Some("malformed_kf"),
            Self::ConversionFailed => Some("conversion_failed"),
            Self::NotConverted => Some("conversion_not_requested"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedActorAnimationDiagnostic {
    pub severity: String,
    pub code: String,
    #[serde(default)]
    pub source_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PreparedActorAnimationClip {
    pub name: String,
    /// Normalized Data-relative path, including the `meshes/` prefix.
    pub source_kf_path: String,
    /// KFFZ spelling before model-directory resolution, or the discovered path
    /// for default-directory enumeration.
    pub original_kf_path: String,
    pub source_fingerprint: String,
    pub status: PreparedActorAnimationClipStatus,
    #[serde(default)]
    pub duration_seconds: Option<f32>,
    #[serde(default)]
    pub source_sequence_name: Option<String>,
    #[serde(default)]
    pub source_start_seconds: Option<f32>,
    #[serde(default)]
    pub source_end_seconds: Option<f32>,
    #[serde(default)]
    pub source_frequency: Option<f32>,
    #[serde(default)]
    pub source_phase: Option<f32>,
    #[serde(default)]
    pub loop_mode: PreparedActorAnimationLoopMode,
    #[serde(default)]
    pub root_motion_policy: PreparedActorAnimationRootMotionPolicy,
    #[serde(default)]
    pub accumulation_root: Option<String>,
    #[serde(default)]
    pub animated_channel_count: usize,
    #[serde(default)]
    pub animated_target_count: usize,
    #[serde(default)]
    pub required_targets: Vec<String>,
    #[serde(default)]
    pub animated_targets: Vec<String>,
    #[serde(default)]
    pub missing_targets: Vec<String>,
    #[serde(default)]
    pub controller_types: Vec<String>,
    #[serde(default)]
    pub interpolator_types: Vec<String>,
    #[serde(default)]
    pub text_keys: Vec<PreparedActorAnimationTextKey>,
    #[serde(default)]
    pub diagnostics: Vec<PreparedActorAnimationDiagnostic>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PreparedActorAnimationSet {
    pub id: String,
    pub skeleton_path: String,
    pub skeleton_fingerprint: String,
    pub source_fingerprint: String,
    #[serde(default)]
    pub clip_pack_asset_path: Option<String>,
    #[serde(default)]
    pub clip_pack_hash: Option<String>,
    pub clips: Vec<PreparedActorAnimationClip>,
    #[serde(default)]
    pub diagnostics: Vec<PreparedActorAnimationDiagnostic>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedActorAnimationReference {
    pub reference_form_id: u32,
    pub base_form_id: u32,
    pub kind: PreparedActorAnimationKind,
    pub animation_set_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PreparedActorAnimationCatalog {
    pub revision: String,
    pub source_fingerprint: String,
    pub actor_mappings: Vec<PreparedActorAnimationReference>,
    pub animation_sets: Vec<PreparedActorAnimationSet>,
    #[serde(default)]
    pub diagnostics: Vec<PreparedActorAnimationDiagnostic>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KffzDecodeResult {
    pub paths: Vec<String>,
    pub diagnostics: Vec<String>,
}

/// Decodes a `KFFZ` zero-terminated filename array without making a malformed
/// field fatal to its actor. Invalid UTF-8 is lossily retained and diagnosed.
#[must_use]
pub fn decode_kffz(data: &[u8]) -> KffzDecodeResult {
    let mut diagnostics = Vec::new();
    if !data.is_empty() && data.last() != Some(&0) {
        diagnostics.push("KFFZ malformed: filename array is not NUL-terminated".to_owned());
    }
    let mut paths = Vec::new();
    for chunk in data
        .split(|byte| *byte == 0)
        .filter(|chunk| !chunk.is_empty())
    {
        if std::str::from_utf8(chunk).is_err() {
            diagnostics.push("KFFZ malformed: filename contains invalid UTF-8".to_owned());
        }
        let path = String::from_utf8_lossy(chunk).replace('\\', "/");
        if !path.is_empty() {
            paths.push(path);
        }
    }
    KffzDecodeResult { paths, diagnostics }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActorAnimationDiscoveryInput {
    pub reference_form_id: u32,
    pub base_form_id: u32,
    pub kind: PreparedActorAnimationKind,
    pub model_path: String,
    pub skeleton_path: String,
    pub skeleton_fingerprint: String,
    pub explicit_kf_paths: Vec<String>,
    pub default_directories: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActorAnimationAsset {
    pub path: String,
    pub fingerprint: String,
    pub state: ActorAnimationAssetState,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ActorAnimationAssetState {
    #[default]
    Compatible,
    Incompatible(String),
    Malformed(String),
}

fn normalize_path(path: &str) -> String {
    let mut output = path.trim().replace('\\', "/").to_ascii_lowercase();
    while output.contains("//") {
        output = output.replace("//", "/");
    }
    let mut parts = Vec::new();
    for part in output.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                let _ = parts.pop();
            }
            value => parts.push(value),
        }
    }
    parts.join("/")
}

#[must_use]
pub fn canonical_mesh_path(path: &str) -> String {
    let path = normalize_path(path);
    if path.starts_with("meshes/") {
        path
    } else {
        format!("meshes/{path}")
    }
}

fn parent_path(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

/// Resolve an explicit filename the same way OpenMW documents ESM4 `mKf`:
/// relative to the directory containing the actor's model. A fully
/// Data-relative `meshes/...` entry remains rooted at `meshes/`.
#[must_use]
pub fn resolve_explicit_kf_path(model_path: &str, entry: &str) -> String {
    let entry = normalize_path(entry);
    if entry.starts_with("meshes/") {
        return entry;
    }
    let model = canonical_mesh_path(model_path);
    let parent = parent_path(&model);
    normalize_path(&format!("{parent}/{entry}"))
}

#[must_use]
pub fn normalize_clip_name(source_path: &str) -> String {
    let file = source_path.rsplit('/').next().unwrap_or(source_path);
    let stem = file.rsplit_once('.').map_or(file, |(stem, _)| stem);
    let mut output = String::new();
    let mut separator = false;
    for character in stem.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            if separator && !output.is_empty() {
                output.push('_');
            }
            separator = false;
            output.push(character);
        } else {
            separator = true;
        }
    }
    if output.is_empty() {
        "clip".to_owned()
    } else {
        output
    }
}

fn assign_clip_names(clips: &mut [PreparedActorAnimationClip]) {
    let mut counts = HashMap::<String, usize>::new();
    for clip in clips {
        let base = normalize_clip_name(&clip.source_kf_path);
        let count = counts.entry(base.clone()).or_default();
        *count += 1;
        clip.name = if *count == 1 {
            base
        } else {
            format!("{base}__{count}")
        };
    }
}

/// Stable, dependency-free content identity. Inputs already contain strong
/// byte fingerprints supplied by the application adapter; FNV-1a combines
/// those identities without adding a hashing crate to `bevyout-core`.
fn stable_identity(parts: impl IntoIterator<Item = String>) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for part in parts {
        for byte in part.bytes().chain(std::iter::once(0)) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    format!("{hash:016x}")
}

fn diagnostic(
    code: &str,
    path: &str,
    message: impl Into<String>,
) -> PreparedActorAnimationDiagnostic {
    PreparedActorAnimationDiagnostic {
        severity: "warning".to_owned(),
        code: code.to_owned(),
        source_path: Some(path.to_owned()),
        message: message.into(),
    }
}

fn discovered_paths(
    actor: &ActorAnimationDiscoveryInput,
    assets: &BTreeMap<String, ActorAnimationAsset>,
) -> Vec<(String, String)> {
    let mut paths = if actor.explicit_kf_paths.is_empty() {
        let mut directories = actor
            .default_directories
            .iter()
            .map(|directory| canonical_mesh_path(directory))
            .collect::<Vec<_>>();
        if directories.is_empty() {
            directories.push(parent_path(&canonical_mesh_path(&actor.skeleton_path)).to_owned());
        }
        directories.sort();
        directories.dedup();
        assets
            .keys()
            .filter(|path| {
                path.ends_with(".kf")
                    && directories.iter().any(|directory| {
                        path.as_str() == directory || path.starts_with(&format!("{directory}/"))
                    })
            })
            .map(|path| (path.clone(), path.clone()))
            .collect::<Vec<_>>()
    } else {
        actor
            .explicit_kf_paths
            .iter()
            .map(|entry| {
                let direct = canonical_mesh_path(entry);
                let resolved = if entry
                    .replace('\\', "/")
                    .to_ascii_lowercase()
                    .starts_with("meshes/")
                {
                    direct
                } else {
                    resolve_explicit_kf_path(&actor.model_path, entry)
                };
                (resolved, entry.replace('\\', "/"))
            })
            .collect()
    };
    paths.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    paths.dedup_by(|left, right| left.0 == right.0);
    paths
}

/// Builds the versioned, deterministic prepared catalog. Conversion fills the
/// ready clips' durations/channel counts and the set's clip-pack path later.
#[must_use]
pub fn build_actor_animation_catalog(
    revision: &str,
    source_fingerprint: &str,
    actors: &[ActorAnimationDiscoveryInput],
    available_assets: &[ActorAnimationAsset],
) -> PreparedActorAnimationCatalog {
    let assets = available_assets
        .iter()
        .cloned()
        .map(|mut asset| {
            asset.path = canonical_mesh_path(&asset.path);
            (asset.path.clone(), asset)
        })
        .collect::<BTreeMap<_, _>>();
    let mut actors = actors.to_vec();
    actors.sort_by_key(|actor| (actor.reference_form_id, actor.base_form_id));

    let mut actor_mappings = Vec::new();
    let mut animation_sets = Vec::new();
    let mut set_indices = HashMap::<String, usize>::new();
    for actor in actors {
        let skeleton_path = canonical_mesh_path(&actor.skeleton_path);
        let mut clips = Vec::new();
        let mut set_diagnostics = Vec::new();
        for (path, original) in discovered_paths(&actor, &assets) {
            let (fingerprint, status, details) = match assets.get(&path) {
                Some(asset) => match &asset.state {
                    ActorAnimationAssetState::Compatible => (
                        asset.fingerprint.clone(),
                        PreparedActorAnimationClipStatus::Ready,
                        None,
                    ),
                    ActorAnimationAssetState::Incompatible(message) => (
                        asset.fingerprint.clone(),
                        PreparedActorAnimationClipStatus::Incompatible,
                        Some(message.clone()),
                    ),
                    ActorAnimationAssetState::Malformed(message) => (
                        asset.fingerprint.clone(),
                        PreparedActorAnimationClipStatus::Malformed,
                        Some(message.clone()),
                    ),
                },
                None => (
                    String::new(),
                    PreparedActorAnimationClipStatus::Missing,
                    Some("KF asset was not found in loose files or loaded archives".to_owned()),
                ),
            };
            let mut diagnostics = Vec::new();
            if let Some(code) = status.diagnostic_code() {
                let item = diagnostic(code, &path, details.unwrap_or_else(|| code.to_owned()));
                set_diagnostics.push(item.clone());
                diagnostics.push(item);
            }
            clips.push(PreparedActorAnimationClip {
                source_kf_path: path,
                original_kf_path: original,
                source_fingerprint: fingerprint,
                status,
                diagnostics,
                ..PreparedActorAnimationClip::default()
            });
        }
        assign_clip_names(&mut clips);
        let set_fingerprint = stable_identity(
            std::iter::once(skeleton_path.clone())
                .chain(std::iter::once(actor.skeleton_fingerprint.clone()))
                .chain(clips.iter().flat_map(|clip| {
                    [
                        clip.source_kf_path.clone(),
                        clip.source_fingerprint.clone(),
                        format!("{:?}", clip.status),
                    ]
                })),
        );
        let set_index = if let Some(index) = set_indices.get(&set_fingerprint).copied() {
            index
        } else {
            let index = animation_sets.len();
            animation_sets.push(PreparedActorAnimationSet {
                id: format!("animation-set-{set_fingerprint}"),
                skeleton_path,
                skeleton_fingerprint: actor.skeleton_fingerprint.clone(),
                source_fingerprint: set_fingerprint.clone(),
                clips,
                diagnostics: set_diagnostics,
                ..PreparedActorAnimationSet::default()
            });
            set_indices.insert(set_fingerprint.clone(), index);
            index
        };
        actor_mappings.push(PreparedActorAnimationReference {
            reference_form_id: actor.reference_form_id,
            base_form_id: actor.base_form_id,
            kind: actor.kind,
            animation_set_id: animation_sets[set_index].id.clone(),
        });
    }
    animation_sets.sort_by(|left, right| left.id.cmp(&right.id));
    PreparedActorAnimationCatalog {
        revision: revision.to_owned(),
        source_fingerprint: source_fingerprint.to_owned(),
        actor_mappings,
        animation_sets,
        diagnostics: Vec::new(),
    }
}

#[cfg(test)]
#[path = "tests/actor_animation.rs"]
mod tests;
