//! Deterministic actor perception and the single authoritative target
//! awareness state (issue #116).
//!
//! Pure, `std`/`serde`-only. The Bevy viewer feeds instantaneous geometry
//! ([`PerceptionInputs`]) computed from transforms, line-of-sight, and life
//! state; this module folds that into one [`AwarenessState`] per observer with
//! acquire/loss **hysteresis**. That state is the sole target authority handed
//! to AI package selection and the future combat boundary -- there is no
//! second, duplicate target. Weapon and damage state are deliberately absent
//! (no combat coupling).

use serde::{Deserialize, Serialize};

/// The class of thing an observer can perceive as a target. No weapon or
/// damage information is attached.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetClass {
    Player,
    Actor,
    Reference,
}

impl TargetClass {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Player => "player",
            Self::Actor => "actor",
            Self::Reference => "reference",
        }
    }

    /// Deterministic priority ordering when two candidates are otherwise
    /// equidistant (lower wins). The player is preferred over other actors.
    const fn priority(self) -> u8 {
        match self {
            Self::Player => 0,
            Self::Actor => 1,
            Self::Reference => 2,
        }
    }
}

/// A stable identity for a perceivable target.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetId {
    pub class: TargetClass,
    /// Stable ACHR/ACRE reference FormID, or `0` for the player.
    pub form_id: u32,
}

impl TargetId {
    #[must_use]
    pub const fn player() -> Self {
        Self {
            class: TargetClass::Player,
            form_id: 0,
        }
    }
}

/// One candidate target's instantaneous, viewer-computed geometry and life
/// state for this tick.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PerceptionInputs {
    pub target: TargetId,
    pub position: [f32; 3],
    /// Distance from observer eye to target, metres.
    pub distance: f32,
    /// Angle between observer facing and the direction to the target, radians.
    pub angle_to_target: f32,
    /// False when occluded (a wall or a *closed* door blocks the ray). A target
    /// that is present but occluded is still a candidate, and decays via the
    /// forget timer rather than being lost instantly.
    pub has_line_of_sight: bool,
    /// False for disabled, dead, or vanished targets. A non-detectable acquired
    /// target is lost immediately (distinct from mere occlusion).
    pub detectable: bool,
}

impl PerceptionInputs {
    #[must_use]
    pub fn is_visible(&self, config: &PerceptionConfig) -> bool {
        self.detectable
            && self.has_line_of_sight
            && self.distance.is_finite()
            && self.distance <= config.sight_range
            && self.angle_to_target <= config.view_cone_half_angle
    }
}

/// Tunable perception thresholds. `acquire_confidence > lose_confidence`
/// provides hysteresis so a target hovering at the detection boundary does not
/// flicker between acquired and lost.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PerceptionConfig {
    pub sight_range: f32,
    /// Half-angle of the forward view cone, radians.
    pub view_cone_half_angle: f32,
    /// Confidence at or above which an unacquired target is acquired.
    pub acquire_confidence: f32,
    /// Confidence at or below which an acquired target is lost.
    pub lose_confidence: f32,
    /// Confidence gained per second while the primary target is visible.
    pub gain_per_second: f32,
    /// Confidence lost per second while the primary target is not visible.
    pub decay_per_second: f32,
    /// Seconds an acquired-but-unseen target is retained before being forgotten
    /// regardless of residual confidence.
    pub forget_seconds: f32,
}

impl Default for PerceptionConfig {
    fn default() -> Self {
        Self {
            sight_range: 40.0,
            view_cone_half_angle: std::f32::consts::FRAC_PI_2, // 90 deg half-cone
            acquire_confidence: 0.75,
            lose_confidence: 0.1,
            gain_per_second: 2.0,
            decay_per_second: 1.0,
            forget_seconds: 5.0,
        }
    }
}

/// The observable result of one perception update.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AwarenessEvent {
    /// No target acquired this tick.
    Idle,
    /// A target crossed the acquire threshold this tick.
    Acquired(TargetId),
    /// The already-acquired target remains acquired.
    Retained(TargetId),
    /// The acquired target was lost this tick (forgotten, occluded past the
    /// timer, disposition-independent).
    Lost(TargetId),
}

/// The single authoritative awareness state for one observer. Serialized as
/// part of save/load so an in-progress acquisition survives a reload.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AwarenessState {
    /// Detection confidence in `[0, 1]`. Wave 6 also stores milli units; the
    /// f32 copy remains so existing saves and the float `update` path migrate.
    pub confidence: f32,
    /// Quantized confidence in `[0, 1000]`. Authoritative for stealth evidence.
    pub confidence_milli: u16,
    /// The one currently acquired target, or `None`.
    pub acquired: Option<TargetId>,
    /// Seconds since the acquired/primary target was last visible.
    pub time_since_seen: f32,
    /// Milliseconds since the acquired/primary target was last visible.
    pub time_since_seen_ms: u32,
    /// Last position the primary target was seen at, for search behavior.
    pub last_known_position: Option<[f32; 3]>,
}

impl AwarenessState {
    #[must_use]
    pub fn is_aware(&self) -> bool {
        self.acquired.is_some()
    }

    #[must_use]
    pub fn target(&self) -> Option<TargetId> {
        self.acquired
    }

    /// Clears all awareness. Called on cell unload or when the observer itself
    /// becomes dead/disabled.
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Advances the state by `dt` seconds given this tick's candidate targets.
    /// Returns the observable event. Deterministic for a fixed input set.
    pub fn update(
        &mut self,
        candidates: &[PerceptionInputs],
        config: &PerceptionConfig,
        dt: f32,
    ) -> AwarenessEvent {
        let dt = if dt.is_finite() && dt > 0.0 { dt } else { 0.0 };

        // While acquired, the acquired target is the primary; otherwise pick
        // the best visible candidate deterministically.
        let acquired_inputs = self
            .acquired
            .and_then(|id| candidates.iter().find(|c| c.target == id));
        let primary = if self.acquired.is_some() {
            acquired_inputs
        } else {
            select_best_visible(candidates, config)
        };
        let primary_visible = primary.is_some_and(|p| p.is_visible(config));

        if let Some(p) = primary.filter(|p| p.is_visible(config)) {
            self.confidence = (self.confidence + config.gain_per_second * dt).min(1.0);
            self.time_since_seen = 0.0;
            self.last_known_position = Some(p.position);
        } else {
            self.confidence = (self.confidence - config.decay_per_second * dt).max(0.0);
            self.time_since_seen += dt;
        }
        self.confidence_milli = if self.confidence.is_finite() {
            (self.confidence.clamp(0.0, 1.0) * 1_000.0).round() as u16
        } else {
            0
        };
        self.time_since_seen_ms = if self.time_since_seen.is_finite() && self.time_since_seen > 0.0
        {
            (self.time_since_seen * 1_000.0).round() as u32
        } else {
            0
        };

        match self.acquired {
            None => {
                if primary_visible && self.confidence >= config.acquire_confidence {
                    let id = primary.expect("primary is Some when visible").target;
                    self.acquired = Some(id);
                    AwarenessEvent::Acquired(id)
                } else {
                    AwarenessEvent::Idle
                }
            }
            Some(id) => {
                // Vanished (gone from the candidate set) or explicitly
                // undetectable (dead/disabled) targets are lost immediately.
                let vanished = acquired_inputs.is_none_or(|c| !c.detectable);
                if vanished
                    || self.confidence <= config.lose_confidence
                    || self.time_since_seen >= config.forget_seconds
                {
                    self.acquired = None;
                    AwarenessEvent::Lost(id)
                } else {
                    AwarenessEvent::Retained(id)
                }
            }
        }
    }
}

/// The best visible candidate: nearest first, then target-class priority, then
/// lowest FormID. Returns `None` when nothing is visible.
#[must_use]
pub fn select_best_visible<'a>(
    candidates: &'a [PerceptionInputs],
    config: &PerceptionConfig,
) -> Option<&'a PerceptionInputs> {
    candidates
        .iter()
        .filter(|c| c.is_visible(config))
        .min_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.target.class.priority().cmp(&b.target.class.priority()))
                .then_with(|| a.target.form_id.cmp(&b.target.form_id))
        })
}

#[cfg(test)]
#[path = "tests/perception.rs"]
mod tests;
