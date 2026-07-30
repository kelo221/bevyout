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
