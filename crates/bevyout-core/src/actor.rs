//! Engine-independent actor assembly and fallback contracts.
//!
//! Preparation resolves Fallout records and source assets into these values;
//! the Bevy viewer only projects the resulting blueprint into ECS entities.

use serde::{Deserialize, Serialize};

/// FO3 biped slot used by authored hair meshes.
pub const BIPED_SLOT_HAIR: u32 = 1 << 1;
/// FO3 biped slot used by hats that replace authored hair meshes.
pub const BIPED_SLOT_HAT: u32 = 1 << 10;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActorKind {
    #[default]
    Humanoid,
    Creature,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActorAttachmentPoint {
    #[default]
    Root,
    Head,
    LeftHand,
    RightHand,
    SideWeapon,
    BackWeapon,
    BackWeaponPistol,
    Spine,
    Pelvis,
    CustomNode(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActorMeshRole {
    Body(u32),
    Head(u32),
    Hair,
    Eyes,
    CreaturePart(u32),
}

impl ActorMeshRole {
    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::Body(index) => format!("Body({index})"),
            Self::Head(index) => format!("Head({index})"),
            Self::Hair => "Hair".to_owned(),
            Self::Eyes => "Eyes".to_owned(),
            Self::CreaturePart(index) => format!("CreaturePart({index})"),
        }
    }

    fn sort_key(self) -> (u8, u32) {
        match self {
            Self::Body(index) => (0, index),
            Self::Head(index) => (1, index),
            Self::Hair => (2, 0),
            Self::Eyes => (3, 0),
            Self::CreaturePart(index) => (4, index),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssembledMeshPart {
    pub name: String,
    pub source_form_id: Option<u32>,
    pub model_path: String,
    pub attachment_point: ActorAttachmentPoint,
    pub role: ActorMeshRole,
    #[serde(default = "default_true")]
    pub is_visible: bool,
}

fn default_true() -> bool {
    true
}

/// Makes source ordering irrelevant while removing exact duplicate parts.
/// Record precedence must already be resolved before this step. Multiple parts
/// may intentionally share a semantic role (notably the left and right eyes).
pub fn canonicalize_mesh_parts(parts: &mut Vec<AssembledMeshPart>) {
    parts.sort_by(|left, right| {
        left.role
            .sort_key()
            .cmp(&right.role.sort_key())
            .then_with(|| {
                left.model_path
                    .to_ascii_lowercase()
                    .cmp(&right.model_path.to_ascii_lowercase())
            })
            .then_with(|| left.source_form_id.cmp(&right.source_form_id))
    });
    parts.dedup_by(|left, right| {
        left.role == right.role
            && left.source_form_id == right.source_form_id
            && left.model_path.eq_ignore_ascii_case(&right.model_path)
            && left.attachment_point == right.attachment_point
    });
}

#[must_use]
pub fn hair_visible(occupied_biped_slots: u32) -> bool {
    occupied_biped_slots & (BIPED_SLOT_HAIR | BIPED_SLOT_HAT) == 0
}

#[must_use]
pub const fn eyes_visible(_occupied_biped_slots: u32) -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssembledApparel {
    pub item_form_id: u32,
    pub model_path: Option<String>,
    pub biped_slot_mask: u32,
    #[serde(default)]
    pub model_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorWeaponCandidate {
    pub item_form_id: u32,
    pub model_path: Option<String>,
    pub damage: u16,
    pub value: i32,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssembledWeapon {
    pub item_form_id: u32,
    pub model_path: Option<String>,
    pub attachment_point: ActorAttachmentPoint,
    pub model_available: bool,
}

/// Picks the strongest authored inventory weapon. Availability affects the
/// diagnostic/runtime projection, never which canonical item is equipped.
#[must_use]
pub fn select_starting_weapon(candidates: &[ActorWeaponCandidate]) -> Option<AssembledWeapon> {
    candidates
        .iter()
        .min_by(|left, right| {
            right
                .damage
                .cmp(&left.damage)
                .then_with(|| right.value.cmp(&left.value))
                .then_with(|| left.item_form_id.cmp(&right.item_form_id))
        })
        .map(|candidate| AssembledWeapon {
            item_form_id: candidate.item_form_id,
            model_path: candidate.model_path.clone(),
            attachment_point: ActorAttachmentPoint::RightHand,
            model_available: candidate.available && candidate.model_path.is_some(),
        })
}

fn valid_scale_component(value: Option<f32>) -> f32 {
    match value {
        Some(value) if value.is_finite() && value > 0.0 => value,
        _ => 1.0,
    }
}

/// Computes one root scale so the assembled skeleton, visuals, colliders, and
/// attachments share the same feet-pivot transform.
#[must_use]
pub fn resolve_actor_root_scale(
    kind: ActorKind,
    reference_scale: f32,
    race_scale: Option<f32>,
    base_scale: Option<f32>,
) -> f32 {
    let reference_scale = valid_scale_component(Some(reference_scale));
    let base_scale = valid_scale_component(base_scale);
    match kind {
        ActorKind::Humanoid => reference_scale * valid_scale_component(race_scale) * base_scale,
        ActorKind::Creature => reference_scale * base_scale,
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActorFallbackLevel {
    #[default]
    AuthoredExact,
    RaceSexSpecific,
    RaceDefault,
    GenericProjectBody,
    ProxyMesh,
}

impl ActorFallbackLevel {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::AuthoredExact => "AuthoredExact",
            Self::RaceSexSpecific => "RaceSexSpecific",
            Self::RaceDefault => "RaceDefault",
            Self::GenericProjectBody => "GenericProjectBody",
            Self::ProxyMesh => "ProxyMesh",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FaceGenAvailability {
    #[default]
    NotAuthored,
    Compatible,
    Incompatible,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FaceGenPolicy {
    #[default]
    NotAuthored,
    Authored,
    RestPoseFallback,
}

impl FaceGenPolicy {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotAuthored => "NotAuthored",
            Self::Authored => "Authored",
            Self::RestPoseFallback => "RestPoseFallback",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActorProxyKind {
    #[default]
    None,
    GenericHumanoid,
    Bounds,
}

impl ActorProxyKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::GenericHumanoid => "GenericHumanoid",
            Self::Bounds => "Bounds",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorFallbackReason {
    MissingFaceGen,
    MissingSkeleton { path: String },
    MissingBodyPart { index: u32, path: String },
    MissingHeadModel { path: String },
    MissingHairModel { path: String },
    MissingEyeModel { path: String },
    MissingCreatureModel { path: String },
    IncompatibleSkin { path: String },
    MissingEquipmentModel { item_form_id: u32, path: String },
}

impl ActorFallbackReason {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingFaceGen => "missing_facegen",
            Self::MissingSkeleton { .. } => "missing_skeleton",
            Self::MissingBodyPart { .. } => "missing_body_part",
            Self::MissingHeadModel { .. } => "missing_head_model",
            Self::MissingHairModel { .. } => "missing_hair_model",
            Self::MissingEyeModel { .. } => "missing_eye_model",
            Self::MissingCreatureModel { .. } => "missing_creature_model",
            Self::IncompatibleSkin { .. } => "incompatible_skin",
            Self::MissingEquipmentModel { .. } => "missing_equipment",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorAppearanceAvailability {
    pub kind: ActorKind,
    pub base_form_id: u32,
    pub reference_form_id: u32,
    pub exact_available: bool,
    pub race_sex_available: bool,
    pub race_default_available: bool,
    pub generic_available: bool,
    pub facegen: FaceGenAvailability,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorFallbackDecision {
    pub base_form_id: u32,
    pub reference_form_id: u32,
    pub level: ActorFallbackLevel,
    pub facegen_policy: FaceGenPolicy,
    pub proxy_kind: ActorProxyKind,
    pub reasons: Vec<ActorFallbackReason>,
}

#[must_use]
pub fn resolve_actor_fallback(
    availability: &ActorAppearanceAvailability,
    mut reasons: Vec<ActorFallbackReason>,
) -> ActorFallbackDecision {
    let facegen_policy = match availability.facegen {
        FaceGenAvailability::NotAuthored => FaceGenPolicy::NotAuthored,
        FaceGenAvailability::Compatible => FaceGenPolicy::Authored,
        FaceGenAvailability::Incompatible => {
            reasons.push(ActorFallbackReason::MissingFaceGen);
            FaceGenPolicy::RestPoseFallback
        }
    };

    let exact_usable =
        availability.exact_available && availability.facegen != FaceGenAvailability::Incompatible;
    let level = if exact_usable {
        ActorFallbackLevel::AuthoredExact
    } else if availability.race_sex_available {
        ActorFallbackLevel::RaceSexSpecific
    } else if availability.race_default_available {
        ActorFallbackLevel::RaceDefault
    } else if availability.generic_available {
        ActorFallbackLevel::GenericProjectBody
    } else {
        ActorFallbackLevel::ProxyMesh
    };
    let proxy_kind = match (level, availability.kind) {
        (ActorFallbackLevel::GenericProjectBody, ActorKind::Humanoid) => {
            ActorProxyKind::GenericHumanoid
        }
        (ActorFallbackLevel::GenericProjectBody | ActorFallbackLevel::ProxyMesh, _) => {
            ActorProxyKind::Bounds
        }
        _ => ActorProxyKind::None,
    };

    reasons.sort();
    reasons.dedup();
    ActorFallbackDecision {
        base_form_id: availability.base_form_id,
        reference_form_id: availability.reference_form_id,
        level,
        facegen_policy,
        proxy_kind,
        reasons,
    }
}

/// The complete, serialized identity and presentation decision for one placed
/// actor. Every downstream runtime component derives from this value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActorAssemblyBlueprint {
    pub source_base_form_id: u32,
    pub resolved_base_form_id: u32,
    pub reference_form_id: u32,
    pub kind: ActorKind,
    #[serde(default)]
    pub female: bool,
    #[serde(default)]
    pub race_form_id: Option<u32>,
    #[serde(default = "default_root_scale")]
    pub root_scale: f32,
    #[serde(default)]
    pub skeleton_path: Option<String>,
    #[serde(default)]
    pub mesh_parts: Vec<AssembledMeshPart>,
    #[serde(default)]
    pub apparel: Vec<AssembledApparel>,
    /// Selected EYES record identity and diffuse texture. Eye geometry remains
    /// the race head parts (indices 6/7); FO3 EYES records do not own a model.
    #[serde(default)]
    pub eye_form_id: Option<u32>,
    #[serde(default)]
    pub eye_texture_path: Option<String>,
    #[serde(default)]
    pub equipped_weapon: Option<AssembledWeapon>,
    #[serde(default)]
    pub fallback: ActorFallbackDecision,
}

fn default_root_scale() -> f32 {
    1.0
}

impl Default for ActorAssemblyBlueprint {
    fn default() -> Self {
        Self {
            source_base_form_id: 0,
            resolved_base_form_id: 0,
            reference_form_id: 0,
            kind: ActorKind::default(),
            female: false,
            race_form_id: None,
            root_scale: default_root_scale(),
            skeleton_path: None,
            mesh_parts: Vec::new(),
            apparel: Vec::new(),
            eye_form_id: None,
            eye_texture_path: None,
            equipped_weapon: None,
            fallback: ActorFallbackDecision::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_weapon_can_still_be_the_canonical_equipment_choice() {
        let selected = select_starting_weapon(&[
            ActorWeaponCandidate {
                item_form_id: 2,
                model_path: Some("pistol.nif".to_owned()),
                damage: 10,
                value: 100,
                available: true,
            },
            ActorWeaponCandidate {
                item_form_id: 1,
                model_path: Some("rifle.nif".to_owned()),
                damage: 20,
                value: 50,
                available: false,
            },
        ])
        .expect("a weapon is selected");

        assert_eq!(selected.item_form_id, 1);
        assert!(!selected.model_available);
    }

    #[test]
    fn fallback_keeps_identity_when_no_visual_assets_are_supported() {
        let decision = resolve_actor_fallback(
            &ActorAppearanceAvailability {
                kind: ActorKind::Creature,
                base_form_id: 0x10,
                reference_form_id: 0x20,
                ..Default::default()
            },
            Vec::new(),
        );

        assert_eq!(decision.level, ActorFallbackLevel::ProxyMesh);
        assert_eq!(decision.proxy_kind, ActorProxyKind::Bounds);
        assert_eq!(decision.base_form_id, 0x10);
        assert_eq!(decision.reference_form_id, 0x20);
    }

    #[test]
    fn blueprint_default_uses_neutral_root_scale() {
        assert_eq!(ActorAssemblyBlueprint::default().root_scale, 1.0);
    }

    #[test]
    fn canonicalization_retains_distinct_left_and_right_eye_meshes() {
        let eye = |name: &str, path: &str| AssembledMeshPart {
            name: name.to_owned(),
            source_form_id: Some(0x4253),
            model_path: path.to_owned(),
            attachment_point: ActorAttachmentPoint::Head,
            role: ActorMeshRole::Eyes,
            is_visible: true,
        };
        let mut parts = vec![
            eye("right eye", "characters/eyes/eyeright.nif"),
            eye("left eye", "characters/eyes/eyeleft.nif"),
            eye("left eye duplicate", "CHARACTERS/EYES/EYELEFT.NIF"),
        ];

        canonicalize_mesh_parts(&mut parts);

        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].model_path, "characters/eyes/eyeleft.nif");
        assert_eq!(parts[1].model_path, "characters/eyes/eyeright.nif");
    }
}
