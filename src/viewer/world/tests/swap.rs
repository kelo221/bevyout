use std::time::Duration;

use bevy::ecs::message::Messages;
use bevy::ecs::system::RunSystemOnce;
use bevy::state::app::StatesPlugin;

use super::*;
use crate::app_state::AppStatePlugin;

/// The synthetic hand-written fixture (never Bethesda-derived) also used
/// by `viewer::tests`; its `asset_root` is a relative path that exists
/// nowhere, so every destination's `scene_manifest_path(..).is_file()`
/// is false (fallback decision) in these tests.
fn fixture_manifest() -> PreparedSceneManifest {
    ron::de::from_str(include_str!("../../../../features/fixtures/scene.ron"))
        .expect("synthetic scene fixture should parse")
}

fn send(app: &mut App, request: RequestStateTransition) {
    app.world_mut()
        .resource_mut::<Messages<RequestStateTransition>>()
        .write(request);
    app.update();
    app.update();
}

fn current_modal(app: &App) -> GameplayModal {
    *app.world().resource::<State<GameplayModal>>().get()
}

fn drive_to_in_game(app: &mut App) {
    send(app, RequestStateTransition::App(AppState::Loading));
    send(app, RequestStateTransition::App(AppState::InGame));
    assert_eq!(
        *app.world().resource::<State<AppState>>().get(),
        AppState::InGame
    );
}

// T59.3: a failed fallback leaves the player entity's transform
// untouched, requests modal None, and queues the failure notice.
#[test]
fn a_failed_fallback_returns_to_source_untouched_with_a_notice() {
    let mut world = World::new();
    world.init_resource::<Messages<RequestStateTransition>>();
    world.init_resource::<interaction::InteractionNotice>();
    world.insert_resource(PendingFallbackResolution(Some(
        FallbackResolution::ReturnToSource {
            source_cell: 0xA,
            destination_cell: 0xB,
        },
    )));
    let player = world.spawn(Transform::from_xyz(1.0, 2.0, 3.0)).id();

    apply_fallback_resolution(&mut world);

    let transform = world.get::<Transform>(player).unwrap();
    assert_eq!(transform.translation, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(
        world.resource::<interaction::InteractionNotice>().text(),
        "Loading failed — returned to 0000000a"
    );
    let messages = world.resource::<Messages<RequestStateTransition>>();
    let requests: Vec<_> = messages.get_cursor().read(messages).cloned().collect();
    assert_eq!(
        requests,
        vec![RequestStateTransition::Modal(GameplayModal::None)]
    );
    assert!(world.resource::<PendingFallbackResolution>().0.is_none());
}

// T59.3: the failure notice names the source cell when the (never
// repointed) active manifest is that cell's and carries a display name.
#[test]
fn the_failure_notice_names_the_source_cell_when_known() {
    let mut world = World::new();
    world.init_resource::<Messages<RequestStateTransition>>();
    world.init_resource::<interaction::InteractionNotice>();
    let mut manifest = fixture_manifest();
    let source_cell = manifest.cell.form_id;
    manifest.cell.name = Some("Vault 101 Atrium".into());
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));
    world.insert_resource(PendingFallbackResolution(Some(
        FallbackResolution::ReturnToSource {
            source_cell,
            destination_cell: 0xB,
        },
    )));

    apply_fallback_resolution(&mut world);

    assert_eq!(
        world.resource::<interaction::InteractionNotice>().text(),
        "Loading failed — returned to Vault 101 Atrium"
    );
}

// T59.3: Esc during `GameplayModal::Loading` cancels the in-flight
// fallback -- pending cleared, modal back to None, player transform
// untouched, cancellation notice (not the failure wording) shown.
#[test]
fn escape_during_loading_cancels_the_pending_fallback() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AppStatePlugin));
    app.init_resource::<interaction::InteractionNotice>();
    app.insert_resource(PendingFallbackSwap::default());
    app.add_systems(Update, cancel_fallback_on_escape);
    drive_to_in_game(&mut app);
    send(
        &mut app,
        RequestStateTransition::Modal(GameplayModal::Loading),
    );
    assert_eq!(current_modal(&app), GameplayModal::Loading);
    let player = app
        .world_mut()
        .spawn(Transform::from_xyz(4.0, 5.0, 6.0))
        .id();
    app.world_mut().resource_mut::<PendingFallbackSwap>().0 = Some(SwapRequest {
        source_cell: 0xA,
        destination_cell: 0xB,
        translation: Vec3::ZERO,
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        door_form_id: 0x99,
    });

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
    app.update();

    assert!(app.world().resource::<PendingFallbackSwap>().0.is_none());
    assert_eq!(current_modal(&app), GameplayModal::None);
    assert_eq!(
        app.world()
            .resource::<interaction::InteractionNotice>()
            .text(),
        "Loading cancelled"
    );
    let transform = app.world().get::<Transform>(player).unwrap();
    assert_eq!(transform.translation, Vec3::new(4.0, 5.0, 6.0));
}

// T59.3 (F59.3's second half): a superseding DoorTravelRequested
// replaces the pending fallback instead of being dropped, and the
// loading modal stays open for the new fallback.
#[test]
fn a_superseding_travel_request_replaces_the_pending_fallback() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AppStatePlugin));
    app.add_message::<interaction::DoorTravelRequested>();
    app.insert_resource(ActiveCell(0xA));
    app.init_resource::<ResidentCells>();
    app.insert_resource(PendingInstantSwap::default());
    app.insert_resource(PendingFallbackSwap::default());
    app.insert_resource(crate::viewer::LoadedSceneManifest(fixture_manifest()));
    app.add_systems(Update, evaluate_door_travel_requests);
    drive_to_in_game(&mut app);

    app.world_mut()
        .write_message(interaction::DoorTravelRequested {
            destination_cell_form_id: 0xB,
            translation: Vec3::ZERO,
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            door_form_id: 0x99,
        });
    app.update();
    app.update();
    assert_eq!(
        app.world()
            .resource::<PendingFallbackSwap>()
            .0
            .map(|request| request.destination_cell),
        Some(0xB)
    );
    assert_eq!(current_modal(&app), GameplayModal::Loading);

    app.world_mut()
        .write_message(interaction::DoorTravelRequested {
            destination_cell_form_id: 0xC,
            translation: Vec3::ZERO,
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            door_form_id: 0x99,
        });
    app.update();
    app.update();
    assert_eq!(
        app.world()
            .resource::<PendingFallbackSwap>()
            .0
            .map(|request| request.destination_cell),
        Some(0xC),
        "the superseding request must replace the pending fallback, not be dropped"
    );
    assert_eq!(current_modal(&app), GameplayModal::Loading);
}

// F59.2: the overlay's background alpha animates in and out instead of
// hard-cutting, and the root only turns Hidden once the out-fade ends.
#[test]
fn the_overlay_fades_in_and_out_instead_of_hard_cutting() {
    let mut world = World::new();
    world.init_resource::<LoadingOverlayFade>();
    world.insert_resource(Time::<()>::default());
    let overlay = world
        .spawn((
            LoadingOverlayRoot,
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, swap_policy::OVERLAY_MAX_ALPHA)),
            Visibility::Hidden,
        ))
        .id();
    let alpha = |world: &World| world.get::<BackgroundColor>(overlay).unwrap().0.alpha();
    let visibility = |world: &World| *world.get::<Visibility>(overlay).unwrap();

    world.run_system_once(show_loading_overlay).unwrap();
    assert_eq!(visibility(&world), Visibility::Inherited);
    assert_eq!(alpha(&world), 0.0);

    world
        .resource_mut::<Time>()
        .advance_by(Duration::from_millis(100));
    world.run_system_once(advance_loading_overlay_fade).unwrap();
    let mid_in = alpha(&world);
    assert!(mid_in > 0.0 && mid_in < swap_policy::OVERLAY_MAX_ALPHA);

    world
        .resource_mut::<Time>()
        .advance_by(Duration::from_millis(200));
    world.run_system_once(advance_loading_overlay_fade).unwrap();
    assert_eq!(alpha(&world), swap_policy::OVERLAY_MAX_ALPHA);
    assert_eq!(visibility(&world), Visibility::Inherited);

    world.run_system_once(hide_loading_overlay).unwrap();
    world
        .resource_mut::<Time>()
        .advance_by(Duration::from_millis(100));
    world.run_system_once(advance_loading_overlay_fade).unwrap();
    let mid_out = alpha(&world);
    assert!(mid_out > 0.0 && mid_out < swap_policy::OVERLAY_MAX_ALPHA);
    assert_eq!(visibility(&world), Visibility::Inherited);

    world
        .resource_mut::<Time>()
        .advance_by(Duration::from_millis(200));
    world.run_system_once(advance_loading_overlay_fade).unwrap();
    assert_eq!(alpha(&world), 0.0);
    assert_eq!(visibility(&world), Visibility::Hidden);
}
