//! Prepared Fallout 3 ImageSpace modifier (IMAD) contracts.
//!
//! These values are deliberately independent of Bevy.  The prepare slice
//! decodes ESM4 records into this shape and the viewer maps it into its pure
//! screen-feedback policy before touching camera/rendering types.

use serde::{Deserialize, Serialize};

/// Bump whenever the serialized modifier or catalog meaning changes,
/// including serde-defaulted fields.
pub const IMAGE_SPACE_MODIFIER_CATALOG_REVISION: &str = "openmw-imad-v3";

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImageSpaceModifierCurveOperation {
    #[default]
    Additive,
    Multiplier,
    /// A named IMAD Time/Color sequence supplies the complete value at the
    /// sampled time rather than an add/multiply delta.
    Set,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImageSpaceModifierProperty {
    #[default]
    Blur,
    DoubleVision,
    Brightness,
    Saturation,
    Contrast,
    Fade,
    RadialBlur,
    RadialCenterX,
    RadialCenterY,
    RadialRampUp,
    RadialStart,
    RadialRampDown,
    RadialDownStart,
    DepthOfFieldStrength,
    DepthOfFieldDistance,
    DepthOfFieldRange,
    MotionBlur,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ImageSpaceModifierValues {
    /// Cinematic brightness multiplier from DNAM.
    pub brightness: f32,
    /// Cinematic saturation multiplier from DNAM.
    pub saturation: f32,
    /// Cinematic contrast multiplier from DNAM.
    pub contrast: f32,
    pub blur: f32,
    pub double_vision: f32,
    pub tint_rgba: [f32; 4],
    pub fade: f32,
    pub radial_blur: f32,
    pub radial_center: [f32; 2],
    pub radial_ramp_up: f32,
    pub radial_start: f32,
    pub radial_ramp_down: f32,
    pub radial_down_start: f32,
    pub depth_of_field_strength: f32,
    pub depth_of_field_distance: f32,
    pub depth_of_field_range: f32,
    pub motion_blur: f32,
}

impl ImageSpaceModifierValues {
    pub const fn neutral() -> Self {
        Self {
            brightness: 1.0,
            saturation: 1.0,
            contrast: 1.0,
            blur: 0.0,
            double_vision: 0.0,
            tint_rgba: [0.0; 4],
            fade: 0.0,
            radial_blur: 0.0,
            radial_center: [0.5, 0.5],
            radial_ramp_up: 0.0,
            radial_start: 0.0,
            radial_ramp_down: 0.0,
            radial_down_start: 0.0,
            depth_of_field_strength: 0.0,
            depth_of_field_distance: 0.0,
            depth_of_field_range: 0.0,
            motion_blur: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ImageSpaceModifierKeyframe {
    pub time_ms: u32,
    pub value: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ImageSpaceModifierCurve {
    pub property: ImageSpaceModifierProperty,
    pub operation: ImageSpaceModifierCurveOperation,
    pub keyframes: Vec<ImageSpaceModifierKeyframe>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ImageSpaceModifierColorKeyframe {
    pub time_ms: u32,
    pub rgba: [f32; 4],
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ImageSpaceModifier {
    pub form_id: u32,
    pub editor_id: Option<String>,
    pub flags: u32,
    pub radial_blur_flags: u32,
    pub depth_of_field_flags: u32,
    pub duration_ms: u32,
    pub static_values: ImageSpaceModifierValues,
    pub curves: Vec<ImageSpaceModifierCurve>,
    #[serde(default)]
    pub color_keyframes: Vec<ImageSpaceModifierColorKeyframe>,
    #[serde(default)]
    pub fade_color_keyframes: Vec<ImageSpaceModifierColorKeyframe>,
    #[serde(default)]
    pub sound_form_ids: Vec<u32>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PreparedImageSpaceModifierCatalog {
    pub revision: String,
    pub source_fingerprint: String,
    pub modifiers: Vec<ImageSpaceModifier>,
}
