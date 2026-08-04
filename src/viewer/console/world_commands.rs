//! World-reference activation and actor-debug console commands.

use super::*;

pub(super) struct WorldCommandProvider;

impl ConsoleCommandProvider for WorldCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new("actorinspect", "actorinspect <actor-reference>", "Report a prepared actor's identity, assembly, animation, fallback tier/reasons, scale, canonical holder, proxy, and weapon attachment state.", actor_inspect),
            ConsoleCommand::new("actoranim", "actoranim <actor-reference> <idle|walk|run|turn_left|turn_right|equip|unequip>", "Request a gameplay animation through the same actor controller used by AI and scripted behavior.", actor_animation).mutating(),
            ConsoleCommand::new("playidle", "playidle <actor-reference> <idle-formid>", "Force a prepared Special Idle or Whole Body authored IDLE through the actor's loaded shared animation set.", play_idle).mutating(),
            ConsoleCommand::new("ragdoll", "ragdoll <actor-reference> [on|off|reset]", "Toggle a prepared NPC/creature's developer ragdoll body; actors stay locked in T-pose by default.", ragdoll).mutating(),
            ConsoleCommand::new("ragdollprobe", "ragdollprobe <actor-reference>", "Report live ragdoll constraint, velocity, sleep, and visual-node errors without changing the actor.", ragdoll_probe),
            ConsoleCommand::new("activate", "activate <reference>", "Activate a door, container, corpse, or pickup reference; a door with a destination requests cell travel (locks bypassed).", activate_reference).mutating(),
            ConsoleCommand::new(
                "setlock",
                "setlock <reference> <level> [<key-formid>|none]",
                "Set (positive level) or clear (0) a door reference's lock level at runtime (GECK lock/unlock parity); updates both the player-facing activation check and nav route planning. An optional third argument sets (or, as \"none\", clears) the door's key FormID requirement -- issue #185: authored FO3 doors rarely carry one, so this is what makes key-aware AI door routing testable without a real keyed door.",
                setlock,
            )
            .mutating(),
            ConsoleCommand::new(
                "tp",
                "tp <x> <y> <z> [<cell-formid>]",
                "Atomically teleport the player to (x, y, z) in metres, optionally after swapping to a prepared cell first.",
                teleport_player,
            )
            .mutating(),
            ConsoleCommand::new(
                "worldstream",
                "worldstream <status|cells|water|presentation|summary|trace 0|1>",
                "Inspect prepared exterior residency and presentation state.",
                worldstream,
            ),
            ConsoleCommand::new(
                "worldlocation",
                "worldlocation",
                "Report the exact saved player location and its interior/exterior owner.",
                world_location,
            ),
            ConsoleCommand::new(
                "worldstate",
                "worldstate <formid>",
                "Report canonical persisted deltas for one cell or exterior cell FormID.",
                world_state,
            ),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

fn world_location(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let location = world
        .get_resource::<super::super::world::CurrentWorldLocation>()
        .and_then(|location| location.0.as_ref())
        .map(serde_json::to_value)
        .transpose()
        .map_err(|error| ConsoleError::new("serialize", error.to_string()))?
        .unwrap_or(serde_json::Value::Null);
    Ok(ConsoleCommandResult::value(json!({
        "location": location,
        "cell_key": world
            .get_resource::<super::super::world::CurrentWorldLocation>()
            .and_then(|location| location.0.as_ref())
            .map(bevyout_core::manifest::exterior::WorldLocation::cell_key),
    })))
}

fn world_state(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [form_id] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "worldstate expects exactly one FormID",
        ));
    };
    let form_id = parse_item_form_id(form_id)
        .ok_or_else(|| ConsoleError::new("bad_form_id", "worldstate FormID must be hexadecimal"))?;
    let cell = world
        .get_resource::<super::super::world::ActiveSaveState>()
        .and_then(|state| state.0.cells.get(&form_id));
    Ok(ConsoleCommandResult::value(json!({
        "cell_form_id": format!("{form_id:08x}"),
        "present": cell.is_some(),
        "references": cell.map_or(0, |cell| cell.references.len()),
        "dropped_items": cell.map_or(0, |cell| cell.dropped_items.len()),
        "actors": cell.map_or(0, |cell| cell.actors.len()),
    })))
}

fn worldstream(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    match invocation.args.as_slice() {
        [] => {
            if world
                .get_resource::<super::super::world::exterior::ExteriorStreamState>()
                .is_none()
            {
                return Err(ConsoleError::new(
                    "unavailable",
                    "exterior streaming is not installed",
                ));
            }
            Ok(ConsoleCommandResult::value(
                super::super::world::exterior::exterior_status_json(world),
            ))
        }
        [command] if command == "status" => {
            if world
                .get_resource::<super::super::world::exterior::ExteriorStreamState>()
                .is_none()
            {
                return Err(ConsoleError::new(
                    "unavailable",
                    "exterior streaming is not installed",
                ));
            }
            Ok(ConsoleCommandResult::value(
                super::super::world::exterior::exterior_status_json(world),
            ))
        }
        [command] if command == "cells" => {
            let state = world
                .get_resource::<super::super::world::exterior::ExteriorStreamState>()
                .ok_or_else(|| {
                    ConsoleError::new("unavailable", "exterior streaming is not installed")
                })?;
            Ok(ConsoleCommandResult::value(
                super::super::world::exterior::exterior_cells_json(state),
            ))
        }
        [command] if command == "water" => {
            let water = world
                .get_resource::<super::super::world::exterior::ExteriorWaterState>()
                .ok_or_else(|| {
                    ConsoleError::new("unavailable", "exterior water is not installed")
                })?;
            Ok(ConsoleCommandResult::value(serde_json::json!({
                "contact": water.contact,
            })))
        }
        [command] if command == "presentation" => Ok(ConsoleCommandResult::value(
            super::super::world::exterior::exterior_presentation_json(world),
        )),
        [command] if command == "summary" => {
            Ok(ConsoleCommandResult::value(exterior_route_summary(world)))
        }
        [command, value] if command == "trace" && matches!(value.as_str(), "0" | "1") => {
            let enabled = value == "1";
            world
                .resource_mut::<super::super::world::exterior::ExteriorStreamState>()
                .trace = enabled;
            let streaming = super::super::world::exterior::exterior_status_json(world);
            Ok(ConsoleCommandResult::new(
                serde_json::json!({
                    "trace": enabled,
                    "streaming": streaming,
                }),
                vec![format!(
                    "exterior stream trace {}",
                    if enabled { "on" } else { "off" }
                )],
            ))
        }
        _ => Err(ConsoleError::new(
            "bad_args",
            "worldstream expects status, cells, water, presentation, summary, or trace 0|1",
        )),
    }
}

fn exterior_route_summary(world: &mut World) -> serde_json::Value {
    super::super::diagnostics::convergence_report(world)
}

pub(super) fn actor_inspect(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "actorinspect requires exactly one actor reference",
        ));
    };
    let entity = resolve_reference(world, selector)?;
    let placement = world
        .get::<interaction::PlacementRoot>(entity)
        .ok_or_else(|| ConsoleError::new("not_actor", "reference has no placement root"))?
        .placement()
        .clone();
    let (semantic_kind, prepared) = match &placement.semantic {
        PreparedSemantic::Npc(prepared) => ("humanoid", prepared),
        PreparedSemantic::Creature(prepared) => ("creature", prepared),
        _ => {
            return Err(ConsoleError::new(
                "not_actor",
                "actorinspect only accepts NPC or creature references",
            ));
        }
    };
    let runtime = world.get::<actor::ActorRuntime>(entity).cloned();
    let runtime_state = world.get::<actor::ActorRuntimeState>(entity).cloned();
    let animation_runtime = world.get::<actor_animation::ActorAnimationRuntime>(entity);
    let assembly = runtime
        .as_ref()
        .and_then(|runtime| runtime.assembly.as_ref())
        .or(prepared.assembly.as_ref());

    let (source_base_form_id, prepared_base_form_id, prepared_kind, root_scale) =
        if let Some(assembly) = assembly {
            (
                assembly.source_base_form_id,
                assembly.resolved_base_form_id,
                match assembly.kind {
                    bevyout_core::actor::ActorKind::Humanoid => "humanoid",
                    bevyout_core::actor::ActorKind::Creature => "creature",
                },
                assembly.root_scale,
            )
        } else {
            (
                placement.base_form_id,
                placement.base_form_id,
                semantic_kind,
                placement.scale,
            )
        };
    let resolved_base_form_id = runtime
        .as_ref()
        .map_or(prepared_base_form_id, |runtime| runtime.base_form_id);
    let kind = runtime
        .as_ref()
        .map_or(prepared_kind, |runtime| match runtime.kind {
            bevyout_core::actor::ActorKind::Humanoid => "humanoid",
            bevyout_core::actor::ActorKind::Creature => "creature",
        });

    let fallback = assembly.map_or_else(
        || {
            json!({
                "tier": "missing_blueprint",
                "facegen_policy": null,
                "proxy_kind": if placement.asset_path.is_none() { "Bounds" } else { "None" },
                "reasons": [{
                    "code": "missing_actor_blueprint",
                    "detail": "prepared actor has no assembly blueprint",
                }],
            })
        },
        |assembly| {
            json!({
                "tier": assembly.fallback.level.label(),
                "facegen_policy": assembly.fallback.facegen_policy.label(),
                "proxy_kind": assembly.fallback.proxy_kind.label(),
                "reasons": assembly.fallback.reasons.iter().map(|reason| json!({
                    "code": reason.code(),
                    "detail": format!("{reason:?}"),
                })).collect::<Vec<_>>(),
            })
        },
    );
    let parts = assembly.map_or_else(Vec::new, |assembly| {
        assembly
            .mesh_parts
            .iter()
            .map(|part| {
                json!({
                    "role": part.role.label(),
                    "source_form_id": part.source_form_id,
                    "model_path": part.model_path,
                    "attachment_point": format!("{:?}", part.attachment_point),
                    "visible": part.is_visible,
                })
            })
            .collect::<Vec<_>>()
    });
    let apparel = assembly.map_or_else(Vec::new, |assembly| {
        assembly
            .apparel
            .iter()
            .map(|item| {
                json!({
                    "item_form_id": item.item_form_id,
                    "model_path": item.model_path,
                    "model_available": item.model_available,
                    "biped_slot_mask": item.biped_slot_mask,
                })
            })
            .collect::<Vec<_>>()
    });
    let eyes = assembly.map(|assembly| {
        json!({
            "form_id": assembly.eye_form_id,
            "texture_path": assembly.eye_texture_path,
            "geometry_parts": assembly.mesh_parts.iter().filter(|part| {
                matches!(part.role, bevyout_core::actor::ActorMeshRole::Eyes)
            }).count(),
        })
    });
    let facegen_reconstruction = assembly.map_or_else(
        || {
            json!({
                "status": "NotAuthored",
                "geometry_status": "NotAuthored",
                "texture_status": "NotAuthored",
                "fingerprint": null,
                "diagnostics": [],
            })
        },
        |assembly| {
            let status = match assembly.fallback.facegen_policy {
                bevyout_core::actor::FaceGenPolicy::Authored if assembly.facegen.is_some() => {
                    "Applied"
                }
                bevyout_core::actor::FaceGenPolicy::RestPoseFallback => "RestPoseFallback",
                _ => "NotAuthored",
            };
            json!({
                "status": status,
                "geometry_status": status,
                "texture_status": status,
                "fingerprint": assembly.facegen_reconstruction_fingerprint,
                "diagnostics": assembly.facegen_diagnostics.iter().map(|diagnostic| json!({
                    "code": diagnostic.code(),
                    "detail": diagnostic.to_string(),
                })).collect::<Vec<_>>(),
            })
        },
    );
    let prepared_weapon = assembly
        .and_then(|assembly| assembly.equipped_weapon.as_ref())
        .map(|weapon| {
            json!({
                "item_form_id": weapon.item_form_id,
                "model_path": weapon.model_path,
                "model_available": weapon.model_available,
                "attachment_point": format!("{:?}", weapon.attachment_point),
            })
        });
    let weapon_runtime = match runtime_state.as_ref().map(|state| &state.weapon) {
        None => json!({ "status": "not_projected" }),
        Some(actor::ActorWeaponRuntimeState::Attached { entity, node }) => {
            let visual = world.get::<actor::ActorWeaponVisual>(*entity);
            json!({
                "status": "attached",
                "entity": entity.to_bits(),
                "node": node,
                "item_form_id": visual.map(|visual| visual.item_form_id),
                "actor_reference_form_id": visual.map(|visual| visual.actor_reference_form_id),
            })
        }
        Some(actor::ActorWeaponRuntimeState::MissingAttachmentNode { expected }) => json!({
            "status": "missing_attachment_node",
            "expected": expected,
        }),
        Some(state) => json!({ "status": state.label() }),
    };
    let diagnostics = runtime_state.as_ref().map_or_else(Vec::new, |state| {
        state
            .diagnostics
            .iter()
            .map(|diagnostic| {
                json!({
                    "code": diagnostic.code,
                    "message": diagnostic.message,
                })
            })
            .collect::<Vec<_>>()
    });
    let canonical_holder = runtime_state.as_ref().map_or_else(
        || json!({ "present": false, "items": [], "equipped_instance_id": null }),
        |state| {
            let canonical = world.get_resource::<interaction::CanonicalItemLedger>();
            let holder_state = canonical
                .and_then(|canonical| canonical.ledger.holders().get(&state.holder));
            let equipped = canonical
                .and_then(|canonical| canonical.ledger.bindings().get(&state.holder))
                .and_then(|bindings| bindings.equipped);
            json!({
                "present": holder_state.is_some(),
                "items": holder_state.map_or_else(Vec::new, |holder| holder.items.iter().map(|item| json!({
                    "instance_id": item.id.0,
                    "base_form_id": item.base_form_id,
                    "count": item.count,
                    "equipped": Some(item.id) == equipped,
                })).collect::<Vec<_>>()),
                "equipped_instance_id": equipped.map(|item| item.0),
            })
        },
    );
    let idle_state = animation_runtime.map_or_else(
        || {
            json!({
                "source": "idle_manager",
                "selected_form_id": null,
                "current_form_id": null,
                "last_form_id": null,
                "next_evaluation_seconds": 0.0,
                "cooldowns": {},
                "collection": {
                    "package_form_id": null,
                    "cursor": 0,
                    "do_once_exhausted": false,
                },
                "last_rejection": null,
            })
        },
        |runtime| {
            let authority = runtime.idle_authority();
            let cooldowns = authority
                .replay_cooldowns
                .iter()
                .map(|(form_id, eligible_at)| (format!("{form_id:08x}"), json!(eligible_at)))
                .collect::<serde_json::Map<_, _>>();
            json!({
                "source": runtime
                    .special_idle_source()
                    .unwrap_or(authority.source)
                    .label(),
                "selected_form_id": authority.current_idle_form_id,
                "current_form_id": runtime.special_idle_form_id(),
                "last_form_id": authority.last_idle_form_id,
                "next_evaluation_seconds": authority.next_eligible_evaluation_seconds,
                "cooldowns": cooldowns,
                "collection": {
                    "package_form_id": authority.active_package_form_id,
                    "cursor": authority.package_collection_cursor,
                    "do_once_exhausted": authority.do_once_exhausted,
                },
                "last_rejection": authority.last_rejection.map(|reason| reason.label()),
            })
        },
    );
    let fallback_tier = assembly.map_or("missing_blueprint", |assembly| {
        assembly.fallback.level.label()
    });
    let reason_codes = assembly.map_or_else(
        || vec!["missing_actor_blueprint".to_owned()],
        |assembly| {
            assembly
                .fallback
                .reasons
                .iter()
                .map(|reason| reason.code().to_owned())
                .collect::<Vec<_>>()
        },
    );
    let summary = format!(
        "actorinspect {:08x} base={:08x} kind={kind} tier={fallback_tier} scale={root_scale:.4} parts={} apparel={} weapon={} reasons={}",
        placement.reference_form_id,
        resolved_base_form_id,
        parts.len(),
        apparel.len(),
        prepared_weapon
            .as_ref()
            .and_then(|weapon| weapon["item_form_id"].as_u64())
            .map_or_else(|| "none".to_owned(), |form_id| format!("{form_id:08x}")),
        if reason_codes.is_empty() {
            "none".to_owned()
        } else {
            reason_codes.join(",")
        },
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": placement.reference_form_id,
            "source_base_form_id": source_base_form_id,
            "base_form_id": resolved_base_form_id,
            "kind": kind,
            "scale": root_scale,
            "skeleton_path": assembly.and_then(|assembly| assembly.skeleton_path.as_ref()),
            "parts": parts,
            "eyes": eyes,
            "facegen": facegen_reconstruction,
            "apparel": apparel,
            "weapon": {
                "prepared": prepared_weapon,
                "runtime": {
                    "state": weapon_runtime,
                    "bound_item_form_id": runtime_state.as_ref().and_then(|state| state.bound_item_form_id),
                    "model_path": runtime_state.as_ref().and_then(|state| state.weapon_model.as_ref()).map(|model| model.model_path.as_str()),
                },
            },
            "fallback": fallback,
            "idle_source": idle_state["source"].clone(),
            "selected_idle_form_id": idle_state["selected_form_id"].clone(),
            "current_idle_form_id": idle_state["current_form_id"].clone(),
            "last_idle_rejection": idle_state["last_rejection"].clone(),
            "idle": idle_state.clone(),
            "animation": {
                "present": animation_runtime.is_some(),
                "set_id": animation_runtime.map(|runtime| runtime.set_id()),
                "state": animation_runtime.map(|runtime| runtime.state_label()),
                "clip": animation_runtime.and_then(|runtime| runtime.clip_name()),
                "bound_targets": animation_runtime.map(|runtime| runtime.bound_target_count()),
                "loop_mode": animation_runtime.map(|runtime| format!("{:?}", runtime.loop_mode())),
                "root_motion_policy": animation_runtime.map(|runtime| format!("{:?}", runtime.root_motion_policy())),
                "diagnostic": animation_runtime.and_then(|runtime| runtime.diagnostic()),
                "idle": idle_state,
            },
            "runtime": {
                "present": runtime.is_some(),
                "holder_seeded": runtime_state.as_ref().is_some_and(|state| state.holder_seeded),
                "holder": runtime_state.as_ref().map(|state| format!("{:?}", state.holder)),
                "canonical": canonical_holder,
                "proxy_entity": runtime_state.as_ref().and_then(|state| state.proxy_entity).map(|entity| entity.to_bits()),
                "diagnostics": diagnostics,
            },
        }),
        vec![summary],
    ))
}

pub(super) fn actor_animation(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector, state] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "actoranim requires an actor reference and animation state",
        ));
    };
    let entity = resolve_reference(world, selector)?;
    let reference_form_id = world
        .get::<actor::ActorRuntime>(entity)
        .map(|runtime| runtime.reference_form_id)
        .ok_or_else(|| ConsoleError::new("not_actor", "reference is not a projected actor"))?;
    let state = actor_animation::policy::ActorAnimationState::parse(state).ok_or_else(|| {
        ConsoleError::new(
            "bad_value",
            "animation state must be idle, walk, run, turn_left, turn_right, equip, or unequip",
        )
    })?;
    actor_animation::request_actor_animation(world, entity, state)
        .map_err(|error| ConsoleError::new("animation_unavailable", error.to_string()))?;
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": reference_form_id,
            "requested_state": state.label(),
        }),
        vec![format!(
            "actoranim {reference_form_id:08x} {}",
            state.label()
        )],
    ))
}

pub(super) fn play_idle(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector, idle_form_id] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "playidle requires an actor reference and IDLE FormID",
        ));
    };
    let entity = resolve_reference(world, selector).map_err(|_| {
        ConsoleError::new(
            "unknown_actor",
            format!("unknown actor reference {selector}"),
        )
    })?;
    let idle_form_id = parse_item_form_id(idle_form_id).ok_or_else(|| {
        ConsoleError::new("bad_form_id", "playidle IDLE FormID must be hexadecimal")
    })?;
    let reference_form_id = actor_animation::request_forced_idle(world, entity, idle_form_id)
        .map_err(|error| ConsoleError::new(error.code, error.message))?;
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": reference_form_id,
            "idle_form_id": format!("{idle_form_id:08x}"),
            "source": "forced",
            "queued": true,
        }),
        vec![format!(
            "playidle {reference_form_id:08x} {idle_form_id:08x} queued"
        )],
    ))
}

/// Scripted door activation for the agent bridge (M2 wave 2 acceptance):
/// resolves a door reference and requests the same `DoorTravelRequested`
/// cell travel that the player's Enter activation produces, skipping the
/// raycast focus, distance, and lock checks (this is a developer command).
pub(super) fn activate_reference(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "activate requires exactly one reference selector",
        ));
    };
    let entity = resolve_reference(world, selector)?;
    let placement = world
        .get::<interaction::PlacementRoot>(entity)
        .ok_or_else(|| ConsoleError::new("not_activatable", "reference has no placement root"))?
        .placement()
        .clone();
    // Wave-4 amendment: containers toggle through the same open-state and
    // clip path as player activation, so the #60/#61 persistence gate can
    // be driven over the agent bridge.
    if matches!(placement.semantic, PreparedSemantic::Corpse) {
        let opened = interaction::scripted_corpse_toggle(world, entity);
        return Ok(ConsoleCommandResult::new(
            json!({
                "reference_form_id": placement.reference_form_id,
                "kind": "corpse",
                "opened": opened,
            }),
            vec![format!(
                "corpse {:08x} {}",
                placement.reference_form_id,
                if opened { "opened" } else { "closed" }
            )],
        ));
    }
    if matches!(placement.semantic, PreparedSemantic::Container) {
        let opened = interaction::scripted_container_toggle(world, entity);
        return Ok(ConsoleCommandResult::new(
            json!({
                "reference_form_id": placement.reference_form_id,
                "opened": opened,
            }),
            vec![format!(
                "container {:08x} {}",
                placement.reference_form_id,
                if opened { "opened" } else { "closed" }
            )],
        ));
    }
    // Issue #84 (F84.2): pickups activate through the same seam as door and
    // container references, mirroring the player's `E` pickup minus the
    // raycast focus and distance-state handling.
    if matches!(placement.semantic, PreparedSemantic::Pickup(_)) {
        let count = interaction::scripted_pickup(world, entity).map_err(|error| match error {
            interaction::ScriptedPickupError::PersistenceNotReady => ConsoleError::new(
                "persistence_not_ready",
                "dropped-item persistence is not ready",
            ),
            interaction::ScriptedPickupError::ItemTransactionFailed => ConsoleError::new(
                "item_transaction_failed",
                "canonical item transaction could not be committed",
            ),
        })?;
        return Ok(ConsoleCommandResult::new(
            json!({
                "reference_form_id": placement.reference_form_id,
                "base_form_id": placement.base_form_id,
                "count": count,
            }),
            vec![format!("picked up {:08x} x{count}", placement.base_form_id)],
        ));
    }
    // Issue #186: a solid activator blocker (vault gear door, blast door) is
    // openable route topology, so `activate` drives its open/close state
    // through the same scripted boundary the nav crossing gate observes --
    // the same human-testable parity #177 gave ordinary in-cell doors. This
    // is the activator *behaviour* (opens freely, no key, no travel), not a
    // door-record special case: the open-state signal it toggles is the
    // shared `InteractionState.open` nav reads for every blocker (verdict
    // §2.1/§2.2).
    if matches!(placement.semantic, PreparedSemantic::Activator) {
        let opened = interaction::scripted_activator_toggle(world, entity);
        return Ok(ConsoleCommandResult::new(
            json!({
                "reference_form_id": placement.reference_form_id,
                "kind": "activator",
                "opened": opened,
            }),
            vec![format!(
                "activator {:08x} {}",
                placement.reference_form_id,
                if opened { "opened" } else { "closed" }
            )],
        ));
    }
    let PreparedSemantic::Door(door) = &placement.semantic else {
        return Err(ConsoleError::new(
            "not_a_door",
            "activate supports only door, activator, container, corpse, and pickup references",
        ));
    };
    // Issue #177: an ordinary in-cell door has no travel destination, and
    // before this it had no open mechanism anywhere in the runtime -- neither
    // this command nor the nav door link, both of which were wired only to
    // travel doors. It now toggles through the same scripted boundary the nav
    // agent's crossing gate uses, so a human (and the manual acceptance
    // script) can drive an interior door directly.
    let Some(destination) = &door.destination else {
        let opened = interaction::scripted_door_toggle(world, entity);
        return Ok(ConsoleCommandResult::new(
            json!({
                "reference_form_id": placement.reference_form_id,
                "kind": "door",
                "opened": opened,
            }),
            vec![format!(
                "door {:08x} {}",
                placement.reference_form_id,
                if opened { "opened" } else { "closed" }
            )],
        ));
    };
    // Wave-3 amendment: route through the same Open-clip lead as player
    // activation so BRP-driven travel animates instead of teleporting.
    let open_lead_ms = interaction::scripted_door_travel(
        world,
        entity,
        interaction::DoorTravelRequested {
            destination_cell_form_id: destination.cell_form_id,
            translation: Vec3::from_array(destination.translation),
            rotation_xyzw: destination.rotation_xyzw,
            door_form_id: placement.reference_form_id,
        },
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": placement.reference_form_id,
            "destination_cell_form_id": destination.cell_form_id,
            "open_lead_ms": open_lead_ms,
        }),
        vec![format!(
            "travel requested to cell {:08x} (open lead {open_lead_ms:.0} ms)",
            destination.cell_form_id
        )],
    ))
}

/// Issue #163: GECK-parity `setlock`/`unlock` console command. A locked
/// in-cell acceptance door didn't exist in either prepared acceptance cell
/// (#155's manual script gap), so this is the runtime surface that flips a
/// door's lock level for both consumers from one call site, matching
/// `interaction::door_is_locked`/`nav::agent::door_open_and_locked`'s shared
/// "level <= 0 or absent is unlocked" rule:
///  - `interaction::PlacementRoot::set_door_lock_level` -- the same
///    component the player's own E-activation reads (`door_is_locked`), so
///    the very next activation attempt sees the change.
///  - `nav::agent::set_door_lock_level` -- `NavArchipelagoState`'s
///    `door_lock_info` snapshot `door_availability_system` polls every
///    frame, so a route through a newly locked door is excluded (or a
///    newly unlocked one re-included) without any extra plumbing.
///
/// Updating both from this single command, rather than leaving each
/// consumer free to read a different copy, is what keeps them from ever
/// disagreeing about a door's lock state.
pub(super) fn setlock(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (selector, level_arg, key_arg) = match invocation.args.as_slice() {
        [selector, level_arg] => (selector, level_arg, None),
        [selector, level_arg, key_arg] => (selector, level_arg, Some(key_arg)),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "setlock requires a reference and a lock level, and accepts an optional key FormID or \"none\"",
            ));
        }
    };
    let entity = resolve_reference(world, selector)?;
    let level: i32 = level_arg.parse().map_err(|_| {
        ConsoleError::new(
            "bad_type",
            "setlock level must be a non-negative integer (0 clears the lock)",
        )
    })?;
    if !(0..=i32::from(i8::MAX)).contains(&level) {
        return Err(ConsoleError::new(
            "bad_type",
            "setlock level must be between 0 and 127",
        ));
    }
    // Issue #185: an optional third argument sets (or, spelled "none",
    // clears) the door's key FormID requirement -- authored FO3 doors
    // rarely carry one even when locked, so this is the only way to make
    // key-aware AI door routing testable without a real keyed door.
    let key_form_id = match key_arg {
        None => None,
        Some(value) if value.eq_ignore_ascii_case("none") => Some(None),
        Some(value) => Some(Some(parse_item_form_id(value).ok_or_else(|| {
            ConsoleError::new(
                "bad_type",
                "setlock key FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f, or \"none\"",
            )
        })?)),
    };
    let placement = world
        .get::<interaction::PlacementRoot>(entity)
        .ok_or_else(|| ConsoleError::new("not_activatable", "reference has no placement root"))?
        .placement()
        .clone();
    if !matches!(placement.semantic, PreparedSemantic::Door(_)) {
        return Err(ConsoleError::new(
            "not_a_door",
            "setlock only accepts door references",
        ));
    }
    // Level 0 clears the lock, matching `door_is_locked`'s own
    // `lock_level.is_none_or(|level| level <= 0)` rule.
    let lock_level = if level == 0 { None } else { Some(level as i8) };
    let mut root = world
        .get_mut::<interaction::PlacementRoot>(entity)
        .expect("placement root presence checked above");
    root.set_door_lock_level(lock_level);
    if let Some(key_form_id) = key_form_id {
        root.set_door_key_form_id(key_form_id);
    }
    nav::agent::set_door_lock_level(world, placement.reference_form_id, lock_level);
    if let Some(key_form_id) = key_form_id {
        nav::agent::set_door_key_form_id(world, placement.reference_form_id, key_form_id);
    }
    let mut summary = match lock_level {
        Some(level) => format!("setlock {:08x} level {level}", placement.reference_form_id),
        None => format!("setlock {:08x} unlocked", placement.reference_form_id),
    };
    if let Some(key_form_id) = key_form_id {
        summary.push_str(&match key_form_id {
            Some(key_form_id) => format!(" key {key_form_id:08x}"),
            None => " key none".to_string(),
        });
    }
    info!("{summary}");
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": placement.reference_form_id,
            "lock_level": lock_level,
            "key_form_id": key_form_id.flatten(),
        }),
        vec![summary],
    ))
}

pub(super) fn ragdoll_probe(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "ragdollprobe requires exactly one actor reference",
        ));
    }
    let entity = resolve_reference(world, &invocation.args[0])?;
    let placement = world
        .get::<interaction::PlacementRoot>(entity)
        .ok_or_else(|| ConsoleError::new("not_actor", "reference has no placement root"))?
        .placement()
        .clone();
    if !matches!(
        placement.semantic,
        PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
    ) {
        return Err(ConsoleError::new(
            "not_actor",
            "ragdollprobe only accepts NPC or creature references",
        ));
    }
    let enabled = world
        .get::<player::RagdollToggle>(entity)
        .is_some_and(|toggle| toggle.0);
    let probe = player::probe_ragdoll(world, entity);
    let summary = format!(
        "ragdollprobe {:08x} enabled={} instantiated={} bodies={} awake={} sleep_disabled={} joints={} max_joint_linear={:.5}m max_joint_angular={:.5}rad max_node_position={:.5}m max_node_angle={:.5}rad max_linear_speed={:.5}m/s max_angular_speed={:.5}rad/s",
        placement.reference_form_id,
        enabled,
        probe.instantiated,
        probe.body_count,
        probe.awake_count,
        probe.sleep_disabled_count,
        probe.joint_count,
        probe.max_joint_linear_separation,
        probe.max_joint_angular_separation,
        probe.max_node_position_error,
        probe.max_node_angle_error,
        probe.max_linear_speed,
        probe.max_angular_speed,
    );
    let value = serde_json::to_value(&probe).map_err(|error| {
        ConsoleError::new("serialization_failed", format!("ragdoll probe: {error}"))
    })?;
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": placement.reference_form_id,
            "enabled": enabled,
            "probe": value,
        }),
        vec![summary],
    ))
}

pub(super) fn ragdoll(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !(1..=2).contains(&invocation.args.len()) {
        return Err(ConsoleError::new(
            "bad_arity",
            "ragdoll requires an actor reference and optional on, off, or reset",
        ));
    }
    let entity = resolve_reference(world, &invocation.args[0])?;
    let placement = world
        .get::<interaction::PlacementRoot>(entity)
        .ok_or_else(|| ConsoleError::new("not_actor", "reference has no placement root"))?
        .placement()
        .clone();
    if !matches!(
        placement.semantic,
        PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
    ) {
        return Err(ConsoleError::new(
            "not_actor",
            "ragdoll only accepts NPC or creature references",
        ));
    }
    let operation = invocation
        .args
        .get(1)
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| "toggle".into());
    match operation.as_str() {
        "on" => {
            world.entity_mut(entity).insert(player::RagdollToggle(true));
        }
        "off" => {
            world
                .entity_mut(entity)
                .insert(player::RagdollToggle(false));
        }
        "reset" => {
            let actor_scale = world
                .get::<actor::ActorRuntime>(entity)
                .and_then(|runtime| runtime.assembly.as_ref())
                .map_or(placement.scale, |assembly| assembly.root_scale);
            world.entity_mut(entity).remove::<player::RagdollToggle>();
            if let Some(mut transform) = world.get_mut::<Transform>(entity) {
                transform.translation = Vec3::from_array(placement.translation);
                transform.rotation = Quat::from_xyzw(
                    placement.rotation_xyzw[0],
                    placement.rotation_xyzw[1],
                    placement.rotation_xyzw[2],
                    placement.rotation_xyzw[3],
                );
                transform.scale = Vec3::splat(actor_scale);
            }
        }
        "toggle" => {
            let enabled = world
                .get::<player::RagdollToggle>(entity)
                .is_some_and(|toggle| toggle.0);
            world
                .entity_mut(entity)
                .insert(player::RagdollToggle(!enabled));
        }
        _ => {
            return Err(ConsoleError::new(
                "bad_value",
                "ragdoll mode must be on, off, or reset",
            ));
        }
    }
    let enabled = world
        .get::<player::RagdollToggle>(entity)
        .is_some_and(|toggle| toggle.0);
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": placement.reference_form_id,
            "enabled": enabled,
            "mode": operation,
        }),
        vec![format!(
            "ragdoll {:08x} {}",
            placement.reference_form_id,
            if enabled { "enabled" } else { "disabled" }
        )],
    ))
}

/// Mirrors `world::preload::scene_manifest_path` (private to that module).
/// Duplicated here only for `tp`'s synchronous prepared-cell existence
/// check -- the actual swap/loader logic itself is not duplicated; `tp`
/// drives the exact same `DoorTravelRequested` message and `world::swap`
/// systems door travel and `activate` already use (see `teleport_player`
/// below).
pub(super) fn tp_scene_manifest_path(asset_root: &std::path::Path, form_id: u32) -> PathBuf {
    asset_root
        .join("scenes")
        .join(format!("{form_id:08x}"))
        .join("scene.ron")
}

pub(super) fn finite_tp_coordinate(value: &str) -> Result<f32, ConsoleError> {
    let parsed = value
        .parse::<f32>()
        .map_err(|_| ConsoleError::new("bad_type", "tp coordinates must be finite numbers"))?;
    if parsed.is_finite() {
        Ok(parsed)
    } else {
        Err(ConsoleError::new(
            "bad_type",
            "tp coordinates must be finite numbers",
        ))
    }
}

/// Issue #152: sets all three axes of the player's `Transform` in one
/// write -- the same mechanism `[player.]setpos` uses
/// (`console::commands::set_position`/`notify_transform_mutated`), just
/// without the intermediate single-axis states that let a mid-sequence
/// physics tick see the player poking through geometry. `player::
/// console_transform_mutated` is the only hook `viewer::console::install`
/// registers with `ConsoleEntityHooks`, so calling it directly here is
/// equivalent to the generic `notify_transform_mutated` dispatch `setpos`
/// goes through (that dispatcher itself is private to `crate::console` and
/// not re-exported for callers outside it).
pub(super) fn teleport_player_in_place(
    world: &mut World,
    position: Vec3,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let entity = resolve_reference(world, "player")?;
    {
        let mut transform = world
            .get_mut::<Transform>(entity)
            .ok_or_else(|| ConsoleError::new("component_missing", "reference has no Transform"))?;
        transform.translation = position;
    }
    player::console_transform_mutated(world, entity);
    Ok(ConsoleCommandResult::new(
        json!({
            "x": position.x,
            "y": position.y,
            "z": position.z,
            "unit": "metres",
        }),
        vec![format!(
            "tp: teleported player to ({:.3}, {:.3}, {:.3}).",
            position.x, position.y, position.z
        )],
    ))
}

/// Issue #152: `tp <x> <y> <z> [<cell-formid>]`. The 3-arg form is an
/// atomic `setpos` (see `teleport_player_in_place`). The 4-arg form reuses
/// the exact same instant-swap/cell-travel machinery door activation and
/// `activate` already drive -- a `DoorTravelRequested` message consumed by
/// `world::swap`'s `evaluate_door_travel_requests` /
/// `apply_pending_instant_swap` (or the loading-screen fallback for a
/// prepared-but-not-resident cell) -- rather than a new loader. Its
/// `translation` is the requested (x, y, z) directly, so `world::swap`'s
/// `activate_resident_cell` places the player there via `player::
/// teleport_active_player` as part of the very same swap a door uses; a
/// synthetic `door_form_id: 0` (never a real FormID) marks it as not a real
/// door for `nav::agent::note_player_swap_door`'s follow-through/freeze
/// bookkeeping. A destination already equal to the active cell skips the
/// swap and falls back to the plain in-place teleport above. An unprepared
/// destination cell fails synchronously and deterministically, matching
/// `activate`'s unknown-reference error style, instead of silently kicking
/// off a fallback load that would only fail later.
pub(super) fn teleport_player(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (coordinates, cell_arg) = match invocation.args.as_slice() {
        [x, y, z] => ([x, y, z], None),
        [x, y, z, cell] => ([x, y, z], Some(cell)),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tp requires <x> <y> <z> and an optional cell FormID",
            ));
        }
    };
    let position = Vec3::new(
        finite_tp_coordinate(coordinates[0])?,
        finite_tp_coordinate(coordinates[1])?,
        finite_tp_coordinate(coordinates[2])?,
    );
    let Some(cell_value) = cell_arg else {
        return teleport_player_in_place(world, position);
    };
    let destination_cell = parse_item_form_id(cell_value).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "tp cell FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f",
        )
    })?;
    if world
        .get_resource::<super::super::world::ActiveCell>()
        .is_some_and(|active| active.0 == destination_cell)
    {
        return teleport_player_in_place(world, position);
    }
    let asset_root = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .map(|manifest| PathBuf::from(&manifest.0.asset_root))
        .ok_or_else(|| {
            ConsoleError::new("cell_unavailable", "no active cell manifest is loaded")
        })?;
    if !tp_scene_manifest_path(&asset_root, destination_cell).is_file() {
        return Err(ConsoleError::new(
            "cell_not_found",
            format!("cell {destination_cell:08x} is not a prepared cell"),
        ));
    }
    let rotation_xyzw = resolve_reference(world, "player")
        .ok()
        .and_then(|entity| world.get::<Transform>(entity))
        .map(|transform| transform.rotation.to_array())
        .unwrap_or([0.0, 0.0, 0.0, 1.0]);
    world.write_message(interaction::DoorTravelRequested {
        destination_cell_form_id: destination_cell,
        translation: position,
        rotation_xyzw,
        door_form_id: 0,
    });
    Ok(ConsoleCommandResult::new(
        json!({
            "x": position.x,
            "y": position.y,
            "z": position.z,
            "cell_form_id": destination_cell,
            "unit": "metres",
        }),
        vec![format!(
            "tp: cell travel requested to {destination_cell:08x}; player will be placed at ({:.3}, {:.3}, {:.3}).",
            position.x, position.y, position.z
        )],
    ))
}
