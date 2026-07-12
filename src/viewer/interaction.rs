use std::collections::{HashMap, HashSet};

use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::prelude::*;

use crate::vsa::{PreparedDoor, PreparedInventoryEntry, PreparedPlacement, PreparedSemantic};

use super::audio::PlaySound;

pub(crate) const INTERACTION_DISTANCE_METERS: f32 = 3.0;
const NOTICE_SECONDS: f32 = 3.0;
const MAX_PARENT_DEPTH: usize = 64;

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
}

#[derive(Resource, Default)]
struct InteractionState {
    focused: Option<Entity>,
    open: HashSet<Entity>,
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
        .add_systems(Startup, spawn_interaction_ui)
        .add_systems(
            Update,
            (update_focused_placement, activate_focused_placement).chain(),
        )
        .add_systems(Update, update_interaction_notice);
}

fn spawn_interaction_ui(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        InteractionPromptText,
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
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<&PlacementRoot>,
    inventory: Res<PlayerInventory>,
    mut state: ResMut<InteractionState>,
    mut prompt: Query<&mut Text, With<InteractionPromptText>>,
) {
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

fn find_placement_root(
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

#[allow(clippy::too_many_arguments)]
fn activate_focused_placement(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    roots: Query<(&PlacementRoot, &GlobalTransform)>,
    mut inventory: ResMut<PlayerInventory>,
    mut state: ResMut<InteractionState>,
    mut notice: ResMut<InteractionNotice>,
    mut sounds: MessageWriter<PlaySound>,
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
            if opening {
                state.open.insert(entity);
                write_sound(&mut sounds, placement.audio.open_sound_form_id, position);
                notice.show(format!(
                    "{name}: {}",
                    inventory_summary(&placement.inventory)
                ));
            } else {
                state.open.remove(&entity);
                write_sound(&mut sounds, placement.audio.close_sound_form_id, position);
                notice.show(format!("Closed {name}"));
            }
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
            if opening {
                state.open.insert(entity);
                write_sound(&mut sounds, placement.audio.open_sound_form_id, position);
            } else {
                state.open.remove(&entity);
                write_sound(&mut sounds, placement.audio.close_sound_form_id, position);
            }
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
                    "; travel is not enabled"
                } else {
                    ""
                }
            );
        }
        PreparedSemantic::Activator => {
            write_sound(
                &mut sounds,
                placement.audio.activate_sound_form_id,
                position,
            );
            notice.show(format!("Activated {name}"));
            info!("activated {} ({:08x})", name, placement.reference_form_id);
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
mod tests {
    use super::*;

    #[test]
    fn inventory_accumulates_whole_stacks() {
        let mut inventory = PlayerInventory::default();
        inventory.add(0x1234, 3);
        inventory.add(0x1234, 2);
        assert_eq!(inventory.count(0x1234), 5);
        assert!(inventory.contains(0x1234));
    }

    #[test]
    fn locked_door_requires_its_key() {
        let door = PreparedDoor {
            lock_level: Some(50),
            key_form_id: Some(0x42),
            destination: None,
        };
        let mut inventory = PlayerInventory::default();
        assert!(door_is_locked(&door, &inventory));
        inventory.add(0x42, 1);
        assert!(!door_is_locked(&door, &inventory));
    }

    #[test]
    fn lock_without_a_key_remains_locked() {
        let door = PreparedDoor {
            lock_level: Some(1),
            key_form_id: None,
            destination: None,
        };
        assert!(door_is_locked(&door, &PlayerInventory::default()));
    }

    #[test]
    fn zero_lock_level_is_unlocked() {
        let door = PreparedDoor {
            lock_level: Some(0),
            key_form_id: Some(0x42),
            destination: None,
        };
        assert!(!door_is_locked(&door, &PlayerInventory::default()));
    }

    #[test]
    fn container_summary_is_bounded() {
        let entries = (0..10)
            .map(|index| PreparedInventoryEntry {
                base_form_id: index,
                count: 1,
                record_kind: "MISC".into(),
                editor_id: Some(format!("Item{index}")),
                display_name: None,
                leveled: false,
            })
            .collect::<Vec<_>>();
        let summary = inventory_summary(&entries);
        assert!(summary.contains("Item0 x1"));
        assert!(summary.contains("+2 more"));
        assert!(!summary.contains("Item8"));
    }
}
