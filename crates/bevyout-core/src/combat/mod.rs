//! Engine-independent combat causality.

pub mod ammo;
pub mod body;
pub mod condition;
pub mod limbs;
pub mod medical;
pub mod rng;

pub use body::{ALL_BODY_PARTS, BodyPartId};
pub use condition::{
    BASIS_POINT_DENOMINATOR, COMBAT_POLICY_REVISION, ConditionDecision, ConditionError,
    JamDecision, JamReason, MAX_JAM_CHANCE_BASIS_POINTS, MIN_DAMAGE_EFFECTIVENESS,
    WeaponConditionPolicy,
};
pub use limbs::{
    HEAD_PERCEPTION_PENALTY, LIMB_MAX_MILLI, LimbCondition, LimbImpact, LimbImpactOutcome,
    LimbState, ShotId, apply_limb_impact,
};
pub use medical::{MedicalSource, RestorationOutcome, STIMPAK_RESTORE_MILLI, restore_limbs};
pub use rng::{
    COMBAT_RNG_REVISION, CombatRngDomain, CombatRngDraw, CombatRngError, CombatRngState,
};
