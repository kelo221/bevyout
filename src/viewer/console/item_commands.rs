//! Inventory, equipment, item-use, and merchant console commands.

use super::*;

pub(super) struct ItemCommandProvider;

impl ConsoleCommandProvider for ItemCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new("additem", "[player.]additem <FormID> [count]", "Add count (default 1) of an item FormID to the player inventory.", add_item).reference_callable(false).mutating(),
            ConsoleCommand::new("equip", "[player.]equip <ItemInstanceId>", "Equip a canonical player item by stable instance id.", equip_item).reference_callable(false).mutating(),
            ConsoleCommand::new("equipitem", "[player.]equipitem <FormID>", "Equip (or unequip, if already equipped) an item FormID already in the player inventory.", equip_item_formid).reference_callable(false).mutating(),
            ConsoleCommand::new("unequip", "[player.]unequip", "Unequip the canonical player item.", unequip_item).reference_callable(false).mutating(),
            ConsoleCommand::new("hotkey", "[player.]hotkey <0..7> <ItemInstanceId>", "Bind a stable canonical item id to a player hotkey slot.", bind_hotkey).reference_callable(false).mutating(),
            ConsoleCommand::new("useitem", "[player.]useitem <ItemInstanceId>", "Consume one unit through the canonical item-use seam.", use_item).reference_callable(false).mutating(),
            ConsoleCommand::new("giveitem", "giveitem <actor-reference> <FormID> [count]", "Add count (default 1) of an item FormID to an NPC/creature's own canonical inventory (issue #185: grants a nav agent's bound actor a door key for AI door-access testing).", give_item).mutating(),
            ConsoleCommand::new("setmerchant", "setmerchant <container-reference> <caps>", "Mark a prepared static container as a merchant with fixed caps.", set_merchant).mutating(),
            ConsoleCommand::new("buy", "buy <merchant-reference> <ItemInstanceId> [count]", "Buy from a prepared static merchant using a Fallout 3 barter quote.", buy_item).mutating(),
            ConsoleCommand::new("sell", "sell <merchant-reference> <ItemInstanceId> [count]", "Sell to a prepared static merchant using a Fallout 3 barter quote.", sell_item).mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

/// Issue #84 (F84.1): `additem`/`player.additem <FormID> [count]` always
/// targets the player inventory -- the Bethesda-console `player.` prefix is
/// accepted (`.reference_callable(false)`) but never used to pick a
/// container, matching real `additem`'s player-only scope. Condition is
/// seeded from `PreparedItemCatalog`'s `max_condition` for Weapon/Apparel
/// stats, exactly as picking the item up in the world would; an absent or
/// uncataloged item still adds, with condition `None`.
pub(super) fn add_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.is_empty() || invocation.args.len() > 2 {
        return Err(ConsoleError::new(
            "bad_arity",
            "additem expects a FormID and an optional count",
        ));
    }
    let form_id = parse_item_form_id(&invocation.args[0]).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "additem FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f",
        )
    })?;
    let count = invocation
        .args
        .get(1)
        .map(|value| {
            value
                .parse::<i32>()
                .map_err(|_| ConsoleError::new("bad_type", "count must be a whole number"))
        })
        .transpose()?
        .unwrap_or(1);
    if count < 1 {
        return Err(ConsoleError::new("bad_count", "count must be at least 1"));
    }
    let condition = world
        .get_resource::<PreparedItemCatalog>()
        .into_iter()
        .flat_map(|catalog| &catalog.items)
        .find(|item| item.base_form_id == form_id)
        .and_then(|item| match &item.stats {
            PreparedItemStats::Weapon { max_condition, .. }
            | PreparedItemStats::Apparel { max_condition, .. } => *max_condition,
            _ => None,
        });
    let stack = InventoryStack {
        base_form_id: form_id,
        count,
        condition,
    };
    let before = world
        .resource::<interaction::PlayerInventory>()
        .legacy_snapshot();
    world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .add_player_item(&before, stack)
        .map_err(|error| ConsoleError::new("item_transaction_failed", error.to_string()))?;
    let _ = world
        .resource_mut::<interaction::PlayerInventory>()
        .add_stack(stack);
    let total = world
        .resource::<interaction::PlayerInventory>()
        .count(form_id);
    Ok(ConsoleCommandResult::new(
        json!({
            "form_id": form_id,
            "count": count,
            "total": total,
        }),
        vec![format!(
            "additem {form_id:08x} x{count}; inventory now has {total}"
        )],
    ))
}

/// Issue #185: the visible surface for granting a *specific* NPC/creature
/// its own canonical inventory item -- `additem` is deliberately
/// player-only (see `add_item`'s doc comment), so there was previously no
/// console command able to put a door's key into an actor's own inventory
/// rather than the player's. This is what makes the key-aware locked-door
/// acceptance script drivable: `giveitem <actor> <door-key-formid>` then a
/// fresh `tna goto`/`tna travel` re-evaluates that actor's nav-agent door
/// overrides against its now-updated inventory (`nav::agent::
/// apply_door_lock_overrides`, called from `goto_agent`/`request_travel`).
pub(super) fn give_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !(2..=3).contains(&invocation.args.len()) {
        return Err(ConsoleError::new(
            "bad_arity",
            "giveitem requires an actor reference, a FormID, and an optional count",
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
            "giveitem only accepts NPC or creature references",
        ));
    }
    let form_id = parse_item_form_id(&invocation.args[1]).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "giveitem FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f",
        )
    })?;
    let count = parse_positive_count(invocation.args.get(2))?;
    let holder = HolderId::Actor {
        reference_form_id: placement.reference_form_id,
    };
    let mut canonical = world.resource_mut::<interaction::CanonicalItemLedger>();
    if !canonical.ledger.holders().contains_key(&holder) {
        canonical
            .ledger
            .insert_holder(
                holder,
                bevyout_core::item_transaction::ItemHolderState::default(),
            )
            .map_err(|error| ConsoleError::new("item_transaction_failed", error.to_string()))?;
    }
    let item_id = canonical
        .ledger
        .insert_new_item(
            holder,
            form_id,
            count,
            bevyout_core::item_transaction::ItemState::default(),
        )
        .map_err(|error| ConsoleError::new("item_transaction_failed", error.to_string()))?;
    info!(
        "giveitem {:08x} {form_id:08x} x{count} (item {})",
        placement.reference_form_id, item_id.0
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "actor_reference_form_id": placement.reference_form_id,
            "form_id": form_id,
            "count": count,
            "item_instance_id": item_id.0,
        }),
        vec![format!(
            "giveitem {:08x}: added {form_id:08x} x{count} to its own inventory",
            placement.reference_form_id
        )],
    ))
}

pub(super) fn parse_item_instance_id(value: &str) -> Option<ItemInstanceId> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u64::from_str_radix(digits, 16)
        .ok()
        .filter(|id| *id != 0)
        .map(ItemInstanceId)
}

pub(super) fn parse_positive_count(value: Option<&String>) -> Result<u32, ConsoleError> {
    value
        .map(|value| {
            value
                .parse::<u32>()
                .map_err(|_| ConsoleError::new("bad_type", "count must be a whole number"))
        })
        .transpose()
        .map(|count| count.unwrap_or(1))
        .and_then(|count| {
            (count > 0)
                .then_some(count)
                .ok_or_else(|| ConsoleError::new("bad_count", "count must be at least 1"))
        })
}

pub(super) fn canonical_equip_target(
    world: &World,
    item_id: ItemInstanceId,
) -> Result<(StackKey, player::equipment::EquipKind), ConsoleError> {
    let key = world
        .resource::<interaction::CanonicalItemLedger>()
        .player_stack_key(item_id)
        .ok_or_else(|| {
            ConsoleError::new(
                "item_not_found",
                "item instance is not in the player inventory",
            )
        })?;
    let item = world
        .get_resource::<PreparedItemCatalog>()
        .and_then(|catalog| {
            catalog
                .items
                .iter()
                .find(|item| item.base_form_id == key.base_form_id)
        })
        .ok_or_else(|| {
            ConsoleError::new(
                "no_catalog_entry",
                "item instance has no prepared item definition",
            )
        })?;
    let kind = interaction::equip_kind_for(item)
        .ok_or_else(|| ConsoleError::new("not_equippable", "item instance cannot be equipped"))?;
    Ok((key, kind))
}

pub(super) fn equip_error(error: player::equipment::EquipError) -> ConsoleError {
    match error {
        player::equipment::EquipError::NotEquippable => {
            ConsoleError::new("not_equippable", "item instance cannot be equipped")
        }
        player::equipment::EquipError::IncompatibleAmmo => ConsoleError::new(
            "incompatible_ammo",
            "item instance ammo does not match the equipped weapon",
        ),
        player::equipment::EquipError::NoWeaponEquipped => ConsoleError::new(
            "no_weapon_equipped",
            "item instance ammo requires a weapon to be equipped first",
        ),
    }
}

/// Equip a canonical player item by stable instance id.
pub(super) fn equip_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [value] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "equip requires an item instance id",
        ));
    };
    let item_id = parse_item_instance_id(value)
        .ok_or_else(|| ConsoleError::new("bad_type", "item instance id must be hexadecimal"))?;
    let (key, kind) = canonical_equip_target(world, item_id)?;
    let previous_equipment = world.resource::<interaction::PlayerEquipment>().clone();
    world
        .resource_mut::<interaction::PlayerEquipment>()
        .equip(key, kind)
        .map_err(equip_error)?;
    if let Err(error) = world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .ledger
        .equip(HolderId::Player, item_id)
    {
        world
            .resource_mut::<interaction::PlayerEquipment>()
            .clone_from(&previous_equipment);
        return Err(ConsoleError::new("equip_failed", error.to_string()));
    }
    Ok(ConsoleCommandResult::new(
        json!({ "item_id": item_id.0, "equipped": true }),
        vec![format!("equipped item {:016x}", item_id.0)],
    ))
}

/// Issue #98 (F98.4): `player.equipitem <FormID>` toggles equip/unequip for
/// an item already in the player inventory, through the same
/// `interaction::equip_kind_for`/`PlayerEquipment::toggle` seam the Pip-Boy
/// and hotkeys use, so all three equip paths behave identically.
pub(super) fn equip_item_formid(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [raw_form_id] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "equipitem expects exactly one FormID",
        ));
    };
    let form_id = parse_item_form_id(raw_form_id).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "equipitem FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f",
        )
    })?;
    let key = world
        .resource::<interaction::PlayerInventory>()
        .stack_states()
        .into_iter()
        .find(|stack| stack.base_form_id == form_id)
        .map(|stack| stack.key())
        .ok_or_else(|| {
            ConsoleError::new(
                "not_in_inventory",
                "equipitem target is not in the player inventory",
            )
        })?;
    let item = world
        .get_resource::<PreparedItemCatalog>()
        .and_then(|catalog| {
            catalog
                .items
                .iter()
                .find(|item| item.base_form_id == form_id)
                .cloned()
        })
        .ok_or_else(|| {
            ConsoleError::new(
                "no_catalog_entry",
                "equipitem target has no prepared item definition",
            )
        })?;
    let kind = interaction::equip_kind_for(&item).ok_or_else(|| {
        ConsoleError::new("not_equippable", "equipitem target cannot be equipped")
    })?;
    let outcome = world
        .resource_mut::<interaction::PlayerEquipment>()
        .toggle(key, kind)
        .map_err(|error| match error {
            player::equipment::EquipError::NotEquippable => {
                ConsoleError::new("not_equippable", "equipitem target cannot be equipped")
            }
            player::equipment::EquipError::IncompatibleAmmo => ConsoleError::new(
                "incompatible_ammo",
                "equipitem ammo does not match the equipped weapon",
            ),
            player::equipment::EquipError::NoWeaponEquipped => ConsoleError::new(
                "no_weapon_equipped",
                "equipitem ammo requires a weapon to be equipped first",
            ),
        })?;
    let equipped = world
        .resource::<interaction::PlayerEquipment>()
        .is_equipped(key);
    Ok(ConsoleCommandResult::new(
        json!({
            "form_id": form_id,
            "equipped": equipped,
            "evicted": outcome
                .evicted
                .iter()
                .map(|key| format!("{:08x}", key.base_form_id))
                .collect::<Vec<_>>(),
        }),
        vec![format!(
            "equipitem {form_id:08x} {}",
            if equipped { "equipped" } else { "unequipped" }
        )],
    ))
}

pub(super) fn unequip_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !invocation.args.is_empty() {
        return Err(ConsoleError::new(
            "bad_arity",
            "unequip accepts no arguments",
        ));
    }
    let item_id = world
        .resource::<interaction::CanonicalItemLedger>()
        .ledger
        .bindings()
        .get(&HolderId::Player)
        .and_then(|bindings| bindings.equipped);
    let key = item_id.and_then(|item_id| {
        world
            .resource::<interaction::CanonicalItemLedger>()
            .player_stack_key(item_id)
    });
    if item_id.is_some() && key.is_none() {
        return Err(ConsoleError::new(
            "unequip_failed",
            "canonical equipped item is not in the player inventory",
        ));
    }
    let item_id = world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .ledger
        .unequip(HolderId::Player)
        .map_err(|error| ConsoleError::new("unequip_failed", error.to_string()))?;
    if let Some(key) = key {
        world
            .resource_mut::<interaction::PlayerEquipment>()
            .unequip(key);
    }
    Ok(ConsoleCommandResult::new(
        json!({ "item_id": item_id.map(|id| id.0), "equipped": false }),
        vec!["player item unequipped".into()],
    ))
}

pub(super) fn bind_hotkey(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [slot, value] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "hotkey requires a slot and item instance id",
        ));
    };
    let slot = slot
        .parse::<usize>()
        .map_err(|_| ConsoleError::new("bad_type", "hotkey slot must be 0..7"))?;
    if slot >= 8 {
        return Err(ConsoleError::new(
            "hotkey_failed",
            "hotkey slot must be 0..7",
        ));
    }
    let item_id = parse_item_instance_id(value)
        .ok_or_else(|| ConsoleError::new("bad_type", "item instance id must be hexadecimal"))?;
    let key = world
        .resource::<interaction::CanonicalItemLedger>()
        .player_stack_key(item_id)
        .ok_or_else(|| {
            ConsoleError::new(
                "item_not_found",
                "item instance is not in the player inventory",
            )
        })?;
    world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .ledger
        .bind_hotkey(HolderId::Player, slot, item_id)
        .map_err(|error| ConsoleError::new("hotkey_failed", error.to_string()))?;
    world
        .get_resource_or_insert_with(super::super::bindings::HotkeyBindings::default)
        .assign((slot + 1) as u8, key);
    Ok(ConsoleCommandResult::new(
        json!({ "slot": slot, "item_id": item_id.0 }),
        vec![format!("hotkey {slot} bound to {:016x}", item_id.0)],
    ))
}

pub(super) fn use_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [value] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "useitem requires an item instance id",
        ));
    };
    let item_id = parse_item_instance_id(value)
        .ok_or_else(|| ConsoleError::new("bad_type", "item instance id must be hexadecimal"))?;
    let used = world
        .resource_scope(
            |world, mut canonical: Mut<interaction::CanonicalItemLedger>| {
                let mut inventory = world.resource_mut::<interaction::PlayerInventory>();
                canonical.use_player_item(&mut inventory, item_id)
            },
        )
        .map_err(|error| ConsoleError::new("useitem_failed", error.to_string()))?;
    // M9 wave 3 (#318): when the consumed item is a cataloged ingestible,
    // its authored effects apply through `effects::apply_ingestible`
    // (health restore, radiation, timed modifiers, addiction roll). Items
    // without an effect-catalog entry consume exactly as before.
    let ingestible = world
        .get_resource::<super::super::effects::EffectCatalog>()
        .and_then(|catalog| catalog.get(used.base_form_id).cloned());
    let restore_limbs = ingestible
        .as_ref()
        .is_some_and(bevyout_core::effects::IngestibleDefinition::restores_limbs);
    let application = ingestible
        .as_ref()
        .map(|definition| super::effect_commands::apply_ingestible_to_player(world, definition));
    let mut value = json!({ "item_id": item_id.0, "base_form_id": used.base_form_id, "count": 1 });
    let mut log = vec![format!("used item {:016x}", item_id.0)];
    if let Some(application) = application {
        value["ingestible"] = effect_commands::application_json(&application);
        log.push(effect_commands::application_summary(&application));
    }
    if restore_limbs {
        limb_commands::restore_selected_player_limb(world);
        value["limb_restore"] = true.into();
    }
    Ok(ConsoleCommandResult::new(value, log))
}

pub(super) fn merchant_stacks(placement: &interaction::PlacementRoot) -> Vec<(u32, i32)> {
    placement
        .placement()
        .inventory
        .iter()
        .filter(|entry| !entry.leveled)
        .map(|entry| (entry.base_form_id, entry.count))
        .collect()
}

pub(super) fn set_merchant(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector, caps] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "setmerchant requires a container reference and caps",
        ));
    };
    let entity = resolve_reference(world, selector)?;
    let (reference_form_id, stacks) = {
        let placement = world
            .get::<interaction::PlacementRoot>(entity)
            .ok_or_else(|| {
                ConsoleError::new("not_a_container", "reference has no placement root")
            })?;
        if !matches!(placement.placement().semantic, PreparedSemantic::Container) {
            return Err(ConsoleError::new(
                "not_a_container",
                "merchant reference is not a container",
            ));
        }
        (
            placement.placement().reference_form_id,
            merchant_stacks(placement),
        )
    };
    let caps = caps
        .parse::<u64>()
        .map_err(|_| ConsoleError::new("bad_type", "merchant caps must be a whole number"))?;
    world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .set_merchant(reference_form_id, &stacks, caps)
        .map_err(|error| ConsoleError::new("merchant_setup_failed", error.to_string()))?;
    Ok(ConsoleCommandResult::new(
        json!({ "reference_form_id": reference_form_id, "caps": caps }),
        vec![format!(
            "merchant {:08x} configured with {caps} caps",
            reference_form_id
        )],
    ))
}

pub(super) fn merchant_transaction(
    world: &mut World,
    invocation: &ConsoleInvocation,
    buying: bool,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector, item_value, count_value @ ..] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "merchant transfer requires reference, item instance id, and optional count",
        ));
    };
    if count_value.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "merchant transfer accepts one optional count",
        ));
    }
    let count = parse_positive_count(count_value.first())?;
    let merchant_entity = resolve_reference(world, selector)?;
    let reference_form_id = world
        .get::<interaction::PlacementRoot>(merchant_entity)
        .ok_or_else(|| ConsoleError::new("not_a_merchant", "reference has no placement root"))?
        .placement()
        .reference_form_id;
    let item_id = parse_item_instance_id(item_value)
        .ok_or_else(|| ConsoleError::new("bad_type", "item instance id must be hexadecimal"))?;
    let merchant = HolderId::FixtureMerchant { reference_form_id };
    let (base_form_id, player_revision, merchant_revision, transaction_id) = {
        let ledger = &world.resource::<interaction::CanonicalItemLedger>().ledger;
        let item = ledger
            .holders()
            .get(&merchant)
            .and_then(|state| state.find(item_id))
            .or_else(|| {
                ledger
                    .holders()
                    .get(&HolderId::Player)
                    .and_then(|state| state.find(item_id))
            })
            .ok_or_else(|| ConsoleError::new("item_not_found", "item instance is not available"))?;
        (
            item.base_form_id,
            ledger
                .holders()
                .get(&HolderId::Player)
                .map(|state| state.revision)
                .unwrap_or(0),
            ledger
                .holders()
                .get(&merchant)
                .map(|state| state.revision)
                .unwrap_or(0),
            ledger.next_transaction_id(),
        )
    };
    let value = world
        .resource::<PreparedItemCatalog>()
        .items
        .iter()
        .find(|definition| definition.base_form_id == base_form_id)
        .and_then(|definition| definition.value)
        .filter(|value| *value >= 0)
        .ok_or_else(|| {
            ConsoleError::new("invalid_price", "item has no non-negative catalog value")
        })? as u64;
    if base_form_id == interaction::item_rules::CAPS_FORM_ID
        || world
            .resource::<PreparedItemCatalog>()
            .items
            .iter()
            .find(|definition| definition.base_form_id == base_form_id)
            .is_some_and(|definition| definition.quest_item)
    {
        return Err(ConsoleError::new(
            "item_not_tradeable",
            "caps and quest items cannot be traded",
        ));
    }
    let player_barter = world
        .get_resource::<super::stats::PlayerProgression>()
        .map(|progression| {
            progression
                .stats
                .skill_value(bevyout_core::actor_state::ActorSkill::Barter)
        })
        .unwrap_or(0);
    let quote = bevyout_core::barter::quote_barter(bevyout_core::barter::BarterQuoteInput {
        direction: if buying {
            bevyout_core::barter::BarterDirection::Buy
        } else {
            bevyout_core::barter::BarterDirection::Sell
        },
        merchant,
        player: HolderId::Player,
        item_id,
        count,
        base_value: value,
        player_barter,
        player_revision,
        merchant_revision,
    })
    .map_err(|error| ConsoleError::new("barter_quote_failed", error.to_string()))?;
    let receipt = {
        let mut canonical = world.resource_mut::<interaction::CanonicalItemLedger>();
        bevyout_core::barter::commit_barter(&mut canonical.ledger, transaction_id, &quote)
            .map_err(|error| ConsoleError::new("merchant_transfer_failed", error.to_string()))?
    };
    Ok(ConsoleCommandResult::new(
        json!({
            "merchant": reference_form_id,
            "item_id": item_id.0,
            "count": count,
            "base_value": value,
            "unit_price": quote.unit_price,
            "total": quote.total,
            "factor_milli": quote.factor_milli,
            "transaction_id": receipt.id.0
        }),
        vec![format!(
            "{} {:016x} x{} at {} caps",
            if buying { "bought" } else { "sold" },
            item_id.0,
            count,
            quote.unit_price
        )],
    ))
}

pub(super) fn buy_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    merchant_transaction(world, invocation, true)
}

pub(super) fn sell_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    merchant_transaction(world, invocation, false)
}
