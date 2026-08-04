use super::*;
use crate::console::{ConsoleInvocation, ConsoleSessionId};
use crate::viewer::nav::NavBackendPlugin;
use bevy::ecs::system::SystemState;
use bevy_boxddd::boxddd::{BodyDef, BodyType, BoxHull, Filter, ShapeDef};

pub(crate) fn invocation(args: &[&str]) -> ConsoleInvocation {
    ConsoleInvocation {
        request_id: 1,
        frame: 1,
        session: ConsoleSessionId::new("test"),
        command: "tna".into(),
        args: args.iter().map(|arg| arg.to_string()).collect(),
        target: None,
    }
}

pub(crate) fn harness_world() -> World {
    let mut world = World::new();
    world.init_resource::<NavArchipelagoState>();
    world.init_resource::<DebugAgentRoster>();
    world.init_resource::<NavAgentLedger>();
    world.init_resource::<PendingPlayerSwapDoor>();
    world
}

// Regression for #241: autonomous binding deliberately takes no console
// roster slot, but the fall guard is gameplay behavior and must cover the
// complete agent component set.
// **Wander-no-open-doors (#198), reproduced through the nav-owned flag.**
// The door-open seam (`request_door_open`) refuses to open doors for an
// actor whose active package must not (Sandbox/Wander) purely by reading the
// nav-owned [`AgentRefusesDoors`] marker -- it no longer reaches up into an
// AI type. The AI slice SETS the marker (`set_agent_refuses_doors` with
// `!family.opens_doors()`); nav reads only its own component here. An
// unflagged agent (every `tna`-driven agent and every door-opening family)
// opens doors exactly as before.
// A permissive `boxddd` collision filter/shape pairing scoped to these
// tests: `step_agent_kcc` takes its filters as parameters, so a fixture
// world only needs *a* consistent category/mask pair, not the real
// player categories (those are private to `player/mod.rs`).
pub(crate) fn fixture_filter() -> boxddd::QueryFilter {
    boxddd::QueryFilter::new().category_bits(1).mask_bits(1)
}

pub(crate) fn fixture_shape_def() -> ShapeDef {
    ShapeDef::builder()
        .filter(Filter {
            category_bits: 1,
            mask_bits: 1,
            group_index: 0,
        })
        .build()
}

pub(crate) fn fixture_capsule() -> boxddd::Capsule {
    boxddd::Capsule::new(
        [0.0, -(AGENT_HEIGHT * 0.5 - AGENT_RADIUS), 0.0],
        [0.0, AGENT_HEIGHT * 0.5 - AGENT_RADIUS, 0.0],
        AGENT_RADIUS,
    )
}

pub(crate) fn add_fixture_box(
    world: &mut boxddd::World,
    center: boxddd::Vec3,
    half_extents: boxddd::Vec3,
) {
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    world.create_hull_shape(
        body,
        &fixture_shape_def(),
        &BoxHull::transformed(
            half_extents.x,
            half_extents.y,
            half_extents.z,
            boxddd::Transform::new(center, boxddd::Quat::IDENTITY),
        ),
    );
}

// Issue #114: the navmesh `sample_point` Y-snap from wave 3's kinematic
// spike is gone -- physics is ground authority now. Drops the agent
// capsule from above a flat floor collider through `step_agent_kcc`
// (the same free helper `apply_agent_physics_movement` calls) with no
// landmass/App involved at all, and asserts it settles to rest on the
// floor via real `boxddd` collision.
// Issue #114 minimal-World test: desired vs. actual velocity feedback.
// A wall square in front of a grounded agent means the KCC sweep
// achieves (near-)zero horizontal displacement no matter what landmass
// desired -- `movement_policy::decide_collision_outcome` must classify
// that as `Blocked`, and the achieved velocity handed back to landmass
// is the real, near-zero one, not the desired one.
// Issue #114 minimal-World test: grounded gating on
// `CellPhysicsReadiness`, mirroring the player controller's own guard
// (`player/movement.rs::apply_player_controls`). While the destination
// cell's static collision has not finished building, the agent must
// not move through geometry that is not there yet -- velocity and
// grounded state are forced to zero/false every tick.
// Regression test (issue #114 added scope, M4 wave 5 real-data
// acceptance finding): the stuck-vs-target distance must also compare
// on the horizontal plane, exactly like the two door-proximity gates
// above. A target sitting directly below/above the agent (same X/Z,
// wildly different Y -- capsule-centre vs. feet-level, or simply a
// route target at a different storey) must never latch `stuck` purely
// from that vertical gap as long as the agent is not moving away from
// it horizontally: a 3D distance check would never close that gap and
// would falsely report `stuck` at a target the agent has, on the
// ground plane that actually matters for navigation, already reached.
// Issue #154 feature 4: a clear merge-portal crossing sweeps the
// agent to the far portal point (not an instant teleport/lerp -- the
// KCC needs several ticks to physically cover the distance) and clears
// `MergeTraversal`/`UsingAnimationLink`/`active_link` once it arrives.
// A link can be reported reached while the capsule is still offset from the
// source portal point. The handoff must first align with that validated
// source point before sweeping to the far side; driving straight at the far
// point can cut the corner through collision even though the authored
// two-segment handoff is clear.
// A reached animation link remains attached while the KCC crossing is in
// flight. The link system runs before `merge_traversal_system`, so calling
// it again must leave the elapsed traversal untouched instead of restarting
// the short portal every fixed tick.
// Issue #154 feature 4 / issue #162: a merge-portal crossing whose far
// side is walled off must fail visibly through the existing
// stuck/blocked reporting (`kcc.stuck`/`kcc.collision_blocked`, the
// same fields `tna status` and the stable `nav agent stuck <id>`/`nav
// agent collision-blocked <id>` log lines already use) rather than
// teleporting the agent through the wall via a scripted lerp. Issue
// #162 replaced the wave-8 wholesale route clear with per-agent
// quarantine: this test now also pins that the specific link's kind
// gets quarantined, `PermittedAnimationLinks` excludes exactly that
// kind (never kind 0, the reserved door kind), the real target is
// captured (not discarded) behind a one-tick `PendingMergeRepath`
// blank, and -- unlike the old behaviour this replaces -- an in-
// progress `travel_intent` survives untouched.
// -------------------------------------------------------------
// Issue #162: resume_pending_merge_repath_system /
// clear_merge_link_quarantine.
// -------------------------------------------------------------

// The normal case: nothing retargeted the agent during the one-tick
// gap, so the real target `merge_traversal_system`'s timeout branch
// captured is restored verbatim and the marker is consumed.
// An `Entity` target (e.g. `tna goto player`) round-trips through the
// snapshot the same way a `Point` does.
// A `tna goto`/`tna travel` issued during the one-tick gap already
// retargeted the agent (`AgentTarget3d` is no longer `None`) -- the
// stale captured target must not clobber it.
// Issue #162 feature 2: `clear_merge_link_quarantine` resets both the
// tracked kind set and the live `PermittedAnimationLinks` component
// back to `All`. `goto_agent`/`request_travel` call this on every new
// target so a previous route's quarantine never leaks into a
// completely different one.
// `tna goto` clears a live quarantine (issue #162 feature 2's
// lifecycle rule): a new goto is a new routing intent, so whatever
// links a previous route quarantined no longer apply.
// `tna travel` clears a live quarantine the same way `tna goto` does.
// Issue #154 real-data acceptance correction: a candidate whose two
// portal points sit on the same connected, unobstructed floor must
// pass runtime collision-visibility validation.
// The FranklinMetro02 real-data finding this correction fixes: a
// candidate whose far portal point overhangs empty space (no floor
// underneath, only a ledge at the near side) must be rejected for
// missing ground support, not accepted and left to sweep an agent off
// the edge into the void at traversal time.
// A candidate whose straight-line crossing is physically blocked by
// intervening geometry (not merely a portal accepted on abstract
// topology alone) must be rejected as swept-blocked.
// Regression test (issue #114 added scope, M4 wave 5 real-data
// acceptance finding): `spawn_test_agent`'s visual child must sit
// exactly centred on its parent (zero local offset), never raised.
// Physics-authoritative movement's parent `Transform` is already the
// capsule *centre* -- the wave-3/4 kinematic agent's `AGENT_HEIGHT /
// 2.0` visual-lift compensated for that agent's `Transform` instead
// sitting at feet level (navmesh-Y-snapped every tick); reintroducing
// that lift on a now-already-centred parent double-counts it and
// floats the rendered capsule a full half-height above the floor even
// though the physics capsule (steps/slopes) sits correctly. Tied
// explicitly to the centre-based parent so this can't silently
// regress if someone reintroduces a feet-level assumption for either
// side of the parent/child pair.
// Issue #134 shipped amendment: wave 3's teardown used to despawn a
// live test agent along with the stale archipelago, losing it. It is
// now ledgered instead -- here with no door noted
// (`PendingPlayerSwapDoor` defaults to `None`), so the agent freezes
// in the *departing* cell at its current position rather than being
// silently dropped.
// Issue #134: a player-initiated swap through the exact door a live
// agent's active route was targeting hands it off to the destination
// cell (follow-through) instead of freezing it in the departing cell.
// Issue #134: a player swap through a door the agent's route was *not*
// targeting freezes it in the departing cell, same as an untargeted
// idle agent -- strict eligibility, no offscreen pathfinding.
// Issue #134: a cell claimed by a ledgered entry spawns exactly one
// agent on activation, at the destination door's own placed position.
// Plan #113 minimal-App test: a travel-door request routes the agent
// to the door triangle and, on arrival, drives the existing
// `DoorLinkState` lifecycle (pause -> scripted-open boundary -> wait
// -> traverse) to the `TravelReached` terminal seam.
// Regression test (issue #114 added scope, M4 wave 5 real-data
// acceptance finding): physics-authoritative movement's `Transform` is
// the capsule *centre*, not feet-level like `triangle_midpoint` (a
// nav-graph point). The wave-3/4 kinematic agent Y-snapped its
// `Transform` onto the navmesh every tick, incidentally erasing this
// gap; every other travel-arrival test in this file sets the agent's Y
// to match the door's exactly, which is why the regression this test
// targets shipped unnoticed. A ~0.9 m vertical offset (roughly
// `AGENT_HEIGHT / 2`, matching the real Vault101a 00028579 numbers from
// acceptance) must not stop the arrival gate from firing.
pub(crate) fn is_paused(world: &World, agent: Entity) -> bool {
    door_link::is_paused(
        world
            .get::<AgentRuntime>(agent)
            .map(|runtime| runtime.door_link)
            .unwrap_or_default(),
    )
}

// Minimal travel-door placement for the lifecycle tests.
pub(crate) fn door_placement(reference_form_id: u32) -> crate::vsa::PreparedPlacement {
    crate::vsa::PreparedPlacement {
        reference_form_id,
        base_form_id: 1,
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
        reference_kind: "REFR".into(),
        base_kind: "DOOR".into(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: crate::vsa::PreparedSemantic::Door(crate::vsa::PreparedDoor {
            lock_level: None,
            key_form_id: None,
            trapped: false,
            destination: None,
        }),
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: "ao-none".into(),
    }
}

// A door triangle (issue #155 feature 3) whose horizontal footprint
// contains `center` -- the shape every `MidRouteDoor` fixture below
// needs now that the crossing gate is point-in-triangle, not
// centroid-proximity. Spans 2 m either side of `center` along X and Z,
// well inside old `MID_ROUTE_DOOR_GATE_DISTANCE` scale but large
// enough that the test-fixture agent positions below (which move
// straight along X, holding Z fixed) reliably land inside it.
pub(crate) fn door_triangle_around(center: Vec3) -> [Vec3; 3] {
    [
        center + Vec3::new(-2.0, 0.0, -2.0),
        center + Vec3::new(2.0, 0.0, -2.0),
        center + Vec3::new(0.0, 0.0, 2.0),
    ]
}

// A door placement at a specific position (issue #134's restore
// tests, which spawn at a resolved door-marker position).
pub(crate) fn door_placement_at(
    reference_form_id: u32,
    translation: [f32; 3],
) -> crate::vsa::PreparedPlacement {
    crate::vsa::PreparedPlacement {
        translation,
        ..door_placement(reference_form_id)
    }
}

// Plan #113 minimal-App test: a locked travel door never scripted-opens
// (no teleporting through closed doors) and resolves to the existing
// deterministic `Failed` status via the wait bound.
// Plan #113 minimal-App test: a door state change triggers exactly one
// repath -- the blocked two-sided link spawns once when the door
// becomes usable, and repeated polls with no further change do
// nothing.
// Issue #163 (`setlock`): the narrow `set_door_lock_level` mutation
// point behaves exactly like a manifest-authored lock for
// `door_availability_system`'s change detection -- inserting a level
// records it (preserving any existing `key_form_id`), and clearing it
// (`None`) flips a previously-locked door usable and drives the exact
// same one-repath link-spawn `a_door_state_change_triggers_exactly_one_
// repath` exercises via a direct field poke, this time through the
// console-facing setter instead.
// Issue #163: a door with no `door_usable` entry (no in-cell nav
// triangles) still records the lock level -- the pure state mutation
// the issue calls out as the fallback when the flip itself isn't
// observable through `door_availability_system` (nothing tracks that
// door for availability polling in the first place).
// Plan #137 minimal-App test (real-data-corrected): a `goto` past a
// closed unlocked door mid-route drives the existing `DoorLinkState`
// lifecycle exactly once via the crossing-check trigger, then returns
// to `Idle` in the same cell. The door is *also* registered as a
// travel door (`travel_doors`) -- real FO3 data shows nearly every
// single-sided door resolves to a travel destination, and this is the
// exact case the orchestrator's real-data review found ungated: an
// agent with no `travel_intent` for this door must not be handed off
// (no ledger entry, no despawn, no `DoorTraversal` -- there is no
// off-mesh gap to lerp across since it merely crosses the triangle on
// the way to a farther point).
// Issue #177 acceptance: the containment gate can be *starved*. Real
// data (Vault 101, `VDoor01`) had an agent routed at a closed in-cell
// door halt ~2 m short of its crossing with a completely free collision
// sweep -- never entering the polygon, so never gating and never
// opening the door. A stalled agent must gate on the crossing its route
// continues through even without standing on it.
// The approach gate must not fire for a door the agent is stalled
// *beside* or *past*: only one its own route continues through.
// Regression test (issue #114 added scope, M4 wave 5 real-data
// acceptance finding): same shape as
// `travel_arrival_tolerates_the_agent_capsule_centre_sitting_above_the_feet_level_door_midpoint`,
// for the #137/#155 mid-route crossing gate -- a capsule-centre agent
// above a feet-level `MidRouteDoor::vertices` triangle must still
// trigger the crossing gate instead of silently clipping through the
// closed door.
// Plan #137 minimal-App test (real-data-corrected): a `tna travel`
// request to a door still produces the full travel lifecycle and
// handoff, even though the very same door is also a crossing-gate
// candidate (`mid_route_doors`) -- the agent's own `travel_intent`
// must exclude that door from the crossing check, or the two paths
// would fight over the same arrival.
// Plan #137 minimal-App test (real-data-corrected): a locked door
// crossed mid-route -- again also registered as a travel door, the
// real-data shape -- by an agent with no `travel_intent` for it never
// scripted-opens and resolves to the existing deterministic `Failed`
// outcome via the wait bound, instead of letting the agent clip
// through.
// Plan #137 minimal-App test: a mid-route door's usability flip reuses
// `door_availability_system` unchanged -- the same generic per-door
// tracking two-sided/travel doors already populate -- so clearing a
// lock while an agent waits on it triggers exactly one repath (a
// `request_door_open` retry) that frees the paused agent.
// Real-data verification (M4 wave 10, post-#153 merge): a `PauseAgent`
// leak on the door-link `Failed` terminal, found live on
// FranklinMetro02 (0001a273) while chasing a reported "unreachable,
// blocked" symptom near door 0007f7e3. Auditing every polygon
// correlation in `nav/mod.rs::mesh_inputs`/`landmass_graph.rs`
// (`door_type_indices`, `resolve_polygon_type_index`, `door_sides`,
// `merge_link_descriptors`) confirmed all of them key strictly by the
// *authored* `PolygonInput::index`/`DoorInput::triangle_index` value
// via `HashMap`/`.find()`, never by list position -- so `#153`'s new
// `.filter(|polygon| polygon.walkable)` (which does introduce list
// *position* gaps relative to the authored index, since filtering
// happens before `landmass_graph` ever sees the polygons) cannot
// misattribute a door's type index or lock-cost override to the wrong
// polygon. Real-data door 0007f7e3's own triangle (mesh 0005429f,
// index 438) was confirmed `walkable: true`/`contains_door: true` with
// vertex positions exactly matching the reported corridor -- the
// original "unreachable" was door 0007f7e3's genuine authored lock
// (level 25) correctly blocking the only route through it, the
// existing, tested mid-route crossing-gate behaviour (issue #137/
// #155), not an index-misalignment bug.
//
// The *real* defect: once `setlock 0007f7e3 0` unblocked the door and
// a *fresh* `tna goto` was reissued, the agent never actually moved
// again -- frozen at the door's own triangle, `tna status` reporting
// `paused` forever even though the door-link FSM itself correctly
// reached `Idle`. `PauseAgent` (inserted the moment this door-link
// cycle first paused the agent) was only ever removed on the
// `is_traversing` transition; the `Failed` terminal above left it
// attached, and `landmass` treats a `PauseAgent`-carrying entity as
// permanently `AgentState::Paused` -- it skips that agent's own path/
// movement solving every tick regardless of any later `AgentTarget3d`
// a fresh `tna goto`/`tna travel` sets. This test pins the fix:
// `PauseAgent` must be gone once the door-link cycle reaches `Failed`,
// the same as it already is on `Traversing`.
// The travel-arrival counterpart: `PauseAgent` must not survive a
// *travel* door's `Failed` terminal either -- the same code branch
// (`is_failed(new_state)`) handles both `LinkDestination::IntraCell`
// and `LinkDestination::Travel`, so the leak (and its fix) apply
// identically to both. `locked_travel_arrival_settles_at_a_stable_
// unreachable_terminal_not_an_oscillation` above already exercises this
// exact setup for the FSM-only assertions; this test adds the
// `PauseAgent` check that revealed the real-data bug.
// Plan #113 minimal-App test: never two concurrent travel requests.
// Issue #215: debug indices are independent and grow beyond the original
// four slots, while the defensive dense-allocation ceiling is enforced.
// Issue #114 feature 4: an indexed `tna goto` addresses exactly the
// named agent slot, leaving every other slot's target untouched --
// the back-compat bare form (no index) still defaults to agent 0.
pub(crate) fn minimal_manifest(cell_form_id: u32) -> PreparedSceneManifest {
    PreparedSceneManifest {
        schema_version: 17,
        prepare_revision: None,
        converter_revision: None,
        physics_schema_version: None,
        asset_root: ".".into(),
        source_plugin: "Fallout3.esm".into(),
        source_fingerprint: "content-hash".into(),
        item_catalog_path: None,
        item_catalog_revision: None,
        item_catalog_hash: None,
        recipe_catalog_path: None,
        recipe_catalog_revision: None,
        recipe_catalog_hash: None,
        actor_catalog_path: None,
        actor_catalog_revision: None,
        actor_catalog_hash: None,
        actor_animation_catalog_path: None,
        actor_animation_catalog_revision: None,
        actor_animation_catalog_hash: None,
        image_space_modifier_catalog_path: None,
        image_space_modifier_catalog_revision: None,
        image_space_modifier_catalog_hash: None,
        source_plugins: Vec::new(),
        visual_issues: Vec::new(),
        cell: crate::vsa::CellInfo {
            form_id: cell_form_id,
            editor_id: None,
            name: None,
            interior: true,
            behave_like_exterior: false,
            ambient_rgba: [0.0; 4],
            directional_rgba: [0.0; 4],
            image_space_form_id: None,
            image_space: None,
            lighting_template_form_id: None,
            lighting_template_flags: 0,
            lighting_template: None,
            raw_lighting: None,
            effective_lighting: None,
            water_form_id: None,
            water_height: None,
            grid: None,
            worldspace_form_id: None,
            day_night_profile: None,
            day_night_preview_profile: None,
        },
        placements: Vec::new(),
        lights: Vec::new(),
        diagnostics: Vec::new(),
        navmeshes: Vec::new(),
        nav_graph: None,
        cell_audio: Default::default(),
        audio_clips: Vec::new(),
        footstep_sets: Vec::new(),
        hard_landing_clips: Vec::new(),
        bake: None,
        static_point_shadows: None,
        reflection_probes: None,
        mutability_summary: Default::default(),
        leveled_lists: Default::default(),
        dialogue: None,
        exterior: None,
    }
}

// -----------------------------------------------------------------
// Issue #169: setlock issued before the archipelago exists.
// -----------------------------------------------------------------

// Writes a synthetic `navgraph.ron` (never Bethesda-derived -- see
// AGENTS.md's git caution) under a scratch cache dir and returns a
// manifest whose `nav_graph.asset_path`/`asset_root` resolve to it,
// plus one door placement -- mirrors `nav_overlay.rs`'s own test
// helper of the same shape (private to that module, so duplicated
// rather than shared, the same rationale `nav/mod.rs::read_nav_graph`'s
// doc comment gives for its own duplication). Exercises
// `ensure_archipelago`'s real file-reading path directly, rather than
// the `already_current` short-circuit every other test in this module
// relies on -- issue #169's bug is specifically in what that path does
// with `door_lock_info` before the short-circuit is even possible (the
// very first build of a session). The mesh carries no door triangles
// of its own: these tests are about `NavArchipelagoState::
// door_lock_info` surviving the rebuild, not about door-typed
// pathing (that is issue #155's own coverage).
pub(crate) fn manifest_with_nav_graph_and_door(
    cell_form_id: u32,
    door_form_id: u32,
    authored_lock_level: Option<i8>,
) -> PreparedSceneManifest {
    let graph = crate::vsa::PreparedNavGraph {
        cell_form_id,
        meshes: vec![crate::vsa::PreparedNavMesh {
            form_id: 0x10,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [4.0, 0.0, 0.0],
                [0.0, 0.0, 4.0],
                [4.0, 0.0, 4.0],
            ],
            polygons: vec![
                crate::vsa::PreparedNavPolygon {
                    index: 0,
                    vertex_indices: [0, 1, 2],
                    ..Default::default()
                },
                crate::vsa::PreparedNavPolygon {
                    index: 1,
                    vertex_indices: [1, 3, 2],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    let dir = std::env::temp_dir().join(format!(
        "bevyout-nav-agent-test-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let relative = "navgraph.ron";
    std::fs::write(
        dir.join(relative),
        ron::ser::to_string_pretty(&graph, ron::ser::PrettyConfig::default()).unwrap(),
    )
    .unwrap();
    let mut manifest = minimal_manifest(cell_form_id);
    manifest.asset_root = dir.to_string_lossy().into_owned();
    manifest.nav_graph = Some(crate::vsa::PreparedNavGraphSource {
        asset_path: relative.into(),
        ..Default::default()
    });
    manifest.placements = vec![crate::vsa::PreparedPlacement {
        semantic: crate::vsa::PreparedSemantic::Door(crate::vsa::PreparedDoor {
            lock_level: authored_lock_level,
            key_form_id: None,
            trapped: false,
            destination: None,
        }),
        ..door_placement(door_form_id)
    }];
    manifest
}

// Minimal resources `ensure_archipelago` unconditionally touches,
// beyond what `harness_world()` already provides -- `PhysicsDisabled`
// is set `true` so the merge-link collision-validation pass (this
// fixture's mesh has no merges anyway) never needs a real
// `BoxdddPhysicsContext`.
pub(crate) fn archipelago_build_world() -> World {
    let mut world = harness_world();
    world.init_resource::<Assets<NavMesh3d>>();
    world.init_resource::<NavCellFallBounds>();
    world.insert_resource(PhysicsDisabled(true));
    world
}

// Build-after-unlock (issue #169's exact repro): the door's authored
// data is locked (`Some(25)`), but a `setlock` unlock landed *before*
// `ensure_archipelago` ever ran -- `NavArchipelagoState` is
// `init_resource`d empty at plugin install, well before the first
// `tna spawn`, so `set_door_lock_level` (`setlock`'s own narrow
// mutation point) already has somewhere to write. The runtime unlock
// must win over the authored lock at build time.
// The build-after-lock counterpart: authored data is unlocked
// (`None`), but a runtime `setlock` recorded a lock before the
// archipelago ever existed.
// Regression pin: a door this session's `setlock` never touched keeps
// its authored value -- the merge in `ensure_archipelago` must not
// blanket-override every door with whatever the (empty)
// `NavArchipelagoState.door_lock_info` happens to hold, only apply an
// actual recorded runtime change.
// A lock change issued *after* the archipelago already exists still
// applies (the pre-#169 path -- `set_door_lock_level` writing directly
// into the live `NavArchipelagoState.door_lock_info`, no rebuild
// needed). Pinned here alongside the early-setlock tests so the two
// timing cases -- before and after the first build -- are both
// covered in one place.
// -----------------------------------------------------------------
// Wave 5 added scope (#114 movement fidelity): fixed-timestep movement,
// player-as-landmass-character avoidance, configurable solve interval.
// -----------------------------------------------------------------

// A `boxddd` collision filter compatible with the *real* hardcoded
// `player::player_collision_filter()`/`stair_support_filter()` queries
// `apply_agent_physics_movement` uses (those category constants are
// private to `player/mod.rs`, so this mirrors their known bit values --
// `WORLD_STATIC = 1`, `STEP_SUPPORT = 16` -- directly): a floor shape
// built with it is both an ordinary collision surface and a
// step-support surface. `mask_bits` is maximally permissive since a
// static, passive shape like a floor is only ever the *target* of a
// query, never the querying side.
pub(crate) fn fixture_floor_filter() -> Filter {
    Filter {
        category_bits: 1 | 16,
        mask_bits: u64::MAX,
        group_index: 0,
    }
}

// A flat floor box (top face at `center.y + half_extents.y`) using
// [`fixture_floor_filter`] rather than [`fixture_shape_def`]'s
// self-consistent-but-arbitrary filter, so the real
// `apply_agent_physics_movement` system (not just the pure
// `step_agent_kcc`/`move_mover` helpers, which take their filter as a
// parameter) actually collides with and stands on it.
pub(crate) fn add_player_compatible_floor(
    world: &mut boxddd::World,
    center: boxddd::Vec3,
    half_extents: boxddd::Vec3,
) {
    let shape_def = ShapeDef::builder().filter(fixture_floor_filter()).build();
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    world.create_hull_shape(
        body,
        &shape_def,
        &BoxHull::transformed(
            half_extents.x,
            half_extents.y,
            half_extents.z,
            boxddd::Transform::new(center, boxddd::Quat::IDENTITY),
        ),
    );
}

// Builds a minimal `App` with the full `NavBackendPlugin` wiring:
// `Landmass3dPlugin` (in `FixedPreUpdate`) plus this file's own
// `FixedUpdate` agent chain and the solve-rate gate on
// `LandmassSystems::Update`, exactly as `install` wires it in the real
// viewer -- plus `TransformPlugin` so `GlobalTransform` reflects
// `Transform` without needing a full render/window stack. Physics
// readiness resources (`PhysicsDisabled`, `CellPhysicsReadiness`) and a
// `BoxdddPhysicsContext` holding a flat floor spanning
// [`spawn_fixture_island`]'s 4x4 footprint (top face at `y = 0.0`,
// matching the island mesh plane exactly) are inserted directly rather
// than through `player::install`, which pulls in the full window/input/
// asset surface these tests do not need. A real floor -- not just an
// empty physics world -- matters here: without one the capsule free-
// falls under gravity every tick and drifts outside the navmesh's
// vertical sampling envelope within a couple dozen ticks, flipping the
// agent to `AgentState::AgentNotOnNavMesh` and losing its desired
// velocity entirely (confirmed the hard way while writing the
// avoidance-deflection test below).
pub(crate) fn fixed_tick_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        bevy::transform::TransformPlugin,
        NavBackendPlugin,
    ));
    app.insert_resource(PhysicsDisabled(false));
    app.insert_resource(CellPhysicsReadiness::Ready);
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(2.0, -0.1, 2.0),
        boxddd::Vec3::new(4.0, 0.1, 4.0),
    );
    app.world_mut()
        .insert_non_send(BoxdddPhysicsContext::from_world(physics_world));
    app
}

// Advances exactly one fixed tick by hand: advances `Time<Fixed>` by
// its configured timestep, publishes that as the generic `Res<Time>`
// clock the way the real fixed-main loop does
// (`bevy_time::fixed::run_fixed_main_schedule`'s own per-expend body),
// then runs `FixedPreUpdate` (landmass, the player-character sync, and
// the solve-rate bookkeeping) followed by `FixedUpdate` (this file's
// agent chain) directly by schedule label -- the same technique
// `nav_overlay.rs`'s own landmass harness test uses for
// `FixedPreUpdate` alone, extended across both schedules so a whole
// tick is deterministic with no dependency on real wall-clock elapsed
// time.
pub(crate) fn run_one_fixed_tick(world: &mut World) {
    let timestep = world.resource::<Time<Fixed>>().timestep();
    world.resource_mut::<Time<Fixed>>().advance_by(timestep);
    let generic = world.resource::<Time<Fixed>>().as_generic();
    *world.resource_mut::<Time>() = generic;
    world.run_schedule(FixedPreUpdate);
    world.run_schedule(FixedUpdate);
}

// Issue #184 regression fixture: a straight walkable corridor the agent
// walks the length of, plus a *connected* side bay chopped into many thin
// sliver triangles whose border edges sit a few metres off the corridor.
// This is the synthetic shape of the 00024512 stall -- a landing joined to
// finely re-triangulated geometry (issue #171 emits exactly these slivers)
// whose borders `landmass::avoidance` flattens into one 2D `dodgy_2d`
// obstacle set. Nothing here is keyed on a cell or a coordinate: it is the
// geometry class, not the instance.
//
// Corridor: `x in [0, 3]`, `z in [0, CORRIDOR_LENGTH]`. Side bay:
// `x in [3, 5.5]`, `z in [4, 8]`, split into `slivers` strips.
pub(crate) fn stall_fixture_mesh(slivers: usize) -> landmass_graph::MeshInput {
    pub(crate) const CORRIDOR_LENGTH: f32 = 14.0;
    pub(crate) const BAY_START: f32 = 4.0;
    pub(crate) const BAY_END: f32 = 8.0;
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut index_of: HashMap<(i64, i64, i64), u32> = HashMap::new();
    // Shared vertex grid: landmass connects polygons by identical vertex
    // *indices*, so every quad corner must resolve to one entry or the
    // whole mesh degenerates into disconnected triangles with no interior
    // edges at all. Keyed in full 3D because the switchback below folds
    // back under the corridor, putting two surfaces at one `(x, z)`.
    let mut vertex = |vertices: &mut Vec<[f32; 3]>, x: f32, y: f32, z: f32| -> u32 {
        let key = (
            (x * 1e4).round() as i64,
            (y * 1e4).round() as i64,
            (z * 1e4).round() as i64,
        );
        *index_of.entry(key).or_insert_with(|| {
            vertices.push([x, y, z]);
            (vertices.len() - 1) as u32
        })
    };

    // The z cuts every quad row shares: 1 m steps along the corridor, plus
    // one cut per sliver through the bay's span so the bay's strips share
    // real edges with the corridor rather than T-junctioning onto it.
    let strip = (BAY_END - BAY_START) / slivers as f32;
    let mut cuts: Vec<f32> = (0..=CORRIDOR_LENGTH as usize).map(|z| z as f32).collect();
    cuts.extend((0..=slivers).map(|index| BAY_START + index as f32 * strip));
    cuts.sort_by(f32::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-4);

    let mut polygons: Vec<landmass_graph::PolygonInput> = Vec::new();
    let mut quad = |vertices: &mut Vec<[f32; 3]>,
                    polygons: &mut Vec<landmass_graph::PolygonInput>,
                    (x0, y0): (f32, f32),
                    (x1, y1): (f32, f32),
                    z0: f32,
                    z1: f32| {
        let (a, b, c, d) = (
            vertex(vertices, x0, y0, z0),
            vertex(vertices, x1, y1, z0),
            vertex(vertices, x0, y0, z1),
            vertex(vertices, x1, y1, z1),
        );
        for mut indices in [[a, b, c], [b, d, c]] {
            // One consistent XZ winding across the whole mesh: the lower
            // flight runs back along -x, which flips a naively-ordered
            // quad's winding and makes landmass reject the mesh outright.
            let corner = |index: u32| {
                let v = vertices[index as usize];
                (v[0], v[2])
            };
            let (p, q, r) = (corner(indices[0]), corner(indices[1]), corner(indices[2]));
            if (q.0 - p.0) * (r.1 - p.1) - (q.1 - p.1) * (r.0 - p.0) < 0.0 {
                indices.swap(1, 2);
            }
            polygons.push(landmass_graph::PolygonInput {
                index: polygons.len() as u32,
                vertex_indices: indices,
                is_water: false,
                is_preferred_pathing: false,
            });
        }
    };

    for pair in cuts.windows(2) {
        let (z0, z1) = (pair[0], pair[1]);
        quad(&mut vertices, &mut polygons, (0.0, 0.0), (3.0, 0.0), z0, z1);
        // A switchback stair descending off the corridor's x = 3 edge and
        // folding back *underneath* it: genuinely connected walkable ground
        // (an agent could walk down it), finely re-triangulated the way
        // issue #171's sub-triangle clip emits real FO3 stairs. This is the
        // ingredient that matters -- `landmass::avoidance` explores into it
        // through that shared edge and `dodgy_2d` is strictly 2D, so the
        // lower flight's borders project straight onto the corridor
        // footprint the agent is standing on.
        if z0 >= BAY_START - 1e-4 && z1 <= BAY_END + 1e-4 {
            quad(
                &mut vertices,
                &mut polygons,
                (3.0, 0.0),
                (4.0, -0.5),
                z0,
                z1,
            );
            quad(
                &mut vertices,
                &mut polygons,
                (4.0, -0.5),
                (0.0, -1.3),
                z0,
                z1,
            );
        }
    }
    landmass_graph::MeshInput {
        form_id: 0x184,
        vertices,
        polygons,
        doors: Vec::new(),
        derived_doors: Vec::new(),
    }
}

// Builds an app around [`stall_fixture_mesh`] with an explicit border
// avoidance horizon, runs an agent the length of the corridor, and reports
// the furthest `z` it reached plus the lowest desired speed it was ever
// steered at. Everything except `obstacle_avoidance_time_horizon` matches
// the shipped `archipelago_options`.
pub(crate) fn run_stall_fixture(horizon: f32) -> (f32, f32) {
    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        bevy::transform::TransformPlugin,
        NavBackendPlugin,
    ));
    app.insert_resource(PhysicsDisabled(false));
    app.insert_resource(CellPhysicsReadiness::Ready);
    let mut physics_world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    add_player_compatible_floor(
        &mut physics_world,
        boxddd::Vec3::new(2.75, -0.1, 7.0),
        boxddd::Vec3::new(6.0, 0.1, 10.0),
    );
    app.world_mut()
        .insert_non_send(BoxdddPhysicsContext::from_world(physics_world));

    let mesh_input = stall_fixture_mesh(64);
    let valid =
        landmass_graph::build_navigation_mesh(&mesh_input, &[], &BTreeMap::new(), &BTreeMap::new())
            .nav_mesh
            .expect("stall fixture validates");
    let handle = app
        .world_mut()
        .resource_mut::<Assets<NavMesh3d>>()
        .add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
    let mut options = archipelago_options();
    options.obstacle_avoidance_time_horizon = horizon;
    let archipelago = app.world_mut().spawn(Archipelago3d::new(options)).id();
    app.world_mut().spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago),
        nav_mesh: NavMeshHandle::<ThreeD>(handle),
    });
    app.world_mut()
        .resource_mut::<NavArchipelagoState>()
        .archipelago = Some(archipelago);

    let centre = Vec3::new(0.0, AGENT_HEIGHT * 0.5, 0.0);
    let agent = spawn_bare_agent(
        app.world_mut(),
        archipelago,
        Vec3::new(1.5, 0.0, 1.0) + centre,
        Vec3::new(1.5, 0.0, 13.0) + centre,
    );
    let mut furthest = f32::MIN;
    let mut slowest = f32::MAX;
    let trace = std::env::var("BEVYOUT_STALL_TRACE").is_ok();
    for tick in 0..600 {
        run_one_fixed_tick(app.world_mut());
        let world = app.world();
        if trace && tick % 20 == 0 {
            let position = world.get::<Transform>(agent).unwrap().translation;
            let desired = world
                .get::<AgentDesiredVelocity3d>(agent)
                .map(|v| v.velocity())
                .unwrap_or(Vec3::ZERO);
            println!(
                "h={horizon} t{tick}: pos=({:.2},{:.2},{:.2}) |d|={:.3} state={:?}",
                position.x,
                position.y,
                position.z,
                desired.length(),
                world.get::<AgentState>(agent).copied()
            );
        }
        furthest = furthest.max(world.get::<Transform>(agent).unwrap().translation.z);
        // Only sample steering while the agent is still short of the
        // target: decelerating on arrival is correct, not a stall.
        if furthest < 12.0
            && let Some(desired) = world.get::<AgentDesiredVelocity3d>(agent)
        {
            slowest = slowest.min(desired.velocity().length());
        }
    }
    (furthest, slowest)
}

// Issue #184: an agent must cross a stretch of corridor that has finely
// re-triangulated walkable geometry a few metres to one side, without its
// steering collapsing. Before the fix, `landmass`'s navmesh-border ORCA
// avoidance flattened that side bay's border edges into a `dodgy_2d`
// obstacle set dense enough to drive the *desired* velocity
// asymptotically to zero -- a contactless halt, with the capsule sweep
// completely free, that `apply_agent_physics_movement` could only report
// as `reason=no_contact_no_progress`.
//
// Asserted as a pair so the fixture itself is proven to reproduce: with
// landmass's stock `0.25` horizon the agent creeps to a halt beside the
// stair and is steered at a near-zero speed, and with the shipped
// `NAV_BORDER_AVOIDANCE_TIME_HORIZON` it walks the whole corridor at its
// full desired speed.
// Issue #184: the shipped options must keep navmesh-border ORCA avoidance
// clamped to at most one fixed tick -- the property that makes the
// asymptotic `1 - dt / horizon` stall impossible -- while leaving
// agent/character avoidance (issue #114 feature 4) at landmass's own
// default. A regression here is silent: it costs no test but reopens the
// contactless-stall class.
// Spawns the same synthetic two-triangle 4x4 island fixture
// `nav_overlay.rs`'s own landmass harness test uses, wired directly
// into `NavArchipelagoState` (bypassing the manifest/
// `ensure_archipelago` plumbing these unit tests do not need). Returns
// the archipelago entity.
pub(crate) fn spawn_fixture_island(world: &mut World) -> Entity {
    let mesh_input = landmass_graph::MeshInput {
        form_id: 0x10,
        vertices: vec![
            [0.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [0.0, 0.0, 4.0],
            [4.0, 0.0, 4.0],
        ],
        polygons: vec![
            landmass_graph::PolygonInput {
                index: 0,
                vertex_indices: [0, 1, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 1,
                vertex_indices: [1, 3, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
        ],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    };
    let valid =
        landmass_graph::build_navigation_mesh(&mesh_input, &[], &BTreeMap::new(), &BTreeMap::new())
            .nav_mesh
            .expect("synthetic square validates");
    let nav_mesh_handle = world.resource_mut::<Assets<NavMesh3d>>().add(NavMesh3d {
        nav_mesh: Arc::new(valid),
    });
    // The exact options `ensure_archipelago` applies for real cells --
    // widened sampling envelope plus the clamped border-avoidance horizon
    // (see `archipelago_options`).
    let archipelago_entity = world.spawn(Archipelago3d::new(archipelago_options())).id();
    world.spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
        nav_mesh: NavMeshHandle::<ThreeD>(nav_mesh_handle),
    });
    world.resource_mut::<NavArchipelagoState>().archipelago = Some(archipelago_entity);
    archipelago_entity
}

// Spawns a bare nav agent (no console-tracked `DebugAgentRoster` slot,
// no visual mesh) directly into `archipelago_entity`, targeting `target`
// from `start`. Mirrors the component set `spawn_test_agent` builds,
// minus the roster bookkeeping and visuals these App-level movement
// tests do not need.
pub(crate) fn spawn_bare_agent(
    world: &mut World,
    archipelago_entity: Entity,
    start: Vec3,
    target: Vec3,
) -> Entity {
    let agent = world
        .spawn((
            NavAgent,
            AgentKcc::default(),
            AgentDesiredVelocityBlend::default(),
            Transform::from_translation(start),
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
        ))
        .id();
    world.entity_mut(agent).insert(AgentTarget3d::Point(target));
    agent
}

// Task 1 (fixed-timestep movement) + the solve-rate gate: the agent
// keeps advancing horizontally toward its target on every fixed tick,
// including a tick the solve is gated off on (`NavSolveRate(2)`).
// Warms up over a few ticks first so both halves of the blend
// (`AgentDesiredVelocityBlend`) hold real, nonzero solved values rather
// than the zero-initialized default.
// Task 2: a landmass character mirrors the FPS player's position and
// actual KCC velocity every fixed tick, and is present in the same
// archipelago the agent/island use (`ArchipelagoRef3d` points at it).
// The player entity is spawned through the real production path
// (`player::set_camera_mode`) rather than constructed by hand: both
// `FpsPlayer` and the rest of `KccState`'s fields are private outside
// `player`, and this wave's file-ownership boundary allows exactly one
// accessor edit to `player/mod.rs` (`KccState::velocity`, made
// `pub(crate)`), not a test-only constructor.
// Task 2 (continued): a landmass character standing directly on an
// agent's straight-line path deflects the agent's desired velocity away
// from that straight line -- RVO avoidance treating the character as a
// non-agent obstacle, driven against a real archipelago (the same
// pattern `nav_overlay.rs`'s own landmass harness test uses).
// Task 3 (solve-output interpolation, user-directed addendum): at
// interval 2, on the in-between (skip) tick, the desired velocity
// `apply_agent_physics_movement` actually applies is strictly between
// the two most recently completed solve outputs -- not equal to
// either. At interval 1, it is always exactly the latest solved value,
// regardless of whatever `previous` holds -- confirming the
// interpolation is an exact no-op at the default rate. Uses an empty
// `boxddd::World` (no static geometry) so the achieved horizontal
// velocity written back to `Velocity3d` is the *unobstructed* applied
// input exactly -- a direct, physics-real assertion on the actual
// consuming system, not just the pure `solve_blend_fraction` table.
// -------------------------------------------------------------
// Issue #155 features 1/2: door polygon typing + query-time lock
// exclusion, exercised against a real `Archipelago3d` solve (this file
// owns the live-Bevy tests -- `landmass_graph.rs` stays Bevy-engine-free,
// see its module doc comment). No physics/floor and no `FixedUpdate`
// movement chain is needed here (unlike `fixed_tick_test_app`'s other
// consumers): these tests only assert `AgentState`, which
// `Landmass3dPlugin`'s `FixedPreUpdate` systems alone produce, exactly
// mirroring `nav_overlay.rs`'s own minimal landmass-only harness test.
// -------------------------------------------------------------

// Two rooms (`Room A` west, `Room B` east), connected by two
// *independent* two-triangle corridors that share no vertex with each
// other: a "door" corridor (triangles 4/5, typed under door FormID
// `0x99` when `with_bypass` doors are wanted) along the south edges,
// and -- only when `with_bypass` is true -- a plain "bypass" corridor
// (triangles 6/7, never typed) along the north edges. `with_bypass:
// false` yields a mesh where the door corridor is the *only* route
// between the rooms (invariants 1/3); `true` adds the independent
// alternate route (invariant 2). Room A's interior point `(0.7, 0.0,
// 1.0)` and Room B's interior point `(8.7, 0.0, 1.7)` are this
// fixture's start/target throughout (Room B is offset +1 in Z from
// Room A -- see the vertex list below for why).
pub(crate) fn door_topology_mesh(with_bypass: bool) -> landmass_graph::MeshInput {
    let vertices = vec![
        [0.0, 0.0, 0.0], // 0: Room A SW
        [2.0, 0.0, 0.0], // 1: Room A SE
        [0.0, 0.0, 4.0], // 2: Room A NW
        [2.0, 0.0, 4.0], // 3: Room A NE
        // Room B is offset +1 in Z relative to Room A (z:1..5, not
        // z:0..4): using the *same* Z range as Room A would put both
        // rooms' south edges (and both north edges) on the exact same
        // Z line, making the door/bypass quads degenerate (three
        // collinear corners, zero area) instead of real triangles.
        [8.0, 0.0, 1.0],  // 4: Room B SW
        [10.0, 0.0, 1.0], // 5: Room B SE
        [8.0, 0.0, 5.0],  // 6: Room B NW
        [10.0, 0.0, 5.0], // 7: Room B NE
    ];
    let mut polygons = vec![
        // Room A (SW/NE halves).
        landmass_graph::PolygonInput {
            index: 0,
            vertex_indices: [0, 1, 2],
            is_water: false,
            is_preferred_pathing: false,
        },
        landmass_graph::PolygonInput {
            index: 1,
            vertex_indices: [1, 3, 2],
            is_water: false,
            is_preferred_pathing: false,
        },
        // Room B (SW/NE halves).
        landmass_graph::PolygonInput {
            index: 2,
            vertex_indices: [4, 5, 6],
            is_water: false,
            is_preferred_pathing: false,
        },
        landmass_graph::PolygonInput {
            index: 3,
            vertex_indices: [5, 7, 6],
            is_water: false,
            is_preferred_pathing: false,
        },
        // Door corridor: Room A's south edge (0,1) <-> Room B's south
        // edge (4,5).
        landmass_graph::PolygonInput {
            index: 4,
            vertex_indices: [0, 1, 4],
            is_water: false,
            is_preferred_pathing: false,
        },
        landmass_graph::PolygonInput {
            index: 5,
            vertex_indices: [1, 5, 4],
            is_water: false,
            is_preferred_pathing: false,
        },
    ];
    if with_bypass {
        // Bypass corridor: Room A's north edge (3,2) <-> Room B's north
        // edge (7,6), reusing those rooms' own existing corner vertices
        // (no new vertices needed) -- and, critically, sharing no
        // vertex at all with the door corridor's own (0,1,4,5), so the
        // two corridors are topologically independent routes.
        polygons.push(landmass_graph::PolygonInput {
            index: 6,
            vertex_indices: [2, 3, 6],
            is_water: false,
            is_preferred_pathing: false,
        });
        polygons.push(landmass_graph::PolygonInput {
            index: 7,
            vertex_indices: [3, 7, 6],
            is_water: false,
            is_preferred_pathing: false,
        });
    }
    landmass_graph::MeshInput {
        form_id: 0x10,
        vertices,
        polygons,
        doors: vec![
            landmass_graph::DoorInput {
                triangle_index: 4,
                door_reference_form_id: Some(0x99),
            },
            landmass_graph::DoorInput {
                triangle_index: 5,
                door_reference_form_id: Some(0x99),
            },
        ],
        derived_doors: Vec::new(),
    }
}

pub(crate) const DOOR_TOPOLOGY_ROOM_A_POINT: Vec3 = Vec3::new(0.7, 0.0, 1.0);
pub(crate) const DOOR_TOPOLOGY_ROOM_B_POINT: Vec3 = Vec3::new(8.7, 0.0, 1.7);

// Builds a minimal landmass-only App (mirrors `nav_overlay.rs`'s own
// harness test, not `fixed_tick_test_app`'s physics-laden one -- these
// tests only need `AgentState`, which `Landmass3dPlugin`'s own
// `FixedPreUpdate` systems alone produce), spawns `door_topology_mesh`
// as a single island, and spawns one agent at `DOOR_TOPOLOGY_ROOM_A_
// POINT` targeting `DOOR_TOPOLOGY_ROOM_B_POINT`. `lock_override`, if
// `Some`, is inserted on the agent *before* the first solve -- the
// "door already locked when the query is issued" shape the wave's
// acceptance script exercises, and issue #155 feature 2's actual
// contract (`apply_door_lock_overrides` is exercised separately, at
// the `NavArchipelagoState`-driven integration level, by the mid-route
// gating tests above; this harness drives the raw `bevy_landmass`
// component directly since it has no `NavArchipelagoState`/manifest to
// build one from).
pub(crate) fn door_topology_test_app(
    with_bypass: bool,
    lock_override: Option<f32>,
) -> (App, Entity) {
    let mesh = door_topology_mesh(with_bypass);
    let door_type_indices = landmass_graph::door_type_indices(std::slice::from_ref(&mesh));
    assert_eq!(
        door_type_indices.get(&0x99),
        Some(&1),
        "test setup: the door must resolve to type index 1"
    );
    let build_result =
        landmass_graph::build_navigation_mesh(&mesh, &[], &door_type_indices, &BTreeMap::new());
    let valid = build_result.nav_mesh.unwrap_or_else(|| {
        panic!(
            "door_topology_mesh always validates: {:?}",
            build_result.diagnostics
        )
    });

    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        Landmass3dPlugin::default(),
    ));
    let nav_mesh_handle = app
        .world_mut()
        .resource_mut::<Assets<NavMesh3d>>()
        .add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
    let archipelago = app
        .world_mut()
        .spawn(Archipelago3d::new(archipelago_options()))
        .id();
    app.world_mut().spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago),
        nav_mesh: NavMeshHandle::<ThreeD>(nav_mesh_handle),
    });

    let mut agent_entity = app.world_mut().spawn((
        Agent3dBundle {
            agent: default(),
            settings: AgentSettings {
                radius: AGENT_RADIUS,
                desired_speed: AGENT_DESIRED_SPEED,
                max_speed: AGENT_MAX_SPEED,
            },
            archipelago_ref: ArchipelagoRef3d::new(archipelago),
        },
        Transform::from_translation(DOOR_TOPOLOGY_ROOM_A_POINT),
        AgentTarget3d::Point(DOOR_TOPOLOGY_ROOM_B_POINT),
    ));
    if let Some(cost) = lock_override {
        let mut overrides = AgentTypeIndexCostOverrides::default();
        assert!(
            overrides.set_type_index_cost(1, cost),
            "test setup: the override cost must be > 0.0"
        );
        agent_entity.insert(overrides);
    }
    let agent = agent_entity.id();
    (app, agent)
}

// -----------------------------------------------------------------
// Issue #177: closed-blocker cost overrides compose with lock state
// -----------------------------------------------------------------

// Bare-`World` fixture for `apply_door_lock_overrides`: one blocker
// FormID with both a gate type index (priced on *usability*, i.e. lock)
// and an interior/blocking type index (priced on *open*, and on whether
// the blocker can be opened at all), so every combination can be
// asserted on one component.
pub(crate) fn closed_blocker_override_world(
    usable: bool,
    open: bool,
    openable: bool,
) -> (World, Entity) {
    let mut world = harness_world();
    let agent = world.spawn_empty().id();
    let mut state = world.resource_mut::<NavArchipelagoState>();
    state.door_usable.insert(0x99, usable);
    state.door_open.insert(0x99, open);
    state.door_type_indices.insert(0x99, 1);
    state.closed_door_type_indices.insert(0x99, 2);
    if openable {
        state.openable_blockers.insert(0x99);
    }
    (world, agent)
}

pub(crate) fn override_costs(world: &World, agent: Entity) -> Vec<(usize, f32)> {
    let mut costs: Vec<(usize, f32)> = world
        .get::<AgentTypeIndexCostOverrides>(agent)
        .expect("the agent must carry overrides")
        .iter()
        .map(|(&index, &cost)| (index, cost))
        .collect();
    costs.sort_by_key(|(index, _)| *index);
    costs
}

// An activator placement whose reference is `reference_form_id`, the
// solid gear-door class issue #186 is about. `Default` audio carries no
// sound FormIDs, so activation is silent in this harness.
pub(crate) fn activator_placement(reference_form_id: u32) -> crate::vsa::PreparedPlacement {
    let mut placement = door_placement(reference_form_id);
    placement.base_kind = "ACTI".into();
    placement.semantic = crate::vsa::PreparedSemantic::Activator;
    placement
}

// Issue #186, the *signal* test (verdict §2.1): drive an activator
// blocker through the **real interaction boundary** and assert nav's
// override lifts -- deliberately not the #177 shape that pokes
// `door_open` directly (`closed_blocker_override_world`), which is why
// this class of desync shipped. A closed, not-openable gear door
// (`VaultGearDoor`'s prepared shape: `openable = false`) is impassable;
// activating it open through `scripted_activator_toggle` -> the shared
// `InteractionState.open` signal -> `door_availability_system` clears the
// override so the route is free; activating it shut restores it.
// The population itself, in isolation (verdict §1: the #177 cost tests
// bypassed this signal, which is why the desync shipped): activating an
// activator inserts it into `InteractionState.open`; this fails if the
// open-state population is ever removed from the activator path.
// -------------------------------------------------------------
// Issue #168: preferred-path base cost, exercised against a real
// `Archipelago3d` solve (this file owns the live-Bevy tests -- see
// `landmass_graph.rs`'s own module doc comment for why it stays
// Bevy-engine-free).
// -------------------------------------------------------------

// A two-room mesh with two independent, geometrically congruent
// corridors (issue #168): south (ordinary) and north (issue #156's
// `NVTR` `PREFERRED_PATHING` flag, `is_preferred_pathing: true`) --
// each corridor is the other translated by exactly `+8` in Z, so a
// route through either is the identical length. `PREFERRED_PATH_
// START`/`PREFERRED_PATH_TARGET` sit at each room's own Z-midpoint,
// equidistant from both corridors by construction: only
// `PREFERRED_PATHING_TYPE_INDEX_COST` (never distance) can make one
// strictly cheaper than the other. Room B is offset `+0.5` in Z from
// Room A at the corridor-connection edges -- the same non-degenerate-
// triangle requirement `door_topology_mesh`'s own doc comment
// explains (three vertices at the identical Z would make a
// zero-area triangle) -- and both corridors carry the identical
// offset, preserving their congruence.
pub(crate) fn preferred_path_mesh() -> landmass_graph::MeshInput {
    landmass_graph::MeshInput {
        form_id: 0x10,
        vertices: vec![
            [0.0, 0.0, 0.0],  // 0: Room A SW
            [2.0, 0.0, 0.0],  // 1: Room A SE
            [0.0, 0.0, 8.0],  // 2: Room A NW
            [2.0, 0.0, 8.0],  // 3: Room A NE
            [8.0, 0.0, 0.5],  // 4: Room B SW
            [10.0, 0.0, 0.5], // 5: Room B SE
            [8.0, 0.0, 8.5],  // 6: Room B NW
            [10.0, 0.0, 8.5], // 7: Room B NE
        ],
        polygons: vec![
            landmass_graph::PolygonInput {
                index: 0,
                vertex_indices: [0, 1, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 1,
                vertex_indices: [1, 3, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 2,
                vertex_indices: [4, 5, 6],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 3,
                vertex_indices: [5, 7, 6],
                is_water: false,
                is_preferred_pathing: false,
            },
            // South corridor (Room A/B south edges): ordinary.
            landmass_graph::PolygonInput {
                index: 4,
                vertex_indices: [0, 1, 4],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 5,
                vertex_indices: [1, 5, 4],
                is_water: false,
                is_preferred_pathing: false,
            },
            // North corridor (Room A/B north edges): preferred pathing.
            landmass_graph::PolygonInput {
                index: 6,
                vertex_indices: [2, 3, 6],
                is_water: false,
                is_preferred_pathing: true,
            },
            landmass_graph::PolygonInput {
                index: 7,
                vertex_indices: [3, 7, 6],
                is_water: false,
                is_preferred_pathing: true,
            },
        ],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    }
}

pub(crate) const PREFERRED_PATH_START: Vec3 = Vec3::new(1.0, 0.0, 4.0);
pub(crate) const PREFERRED_PATH_TARGET: Vec3 = Vec3::new(9.0, 0.0, 4.5);

// -------------------------------------------------------------
// Issue #165: locked travel-target door respects runtime lock state.
// -------------------------------------------------------------

// Real-data root cause (found by driving `locked_travel_door_fails_
// deterministically_without_opening`'s shape through the actual
// `NavBackendPlugin` schedule instead of hand-calling `door_link_
// system`): `request_door_open`'s internal lock check already refused
// to open the door, so the door genuinely never opened -- but the
// `Failed` transition only cleared `travel_intent`, leaving
// `AgentTarget3d` still pointed at the door's own triangle. Every real
// travel door is also a `mid_route_doors` candidate (`nav/agent.rs`'s
// module doc: `single_sided_doors` populates both sets), and the
// mid-route gate's travel-intent exclusion is keyed on `travel_intent`
// alone -- once that clears, the very next tick the gate "rediscovers"
// the agent standing in the door's own triangle with a target still
// set, and restarts the whole pause -> wait -> `Failed` cycle via
// `IntraCell`, forever: `tna status` observed alternating between
// `Paused` and `Unreachable` on a real locked travel door instead of
// settling at the documented terminal. This test pins that exact
// shape (the door registered in both `travel_doors` and
// `mid_route_doors`, as every real one is) and proves the fix holds
// across many more ticks than `MAX_WAIT_TICKS`, not just the first
// `Failed` transition.
// F165.2: unlocking the door and reissuing the travel (the existing
// one-repath retry contract -- `request_travel` only refuses a
// concurrent request, and `door_link::transition`'s own table already
// restarts the lifecycle cleanly from `Failed` on a fresh
// `LinkReached`) completes the hand-off normally.
// Real-data acceptance follow-up (orchestrator, contaminated-leg-B
// measurement): a *prior* successful travel through this exact door
// leaves it physically open in `InteractionState.open` forever (a
// hand-off never closes it). A later `setlock` + reissued `tna
// travel` then reaches the travel-arrival branch with the door
// already open on the very first tick -- no fresh scripted-open
// request is ever needed, so the lock check living on the open-
// *request* path (the arrival branch's `crossing_gate` consult,
// `request_door_open`'s internal check) never runs, and without the
// `Paused`-arm fix below the agent would walk straight through into
// `Traversing` -> `TravelReached` -> a scripted hand-off through a
// locked door. A hand-off is a scripted cell transition, not a
// physical walk-through: lock state must be authoritative for it
// regardless of the door's current physical open state.
// ---------------------------------------------------------------
// Issue #172: authored-stair step capability of the agent KCC.
//
// These pin the swept-capsule step behaviour `step_agent_kcc` gets
// from the shared `player::move_mover`/`try_step_up`/`try_step_down`
// helpers against FO3-scale stair geometry built as *triangle meshes*
// (the shape authored `AuthoredHavok` statics cook to), including the
// seam between two adjacent colliders.
//
// They exist because #172 was filed as a stair-climbing defect after
// agents wedged in Vault 101 Entrance (00024512) at z ~= -80.4.
// Replaying that cell's real collision through this same
// `step_agent_kcc` entry point showed the wedge is *not* a step
// failure: the capsule is pressed against the closed `VaultGearDoor`
// activator collider, whose face sits at z = -80.0 (agent radius 0.35
// -> capsule centre stops at -80.35, the measured value). Removing
// that one collider from the replay lets the agent walk straight
// through. See the issue for the full evidence. The coverage below
// stays as the regression guard that stair traversal itself is, and
// remains, sound.
// ---------------------------------------------------------------

// Appends an axis-aligned box as triangles, wound both ways: prepared
// static collision is cooked two-sided (see `player::collision`'s
// `TriangleMesh` path), so fixtures must be too.
pub(crate) fn push_box_triangles(
    vertices: &mut Vec<boxddd::Vec3>,
    indices: &mut Vec<i32>,
    min: [f32; 3],
    max: [f32; 3],
) {
    let base = i32::try_from(vertices.len()).expect("fixture vertex count fits in i32");
    for &(x, y, z) in &[
        (min[0], min[1], min[2]),
        (max[0], min[1], min[2]),
        (max[0], min[1], max[2]),
        (min[0], min[1], max[2]),
        (min[0], max[1], min[2]),
        (max[0], max[1], min[2]),
        (max[0], max[1], max[2]),
        (min[0], max[1], max[2]),
    ] {
        vertices.push(boxddd::Vec3::new(x, y, z));
    }
    pub(crate) const FACES: [[i32; 3]; 12] = [
        [0, 1, 2],
        [0, 2, 3],
        [4, 6, 5],
        [4, 7, 6],
        [0, 4, 5],
        [0, 5, 1],
        [1, 5, 6],
        [1, 6, 2],
        [2, 6, 7],
        [2, 7, 3],
        [3, 7, 4],
        [3, 4, 0],
    ];
    for face in FACES {
        indices.extend_from_slice(&[base + face[0], base + face[1], base + face[2]]);
        indices.extend_from_slice(&[base + face[0], base + face[2], base + face[1]]);
    }
}

pub(crate) fn add_fixture_mesh(
    world: &mut boxddd::World,
    vertices: Vec<boxddd::Vec3>,
    indices: Vec<i32>,
) {
    let body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    let mesh = boxddd::MeshData::builder(vertices, indices)
        .build()
        .expect("fixture triangle mesh");
    world
        .try_create_mesh_shape(
            body,
            &fixture_shape_def(),
            mesh,
            boxddd::Vec3::new(1.0, 1.0, 1.0),
        )
        .expect("fixture triangle mesh shape");
}

pub(crate) const STAIR_STEPS: usize = 10;
pub(crate) const STAIR_RISE: f32 = 0.24;
pub(crate) const STAIR_RUN: f32 = 0.28;

// A flight of `STAIR_STEPS` FO3-scale treads ascending in +Z between
// two landings, split into **two separate TriangleMesh statics** after
// `seam_after` treads -- the two-collider seam #172 called out.
pub(crate) fn add_stair_fixture(world: &mut boxddd::World, seam_after: usize) {
    let half_width = 2.0;
    let mut lower = (Vec::new(), Vec::new());
    let mut upper = (Vec::new(), Vec::new());
    push_box_triangles(
        &mut lower.0,
        &mut lower.1,
        [-half_width, -1.0, -4.0],
        [half_width, 0.0, 0.0],
    );
    for index in 0..STAIR_STEPS {
        let z0 = index as f32 * STAIR_RUN;
        let top = (index + 1) as f32 * STAIR_RISE;
        let target = if index < seam_after {
            &mut lower
        } else {
            &mut upper
        };
        push_box_triangles(
            &mut target.0,
            &mut target.1,
            [-half_width, top - 1.0, z0],
            [half_width, top, z0 + STAIR_RUN],
        );
    }
    let top = STAIR_STEPS as f32 * STAIR_RISE;
    let z0 = STAIR_STEPS as f32 * STAIR_RUN;
    push_box_triangles(
        &mut upper.0,
        &mut upper.1,
        [-half_width, top - 1.0, z0],
        [half_width, top, z0 + 4.0],
    );
    add_fixture_mesh(world, lower.0, lower.1);
    add_fixture_mesh(world, upper.0, upper.1);
}

// Walks the agent capsule through `step_agent_kcc` for `ticks` fixed
// steps at `AGENT_DESIRED_SPEED`, returning the position trace.
pub(crate) fn walk_agent(
    world: &mut boxddd::World,
    start: Vec3,
    desired: Vec2,
    ticks: usize,
) -> Vec<Vec3> {
    let mover = fixture_capsule();
    let filter = fixture_filter();
    let mut position = start;
    let mut velocity = Vec3::ZERO;
    let mut grounded = false;
    let mut trace = Vec::with_capacity(ticks);
    for _ in 0..ticks {
        let (new_position, new_velocity, new_grounded) = step_agent_kcc(
            world,
            &mover,
            filter,
            filter,
            position,
            velocity,
            grounded,
            desired,
            1.0 / 60.0,
        );
        position = new_position;
        velocity = new_velocity;
        grounded = new_grounded;
        trace.push(position);
    }
    trace
}

// F172.1 (ascending): the swept KCC climbs authored-scale risers and
// carries the climb across the seam between two TriangleMesh statics.
// F172.1 (descending): the same flight, walked downward. Guards the
// step-down probe, and with it the #164 fall guard's premise that
// walking a stair down is never a fall.
// F172.1 (negative): step handling stays bounded. A ledge taller than
// the shared step height is not climbable, so the agent stops in
// front of it rather than being lifted onto it.
// ---------------------------------------------------------------
// Issue #148 wedge investigation harness (env-gated, no committed
// game data). Rebuilds a prepared cell's collision through the *real*
// `player::create_prepared_shape` cook, keeps a shape -> placement
// map the runtime does not keep, and replays `step_agent_kcc` so a
// wedge can be attributed to a named collider.
//
//   BEVYOUT_WEDGE_SCENE=/abs/path/scene.ron \
//   BEVYOUT_WEDGE_START=9.6,106,-73.1 \
//   BEVYOUT_WEDGE_TARGET=5,106,-73 \
//   cargo test-dev --lib wedge_replay -- --nocapture --ignored
// ---------------------------------------------------------------

pub(crate) fn wedge_vec(name: &str, fallback: Vec3) -> Vec3 {
    let Ok(raw) = std::env::var(name) else {
        return fallback;
    };
    let parts = raw
        .split(',')
        .map(|part| part.trim().parse::<f32>().expect("numeric wedge vector"))
        .collect::<Vec<_>>();
    assert_eq!(parts.len(), 3, "{name} must be x,y,z");
    Vec3::new(parts[0], parts[1], parts[2])
}

pub(crate) struct WedgeWorld {
    pub(crate) world: boxddd::World,
    pub(crate) owners: HashMap<u32, String>,
}

impl WedgeWorld {
    pub(crate) fn owner(&self, shape: boxddd::ShapeId) -> String {
        self.owners
            .get(&shape_key(shape))
            .cloned()
            .unwrap_or_else(|| format!("<unmapped shape {:?}>", shape))
    }
}

pub(crate) fn shape_key(shape: boxddd::ShapeId) -> u32 {
    // `ShapeId` is opaque; its Debug form is stable enough to key on
    // within one world, and cheaper than threading a parallel index.
    let text = format!("{shape:?}");
    let digits = text
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    digits.parse().unwrap_or(0)
}

// Cooks every enabled placement's prepared collision exactly the way
// `player::build_prepared_colliders` does (same shapes, same
// categories/masks), recording which placement each shape came from.
pub(crate) fn build_wedge_world(scene: &std::path::Path, skip: &[u32]) -> WedgeWorld {
    let text = std::fs::read_to_string(scene).expect("scene manifest");
    let manifest: crate::vsa::PreparedSceneManifest =
        ron::de::from_str(&text).expect("valid scene manifest");
    let asset_root = scene.parent().unwrap().parent().unwrap().parent().unwrap();

    let mut world = boxddd::World::new(boxddd::WorldDef::default()).expect("BoxDDD world");
    let static_body = world.create_body(BodyDef::builder().body_type(BodyType::Static).build());
    let mut owners = HashMap::new();

    for placement in &manifest.placements {
        if !placement.initially_enabled || skip.contains(&placement.reference_form_id) {
            continue;
        }
        if matches!(
            placement.semantic,
            crate::vsa::PreparedSemantic::Npc(_) | crate::vsa::PreparedSemantic::Creature(_)
        ) {
            continue;
        }
        let Some(relative) = placement.physics_asset_path.as_ref() else {
            continue;
        };
        let path = asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
        let Ok(asset) = crate::vsa::read_physics_asset(&path) else {
            continue;
        };
        let dynamic =
            placement.physics_classification == crate::vsa::PreparedPhysicsClassification::Dynamic;
        for body in &asset.bodies {
            let body_id = if dynamic {
                world.create_body(BodyDef::builder().body_type(BodyType::Dynamic).build())
            } else {
                static_body
            };
            for shape in &body.shapes {
                let created = player::create_prepared_shape(
                    &mut world,
                    body_id,
                    body,
                    shape,
                    placement,
                    player::PreparedShapeOptions {
                        dynamic,
                        local_space: false,
                        collision_group: 0,
                    },
                );
                if let Some((shape_id, _)) = created {
                    owners.insert(
                        shape_key(shape_id),
                        format!(
                            "{} ({:08x}) {:?}/{}",
                            placement.editor_id.as_deref().unwrap_or("<no editor id>"),
                            placement.reference_form_id,
                            placement.physics_classification,
                            shape.kind(),
                        ),
                    );
                }
            }
        }
    }
    WedgeWorld { world, owners }
}

// Issue #184 investigation harness: the same env-gated replay as
// `wedge_replay`, but with the *real* archipelago (this cell's prepared
// nav graph) driving steering instead of a straight line, so a stall
// can be attributed to landmass rather than the KCC.
// ---------------------------------------------------------------------------
// Issue #268: the console debug-info HUD's read-only agent projection.

// Runs the real [`HudAgentProjection`] system param against `world` exactly
// the way `diagnostics::update_debug_info_hud` now consumes it.
pub(crate) fn hud_projection_lines(world: &mut World) -> Vec<String> {
    let mut state = SystemState::<HudAgentProjection>::new(world);
    state
        .get(world)
        .expect("the projection's params always validate")
        .status_lines()
}
