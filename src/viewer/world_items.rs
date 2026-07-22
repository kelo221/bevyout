//! Player-created dropped world items (M3 wave 1, issue #72).

use std::collections::HashMap;

use bevy::prelude::*;
use bevy_boxddd::boxddd::{self, QueryFilter};
use bevy_boxddd::prelude::BoxdddPhysicsContext;

use crate::app_state::{AppState, GameplayModal};
use crate::vsa::{
    PreparedDropCollider, PreparedItemCatalog, PreparedItemDefinition,
    PreparedPhysicsClassification, PreparedPickup, PreparedPlacement, PreparedRuntimeMutability,
    PreparedSemantic,
};

use super::audio::PlaySound;
use super::interaction::{
    CanonicalItemLedger, InteractionNotice, PlacementRoot, PlayerEquipment, PlayerInventory,
    item_rules,
};
use super::inventory::{InventoryStack, TransferResult};
use super::pipboy::DropInventoryStackRequested;
use super::player::{CAPSULE_HEIGHT, FpsPlayer};
use super::world::{ActiveCell, ResidentCells};

mod drop_policy;

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct RuntimeWorldItem {
    pub(crate) runtime_id: u64,
    pub(crate) stack: InventoryStack,
    pub(crate) cell_form_id: u32,
}

#[derive(Component, Clone)]
pub(crate) struct PendingRuntimeCollider {
    pub(crate) placement: PreparedPlacement,
    pub(crate) drop_collider: PreparedDropCollider,
    pub(crate) restore: Option<super::world::DynamicBodyRestore>,
}

#[derive(Clone)]
struct ItemWorldAsset {
    placement: PreparedPlacement,
    drop_collider: PreparedDropCollider,
    quest_item: bool,
}

#[derive(Resource, Default)]
struct ItemWorldAssets(HashMap<u32, ItemWorldAsset>);

#[derive(Resource, Debug)]
pub(crate) struct NextRuntimeItemId(pub(crate) u64);

impl Default for NextRuntimeItemId {
    fn default() -> Self {
        Self(1)
    }
}

pub(crate) struct WorldItemsPlugin;

impl Plugin for WorldItemsPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    app.init_resource::<ItemWorldAssets>()
        .init_resource::<NextRuntimeItemId>()
        .add_systems(OnEnter(AppState::InGame), collect_item_world_assets)
        .add_systems(
            Update,
            drop_inventory_items
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::PipBoy)),
        )
        .add_systems(Update, super::player::attach_runtime_item_colliders);
}

fn collect_item_world_assets(
    catalog: Res<PreparedItemCatalog>,
    mut assets: ResMut<ItemWorldAssets>,
) {
    for item in &catalog.items {
        if let Some(asset) = prepared_catalog_asset(item) {
            assets.0.insert(item.base_form_id, asset);
        }
    }
}

fn prepared_catalog_asset(item: &PreparedItemDefinition) -> Option<ItemWorldAsset> {
    let asset_path = item.world_asset_path.clone()?;
    if matches!(item.drop_collider, PreparedDropCollider::Missing) {
        return None;
    }
    Some(ItemWorldAsset {
        placement: PreparedPlacement {
            reference_form_id: 0,
            base_form_id: item.base_form_id,
            asset_path: Some(asset_path),
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: item.physics_asset_path.clone(),
            physics_source: None,
            physics_classification: PreparedPhysicsClassification::Dynamic,
            step_support: false,
            mutability: PreparedRuntimeMutability::ScriptAddressable,
            mutability_root_form_id: None,
            reference_kind: "Object".into(),
            base_kind: item.record_kind.clone(),
            editor_id: item.editor_id.clone(),
            display_name: item.display_name.clone(),
            count: 1,
            semantic: PreparedSemantic::Pickup(PreparedPickup {
                category: item.record_kind.clone(),
                value: item.value,
                weight: item.weight,
            }),
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            linked_reference_form_id: None,
            inventory: Vec::new(),
            audio: item.audio.clone(),
            ao_mode: "ao-none".into(),
        },
        drop_collider: item.drop_collider.clone(),
        quest_item: item.quest_item,
    })
}

fn resolve_drop_pose(
    context: &BoxdddPhysicsContext,
    camera_origin: Vec3,
    camera_forward: Vec3,
    player_fallback: Vec3,
) -> (Vec3, drop_policy::DropPlacementDecision) {
    let fallback = if player_fallback.is_finite() {
        player_fallback
    } else {
        camera_origin
    };
    let direction = camera_forward.normalize_or_zero();
    if !camera_origin.is_finite() || !direction.is_finite() || direction == Vec3::ZERO {
        return (fallback, drop_policy::DropPlacementDecision::fallback());
    }

    let decision = drop_policy::choose_candidate(|distance| {
        let Some(world) = context.world() else {
            return true;
        };
        let origin = boxddd::Vec3::new(camera_origin.x, camera_origin.y, camera_origin.z);
        let length = distance + drop_policy::DROP_COLLISION_CLEARANCE_METERS;
        let translation = boxddd::Vec3::new(
            direction.x * length,
            direction.y * length,
            direction.z * length,
        );
        let filter = QueryFilter::new().category_bits(4).mask_bits(1 | 2);
        world
            .cast_ray(origin, translation, filter)
            .map_or(true, |hits| !hits.is_empty())
    });
    let position = decision
        .distance
        .map_or(fallback, |distance| camera_origin + direction * distance);
    (position, decision)
}

#[allow(clippy::too_many_arguments)]
fn drop_inventory_items(
    mut commands: Commands,
    mut requests: MessageReader<DropInventoryStackRequested>,
    mut inventory: ResMut<PlayerInventory>,
    mut canonical: ResMut<CanonicalItemLedger>,
    equipment: Res<PlayerEquipment>,
    assets: Res<ItemWorldAssets>,
    asset_server: Res<AssetServer>,
    active: Res<ActiveCell>,
    residents: Res<ResidentCells>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    players: Query<&GlobalTransform, With<FpsPlayer>>,
    physics: NonSend<BoxdddPhysicsContext>,
    mut next_id: ResMut<NextRuntimeItemId>,
    mut notice: ResMut<InteractionNotice>,
    mut sounds: MessageWriter<PlaySound>,
) {
    for request in requests.read() {
        let Some(prepared_asset) = assets.0.get(&request.key.base_form_id) else {
            notice.show("That item has no prepared world asset");
            continue;
        };
        // Issue #81: the authoritative drop path refuses quest items and
        // caps regardless of which UI produced the request.
        if let Err(rejection) =
            item_rules::can_drop(request.key.base_form_id, prepared_asset.quest_item)
        {
            notice.show(match rejection {
                item_rules::TransferRejection::QuestItem => "Quest items cannot be dropped",
                item_rules::TransferRejection::Caps => "Caps cannot be dropped",
            });
            continue;
        }
        // Issue #98 (F98.2): equipped items cannot be dropped -- the same
        // authoritative choke point as the quest/caps check above.
        if equipment.is_equipped(request.key) {
            notice.show("Equipped items cannot be dropped");
            continue;
        }
        let template = &prepared_asset.placement;
        let Some(asset_path) = template.asset_path.as_ref() else {
            notice.show("That item has no prepared world model");
            continue;
        };
        let Some(cell_root) = residents.0.get(&active.0).map(|resident| resident.root) else {
            notice.show("The active cell is not ready for dropping");
            continue;
        };
        let view = cameras
            .iter()
            .find(|transform| transform.forward().is_finite())
            .or_else(|| players.iter().next());
        let Some(view) = view else {
            notice.show("No player view is available for dropping");
            continue;
        };
        if request.count <= 0 {
            notice.show("Drop quantity must be positive");
            continue;
        }
        let player_fallback = players
            .iter()
            .next()
            .map(|transform| {
                transform.translation()
                    + Vec3::Y * (CAPSULE_HEIGHT * 0.5 + drop_policy::DROP_FALLBACK_CLEARANCE_METERS)
            })
            .unwrap_or_else(|| view.translation());
        let (spawn, placement_decision) = resolve_drop_pose(
            &physics,
            view.translation(),
            view.forward().into(),
            player_fallback,
        );
        if inventory.available(request.key) < request.count {
            notice.show("The selected stack changed before it could be dropped");
            continue;
        }
        let runtime_id = next_id.0;
        next_id.0 = next_id.0.saturating_add(1).max(1);
        let stack = InventoryStack {
            base_form_id: request.key.base_form_id,
            count: request.count,
            condition: request.key.condition,
        };
        let before = inventory.legacy_snapshot();
        if let Err(error) = canonical.move_player_to_runtime(&before, runtime_id, active.0, stack) {
            notice.show(format!("Item transaction failed: {error}"));
            continue;
        }
        if !matches!(
            inventory.remove(request.key, request.count),
            TransferResult::Applied { .. }
        ) {
            notice.show("The selected stack changed before it could be dropped");
            continue;
        }
        let mut placement = template.clone();
        placement.reference_form_id = 0xff00_0000 | (runtime_id as u32 & 0x00ff_ffff);
        placement.translation = spawn.to_array();
        placement.rotation_xyzw = Quat::IDENTITY.to_array();
        placement.count = request.count;
        placement.owner_form_id = None;
        placement.owner_faction_rank = None;
        placement.physics_classification = PreparedPhysicsClassification::Dynamic;
        if request.count > 1 {
            let base_name = placement
                .display_name
                .clone()
                .or_else(|| placement.editor_id.clone())
                .unwrap_or_else(|| format!("{:08X}", placement.base_form_id));
            placement.display_name = Some(format!("{base_name} ({})", request.count));
        }
        let handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone()));
        let entity = commands
            .spawn((
                WorldAssetRoot(handle),
                PlacementRoot::new(placement.clone()),
                RuntimeWorldItem {
                    runtime_id,
                    stack,
                    cell_form_id: active.0,
                },
                PendingRuntimeCollider {
                    placement: placement.clone(),
                    drop_collider: prepared_asset.drop_collider.clone(),
                    restore: None,
                },
                Transform {
                    translation: spawn,
                    rotation: Quat::IDENTITY,
                    scale: Vec3::splat(placement.scale),
                },
                ChildOf(cell_root),
            ))
            .id();
        if let Some(sound) = placement.audio.drop_sound_form_id {
            sounds.write(PlaySound::at(sound, spawn));
        }
        notice.show(format!(
            "Dropped {}{}",
            placement
                .display_name
                .as_deref()
                .or(placement.editor_id.as_deref())
                .unwrap_or("item"),
            if request.count == 1 { "" } else { " stack" }
        ));
        info!(
            "drop runtime={} base={:08x} count={} cell={:08x} placement={:?} distance={:?} entity={entity:?}",
            runtime_id,
            request.key.base_form_id,
            request.count,
            active.0,
            placement_decision.mode,
            placement_decision.distance,
        );
    }
}

pub(crate) fn restore_dropped_items(world: &mut World, cell: u32, root: Entity) {
    let dropped = world
        .get_resource::<super::world::ActiveSaveState>()
        .and_then(|state| state.0.cells.get(&cell))
        .map(|cell| cell.dropped_items.values().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    if dropped.is_empty() {
        return;
    }
    let existing = {
        let mut query = world.query::<&RuntimeWorldItem>();
        query
            .iter(world)
            .filter(|item| item.cell_form_id == cell)
            .map(|item| item.runtime_id)
            .collect::<std::collections::HashSet<_>>()
    };
    for saved in dropped {
        if existing.contains(&saved.runtime_id) {
            continue;
        }
        let prepared_asset = world
            .get_resource::<PreparedItemCatalog>()
            .and_then(|catalog| {
                catalog
                    .items
                    .iter()
                    .find(|item| item.base_form_id == saved.stack.base_form_id)
            })
            .and_then(prepared_catalog_asset);
        let Some(prepared_asset) = prepared_asset else {
            warn!(
                "save restore dropped runtime={} missing prepared base {:08x}",
                saved.runtime_id, saved.stack.base_form_id
            );
            continue;
        };
        if let Some(mut canonical) = world.get_resource_mut::<CanonicalItemLedger>()
            && let Err(error) = canonical.ensure_runtime_item(
                saved.runtime_id,
                cell,
                InventoryStack {
                    base_form_id: saved.stack.base_form_id,
                    count: saved.stack.count,
                    condition: saved.stack.condition,
                },
            )
        {
            warn!(
                "save restore dropped runtime={} canonical item rejected: {error}",
                saved.runtime_id
            );
            continue;
        }
        let template = &prepared_asset.placement;
        let mut placement = template.clone();
        placement.reference_form_id = 0xff00_0000 | (saved.runtime_id as u32 & 0x00ff_ffff);
        placement.translation = saved.transform.translation;
        placement.rotation_xyzw = saved.transform.rotation_xyzw;
        placement.scale = saved.transform.scale[0];
        placement.count = saved.stack.count;
        placement.physics_classification = PreparedPhysicsClassification::Dynamic;
        if saved.stack.count > 1 {
            let base_name = placement
                .display_name
                .clone()
                .or_else(|| placement.editor_id.clone())
                .unwrap_or_else(|| format!("{:08X}", placement.base_form_id));
            placement.display_name = Some(format!("{base_name} ({})", saved.stack.count));
        }
        let path = placement
            .asset_path
            .as_ref()
            .expect("filtered prepared asset")
            .clone();
        let handle = world
            .resource::<AssetServer>()
            .load(GltfAssetLabel::Scene(0).from_asset(path));
        let rotation = Quat::from_array(saved.transform.rotation_xyzw).normalize();
        world.spawn((
            WorldAssetRoot(handle),
            PlacementRoot::new(placement.clone()),
            RuntimeWorldItem {
                runtime_id: saved.runtime_id,
                stack: InventoryStack {
                    base_form_id: saved.stack.base_form_id,
                    count: saved.stack.count,
                    condition: saved.stack.condition,
                },
                cell_form_id: cell,
            },
            PendingRuntimeCollider {
                placement,
                drop_collider: prepared_asset.drop_collider,
                restore: Some(super::world::DynamicBodyRestore {
                    translation: Vec3::from_array(saved.transform.translation),
                    rotation,
                    linear_velocity: Vec3::from_array(saved.body.linear_velocity),
                    angular_velocity: Vec3::from_array(saved.body.angular_velocity),
                    sleeping: saved.body.sleeping,
                }),
            },
            Transform {
                translation: Vec3::from_array(saved.transform.translation),
                rotation,
                scale: Vec3::from_array(saved.transform.scale),
            },
            ChildOf(root),
        ));
        info!(
            "save restore dropped runtime={} base={:08x} count={} cell={cell:08x}",
            saved.runtime_id, saved.stack.base_form_id, saved.stack.count
        );
    }
}
