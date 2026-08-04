use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_boxddd::boxddd;
use bevy_boxddd::prelude::BoxdddPhysicsContext;
use bevy_landmass::UsingAnimationLink;
use bevy_landmass::prelude::*;

use crate::viewer::actor::ActorRuntime;
use crate::viewer::nav::agent::*;
use crate::viewer::nav::movement_policy;
use crate::viewer::player;
use crate::viewer::player::{CellPhysicsReadiness, PhysicsDisabled};

pub(crate) type MergeTraversalQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Transform,
        &'static mut AgentKcc,
        &'static mut MergeTraversal,
        &'static mut AgentRuntime,
        Option<&'static AgentTarget3d>,
    ),
    With<NavAgent>,
>;

/// Sweeps every agent currently crossing a same-cell merge portal (issue
/// #154 feature 4) toward the far portal point with the same
/// `step_agent_kcc` physics `apply_agent_physics_movement` uses for
/// ordinary movement, instead of `door_traversal_system`'s scripted lerp --
/// a portal candidate that turns out to be collision-blocked must fail
/// visibly (the same stable `nav agent collision-blocked <id>`/`nav agent
/// stuck <id>` lines ordinary movement already reports, feeding `tna
/// status`'s `blocked=`/`stuck=` fields) rather than teleport the agent
/// through geometry. Runs in the same `FixedUpdate` chain slot
/// `door_traversal_system` occupies for the door lifecycle
/// (`NavBackendPlugin::build`): the two systems drive entirely disjoint
/// entity sets (`MergeTraversal` vs. `DoorTraversal`), so ordering between
/// them does not matter, and `apply_agent_physics_movement` itself excludes
/// both marker components (`AgentPhysicsQuery`) so nothing ever
/// double-drives the same entity's `Transform` in one tick.
#[allow(clippy::too_many_arguments)]
pub(crate) fn merge_traversal_system(
    time: Res<Time>,
    physics_disabled: Res<PhysicsDisabled>,
    cell_physics: Res<CellPhysicsReadiness>,
    roster: Res<DebugAgentRoster>,
    archipelago_state: Res<NavArchipelagoState>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut agents: MergeTraversalQuery<'_, '_>,
    // Issue #241: same identity sources as `apply_agent_physics_movement`'s
    // telemetry, so a non-roster agent's quarantine is reported too.
    actor_identities: Query<&ActorRuntime>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 || physics_disabled.0 || !cell_physics.static_collision_ready() {
        // Physics is not ready to collide against yet (or is globally
        // disabled): hold every in-progress crossing in place rather than
        // advance it against a world that cannot resolve collision this
        // tick, mirroring `apply_agent_physics_movement`'s own guard.
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };
    let mover = boxddd::Capsule::new(
        [0.0, -(AGENT_HEIGHT * 0.5 - AGENT_RADIUS), 0.0],
        [0.0, AGENT_HEIGHT * 0.5 - AGENT_RADIUS, 0.0],
        AGENT_RADIUS,
    );
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();

    for (entity, mut transform, mut kcc, mut traversal, mut runtime, current_target) in &mut agents
    {
        if !traversal.crossing_started
            && movement_policy::nav_point_reached(
                transform.translation.to_array(),
                traversal.source.to_array(),
                MERGE_SOURCE_ALIGNMENT_DISTANCE,
                AGENT_HEIGHT,
            )
        {
            traversal.crossing_started = true;
            // Do not carry a diagonal alignment velocity into the seam. The
            // next KCC step is authored solely by the source->target leg.
            kcc.velocity.x = 0.0;
            kcc.velocity.z = 0.0;
            info!(
                "nav agent merge source aligned entity={entity:?} source=({:.2},{:.2},{:.2}) target=({:.2},{:.2},{:.2})",
                traversal.source.x,
                traversal.source.y,
                traversal.source.z,
                traversal.target.x,
                traversal.target.y,
                traversal.target.z,
            );
        }

        if traversal.crossing_started
            && traversal.elapsed > 0.0
            && movement_policy::nav_point_reached(
                transform.translation.to_array(),
                traversal.target.to_array(),
                traversal.reached_distance,
                AGENT_HEIGHT,
            )
        {
            info!(
                "nav agent merge complete entity={entity:?} position=({:.2},{:.2},{:.2}) target=({:.2},{:.2},{:.2}) elapsed={:.2}s",
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
                traversal.target.x,
                traversal.target.y,
                traversal.target.z,
                traversal.elapsed,
            );
            commands
                .entity(entity)
                .remove::<MergeTraversal>()
                .remove::<UsingAnimationLink>()
                // Landmass synchronizes this output component in
                // `FixedPreUpdate`, one phase after our FixedUpdate
                // completion. Consume the stale marker now so the link
                // driver cannot restart the same handoff in between.
                .remove::<ReachedAnimationLink3d>()
                .insert(RefreshLandmassAnimationLinkInput);
            // A portal shorter than the agent's reached-link distance can
            // leave Landmass's old corridor selecting the same link even
            // after the capsule is on its far endpoint. Clear that corridor
            // for one solve tick, then restore the real destination from the
            // far-side position so the route resumes beyond the handoff.
            if let Some(snapshot) = current_target.and_then(AgentTargetSnapshot::capture) {
                commands
                    .entity(entity)
                    .insert(AgentTarget3d::None)
                    .insert(PendingMergeRepath { target: snapshot });
            }
            runtime.active_link = None;
            continue;
        }

        traversal.elapsed += dt;
        if traversal.elapsed > traversal.timeout {
            let was_reported = kcc.stuck || kcc.collision_blocked;
            kcc.stuck = true;
            kcc.collision_blocked = true;
            commands
                .entity(entity)
                .remove::<MergeTraversal>()
                .remove::<UsingAnimationLink>()
                .remove::<ReachedAnimationLink3d>()
                .insert(RefreshLandmassAnimationLinkInput);
            runtime.active_link = None;

            // Issue #162: per-agent portal quarantine, not a wholesale
            // route clear. Excludes just this link's kind
            // (`landmass_graph::merge_link_kind`, assigned per validated
            // merge candidate at archipelago build) from this agent's own
            // `PermittedAnimationLinks` -- kind 0 (every door link) is
            // never touched, so a blocked merge seam can never lock an
            // unrelated door. The real destination is kept: `AgentTarget3d`
            // is blanked for exactly one tick (`PendingMergeRepath`,
            // `resume_pending_merge_repath_system`) to force landmass's own
            // solver to discard the now-stale corridor (which still points
            // through the blocked link) and search again with the updated
            // exclusion, instead of resuming the identical path. An absent
            // or already-`None` target has nothing worth restoring, so it
            // is left as-is.
            runtime
                .quarantined_merge_link_kinds
                .insert(traversal.link_kind);
            let permitted = permitted_animation_links_for(
                &runtime.quarantined_merge_link_kinds,
                archipelago_state.merge_link_kind_count,
            );
            commands.entity(entity).insert(permitted);
            if let Some(snapshot) = current_target.and_then(AgentTargetSnapshot::capture) {
                commands
                    .entity(entity)
                    .insert(AgentTarget3d::None)
                    .insert(PendingMergeRepath { target: snapshot });
            }
            if !was_reported {
                let id = format_agent_id(
                    roster.index_of(entity),
                    actor_identities
                        .get(entity)
                        .ok()
                        .map(|actor| actor.reference_form_id),
                    entity,
                );
                let stage = if traversal.crossing_started {
                    "crossing"
                } else {
                    "source_alignment"
                };
                let waypoint = if traversal.crossing_started {
                    traversal.target
                } else {
                    traversal.source
                };
                let to_waypoint = waypoint - transform.translation;
                let desired = Vec2::new(to_waypoint.x, to_waypoint.z).normalize_or_zero()
                    * AGENT_DESIRED_SPEED;
                let contact = world_contact_report(
                    world,
                    &mover,
                    transform.translation,
                    collision_filter,
                    desired,
                );
                warn!(
                    "nav agent portal blocked: stage={stage} timeout={:.1}s source=({:.2},{:.2},{:.2}) target=({:.2},{:.2},{:.2}) position=({:.2},{:.2},{:.2}) {contact}",
                    traversal.timeout,
                    traversal.source.x,
                    traversal.source.y,
                    traversal.source.z,
                    traversal.target.x,
                    traversal.target.y,
                    traversal.target.z,
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                );
                info!(
                    "nav agent portal quarantined {id} link={}",
                    traversal.link_kind
                );
                info!("nav agent collision-blocked {id}");
                info!("nav agent stuck {id}");
            }
            continue;
        }

        let waypoint = if traversal.crossing_started {
            traversal.target
        } else {
            traversal.source
        };
        let to_target = waypoint - transform.translation;
        let horizontal = Vec2::new(to_target.x, to_target.z);
        let desired_horizontal = if horizontal.length() > f32::EPSILON {
            horizontal.normalize() * AGENT_DESIRED_SPEED
        } else {
            Vec2::ZERO
        };
        let (new_position, new_velocity, grounded) = step_agent_kcc(
            world,
            &mover,
            collision_filter,
            support_filter,
            transform.translation,
            kcc.velocity,
            kcc.grounded,
            desired_horizontal,
            dt,
        );
        write_agent_translation(&mut transform, new_position);
        kcc.velocity = new_velocity;
        kcc.grounded = grounded;
    }
}

/// Restores the real `AgentTarget3d` one solve tick after
/// `merge_traversal_system` deliberately blanked it to `AgentTarget3d::None`
/// for either a blocked-link quarantine (issue #162) or a completed short
/// merge handoff.
///
/// Why the blank tick is needed at all: `landmass`'s own repath decision
/// (`landmass::agent::does_agent_need_repath`) only recomputes a path when
/// either the target transitions from absent to present, or the existing
/// corridor is structurally invalidated (an island/link actually
/// added/removed from the graph). Merely swapping this agent's
/// `PermittedAnimationLinks` does neither -- the just-failed portal step
/// was already behind the corridor's tracked progress, so the *existing*
/// path would simply be resumed unchanged and the agent would walk straight
/// back into the same blocked link. A very short completed link can similarly
/// stay selected because both endpoints fall inside its reached distance.
/// Blanking the target for exactly one tick forces
/// `RepathResult::ClearPathNoTarget` that tick (observed by
/// `LandmassSystems::Update` in `FixedPreUpdate`, which this system runs
/// `.after`); restoring it here lets the *next* tick's `Update` see
/// `current_path: None` plus a real target again, which is
/// `does_agent_need_repath`'s unconditional `NeedsRepath` case -- a genuine
/// fresh solve that honours the just-updated quarantine or starts from the
/// completed link's far side.
///
/// Skips the restore (but still removes the marker) when `AgentTarget3d` is
/// no longer `None`: a `tna goto`/`tna travel` issued during the one-tick
/// gap already retargeted the agent (and, via `clear_merge_link_
/// quarantine`, reset its quarantine too), so there is nothing stale left
/// to restore -- overwriting the fresh target with the stale captured one
/// would silently discard that newer command.
/// The wall-clock deadline (seconds) for a swept merge-portal crossing of
/// `initial_distance` metres (issue #154 feature 4) -- see
/// [`MERGE_TRAVERSAL_TIMEOUT_FACTOR`]'s doc comment for why this is an
/// absolute budget rather than a resettable no-progress counter.
pub(crate) fn merge_traversal_timeout(initial_distance: f32) -> f32 {
    (initial_distance.max(0.0) / AGENT_DESIRED_SPEED) * MERGE_TRAVERSAL_TIMEOUT_FACTOR
        + MERGE_TRAVERSAL_TIMEOUT_FLOOR_SECONDS
}

/// Keeps a short merge portal from satisfying the normal arrival tolerance
/// while the agent is still at its source endpoint. The tolerance remains the
/// normal `0.5 m` for ordinary links, but for a short link it is at most half
/// the distance measured when the crossing starts, guaranteeing at least one
/// real KCC movement step before completion. An exact zero-distance crossing
/// is already at its destination and keeps the ordinary tolerance.
pub(crate) fn merge_traversal_reached_distance(initial_distance: f32) -> f32 {
    if initial_distance <= f32::EPSILON {
        MERGE_TRAVERSAL_REACHED_DISTANCE
    } else {
        MERGE_TRAVERSAL_REACHED_DISTANCE.min(initial_distance * 0.5)
    }
}

/// Returns true when the capsule is already on, and has progressed past the
/// source of, the validated horizontal portal segment. Landmass can emit the
/// reached-link marker after advancing the agent between two very close
/// endpoints; sending that capsule back to `source` would reverse it into the
/// seam instead of completing the handoff.
pub(crate) fn merge_crossing_already_started(position: Vec3, source: Vec3, target: Vec3) -> bool {
    if movement_policy::horizontal_distance(position.to_array(), source.to_array())
        <= MERGE_SOURCE_ALIGNMENT_DISTANCE
    {
        return true;
    }
    let segment = Vec2::new(target.x - source.x, target.z - source.z);
    let length_squared = segment.length_squared();
    if length_squared <= f32::EPSILON {
        return true;
    }
    let relative = Vec2::new(position.x - source.x, position.z - source.z);
    let progress = relative.dot(segment) / length_squared;
    if progress <= 0.0 {
        return false;
    }
    let closest = Vec2::new(source.x, source.z) + segment * progress.clamp(0.0, 1.0);
    closest.distance(Vec2::new(position.x, position.z)) <= AGENT_RADIUS
}
