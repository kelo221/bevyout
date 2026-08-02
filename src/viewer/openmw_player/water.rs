//! Pure water-contact and medium-transition policy.
//!
//! `resolve_water_contact` remains the core authority for authored water
//! geometry and surface-relative depth. These helpers classify that result
//! and compare it with the existing runtime submerged flag; they do not own a
//! second water resource. W4-C will apply the result to `ExteriorWaterState`
//! and `SwimmingState`.

use bevyout_core::manifest::exterior::{
    ExteriorWaterContact, PreparedWater, resolve_water_contact,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WaterPhase {
    Dry,
    Surface,
    Submerged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WaterTransition {
    None,
    Entered,
    Exited,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WaterPolicyResult {
    pub(crate) contact: Option<ExteriorWaterContact>,
    pub(crate) phase: WaterPhase,
    pub(crate) transition: WaterTransition,
}

/// Resolve authored contact and the water entry/exit event for this sample.
///
/// The previous state is deliberately only the existing submerged bit. A
/// surface contact is still dry for movement purposes, while a transition to
/// any submerged contact is an entry and a transition away from it is an
/// exit. Invalid authored/player heights follow the core resolver's `None`
/// result and therefore resolve as dry.
pub(crate) fn resolve_water_policy(
    was_submerged: bool,
    water: Option<&PreparedWater>,
    player_height: f32,
) -> WaterPolicyResult {
    let contact = resolve_water_contact(water, player_height);
    let phase = match contact {
        None => WaterPhase::Dry,
        Some(contact) if contact.submerged => WaterPhase::Submerged,
        Some(_) => WaterPhase::Surface,
    };
    let is_submerged = phase == WaterPhase::Submerged;
    let transition = match (was_submerged, is_submerged) {
        (false, true) => WaterTransition::Entered,
        (true, false) => WaterTransition::Exited,
        _ => WaterTransition::None,
    };
    WaterPolicyResult {
        contact,
        phase,
        transition,
    }
}
