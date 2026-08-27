//! Stable semantic body parts. Core never sees mesh paths or Bevy entities.

use serde::{Deserialize, Serialize};

/// The six anatomical pools Fallout 3 exposes to combat, Pip-Boy, and aid.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BodyPartId {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

pub const ALL_BODY_PARTS: [BodyPartId; 6] = [
    BodyPartId::Head,
    BodyPartId::Torso,
    BodyPartId::LeftArm,
    BodyPartId::RightArm,
    BodyPartId::LeftLeg,
    BodyPartId::RightLeg,
];

impl BodyPartId {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Torso => "torso",
            Self::LeftArm => "left_arm",
            Self::RightArm => "right_arm",
            Self::LeftLeg => "left_leg",
            Self::RightLeg => "right_leg",
        }
    }

    /// Parses a Gherkin or console label such as `"left arm"` or `"left_arm"`.
    #[must_use]
    pub fn parse(label: &str) -> Option<Self> {
        let normalized = label.trim().to_ascii_lowercase().replace(' ', "_");
        ALL_BODY_PARTS
            .into_iter()
            .find(|part| part.label() == normalized)
    }

    /// Maps a viewer/prepare node name onto a body part. Unknown or unmarked
    /// names fall back to torso; core never receives a mesh path.
    #[must_use]
    pub fn from_node_name(name: &str) -> Self {
        let normalized = name.to_ascii_lowercase();
        let tokens: Vec<&str> = normalized
            .split(|ch: char| !ch.is_ascii_alphanumeric())
            .filter(|token| !token.is_empty())
            .collect();
        let joined = tokens.join("");
        if contains_any(&joined, &["head", "skull", "brain", "face", "neck"]) {
            return Self::Head;
        }
        let side = side_from_tokens(&tokens);
        if is_arm(&joined) {
            return match side {
                Some('l') => Self::LeftArm,
                Some('r') => Self::RightArm,
                _ => Self::Torso,
            };
        }
        if is_leg(&joined) {
            return match side {
                Some('l') => Self::LeftLeg,
                Some('r') => Self::RightLeg,
                _ => Self::Torso,
            };
        }
        Self::Torso
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn is_arm(joined: &str) -> bool {
    contains_any(
        joined,
        &["arm", "hand", "finger", "clavicle", "shoulder", "wrist"],
    )
}

fn is_leg(joined: &str) -> bool {
    contains_any(
        joined,
        &[
            "leg", "thigh", "calf", "foot", "toe", "knee", "ankle", "shin",
        ],
    )
}

fn side_from_tokens(tokens: &[&str]) -> Option<char> {
    for token in tokens {
        if *token == "l" || *token == "left" || token.starts_with("left") {
            return Some('l');
        }
        if *token == "r" || *token == "right" || token.starts_with("right") {
            return Some('r');
        }
        if token.starts_with('l') && token.len() > 1 && is_limb_suffix(&token[1..]) {
            return Some('l');
        }
        if token.starts_with('r') && token.len() > 1 && is_limb_suffix(&token[1..]) {
            return Some('r');
        }
    }
    None
}

fn is_limb_suffix(suffix: &str) -> bool {
    is_arm(suffix) || is_leg(suffix)
}
