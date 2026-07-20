//! Player-driven activation for pickups, loot holders, doors, and activators.

use super::*;

/// The #74 resolver's inputs, bundled so `activate_focused_placement` stays
/// under Bevy's 16-parameter system limit: which cell the roll happens in,
/// that cell's manifest (for the leveled-list bodies), and the playthrough
/// seed the roll derives from. All `Option` because they belong to the
/// `world`/`persist` slices — bare-App interaction tests run without them,
/// and a missing resource just resolves against defaults (cell 0, no list
/// bodies, seed 0).
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct LeveledResolveContext<'w> {
    active_cell: Option<Res<'w, ActiveCell>>,
    resident_cells: Option<Res<'w, ResidentCells>>,
    playthrough_seed: Option<Res<'w, PlaythroughSeed>>,
}

/// Bundles the container-open resources so `activate_focused_placement`
/// stays under Bevy's 16-parameter system limit alongside wave-1's dropped
/// item / catalog params (see `PickupContext` and `LeveledResolveContext`
/// above for the same reasoning).
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct ContainerActivation<'w> {
    states: ResMut<'w, ContainerStates>,
    active: ResMut<'w, ActiveContainerTarget>,
    modal_requests: MessageWriter<'w, RequestStateTransition>,
}

/// Bundles wave-1's dropped-item retrieval resources (save-state removal,
/// catalog condition lookup) for the same 16-parameter reason as
/// `ContainerActivation`.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct PickupContext<'w> {
    save_state: Option<ResMut<'w, super::super::world::ActiveSaveState>>,
    catalog: Option<Res<'w, PreparedItemCatalog>>,
}

/// Converts the manifest's leveled-list bodies into `leveled`'s std-only
/// mirror types (the same mirror pattern `persist_policy` uses toward
/// `save`; see `leveled`'s module doc).
pub(super) fn leveled_lists_from_manifest(
    lists: &BTreeMap<u32, PreparedLeveledList>,
) -> BTreeMap<u32, leveled::PreparedLeveledList> {
    lists
        .iter()
        .map(|(form_id, list)| {
            (
                *form_id,
                leveled::PreparedLeveledList {
                    chance_none: list.chance_none,
                    flags: list.flags,
                    entries: list
                        .entries
                        .iter()
                        .map(|entry| leveled::PreparedLeveledEntry {
                            level: entry.level,
                            base_form_id: entry.base_form_id,
                            count: entry.count,
                        })
                        .collect(),
                },
            )
        })
        .collect()
}

/// Seeds one FormID-keyed loot holder through the existing container policy.
/// The returned pair is (stack_count, stacks) for deterministic scripted logs.
pub(super) fn seed_loot_holder(
    states: &mut ContainerStates,
    placement: &PreparedPlacement,
    active_cell: u32,
    playthrough_seed: u64,
    leveled_lists: &BTreeMap<u32, leveled::PreparedLeveledList>,
) -> (usize, Vec<(u32, i32)>) {
    let seed_entries: Vec<container_policy::SeedEntry> = placement
        .inventory
        .iter()
        .map(|entry| container_policy::SeedEntry {
            base_form_id: entry.base_form_id,
            count: entry.count,
            leveled: entry.leveled,
        })
        .collect();
    let seed =
        leveled::LeveledSeed::derive(playthrough_seed, active_cell, placement.reference_form_id);
    let resolved = states.open(placement.reference_form_id, &seed_entries, |list_form_id| {
        leveled::resolve_leveled(list_form_id, leveled_lists, seed, PLAYER_LEVEL)
    });
    (resolved.stacks.len(), resolved.stacks.clone())
}
#[allow(clippy::too_many_arguments)]
pub(super) fn activate_focused_placement(
    keys: Res<ButtonInput<KeyCode>>,
    mode: Res<CameraModeState>,
    mut commands: Commands,
    roots: Query<(
        &PlacementRoot,
        &GlobalTransform,
        Option<&super::super::world_items::RuntimeWorldItem>,
    )>,
    animated: Query<&animation::AnimatedPlacement>,
    mut inventory: ResMut<PlayerInventory>,
    mut canonical: ResMut<CanonicalItemLedger>,
    mut state: ResMut<InteractionState>,
    mut notice: ResMut<InteractionNotice>,
    mut sounds: MessageWriter<PlaySound>,
    mut door_travel: MessageWriter<DoorTravelRequested>,
    mut animation_playback: MessageWriter<animation::PlayPlacementAnimation>,
    mut pending_travel: ResMut<PendingDoorTravel>,
    mut container_activation: ContainerActivation,
    resolve_context: LeveledResolveContext,
    mut pickup_context: PickupContext,
) {
    if mode.mode != CameraMode::Fps || !keys.just_pressed(KeyCode::KeyE) {
        return;
    }
    let Some(entity) = state.focused else {
        return;
    };
    let Ok((root, transform, runtime_item)) = roots.get(entity) else {
        state.focused = None;
        return;
    };
    let placement = &root.placement;
    let position = transform.translation();
    let name = placement_name(placement);

    match &placement.semantic {
        PreparedSemantic::Pickup(_) => {
            let count = placement.count.max(1);
            if let Some(runtime_item) = runtime_item {
                let Some(save_state) = pickup_context.save_state.as_mut() else {
                    notice.show("Dropped-item persistence is not ready");
                    return;
                };
                let before = inventory.legacy_snapshot();
                if canonical
                    .ensure_runtime_item(
                        runtime_item.runtime_id,
                        runtime_item.cell_form_id,
                        runtime_item.stack,
                    )
                    .and_then(|_| {
                        canonical.move_runtime_to_player(
                            &before,
                            runtime_item.runtime_id,
                            runtime_item.cell_form_id,
                        )
                    })
                    .is_err()
                {
                    notice.show("Item transaction failed while retrieving the dropped item");
                    return;
                }
                let _ = inventory.add_stack(runtime_item.stack);
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
                let condition = pickup_context
                    .catalog
                    .as_deref()
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
                let before = inventory.legacy_snapshot();
                if canonical.add_player_item(&before, stack).is_err() {
                    notice.show("Item transaction failed while picking up the item");
                    return;
                }
                let _ = inventory.add_stack(stack);
            }
            // Issue #81 (F81.4): picking up an owned reference is theft; no
            // crime/karma consequences in M3, only the stable log line.
            // Player-dropped runtime items carry no owner.
            if let item_rules::TakeClassification::Steal { owner_form_id } =
                item_rules::classify_take(placement.owner_form_id)
            {
                info!(
                    "steal {:08x} owner {:08x}",
                    placement.base_form_id, owner_form_id
                );
            }
            write_pickup_sound(&mut sounds, placement.audio.pickup_sound_form_id, position);
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
        PreparedSemantic::Container | PreparedSemantic::Corpse => {
            // Issue #75 (F75.2): activation now opens the paused transfer
            // modal rather than toggling a notice in place. `E` on an
            // already-open container is unreachable in practice --
            // `activate_focused_placement` only runs in `GameplayModal::None`
            // (see `install`), and opening a container leaves that state --
            // but a leftover `open` marker (e.g. restored by a future
            // persistence apply before the modal system observes it) is
            // guarded against rather than trusted.
            if state.open.contains(&entity) {
                return;
            }
            let active_cell = resolve_context
                .active_cell
                .as_ref()
                .map(|cell| cell.0)
                .unwrap_or_default();
            let leveled_lists = if placement.inventory.iter().any(|entry| entry.leveled) {
                resolve_context
                    .resident_cells
                    .as_ref()
                    .and_then(|cells| cells.0.get(&active_cell))
                    .map(|resident| leveled_lists_from_manifest(&resident.manifest.leveled_lists))
                    .unwrap_or_default()
            } else {
                BTreeMap::new()
            };
            let stack_data = seed_loot_holder(
                &mut container_activation.states,
                placement,
                active_cell,
                resolve_context
                    .playthrough_seed
                    .as_ref()
                    .map(|seed| seed.0)
                    .unwrap_or_default(),
                &leveled_lists,
            );
            state.open.insert(entity);
            container_activation.active.0 = Some(ActiveContainer {
                kind: match &placement.semantic {
                    PreparedSemantic::Container => LootHolderKind::Container,
                    PreparedSemantic::Corpse => LootHolderKind::Corpse,
                    _ => unreachable!("guarded loot-holder activation"),
                },
                entity,
                reference_form_id: placement.reference_form_id,
                name: name.clone(),
                item_names: container_item_names(&placement.inventory),
                owner_form_id: placement.owner_form_id,
            });
            write_container_sound(&mut sounds, placement.audio.open_sound_form_id, position);
            animation_playback.write(animation::PlayPlacementAnimation {
                root: entity,
                transition: ClipTransition::Opening,
                lead_ms: 0.0,
            });
            container_activation
                .modal_requests
                .write(RequestStateTransition::Modal(GameplayModal::Container));
            info!(
                "{} {} ({:08x}) opened with {} stacks",
                match &placement.semantic {
                    PreparedSemantic::Container => "container",
                    PreparedSemantic::Corpse => "corpse",
                    _ => unreachable!("guarded loot-holder activation"),
                },
                name,
                placement.reference_form_id,
                stack_data.0
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
            // Issue #186: the open-state toggle goes through the shared
            // blocker signal (`InteractionState::toggle_open`), the same one
            // the Activator arm below uses -- a door is one blocker
            // *behaviour*, not the sole record type allowed to record open
            // state (verdict §2.2).
            let opening = state.toggle_open(entity);
            let transition = if opening {
                write_sound(&mut sounds, placement.audio.open_sound_form_id, position);
                ClipTransition::Opening
            } else {
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
                    door_form_id: placement.reference_form_id,
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
            // Issue #186 / verdict §2.1: a solid activator (vault gear door,
            // blast door) is a capsule blocker, so its runtime open/close
            // state is route topology nav reads from `InteractionState.open`
            // -- exactly like a door's. Its record *type* selects only the
            // *behaviour*: it opens and closes freely, with no key and no
            // travel destination. The open-state population itself is the
            // shared blocker signal (`toggle_open`), not a door-specific one,
            // so this can never again animate open in the world while nav
            // still models it shut. Before this the arm played `Opening`
            // unconditionally and never touched `open`, holding the gear
            // door's polygons at `INFINITY` forever.
            let opening = state.toggle_open(entity);
            let (sound, transition) = if opening {
                (
                    placement
                        .audio
                        .open_sound_form_id
                        .or(placement.audio.activate_sound_form_id),
                    ClipTransition::Opening,
                )
            } else {
                (
                    placement
                        .audio
                        .close_sound_form_id
                        .or(placement.audio.activate_sound_form_id),
                    ClipTransition::Closing,
                )
            };
            write_sound(&mut sounds, sound, position);
            notice.show(format!(
                "{} {name}",
                if opening { "Opened" } else { "Closed" }
            ));
            info!(
                "activator {} ({:08x}) {}",
                name,
                placement.reference_form_id,
                if opening { "opened" } else { "closed" }
            );
            animation_playback.write(animation::PlayPlacementAnimation {
                root: entity,
                transition,
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

fn write_pickup_sound(sounds: &mut MessageWriter<PlaySound>, form_id: Option<u32>, position: Vec3) {
    if let Some(form_id) = form_id {
        sounds.write(PlaySound::pickup_at(form_id, position));
    }
}

fn write_container_sound(
    sounds: &mut MessageWriter<PlaySound>,
    form_id: Option<u32>,
    position: Vec3,
) {
    if let Some(form_id) = form_id {
        sounds.write(PlaySound::container_at(form_id, position));
    }
}
