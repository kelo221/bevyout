use super::*;

#[test]
fn inventory_accumulates_whole_stacks() {
    let mut inventory = PlayerInventory::default();
    inventory.add(0x1234, 3);
    inventory.add(0x1234, 2);
    assert_eq!(inventory.count(0x1234), 5);
    assert!(inventory.contains(0x1234));
}

#[test]
fn locked_door_requires_its_key() {
    let door = PreparedDoor {
        lock_level: Some(50),
        key_form_id: Some(0x42),
        destination: None,
    };
    let mut inventory = PlayerInventory::default();
    assert!(door_is_locked(&door, &inventory));
    inventory.add(0x42, 1);
    assert!(!door_is_locked(&door, &inventory));
}

#[test]
fn lock_without_a_key_remains_locked() {
    let door = PreparedDoor {
        lock_level: Some(1),
        key_form_id: None,
        destination: None,
    };
    assert!(door_is_locked(&door, &PlayerInventory::default()));
}

#[test]
fn zero_lock_level_is_unlocked() {
    let door = PreparedDoor {
        lock_level: Some(0),
        key_form_id: Some(0x42),
        destination: None,
    };
    assert!(!door_is_locked(&door, &PlayerInventory::default()));
}

#[test]
fn container_summary_is_bounded() {
    let entries = (0..10)
        .map(|index| PreparedInventoryEntry {
            base_form_id: index,
            count: 1,
            record_kind: "MISC".into(),
            editor_id: Some(format!("Item{index}")),
            display_name: None,
            leveled: false,
        })
        .collect::<Vec<_>>();
    let summary = inventory_summary(&entries);
    assert!(summary.contains("Item0 x1"));
    assert!(summary.contains("+2 more"));
    assert!(!summary.contains("Item8"));
}

// T57.4: Bevy-side door-travel/animation integration, driven end-to-end
// through `activate_focused_placement`/`tick_pending_door_travel` (a bare
// `App`, not `animation::install` -- clip discovery is that module's own
// concern, covered by `animation::tests`).
mod door_travel_animation {
    use super::*;
    use crate::app_state::{AppState, GameplayModal};
    use crate::vsa::{PreparedDoorDestination, PreparedPhysicsClassification};
    use bevy::mesh::MeshPlugin;
    use bevy::state::app::StatesPlugin;
    use bevy::time::TimeUpdateStrategy;
    use std::time::Duration;

    fn door_placement(destination: Option<PreparedDoorDestination>) -> PreparedPlacement {
        PreparedPlacement {
            reference_form_id: 0x0002_8579,
            base_form_id: 1,
            asset_path: None,
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: None,
            physics_source: None,
            physics_classification: PreparedPhysicsClassification::Static,
            step_support: false,
            mutability: Default::default(),
            mutability_root_form_id: None,
            reference_kind: "REFR".into(),
            base_kind: "DOOR".into(),
            editor_id: Some("TestDoor".into()),
            display_name: None,
            count: 1,
            semantic: PreparedSemantic::Door(PreparedDoor {
                lock_level: None,
                key_form_id: None,
                destination,
            }),
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        }
    }

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            TransformPlugin,
            AssetPlugin::default(),
            MeshPlugin,
        ))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1.0 / 60.0,
        )))
        .insert_resource(ButtonInput::<KeyCode>::default())
        .insert_resource(RefRegistry::default())
        .insert_resource(crate::console::ConsoleSessionStore::default());
        app.insert_state(AppState::InGame);
        app.insert_state(GameplayModal::None);
        install(&mut app);
        app.add_message::<animation::PlayPlacementAnimation>();
        app.add_message::<PlaySound>();
        app.update();
        app
    }

    /// Focuses `entity` and presses Enter for exactly one `app.update()`.
    fn activate(app: &mut App, entity: Entity) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        app.world_mut().resource_mut::<InteractionState>().focused = Some(entity);
        app.update();
        // `release()` alone doesn't clear `just_pressed` (that's the real
        // `InputPlugin`'s job, which this bare-`MinimalPlugins` app doesn't
        // run) -- without this, every subsequent `app.update()` in a test
        // would see `just_pressed(Enter)` still true and re-activate.
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(KeyCode::Enter);
        keys.clear_just_pressed(KeyCode::Enter);
    }

    fn play_requests(app: &App) -> Vec<(Entity, ClipTransition)> {
        let messages = app
            .world()
            .resource::<Messages<animation::PlayPlacementAnimation>>();
        messages
            .get_cursor()
            .read(messages)
            .map(|event| (event.root, event.transition))
            .collect()
    }

    fn travel_requests(app: &App) -> Vec<DoorTravelRequested> {
        let messages = app.world().resource::<Messages<DoorTravelRequested>>();
        messages.get_cursor().read(messages).copied().collect()
    }

    // T57.4: opening a door with no discovered clip queues exactly one play
    // request for "Opening" and travels the same frame -- wave-2's
    // behavior preserved bit-for-bit when there's nothing to animate.
    #[test]
    fn opening_a_clipless_travel_door_queues_one_play_request_and_travels_same_frame() {
        let mut app = build_app();
        let destination = PreparedDoorDestination {
            door_reference_form_id: 0x99,
            cell_form_id: 0x0002_4511,
            translation: [1.0, 2.0, 3.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        };
        let entity = app
            .world_mut()
            .spawn((
                PlacementRoot::new(door_placement(Some(destination))),
                GlobalTransform::default(),
            ))
            .id();

        activate(&mut app, entity);

        let requests = play_requests(&app);
        assert_eq!(requests, vec![(entity, ClipTransition::Opening)]);

        let travel = travel_requests(&app);
        assert_eq!(travel.len(), 1);
        assert_eq!(travel[0].destination_cell_form_id, 0x0002_4511);
    }

    // T57.4: with a discovered "Open" clip, the deferred travel message is
    // written only once the lead elapses, not on the activation frame.
    #[test]
    fn opening_a_travel_door_with_a_clip_defers_travel_until_the_lead_elapses() {
        let mut app = build_app();
        let destination = PreparedDoorDestination {
            door_reference_form_id: 0x99,
            cell_form_id: 0x0002_4511,
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        };
        let entity = app
            .world_mut()
            .spawn((
                PlacementRoot::new(door_placement(Some(destination))),
                GlobalTransform::default(),
            ))
            .id();
        // Stand in for animation.rs's discovery pipeline (that module's own
        // concern; see `animation::tests`), giving this door a 0.2s "Open"
        // clip -- comfortably under the 0.6s cap.
        app.world_mut()
            .entity_mut(entity)
            .insert(animation::AnimatedPlacement::for_test(
                entity,
                &[("Open", 0.2), ("Close", 0.2)],
            ));

        activate(&mut app, entity);

        assert_eq!(play_requests(&app), vec![(entity, ClipTransition::Opening)]);
        assert!(
            travel_requests(&app).is_empty(),
            "a 0.2s lead must not have elapsed on the activation frame"
        );

        // 12 more frames at 1/60s (~0.2s) elapses the lead.
        for _ in 0..12 {
            app.update();
        }
        let travel = travel_requests(&app);
        assert_eq!(travel.len(), 1);
        assert_eq!(travel[0].destination_cell_form_id, 0x0002_4511);
    }

    // F57.4: re-activating mid-animation reverses cleanly -- a second
    // Enter press while a lead is still pending closes the door and cancels
    // the stale pending travel instead of letting it still fire later.
    #[test]
    fn closing_before_the_lead_elapses_cancels_the_pending_travel() {
        let mut app = build_app();
        let destination = PreparedDoorDestination {
            door_reference_form_id: 0x99,
            cell_form_id: 0x0002_4511,
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        };
        let entity = app
            .world_mut()
            .spawn((
                PlacementRoot::new(door_placement(Some(destination))),
                GlobalTransform::default(),
            ))
            .id();
        app.world_mut()
            .entity_mut(entity)
            .insert(animation::AnimatedPlacement::for_test(
                entity,
                &[("Open", 0.5), ("Close", 0.5)],
            ));

        activate(&mut app, entity);
        assert!(travel_requests(&app).is_empty());

        // Closing again before the 0.5s lead elapses cancels it.
        activate(&mut app, entity);
        assert_eq!(
            play_requests(&app)
                .into_iter()
                .next_back()
                .map(|(_, transition)| transition),
            Some(ClipTransition::Closing)
        );

        // Advance well past where the original 0.5s lead would have fired,
        // checking every frame -- `Messages` only retains two frames, so a
        // stale write could otherwise be pruned before a single check at
        // the end of the loop ever saw it.
        for _ in 0..40 {
            app.update();
            assert!(
                travel_requests(&app).is_empty(),
                "closing before the lead elapsed must cancel the pending travel"
            );
        }
    }
}
