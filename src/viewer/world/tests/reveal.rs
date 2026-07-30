use super::*;

fn spawn_destination_with_children(world: &mut World, count: usize) -> (Entity, Vec<Entity>) {
    let root = world.spawn((Transform::default(), Visibility::Hidden)).id();
    let mut children = Vec::new();
    for i in 0..count {
        let placement = crate::vsa::PreparedPlacement {
            reference_form_id: i as u32,
            base_form_id: 0,
            asset_path: None,
            translation: [i as f32, 0.0, 0.0],
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
        };
        let entity = world
            .spawn((
                interaction::PlacementRoot::new(placement),
                Visibility::Inherited,
                ChildOf(root),
            ))
            .id();
        children.push(entity);
    }
    (root, children)
}

fn is_hidden(world: &World, entity: Entity) -> bool {
    *world.get::<Visibility>(entity).unwrap() == Visibility::Hidden
}

fn is_inherited(world: &World, entity: Entity) -> bool {
    *world.get::<Visibility>(entity).unwrap() == Visibility::Inherited
}

// T55.2: a cell at or under one chunk's worth of entities reveals fully
// in the activation call, with nothing left pending.
#[test]
fn a_small_cell_reveals_fully_in_one_call() {
    let mut world = World::new();
    world.init_resource::<PendingReveal>();
    world.init_resource::<RevealTelemetry>();
    let (root, children) = spawn_destination_with_children(&mut world, 5);

    begin_chunked_reveal(&mut world, 0x100, root, Vec3::ZERO, 128);

    for &entity in &children {
        assert!(is_inherited(&world, entity), "entity should be visible");
    }
    assert!(world.resource::<PendingReveal>().0.is_none());
}

// T55.3: revealing a large cell across frames leaves no entity
// permanently hidden.
#[test]
fn revealing_across_frames_leaves_no_entity_permanently_hidden() {
    let mut world = World::new();
    world.init_resource::<PendingReveal>();
    world.init_resource::<RevealTelemetry>();
    let (root, children) = spawn_destination_with_children(&mut world, 10);

    begin_chunked_reveal(&mut world, 0x100, root, Vec3::ZERO, 3);

    // First chunk (nearest the arrival point) is visible immediately;
    // the rest are forced hidden pending drain.
    let hidden_count = children
        .iter()
        .filter(|&&entity| is_hidden(&world, entity))
        .count();
    assert_eq!(hidden_count, 7); // 10 - first chunk of 3
    assert!(world.resource::<PendingReveal>().0.is_some());

    // Drain every remaining chunk.
    for _ in 0..10 {
        advance_pending_reveal(&mut world);
    }

    for &entity in &children {
        assert!(
            is_inherited(&world, entity),
            "entity {entity:?} was left hidden after the reveal fully drained"
        );
    }
    assert!(world.resource::<PendingReveal>().0.is_none());
}

// T55.3: an interrupting second swap mid-reveal fast-forwards the first
// reveal instead of stranding its remaining hidden entities.
#[test]
fn an_interrupting_second_swap_does_not_strand_hidden_entities() {
    let mut world = World::new();
    world.init_resource::<PendingReveal>();
    world.init_resource::<RevealTelemetry>();
    let (root_a, children_a) = spawn_destination_with_children(&mut world, 10);

    begin_chunked_reveal(&mut world, 0x100, root_a, Vec3::ZERO, 3);
    assert!(world.resource::<PendingReveal>().0.is_some());

    // Interrupt before cell A's reveal finishes draining.
    let (root_b, children_b) = spawn_destination_with_children(&mut world, 4);
    begin_chunked_reveal(&mut world, 0x200, root_b, Vec3::ZERO, 3);

    // Cell A's entities must all be visible now (fast-forwarded), not
    // stranded hidden.
    for &entity in &children_a {
        assert!(
            is_inherited(&world, entity),
            "cell A entity {entity:?} was stranded hidden by the interrupting swap"
        );
    }

    // Cell B's reveal is now the pending one, with its own nearest
    // chunk already visible.
    let hidden_count = children_b
        .iter()
        .filter(|&&entity| is_hidden(&world, entity))
        .count();
    assert_eq!(hidden_count, 1); // 4 - first chunk of 3
    assert_eq!(
        world
            .resource::<PendingReveal>()
            .0
            .as_ref()
            .unwrap()
            .form_id,
        0x200
    );
}

// Eviction safety: a despawned entity referenced by a still-pending
// reveal chunk is skipped, not panicked on.
#[test]
fn advancing_a_reveal_with_a_despawned_entity_does_not_panic() {
    let mut world = World::new();
    world.init_resource::<PendingReveal>();
    world.init_resource::<RevealTelemetry>();
    let (root, children) = spawn_destination_with_children(&mut world, 6);

    begin_chunked_reveal(&mut world, 0x100, root, Vec3::ZERO, 2);
    // Despawn an entity still sitting in a queued tail chunk.
    let victim = *children.last().unwrap();
    world.despawn(victim);

    for _ in 0..5 {
        advance_pending_reveal(&mut world);
    }
    assert!(world.resource::<PendingReveal>().0.is_none());
}
