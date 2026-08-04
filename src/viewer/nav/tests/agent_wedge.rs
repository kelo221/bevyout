use std::collections::HashSet;

use super::*;
use crate::console::{ConsoleError, ConsoleInvocation, ConsoleSessionId};
use crate::viewer::nav::world::links::*;
use crate::viewer::nav::world::portals::*;
use crate::vsa::{PreparedNavGraph, PreparedNavMesh, PreparedNavPolygon};
use bevy::ecs::system::SystemState;
use bevy_boxddd::boxddd::{BodyDef, BodyType, BoxHull, Filter, ShapeDef};
use bevy_landmass::prelude::*;
use bevyout_core::manifest::exterior::ExteriorBorderPortal;

use super::tests_support::*;

#[test]
#[ignore = "requires a prepared cell: set BEVYOUT_WEDGE_SCENE"]
fn wedge_replay() {
    let Ok(scene) = std::env::var("BEVYOUT_WEDGE_SCENE") else {
        return;
    };
    let scene = std::path::PathBuf::from(scene);
    let start = wedge_vec("BEVYOUT_WEDGE_START", Vec3::new(9.6, 106.0, -73.1));
    let target = wedge_vec("BEVYOUT_WEDGE_TARGET", Vec3::new(5.0, 106.0, -73.0));

    let skip = std::env::var("BEVYOUT_WEDGE_SKIP")
        .ok()
        .map(|raw| {
            raw.split(',')
                .filter(|part| !part.trim().is_empty())
                .map(|part| {
                    u32::from_str_radix(part.trim().trim_start_matches("0x"), 16)
                        .expect("hex reference form id")
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut wedge = build_wedge_world(&scene, &skip);
    {
        let mover = fixture_capsule();
        let cf = player::player_collision_filter();
        for (label, probe) in [
            ("spawn", start + Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0)),
            ("target", target + Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0)),
        ] {
            let bp = player::to_box_vec3(probe);
            let planes = wedge
                .world
                .collide_mover(bp, &mover, cf)
                .unwrap_or_default();
            let ground = wedge
                .world
                .cast_mover(bp, &mover, boxddd::Vec3::new(0.0, -1.2, 0.0), cf)
                .unwrap_or(1.0);
            println!(
                "{label} ({:.2},{:.2},{:.2}): contacts={} ground_cast={ground:.3}",
                probe.x,
                probe.y,
                probe.z,
                planes.len()
            );
            for plane in planes.iter().take(4) {
                println!(
                    "    n=({:.2},{:.2},{:.2}) <- {}",
                    plane.plane.normal.x,
                    plane.plane.normal.y,
                    plane.plane.normal.z,
                    wedge.owner(plane.shape_id)
                );
            }
        }
    }
    println!("cooked {} shapes (skipped {skip:08x?})", wedge.owners.len());

    let mover = fixture_capsule();
    let collision_filter = player::player_collision_filter();
    let support_filter = player::stair_support_filter();

    let mut position = start + Vec3::new(0.0, AGENT_HEIGHT / 2.0, 0.0);
    let mut velocity = Vec3::ZERO;
    let mut grounded = false;
    for tick in 0..600 {
        let to_target = Vec2::new(target.x - position.x, target.z - position.z);
        let desired = to_target.normalize_or_zero() * AGENT_DESIRED_SPEED;
        let (p, v, g) = step_agent_kcc(
            &mut wedge.world,
            &mover,
            collision_filter,
            support_filter,
            position,
            velocity,
            grounded,
            desired,
            1.0 / 60.0,
        );
        let moved = (p - position).length();
        if tick < 5 || tick % 60 == 0 {
            println!(
                "t{tick}: ({:.3},{:.3},{:.3}) grounded={g} moved={moved:.4}",
                p.x, p.y, p.z
            );
        }
        position = p;
        velocity = v;
        grounded = g;
    }
    println!(
        "REST ({:.3},{:.3},{:.3}) grounded={grounded}",
        position.x, position.y, position.z
    );

    // Who is touching the capsule at rest?
    let box_pos = player::to_box_vec3(position);
    let planes = wedge
        .world
        .collide_mover(box_pos, &mover, collision_filter)
        .unwrap_or_default();
    println!("contacts at rest: {}", planes.len());
    for plane in &planes {
        println!(
            "  normal=({:.3},{:.3},{:.3}) point=({:.2},{:.2},{:.2}) <- {}",
            plane.plane.normal.x,
            plane.plane.normal.y,
            plane.plane.normal.z,
            plane.point.x,
            plane.point.y,
            plane.point.z,
            wedge.owner(plane.shape_id)
        );
    }

    // What stops the forward sweep?
    let to_target = Vec2::new(target.x - position.x, target.z - position.z);
    let step = to_target.normalize_or_zero() * AGENT_DESIRED_SPEED / 60.0;
    let delta = boxddd::Vec3::new(step.x, 0.0, step.y);
    let fraction = wedge
        .world
        .cast_mover(box_pos, &mover, delta, collision_filter)
        .unwrap_or(1.0);
    println!("forward sweep fraction={fraction:.4} (1.0 = unobstructed)");
}

#[test]
#[ignore = "requires a prepared cell: set BEVYOUT_WEDGE_SCENE"]
fn stall_replay() {
    let Ok(scene) = std::env::var("BEVYOUT_WEDGE_SCENE") else {
        return;
    };
    let scene = std::path::PathBuf::from(scene);
    let start = wedge_vec("BEVYOUT_WEDGE_START", Vec3::new(9.6, 106.0, -73.1));
    let target = wedge_vec("BEVYOUT_WEDGE_TARGET", Vec3::new(5.0, 106.0, -73.0));
    let graph_path = scene.parent().unwrap().join("navmesh/navgraph.ron");

    let wedge = build_wedge_world(&scene, &[]);
    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        bevy::transform::TransformPlugin,
        NavBackendPlugin,
    ));
    app.insert_resource(PhysicsDisabled(false));
    app.insert_resource(CellPhysicsReadiness::Ready);
    app.world_mut()
        .insert_non_send(BoxdddPhysicsContext::from_world(wedge.world));

    let graph = super::super::read_nav_graph(&graph_path).expect("nav graph");
    let mesh_inputs = super::super::mesh_inputs(&graph);
    let door_type_indices = landmass_graph::door_type_indices(&mesh_inputs);
    let closed_door_type_indices =
        landmass_graph::closed_door_type_indices(&mesh_inputs, &door_type_indices);
    let mut options = archipelago_options();
    // Issue #184 kept these two overrides: sweeping them is how the stall
    // was attributed to border avoidance in the first place (the horizon
    // sets the decay rate, the neighbourhood the border set).
    if let Ok(raw) = std::env::var("BEVYOUT_OBSTACLE_HORIZON") {
        options.obstacle_avoidance_time_horizon = raw.parse().expect("numeric horizon");
    }
    if let Ok(raw) = std::env::var("BEVYOUT_NEIGHBOURHOOD") {
        options.neighbourhood = raw.parse().expect("numeric neighbourhood");
    }
    println!(
        "obstacle_avoidance_time_horizon={} neighbourhood={}",
        options.obstacle_avoidance_time_horizon, options.neighbourhood
    );
    let archipelago_entity = app.world_mut().spawn(Archipelago3d::new(options)).id();
    apply_preferred_pathing_base_cost(
        app.world_mut(),
        archipelago_entity,
        &door_type_indices,
        &closed_door_type_indices,
    );
    for mesh in &mesh_inputs {
        let Some(valid) =
            landmass_graph::build_navigation_mesh(mesh, &[], &door_type_indices, &BTreeMap::new())
                .nav_mesh
        else {
            continue;
        };
        let handle = app
            .world_mut()
            .resource_mut::<Assets<NavMesh3d>>()
            .add(NavMesh3d {
                nav_mesh: Arc::new(valid),
            });
        app.world_mut().spawn(Island3dBundle {
            island: Island,
            archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            nav_mesh: NavMeshHandle::<ThreeD>(handle),
        });
    }
    app.world_mut()
        .resource_mut::<NavArchipelagoState>()
        .archipelago = Some(archipelago_entity);

    let centre = Vec3::new(0.0, AGENT_HEIGHT * 0.5, 0.0);
    let agent = spawn_bare_agent(
        app.world_mut(),
        archipelago_entity,
        start + centre,
        target + centre,
    );
    for tick in 0..600 {
        run_one_fixed_tick(app.world_mut());
        let world = app.world();
        let position = world.get::<Transform>(agent).unwrap().translation;
        let desired = world
            .get::<AgentDesiredVelocity3d>(agent)
            .map(|value| value.velocity())
            .unwrap_or(Vec3::ZERO);
        let state = world.get::<AgentState>(agent).copied();
        let kcc = world.get::<AgentKcc>(agent).unwrap();
        let (stuck, blocked, recovery, without) = (
            kcc.stuck,
            kcc.collision_blocked,
            kcc.recovery_active,
            kcc.ticks_without_progress,
        );
        let sampled = world
            .get::<Archipelago3d>(archipelago_entity)
            .and_then(|arch| {
                arch.sample_point(position, &AGENT_POINT_SAMPLE_DISTANCE)
                    .ok()
                    .map(|p| (p.point(), p.type_index()))
            });
        if let Some((point, type_index)) = sampled
            && tick % 20 == 0
        {
            println!(
                "    sample -> ({:.3},{:.3},{:.3}) type={type_index} dy={:.3} dxz={:.3}",
                point.x,
                point.y,
                point.z,
                position.y - point.y,
                Vec2::new(point.x - position.x, point.z - position.z).length()
            );
        }
        if tick % 20 == 0 {
            println!(
                "t{tick}: pos=({:.3},{:.3},{:.3}) desired=({:.3},{:.3},{:.3}) |d|={:.3} state={state:?} stuck={stuck} blocked={blocked} rec={recovery} nprog={without}",
                position.x,
                position.y,
                position.z,
                desired.x,
                desired.y,
                desired.z,
                desired.length()
            );
        }
    }
}
