//! Engine-independent player-weapon actions and persisted actor damage.
//!
//! This module deliberately knows nothing about Bevy input, rendering,
//! raycasts, audio, or ammunition holders. Runtime adapters translate accepted
//! actions into those presentation effects.

use std::error::Error;
use std::fmt;

use crate::actor_state::{
    ActorDefinition, ActorInstanceState, ActorLifeState, ActorStateError, ActorValue,
};

pub const DEFAULT_FIRE_SECONDS: f32 = 0.12;
pub const DEFAULT_RELOAD_SECONDS: f32 = 1.5;
pub const DEFAULT_MUZZLE_FLASH_SECONDS: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeaponDefinition {
    pub damage: f32,
    pub range_meters: f32,
    pub fire_seconds: f32,
    pub reload_seconds: f32,
}

impl WeaponDefinition {
    #[must_use]
    pub const fn new(damage: f32, range_meters: f32) -> Self {
        Self {
            damage,
            range_meters,
            fire_seconds: DEFAULT_FIRE_SECONDS,
            reload_seconds: DEFAULT_RELOAD_SECONDS,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WeaponAction {
    #[default]
    Idle,
    Firing,
    Reloading,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FireDecision {
    Fired { shot_index: u64 },
    BlockedFiring,
    BlockedReloading,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReloadDecision {
    Started,
    BlockedFiring,
    AlreadyReloading,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeaponState {
    definition: WeaponDefinition,
    action: WeaponAction,
    action_elapsed: f32,
    shots_fired: u64,
}

impl WeaponState {
    #[must_use]
    pub const fn new(definition: WeaponDefinition) -> Self {
        Self {
            definition,
            action: WeaponAction::Idle,
            action_elapsed: 0.0,
            shots_fired: 0,
        }
    }

    #[must_use]
    pub const fn definition(&self) -> WeaponDefinition {
        self.definition
    }

    #[must_use]
    pub const fn action(&self) -> WeaponAction {
        self.action
    }

    #[must_use]
    pub const fn shots_fired(&self) -> u64 {
        self.shots_fired
    }

    #[must_use]
    pub fn action_progress(&self) -> f32 {
        let duration = match self.action {
            WeaponAction::Idle => return 0.0,
            WeaponAction::Firing => self.definition.fire_seconds,
            WeaponAction::Reloading => self.definition.reload_seconds,
        };
        if !duration.is_finite() || duration <= 0.0 {
            1.0
        } else {
            (self.action_elapsed / duration).clamp(0.0, 1.0)
        }
    }

    pub fn request_fire(&mut self) -> FireDecision {
        match self.action {
            WeaponAction::Idle => {
                self.action = WeaponAction::Firing;
                self.action_elapsed = 0.0;
                self.shots_fired = self.shots_fired.saturating_add(1);
                FireDecision::Fired {
                    shot_index: self.shots_fired,
                }
            }
            WeaponAction::Firing => FireDecision::BlockedFiring,
            WeaponAction::Reloading => FireDecision::BlockedReloading,
        }
    }

    pub fn request_reload(&mut self) -> ReloadDecision {
        match self.action {
            WeaponAction::Idle => {
                self.action = WeaponAction::Reloading;
                self.action_elapsed = 0.0;
                ReloadDecision::Started
            }
            WeaponAction::Firing => ReloadDecision::BlockedFiring,
            WeaponAction::Reloading => ReloadDecision::AlreadyReloading,
        }
    }

    pub fn advance(&mut self, delta_seconds: f32) {
        if self.action == WeaponAction::Idle || !delta_seconds.is_finite() || delta_seconds <= 0.0 {
            return;
        }
        self.action_elapsed += delta_seconds;
        let duration = match self.action {
            WeaponAction::Idle => 0.0,
            WeaponAction::Firing => self.definition.fire_seconds,
            WeaponAction::Reloading => self.definition.reload_seconds,
        };
        if self.action_elapsed >= duration.max(0.0) {
            self.action = WeaponAction::Idle;
            self.action_elapsed = 0.0;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageOutcome {
    pub previous_health: f32,
    pub applied_damage: f32,
    pub remaining_health: f32,
    pub killed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DamageError {
    InvalidAmount(f32),
    ActorState(ActorStateError),
}

impl fmt::Display for DamageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAmount(amount) => {
                write!(
                    formatter,
                    "weapon damage must be positive and finite, got {amount}"
                )
            }
            Self::ActorState(error) => write!(formatter, "updating actor damage state: {error}"),
        }
    }
}

impl Error for DamageError {}

/// Applies damage by changing the actor's canonical runtime health mutation.
///
/// Derived health is never snapshotted. Re-resolving the immutable definition
/// plus this mutation after save/load produces the same remaining health.
pub fn apply_actor_damage(
    definition: &ActorDefinition,
    state: &mut ActorInstanceState,
    amount: f32,
) -> Result<DamageOutcome, DamageError> {
    if !amount.is_finite() || amount <= 0.0 {
        return Err(DamageError::InvalidAmount(amount));
    }
    let resolved = definition.resolve_value(state, ActorValue::Health);
    let previous_health = resolved.effective.max(0.0);
    if state.life_state == ActorLifeState::Dead {
        return Ok(DamageOutcome {
            previous_health,
            applied_damage: 0.0,
            remaining_health: previous_health,
            killed: false,
        });
    }
    let applied_damage = amount.min(previous_health);
    let remaining_health = (previous_health - applied_damage).max(0.0);
    let authored_health = resolved.inherited_or_base + resolved.authored_modifier;
    state
        .set_value_mutation(ActorValue::Health, remaining_health - authored_health)
        .map_err(DamageError::ActorState)?;
    let killed = remaining_health <= f32::EPSILON;
    if killed {
        state.life_state = ActorLifeState::Dead;
    }
    Ok(DamageOutcome {
        previous_health,
        applied_damage,
        remaining_health,
        killed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reload_blocks_fire_until_advance_completes() {
        let mut state = WeaponState::new(WeaponDefinition::new(9.0, 100.0));
        assert_eq!(state.request_reload(), ReloadDecision::Started);
        assert_eq!(state.request_fire(), FireDecision::BlockedReloading);
        state.advance(DEFAULT_RELOAD_SECONDS);
        assert_eq!(state.request_fire(), FireDecision::Fired { shot_index: 1 });
    }

    #[test]
    fn damage_mutates_health_and_marks_lethal_state() {
        let mut definition = ActorDefinition {
            base_form_id: 1,
            reference_form_id: 2,
            ..Default::default()
        };
        definition.base_values.insert(ActorValue::Health, 8.0);
        let mut state = ActorInstanceState::new(2, ActorLifeState::Alive);

        let outcome = apply_actor_damage(&definition, &mut state, 9.0).unwrap();

        assert_eq!(outcome.remaining_health, 0.0);
        assert!(outcome.killed);
        assert_eq!(state.life_state, ActorLifeState::Dead);
        assert_eq!(
            definition
                .resolve_value(&state, ActorValue::Health)
                .effective,
            0.0
        );
    }
}
