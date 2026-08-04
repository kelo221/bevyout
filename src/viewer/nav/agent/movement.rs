/// One tick of the physics-authoritative agent KCC (issue #114): sweeps
/// `mover` through the real `boxddd::World` using the same free
/// `move_mover`/step-support helpers the player capsule controller uses
/// (`player/movement.rs`), taking landmass's desired horizontal velocity as
/// this tick's input and integrating gravity exactly like the player when
/// airborne. Grounded/step decisions are `movement_policy::decide_grounded`
/// calls, not inline logic. Pulled out of the Bevy system so it is directly
/// testable against a real `boxddd::World` fixture (`#[cfg(test)]`, mirrors
/// `player/tests/mod.rs`'s own `move_mover` fixtures) without a Bevy `App`.
/// Returns the new world position, the new KCC velocity to remember for
/// next tick, and the new grounded state.
use super::*;

pub(crate) fn write_agent_translation(transform: &mut Transform, position: Vec3) {
    transform.translation = position;
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn step_agent_kcc(
    world: &mut boxddd::World,
    mover: &boxddd::Capsule,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
    origin: Vec3,
    kcc_velocity_in: Vec3,
    grounded_in: bool,
    desired_horizontal: Vec2,
    dt: f32,
) -> (Vec3, Vec3, bool) {
    let box_origin = player::to_box_vec3(origin);
    let initial_planes = world
        .collide_mover(box_origin, mover, collision_filter)
        .unwrap_or_default();
    // Preserve support for one solve while crossing a tread edge, exactly
    // like the player controller -- the final sweep below decides whether
    // that support still exists.
    let mut grounded = grounded_in
        || movement_policy::decide_grounded(movement_policy::GroundedObservation {
            has_walkable_plane: player::has_walkable_plane(&initial_planes),
            stepped_up: false,
        });

    let mut velocity = kcc_velocity_in;
    velocity.x = desired_horizontal.x;
    velocity.z = desired_horizontal.y;
    if grounded {
        if velocity.y < 0.0 {
            velocity.y = 0.0;
        }
    } else {
        velocity.y -= GRAVITY * dt;
    }

    let desired_delta = player::to_box_vec3(velocity * dt);
    let intentional_horizontal_motion = desired_horizontal.length_squared() > f32::EPSILON;
    let (mut position, planes, stepped_up, _attempted) = player::move_mover(
        world,
        box_origin,
        mover,
        desired_delta,
        collision_filter,
        support_filter,
        grounded && intentional_horizontal_motion,
        false,
    );
    grounded = movement_policy::decide_grounded(movement_policy::GroundedObservation {
        has_walkable_plane: player::has_walkable_plane(&planes),
        stepped_up,
    });
    if !grounded && velocity.y <= 0.0 {
        if intentional_horizontal_motion
            && let Some(supported) =
                player::try_forward_step_support(world, position, desired_delta, support_filter)
        {
            position = supported;
            grounded = true;
        } else if let Some(snapped) = player::try_step_down(
            world,
            position,
            mover,
            desired_delta,
            collision_filter,
            support_filter,
        ) {
            position = snapped;
            grounded = true;
        }
    }
    if grounded && velocity.y < 0.0 {
        velocity.y = 0.0;
    }
    (player::from_box_vec3(position), velocity, grounded)
}

/// Issue #148 diagnostic: summarises what a blocked agent's capsule is
/// actually touching, as a single stable log field. Emits every
/// non-walkable contact plane (normal + world contact point) plus the
/// fraction of this tick's desired motion the sweep could achieve, so a
/// wedge can be attributed to a real surface instead of guessed at from a
/// scene-manifest footprint scan. Footprint scans keyed on the capsule
/// *centre* miss the blocker by exactly `AGENT_RADIUS`, which is how
/// #148's metro wedge stayed misattributed across four builds.
pub(crate) fn world_contact_report(
    world: &mut boxddd::World,
    mover: &boxddd::Capsule,
    position: Vec3,
    collision_filter: boxddd::QueryFilter,
    desired_horizontal: Vec2,
) -> String {
    let origin = player::to_box_vec3(position);
    let planes = world
        .collide_mover(origin, mover, collision_filter)
        .unwrap_or_default();
    let mut blocking = planes
        .iter()
        .filter(|plane| plane.plane.normal.y < WALKABLE_CONTACT_NORMAL_Y)
        .map(|plane| {
            format!(
                "n=({:.2},{:.2},{:.2})@({:.2},{:.2},{:.2})",
                plane.plane.normal.x,
                plane.plane.normal.y,
                plane.plane.normal.z,
                plane.point.x,
                plane.point.y,
                plane.point.z,
            )
        })
        .collect::<Vec<_>>();
    blocking.sort();
    blocking.dedup();
    let step = desired_horizontal / 60.0;
    let fraction = world
        .cast_mover(
            origin,
            mover,
            boxddd::Vec3::new(step.x, 0.0, step.y),
            collision_filter,
        )
        .unwrap_or(1.0);
    // Issue #177: the reason is now stated, not left to be inferred. A
    // shortfall in *achieved* speed is what latches `collision_blocked`
    // (`movement_policy::decide_collision_outcome` compares desired against
    // achieved, and never consults contact geometry), so the flag fires just
    // as readily for an agent whose steering produced no motion as for one
    // wedged against a wall. Reporting both cases as "collision-blocked" sent
    // acceptance chasing colliders that were not there.
    let reason = if blocking.is_empty() && fraction >= 0.999 {
        "no_contact_no_progress"
    } else {
        "obstructed"
    };
    format!(
        "reason={reason} sweep_fraction={fraction:.3} blocking_planes=[{}]",
        blocking.join(" ")
    )
}

pub(crate) type AgentPhysicsQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Transform,
        &'static mut Velocity3d,
        &'static AgentDesiredVelocityBlend,
        &'static mut AgentKcc,
        Option<&'static AgentTarget3d>,
        Option<&'static AgentState>,
    ),
    (
        With<NavAgent>,
        Without<DoorTraversal>,
        // Issue #154 feature 4: a merge-portal crossing is swept by its own
        // `merge_traversal_system`, not this system.
        Without<MergeTraversal>,
    ),
>;

/// Physics-authoritative agent movement (issue #114): landmass's desired
/// velocity is this tick's *input*, not the final motion -- specifically
/// the blend `movement_policy::solve_blend_fraction` produces between
/// `AgentDesiredVelocityBlend`'s previous and latest solved values (issue
/// #114 added scope, wave 5's configurable solve interval), not
/// `AgentDesiredVelocity3d` directly; at the default interval of `1` this
/// blend is always exactly the latest value, so behaviour is unchanged from
/// a `1`-tick solve cadence. Each agent's own capsule KCC (`step_agent_kcc`,
/// mirroring `player/movement.rs`'s controller against the same shared
/// `bevy_boxddd` world) resolves collision/steps/slopes/gravity and moves
/// the `Transform`; the actual achieved velocity is what gets written back
/// to `Velocity3d` for landmass to plan against next solve -- navigation
/// proposes, physics disposes. Gated on `CellPhysicsReadiness` exactly like
/// the player (`player/movement.rs::apply_player_controls`): an agent must
/// not move through geometry that has not finished building. Door-link
/// crossings (`DoorTraversal`) are excluded; the lerp owns the transform
/// there. Feeds `movement_policy::decide_collision_outcome`/`decide_stuck`
/// per agent and emits the stable `nav agent collision-blocked <id>`/`nav
/// agent stuck <id>` lines on the rising edge, plus one forced repath
/// (re-inserting the current target) the first time an agent's route stops
/// making progress.
#[allow(clippy::too_many_arguments)]
pub(crate) fn apply_agent_physics_movement(
    time: Res<Time>,
    physics_disabled: Res<PhysicsDisabled>,
    cell_physics: Res<CellPhysicsReadiness>,
    roster: Res<DebugAgentRoster>,
    solve_counter: Res<NavSolveStepCounter>,
    solve_rate: Res<NavSolveRate>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut agents: AgentPhysicsQuery<'_, '_>,
    targets: Query<&GlobalTransform>,
    // Issue #241: the diagnostics below identify a non-roster agent by its
    // bound actor's reference FormID, so they need the actor identity
    // alongside the roster. Read-only and disjoint from `agents`.
    actor_identities: Query<&ActorRuntime>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    if physics_disabled.0 || !cell_physics.static_collision_ready() {
        for (_, _, mut velocity, _, mut kcc, _, _) in &mut agents {
            velocity.velocity = Vec3::ZERO;
            kcc.velocity = Vec3::ZERO;
            kcc.grounded = false;
        }
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
    // Same collision categories the player capsule queries against -- one
    // shared static/dynamic world, no separate agent-only layer.
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();
    // Same blend fraction for every agent this tick -- the solve gate is
    // global, not per-agent.
    let solve_blend_fraction = movement_policy::solve_blend_fraction(
        movement_policy::steps_since_solve(solve_counter.0, solve_rate.0),
        solve_rate.0,
    );
    // Issue #241: every agent gets telemetry, identified by roster index when
    // it has one and by bound-actor FormID otherwise -- the diagnostics used
    // to be gated on `roster.index_of(entity)` and therefore never fired for
    // an autonomously-bound actor, the exact population #148 is about.
    let agent_id = |entity: Entity| {
        format_agent_id(
            roster.index_of(entity),
            actor_identities
                .get(entity)
                .ok()
                .map(|actor| actor.reference_form_id),
            entity,
        )
    };

    for (entity, mut transform, mut velocity, blend, mut kcc, target, agent_state) in &mut agents {
        let desired_velocity = blend.previous.lerp(blend.latest, solve_blend_fraction);
        let desired_horizontal = Vec2::new(desired_velocity.x, desired_velocity.z);
        // Issue #188: one capsule-centre convention reaches the KCC whatever
        // the entity transform means -- zero for `tna` capsules, half a
        // capsule height for a feet-level bound actor root. See
        // `AgentKcc::capsule_centre_offset_y`.
        let centre_offset = Vec3::new(0.0, kcc.capsule_centre_offset_y, 0.0);
        let centre = transform.translation + centre_offset;
        let (new_centre, new_kcc_velocity, grounded) = step_agent_kcc(
            world,
            &mover,
            collision_filter,
            support_filter,
            centre,
            kcc.velocity,
            kcc.grounded,
            desired_horizontal,
            dt,
        );
        let new_position = new_centre - centre_offset;
        let achieved = (new_centre - centre) / dt;
        write_agent_translation(&mut transform, new_position);
        kcc.velocity = new_kcc_velocity;
        kcc.grounded = grounded;
        velocity.velocity = achieved;
        // Issue #188: hand the locomotion consumer the very pair this
        // system just computed, rather than letting it derive a second one.
        kcc.last_desired_horizontal = desired_horizontal;
        kcc.last_achieved_horizontal = Vec2::new(achieved.x, achieved.z);

        let outcome =
            movement_policy::decide_collision_outcome(movement_policy::VelocityObservation {
                desired_horizontal_speed: desired_horizontal.length(),
                achieved_horizontal_speed: Vec2::new(achieved.x, achieved.z).length(),
            });
        let was_blocked = kcc.collision_blocked;
        kcc.collision_blocked = matches!(outcome, movement_policy::CollisionOutcome::Blocked);
        if kcc.collision_blocked && !was_blocked {
            let id = agent_id(entity);
            info!("nav agent collision-blocked {id}");
            // Issue #148: "blocked" alone never said *what* blocked, which
            // sent two waves chasing the wrong collider. Report the
            // obstructing contact geometry on the rising edge: the
            // non-walkable contact normals and world points locate the
            // offending surface directly (a wall face at
            // `point +/- AGENT_RADIUS` along its normal). Rising edge only,
            // so a permanently wedged agent logs this once, not per tick.
            // `new_centre`, not `transform.translation`: the contact probe
            // must query the capsule where the capsule actually is (issue
            // #188 added the feet-level bound-actor convention).
            let contacts = world_contact_report(
                world,
                &mover,
                new_centre,
                collision_filter,
                desired_horizontal,
            );
            info!("nav agent collision-blocked {id} contacts {contacts}");
        }

        // Stuck detection needs a live target distance -- an agent with no
        // active point/entity target is never stuck (nothing to make
        // progress toward).
        let target_point = match target {
            Some(AgentTarget3d::Point(point)) => Some(*point),
            Some(AgentTarget3d::Entity(target_entity)) => targets
                .get(*target_entity)
                .ok()
                .map(GlobalTransform::translation),
            _ => None,
        };
        let Some(target_point) = target_point else {
            continue;
        };
        // Horizontal, not 3D: `new_position` is the capsule *centre*,
        // `target_point` a feet-level nav-graph point (see
        // `movement_policy::horizontal_distance`'s doc comment) -- a 3D
        // distance never closes the constant ~half-`AGENT_HEIGHT` gap
        // between them, so the agent would falsely latch `stuck` at every
        // target it has actually reached. No vertical guard here (unlike
        // the door-proximity gates): there is exactly one committed target,
        // not an array of candidates a stray vertical match could
        // misidentify, and a route with real elevation change still wants
        // horizontal progress to dominate the distance-to-target measure.
        //
        // This `distance` now feeds only the arrival short-circuit just
        // below (`arrival_resets_stuck`, issue #136 follow-up) -- literal
        // proximity to the final target is exactly what "have I arrived"
        // means. Issue #157: it no longer feeds `decide_stuck` itself, see
        // `progress_distance` below.
        let distance =
            movement_policy::horizontal_distance(new_position.to_array(), target_point.to_array());

        // Corridor-progress signal (issue #157, see `movement_policy`'s
        // module doc comment): integrate this tick's real, KCC-resolved
        // horizontal motion projected onto whatever direction landmass is
        // *currently* steering toward, then negate the running total into
        // the same monotone "smaller is better" shape `decide_stuck`
        // already expects. A route leg that must move away from the final
        // target keeps registering fresh progress every tick it is
        // actually walking the corridor, unlike literal
        // distance-to-final-target.
        kcc.route_progress += movement_policy::route_progress_delta(
            [desired_horizontal.x, desired_horizontal.y],
            [achieved.x, achieved.z],
        ) * dt;
        let progress_distance = -kcc.route_progress;
        if kcc.best_distance == f32::MAX {
            kcc.best_distance = progress_distance;
        }
        // Genuinely at the target: "reached" and "stuck" are mutually
        // exclusive outcomes, not independent facts. Without this
        // short-circuit, `decide_stuck`'s "no further improvement possible"
        // window (`STUCK_RECOVERY_TICKS + STUCK_FAILURE_TICKS`, ~2 s at
        // 64 Hz) would eventually latch `stuck` for *any* agent that simply
        // arrives and stops -- once at the closest point it can get,
        // distance stops strictly decreasing regardless of *why* (blocked,
        // or done). A `Stuck` latch from a previous route also clears here:
        // reaching a (possibly re-issued) target is unambiguous progress.
        //
        // Two independent signals feed this, not just the horizontal
        // distance threshold (issue #136 follow-up): `bevy_landmass`'s own
        // `AgentState::ReachedTarget` is authoritative ground truth for
        // "landmass itself has stopped issuing meaningful movement toward
        // this target" and must reset stuck detection even when this
        // system's own recomputed distance disagrees. Post-erosion, a raw
        // (un-sampled) `AgentTarget3d::Point` from `tna goto` can have its
        // nearest reachable point end up farther than
        // `AGENT_TARGET_REACHED_DISTANCE` from the literal requested
        // coordinate (the walkable boundary shrank inward by the agent
        // radius) even though the agent is genuinely as close as it can
        // physically get -- without also trusting landmass's own state
        // here, the no-progress window would eventually (and incorrectly)
        // latch `stuck` on a route that had already finished. See
        // `movement_policy::arrival_resets_stuck`'s doc comment.
        let landmass_reached = matches!(agent_state, Some(AgentState::ReachedTarget));
        if movement_policy::arrival_resets_stuck(
            distance,
            AGENT_TARGET_REACHED_DISTANCE,
            landmass_reached,
        ) {
            kcc.best_distance = progress_distance;
            kcc.ticks_without_progress = 0;
            kcc.recovery_active = false;
            kcc.stuck = false;
            continue;
        }
        let decision = movement_policy::decide_stuck(movement_policy::StuckObservation {
            distance_to_target: progress_distance,
            best_distance_so_far: kcc.best_distance,
            ticks_without_progress: kcc.ticks_without_progress,
            recovery_active: kcc.recovery_active,
        });
        let progressed =
            progress_distance + movement_policy::STUCK_PROGRESS_EPSILON < kcc.best_distance;
        if progressed {
            kcc.best_distance = progress_distance;
            kcc.ticks_without_progress = 0;
            kcc.recovery_active = false;
        } else {
            kcc.ticks_without_progress = kcc.ticks_without_progress.saturating_add(1);
        }
        match decision {
            movement_policy::StuckDecision::Progressing => {}
            movement_policy::StuckDecision::StartRecovery => {
                kcc.recovery_active = true;
                // Force a landmass repath by re-inserting the current
                // target -- the same technique `door_availability_system`
                // uses on a door-usability flip. `AgentTarget3d` is not
                // `Clone`; rebuild the equivalent value by matching its
                // variants.
                let rebuilt = match target {
                    Some(AgentTarget3d::Point(point)) => Some(AgentTarget3d::Point(*point)),
                    Some(AgentTarget3d::Entity(target_entity)) => {
                        Some(AgentTarget3d::Entity(*target_entity))
                    }
                    _ => None,
                };
                if let Some(rebuilt) = rebuilt {
                    commands.entity(entity).insert(rebuilt);
                }
                info!("nav agent stuck-recovery {}", agent_id(entity));
            }
            movement_policy::StuckDecision::RecoveryPending => {}
            movement_policy::StuckDecision::Stuck => {
                let was_stuck = kcc.stuck;
                kcc.stuck = true;
                if !was_stuck {
                    info!("nav agent stuck {}", agent_id(entity));
                }
            }
        }
    }
}
