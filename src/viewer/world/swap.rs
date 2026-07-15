//! Bevy-side driver for issue #52's instant door-transition cell swap.
//!
//! A door activation with a resolved destination (`interaction::
//! DoorTravelRequested`, written by `activate_focused_placement`) is
//! evaluated the same frame by `evaluate_door_travel_requests` against
//! `swap_policy::swap_decision`: a resident, `Ready` destination swaps
//! instantly (`apply_pending_instant_swap`, an exclusive system chained
//! immediately after); anything else enters the `GameplayModal::Loading`
//! fallback, reusing `preload.rs`'s background manifest parse
//! (`spawn_preload_parse_task`) to load the destination on demand, then
//! runs the exact same activation steps once it becomes resident `Ready`
//! (`check_fallback_progress` / `apply_fallback_resolution`). A fallback
//! whose parse fails (`preload::PreloadParseFailed`) returns the player to
//! the source cell untouched.
//!
//! Issue #59 adds the fallback lifecycle around that
//! (`swap_policy::fallback_lifecycle_outcome`): the loading overlay's
//! background alpha fades in and out (`advance_loading_overlay_fade`,
//! driven by `swap_policy::fade_in_alpha`/`fade_out_alpha`), Esc cancels an
//! in-flight fallback (`cancel_fallback_on_escape` -- Esc is otherwise
//! unbound while `GameplayModal::Loading` is up, see
//! `app_state::toggle_paused_on_escape`), a superseding
//! `DoorTravelRequested` replaces a pending fallback instead of being
//! dropped, and a failed or cancelled fallback surfaces a player-visible
//! notice through `interaction::InteractionNotice`. Instant swaps never
//! enter `GameplayModal::Loading`, so none of this touches them.
//!
//! Nothing here is ported from OpenMW; it is bevyout's own design against
//! the door-transition flow read (not copied) from
//! `apps/openmw/mwworld/scene.cpp`'s `changeCellByMovingPlayer`.

use std::collections::HashMap;
use std::path::Path;

use bevy::prelude::*;

use crate::app_state::{AppState, GameplayModal, RequestStateTransition};
use crate::console::RefRegistry;
use crate::vsa::PreparedSceneManifest;

use super::super::{audio, interaction, player, scene};
use super::preload::{
    ActiveCell, PreloadParseFailed, ResidentCells, ResidentState, scene_manifest_path,
    spawn_preload_parse_task,
};
use super::reveal;
use super::swap_policy;

#[derive(Debug, Clone, Copy, PartialEq)]
struct SwapRequest {
    source_cell: u32,
    destination_cell: u32,
    translation: Vec3,
    rotation_xyzw: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SwapKind {
    Instant,
    Fallback,
}

#[derive(Resource, Default)]
struct PendingInstantSwap(Option<SwapRequest>);

#[derive(Resource, Default)]
struct PendingFallbackSwap(Option<SwapRequest>);

enum FallbackResolution {
    Proceed(SwapRequest),
    ReturnToSource {
        source_cell: u32,
        destination_cell: u32,
    },
}

#[derive(Resource, Default)]
struct PendingFallbackResolution(Option<FallbackResolution>);

/// Frames measured per transition (F52.5): the swap frame itself plus the
/// next 30.
const SWAP_TELEMETRY_FRAMES: u32 = 31;

struct SwapMeasurement {
    source_cell: u32,
    destination_cell: u32,
    kind: SwapKind,
    frames_remaining: u32,
    max_frame_ms: f64,
}

#[derive(Resource, Default)]
struct SwapTelemetry(Option<SwapMeasurement>);

#[derive(Component)]
struct LoadingOverlayRoot;

/// F59.2: where the loading overlay's background fade currently is. The
/// elapsed seconds feed `swap_policy::fade_in_alpha`/`fade_out_alpha`;
/// `Hidden`/`Shown` are the settled endpoints where no per-frame work
/// happens.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum OverlayFadePhase {
    #[default]
    Hidden,
    FadingIn {
        elapsed_seconds: f32,
    },
    Shown,
    FadingOut {
        elapsed_seconds: f32,
    },
}

#[derive(Resource, Default)]
struct LoadingOverlayFade(OverlayFadePhase);

pub(crate) fn install(app: &mut App) {
    app.insert_resource(PendingInstantSwap::default())
        .insert_resource(PendingFallbackSwap::default())
        .insert_resource(PendingFallbackResolution::default())
        .insert_resource(SwapTelemetry::default())
        .insert_resource(LoadingOverlayFade::default())
        .add_systems(Startup, spawn_loading_overlay_ui)
        .add_systems(OnEnter(GameplayModal::Loading), show_loading_overlay)
        .add_systems(OnExit(GameplayModal::Loading), hide_loading_overlay)
        .add_systems(
            Update,
            (
                evaluate_door_travel_requests.after(interaction::DoorActivationSet),
                apply_pending_instant_swap.after(evaluate_door_travel_requests),
                // F59.3: Esc-cancel is evaluated before this frame's
                // progress check so a same-frame race resolves in the
                // player's favor (the cancel wins; a destination that
                // happened to become ready this frame stays resident for
                // the next attempt).
                cancel_fallback_on_escape
                    .after(evaluate_door_travel_requests)
                    .before(check_fallback_progress),
                check_fallback_progress.after(evaluate_door_travel_requests),
                apply_fallback_resolution.after(check_fallback_progress),
                // F59.2: ticks the overlay's background alpha; runs
                // unconditionally within InGame (the out-fade happens after
                // `GameplayModal::Loading` has already exited).
                advance_loading_overlay_fade,
                measure_swap_frame_times
                    .after(apply_pending_instant_swap)
                    .after(apply_fallback_resolution),
                // Issue #55: drains one bounded reveal chunk per frame and
                // logs the `reveal ...` telemetry line once its window
                // elapses. Ordered after both activation systems so the
                // very first sample in `measure_reveal_frame_time`'s window
                // is the activation frame itself.
                reveal::advance_pending_reveal
                    .after(apply_pending_instant_swap)
                    .after(apply_fallback_resolution),
                reveal::measure_reveal_frame_time
                    .after(apply_pending_instant_swap)
                    .after(apply_fallback_resolution),
            )
                .run_if(in_state(AppState::InGame)),
        );
}

/// F52.1/F52.2/F52.4 entry point: resolves each door-travel request's
/// `SwapDecision` from the destination's residency and stages the
/// corresponding resource for the exclusive systems chained right after
/// this one to consume the same frame (Bevy's message double-buffering
/// swaps once per frame in `First`, not between systems within a frame, so
/// a message written by `activate_focused_placement` earlier this frame is
/// still visible here).
/// F59.3's superseding rule: a `DoorTravelRequested` arriving while a
/// fallback is still pending replaces that fallback (the lifecycle table's
/// `Supersede`: cancel the old, then evaluate the new request exactly like
/// a fresh one) instead of being dropped. Only an instant swap staged this
/// same frame -- consumed by `apply_pending_instant_swap` immediately after
/// this system -- still ignores a second request.
#[allow(clippy::too_many_arguments)]
fn evaluate_door_travel_requests(
    mut commands: Commands,
    mut requests: MessageReader<interaction::DoorTravelRequested>,
    active_cell: Res<ActiveCell>,
    resident_cells: Res<ResidentCells>,
    manifest: Res<PreparedSceneManifest>,
    modal: Res<State<GameplayModal>>,
    mut pending_instant: ResMut<PendingInstantSwap>,
    mut pending_fallback: ResMut<PendingFallbackSwap>,
    mut state_requests: MessageWriter<RequestStateTransition>,
) {
    let was_loading_modal = *modal.get() == GameplayModal::Loading;
    let mut superseded_a_fallback = false;
    for request in requests.read() {
        if pending_instant.0.is_some() {
            warn!(
                "door travel to {:08x} ignored; a swap is already in progress",
                request.destination_cell_form_id
            );
            continue;
        }
        if let Some(old) = pending_fallback.0.take() {
            debug_assert_eq!(
                swap_policy::fallback_lifecycle_outcome(
                    swap_policy::FallbackState::InFlight,
                    swap_policy::FallbackEvent::SupersedingRequest,
                ),
                swap_policy::FallbackLifecycleOutcome::Supersede
            );
            info!(
                "swap {:08x}->{:08x} fallback cancelled (superseded by door travel to {:08x})",
                old.source_cell, old.destination_cell, request.destination_cell_form_id
            );
            superseded_a_fallback = true;
        }
        let destination = request.destination_cell_form_id;
        let residency = match resident_cells
            .0
            .get(&destination)
            .map(|resident| resident.state)
        {
            Some(ResidentState::Ready) => swap_policy::Residency::Ready,
            Some(ResidentState::Loading) => swap_policy::Residency::Loading,
            None => swap_policy::Residency::Absent,
        };
        let asset_root = Path::new(&manifest.asset_root);
        let manifest_exists = scene_manifest_path(asset_root, destination).is_file();
        let swap_request = SwapRequest {
            source_cell: active_cell.0,
            destination_cell: destination,
            translation: request.translation,
            rotation_xyzw: request.rotation_xyzw,
        };
        match swap_policy::swap_decision(manifest_exists, residency) {
            swap_policy::SwapDecision::Instant => {
                pending_instant.0 = Some(swap_request);
            }
            swap_policy::SwapDecision::Fallback => {
                if residency == swap_policy::Residency::Absent {
                    spawn_preload_parse_task(&mut commands, asset_root, destination);
                }
                pending_fallback.0 = Some(swap_request);
            }
        }
    }
    // Net modal effect, written once after the loop so a supersede never
    // requests an illegal Loading->Loading (or a doubled None->Loading)
    // transition: a fresh fallback opens the overlay; a superseding request
    // that resolved Instant closes the one the cancelled fallback opened.
    let has_fallback_now = pending_fallback.0.is_some();
    if has_fallback_now && !was_loading_modal {
        state_requests.write(RequestStateTransition::Modal(GameplayModal::Loading));
    } else if !has_fallback_now && was_loading_modal && superseded_a_fallback {
        state_requests.write(RequestStateTransition::Modal(GameplayModal::None));
    }
}

/// F52.2: performs the whole instant swap in this same frame.
fn apply_pending_instant_swap(world: &mut World) {
    let Some(request) = world.resource_mut::<PendingInstantSwap>().0.take() else {
        return;
    };
    activate_resident_cell(world, request, SwapKind::Instant);
}

/// F59.3: Esc while the loading overlay is up cancels the in-flight
/// fallback cleanly -- pending state cleared, modal returned to `None`
/// (fading the overlay out), player untouched at the source door, notice
/// shown. Esc is otherwise unbound during `GameplayModal::Loading`
/// (`app_state::toggle_paused_on_escape` explicitly ignores it), so this
/// binding conflicts with nothing. The already-spawned background parse
/// task is deliberately left to finish: at worst the destination becomes a
/// resident preloaded cell, making the retry instant.
fn cancel_fallback_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    modal: Res<State<GameplayModal>>,
    mut pending_fallback: ResMut<PendingFallbackSwap>,
    mut notice: ResMut<interaction::InteractionNotice>,
    mut state_requests: MessageWriter<RequestStateTransition>,
) {
    if *app_state.get() != AppState::InGame
        || *modal.get() != GameplayModal::Loading
        || !keys.just_pressed(KeyCode::Escape)
    {
        return;
    }
    let Some(request) = pending_fallback.0.take() else {
        return;
    };
    debug_assert_eq!(
        swap_policy::fallback_lifecycle_outcome(
            swap_policy::FallbackState::InFlight,
            swap_policy::FallbackEvent::PlayerCancelled,
        ),
        swap_policy::FallbackLifecycleOutcome::Cancel
    );
    info!(
        "swap {:08x}->{:08x} fallback cancelled",
        request.source_cell, request.destination_cell
    );
    notice.show("Loading cancelled");
    state_requests.write(RequestStateTransition::Modal(GameplayModal::None));
}

/// F52.4: watches a pending fallback swap's destination cell until it
/// becomes resident `Ready` (via the reused `preload.rs` background parse)
/// or its parse fails, and stages the resolution for
/// `apply_fallback_resolution` to consume the same frame.
fn check_fallback_progress(
    resident_cells: Res<ResidentCells>,
    mut pending_fallback: ResMut<PendingFallbackSwap>,
    mut parse_failed: MessageReader<PreloadParseFailed>,
    mut resolution: ResMut<PendingFallbackResolution>,
) {
    let failed_ids: Vec<u32> = parse_failed.read().map(|failure| failure.form_id).collect();
    let state = if pending_fallback.0.is_some() {
        swap_policy::FallbackState::InFlight
    } else {
        swap_policy::FallbackState::Idle
    };
    let Some(request) = pending_fallback.0 else {
        return;
    };
    if failed_ids.contains(&request.destination_cell) {
        debug_assert_eq!(
            swap_policy::fallback_lifecycle_outcome(state, swap_policy::FallbackEvent::ParseFailed),
            swap_policy::FallbackLifecycleOutcome::ReturnToSource
        );
        resolution.0 = Some(FallbackResolution::ReturnToSource {
            source_cell: request.source_cell,
            destination_cell: request.destination_cell,
        });
        pending_fallback.0 = None;
        return;
    }
    let ready = resident_cells
        .0
        .get(&request.destination_cell)
        .is_some_and(|resident| resident.state == ResidentState::Ready);
    if ready {
        debug_assert_eq!(
            swap_policy::fallback_lifecycle_outcome(
                state,
                swap_policy::FallbackEvent::DestinationReady
            ),
            swap_policy::FallbackLifecycleOutcome::Proceed
        );
        resolution.0 = Some(FallbackResolution::Proceed(request));
        pending_fallback.0 = None;
    }
}

/// F52.4: applies whatever `check_fallback_progress` resolved this frame --
/// either the same activation steps the instant path uses, or a warned
/// `ReturnToSource` that leaves the player exactly where they were. Either
/// way, closes the loading overlay by returning `GameplayModal` to `None`.
///
/// F59.4: `ReturnToSource` additionally surfaces a player-visible notice
/// through the interaction HUD, naming the source cell the player is still
/// standing in (the active `PreparedSceneManifest` was never repointed, so
/// its `cell` metadata is that source cell's).
fn apply_fallback_resolution(world: &mut World) {
    let Some(resolution) = world.resource_mut::<PendingFallbackResolution>().0.take() else {
        return;
    };
    match resolution {
        FallbackResolution::Proceed(request) => {
            activate_resident_cell(world, request, SwapKind::Fallback);
        }
        FallbackResolution::ReturnToSource {
            source_cell,
            destination_cell,
        } => {
            warn!(
                "swap {source_cell:08x}->{destination_cell:08x} fallback load failed; returning to source cell"
            );
            let source_label = world
                .get_resource::<PreparedSceneManifest>()
                .filter(|manifest| manifest.cell.form_id == source_cell)
                .and_then(|manifest| {
                    manifest
                        .cell
                        .name
                        .clone()
                        .or_else(|| manifest.cell.editor_id.clone())
                })
                .unwrap_or_else(|| format!("{source_cell:08x}"));
            if let Some(mut notice) = world.get_resource_mut::<interaction::InteractionNotice>() {
                notice.show(format!("Loading failed — returned to {source_label}"));
            }
        }
    }
    world.write_message(RequestStateTransition::Modal(GameplayModal::None));
}

/// Shared activation steps for both the instant and (once its background
/// load completes) fallback swap paths (F52.2, F52.3): swaps root
/// visibility, teleports the player, repoints `ActiveCell` and
/// `PreparedSceneManifest` (letting the #51 preloader replan and
/// `apply_fog_strength`'s change-detection follow via the explicit
/// `scene::refresh_environment_for_active_cell` refresh), moves
/// `RefRegistry` registration from the source cell's placements to the
/// destination's, captures the source cell's persistent state and applies
/// the destination's saved deltas through `world::persist` (issues
/// #60/#61), switches the cell ambient loop, and queues the destination's
/// colliders for staggered construction.
fn activate_resident_cell(world: &mut World, request: SwapRequest, kind: SwapKind) {
    let SwapRequest {
        source_cell,
        destination_cell,
        translation,
        rotation_xyzw,
    } = request;

    let Some(destination_root) = world
        .resource::<ResidentCells>()
        .0
        .get(&destination_cell)
        .map(|resident| resident.root)
    else {
        warn!(
            "swap {source_cell:08x}->{destination_cell:08x}: destination is no longer resident; aborting"
        );
        return;
    };
    let destination_manifest = world
        .resource::<ResidentCells>()
        .0
        .get(&destination_cell)
        .expect("checked destination residency above")
        .manifest
        .clone();
    let source_root = world
        .resource::<ResidentCells>()
        .0
        .get(&source_cell)
        .map(|resident| resident.root);

    // Issue #61 (F61.1): snapshot the departing cell's dynamic/open/taken
    // state into `ActiveSaveState` while its entities are still live.
    super::persist::capture_cell_state(world, source_cell);
    // Issue #63: then release the departing cell's colliders — the capture
    // above holds the dynamic poses, and the revisit rebuild restores them
    // through the save layer.
    player::teardown_cell_colliders(world, source_cell);

    if let Some(source_root) = source_root {
        world.entity_mut(source_root).insert(Visibility::Hidden);
    }

    player::teleport_active_player(world, translation, rotation_xyzw);

    world.insert_resource(ActiveCell(destination_cell));
    world.insert_resource((*destination_manifest).clone());
    scene::refresh_environment_for_active_cell(world);

    swap_refs(world, source_root, destination_root);
    super::persist::apply_cell_state(world, destination_cell, destination_root);
    audio::rebuild_ambient_for_active_cell(world);

    // Issue #55: reveals the destination in bounded, nearest-arrival-first
    // chunks instead of flipping every placement visible in this one frame
    // -- must run after `persist::apply_cell_state` so it only ever touches
    // entities that function already decided should be visible (deleted/
    // disabled references stay `Hidden`, untouched). This is also what
    // makes `destination_root` visible.
    reveal::begin_chunked_reveal(
        world,
        destination_cell,
        destination_root,
        translation,
        reveal::REVEAL_BUDGET_PER_FRAME,
    );

    let root_by_reference = collect_root_by_reference(world, destination_root);
    // Collider construction keeps this swap instant: static/keyframed work
    // is drained first, and only then are gravity-enabled dynamic bodies
    // created. Player movement is parked by CellPhysicsReadiness until that
    // static barrier exists.
    player::queue_collider_build(world, destination_manifest, root_by_reference);

    let kind_label = match kind {
        SwapKind::Instant => "instant",
        SwapKind::Fallback => "fallback",
    };
    info!("swap {source_cell:08x}->{destination_cell:08x} {kind_label} activated");
    start_swap_telemetry(world, source_cell, destination_cell, kind);
}

fn collect_root_by_reference(world: &mut World, root: Entity) -> HashMap<u32, Entity> {
    let mut query = world.query::<(Entity, &interaction::PlacementRoot, &ChildOf)>();
    query
        .iter(world)
        .filter(|(_, _, child_of)| child_of.parent() == root)
        .map(|(entity, placement_root, _)| (placement_root.placement().reference_form_id, entity))
        .collect()
}

/// F52.2's `RefRegistry` seam: unregisters the source cell's placements
/// (only the active cell's refs should be console-selectable, matching how
/// `preload.rs` never registers a preloaded cell's refs in the first
/// place) and registers the destination's.
fn swap_refs(world: &mut World, source_root: Option<Entity>, destination_root: Entity) {
    let mut query = world.query::<(Entity, &interaction::PlacementRoot, &ChildOf)>();
    let mut to_unregister = Vec::new();
    let mut to_register = Vec::new();
    for (entity, placement_root, child_of) in query.iter(world) {
        let parent = child_of.parent();
        if Some(parent) == source_root {
            to_unregister.push(entity);
        } else if parent == destination_root {
            let placement = placement_root.placement();
            to_register.push((
                entity,
                placement.reference_form_id,
                placement.editor_id.clone(),
            ));
        }
    }
    let mut references = world.resource_mut::<RefRegistry>();
    for entity in to_unregister {
        references.unregister(entity);
    }
    for (entity, form_id, editor_id) in to_register {
        references.register(entity, form_id, editor_id.as_deref());
    }
}

fn start_swap_telemetry(
    world: &mut World,
    source_cell: u32,
    destination_cell: u32,
    kind: SwapKind,
) {
    world.resource_mut::<SwapTelemetry>().0 = Some(SwapMeasurement {
        source_cell,
        destination_cell,
        kind,
        frames_remaining: SWAP_TELEMETRY_FRAMES,
        max_frame_ms: 0.0,
    });
}

/// F52.5: logs exactly one line per transition once `SWAP_TELEMETRY_FRAMES`
/// have elapsed since activation (the swap frame itself, since this system
/// is ordered after both activation systems in the same frame, plus the
/// next 30).
fn measure_swap_frame_times(time: Res<Time>, mut telemetry: ResMut<SwapTelemetry>) {
    let Some(measurement) = telemetry.0.as_mut() else {
        return;
    };
    let frame_ms = time.delta().as_secs_f64() * 1000.0;
    measurement.max_frame_ms = measurement.max_frame_ms.max(frame_ms);
    measurement.frames_remaining = measurement.frames_remaining.saturating_sub(1);
    if measurement.frames_remaining == 0 {
        let kind_label = match measurement.kind {
            SwapKind::Instant => "instant",
            SwapKind::Fallback => "fallback",
        };
        info!(
            "swap {:08x}->{:08x} {kind_label} max_frame_ms={:.1}",
            measurement.source_cell, measurement.destination_cell, measurement.max_frame_ms
        );
        telemetry.0 = None;
    }
}

fn spawn_loading_overlay_ui(mut commands: Commands) {
    commands
        .spawn((
            LoadingOverlayRoot,
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                right: px(0),
                top: px(0),
                bottom: px(0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, swap_policy::OVERLAY_MAX_ALPHA)),
            ZIndex(2000),
            Visibility::Hidden,
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("Loading..."),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
            ));
        });
}

/// F59.2: entering `GameplayModal::Loading` starts the overlay's fade-in
/// instead of hard-cutting to opaque. Re-entering mid-out-fade resumes from
/// the equivalent in-fade position (`fade_out_alpha(t)` ==
/// `fade_in_alpha(duration - t)`, pinned by T59.2's symmetry test) so the
/// alpha never jumps.
fn show_loading_overlay(
    mut fade: ResMut<LoadingOverlayFade>,
    mut roots: Query<(&mut Visibility, &mut BackgroundColor), With<LoadingOverlayRoot>>,
) {
    let elapsed_seconds = match fade.0 {
        OverlayFadePhase::FadingOut { elapsed_seconds } => {
            (swap_policy::OVERLAY_FADE_SECONDS - elapsed_seconds).max(0.0)
        }
        OverlayFadePhase::Shown | OverlayFadePhase::FadingIn { .. } => {
            return;
        }
        OverlayFadePhase::Hidden => 0.0,
    };
    fade.0 = OverlayFadePhase::FadingIn { elapsed_seconds };
    let alpha = swap_policy::fade_in_alpha(
        elapsed_seconds,
        swap_policy::OVERLAY_FADE_SECONDS,
        swap_policy::OVERLAY_MAX_ALPHA,
    );
    for (mut visibility, mut background) in &mut roots {
        *visibility = Visibility::Inherited;
        background.0.set_alpha(alpha);
    }
}

/// F59.2: exiting `GameplayModal::Loading` starts the overlay's fade-out;
/// the root stays visible until `advance_loading_overlay_fade` finishes the
/// fade and hides it. An exit mid-in-fade resumes from the symmetric
/// out-fade position, mirroring `show_loading_overlay`.
fn hide_loading_overlay(mut fade: ResMut<LoadingOverlayFade>) {
    let elapsed_seconds = match fade.0 {
        OverlayFadePhase::FadingIn { elapsed_seconds } => {
            (swap_policy::OVERLAY_FADE_SECONDS - elapsed_seconds).max(0.0)
        }
        OverlayFadePhase::Hidden | OverlayFadePhase::FadingOut { .. } => {
            return;
        }
        OverlayFadePhase::Shown => 0.0,
    };
    fade.0 = OverlayFadePhase::FadingOut { elapsed_seconds };
}

/// F59.2: advances whichever fade is running and writes the eased alpha to
/// the overlay's background. The endpoints settle into `Shown`/`Hidden`
/// (the latter also flipping the root's `Visibility` back to `Hidden`), so
/// this is a no-op almost every frame.
fn advance_loading_overlay_fade(
    time: Res<Time>,
    mut fade: ResMut<LoadingOverlayFade>,
    mut roots: Query<(&mut Visibility, &mut BackgroundColor), With<LoadingOverlayRoot>>,
) {
    match fade.0 {
        OverlayFadePhase::Hidden | OverlayFadePhase::Shown => {}
        OverlayFadePhase::FadingIn { elapsed_seconds } => {
            let elapsed_seconds = elapsed_seconds + time.delta_secs();
            let alpha = swap_policy::fade_in_alpha(
                elapsed_seconds,
                swap_policy::OVERLAY_FADE_SECONDS,
                swap_policy::OVERLAY_MAX_ALPHA,
            );
            for (_, mut background) in &mut roots {
                background.0.set_alpha(alpha);
            }
            fade.0 = if elapsed_seconds >= swap_policy::OVERLAY_FADE_SECONDS {
                OverlayFadePhase::Shown
            } else {
                OverlayFadePhase::FadingIn { elapsed_seconds }
            };
        }
        OverlayFadePhase::FadingOut { elapsed_seconds } => {
            let elapsed_seconds = elapsed_seconds + time.delta_secs();
            let alpha = swap_policy::fade_out_alpha(
                elapsed_seconds,
                swap_policy::OVERLAY_FADE_SECONDS,
                swap_policy::OVERLAY_MAX_ALPHA,
            );
            let done = elapsed_seconds >= swap_policy::OVERLAY_FADE_SECONDS;
            for (mut visibility, mut background) in &mut roots {
                background.0.set_alpha(alpha);
                if done {
                    *visibility = Visibility::Hidden;
                }
            }
            fade.0 = if done {
                OverlayFadePhase::Hidden
            } else {
                OverlayFadePhase::FadingOut { elapsed_seconds }
            };
        }
    }
}

#[cfg(test)]
mod tests {
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
        ron::de::from_str(include_str!("../../../features/fixtures/scene.ron"))
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
        world.insert_resource(manifest);
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
        app.insert_resource(fixture_manifest());
        app.add_systems(Update, evaluate_door_travel_requests);
        drive_to_in_game(&mut app);

        app.world_mut()
            .write_message(interaction::DoorTravelRequested {
                destination_cell_form_id: 0xB,
                translation: Vec3::ZERO,
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
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
}
