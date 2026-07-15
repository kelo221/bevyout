use std::collections::{HashMap, HashSet};

use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::prelude::*;

use crate::app_state::{AppState, GameplayModal};
use crate::console::{ConsoleSessionStore, RefRegistry};
use crate::vsa::{PreparedDoor, PreparedInventoryEntry, PreparedPlacement, PreparedSemantic};

use super::animation::{self, ClipTransition};
use super::audio::PlaySound;

pub(crate) const INTERACTION_DISTANCE_METERS: f32 = 3.0;
const NOTICE_SECONDS: f32 = 3.0;
const FOCUS_RAYCAST_INTERVAL_SECONDS: f32 = 0.1;
const MAX_PARENT_DEPTH: usize = 64;

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
#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct DoorTravelRequested {
    pub(crate) destination_cell_form_id: u32,
    pub(crate) translation: Vec3,
    pub(crate) rotation_xyzw: [f32; 4],
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
struct PendingDoorTravel(Option<PendingTravel>);

struct PendingTravel {
    entity: Entity,
    remaining_seconds: f32,
    request: DoorTravelRequested,
}

/// F57.3: counts a pending travel's open-lead down every frame this set
/// runs (gated the same as door activation itself: `AppState::InGame` and
/// `GameplayModal::None`, so a modal opening mid-lead pauses the countdown
/// exactly like it pauses everything else in this chain) and writes
/// `DoorTravelRequested` once it elapses.
fn tick_pending_door_travel(
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

/// Wave-3 shipped amendment: scripted (console/BRP) door activation follows
/// the same Open-clip lead as the player's Enter activation — the door is
/// marked open, its clip plays, and the travel request is staged behind the
/// lead. Zero lead (no clip) writes the message this same frame, exactly the
/// wave-2 `activate` behavior. Returns the lead in milliseconds so the
/// console can report it.
pub(crate) fn scripted_door_travel(
    world: &mut World,
    entity: Entity,
    request: DoorTravelRequested,
) -> f32 {
    let open_clip_seconds = world
        .get::<animation::AnimatedPlacement>(entity)
        .and_then(|animated| animated.clip_seconds("Open"));
    let lead_seconds =
        animation::open_lead_seconds(open_clip_seconds, animation::OPEN_LEAD_CAP_SECONDS);
    world
        .get_resource_or_insert_with(InteractionState::default)
        .open
        .insert(entity);
    world.write_message(animation::PlayPlacementAnimation {
        root: entity,
        transition: ClipTransition::Opening,
        lead_ms: lead_seconds * 1000.0,
    });
    if lead_seconds <= 0.0 {
        world.write_message(request);
    } else {
        world
            .get_resource_or_insert_with(PendingDoorTravel::default)
            .0 = Some(PendingTravel {
            entity,
            remaining_seconds: lead_seconds,
            request,
        });
    }
    lead_seconds * 1000.0
}

/// Attach this component to the root that owns a prepared placement's scene.
/// Mesh-ray hits are walked through `ChildOf` ancestors until this root is found.
#[derive(Component, Clone, Debug)]
pub(crate) struct PlacementRoot {
    placement: PreparedPlacement,
}

impl PlacementRoot {
    pub(crate) fn new(placement: PreparedPlacement) -> Self {
        Self { placement }
    }

    pub(crate) fn uses_quick_ao(&self) -> bool {
        self.placement.ao_mode == "ao-quick-v1"
    }

    pub(crate) fn placement(&self) -> &PreparedPlacement {
        &self.placement
    }
}

#[derive(Resource, Default, Debug)]
pub(crate) struct PlayerInventory {
    counts: HashMap<u32, i32>,
}

impl PlayerInventory {
    pub(crate) fn count(&self, form_id: u32) -> i32 {
        self.counts.get(&form_id).copied().unwrap_or(0)
    }

    pub(crate) fn contains(&self, form_id: u32) -> bool {
        self.count(form_id) > 0
    }

    fn add(&mut self, form_id: u32, count: i32) {
        *self.counts.entry(form_id).or_default() += count.max(1);
    }

    /// Issue #60 (F60.4): the inventory as sorted `(base_form_id, count)`
    /// stacks, the shape the save format's player record wants.
    pub(crate) fn stacks(&self) -> Vec<(u32, i32)> {
        let mut stacks: Vec<(u32, i32)> = self
            .counts
            .iter()
            .filter(|&(_, &count)| count != 0)
            .map(|(&form_id, &count)| (form_id, count))
            .collect();
        stacks.sort_unstable_by_key(|(form_id, _)| *form_id);
        stacks
    }

    /// Issue #60 (F60.4): rebuilds the inventory from a loaded save's
    /// player record.
    pub(crate) fn from_stacks(stacks: impl IntoIterator<Item = (u32, i32)>) -> Self {
        Self {
            counts: stacks.into_iter().collect(),
        }
    }
}

/// `open` is pub(crate) for issues #60/#61: `world::persist` captures
/// door/container open state on the way out of a cell and re-inserts it on
/// apply. Everything else stays private to this module.
#[derive(Resource, Default)]
pub(crate) struct InteractionState {
    focused: Option<Entity>,
    pub(crate) open: HashSet<Entity>,
}

#[derive(Resource, Default)]
struct InteractionNotice {
    text: String,
    remaining_seconds: f32,
}

impl InteractionNotice {
    fn show(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.remaining_seconds = NOTICE_SECONDS;
    }
}

#[derive(Component)]
struct InteractionPromptText;

#[derive(Component)]
struct InteractionNoticeText;

pub(crate) fn install(app: &mut App) {
    app.init_resource::<PlayerInventory>()
        .init_resource::<InteractionState>()
        .init_resource::<InteractionNotice>()
        .init_resource::<PendingDoorTravel>()
        .add_message::<DoorTravelRequested>()
        .add_systems(Startup, spawn_interaction_ui)
        .add_systems(
            Update,
            (
                update_focused_placement,
                activate_focused_placement,
                tick_pending_door_travel,
            )
                .chain()
                .in_set(DoorActivationSet)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::None)),
        )
        .add_systems(
            Update,
            (update_interaction_notice, cleanup_removed_placements),
        );
}

fn spawn_interaction_ui(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        InteractionPromptText,
        super::console::GameUi,
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(58.0),
            margin: UiRect::left(Val::Px(-140.0)),
            width: Val::Px(280.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ZIndex(110),
    ));
    commands.spawn((
        Text::new(""),
        InteractionNoticeText,
        super::console::GameUi,
        TextColor(Color::srgb(1.0, 0.9, 0.5)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(63.0),
            margin: UiRect::left(Val::Px(-240.0)),
            width: Val::Px(480.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ZIndex(110),
    ));
}

#[allow(clippy::too_many_arguments)]
fn update_focused_placement(
    time: Res<Time>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<&PlacementRoot>,
    inventory: Res<PlayerInventory>,
    mut state: ResMut<InteractionState>,
    mut prompt: Query<&mut Text, With<InteractionPromptText>>,
    mut raycast_elapsed: Local<f32>,
) {
    *raycast_elapsed += time.delta_secs();
    if *raycast_elapsed < FOCUS_RAYCAST_INTERVAL_SECONDS {
        return;
    }
    *raycast_elapsed = 0.0;

    let focused = active_center_ray(&cameras).and_then(|ray| {
        let settings = MeshRayCastSettings {
            visibility: RayCastVisibility::VisibleInView,
            ..default()
        };
        let (hit_entity, hit) = raycast.cast_ray(ray, &settings).first()?;
        if hit.distance > INTERACTION_DISTANCE_METERS {
            return None;
        }
        let root_entity = find_placement_root(*hit_entity, &parents, &roots)?;
        let root = roots.get(root_entity).ok()?;
        interaction_prompt(
            &root.placement,
            state.open.contains(&root_entity),
            &inventory,
        )
        .map(|text| (root_entity, text))
    });

    state.focused = focused.as_ref().map(|(entity, _)| *entity);
    if let Ok(mut prompt) = prompt.single_mut() {
        prompt.0 = focused.map(|(_, text)| text).unwrap_or_default();
    }
}

fn active_center_ray(
    cameras: &Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Option<Ray3d> {
    cameras.iter().find_map(|(camera, transform)| {
        if !camera.is_active {
            return None;
        }
        let viewport = camera.logical_viewport_size()?;
        camera.viewport_to_world(transform, viewport * 0.5).ok()
    })
}

pub(crate) fn find_placement_root(
    mut entity: Entity,
    parents: &Query<&ChildOf>,
    roots: &Query<&PlacementRoot>,
) -> Option<Entity> {
    for _ in 0..MAX_PARENT_DEPTH {
        if roots.contains(entity) {
            return Some(entity);
        }
        entity = parents.get(entity).ok()?.parent();
    }
    warn!("placement hierarchy exceeded {MAX_PARENT_DEPTH} ancestors");
    None
}

fn cleanup_removed_placements(
    mut removed: RemovedComponents<PlacementRoot>,
    mut references: ResMut<RefRegistry>,
    mut sessions: ResMut<ConsoleSessionStore>,
) {
    for entity in removed.read() {
        references.unregister(entity);
        sessions.clear_entity(entity);
    }
}

#[allow(clippy::too_many_arguments)]
fn activate_focused_placement(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    roots: Query<(&PlacementRoot, &GlobalTransform)>,
    animated: Query<&animation::AnimatedPlacement>,
    mut inventory: ResMut<PlayerInventory>,
    mut state: ResMut<InteractionState>,
    mut notice: ResMut<InteractionNotice>,
    mut sounds: MessageWriter<PlaySound>,
    mut door_travel: MessageWriter<DoorTravelRequested>,
    mut animation_playback: MessageWriter<animation::PlayPlacementAnimation>,
    mut pending_travel: ResMut<PendingDoorTravel>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Some(entity) = state.focused else {
        return;
    };
    let Ok((root, transform)) = roots.get(entity) else {
        state.focused = None;
        return;
    };
    let placement = &root.placement;
    let position = transform.translation();
    let name = placement_name(placement);

    match &placement.semantic {
        PreparedSemantic::Pickup(_) => {
            let count = placement.count.max(1);
            inventory.add(placement.base_form_id, count);
            write_sound(&mut sounds, placement.audio.pickup_sound_form_id, position);
            notice.show(format!("Picked up {name} x{count}"));
            info!(
                "picked up {} x{} ({:08x}); inventory now has {}",
                name,
                count,
                placement.base_form_id,
                inventory.count(placement.base_form_id)
            );
            state.focused = None;
            state.open.remove(&entity);
            commands.entity(entity).despawn();
        }
        PreparedSemantic::Container => {
            let opening = !state.open.contains(&entity);
            let transition = if opening {
                state.open.insert(entity);
                write_sound(&mut sounds, placement.audio.open_sound_form_id, position);
                notice.show(format!(
                    "{name}: {}",
                    inventory_summary(&placement.inventory)
                ));
                ClipTransition::Opening
            } else {
                state.open.remove(&entity);
                write_sound(&mut sounds, placement.audio.close_sound_form_id, position);
                notice.show(format!("Closed {name}"));
                ClipTransition::Closing
            };
            animation_playback.write(animation::PlayPlacementAnimation {
                root: entity,
                transition,
                lead_ms: 0.0,
            });
            info!(
                "container {} ({:08x}) {} with {} fixed entries",
                name,
                placement.reference_form_id,
                if opening { "opened" } else { "closed" },
                placement.inventory.len()
            );
        }
        PreparedSemantic::Door(door) => {
            if door_is_locked(door, &inventory) {
                notice.show(format!("{name} is locked"));
                info!(
                    "door {} ({:08x}) is locked; key {:?}",
                    name, placement.reference_form_id, door.key_form_id
                );
                return;
            }
            let opening = !state.open.contains(&entity);
            let transition = if opening {
                state.open.insert(entity);
                write_sound(&mut sounds, placement.audio.open_sound_form_id, position);
                ClipTransition::Opening
            } else {
                state.open.remove(&entity);
                write_sound(&mut sounds, placement.audio.close_sound_form_id, position);
                // F57.4: closing before this door's own open-lead elapses
                // cancels the still-pending travel rather than letting a
                // stale swap fire after the player has already reversed
                // course.
                if pending_travel
                    .0
                    .as_ref()
                    .is_some_and(|pending| pending.entity == entity)
                {
                    pending_travel.0 = None;
                }
                ClipTransition::Closing
            };
            notice.show(format!(
                "{} {name}",
                if opening { "Opened" } else { "Closed" }
            ));
            info!(
                "door {} ({:08x}) {}{}",
                name,
                placement.reference_form_id,
                if opening { "opened" } else { "closed" },
                if door.destination.is_some() {
                    "; travel requested"
                } else {
                    ""
                }
            );
            // Issue #57: a travel door's Open clip gets a lead -- computed
            // from `AnimatedPlacement`'s discovered "Open" clip duration, if
            // any -- before `DoorTravelRequested` fires, so the door is
            // visibly open before the (already instant) cell swap. No clip
            // means zero lead: `world::swap` sees the message this same
            // frame, exactly like wave 2.
            let lead_seconds = if opening && door.destination.is_some() {
                let open_clip_seconds = animated
                    .get(entity)
                    .ok()
                    .and_then(|animated| animated.clip_seconds("Open"));
                animation::open_lead_seconds(open_clip_seconds, animation::OPEN_LEAD_CAP_SECONDS)
            } else {
                0.0
            };
            animation_playback.write(animation::PlayPlacementAnimation {
                root: entity,
                transition,
                lead_ms: lead_seconds * 1000.0,
            });
            // Issue #52: entering (opening) a door with a resolved
            // destination requests a cell swap; `world::swap` decides
            // instant vs. loading-screen fallback from cell residency.
            if opening && let Some(destination) = &door.destination {
                let request = DoorTravelRequested {
                    destination_cell_form_id: destination.cell_form_id,
                    translation: Vec3::from_array(destination.translation),
                    rotation_xyzw: destination.rotation_xyzw,
                };
                if lead_seconds <= 0.0 {
                    door_travel.write(request);
                } else {
                    pending_travel.0 = Some(PendingTravel {
                        entity,
                        remaining_seconds: lead_seconds,
                        request,
                    });
                }
            }
        }
        PreparedSemantic::Activator => {
            write_sound(
                &mut sounds,
                placement.audio.activate_sound_form_id,
                position,
            );
            notice.show(format!("Activated {name}"));
            info!("activated {} ({:08x})", name, placement.reference_form_id);
            animation_playback.write(animation::PlayPlacementAnimation {
                root: entity,
                transition: ClipTransition::Opening,
                lead_ms: 0.0,
            });
        }
        _ => {}
    }
}

fn write_sound(sounds: &mut MessageWriter<PlaySound>, form_id: Option<u32>, position: Vec3) {
    if let Some(form_id) = form_id {
        sounds.write(PlaySound::at(form_id, position));
    }
}

fn update_interaction_notice(
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

fn interaction_prompt(
    placement: &PreparedPlacement,
    is_open: bool,
    inventory: &PlayerInventory,
) -> Option<String> {
    let name = placement_name(placement);
    match &placement.semantic {
        PreparedSemantic::Pickup(_) => Some(format!(
            "[Enter] Take {name}{}",
            if placement.count > 1 {
                format!(" x{}", placement.count)
            } else {
                String::new()
            }
        )),
        PreparedSemantic::Container => Some(format!(
            "[Enter] {} {name}",
            if is_open { "Close" } else { "Open" }
        )),
        PreparedSemantic::Door(door) => {
            if door_is_locked(door, inventory) {
                Some(format!("[Enter] {name} (Locked)"))
            } else {
                Some(format!(
                    "[Enter] {} {name}",
                    if is_open { "Close" } else { "Open" }
                ))
            }
        }
        PreparedSemantic::Activator => Some(format!("[Enter] Activate {name}")),
        _ => None,
    }
}

fn placement_name(placement: &PreparedPlacement) -> String {
    placement
        .display_name
        .as_deref()
        .or(placement.editor_id.as_deref())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{:08x}", placement.base_form_id))
}

fn door_is_locked(door: &PreparedDoor, inventory: &PlayerInventory) -> bool {
    if door.lock_level.is_none_or(|level| level <= 0) {
        return false;
    }
    door.key_form_id
        .is_none_or(|key_form_id| !inventory.contains(key_form_id))
}

fn inventory_summary(entries: &[PreparedInventoryEntry]) -> String {
    if entries.is_empty() {
        return "empty".into();
    }
    const DISPLAY_LIMIT: usize = 8;
    let mut summary = entries
        .iter()
        .take(DISPLAY_LIMIT)
        .map(|entry| {
            let name = entry
                .display_name
                .as_deref()
                .or(entry.editor_id.as_deref())
                .map(str::to_owned)
                .unwrap_or_else(|| format!("{:08x}", entry.base_form_id));
            format!("{name} x{}", entry.count)
        })
        .collect::<Vec<_>>()
        .join(", ");
    if entries.len() > DISPLAY_LIMIT {
        summary.push_str(&format!(", +{} more", entries.len() - DISPLAY_LIMIT));
    }
    summary
}

#[cfg(test)]
#[path = "interaction/tests/mod.rs"]
mod tests;
