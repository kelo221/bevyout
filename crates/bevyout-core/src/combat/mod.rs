//! Engine-independent combat causality.

pub mod ammo;
pub mod condition;
pub mod rng;

pub use condition::{
    BASIS_POINT_DENOMINATOR, COMBAT_POLICY_REVISION, ConditionDecision, ConditionError,
    JamDecision, JamReason, MAX_JAM_CHANCE_BASIS_POINTS, MIN_DAMAGE_EFFECTIVENESS,
    WeaponConditionPolicy,
};
pub use rng::{
    COMBAT_RNG_REVISION, CombatRngDomain, CombatRngDraw, CombatRngError, CombatRngState,
};
