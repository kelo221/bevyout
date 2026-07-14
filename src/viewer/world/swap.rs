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

/// F52.3's live-save seam: `src/save` is deliberately Bevy-free (std +
/// `anyhow`/`sha2` only, no `Resource` derive), so this thin wrapper is the
/// viewer's own resource type. Nothing inserts it yet -- no save/load flow
/// is wired into the viewer (see this issue's final report) -- so
/// `apply_save_state_to_cell` reading it via `World::get_resource` naturally
/// falls back to an empty `PersistentWorldState::default()` until one does.
#[derive(Resource, Default)]
pub(crate) struct ActiveSaveState(pub(crate) crate::save::PersistentWorldState);

pub(crate) fn install(app: &mut App) {
    app.insert_resource(PendingInstantSwap::default())
        .insert_resource(PendingFallbackSwap::default())
        .insert_resource(PendingFallbackResolution::default())
        .insert_resource(SwapTelemetry::default())
        .add_systems(Startup, spawn_loading_overlay_ui)
        .add_systems(OnEnter(GameplayModal::Loading), show_loading_overlay)
        .add_systems(OnExit(GameplayModal::Loading), hide_loading_overlay)
        .add_systems(
            Update,
            (
                evaluate_door_travel_requests.after(interaction::DoorActivationSet),
                apply_pending_instant_swap.after(evaluate_door_travel_requests),
                check_fallback_progress.after(evaluate_door_travel_requests),
                apply_fallback_resolution.after(check_fallback_progress),
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
#[allow(clippy::too_many_arguments)]
fn evaluate_door_travel_requests(
    mut commands: Commands,
    mut requests: MessageReader<interaction::DoorTravelRequested>,
    active_cell: Res<ActiveCell>,
    resident_cells: Res<ResidentCells>,
    manifest: Res<PreparedSceneManifest>,
    mut pending_instant: ResMut<PendingInstantSwap>,
    mut pending_fallback: ResMut<PendingFallbackSwap>,
    mut state_requests: MessageWriter<RequestStateTransition>,
) {
    for request in requests.read() {
        if pending_instant.0.is_some() || pending_fallback.0.is_some() {
            warn!(
                "door travel to {:08x} ignored; a swap is already in progress",
                request.destination_cell_form_id
            );
            continue;
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
                state_requests.write(RequestStateTransition::Modal(GameplayModal::Loading));
            }
        }
    }
}

/// F52.2: performs the whole instant swap in this same frame.
fn apply_pending_instant_swap(world: &mut World) {
    let Some(request) = world.resource_mut::<PendingInstantSwap>().0.take() else {
        return;
    };
    activate_resident_cell(world, request, SwapKind::Instant);
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
    let Some(request) = pending_fallback.0 else {
        return;
    };
    if failed_ids.contains(&request.destination_cell) {
        debug_assert_eq!(
            swap_policy::fallback_outcome(false),
            swap_policy::FallbackOutcome::ReturnToSource
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
            swap_policy::fallback_outcome(true),
            swap_policy::FallbackOutcome::Proceed
        );
        resolution.0 = Some(FallbackResolution::Proceed(request));
        pending_fallback.0 = None;
    }
}

/// F52.4: applies whatever `check_fallback_progress` resolved this frame --
/// either the same activation steps the instant path uses, or a warned
/// `ReturnToSource` that leaves the player exactly where they were. Either
/// way, closes the loading overlay by returning `GameplayModal` to `None`.
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
/// destination's, applies persistent save-state deltas to the destination's
/// placements, switches the cell ambient loop, and queues the destination's
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

    if let Some(source_root) = source_root {
        world.entity_mut(source_root).insert(Visibility::Hidden);
    }

    player::teleport_active_player(world, translation, rotation_xyzw);

    world.insert_resource(ActiveCell(destination_cell));
    world.insert_resource((*destination_manifest).clone());
    scene::refresh_environment_for_active_cell(world);

    swap_refs(world, source_root, destination_root);
    apply_save_state_to_cell(world, destination_cell, destination_root);
    audio::rebuild_ambient_for_active_cell(world);

    // Issue #55: reveals the destination in bounded, nearest-arrival-first
    // chunks instead of flipping every placement visible in this one frame
    // -- must run after `apply_save_state_to_cell` so it only ever touches
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

/// F52.3: applies `apply_persistent_cell_state` to the destination cell's
/// already-spawned placement roots, sourcing the cell's saved delta from a
/// live `save::PersistentWorldState` resource if the viewer has one
/// inserted, or an empty default otherwise (no save/load flow is wired
/// into the viewer yet -- see this issue's final report).
fn apply_save_state_to_cell(world: &mut World, destination_cell: u32, destination_root: Entity) {
    let deltas: HashMap<u32, swap_policy::ReferenceDelta> = world
        .get_resource::<ActiveSaveState>()
        .and_then(|state| state.0.cells.get(&destination_cell))
        .map(|cell_state| {
            cell_state
                .references
                .iter()
                .map(|(form_id, delta)| {
                    (
                        *form_id,
                        swap_policy::ReferenceDelta {
                            enabled: delta.enabled,
                            deleted: delta.deleted,
                            transform: delta.transform.map(|transform| {
                                swap_policy::TransformDelta {
                                    translation: transform.translation,
                                    rotation_xyzw: transform.rotation_xyzw,
                                    scale: transform.scale,
                                }
                            }),
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    let mut query = world.query::<(Entity, &interaction::PlacementRoot, &ChildOf)>();
    let placements: Vec<(Entity, u32)> = query
        .iter(world)
        .filter(|(_, _, child_of)| child_of.parent() == destination_root)
        .map(|(entity, placement_root, _)| (entity, placement_root.placement().reference_form_id))
        .collect();
    let placement_refs: Vec<swap_policy::PlacementRef> = placements
        .iter()
        .map(|(_, form_id)| swap_policy::PlacementRef {
            reference_form_id: *form_id,
        })
        .collect();
    let applications = swap_policy::apply_persistent_cell_state(&deltas, &placement_refs);
    let entity_by_form_id: HashMap<u32, Entity> = placements
        .into_iter()
        .map(|(entity, form_id)| (form_id, entity))
        .collect();

    for application in applications {
        let Some(entity) = entity_by_form_id
            .get(&application.reference_form_id)
            .copied()
        else {
            continue;
        };
        let visibility = match application.visibility {
            swap_policy::VisibilityDecision::Visible => Visibility::Inherited,
            swap_policy::VisibilityDecision::Hidden => Visibility::Hidden,
        };
        world.entity_mut(entity).insert(visibility);
        if let Some(transform) = application.transform {
            world.entity_mut(entity).insert(Transform {
                translation: Vec3::from_array(transform.translation),
                rotation: Quat::from_xyzw(
                    transform.rotation_xyzw[0],
                    transform.rotation_xyzw[1],
                    transform.rotation_xyzw[2],
                    transform.rotation_xyzw[3],
                ),
                scale: Vec3::from_array(transform.scale),
            });
        }
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
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

fn show_loading_overlay(mut roots: Query<&mut Visibility, With<LoadingOverlayRoot>>) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Inherited;
    }
}

fn hide_loading_overlay(mut roots: Query<&mut Visibility, With<LoadingOverlayRoot>>) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Hidden;
    }
}
