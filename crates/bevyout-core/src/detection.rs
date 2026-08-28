//! Quantized stealth evidence folded into the existing awareness authority.
//!
//! The viewer may compute geometry in floats, but it must quantize once before
//! this module. Hidden/Caution/Danger is a HUD projection, not a second
//! stealth authority.

use serde::{Deserialize, Serialize};

use crate::perception::{
    AwarenessEvent, AwarenessState, PerceptionConfig, TargetId, select_best_visible,
};

/// Basis points (`0..=10_000`) used by light and noise terms.
pub const DETECTION_BPS_MAX: u16 = 10_000;
/// One metre in millimetres.
pub const MM_PER_METRE: u32 = 1_000;

/// Viewer-quantized, integer detection evidence for one observer/subject pair.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetectionEvidence {
    pub observer: TargetId,
    pub subject: TargetId,
    pub distance_mm: u32,
    pub angle_millidegrees: u32,
    pub light_bps: u16,
    pub movement_noise_bps: u16,
    pub armor_noise_bps: u16,
    pub observer_perception: u16,
    pub has_line_of_sight: bool,
    pub delta_ms: u32,
}

/// Integer perception thresholds. Gain/decay are milli-confidence per second.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetectionConfig {
    pub sight_range_mm: u32,
    pub view_cone_half_angle_millidegrees: u32,
    pub acquire_confidence_milli: u16,
    pub lose_confidence_milli: u16,
    pub gain_per_second_milli: u16,
    pub decay_per_second_milli: u16,
    pub forget_ms: u32,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            sight_range_mm: 40_000,
            view_cone_half_angle_millidegrees: 90_000,
            acquire_confidence_milli: 750,
            lose_confidence_milli: 100,
            gain_per_second_milli: 2_000,
            decay_per_second_milli: 1_000,
            forget_ms: 5_000,
        }
    }
}

impl From<PerceptionConfig> for DetectionConfig {
    fn from(config: PerceptionConfig) -> Self {
        Self {
            sight_range_mm: (config.sight_range.max(0.0) * MM_PER_METRE as f32).round() as u32,
            view_cone_half_angle_millidegrees: (config.view_cone_half_angle.abs() * 180_000.0
                / std::f32::consts::PI)
                .round() as u32,
            acquire_confidence_milli: crate::detection::confidence_to_milli(
                config.acquire_confidence,
            ),
            lose_confidence_milli: crate::detection::confidence_to_milli(config.lose_confidence),
            gain_per_second_milli: (config.gain_per_second.max(0.0) * 1_000.0).round() as u16,
            decay_per_second_milli: (config.decay_per_second.max(0.0) * 1_000.0).round() as u16,
            forget_ms: (config.forget_seconds.max(0.0) * 1_000.0).round() as u32,
        }
    }
}

impl DetectionConfig {
    /// Fast golden-vector config: 400 ms of a maxed factor does not acquire;
    /// 800 ms does.
    #[must_use]
    pub fn golden() -> Self {
        Self {
            gain_per_second_milli: 1_000,
            decay_per_second_milli: 1_000,
            acquire_confidence_milli: 750,
            lose_confidence_milli: 100,
            forget_ms: 2_000,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionError {
    NonFiniteGeometry,
}

/// HUD projection reconstructed from observer awareness + hostility.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetectionHud {
    #[default]
    Hidden,
    Caution,
    Danger,
}

impl DetectionHud {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::Caution => "caution",
            Self::Danger => "danger",
        }
    }
}

/// One observer's contribution to the HUD projection.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObserverHudInput {
    pub hostile: bool,
    pub acquired_player: bool,
    pub confidence_milli: u16,
}

#[must_use]
pub fn project_detection_hud(observers: &[ObserverHudInput]) -> DetectionHud {
    if observers
        .iter()
        .any(|observer| observer.hostile && observer.acquired_player)
    {
        return DetectionHud::Danger;
    }
    if observers
        .iter()
        .any(|observer| observer.hostile && observer.confidence_milli > 0)
    {
        return DetectionHud::Caution;
    }
    DetectionHud::Hidden
}

/// Quantize viewer metres/radians. NaN and infinity are rejected.
pub fn quantize_geometry(distance_m: f32, angle_rad: f32) -> Result<(u32, u32), DetectionError> {
    if !distance_m.is_finite() || !angle_rad.is_finite() {
        return Err(DetectionError::NonFiniteGeometry);
    }
    let distance_mm = (distance_m.max(0.0) * MM_PER_METRE as f32).round() as u32;
    let millidegrees = (angle_rad.abs() * 180_000.0 / std::f32::consts::PI).round() as u32;
    Ok((distance_mm, millidegrees))
}

/// Gameplay light from prepared cell ambient RGB. Non-finite channels reject.
pub fn gameplay_light_bps(ambient_rgba: [f32; 4]) -> Result<u16, DetectionError> {
    let [r, g, b, _] = ambient_rgba;
    if !r.is_finite() || !g.is_finite() || !b.is_finite() {
        return Err(DetectionError::NonFiniteGeometry);
    }
    let luma = (0.2126 * r.max(0.0) + 0.7152 * g.max(0.0) + 0.0722 * b.max(0.0)).min(1.0);
    Ok((luma * f32::from(DETECTION_BPS_MAX)).round() as u16)
}

/// Convert legacy `[0, 1]` confidence into milli units.
#[must_use]
pub fn confidence_to_milli(confidence: f32) -> u16 {
    if !confidence.is_finite() {
        return 0;
    }
    (confidence.clamp(0.0, 1.0) * 1_000.0).round() as u16
}

/// Copy a legacy f32 awareness snapshot into milli fields when they are empty.
pub fn migrate_legacy_awareness(state: &mut AwarenessState) {
    if state.confidence_milli == 0 && state.confidence > 0.0 {
        state.confidence_milli = confidence_to_milli(state.confidence);
    }
    if state.time_since_seen_ms == 0
        && state.time_since_seen > 0.0
        && state.time_since_seen.is_finite()
    {
        state.time_since_seen_ms = (state.time_since_seen * 1_000.0).round() as u32;
    }
}

#[must_use]
pub fn evidence_is_visible(evidence: &DetectionEvidence, config: &DetectionConfig) -> bool {
    evidence.has_line_of_sight
        && evidence.distance_mm <= config.sight_range_mm
        && evidence.angle_millidegrees <= config.view_cone_half_angle_millidegrees
}

/// Combined detection score in basis points. Any one maxed term reaches 10_000.
#[must_use]
pub fn detection_score_bps(evidence: &DetectionEvidence) -> u32 {
    if !evidence.has_line_of_sight {
        return 0;
    }
    let perception_bps = u32::from(evidence.observer_perception).saturating_mul(1_000);
    u32::from(evidence.light_bps)
        .saturating_add(u32::from(evidence.movement_noise_bps))
        .saturating_add(u32::from(evidence.armor_noise_bps))
        .saturating_add(perception_bps)
        .min(u32::from(DETECTION_BPS_MAX))
}

fn apply_score(
    state: &mut AwarenessState,
    score_bps: u32,
    delta_ms: u32,
    config: &DetectionConfig,
) {
    if score_bps == 0 {
        let decay = u32::from(config.decay_per_second_milli).saturating_mul(delta_ms) / 1_000;
        state.confidence_milli =
            u16::try_from(u32::from(state.confidence_milli).saturating_sub(decay)).unwrap_or(0);
        state.time_since_seen_ms = state.time_since_seen_ms.saturating_add(delta_ms);
    } else {
        let gain = u32::from(config.gain_per_second_milli)
            .saturating_mul(delta_ms)
            .saturating_mul(score_bps)
            / (1_000 * u32::from(DETECTION_BPS_MAX));
        state.confidence_milli = u16::try_from(
            u32::from(state.confidence_milli)
                .saturating_add(gain)
                .min(1_000),
        )
        .unwrap_or(1_000);
        state.time_since_seen_ms = 0;
    }
    state.confidence = f32::from(state.confidence_milli) / 1_000.0;
    state.time_since_seen = state.time_since_seen_ms as f32 / 1_000.0;
}

fn as_perception_inputs(evidence: &DetectionEvidence) -> crate::perception::PerceptionInputs {
    crate::perception::PerceptionInputs {
        target: evidence.subject,
        position: [evidence.distance_mm as f32 / MM_PER_METRE as f32, 0.0, 0.0],
        distance: evidence.distance_mm as f32 / MM_PER_METRE as f32,
        angle_to_target: evidence.angle_millidegrees as f32 * std::f32::consts::PI / 180_000.0,
        has_line_of_sight: evidence.has_line_of_sight,
        detectable: true,
    }
}

/// Advances the existing [`AwarenessState`] from quantized evidence.
pub fn update_from_evidence(
    state: &mut AwarenessState,
    evidence: &[DetectionEvidence],
    config: &DetectionConfig,
) -> AwarenessEvent {
    migrate_legacy_awareness(state);
    let float_config = PerceptionConfig {
        sight_range: config.sight_range_mm as f32 / MM_PER_METRE as f32,
        view_cone_half_angle: config.view_cone_half_angle_millidegrees as f32
            * std::f32::consts::PI
            / 180_000.0,
        acquire_confidence: f32::from(config.acquire_confidence_milli) / 1_000.0,
        lose_confidence: f32::from(config.lose_confidence_milli) / 1_000.0,
        gain_per_second: f32::from(config.gain_per_second_milli) / 1_000.0,
        decay_per_second: f32::from(config.decay_per_second_milli) / 1_000.0,
        forget_seconds: config.forget_ms as f32 / 1_000.0,
    };
    let inputs: Vec<_> = evidence.iter().map(as_perception_inputs).collect();
    let acquired_evidence = state
        .acquired
        .and_then(|id| evidence.iter().find(|e| e.subject == id));
    let primary = if state.acquired.is_some() {
        acquired_evidence
    } else {
        select_best_visible(&inputs, &float_config)
            .and_then(|chosen| evidence.iter().find(|e| e.subject == chosen.target))
    };
    let visible = primary.is_some_and(|e| evidence_is_visible(e, config));
    let score = if visible {
        detection_score_bps(primary.expect("visible implies primary"))
    } else {
        0
    };
    let delta_ms = evidence.iter().map(|e| e.delta_ms).max().unwrap_or(0);
    apply_score(state, score, delta_ms, config);
    if let Some(p) = primary.filter(|e| evidence_is_visible(e, config) && score > 0) {
        state.last_known_position = Some([p.distance_mm as f32 / MM_PER_METRE as f32, 0.0, 0.0]);
    }

    match state.acquired {
        None => {
            if visible && score > 0 && state.confidence_milli >= config.acquire_confidence_milli {
                let id = primary.expect("visible implies primary").subject;
                state.acquired = Some(id);
                AwarenessEvent::Acquired(id)
            } else {
                AwarenessEvent::Idle
            }
        }
        Some(id) => {
            let vanished = acquired_evidence.is_none();
            if vanished
                || state.confidence_milli <= config.lose_confidence_milli
                || state.time_since_seen_ms >= config.forget_ms
            {
                state.acquired = None;
                AwarenessEvent::Lost(id)
            } else {
                AwarenessEvent::Retained(id)
            }
        }
    }
}

#[cfg(test)]
#[path = "tests/detection.rs"]
mod tests;
