//! Interaction HUD presentation and stable player-facing labels.

use super::*;

pub(super) fn update_interaction_notice(
    time: Res<Time>,
    mut notice: ResMut<InteractionNotice>,
    mut text: Query<&mut Text, With<InteractionNoticeText>>,
) {
    if notice.remaining_seconds > 0.0 {
        notice.remaining_seconds = (notice.remaining_seconds - time.delta_secs()).max(0.0);
        if notice.remaining_seconds == 0.0 {
            notice.text.clear();
        }
    }
    if let Ok(mut text) = text.single_mut() {
        text.0.clone_from(&notice.text);
    }
}

pub(super) fn interaction_prompt(
    placement: &PreparedPlacement,
    is_open: bool,
    inventory: &PlayerInventory,
    has_dialogue: bool,
) -> Option<String> {
    let name = placement_name(placement);
    match &placement.semantic {
        PreparedSemantic::Pickup(_) => Some(format!(
            "[E] Take {name}{}",
            if placement.count > 1 {
                format!(" x{}", placement.count)
            } else {
                String::new()
            }
        )),
        PreparedSemantic::Container => Some(format!(
            "[E] {} {name}",
            if is_open { "Close" } else { "Open" }
        )),
        PreparedSemantic::Corpse => Some(format!(
            "[E] {} {name}",
            if is_open { "Close" } else { "Loot" }
        )),
        PreparedSemantic::Door(door) => {
            if door_is_locked(door, inventory) {
                Some(format!("[E] {name} (Locked)"))
            } else {
                Some(format!(
                    "[E] {} {name}",
                    if is_open { "Close" } else { "Open" }
                ))
            }
        }
        PreparedSemantic::Activator => Some(format!("[E] Activate {name}")),
        PreparedSemantic::Npc(_) if has_dialogue => Some(format!("[E] Talk to {name}")),
        _ => None,
    }
}

pub(super) fn placement_name(placement: &PreparedPlacement) -> String {
    placement
        .display_name
        .as_deref()
        .or(placement.editor_id.as_deref())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{:08x}", placement.base_form_id))
}

/// Whether `door` is currently locked and the player cannot bypass it with
/// a held key. `pub(crate)` (not `fn`) so `viewer::nav` can reuse this exact
/// check for excluding blocked door links from route planning (issue #113,
/// M4 wave 4 feature 4) rather than re-deriving its own copy.
pub(crate) fn door_is_locked(door: &PreparedDoor, inventory: &PlayerInventory) -> bool {
    if door.lock_level.is_none_or(|level| level <= 0) {
        return false;
    }
    door.key_form_id
        .is_none_or(|key_form_id| !inventory.contains(key_form_id))
}

fn entry_display_name(entry: &PreparedInventoryEntry) -> String {
    entry
        .display_name
        .as_deref()
        .or(entry.editor_id.as_deref())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{:08x}", entry.base_form_id))
}

pub(super) fn inventory_summary(entries: &[PreparedInventoryEntry]) -> String {
    if entries.is_empty() {
        return "empty".into();
    }
    const DISPLAY_LIMIT: usize = 8;
    let mut summary = entries
        .iter()
        .take(DISPLAY_LIMIT)
        .map(|entry| format!("{} x{}", entry_display_name(entry), entry.count))
        .collect::<Vec<_>>()
        .join(", ");
    if entries.len() > DISPLAY_LIMIT {
        summary.push_str(&format!(", +{} more", entries.len() - DISPLAY_LIMIT));
    }
    summary
}

/// F75.2: best-effort `base_form_id -> name` lookup for the transfer modal,
/// built from a container's fixed (non-leveled) prepared inventory entries
/// at open time. Leveled-resolved items and anything already in the
/// player's inventory from elsewhere have no name source here -- the
/// transfer UI falls back to the hex form id for those, matching this
/// issue's "name + count is enough" scope.
pub(super) fn container_item_names(entries: &[PreparedInventoryEntry]) -> HashMap<u32, String> {
    entries
        .iter()
        .map(|entry| (entry.base_form_id, entry_display_name(entry)))
        .collect()
}
