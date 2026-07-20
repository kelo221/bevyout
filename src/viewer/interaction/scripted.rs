//! Scripted console and agent-bridge interaction entry points.
//!
//! These functions intentionally bypass player focus while sharing the same
//! state, animation, audio, item-ledger, and persistence contracts as runtime
//! activation.

use super::*;

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

/// Wave-4 acceptance seam: scripted (console/BRP) container activation,
/// mirroring `activate_focused_placement`'s container branch (open-set
/// toggle, clip, sound, notice) minus the raycast focus/distance checks.
/// Returns the new open state. Used to drive the #60/#61 gate walk-through
/// ("open a container, revisit, restart") over the agent bridge.
pub(crate) fn scripted_container_toggle(world: &mut World, entity: Entity) -> bool {
    let placement = world
        .get::<PlacementRoot>(entity)
        .expect("caller resolved a placement root")
        .placement()
        .clone();
    let name = placement
        .display_name
        .clone()
        .or_else(|| placement.editor_id.clone())
        .unwrap_or_else(|| format!("{:08x}", placement.reference_form_id));
    let position = world
        .get::<GlobalTransform>(entity)
        .map(|transform| transform.translation())
        .unwrap_or_default();
    let mut state = world.get_resource_or_insert_with(InteractionState::default);
    let opening = !state.open.contains(&entity);
    let (sound, transition) = if opening {
        state.open.insert(entity);
        (placement.audio.open_sound_form_id, ClipTransition::Opening)
    } else {
        state.open.remove(&entity);
        (placement.audio.close_sound_form_id, ClipTransition::Closing)
    };
    if let Some(form_id) = sound {
        world.write_message(PlaySound::container_at(form_id, position));
    }
    world.write_message(animation::PlayPlacementAnimation {
        root: entity,
        transition,
        lead_ms: 0.0,
    });
    let notice_text = if opening {
        format!("{name}: {}", inventory_summary(&placement.inventory))
    } else {
        format!("Closed {name}")
    };
    world
        .get_resource_or_insert_with(InteractionNotice::default)
        .show(notice_text);
    // Wave-2 seam: scripted opens go through the same seed-once container
    // store as the player path, so console/BRP `activate` observes (and
    // triggers) exactly the state the transfer modal and #76's capture see —
    // including the deterministic first-open leveled roll. Stacks are logged
    // for gate evidence (determinism across runs, no re-roll after reload).
    if opening {
        let active_cell = world
            .get_resource::<ActiveCell>()
            .map(|cell| cell.0)
            .unwrap_or_default();
        let playthrough_seed = world
            .get_resource::<PlaythroughSeed>()
            .map(|seed| seed.0)
            .unwrap_or_default();
        let leveled_lists = if placement.inventory.iter().any(|entry| entry.leveled) {
            world
                .get_resource::<ResidentCells>()
                .and_then(|cells| cells.0.get(&active_cell))
                .map(|resident| leveled_lists_from_manifest(&resident.manifest.leveled_lists))
                .unwrap_or_default()
        } else {
            BTreeMap::new()
        };
        let mut states = world.get_resource_or_insert_with(ContainerStates::default);
        let resolved = seed_loot_holder(
            &mut states,
            &placement,
            active_cell,
            playthrough_seed,
            &leveled_lists,
        );
        info!(
            "container {} ({:08x}) opened with {} stacks: {:?}",
            name, placement.reference_form_id, resolved.0, resolved.1
        );
    } else {
        info!(
            "container {} ({:08x}) closed",
            name, placement.reference_form_id
        );
    }
    opening
}

/// M4 wave 3 (#112): scripted intra-cell door open, for the `bevy_landmass`
/// nav-agent door-link spike (`nav::agent`'s door-link system) to request an
/// off-mesh-link door through the same boundary the `activate` console
/// command uses, rather than inventing a parallel door-opening path.
/// `scripted_door_travel` (above) only covers the destination/cell-swap
/// case; the real player path (`activate_focused_placement`'s door branch)
/// is inline in a multi-`Query`/`MessageWriter` system and not directly
/// callable from `&mut World`. This mirrors `scripted_container_toggle`'s
/// shape (state toggle, sound, animation clip) restricted to the "ensure
/// open" direction a traversing agent needs, and -- like
/// `console::activate_reference`'s scripted door branch -- bypasses locks,
/// since this is dev/agent tooling, not the player's raycast-focused `E`
/// activation. Idempotent: a door already open is left open. Returns
/// whether this call transitioned the door from closed to open.
pub(crate) fn scripted_door_open(world: &mut World, entity: Entity) -> bool {
    scripted_door_set_open(world, entity, true)
}

/// Issue #177: scripted intra-cell door *toggle*, the console/BRP surface for
/// an ordinary door with no travel destination. Before this, `activate` on
/// such a door failed with `no_destination` -- the console path, like the nav
/// link path, was wired only to travel doors, so an in-cell door had no open
/// mechanism anywhere in the runtime and neither a human nor the manual
/// acceptance script could drive one. Shares every step with
/// [`scripted_door_open`] (state, sound, animation clip, log line); only the
/// direction differs. Returns whether the door is open after this call.
pub(crate) fn scripted_door_toggle(world: &mut World, entity: Entity) -> bool {
    let open = world
        .get_resource_or_insert_with(InteractionState::default)
        .open
        .contains(&entity);
    scripted_door_set_open(world, entity, !open)
}

/// Shared implementation for [`scripted_door_open`] and
/// [`scripted_door_toggle`]: drives `entity`'s door state to `open`,
/// emitting the matching sound cue, animation transition and log line.
/// A no-op when the door is already in the requested state. Returns the
/// door's state after the call, so `scripted_door_open`'s historical
/// "did this transition it" contract is preserved by its own wrapper (it
/// only ever requests `true`, where "already open" and "no-op" coincide with
/// the pre-#177 `false` return).
fn scripted_door_set_open(world: &mut World, entity: Entity, open: bool) -> bool {
    let placement = world
        .get::<PlacementRoot>(entity)
        .expect("caller resolved a placement root")
        .placement()
        .clone();
    let name = placement_name(&placement);
    let position = world
        .get::<GlobalTransform>(entity)
        .map(|transform| transform.translation())
        .unwrap_or_default();
    let was_open = world
        .get_resource_or_insert_with(InteractionState::default)
        .open
        .contains(&entity);
    if was_open == open {
        return open && !was_open;
    }
    let sound = {
        let mut state = world.get_resource_or_insert_with(InteractionState::default);
        if open {
            state.open.insert(entity);
            placement.audio.open_sound_form_id
        } else {
            state.open.remove(&entity);
            placement.audio.close_sound_form_id
        }
    };
    if let Some(form_id) = sound {
        world.write_message(PlaySound::at(form_id, position));
    }
    world.write_message(animation::PlayPlacementAnimation {
        root: entity,
        transition: if open {
            ClipTransition::Opening
        } else {
            ClipTransition::Closing
        },
        lead_ms: 0.0,
    });
    info!(
        "door {} ({:08x}) {} (scripted, nav agent)",
        name,
        placement.reference_form_id,
        if open { "opened" } else { "closed" }
    );
    open
}

/// F118.2: scripted corpse activation enters the same transfer modal as
/// player activation. A corpse is deliberately staged data in this slice;
/// no actor death or AI transition is implied by this function.
pub(crate) fn scripted_corpse_toggle(world: &mut World, entity: Entity) -> bool {
    let placement = world
        .get::<PlacementRoot>(entity)
        .expect("caller resolved a placement root")
        .placement()
        .clone();
    let name = placement_name(&placement);
    let position = world
        .get::<GlobalTransform>(entity)
        .map(|transform| transform.translation())
        .or_else(|| {
            world
                .get::<Transform>(entity)
                .map(|transform| transform.translation)
        })
        .unwrap_or_default();
    let opening = {
        let mut state = world.get_resource_or_insert_with(InteractionState::default);
        if state.open.contains(&entity) {
            state.open.remove(&entity);
            false
        } else {
            state.open.insert(entity);
            true
        }
    };

    if opening {
        let active_cell = world
            .get_resource::<ActiveCell>()
            .map(|cell| cell.0)
            .unwrap_or_default();
        let playthrough_seed = world
            .get_resource::<PlaythroughSeed>()
            .map(|seed| seed.0)
            .unwrap_or_default();
        let leveled_lists = if placement.inventory.iter().any(|entry| entry.leveled) {
            world
                .get_resource::<ResidentCells>()
                .and_then(|cells| cells.0.get(&active_cell))
                .map(|resident| leveled_lists_from_manifest(&resident.manifest.leveled_lists))
                .unwrap_or_default()
        } else {
            BTreeMap::new()
        };
        let resolved = {
            let mut states = world.get_resource_or_insert_with(ContainerStates::default);
            seed_loot_holder(
                &mut states,
                &placement,
                active_cell,
                playthrough_seed,
                &leveled_lists,
            )
        };
        world
            .get_resource_or_insert_with(ActiveContainerTarget::default)
            .0 = Some(ActiveContainer {
            kind: LootHolderKind::Corpse,
            entity,
            reference_form_id: placement.reference_form_id,
            name: name.clone(),
            item_names: container_item_names(&placement.inventory),
            owner_form_id: placement.owner_form_id,
        });
        if world.contains_resource::<Messages<PlaySound>>()
            && let Some(form_id) = placement.audio.open_sound_form_id
        {
            world.write_message(PlaySound::at(form_id, position));
        }
        if world.contains_resource::<Messages<animation::PlayPlacementAnimation>>() {
            world.write_message(animation::PlayPlacementAnimation {
                root: entity,
                transition: ClipTransition::Opening,
                lead_ms: 0.0,
            });
        }
        world
            .get_resource_or_insert_with(InteractionNotice::default)
            .show(format!("Looting {name}"));
        if world.contains_resource::<Messages<RequestStateTransition>>() {
            world.write_message(RequestStateTransition::Modal(GameplayModal::Container));
        }
        info!(
            "corpse {} ({:08x}) opened with {} stacks",
            name, placement.reference_form_id, resolved.0
        );
    } else {
        let active_target_matches = world
            .get_resource::<ActiveContainerTarget>()
            .and_then(|target| target.0.as_ref())
            .is_some_and(|target| target.entity == entity && target.kind == LootHolderKind::Corpse);
        if active_target_matches {
            world.resource_mut::<ActiveContainerTarget>().0 = None;
        }
        if world.contains_resource::<Messages<PlaySound>>()
            && let Some(form_id) = placement.audio.close_sound_form_id
        {
            world.write_message(PlaySound::at(form_id, position));
        }
        if world.contains_resource::<Messages<animation::PlayPlacementAnimation>>() {
            world.write_message(animation::PlayPlacementAnimation {
                root: entity,
                transition: ClipTransition::Closing,
                lead_ms: 0.0,
            });
        }
        world
            .get_resource_or_insert_with(InteractionNotice::default)
            .show(format!("Closed {name}"));
        if world
            .get_resource::<State<GameplayModal>>()
            .is_some_and(|state| *state.get() == GameplayModal::Container)
            && world.contains_resource::<Messages<RequestStateTransition>>()
        {
            world.write_message(RequestStateTransition::Modal(GameplayModal::None));
        }
        info!(
            "corpse {} ({:08x}) closed",
            name, placement.reference_form_id
        );
    }
    opening
}

/// Issue #84 (F84.2): scripted (console/BRP) pickup, mirroring
/// `activate_focused_placement`'s `Pickup` arm minus the raycast focus and
/// distance-state handling -- the runtime-item branch (inventory add plus
/// `ActiveSaveState` dropped-item cleanup), the prepared-item branch
/// (`PreparedItemCatalog` `max_condition` lookup for Weapon/Apparel stats),
/// the #81 steal classification log, the pickup sound, and the notice all
/// carry over verbatim. Returns the number of items added, or an error if a
/// runtime item's persistence resource is not ready.
pub(crate) fn scripted_pickup(
    world: &mut World,
    entity: Entity,
) -> Result<i32, ScriptedPickupError> {
    let placement = world
        .get::<PlacementRoot>(entity)
        .expect("caller resolved a placement root")
        .placement()
        .clone();
    let runtime_item = world
        .get::<super::super::world_items::RuntimeWorldItem>(entity)
        .copied();
    let position = world
        .get::<GlobalTransform>(entity)
        .map(|transform| transform.translation())
        .unwrap_or_default();
    let name = placement_name(&placement);
    let count = placement.count.max(1);

    if let Some(runtime_item) = runtime_item {
        if !world.contains_resource::<super::super::world::ActiveSaveState>() {
            return Err(ScriptedPickupError::PersistenceNotReady);
        }
        let stack = runtime_item.stack;
        let before = world.resource::<PlayerInventory>().legacy_snapshot();
        {
            let mut canonical = world.resource_mut::<CanonicalItemLedger>();
            canonical
                .ensure_runtime_item(runtime_item.runtime_id, runtime_item.cell_form_id, stack)
                .map_err(|_| ScriptedPickupError::ItemTransactionFailed)?;
            canonical
                .move_runtime_to_player(&before, runtime_item.runtime_id, runtime_item.cell_form_id)
                .map_err(|_| ScriptedPickupError::ItemTransactionFailed)?;
        }
        let _ = world.resource_mut::<PlayerInventory>().add_stack(stack);
        let mut save_state = world.resource_mut::<super::super::world::ActiveSaveState>();
        if let Some(cell) = save_state.0.cells.get_mut(&runtime_item.cell_form_id) {
            cell.dropped_items.remove(&runtime_item.runtime_id);
            if cell.references.is_empty() && cell.dropped_items.is_empty() {
                save_state.0.cells.remove(&runtime_item.cell_form_id);
            }
        }
        info!(
            "retrieved runtime item {} from cell {:08x}",
            runtime_item.runtime_id, runtime_item.cell_form_id
        );
    } else {
        let condition = world
            .get_resource::<PreparedItemCatalog>()
            .into_iter()
            .flat_map(|catalog| &catalog.items)
            .find(|item| item.base_form_id == placement.base_form_id)
            .and_then(|item| match &item.stats {
                PreparedItemStats::Weapon { max_condition, .. }
                | PreparedItemStats::Apparel { max_condition, .. } => *max_condition,
                _ => None,
            });
        let stack = InventoryStack {
            base_form_id: placement.base_form_id,
            count,
            condition,
        };
        let before = world.resource::<PlayerInventory>().legacy_snapshot();
        world
            .resource_mut::<CanonicalItemLedger>()
            .add_player_item(&before, stack)
            .map_err(|_| ScriptedPickupError::ItemTransactionFailed)?;
        let _ = world.resource_mut::<PlayerInventory>().add_stack(stack);
    }
    // Issue #81 (F81.4): picking up an owned reference is theft; no
    // crime/karma consequences in M3, only the stable log line.
    if let item_rules::TakeClassification::Steal { owner_form_id } =
        item_rules::classify_take(placement.owner_form_id)
    {
        info!(
            "steal {:08x} owner {:08x}",
            placement.base_form_id, owner_form_id
        );
    }
    if let Some(form_id) = placement.audio.pickup_sound_form_id {
        world.write_message(PlaySound::pickup_at(form_id, position));
    }
    world
        .get_resource_or_insert_with(InteractionNotice::default)
        .show(format!("Picked up {name} x{count}"));
    info!(
        "picked up {} x{} ({:08x}); inventory now has {}",
        name,
        count,
        placement.base_form_id,
        world
            .resource::<PlayerInventory>()
            .count(placement.base_form_id)
    );
    world
        .get_resource_or_insert_with(InteractionState::default)
        .open
        .remove(&entity);
    world.despawn(entity);
    Ok(count)
}

/// Failure mode for [`scripted_pickup`]: a runtime (player-dropped) item was
/// targeted before `ActiveSaveState` exists, so the dropped-item cleanup that
/// keeps persistence consistent cannot run yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScriptedPickupError {
    PersistenceNotReady,
    ItemTransactionFailed,
}
