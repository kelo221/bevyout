use super::*;
use crate::item_transaction::{
    HolderId, ItemExtraEntry, ItemHolderState, ItemInstance, ItemInstanceId, ItemLedger, ItemState,
    OwnershipProvenance,
};

fn placement(reference_form_id: u32, translation: [f32; 3]) -> PreparedPlacement {
    PreparedPlacement {
        reference_form_id,
        base_form_id: 0x1,
        asset_path: Some("meshes/test.glb".into()),
        translation,
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: Default::default(),
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: "REFR".into(),
        base_kind: "STAT".into(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: Default::default(),
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: "ao-none".into(),
    }
}

fn test_world() -> World {
    let mut world = World::new();
    world.init_resource::<ActiveSaveState>();
    world.init_resource::<PersistRestores>();
    world.init_resource::<interaction::InteractionState>();
    world.init_resource::<bevy::ecs::message::Messages<animation::PlayPlacementAnimation>>();
    world.init_resource::<interaction::ContainerStates>();
    world
}

fn container_placement(
    reference_form_id: u32,
    entries: Vec<PreparedInventoryEntry>,
) -> PreparedPlacement {
    PreparedPlacement {
        semantic: PreparedSemantic::Container,
        inventory: entries,
        ..placement(reference_form_id, [0.0, 0.0, 0.0])
    }
}

fn inventory_entry(base_form_id: u32, count: i32, leveled: bool) -> PreparedInventoryEntry {
    PreparedInventoryEntry {
        base_form_id,
        count,
        record_kind: "MISC".into(),
        editor_id: None,
        display_name: None,
        leveled,
    }
}

fn spawn_cell(world: &mut World, placements: &[PreparedPlacement]) -> (Entity, Vec<Entity>) {
    let root = world.spawn((Transform::default(), Visibility::Hidden)).id();
    let children = placements
        .iter()
        .map(|placement| {
            world
                .spawn((
                    interaction::PlacementRoot::new(placement.clone()),
                    Transform {
                        translation: Vec3::from_array(placement.translation),
                        rotation: Quat::from_array(placement.rotation_xyzw),
                        scale: Vec3::splat(placement.scale),
                    },
                    Visibility::Inherited,
                    ChildOf(root),
                ))
                .id()
        })
        .collect();
    (root, children)
}

// T60.4: a moved placement's transform survives capture -> despawn ->
// respawn -> apply (the swap-away-and-back shape, minus the swap driver).
#[test]
fn capture_and_apply_restore_a_moved_placement_transform() {
    let mut world = test_world();
    let placements = [
        placement(0x10, [0.0, 0.0, 0.0]),
        placement(0x20, [5.0, 0.0, 0.0]),
    ];
    let (root, children) = spawn_cell(&mut world, &placements);
    world
        .entity_mut(children[0])
        .insert(Transform::from_xyz(1.0, 2.0, 3.0));

    capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
    let state = world.resource::<ActiveSaveState>();
    let cell = state.0.cells.get(&0xC0DE).expect("cell state captured");
    assert!(cell.references.contains_key(&0x10));
    assert!(
        !cell.references.contains_key(&0x20),
        "untouched ref must produce no delta"
    );

    // Simulate eviction + fresh respawn at baseline pose.
    world.entity_mut(root).despawn();
    let (root, children) = spawn_cell(&mut world, &placements);
    apply_cell_placements(&mut world, 0xC0DE, root, &placements);

    let restored = world.get::<Transform>(children[0]).unwrap();
    assert_eq!(restored.translation, Vec3::new(1.0, 2.0, 3.0));
    let untouched = world.get::<Transform>(children[1]).unwrap();
    assert_eq!(untouched.translation, Vec3::new(5.0, 0.0, 0.0));
    // The restore is staged for the collider build (no physics here).
    assert!(
        world
            .resource::<PersistRestores>()
            .bodies
            .contains_key(&0x10)
    );
}

#[test]
fn assetless_actor_proxy_is_included_in_persistence_capture() {
    let mut world = test_world();
    let actor = PreparedPlacement {
        asset_path: None,
        semantic: PreparedSemantic::Npc(crate::vsa::PreparedActor::default()),
        ..placement(0x30, [0.0, 0.0, 0.0])
    };
    assert!(placement_is_spawnable(&actor, false));
    let (root, children) = spawn_cell(&mut world, std::slice::from_ref(&actor));
    world
        .entity_mut(children[0])
        .insert(Transform::from_xyz(1.0, 2.0, 3.0));

    capture_cell_placements(&mut world, 0xC0DE, root, &[actor], false);

    let saved = &world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x30];
    assert_eq!(saved.transform.unwrap().translation, [1.0, 2.0, 3.0]);
    assert!(!saved.deleted);
}

// T60.4: a taken pickup (despawned entity) is captured deleted, and a
// later apply despawns the freshly-spawned entity and suppresses its
// collider build.
#[test]
fn a_taken_pickup_stays_deleted_across_capture_and_apply() {
    let mut world = test_world();
    let placements = [placement(0x10, [0.0, 0.0, 0.0])];
    let (root, children) = spawn_cell(&mut world, &placements);
    world.entity_mut(children[0]).despawn();

    capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
    assert!(world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x10].deleted);

    world.entity_mut(root).despawn();
    let (root, children) = spawn_cell(&mut world, &placements);
    apply_cell_placements(&mut world, 0xC0DE, root, &placements);
    assert!(
        world.get_entity(children[0]).is_err(),
        "deleted ref must despawn"
    );
    assert!(
        world
            .resource::<PersistRestores>()
            .suppressed
            .contains(&0x10)
    );
}

// T60.4: an open container is captured activated and re-opened on apply
// (open set + Opening clip replayed exactly once).
#[test]
fn an_open_container_reopens_on_apply() {
    let mut world = test_world();
    let placements = [placement(0x10, [0.0, 0.0, 0.0])];
    let (root, children) = spawn_cell(&mut world, &placements);
    world
        .resource_mut::<interaction::InteractionState>()
        .open
        .insert(children[0]);

    capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
    assert_eq!(
        world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x10].activated,
        Some(true)
    );

    world.entity_mut(root).despawn();
    world
        .resource_mut::<interaction::InteractionState>()
        .open
        .clear();
    let (root, children) = spawn_cell(&mut world, &placements);
    apply_cell_placements(&mut world, 0xC0DE, root, &placements);
    assert!(
        world
            .resource::<interaction::InteractionState>()
            .open
            .contains(&children[0])
    );
    let messages =
        world.resource::<bevy::ecs::message::Messages<animation::PlayPlacementAnimation>>();
    assert_eq!(messages.iter_current_update_messages().count(), 1);

    // Re-applying while already open must not replay the clip.
    apply_cell_placements(&mut world, 0xC0DE, root, &placements);
    let messages =
        world.resource::<bevy::ecs::message::Messages<animation::PlayPlacementAnimation>>();
    assert_eq!(messages.iter_current_update_messages().count(), 1);
}

// A loaded save's enabled override survives a capture that cannot
// observe enable state (nothing toggles it at runtime yet).
#[test]
fn capture_preserves_a_loaded_enabled_override() {
    let mut world = test_world();
    let placements = [placement(0x10, [0.0, 0.0, 0.0])];
    let (root, _children) = spawn_cell(&mut world, &placements);
    world
        .resource_mut::<ActiveSaveState>()
        .0
        .cells
        .entry(0xC0DE)
        .or_default()
        .references
        .insert(
            0x10,
            PersistentReferenceDelta {
                enabled: Some(false),
                ..Default::default()
            },
        );

    capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
    assert_eq!(
        world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x10].enabled,
        Some(false)
    );
}

// F60.2: a disabled-by-save reference is hidden on apply.
#[test]
fn apply_hides_a_reference_disabled_by_the_save() {
    let mut world = test_world();
    let placements = [placement(0x10, [0.0, 0.0, 0.0])];
    let (root, children) = spawn_cell(&mut world, &placements);
    world
        .resource_mut::<ActiveSaveState>()
        .0
        .cells
        .entry(0xC0DE)
        .or_default()
        .references
        .insert(
            0x10,
            PersistentReferenceDelta {
                enabled: Some(false),
                ..Default::default()
            },
        );
    apply_cell_placements(&mut world, 0xC0DE, root, &placements);
    assert_eq!(
        world.get::<Visibility>(children[0]),
        Some(&Visibility::Hidden)
    );
}

fn minimal_manifest(cell_form_id: u32) -> PreparedSceneManifest {
    PreparedSceneManifest {
        schema_version: 13,
        prepare_revision: None,
        converter_revision: None,
        physics_schema_version: None,
        asset_root: ".".into(),
        source_plugin: "Fallout3.esm".into(),
        source_fingerprint: "content-hash".into(),
        item_catalog_path: None,
        item_catalog_revision: None,
        item_catalog_hash: None,
        recipe_catalog_path: None,
        recipe_catalog_revision: None,
        recipe_catalog_hash: None,
        actor_catalog_path: None,
        actor_catalog_revision: None,
        actor_catalog_hash: None,
        actor_animation_catalog_path: None,
        actor_animation_catalog_revision: None,
        actor_animation_catalog_hash: None,
        image_space_modifier_catalog_path: None,
        image_space_modifier_catalog_revision: None,
        image_space_modifier_catalog_hash: None,
        // `PreparedPluginSource` is not re-exported from `crate::vsa`
        // and widening that surface for a test is not worth it; an
        // empty plugin list exercises the same identity plumbing.
        source_plugins: Vec::new(),
        visual_issues: Vec::new(),
        cell: crate::vsa::CellInfo {
            form_id: cell_form_id,
            editor_id: None,
            name: None,
            interior: true,
            behave_like_exterior: false,
            ambient_rgba: [0.0; 4],
            directional_rgba: [0.0; 4],
            image_space_form_id: None,
            image_space: None,
            lighting_template_form_id: None,
            lighting_template_flags: 0,
            lighting_template: None,
            raw_lighting: None,
            effective_lighting: None,
            water_form_id: None,
            water_height: None,
            grid: None,
            worldspace_form_id: None,
            day_night_profile: None,
            day_night_preview_profile: None,
        },
        placements: Vec::new(),
        lights: Vec::new(),
        diagnostics: Vec::new(),
        navmeshes: Vec::new(),
        nav_graph: None,
        cell_audio: Default::default(),
        audio_clips: Vec::new(),
        footstep_sets: Vec::new(),
        hard_landing_clips: Vec::new(),
        bake: None,
        static_point_shadows: None,
        reflection_probes: None,
        mutability_summary: Default::default(),
        leveled_lists: Default::default(),
        dialogue: None,
    }
}

// F60.3/F60.4: the console `save` path captures the active cell, stamps
// the manifest's content identity, includes the player inventory, and
// the written slot round-trips through `SaveStore`.
#[test]
fn write_save_slot_round_trips_world_state_and_player_inventory() {
    let save_dir = std::env::temp_dir().join(format!(
        "bevyout-persist-test-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut world = test_world();
    world.init_resource::<super::super::preload::ResidentCells>();
    world.insert_resource(ActiveCell(0xC0DE));
    world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xC0DE)));
    world.insert_resource(SaveDirectory(save_dir.clone()));
    world.insert_resource(interaction::PlayerInventory::from_stacks([(0x42, 2)]));
    world
        .resource_mut::<ActiveSaveState>()
        .0
        .cells
        .entry(0xC0DE)
        .or_default()
        .references
        .insert(
            0x10,
            PersistentReferenceDelta {
                deleted: true,
                ..Default::default()
            },
        );

    let path = write_save_slot(&mut world, "slot1").expect("save must write");
    assert!(path.ends_with("slot1.bevyoutsave"));

    let outcome = SaveStore::from_save_dir(&save_dir)
        .read_slot("slot1")
        .expect("slot must read back");
    assert_eq!(outcome.save.header.content_fingerprint, "content-hash");
    assert_eq!(outcome.save.header.current_cell, 0xC0DE);
    assert!(outcome.save.header.plugins.is_empty());
    assert!(outcome.save.world.cells[&0xC0DE].references[&0x10].deleted);
    assert_eq!(
        outcome.save.player.as_ref().unwrap().inventory,
        vec![ItemStack {
            base_form_id: 0x42,
            count: 2,
            condition: None,
        }]
    );
    assert!(
        outcome
            .save
            .canonical
            .as_ref()
            .and_then(|snapshot| snapshot.holders.get(&HolderId::Player))
            .is_some()
    );
    let _ = std::fs::remove_dir_all(save_dir);
}

#[test]
fn save_uses_canonical_player_items_and_preserves_opaque_state_and_revision() {
    let save_dir = std::env::temp_dir().join(format!(
        "bevyout-canonical-save-test-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut world = test_world();
    world.init_resource::<super::super::preload::ResidentCells>();
    world.insert_resource(ActiveCell(0xC0DE));
    world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xC0DE)));
    world.insert_resource(SaveDirectory(save_dir.clone()));
    world.insert_resource(interaction::PlayerInventory::from_stacks([(0x99, 99)]));

    let item = ItemInstance::new(
        ItemInstanceId(42),
        0x42,
        2,
        ItemState {
            condition: Some(80),
            ownership: OwnershipProvenance {
                origin_owner_form_id: Some(0x1234),
                origin_faction_rank: Some(2),
                stolen: true,
            },
            extras: vec![ItemExtraEntry {
                namespace_form_id: 0x77,
                tag: *b"TEST",
                payload: vec![1, 2, 3],
            }],
            ..Default::default()
        },
    )
    .unwrap();
    let state = ItemHolderState {
        items: vec![item.clone()],
        caps: 12,
        revision: 7,
    };
    let mut canonical = ItemLedger::new();
    canonical.insert_holder(HolderId::Player, state).unwrap();
    canonical.bind_hotkey(HolderId::Player, 0, item.id).unwrap();
    canonical.equip(HolderId::Player, item.id).unwrap();
    let before = canonical.snapshot();
    world.insert_resource(interaction::CanonicalItemLedger { ledger: canonical });

    write_save_slot(&mut world, "canonical").expect("canonical save must write");

    let after = world
        .resource::<interaction::CanonicalItemLedger>()
        .snapshot();
    assert_eq!(
        after.holders[&HolderId::Player].revision,
        before.holders[&HolderId::Player].revision
    );
    assert_eq!(after, before);
    let outcome = SaveStore::from_save_dir(&save_dir)
        .read_slot("canonical")
        .expect("canonical slot must read back");
    let saved = outcome.save.canonical.expect("v3 canonical state");
    assert_eq!(saved, before);
    assert_eq!(
        outcome.save.player.unwrap().inventory,
        vec![ItemStack {
            base_form_id: 0x42,
            count: 2,
            condition: Some(80),
        }]
    );
    let _ = std::fs::remove_dir_all(save_dir);
}

#[test]
fn save_does_not_bootstrap_over_an_intentionally_empty_canonical_player() {
    let save_dir = std::env::temp_dir().join(format!(
        "bevyout-empty-canonical-save-test-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut world = test_world();
    world.init_resource::<super::super::preload::ResidentCells>();
    world.insert_resource(ActiveCell(0xC0DE));
    world.insert_resource(crate::viewer::LoadedSceneManifest(minimal_manifest(0xC0DE)));
    world.insert_resource(SaveDirectory(save_dir.clone()));
    world.insert_resource(interaction::PlayerInventory::from_stacks([(0x99, 99)]));

    let mut canonical = ItemLedger::new();
    canonical
        .insert_holder(
            HolderId::Player,
            ItemHolderState {
                revision: 11,
                ..Default::default()
            },
        )
        .unwrap();
    world.insert_resource(interaction::CanonicalItemLedger { ledger: canonical });

    write_save_slot(&mut world, "empty").expect("empty canonical save must write");

    let outcome = SaveStore::from_save_dir(&save_dir)
        .read_slot("empty")
        .expect("empty canonical slot must read back");
    let snapshot = outcome.save.canonical.expect("v3 canonical state");
    let player = &snapshot.holders[&HolderId::Player];
    assert!(player.items.is_empty());
    assert_eq!(player.revision, 11);
    assert!(outcome.save.player.unwrap().inventory.is_empty());
    let _ = std::fs::remove_dir_all(save_dir);
}

// Issue #76 (F76.2/F76.3): a looted, leveled-resolved container survives
// capture -> despawn/evict -> respawn -> apply, seeding
// `interaction::ContainerStates` with the exact stacks and resolved marker it had
// when the cell was left, the same swap-away-and-back shape
// `capture_and_apply_restore_a_moved_placement_transform` exercises for
// transforms.
#[test]
fn capture_and_apply_restore_looted_container_state() {
    let mut world = test_world();
    let placements = [container_placement(
        0x900,
        vec![inventory_entry(0x10, 3, false)],
    )];
    let (root, _children) = spawn_cell(&mut world, &placements);
    world
        .resource_mut::<interaction::ContainerStates>()
        .0
        .insert(
            0x900,
            interaction::container_policy::ContainerState {
                stacks: vec![(0x10, 1)],
                resolved: true,
            },
        );

    capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
    let saved = &world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x900];
    assert_eq!(
        saved.inventory,
        Some(vec![ItemStack {
            base_form_id: 0x10,
            count: 1,
            condition: None,
        }])
    );
    assert_eq!(saved.leveled_resolved, Some(true));

    // Simulate eviction + fresh respawn with a cleared runtime state.
    world.entity_mut(root).despawn();
    world
        .resource_mut::<interaction::ContainerStates>()
        .0
        .clear();
    let (root, _children) = spawn_cell(&mut world, &placements);
    apply_cell_placements(&mut world, 0xC0DE, root, &placements);

    let restored = &world.resource::<interaction::ContainerStates>().0[&0x900];
    assert_eq!(restored.stacks, vec![(0x10, 1)]);
    assert!(restored.resolved);
}

// F118.3: corpse contents reuse the FormID-keyed ContainerStates and the
// existing OBJE inventory delta, so leaving and re-entering a cell does
// not lose or duplicate a looted stack.
#[test]
fn capture_and_apply_restore_looted_corpse_state() {
    let mut world = test_world();
    let mut corpse = container_placement(0x902, vec![inventory_entry(0x10, 3, false)]);
    corpse.semantic = PreparedSemantic::Corpse;
    corpse.base_kind = "ACHR".into();
    corpse.display_name = Some("Corpse".into());
    let placements = [corpse];
    let (root, _children) = spawn_cell(&mut world, &placements);
    world
        .resource_mut::<interaction::ContainerStates>()
        .0
        .insert(
            0x902,
            interaction::container_policy::ContainerState {
                stacks: vec![(0x10, 1)],
                resolved: true,
            },
        );

    capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
    let saved = &world.resource::<ActiveSaveState>().0.cells[&0xC0DE].references[&0x902];
    assert_eq!(
        saved.inventory,
        Some(vec![ItemStack {
            base_form_id: 0x10,
            count: 1,
            condition: None,
        }])
    );
    assert_eq!(saved.leveled_resolved, Some(true));

    world.entity_mut(root).despawn();
    world
        .resource_mut::<interaction::ContainerStates>()
        .0
        .clear();
    let (root, _children) = spawn_cell(&mut world, &placements);
    apply_cell_placements(&mut world, 0xC0DE, root, &placements);

    let restored = &world.resource::<interaction::ContainerStates>().0[&0x902];
    assert_eq!(restored.stacks, vec![(0x10, 1)]);
    assert!(restored.resolved);
}

// F118.3 compatibility: an old save with no corpse section does not
// synthesize a corpse runtime holder during apply.
#[test]
fn apply_old_save_without_corpse_delta_leaves_corpse_unseeded() {
    let mut world = test_world();
    let mut corpse = container_placement(0x903, vec![inventory_entry(0x10, 3, false)]);
    corpse.semantic = PreparedSemantic::Corpse;
    corpse.base_kind = "ACHR".into();
    let placements = [corpse];
    let (root, _children) = spawn_cell(&mut world, &placements);

    apply_cell_placements(&mut world, 0xC0DE, root, &placements);

    assert!(
        !world
            .resource::<interaction::ContainerStates>()
            .0
            .contains_key(&0x903)
    );
}

// F76.2: a container whose stacks and resolved marker never diverge from
// the manifest baseline produces no delta and no seed on the next apply
// (it still rolls on first open, per F76.3).
#[test]
fn an_untouched_container_captures_and_seeds_nothing() {
    let mut world = test_world();
    let placements = [container_placement(
        0x901,
        vec![inventory_entry(0x10, 3, false)],
    )];
    let (root, _children) = spawn_cell(&mut world, &placements);
    world
        .resource_mut::<interaction::ContainerStates>()
        .0
        .insert(
            0x901,
            interaction::container_policy::ContainerState {
                stacks: vec![(0x10, 3)],
                resolved: false,
            },
        );

    capture_cell_placements(&mut world, 0xC0DE, root, &placements, false);
    assert!(
        !world
            .resource::<ActiveSaveState>()
            .0
            .cells
            .get(&0xC0DE)
            .is_some_and(|cell| cell.references.contains_key(&0x901))
    );

    world.entity_mut(root).despawn();
    world
        .resource_mut::<interaction::ContainerStates>()
        .0
        .clear();
    let (root, _children) = spawn_cell(&mut world, &placements);
    apply_cell_placements(&mut world, 0xC0DE, root, &placements);
    assert!(
        !world
            .resource::<interaction::ContainerStates>()
            .0
            .contains_key(&0x901)
    );
}
