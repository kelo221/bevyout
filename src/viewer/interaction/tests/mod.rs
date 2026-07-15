use super::*;

fn grant(inventory: &mut PlayerInventory, base_form_id: u32, count: i32) {
    let _ = inventory.add_stack(InventoryStack {
        base_form_id,
        count,
        condition: None,
    });
}

#[test]
fn inventory_accumulates_whole_stacks() {
    let mut inventory = PlayerInventory::default();
    grant(&mut inventory, 0x1234, 3);
    grant(&mut inventory, 0x1234, 2);
    assert_eq!(inventory.count(0x1234), 5);
    assert!(inventory.contains(0x1234));
}

// F75.3/#71: `add_stack`/`remove` are the sanctioned container-transfer
// seam -- non-positive counts are a no-op (`InvalidCount`), and `remove` is
// atomic: removing more than is held is rejected outright rather than
// flooring at zero (see `inventory::Inventory::remove`, and
// `failed_remove_is_atomic` in that module for the same contract).
#[test]
fn grant_ignores_non_positive_counts() {
    let mut inventory = PlayerInventory::default();
    grant(&mut inventory, 0x1234, 0);
    grant(&mut inventory, 0x1234, -5);
    assert_eq!(inventory.count(0x1234), 0);
}

#[test]
fn remove_more_than_held_is_rejected_atomically() {
    let mut inventory = PlayerInventory::default();
    grant(&mut inventory, 0x1234, 3);
    let key = StackKey {
        base_form_id: 0x1234,
        condition: None,
    };
    assert_eq!(inventory.remove(key, 10), TransferResult::InsufficientItems);
    assert_eq!(inventory.count(0x1234), 3);
}

#[test]
fn remove_ignores_non_positive_counts() {
    let mut inventory = PlayerInventory::default();
    grant(&mut inventory, 0x1234, 3);
    let key = StackKey {
        base_form_id: 0x1234,
        condition: None,
    };
    assert_eq!(inventory.remove(key, 0), TransferResult::InvalidCount);
    assert_eq!(inventory.remove(key, -1), TransferResult::InvalidCount);
    assert_eq!(inventory.count(0x1234), 3);
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
    grant(&mut inventory, 0x42, 1);
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

#[test]
fn interaction_prompts_use_e_in_fps_mode() {
    let placement = PreparedPlacement {
        reference_form_id: 1,
        base_form_id: 2,
        asset_path: None,
        translation: [0.0; 3],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: Default::default(),
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: "CONT".into(),
        base_kind: "CONT".into(),
        editor_id: Some("TestContainer".into()),
        display_name: Some("Test Container".into()),
        count: 1,
        semantic: PreparedSemantic::Container,
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: "ao-none".into(),
    };

    let prompt = interaction_prompt(&placement, false, &PlayerInventory::default())
        .expect("containers should have an interaction prompt");
    assert!(prompt.starts_with("[E]"));
    assert!(!prompt.contains("Enter"));
}

#[test]
fn probe_status_distinguishes_reference_static_and_no_target() {
    assert_eq!(
        probe_status_message(true, Some("VaultDoorRef (0007b240)")),
        "probe: VaultDoorRef (0007b240)"
    );
    assert_eq!(
        probe_status_message(true, None),
        "probe: NOT_IMPLEMENTED (static-batched geometry)"
    );
    assert_eq!(probe_status_message(false, None), "probe: no target");
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
        .insert_resource(CameraModeState {
            mode: CameraMode::Fps,
            ..default()
        })
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

    /// Focuses `entity` and presses E for exactly one `app.update()`.
    fn activate(app: &mut App, entity: Entity) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.world_mut().resource_mut::<InteractionState>().focused = Some(entity);
        app.update();
        // `release()` alone doesn't clear `just_pressed` (that's the real
        // `InputPlugin`'s job, which this bare-`MinimalPlugins` app doesn't
        // run) -- without this, every subsequent `app.update()` in a test
        // would see `just_pressed(Enter)` still true and re-activate.
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(KeyCode::KeyE);
        keys.clear_just_pressed(KeyCode::KeyE);
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

// T75.3: Bevy-side container-transfer integration, driven end-to-end
// through `activate_focused_placement`/`transfer_ui` (a bare `App` plus
// `AppStatePlugin`, needed here -- unlike `door_travel_animation` above --
// because opening/closing the transfer modal goes through real
// `GameplayModal` transitions and their `OnEnter`/`OnExit` hooks).
mod container_transfer {
    use super::*;
    use crate::app_state::{AppState, AppStatePlugin, GameplayModal, RequestStateTransition};
    use crate::vsa::PreparedPhysicsClassification;
    use bevy::ecs::message::Messages;
    use bevy::mesh::MeshPlugin;
    use bevy::state::app::StatesPlugin;
    use bevy::time::TimeUpdateStrategy;
    use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow, Window};
    use std::time::Duration;

    fn container_placement(inventory: Vec<PreparedInventoryEntry>) -> PreparedPlacement {
        let mut placement = PreparedPlacement {
            reference_form_id: 0x0003_0001,
            base_form_id: 0x2,
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
            base_kind: "CONT".into(),
            editor_id: Some("TestContainer".into()),
            display_name: Some("Footlocker".into()),
            count: 1,
            semantic: PreparedSemantic::Container,
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory,
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        };
        // `PreparedPlacementAudio` isn't reachable from here (`vsa::manifest`
        // is private to the `vsa` slice) -- its fields are `pub(crate)`
        // though, so setting them by name on an already-built default value
        // sidesteps needing the type name at all.
        placement.audio.open_sound_form_id = Some(0x100);
        placement.audio.close_sound_form_id = Some(0x101);
        placement
    }

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            TransformPlugin,
            AssetPlugin::default(),
            MeshPlugin,
            AppStatePlugin,
        ))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1.0 / 60.0,
        )))
        .insert_resource(ButtonInput::<KeyCode>::default())
        .insert_resource(ButtonInput::<MouseButton>::default())
        .insert_resource(CameraModeState {
            mode: CameraMode::Fps,
            ..default()
        })
        .insert_resource(RefRegistry::default())
        .insert_resource(crate::console::ConsoleSessionStore::default())
        .insert_resource(PreparedItemCatalog::default());
        install(&mut app);
        app.add_message::<animation::PlayPlacementAnimation>();
        app.add_message::<PlaySound>();
        app.world_mut().spawn((Window::default(), PrimaryWindow));
        drive_to_in_game(&mut app);
        app
    }

    fn send(app: &mut App, request: RequestStateTransition) {
        app.world_mut()
            .resource_mut::<Messages<RequestStateTransition>>()
            .write(request);
        app.update();
        app.update();
    }

    fn drive_to_in_game(app: &mut App) {
        send(app, RequestStateTransition::App(AppState::Loading));
        send(app, RequestStateTransition::App(AppState::InGame));
        assert_eq!(
            *app.world().resource::<State<AppState>>().get(),
            AppState::InGame
        );
    }

    fn current_modal(app: &App) -> GameplayModal {
        *app.world().resource::<State<GameplayModal>>().get()
    }

    /// Focuses `entity`, presses E, and updates twice: once for
    /// `activate_focused_placement` to write the modal request (and
    /// `apply_state_transition_requests` to queue it in `PostUpdate`), once
    /// more for `StateTransition` to apply it and fire `OnEnter(Dialogue)`
    /// (see `world::swap`'s tests for the same two-update pattern).
    fn open_container(app: &mut App, entity: Entity) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.world_mut().resource_mut::<InteractionState>().focused = Some(entity);
        app.update();
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyE);
            keys.clear_just_pressed(KeyCode::KeyE);
        }
        app.update();
    }

    fn press_escape(app: &mut App) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
        app.update();
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::Escape);
            keys.clear_just_pressed(KeyCode::Escape);
        }
        app.update();
    }

    fn play_sounds(app: &App) -> Vec<u32> {
        let messages = app.world().resource::<Messages<PlaySound>>();
        messages
            .get_cursor()
            .read(messages)
            .map(|event| event.form_id)
            .collect()
    }

    fn cursor_state(app: &mut App) -> (bool, CursorGrabMode) {
        let mut query = app
            .world_mut()
            .query_filtered::<&CursorOptions, With<PrimaryWindow>>();
        let cursor = query.single(app.world()).unwrap();
        (cursor.visible, cursor.grab_mode)
    }

    // T75.3: activating a container opens the paused transfer modal, seeds
    // `ContainerStates` from its prepared inventory, plays the open sound,
    // and marks it `open` -- F75.2/F75.4.
    #[test]
    fn activating_a_container_opens_the_paused_transfer_modal() {
        let mut app = build_app();
        let inventory = vec![PreparedInventoryEntry {
            base_form_id: 0x10,
            count: 3,
            record_kind: "MISC".into(),
            editor_id: Some("Bottlecap".into()),
            display_name: None,
            leveled: false,
        }];
        let entity = app
            .world_mut()
            .spawn((
                PlacementRoot::new(container_placement(inventory)),
                GlobalTransform::default(),
            ))
            .id();

        open_container(&mut app, entity);

        assert_eq!(current_modal(&app), GameplayModal::Container);
        assert!(app.world().resource::<Time<Virtual>>().is_paused());
        assert!(
            app.world()
                .resource::<InteractionState>()
                .open
                .contains(&entity)
        );
        let stacks = app
            .world()
            .resource::<ContainerStates>()
            .get(0x0003_0001)
            .map(|state| state.stacks.clone());
        assert_eq!(stacks, Some(vec![(0x10, 3)]));
        assert_eq!(play_sounds(&app), vec![0x100]);
        let (visible, grab_mode) = cursor_state(&mut app);
        assert!(visible);
        assert_eq!(grab_mode, CursorGrabMode::None);
    }

    // T75.3: Esc closes the modal, restores the cursor/pause state, plays
    // the close sound, and clears the container's `open` marker -- F75.4.
    #[test]
    fn escape_closes_the_modal_and_restores_gameplay_state() {
        let mut app = build_app();
        let entity = app
            .world_mut()
            .spawn((
                PlacementRoot::new(container_placement(Vec::new())),
                GlobalTransform::default(),
            ))
            .id();
        open_container(&mut app, entity);
        assert_eq!(current_modal(&app), GameplayModal::Container);

        press_escape(&mut app);

        assert_eq!(current_modal(&app), GameplayModal::None);
        assert!(!app.world().resource::<Time<Virtual>>().is_paused());
        assert!(
            !app.world()
                .resource::<InteractionState>()
                .open
                .contains(&entity)
        );
        let (visible, grab_mode) = cursor_state(&mut app);
        assert!(!visible);
        assert_eq!(grab_mode, CursorGrabMode::Locked);
        assert!(play_sounds(&app).contains(&0x101));
    }

    // T75.3: gameplay input is blocked while the modal is open --
    // `activate_focused_placement` only runs in `GameplayModal::None` (see
    // `install`), so a second `E` press while the modal is up must not
    // re-run the container-open branch (no second open sound, modal stays
    // put).
    #[test]
    fn gameplay_input_is_blocked_while_the_modal_is_open() {
        let mut app = build_app();
        let entity = app
            .world_mut()
            .spawn((
                PlacementRoot::new(container_placement(Vec::new())),
                GlobalTransform::default(),
            ))
            .id();
        open_container(&mut app, entity);
        assert_eq!(current_modal(&app), GameplayModal::Container);
        assert_eq!(play_sounds(&app), vec![0x100]);

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update();

        assert_eq!(current_modal(&app), GameplayModal::Container);
        assert!(
            play_sounds(&app).is_empty(),
            "activate_focused_placement must not run while the modal is open"
        );
    }
}
