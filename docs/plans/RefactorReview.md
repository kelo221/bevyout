# Review verdict

**Request changes.** The refactor is structurally headed in the right direction, but I found two blocking runtime issues:

1. A new or cancelled goal does not reliably terminate the previous door/travel lifecycle.
2. Nav-world rebuilding still destroys the working world before validating its replacement.

Both are especially problematic for Fallout-style package preemption and exterior-cell streaming.

I reviewed branch commit `82c8865`, covering the split from the former `agent.rs` into the new navigation modules. ([GitHub][1])

---

## 1. **P1 — Cancelling or replacing a goal does not cancel the old door/travel operation**

### Relevant code

* `src/viewer/nav/api.rs:88–98`
* `src/viewer/nav/agent/routing.rs:9–35`
* `src/viewer/nav/agent/components.rs:489–507`
* `src/viewer/nav/doors/runtime.rs`
* `src/viewer/nav/doors/fsm.rs`

`cancel_goal()` currently only calls `clear_agent_target()`, which inserts `AgentTarget3d::None`. Setting a new point/entity goal resets some route timing and KCC progress state, but does not clear:

* `AgentRuntime::door_link`
* `pending_traversal`
* `active_link`
* `travel_intent`
* `PauseAgent`

The door FSM also has no explicit cancellation or goal-replacement transition. A paused or traversing door operation can therefore remain active after the owning AI package has cancelled or replaced its route. ([GitHub][2])

This is not only stale bookkeeping. The door runtime continues advancing paused and traversing states, while travel-door arrival is driven from the stored `travel_intent`. It does not require that the original goal is still current. ([GitHub][3])

### Concrete failures

#### Cancelled travel can still happen

```text
Package requests TravelDoor A
→ actor reaches/approaches A
→ package is interrupted by combat or dialogue
→ cancel_goal()
→ old travel_intent remains
→ door open/traversal/handoff may still complete
```

#### A new package can inherit an old door failure

The unified observation API gives terminal door states precedence over ordinary Landmass route state. Therefore, after a door reaches `Failed`, setting a fresh route can continue reporting the old failure instead of `Routing`. AI converts that observation into package route failure and scheduled packages consume their retry budget. ([GitHub][4])

That is particularly damaging for Fallout-style schedules, where packages are routinely replaced by combat, dialogue, sleep, eat, follow, sandbox, and scripted packages.

### Required fix

Create one authoritative route transition seam:

```rust
fn replace_goal(
    world: &mut World,
    actor: Entity,
    goal: Option<NavGoal>,
) -> Result<RouteGeneration, NavError>;
```

It should own all state associated with the previous goal:

```rust
runtime.door_link = DoorLinkState::Idle;
runtime.pending_traversal = None;
runtime.active_link = None;
runtime.travel_intent = None;

commands.entity(actor).remove::<PauseAgent>();

clear_route_target(...);
reset_route_timing(...);
reset_kcc_progress(...);
```

For an actor already physically crossing a doorway, abruptly resetting movement may be unsafe. In that case:

* mark the current traversal as finishing;
* store the replacement goal;
* suppress travel handoff unless its route generation still matches;
* begin the replacement after crossing completes.

Add a generation identifier:

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
struct RouteGeneration(u64);

struct TravelIntent {
    generation: RouteGeneration,
    door: FormId,
    // ...
}
```

Every terminal observation, traversal completion, and handoff request should carry the generation that created it. Stale completions can then be ignored deterministically.

### Required regression tests

* Travel-door goal followed by cancellation near the door never opens or hands off.
* Travel-door A followed by point B clears A’s travel intent.
* Replacing a paused door route stops waiting on the old door.
* A new route after `DoorLinkState::Failed` reports `Routing`, not `Failed`.
* A new route after `TravelReached` no longer reports travel-ready.
* Completion from an earlier generation cannot complete or fail the newer package.

---

## 2. **P1 — Nav-world rebuilding is destructive rather than transactional**

### Relevant code

* `src/viewer/nav/world/build.rs:35–47`
* `src/viewer/nav/world/build.rs:117–229`
* `src/viewer/nav/world/build.rs:533–555`

`ensure_archipelago()` tears down the current archipelago before it has successfully read, validated, and built the replacement. After teardown, several operations can still fail:

* resident exterior graph loading;
* cell nav graph loading;
* player-position validation;
* island generation;
* finding any valid navigable islands.

The replacement `NavArchipelagoState` is only committed at the end. ([GitHub][5])

### Failure mode

```text
Exterior world A is active and usable
→ streamed resident grid signature changes
→ ensure_archipelago starts rebuilding B
→ A is despawned
→ one graph required for B fails validation/loading
→ function exits without committing B
```

The application is then left without the previously valid nav world. Existing actors may still carry references to the despawned archipelago until their state is separately repaired.

This is a high-risk design for Fallout 3-style exterior streaming because resident exterior composition changes continually around the player.

### Required fix

Use a staged build:

```rust
fn ensure_active_world(world: &mut World) -> Result<(), NavBuildError> {
    let input = collect_build_input(world)?;

    if current_world_matches(world, &input.key) {
        return Ok(());
    }

    let plan = build_world_plan(&input)?;
    let pending = spawn_pending_world(world, &plan)?;

    validate_pending_world(world, &pending)?;
    commit_pending_world(world, pending);

    Ok(())
}
```

The commit operation should:

1. Replace `NavArchipelagoState` with the fully built pending state.
2. Retarget or rebind live agents as required.
3. Despawn the previous world only after the new world is authoritative.
4. Clean up pending entities if any spawn or validation stage fails.

### Required regression tests

* Build world A successfully.
* Force graph loading or validation for world B to fail.
* Assert that A remains active.
* Assert that all A-owned islands, links, and the archipelago entity still exist.
* Assert that no partially spawned B entities remain.
* Assert that live agents still reference a valid archipelago.

---

## 3. **P2 — `tna bind` actors can be despawned and restored as debug capsules**

### Relevant code

* `src/viewer/nav/debug/command.rs:217–249`
* `src/viewer/nav/handoff/ledger.rs:17–29`
* `src/viewer/nav/handoff/ledger.rs:89–90`
* `src/viewer/nav/handoff/ledger.rs:167–209`
* `src/viewer/nav/doors/traversal.rs:103–144`

`tna bind` can attach navigation to an actual projected actor. That actor is then inserted into `DebugAgentRoster` and receives `DebugNavAgent`. ([GitHub][6])

During player-driven cell handoff, ledger code treats every roster entity as a disposable debug agent:

* it records the roster entry;
* despawns the entity;
* later restores it through `spawn_test_agent`;
* marks the replacement as `DebugNavAgent`.

The restored entity is therefore a test capsule rather than the original actor. The same ownership assumption appears in travel-door completion. ([GitHub][7])

This contrasts with `tna despawn`, which already distinguishes a bound actor and calls `release_bound_actor()` rather than deleting it. ([GitHub][6])

### Consequence

```text
tna bind <real NPC>
→ player changes cell or uses travel
→ real NPC is despawned
→ later restored as cyan debug capsule
```

Actor identity and actor-specific components can be lost, including animation, inventory, AI state, persistence metadata, and projection bindings.

### Required fix

Represent debug ownership explicitly:

```rust
enum DebugAgentOrigin {
    SpawnedCapsule,
    BoundActor {
        reference_form_id: u32,
    },
}

struct DebugAgentEntry {
    entity: Entity,
    origin: DebugAgentOrigin,
}
```

Only `SpawnedCapsule` entries should be serialized by the debug capsule ledger and reconstructed through `spawn_test_agent()`.

For bound actors, either:

* route handoff through the production actor-persistence system; or
* reject `tna travel` and debug-ledger handoff with a typed unsupported-operation error.

Do not silently convert a production actor into a debug agent.

---

## 4. **P2 — `NavGoal::TravelDoor` bypasses common route initialization**

### Relevant code

* `src/viewer/nav/agent/routing.rs:9–35`
* `src/viewer/nav/doors/travel.rs:35–73`
* `src/viewer/nav/agent/components.rs:416–451`

Point and entity goals pass through `route_agent_to_target()`, which resets:

* `goto_started_at`
* latency logging state;
* best-distance tracking;
* ticks without progress;
* recovery state;
* stuck state.

Travel-door goals directly insert `AgentTarget3d::Point`, apply door overrides, and store the travel intent. They do not pass through that common initialization path. ([GitHub][4])

A travel goal can therefore inherit stale recovery and stuck data from its previous route.

### Required fix

Resolve the travel-door endpoint first, then invoke the common route transition:

```rust
let resolved = resolve_travel_door(world, door)?;

begin_goal(
    world,
    actor,
    RouteGoal {
        target: resolved.approach_point,
        kind: RouteKind::TravelDoor {
            door,
            destination: resolved.destination,
        },
    },
)
```

Point, entity, and travel-door routing should have exactly one shared initialization path.

---

## 5. **P2 — The fallible API does not consistently validate actors, and AI discards errors**

### Relevant code

* `src/viewer/nav/api.rs:88–95`
* `src/viewer/nav/agent/routing.rs:9–24`
* `src/viewer/nav/doors/travel.rs:35–73`
* `src/viewer/ai/family_runtime.rs:227–235`

`set_goal()` returns a `Result`, but point and entity goals invoke effectively infallible paths and then return `Ok(())`. `route_agent_to_target()` uses `world.entity_mut(actor)`, which can panic for a stale entity rather than returning a navigation error. It also does not establish that the entity is a fully bound navigation actor. ([GitHub][2])

Travel routing has a different issue: an actor without `AgentRuntime` can still receive the point target and return success, but no `travel_intent` is armed. The request then behaves as an ordinary point route rather than a valid travel operation. ([GitHub][8])

AI currently discards the result of `api::set_goal()`, so even after validation is improved, a rejected route can leave the package behaving as though routing began. ([GitHub][9])

### Required fix

Use typed validation:

```rust
enum NavError {
    ActorUnavailable(Entity),
    ActorNotBound(Entity),
    TargetUnavailable(Entity),
    WorldUnavailable,
    GoalBusy,
    DoorUnavailable(FormId),
}
```

Before accepting a goal:

```rust
let actor_ref = world
    .get_entity(actor)
    .ok_or(NavError::ActorUnavailable(actor))?;

if !actor_ref.contains::<NavAgent>()
    || !actor_ref.contains::<AgentRuntime>()
    || !actor_ref.contains::<AgentKcc>()
{
    return Err(NavError::ActorNotBound(actor));
}
```

AI should convert route submission failure into a deterministic package observation:

```rust
match nav::api::set_goal(world, actor, goal) {
    Ok(()) => package.mark_route_started(),
    Err(error) => {
        package.mark_route_failed();
        log_nav_submission_failure(actor, &error);
    }
}
```

Also distinguish `ActorNotBound` from `WorldUnavailable`; those are different faults and require different remediation.

---

## 6. **P3 — The file split is stronger than the dependency split**

### Relevant code

* `src/viewer/nav/agent/mod.rs`
* `src/viewer/nav/plugin.rs`
* `src/viewer/nav/debug/capsule.rs`
* `src/viewer/nav/debug/roster.rs`
* `src/viewer/nav/debug/probes.rs`
* navigation architecture tests

`agent/mod.rs` is described as a narrow composition layer, but currently:

* uses `#![allow(unused_imports)]`;
* imports broad Bevy, Landmass, world, debug, handoff, door, and traversal dependencies;
* re-exports multiple capabilities;
* remains an umbrella namespace.

`plugin.rs` imports `crate::viewer::nav::agent::*`, and other newly split modules also depend on the same umbrella. ([GitHub][10])

Several intended ownership modules are currently placeholders:

* `debug/capsule.rs`
* `debug/roster.rs`
* `debug/probes.rs`

The implementation remains concentrated in the large debug command module. ([GitHub][11])

The architecture tests verify file existence and line caps, but do not yet prove that the named modules own the corresponding behavior. ([GitHub][12])

### Required fix

Move actual ownership into the intended files:

```text
debug/roster.rs
    DebugAgentRoster
    DebugAgentEntry
    DebugAgentOrigin

debug/capsule.rs
    spawn_debug_capsule
    despawn_debug_capsule
    capsule appearance and KCC setup

debug/probes.rs
    path
    probe
    status
    solver diagnostics

debug/command.rs
    command parsing and dispatch only
```

Replace glob dependencies with exact imports:

```rust
use crate::viewer::nav::{
    agent::movement::apply_agent_physics_movement,
    doors::availability::door_availability_system,
    handoff::ledger::restore_handed_off_agents,
    world::build::ensure_archipelago,
};
```

Then strengthen architecture tests to reject:

```text
use crate::viewer::nav::agent::*;
```

outside `agent/`, and verify that key owner types are declared in their intended module rather than merely re-exported.

---

# What improved

The branch does accomplish several important structural goals:

* The 6,500-line `agent.rs` has been removed.
* Navigation now has capability-oriented directories.
* `nav::api` gives AI a narrower boundary.
* The API does not expose Landmass or BoxDDD types.
* AI-to-navigation dependency guards have been added.
* Named `NavRuntimeSet` ordering makes schedule relationships easier to inspect.
* `NavAgent` and `DebugNavAgent` are clearer names than the previous test-marker arrangement.
* Tests have begun moving into capability-specific files.
* The single normal translation-writer invariant is guarded. ([GitHub][1])

The code is materially easier to navigate than before. The remaining work is mainly about making the new module boundaries authoritative at runtime, rather than only organizational.

# Recommended patch order

1. **Centralize goal replacement and cancellation**, including route generations and door/travel cleanup.
2. **Make nav-world replacement transactional.**
3. **Separate debug-capsule ownership from bound production actors.**
4. Route `TravelDoor` through the common goal initialization path.
5. Harden navigation API validation and handle submission errors in AI.
6. Replace `agent::*` imports and populate the currently empty ownership modules.
7. Add regression tests for package preemption, stale door completion, failed exterior rebuild, and bound-actor handoff.

After items 1 and 2, the branch would be much closer to mergeable. Items 3–5 should preferably remain in this branch because they concern contracts introduced or exposed by the refactor.

## Verification limitation

This was a source-level review of commit `82c8865`. I could not run `cargo fmt`, `cargo clippy`, or `cargo test` because the execution environment could not clone or download the complete repository, so compilation and runtime verification remain outstanding.

[1]: https://github.com/kelo221/bevyout/compare/master...AgentRefactor.patch "https://github.com/kelo221/bevyout/compare/master...AgentRefactor.patch"
[2]: https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/api.rs "https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/api.rs"
[3]: https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/doors/traversal.rs "https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/doors/traversal.rs"
[4]: https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/agent/routing.rs "https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/agent/routing.rs"
[5]: https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/world/build.rs "https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/world/build.rs"
[6]: https://github.com/kelo221/bevyout/raw/refs/heads/AgentRefactor/src/viewer/nav/debug/command.rs "https://github.com/kelo221/bevyout/raw/refs/heads/AgentRefactor/src/viewer/nav/debug/command.rs"
[7]: https://github.com/kelo221/bevyout/raw/refs/heads/AgentRefactor/src/viewer/nav/handoff/ledger.rs "https://github.com/kelo221/bevyout/raw/refs/heads/AgentRefactor/src/viewer/nav/handoff/ledger.rs"
[8]: https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/doors/travel.rs "https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/doors/travel.rs"
[9]: https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/ai/family_runtime.rs "https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/ai/family_runtime.rs"
[10]: https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/agent/mod.rs "https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/agent/mod.rs"
[11]: https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/debug/capsule.rs "https://raw.githubusercontent.com/kelo221/bevyout/AgentRefactor/src/viewer/nav/debug/capsule.rs"
[12]: https://github.com/kelo221/bevyout/raw/refs/heads/AgentRefactor/tests/architecture.rs "https://github.com/kelo221/bevyout/raw/refs/heads/AgentRefactor/tests/architecture.rs"
