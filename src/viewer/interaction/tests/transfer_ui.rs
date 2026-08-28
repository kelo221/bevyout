use bevyout_core::items::OwnershipClaim;

use super::*;

fn test_item(base_form_id: u32, name: &str) -> PreparedItemDefinition {
    PreparedItemDefinition {
        base_form_id,
        record_kind: "MISC".into(),
        category: crate::vsa::PreparedItemCategory::Misc,
        editor_id: None,
        display_name: Some(name.into()),
        source_model_path: None,
        icon_asset_path: Some(format!("icons/{base_form_id:08x}.ktx2")),
        world_asset_path: None,
        physics_asset_path: None,
        drop_collider: Default::default(),
        value: Some(5),
        weight: Some(1.0),
        quest_item: false,
        stats: crate::vsa::PreparedItemStats::Misc,
        audio: Default::default(),
    }
}

#[test]
fn active_transfer_pane_selects_the_matching_icon_and_details_stack() {
    let active = super::super::ActiveContainer {
        kind: super::super::LootHolderKind::Container,
        entity: Entity::PLACEHOLDER,
        reference_form_id: 0x100,
        name: "Test Container".into(),
        item_names: Default::default(),
        owner_form_id: None,
        owner_faction_rank: None,
    };
    let states = ContainerStates(std::collections::HashMap::from([(
        0x100,
        container_policy::ContainerState {
            stacks: vec![(0x10, 2)],
            resolved: true,
        },
    )]));
    let inventory = PlayerInventory::from_stack_states([InventoryStack {
        base_form_id: 0x20,
        count: 3,
        condition: Some(50),
    }]);
    let catalog = PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![
            test_item(0x10, "Container Item"),
            test_item(0x20, "Player Item"),
        ],
    };
    let mut ui = TransferUiState {
        selected_container: Some(0x10),
        selected_player: Some(StackKey {
            base_form_id: 0x20,
            condition: Some(50),
        }),
        active_pane: TransferPaneSide::Container,
    };

    let (stack, item) = selected_detail(&active, &states, &inventory, &catalog, &ui).unwrap();
    assert_eq!((stack.base_form_id, stack.count), (0x10, 2));
    assert_eq!(item.icon_asset_path.as_deref(), Some("icons/00000010.ktx2"));

    ui.active_pane = TransferPaneSide::Player;
    let (stack, item) = selected_detail(&active, &states, &inventory, &catalog, &ui).unwrap();
    assert_eq!(
        (stack.base_form_id, stack.count, stack.condition),
        (0x20, 3, Some(50))
    );
    assert_eq!(item.icon_asset_path.as_deref(), Some("icons/00000020.ktx2"));
}

#[test]
fn left_clicking_container_row_takes_item_and_plays_pickup_sound() {
    let mut item = test_item(0x10, "Container Item");
    item.audio.pickup_sound_form_id = Some(0x20);
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_message::<PlaySound>()
        .insert_resource(ActiveContainerTarget(Some(super::super::ActiveContainer {
            kind: super::super::LootHolderKind::Container,
            entity: Entity::PLACEHOLDER,
            reference_form_id: 0x100,
            name: "Test Container".into(),
            item_names: Default::default(),
            owner_form_id: None,
            owner_faction_rank: None,
        })))
        .insert_resource(ContainerStates(std::collections::HashMap::from([(
            0x100,
            container_policy::ContainerState {
                stacks: vec![(0x10, 1)],
                resolved: true,
            },
        )])))
        .insert_resource(PlayerInventory::default())
        .insert_resource(CanonicalItemLedger::default())
        .insert_resource(PreparedItemCatalog {
            revision: "test".into(),
            source_fingerprint: "test".into(),
            items: vec![item],
        })
        .insert_resource(TransferUiState::default())
        .add_systems(Update, handle_container_rows);
    app.world_mut()
        .spawn((Interaction::Pressed, ContainerRow(0x10)));

    app.update();

    assert_eq!(app.world().resource::<PlayerInventory>().count(0x10), 1);
    let messages = app.world().resource::<Messages<PlaySound>>();
    let sounds = messages
        .get_cursor()
        .read(messages)
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(sounds.len(), 1);
    assert_eq!((sounds[0].form_id, sounds[0].position), (0x20, None));
}

#[test]
fn left_clicking_player_row_stores_item_and_plays_drop_sound() {
    let mut item = test_item(0x10, "Player Item");
    item.audio.drop_sound_form_id = Some(0x30);
    let key = StackKey {
        base_form_id: 0x10,
        condition: None,
    };
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_message::<PlaySound>()
        .insert_resource(ActiveContainerTarget(Some(super::super::ActiveContainer {
            kind: super::super::LootHolderKind::Container,
            entity: Entity::PLACEHOLDER,
            reference_form_id: 0x100,
            name: "Test Container".into(),
            item_names: Default::default(),
            owner_form_id: None,
            owner_faction_rank: None,
        })))
        .insert_resource(ContainerStates(std::collections::HashMap::from([(
            0x100,
            container_policy::ContainerState {
                stacks: Vec::new(),
                resolved: true,
            },
        )])))
        .insert_resource(PlayerInventory::from_stack_states([InventoryStack {
            base_form_id: 0x10,
            count: 1,
            condition: None,
        }]))
        .insert_resource(CanonicalItemLedger::default())
        .insert_resource(PlayerEquipment::default())
        .insert_resource(PreparedItemCatalog {
            revision: "test".into(),
            source_fingerprint: "test".into(),
            items: vec![item],
        })
        .insert_resource(TransferUiState::default())
        .add_systems(Update, handle_player_rows);
    app.world_mut()
        .spawn((Interaction::Pressed, PlayerRow(key)));

    app.update();

    assert_eq!(app.world().resource::<PlayerInventory>().count(0x10), 0);
    let states = app.world().resource::<ContainerStates>();
    assert_eq!(
        container_policy::stack_count(&states.get(0x100).unwrap().stacks, 0x10),
        1
    );
    let messages = app.world().resource::<Messages<PlaySound>>();
    let sounds = messages
        .get_cursor()
        .read(messages)
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(sounds.len(), 1);
    assert_eq!((sounds[0].form_id, sounds[0].position), (0x30, None));
}

#[test]
fn take_all_moves_every_container_stack_through_the_canonical_path() {
    let mut container = container_policy::ContainerState {
        stacks: vec![(0x10, 2), (0x20, 3)],
        resolved: true,
    };
    let mut inventory = PlayerInventory::default();
    let mut canonical = CanonicalItemLedger::default();

    let mut theft = super::TheftReportContext {
        claim: OwnershipClaim::default(),
        reference_form_id: 0x100,
        catalogs: None,
        progression: None,
        witnesses: &[],
    };
    let moved = take_all(&mut container, &mut inventory, &mut canonical, &mut theft);

    assert!(moved.iter().all(|(_, result)| result.is_ok()));
    assert!(container.stacks.is_empty());
    assert_eq!(inventory.count(0x10), 2);
    assert_eq!(inventory.count(0x20), 3);
}

#[test]
fn transfer_uses_pickup_sound_when_taking_and_drop_sound_when_storing() {
    let mut item = test_item(0x10, "Item");
    item.audio.pickup_sound_form_id = Some(0x20);
    item.audio.drop_sound_form_id = Some(0x30);
    let catalog = PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![item],
    };

    let take = transfer_sound_request(&catalog, 0x10, TransferDirection::ContainerToPlayer)
        .expect("taking should use the prepared pickup sound");
    let store = transfer_sound_request(&catalog, 0x10, TransferDirection::PlayerToContainer)
        .expect("storing should use the prepared drop sound");

    assert_eq!((take.form_id, take.position), (0x20, None));
    assert_eq!((store.form_id, store.position), (0x30, None));
}
