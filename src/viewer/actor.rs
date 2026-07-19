//! Runtime projection of prepared actor assembly blueprints.
//!
//! Preparation is authoritative for identity, fallback selection, mesh
//! assembly, and equipment choice.  This module only turns that serialized
//! decision into one placement-root component set, a canonical item-holder
//! projection, an optional bounds proxy, and delayed scene attachments.

use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;
use bevyout_core::actor::{ActorAssemblyBlueprint, ActorFallbackLevel, ActorKind, ActorProxyKind};
use bevyout_core::item_transaction::{HolderId, ItemHolderState, ItemState};
use std::collections::HashSet;

use super::interaction::{CanonicalItemLedger, PlacementRoot};
use crate::vsa::{PreparedItemCatalog, PreparedItemCategory, PreparedPlacement};

use super::{PreparedSemantic, WorldAssetRoot};

pub(crate) struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_proxy_assets).add_systems(
            Update,
            (
                project_prepared_actors,
                reconcile_canonical_weapon_binding,
                attach_pending_weapons,
            )
                .chain()
                .in_set(super::plugins::ViewerSet::WorldSync),
        );
    }
}

/// Stable runtime identity for a living prepared actor.  It is inserted on
/// the same [`PlacementRoot`] that owns the actor's scene (or proxy), never on
/// a second transform root.
#[derive(Component, Clone, Debug)]
pub(crate) struct ActorRuntime {
    pub(crate) base_form_id: u32,
    pub(crate) reference_form_id: u32,
    pub(crate) kind: ActorKind,
    pub(crate) assembly: Option<ActorAssemblyBlueprint>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ActorWeaponRuntimeState {
    None,
    MissingModel,
    UnsupportedItem,
    PendingAttachment { loaded_frames: u8 },
    Attached { entity: Entity, node: String },
    MissingAttachmentNode { expected: String },
}

impl ActorWeaponRuntimeState {
    pub(crate) const fn label(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::MissingModel => "missing_model",
            Self::UnsupportedItem => "unsupported_item",
            Self::PendingAttachment { .. } => "pending_attachment",
            Self::Attached { .. } => "attached",
            Self::MissingAttachmentNode { .. } => "missing_attachment_node",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActorWeaponModel {
    pub(crate) item_form_id: u32,
    pub(crate) model_path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActorRuntimeDiagnostic {
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

/// Mutable ECS projection state.  The prepared assembly remains immutable on
/// [`ActorRuntime`]; this component reports what the viewer managed to create.
#[derive(Component, Clone, Debug)]
pub(crate) struct ActorRuntimeState {
    pub(crate) holder: HolderId,
    pub(crate) holder_seeded: bool,
    pub(crate) proxy_entity: Option<Entity>,
    pub(crate) bound_item_form_id: Option<u32>,
    pub(crate) weapon_model: Option<ActorWeaponModel>,
    pub(crate) weapon: ActorWeaponRuntimeState,
    pub(crate) diagnostics: Vec<ActorRuntimeDiagnostic>,
}

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct ActorProxyVisual;

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct ActorWeaponVisual {
    pub(crate) item_form_id: u32,
    pub(crate) actor_reference_form_id: u32,
}

#[derive(Resource)]
struct ActorProxyAssets {
    humanoid: Handle<Mesh>,
    creature: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn create_proxy_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(ActorProxyAssets {
        humanoid: meshes.add(Cuboid::new(0.7, 1.8, 0.5)),
        creature: meshes.add(Cuboid::new(1.4, 0.9, 1.8)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgba(0.95, 0.12, 0.55, 0.38),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
    });
}

pub(crate) fn is_actor_semantic(semantic: &PreparedSemantic) -> bool {
    matches!(
        semantic,
        PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
    )
}

/// Root scale chosen at scene-spawn time so every downstream system observes
/// the prepared feet-pivot scale, including systems that run before the actor
/// projection inserts its marker components.
pub(crate) fn placement_root_scale(placement: &PreparedPlacement) -> f32 {
    let assembly =
        prepared_actor(&placement.semantic).and_then(|(_, prepared)| prepared.assembly.as_ref());
    valid_root_scale(assembly, placement.scale)
}

fn prepared_actor(
    semantic: &PreparedSemantic,
) -> Option<(ActorKind, &bevyout_core::manifest::PreparedActor)> {
    match semantic {
        PreparedSemantic::Npc(actor) => Some((ActorKind::Humanoid, actor)),
        PreparedSemantic::Creature(actor) => Some((ActorKind::Creature, actor)),
        _ => None,
    }
}

fn valid_root_scale(assembly: Option<&ActorAssemblyBlueprint>, placement_scale: f32) -> f32 {
    let candidate = assembly.map_or(placement_scale, |assembly| assembly.root_scale);
    if candidate.is_finite() && candidate > 0.0 {
        candidate
    } else {
        1.0
    }
}

fn initial_weapon_projection(
    assembly: Option<&ActorAssemblyBlueprint>,
) -> (
    Option<u32>,
    Option<ActorWeaponModel>,
    ActorWeaponRuntimeState,
) {
    match assembly.and_then(|assembly| assembly.equipped_weapon.as_ref()) {
        None => (None, None, ActorWeaponRuntimeState::None),
        Some(weapon) if weapon.model_available && weapon.model_path.is_some() => (
            Some(weapon.item_form_id),
            Some(ActorWeaponModel {
                item_form_id: weapon.item_form_id,
                model_path: weapon
                    .model_path
                    .clone()
                    .expect("checked prepared model path"),
            }),
            ActorWeaponRuntimeState::PendingAttachment { loaded_frames: 0 },
        ),
        Some(weapon) => (
            Some(weapon.item_form_id),
            None,
            ActorWeaponRuntimeState::MissingModel,
        ),
    }
}

fn needs_proxy_visual(asset_path: Option<&str>, assembly: Option<&ActorAssemblyBlueprint>) -> bool {
    asset_path.is_none()
        && assembly.is_none_or(|assembly| {
            assembly.fallback.level == ActorFallbackLevel::ProxyMesh
                || assembly.fallback.proxy_kind != ActorProxyKind::None
        })
}

#[allow(clippy::too_many_arguments)]
fn project_prepared_actors(
    mut commands: Commands,
    roots: Query<(Entity, &PlacementRoot), Added<PlacementRoot>>,
    mut transforms: Query<&mut Transform>,
    proxy_assets: Res<ActorProxyAssets>,
    mut canonical: ResMut<CanonicalItemLedger>,
) {
    for (entity, root) in &roots {
        let placement = root.placement();
        let Some((semantic_kind, prepared)) = prepared_actor(&placement.semantic) else {
            continue;
        };
        let assembly = prepared.assembly.clone();
        let kind = assembly
            .as_ref()
            .map_or(semantic_kind, |assembly| assembly.kind);
        // The placement reference is the runtime identity registered by the
        // scene spawner. Keep it authoritative even if a malformed/stale
        // blueprint carries zero or a mismatched reference.
        let reference_form_id = placement.reference_form_id;
        let base_form_id = assembly
            .as_ref()
            .map_or(placement.base_form_id, |assembly| {
                if assembly.resolved_base_form_id != 0 {
                    assembly.resolved_base_form_id
                } else {
                    placement.base_form_id
                }
            });
        let root_scale = valid_root_scale(assembly.as_ref(), placement.scale);
        if let Ok(mut transform) = transforms.get_mut(entity) {
            transform.scale = Vec3::splat(root_scale);
        }

        let holder = HolderId::Actor { reference_form_id };
        let mut diagnostics = Vec::new();
        if assembly.is_none() {
            diagnostics.push(ActorRuntimeDiagnostic {
                code: "missing_actor_blueprint",
                message: "prepared actor has no assembly blueprint".to_owned(),
            });
        }
        if assembly.as_ref().is_some_and(|assembly| {
            assembly.reference_form_id != 0
                && assembly.reference_form_id != placement.reference_form_id
        }) {
            diagnostics.push(ActorRuntimeDiagnostic {
                code: "actor_identity_mismatch",
                message: format!(
                    "blueprint reference {:08x} differs from placement {:08x}",
                    assembly
                        .as_ref()
                        .map_or(0, |assembly| assembly.reference_form_id),
                    placement.reference_form_id,
                ),
            });
        }
        let holder_seeded = seed_actor_holder(
            &mut canonical,
            holder,
            &placement.inventory,
            assembly.as_ref(),
            &mut diagnostics,
        );
        let (bound_item_form_id, weapon_model, weapon) =
            initial_weapon_projection(assembly.as_ref());
        if matches!(weapon, ActorWeaponRuntimeState::MissingModel) {
            let item_form_id = assembly
                .as_ref()
                .and_then(|assembly| assembly.equipped_weapon.as_ref())
                .map_or(0, |weapon| weapon.item_form_id);
            diagnostics.push(ActorRuntimeDiagnostic {
                code: "missing_weapon_model",
                message: format!("equipped weapon {item_form_id:08x} has no prepared model"),
            });
        }

        let needs_proxy = needs_proxy_visual(placement.asset_path.as_deref(), assembly.as_ref());
        let proxy_entity = needs_proxy.then(|| {
            let (mesh, vertical_offset) = match kind {
                ActorKind::Humanoid => (proxy_assets.humanoid.clone(), 0.9),
                ActorKind::Creature => (proxy_assets.creature.clone(), 0.45),
            };
            commands
                .spawn((
                    ActorProxyVisual,
                    Mesh3d(mesh),
                    MeshMaterial3d(proxy_assets.material.clone()),
                    Transform::from_xyz(0.0, vertical_offset, 0.0),
                    ChildOf(entity),
                ))
                .id()
        });
        if proxy_entity.is_some() {
            diagnostics.push(ActorRuntimeDiagnostic {
                code: "actor_proxy_visual",
                message: "viewer spawned deterministic actor bounds proxy".to_owned(),
            });
        }

        commands.entity(entity).insert((
            ActorRuntime {
                base_form_id,
                reference_form_id,
                kind,
                assembly,
            },
            ActorRuntimeState {
                holder,
                holder_seeded,
                proxy_entity,
                bound_item_form_id,
                weapon_model,
                weapon,
                diagnostics,
            },
        ));
    }
}

fn seed_actor_holder(
    canonical: &mut CanonicalItemLedger,
    holder: HolderId,
    inventory: &[bevyout_core::manifest::PreparedInventoryEntry],
    assembly: Option<&ActorAssemblyBlueprint>,
    diagnostics: &mut Vec<ActorRuntimeDiagnostic>,
) -> bool {
    if canonical.ledger.holders().contains_key(&holder) {
        return true;
    }
    if let Err(error) = canonical
        .ledger
        .insert_holder(holder, ItemHolderState::default())
    {
        diagnostics.push(ActorRuntimeDiagnostic {
            code: "actor_holder_seed_failed",
            message: error.to_string(),
        });
        return false;
    }

    let mut equipped_instance = None;
    let mut seeded_form_ids = HashSet::new();
    let equipped_form_id = assembly
        .and_then(|assembly| assembly.equipped_weapon.as_ref())
        .map(|weapon| weapon.item_form_id);
    for entry in inventory.iter().filter(|entry| entry.count > 0) {
        if entry.leveled {
            diagnostics.push(ActorRuntimeDiagnostic {
                code: "actor_leveled_inventory_deferred",
                message: format!(
                    "leveled inventory entry {:08x} is not a canonical item instance",
                    entry.base_form_id
                ),
            });
            continue;
        }
        let Ok(count) = u32::try_from(entry.count) else {
            continue;
        };
        match canonical.ledger.insert_new_item(
            holder,
            entry.base_form_id,
            count,
            ItemState::default(),
        ) {
            Ok(instance) if Some(entry.base_form_id) == equipped_form_id => {
                seeded_form_ids.insert(entry.base_form_id);
                equipped_instance = Some(instance);
            }
            Ok(_) => {
                seeded_form_ids.insert(entry.base_form_id);
            }
            Err(error) => diagnostics.push(ActorRuntimeDiagnostic {
                code: "actor_inventory_seed_failed",
                message: format!("item {:08x}: {error}", entry.base_form_id),
            }),
        }
    }
    if let Some(assembly) = assembly {
        for apparel in &assembly.apparel {
            if seeded_form_ids.insert(apparel.item_form_id)
                && let Err(error) = canonical.ledger.insert_new_item(
                    holder,
                    apparel.item_form_id,
                    1,
                    ItemState::default(),
                )
            {
                diagnostics.push(ActorRuntimeDiagnostic {
                    code: "actor_inventory_seed_failed",
                    message: format!("apparel {:08x}: {error}", apparel.item_form_id),
                });
            }
        }
        if let Some(weapon) = assembly.equipped_weapon.as_ref()
            && equipped_instance.is_none()
        {
            match canonical.ledger.insert_new_item(
                holder,
                weapon.item_form_id,
                1,
                ItemState::default(),
            ) {
                Ok(instance) => {
                    equipped_instance = Some(instance);
                    diagnostics.push(ActorRuntimeDiagnostic {
                        code: "equipped_weapon_seeded_from_blueprint",
                        message: format!(
                            "equipped weapon {:08x} was absent from prepared inventory",
                            weapon.item_form_id
                        ),
                    });
                }
                Err(error) => diagnostics.push(ActorRuntimeDiagnostic {
                    code: "actor_inventory_seed_failed",
                    message: format!("weapon {:08x}: {error}", weapon.item_form_id),
                }),
            }
        }
    }
    if let Some(equipped_form_id) = equipped_form_id {
        match equipped_instance {
            Some(item_id) => {
                if let Err(error) = canonical.ledger.equip(holder, item_id) {
                    diagnostics.push(ActorRuntimeDiagnostic {
                        code: "actor_equipment_bind_failed",
                        message: format!("weapon {equipped_form_id:08x}: {error}"),
                    });
                }
            }
            None => diagnostics.push(ActorRuntimeDiagnostic {
                code: "equipped_weapon_not_in_inventory",
                message: format!(
                    "equipped weapon {equipped_form_id:08x} could not enter the actor holder"
                ),
            }),
        }
    }
    true
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum BoundWeaponResolution {
    Unequipped,
    Known(ActorWeaponModel),
    MissingModel(u32),
    UnsupportedItem(u32),
}

fn bound_item_form_id(canonical: &CanonicalItemLedger, holder: HolderId) -> Option<u32> {
    let item_id = canonical.ledger.bindings().get(&holder)?.equipped?;
    canonical
        .ledger
        .holders()
        .get(&holder)?
        .find(item_id)
        .map(|item| item.base_form_id)
}

fn resolve_bound_weapon(
    actor: &ActorRuntime,
    catalog: &PreparedItemCatalog,
    item_form_id: Option<u32>,
) -> BoundWeaponResolution {
    let Some(item_form_id) = item_form_id else {
        return BoundWeaponResolution::Unequipped;
    };
    if let Some(weapon) = actor
        .assembly
        .as_ref()
        .and_then(|assembly| assembly.equipped_weapon.as_ref())
        .filter(|weapon| weapon.item_form_id == item_form_id)
        && weapon.model_available
        && let Some(model_path) = weapon.model_path.as_ref()
    {
        return BoundWeaponResolution::Known(ActorWeaponModel {
            item_form_id,
            model_path: model_path.clone(),
        });
    }
    let Some(item) = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == item_form_id)
    else {
        return if actor
            .assembly
            .as_ref()
            .and_then(|assembly| assembly.equipped_weapon.as_ref())
            .is_some_and(|weapon| weapon.item_form_id == item_form_id)
        {
            BoundWeaponResolution::MissingModel(item_form_id)
        } else {
            BoundWeaponResolution::UnsupportedItem(item_form_id)
        };
    };
    if !matches!(item.category, PreparedItemCategory::Weapons) {
        return BoundWeaponResolution::UnsupportedItem(item_form_id);
    }
    item.world_asset_path.as_ref().map_or(
        BoundWeaponResolution::MissingModel(item_form_id),
        |model_path| {
            BoundWeaponResolution::Known(ActorWeaponModel {
                item_form_id,
                model_path: model_path.clone(),
            })
        },
    )
}

fn clear_runtime_weapon_diagnostics(state: &mut ActorRuntimeState) {
    state.diagnostics.retain(|diagnostic| {
        !matches!(
            diagnostic.code,
            "missing_weapon_model"
                | "actor_equipment_unsupported"
                | "missing_weapon_attachment_node"
        )
    });
}

fn reconcile_canonical_weapon_binding(
    mut commands: Commands,
    canonical: Res<CanonicalItemLedger>,
    catalog: Res<PreparedItemCatalog>,
    live_visuals: Query<(), With<ActorWeaponVisual>>,
    mut actors: Query<(&ActorRuntime, &mut ActorRuntimeState)>,
) {
    for (actor, mut state) in &mut actors {
        let bound_item = bound_item_form_id(&canonical, state.holder);
        let resolution = resolve_bound_weapon(actor, &catalog, bound_item);
        let desired_model = match &resolution {
            BoundWeaponResolution::Known(model) => Some(model.clone()),
            _ => None,
        };
        let state_matches = state.bound_item_form_id == bound_item
            && state.weapon_model == desired_model
            && matches!(
                (&resolution, &state.weapon),
                (
                    BoundWeaponResolution::Unequipped,
                    ActorWeaponRuntimeState::None
                ) | (
                    BoundWeaponResolution::MissingModel(_),
                    ActorWeaponRuntimeState::MissingModel
                ) | (
                    BoundWeaponResolution::UnsupportedItem(_),
                    ActorWeaponRuntimeState::UnsupportedItem,
                ) | (
                    BoundWeaponResolution::Known(_),
                    ActorWeaponRuntimeState::PendingAttachment { .. },
                ) | (
                    BoundWeaponResolution::Known(_),
                    ActorWeaponRuntimeState::MissingAttachmentNode { .. },
                ) | (
                    BoundWeaponResolution::Known(_),
                    ActorWeaponRuntimeState::Attached { .. }
                )
            );
        if state_matches {
            continue;
        }

        let stale_visual = match &state.weapon {
            ActorWeaponRuntimeState::Attached { entity, .. } => Some(*entity),
            _ => None,
        };
        if let Some(entity) = stale_visual
            && live_visuals.contains(entity)
        {
            commands.entity(entity).despawn();
            info!(
                "actor weapon detached reference={:08x} item={:08x}",
                actor.reference_form_id,
                state.bound_item_form_id.unwrap_or_default(),
            );
        }
        clear_runtime_weapon_diagnostics(&mut state);
        state.bound_item_form_id = bound_item;
        state.weapon_model = desired_model;
        state.weapon = match resolution {
            BoundWeaponResolution::Unequipped => ActorWeaponRuntimeState::None,
            BoundWeaponResolution::Known(_) => {
                ActorWeaponRuntimeState::PendingAttachment { loaded_frames: 0 }
            }
            BoundWeaponResolution::MissingModel(item_form_id) => {
                state.diagnostics.push(ActorRuntimeDiagnostic {
                    code: "missing_weapon_model",
                    message: format!(
                        "canonically equipped weapon {item_form_id:08x} has no prepared model"
                    ),
                });
                ActorWeaponRuntimeState::MissingModel
            }
            BoundWeaponResolution::UnsupportedItem(item_form_id) => {
                state.diagnostics.push(ActorRuntimeDiagnostic {
                    code: "actor_equipment_unsupported",
                    message: format!(
                        "canonically equipped item {item_form_id:08x} is not a prepared weapon"
                    ),
                });
                ActorWeaponRuntimeState::UnsupportedItem
            }
        };
    }
}

fn find_attachment_node(
    root: Entity,
    children: &Query<&Children>,
    names: &Query<&Name>,
) -> Option<(Entity, String)> {
    const CANDIDATES: [&str; 4] = ["Weapon", "RightHand", "Bip01 R Hand", "R Hand"];
    let mut descendants = vec![root];
    let mut named = Vec::new();
    while let Some(entity) = descendants.pop() {
        if let Ok(name) = names.get(entity) {
            named.push((entity, name.as_str().to_owned()));
        }
        if let Ok(child_list) = children.get(entity) {
            descendants.extend(child_list.iter());
        }
    }
    for candidate in CANDIDATES {
        if let Some(found) = named.iter().find(|(_, name)| {
            name.eq_ignore_ascii_case(candidate)
                || super::player::actor_node_names_match(candidate, name)
        }) {
            return Some(found.clone());
        }
    }
    None
}

fn attach_pending_weapons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut actors: Query<(
        Entity,
        &ActorRuntime,
        &mut ActorRuntimeState,
        Option<&WorldAssetRoot>,
    )>,
    children: Query<&Children>,
    names: Query<&Name>,
) {
    for (root, actor, mut state, scene_root) in &mut actors {
        let loaded_frames = match &state.weapon {
            ActorWeaponRuntimeState::PendingAttachment { loaded_frames } => *loaded_frames,
            _ => continue,
        };
        let Some(weapon) = state.weapon_model.clone() else {
            state.weapon = ActorWeaponRuntimeState::None;
            continue;
        };
        if let Some((attachment, node)) = find_attachment_node(root, &children, &names) {
            let handle =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(weapon.model_path.clone()));
            let weapon_entity = commands
                .spawn((
                    ActorWeaponVisual {
                        item_form_id: weapon.item_form_id,
                        actor_reference_form_id: actor.reference_form_id,
                    },
                    WorldAssetRoot(handle),
                    Transform::IDENTITY,
                    ChildOf(attachment),
                ))
                .id();
            info!(
                "actor weapon attached reference={:08x} item={:08x} node={node}",
                actor.reference_form_id, weapon.item_form_id,
            );
            state.weapon = ActorWeaponRuntimeState::Attached {
                entity: weapon_entity,
                node,
            };
            continue;
        }

        let hierarchy_ready = scene_root
            .is_none_or(|scene_root| asset_server.is_loaded_with_dependencies(scene_root.0.id()));
        if hierarchy_ready {
            // Asset readiness can precede WorldAsset hierarchy insertion by a
            // frame. Give the scene spawner a bounded grace window before a
            // missing named socket becomes a terminal diagnostic.
            let loaded_frames = loaded_frames.saturating_add(1);
            if loaded_frames < 3 && scene_root.is_some() {
                state.weapon = ActorWeaponRuntimeState::PendingAttachment { loaded_frames };
                continue;
            }
            const EXPECTED: &str = "RightHand/Weapon";
            warn!(
                "actor weapon attachment missing reference={:08x} item={:08x} expected={EXPECTED}",
                actor.reference_form_id, weapon.item_form_id,
            );
            state.diagnostics.push(ActorRuntimeDiagnostic {
                code: "missing_weapon_attachment_node",
                message: format!(
                    "weapon {:08x} could not find {EXPECTED}",
                    weapon.item_form_id
                ),
            });
            state.weapon = ActorWeaponRuntimeState::MissingAttachmentNode {
                expected: EXPECTED.to_owned(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::SystemState;
    use bevyout_core::actor::{ActorAttachmentPoint, AssembledWeapon};

    #[test]
    fn attachment_discovery_prefers_weapon_socket_over_right_hand() {
        let mut world = World::new();
        let root = world.spawn(Name::new("ActorRoot")).id();
        let hand = world.spawn((Name::new("Bip01 R Hand"), ChildOf(root))).id();
        let socket = world.spawn((Name::new("Weapon"), ChildOf(hand))).id();
        let mut queries = SystemState::<(Query<&Children>, Query<&Name>)>::new(&mut world);
        let (children, names) = queries.get(&world).expect("valid read-only queries");
        let found = find_attachment_node(root, &children, &names).expect("weapon socket");
        assert_eq!(found.0, socket);
        assert_eq!(found.1, "Weapon");
    }

    #[test]
    fn root_scale_rejects_invalid_prepared_values() {
        let mut assembly = ActorAssemblyBlueprint {
            root_scale: 1.25,
            ..Default::default()
        };
        assert_eq!(valid_root_scale(Some(&assembly), 0.5), 1.25);
        assembly.root_scale = f32::NAN;
        assert_eq!(valid_root_scale(Some(&assembly), 0.5), 1.0);
    }

    #[test]
    fn assetless_actor_semantics_still_require_a_runtime_root() {
        assert!(is_actor_semantic(
            &PreparedSemantic::Npc(Default::default())
        ));
        assert!(is_actor_semantic(&PreparedSemantic::Creature(
            Default::default()
        )));
        assert!(!is_actor_semantic(&PreparedSemantic::Static));
    }

    #[test]
    fn every_assetless_proxy_kind_gets_a_bounds_visual() {
        for proxy_kind in [ActorProxyKind::GenericHumanoid, ActorProxyKind::Bounds] {
            let assembly = ActorAssemblyBlueprint {
                fallback: bevyout_core::actor::ActorFallbackDecision {
                    proxy_kind,
                    ..Default::default()
                },
                ..Default::default()
            };
            assert!(needs_proxy_visual(None, Some(&assembly)));
            assert!(!needs_proxy_visual(
                Some("assets/actor.glb"),
                Some(&assembly)
            ));
        }
    }

    #[test]
    fn projection_keeps_assetless_identity_scale_holder_and_proxy_on_one_root() {
        let blueprint = ActorAssemblyBlueprint {
            source_base_form_id: 0x10,
            resolved_base_form_id: 0x11,
            reference_form_id: 0x20,
            kind: ActorKind::Humanoid,
            root_scale: 1.2,
            fallback: bevyout_core::actor::ActorFallbackDecision {
                base_form_id: 0x11,
                reference_form_id: 0x20,
                level: ActorFallbackLevel::GenericProjectBody,
                proxy_kind: ActorProxyKind::GenericHumanoid,
                ..Default::default()
            },
            ..Default::default()
        };
        let placement_ron = r#"(
            reference_form_id: 32,
            base_form_id: 16,
            asset_path: None,
            translation: (1.0, 2.0, 3.0),
            rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
            scale: 0.5,
            error: None,
            semantic: Npc((base_template_form_id: None, assembly: None)),
        )"#;
        let mut placement: PreparedPlacement = ron::from_str(placement_ron).unwrap();
        let PreparedSemantic::Npc(prepared) = &mut placement.semantic else {
            unreachable!("test semantic is an NPC")
        };
        prepared.assembly = Some(blueprint);

        let mut app = App::new();
        app.insert_resource(CanonicalItemLedger::default())
            .insert_resource(ActorProxyAssets {
                humanoid: Handle::default(),
                creature: Handle::default(),
                material: Handle::default(),
            })
            .add_systems(Update, project_prepared_actors);
        let root = app
            .world_mut()
            .spawn((PlacementRoot::new(placement), Transform::default()))
            .id();

        app.update();

        let runtime = app
            .world()
            .get::<ActorRuntime>(root)
            .expect("actor identity");
        assert_eq!(runtime.reference_form_id, 0x20);
        assert_eq!(runtime.base_form_id, 0x11);
        assert_eq!(
            app.world().get::<Transform>(root).unwrap().scale,
            Vec3::splat(1.2)
        );
        let state = app
            .world()
            .get::<ActorRuntimeState>(root)
            .expect("actor projection state");
        assert!(state.holder_seeded);
        let proxy = state.proxy_entity.expect("generic fallback bounds proxy");
        assert!(app.world().get::<ActorProxyVisual>(proxy).is_some());
        assert_eq!(
            app.world().get::<ChildOf>(proxy).map(ChildOf::parent),
            Some(root)
        );
        assert!(
            app.world()
                .resource::<CanonicalItemLedger>()
                .ledger
                .holders()
                .contains_key(&HolderId::Actor {
                    reference_form_id: 0x20,
                })
        );
    }

    #[test]
    fn canonical_holder_seeds_and_equips_the_prepared_weapon() {
        let holder = HolderId::Actor {
            reference_form_id: 0x20,
        };
        let assembly = ActorAssemblyBlueprint {
            reference_form_id: 0x20,
            equipped_weapon: Some(AssembledWeapon {
                item_form_id: 0x30,
                model_path: Some("assets/weapon.glb".into()),
                attachment_point: ActorAttachmentPoint::RightHand,
                model_available: true,
            }),
            ..Default::default()
        };
        let inventory = [bevyout_core::manifest::PreparedInventoryEntry {
            base_form_id: 0x30,
            count: 1,
            record_kind: "WEAP".into(),
            editor_id: None,
            display_name: None,
            leveled: false,
        }];
        let mut canonical = CanonicalItemLedger::default();
        let mut diagnostics = Vec::new();

        assert!(seed_actor_holder(
            &mut canonical,
            holder,
            &inventory,
            Some(&assembly),
            &mut diagnostics,
        ));
        assert!(diagnostics.is_empty());
        let equipped = canonical.ledger.bindings()[&holder]
            .equipped
            .expect("equipped weapon binding");
        assert_eq!(
            canonical.ledger.holders()[&holder]
                .find(equipped)
                .expect("equipped item")
                .base_form_id,
            0x30
        );
    }

    #[test]
    fn canonical_binding_attaches_supported_weapon_and_detaches_stale_visual() {
        let holder = HolderId::Actor {
            reference_form_id: 0x20,
        };
        let mut canonical = CanonicalItemLedger::default();
        canonical
            .ledger
            .insert_holder(holder, ItemHolderState::default())
            .unwrap();
        let instance = canonical
            .ledger
            .insert_new_item(holder, 0x30, 1, ItemState::default())
            .unwrap();
        canonical.ledger.equip(holder, instance).unwrap();

        let mut app = App::new();
        app.insert_resource(canonical)
            .insert_resource(PreparedItemCatalog::default())
            .add_systems(Update, reconcile_canonical_weapon_binding);
        let actor = app
            .world_mut()
            .spawn((
                ActorRuntime {
                    base_form_id: 0x10,
                    reference_form_id: 0x20,
                    kind: ActorKind::Humanoid,
                    assembly: Some(ActorAssemblyBlueprint {
                        reference_form_id: 0x20,
                        equipped_weapon: Some(AssembledWeapon {
                            item_form_id: 0x30,
                            model_path: Some("assets/weapon.glb".into()),
                            attachment_point: ActorAttachmentPoint::RightHand,
                            model_available: true,
                        }),
                        ..Default::default()
                    }),
                },
                ActorRuntimeState {
                    holder,
                    holder_seeded: true,
                    proxy_entity: None,
                    bound_item_form_id: None,
                    weapon_model: None,
                    weapon: ActorWeaponRuntimeState::None,
                    diagnostics: Vec::new(),
                },
            ))
            .id();

        app.update();
        let state = app.world().get::<ActorRuntimeState>(actor).unwrap();
        assert_eq!(state.bound_item_form_id, Some(0x30));
        assert!(matches!(
            state.weapon,
            ActorWeaponRuntimeState::PendingAttachment { .. }
        ));

        let visual = app
            .world_mut()
            .spawn(ActorWeaponVisual {
                item_form_id: 0x30,
                actor_reference_form_id: 0x20,
            })
            .id();
        app.world_mut()
            .get_mut::<ActorRuntimeState>(actor)
            .unwrap()
            .weapon = ActorWeaponRuntimeState::Attached {
            entity: visual,
            node: "Weapon".into(),
        };
        app.world_mut()
            .resource_mut::<CanonicalItemLedger>()
            .ledger
            .unequip(holder)
            .unwrap();

        app.update();
        let state = app.world().get::<ActorRuntimeState>(actor).unwrap();
        assert_eq!(state.bound_item_form_id, None);
        assert!(matches!(state.weapon, ActorWeaponRuntimeState::None));
        assert!(app.world().get_entity(visual).is_err());
    }

    #[test]
    fn attached_weapon_is_not_reset_while_its_deferred_spawn_is_not_queryable() {
        let holder = HolderId::Actor {
            reference_form_id: 0x20,
        };
        let mut canonical = CanonicalItemLedger::default();
        canonical
            .ledger
            .insert_holder(holder, ItemHolderState::default())
            .unwrap();
        let instance = canonical
            .ledger
            .insert_new_item(holder, 0x30, 1, ItemState::default())
            .unwrap();
        canonical.ledger.equip(holder, instance).unwrap();

        let deferred_entity = Entity::from_raw_u32(900).unwrap();
        let mut app = App::new();
        app.insert_resource(canonical)
            .insert_resource(PreparedItemCatalog::default())
            .add_systems(Update, reconcile_canonical_weapon_binding);
        let actor = app
            .world_mut()
            .spawn((
                ActorRuntime {
                    base_form_id: 0x10,
                    reference_form_id: 0x20,
                    kind: ActorKind::Humanoid,
                    assembly: Some(ActorAssemblyBlueprint {
                        reference_form_id: 0x20,
                        equipped_weapon: Some(AssembledWeapon {
                            item_form_id: 0x30,
                            model_path: Some("assets/weapon.glb".into()),
                            attachment_point: ActorAttachmentPoint::RightHand,
                            model_available: true,
                        }),
                        ..Default::default()
                    }),
                },
                ActorRuntimeState {
                    holder,
                    holder_seeded: true,
                    proxy_entity: None,
                    bound_item_form_id: Some(0x30),
                    weapon_model: Some(ActorWeaponModel {
                        item_form_id: 0x30,
                        model_path: "assets/weapon.glb".into(),
                    }),
                    weapon: ActorWeaponRuntimeState::Attached {
                        entity: deferred_entity,
                        node: "Weapon".into(),
                    },
                    diagnostics: Vec::new(),
                },
            ))
            .id();

        app.update();

        assert!(matches!(
            app.world().get::<ActorRuntimeState>(actor).unwrap().weapon,
            ActorWeaponRuntimeState::Attached { entity, .. } if entity == deferred_entity
        ));
    }
}
