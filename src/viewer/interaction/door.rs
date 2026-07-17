//! Door activation ordering and delayed travel lifecycle.

use super::*;

/// Issue #52: written when the player opens a door whose `destination` is
/// `Some`, and consumed the same frame by `world::swap`'s eligibility system
/// (ordered `.after(DoorActivationSet)`) to drive either an instant cell
/// swap or a loading-screen fallback. Translation/rotation are already in
/// Bevy coordinates (converted at prepare time), matching
/// `PreparedDoorDestination`.
///
/// Issue #57: `activate_focused_placement` no longer always writes this
/// directly. A door with an `Open` clip stages it in `PendingDoorTravel`
/// instead, and `tick_pending_door_travel` (also `.in_set(DoorActivationSet)`,
/// chained right after) writes it once the open-lead elapses -- possibly
/// several frames later, but always from a system inside this set, so
/// `world::swap`'s same-frame contract holds on the frame the lead expires.
/// A door with no clip (zero lead) still writes it the same frame it
/// activates, bit-for-bit wave-2's behavior.
///
/// Issue #134: `door_form_id` is the *origin* door reference the player
/// activated (`PreparedPlacement::reference_form_id`) -- `world::swap`
/// threads it through to `nav::agent::note_player_swap_door` so the
/// intercell agent ledger can decide follow-through vs. freeze for any
/// live nav agent still in the departing cell.
#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct DoorTravelRequested {
    pub(crate) destination_cell_form_id: u32,
    pub(crate) translation: Vec3,
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) door_form_id: u32,
}

/// Ordering handle for `world::swap`'s door-travel systems: message readers
/// scheduled `.after(DoorActivationSet)` see `DoorTravelRequested` messages
/// written this same frame (Bevy's message double-buffering swaps once per
/// frame in `First`, not between systems), so the eligibility check and any
/// instant swap complete in the same frame as the door activation itself.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DoorActivationSet;

/// F57.3: a travel door's `DoorTravelRequested` write staged behind its
/// `Open` clip's lead (`animation::open_lead_seconds`). Only one travel can
/// be pending at a time -- same constraint `world::swap`'s
/// `PendingInstantSwap`/`PendingFallbackSwap` already enforce for the
/// message itself.
#[derive(Resource, Default)]
pub(super) struct PendingDoorTravel(pub(super) Option<PendingTravel>);

pub(super) struct PendingTravel {
    pub(super) entity: Entity,
    pub(super) remaining_seconds: f32,
    pub(super) request: DoorTravelRequested,
}

/// F57.3: counts a pending travel's open-lead down every frame this set
/// runs (gated the same as door activation itself: `AppState::InGame` and
/// `GameplayModal::None`, so a modal opening mid-lead pauses the countdown
/// exactly like it pauses everything else in this chain) and writes
/// `DoorTravelRequested` once it elapses.
pub(super) fn tick_pending_door_travel(
    time: Res<Time>,
    mut pending: ResMut<PendingDoorTravel>,
    mut door_travel: MessageWriter<DoorTravelRequested>,
) {
    let Some(travel) = pending.0.as_mut() else {
        return;
    };
    travel.remaining_seconds -= time.delta_secs();
    if travel.remaining_seconds <= 0.0 {
        let request = travel.request;
        pending.0 = None;
        door_travel.write(request);
    }
}
