use super::actor_binding;
use super::*;

pub(crate) fn advance_nav_solve_step_counter(mut counter: ResMut<NavSolveStepCounter>) {
    counter.0 = counter.0.wrapping_add(1);
}

/// Run condition gating `LandmassSystems::Update` -- the actual
/// pathfinding+avoidance solve -- to `NavSolveRate`'s configured interval.
pub(crate) fn nav_solve_gate(counter: Res<NavSolveStepCounter>, rate: Res<NavSolveRate>) -> bool {
    movement_policy::should_solve(counter.0, rate.0)
}

/// Shifts each agent's `AgentDesiredVelocityBlend` on a fixed tick that
/// actually solved (issue #114 added scope, wave 5): `previous <- old
/// latest`, `latest <- this tick's freshly-synced `AgentDesiredVelocity3d``.
/// Runs in `FixedPreUpdate` after `LandmassSystems::Output`, so
/// `AgentDesiredVelocity3d` already reflects this tick's solve (or, on a
/// gated-off tick, whatever value the last solve left it holding). A no-op
/// on a gated-off tick -- the blend pair is left exactly as the last real
/// solve set it, for `apply_agent_physics_movement` to keep interpolating
/// toward.
/// Shifts each agent's desired-velocity blend after a real Landmass solve.
pub(crate) fn update_agent_desired_velocity_blend(
    counter: Res<NavSolveStepCounter>,
    rate: Res<NavSolveRate>,
    mut agents: Query<(&AgentDesiredVelocity3d, &mut AgentDesiredVelocityBlend), With<NavAgent>>,
) {
    if !movement_policy::should_solve(counter.0, rate.0) {
        return;
    }
    for (desired, mut blend) in &mut agents {
        blend.previous = blend.latest;
        blend.latest = desired.velocity();
    }
}

pub(crate) fn agent_components(
    archipelago_entity: Entity,
    capsule_centre_offset_y: f32,
) -> impl Bundle + use<> {
    (
        AgentRuntime::default(),
        AgentKcc {
            capsule_centre_offset_y,
            ..default()
        },
        AgentDesiredVelocityBlend::default(),
        Agent3dBundle {
            agent: default(),
            settings: AgentSettings {
                radius: AGENT_RADIUS,
                desired_speed: AGENT_DESIRED_SPEED,
                max_speed: AGENT_MAX_SPEED,
            },
            archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
        },
        TargetReachedCondition::Distance(Some(AGENT_TARGET_REACHED_DISTANCE)),
    )
}

/// Strips everything `agent_components` (plus the marker and target) added,
/// returning the entity to whatever it was before it owned an agent. Used
/// only to release a *bound actor* -- a `tna` capsule is despawned whole
/// instead, since the capsule exists for nothing else.
pub(crate) fn remove_agent_components(entity: &mut EntityWorldMut<'_>) {
    entity.remove::<(
        NavAgent,
        AgentRuntime,
        AgentKcc,
        AgentDesiredVelocityBlend,
        Agent3dBundle,
        TargetReachedCondition,
        AgentTarget3d,
    )>();
    // Any traversal in flight belongs to the agent, not the actor.
    entity.remove::<(
        DoorTraversal,
        MergeTraversal,
        PendingMergeRepath,
        RefreshLandmassAnimationLinkInput,
        SuspendedLandmassTypeIndexCosts,
        DebugNavAgent,
    )>();
    // Nav-owned per-agent AI-facing state (door policy + facing ownership)
    // belongs to the agent too: a released actor opens doors and steers its own
    // facing again.
    entity.remove::<(AgentRefusesDoors, actor_binding::FacingAuthority)>();
}

// ---------------------------------------------------------------------
// Bound-actor release contract (nav-owned; issues: package leak + layering)
// ---------------------------------------------------------------------

/// Nav-owned teardown hooks other slices register so
/// [`actor_binding::release_bound_actor`] can dismantle their per-actor state
/// (e.g. the AI slice's running package controller and any interaction point it
/// claimed) when nav releases a bound actor via the #164 fall guard or `tna
/// despawn`. Nav owns and invokes the contract; the AI slice satisfies it -- so
/// nav never imports a concrete AI type, and a released actor can no longer be
/// ticked by an AI system on an agent it no longer has. Empty (and free) until a
/// slice registers a hook.
/// A single registered bound-actor teardown callback.
pub(crate) type BoundActorReleaseHook = Box<dyn Fn(&mut World, Entity) + Send + Sync>;

#[derive(Resource, Default)]
pub(crate) struct BoundActorReleaseHooks(Vec<BoundActorReleaseHook>);

impl BoundActorReleaseHooks {
    fn push(&mut self, hook: impl Fn(&mut World, Entity) + Send + Sync + 'static) {
        self.0.push(Box::new(hook));
    }
}

/// Registers a teardown `hook` invoked (in registration order) by
/// [`actor_binding::release_bound_actor`] for the entity being released. The
/// nav-owned inverse of an AI slice reaching into nav's release path: the AI
/// plugin calls this on build (via `world_mut()`), so releasing a bound actor
/// tears its package controller and occupancy down without nav ever naming an
/// AI type. Takes `&mut World` (not `&mut App`) so tests can register the same
/// hook against a bare world.
pub(crate) fn register_bound_actor_release_hook(
    world: &mut World,
    hook: impl Fn(&mut World, Entity) + Send + Sync + 'static,
) {
    world
        .get_resource_or_insert_with(BoundActorReleaseHooks::default)
        .push(hook);
}

/// Runs every registered release hook against `entity`. Takes the resource out
/// of the world while invoking so each hook receives `&mut World` without
/// aliasing the resource; a no-op when no slice has registered a hook.
pub(crate) fn run_bound_actor_release_hooks(world: &mut World, entity: Entity) {
    let Some(hooks) = world.remove_resource::<BoundActorReleaseHooks>() else {
        return;
    };
    for hook in &hooks.0 {
        hook(world, entity);
    }
    world.insert_resource(hooks);
}

/// The bound-actor release entry point delegated to by the #164 fall guard and
/// `tna despawn`: strips the nav agent set and runs every registered teardown
/// hook. Re-exported at `pub(crate)` so the AI slice can exercise the full
/// release contract (including its own hook) in tests.
pub(crate) fn release_bound_actor(world: &mut World, entity: Entity) {
    actor_binding::release_bound_actor(world, entity);
}

/// Nav-owned marker set on a bound actor whose active AI package must not open
/// doors (Sandbox/Wander, #198 -- OpenMW's `AiWander` never calls
/// `openDoors()`). Nav reads only this marker at the door-open seam; the AI
/// slice sets it via [`set_agent_refuses_doors`], so nav no longer imports an
/// AI type to learn a package's door policy. Absent == opens doors normally
/// (every `tna`-driven agent and every door-opening family).
#[derive(Component, Default)]
pub(crate) struct AgentRefusesDoors;

/// Sets or clears the nav-owned [`AgentRefusesDoors`] marker on `entity`. The AI
/// package adapter calls this when it starts/stops a family, passing
/// `!family.opens_doors()`; nav never learns the family type itself.
pub(crate) fn set_agent_refuses_doors(world: &mut World, entity: Entity, refuses: bool) {
    let Ok(mut entity) = world.get_entity_mut(entity) else {
        return;
    };
    if refuses {
        entity.insert(AgentRefusesDoors);
    } else {
        entity.remove::<AgentRefusesDoors>();
    }
}

/// Sets a bound actor's facing authority (rotation double-writer contract). When
/// `pose_authored` is true, the nav-derived facing writer
/// ([`actor_binding::face_bound_actors`]) yields, so the AI adapter's authored
/// idle-marker yaw is the only rotation written that frame; false hands facing
/// back to navigation. The AI adapter sets it; nav reads only its own component
/// -- so the two rotation writers can no longer race across a Stop/arrival
/// transition (pose takes precedence over a still-decaying desired velocity).
pub(crate) fn set_facing_authority(world: &mut World, entity: Entity, pose_authored: bool) {
    let Ok(mut entity) = world.get_entity_mut(entity) else {
        return;
    };
    entity.insert(if pose_authored {
        actor_binding::FacingAuthority::PoseAuthored
    } else {
        actor_binding::FacingAuthority::NavDerived
    });
}

/// Core, index-free bind (issue #215's extraction for the autonomous
/// package driver, #218): gives an already-projected actor entity a nav
/// agent -- the same component set `bind_agent` (the `tna bind` console
/// wrapper) inserts, with none of the console index/`ConsoleCommandResult`
/// layer. Callers that do not need a debug index (the autonomous driver)
/// call this directly; `bind_agent` calls it too and only adds the roster
/// bookkeeping on top, so there is exactly one bind implementation.
pub(crate) fn bind_agent_entity(world: &mut World, entity: Entity) -> Result<(), api::NavError> {
    ensure_archipelago(world)?;
    if world.get::<AgentKcc>(entity).is_some() {
        return Err(api::NavError::new(
            "already_bound",
            "that actor already owns a nav agent",
        ));
    }
    let archipelago_entity = world
        .resource::<NavArchipelagoState>()
        .archipelago
        .expect("ensure_archipelago populated the archipelago");
    world.entity_mut(entity).insert((
        NavAgent,
        actor_binding::NavBoundActor::default(),
        agent_components(
            archipelago_entity,
            actor_binding::BOUND_ACTOR_CAPSULE_OFFSET_Y,
        ),
    ));
    Ok(())
}
