use super::*;

fn floor(x0: f32, x1: f32, z0: f32, z1: f32, y: f32) -> Vec<CollisionTriangle> {
    vec![
        CollisionTriangle {
            vertices: [[x0, y, z0], [x1, y, z0], [x1, y, z1]],
        },
        CollisionTriangle {
            vertices: [[x0, y, z0], [x1, y, z1], [x0, y, z1]],
        },
    ]
}

/// A vertical wall quad spanning `x=[x0,x1]` at fixed `z`, from `y0` up to
/// `y1` -- a wall-like collider (normal in the XZ plane).
fn wall(x0: f32, x1: f32, z: f32, y0: f32, y1: f32) -> Vec<CollisionTriangle> {
    vec![
        CollisionTriangle {
            vertices: [[x0, y0, z], [x1, y0, z], [x1, y1, z]],
        },
        CollisionTriangle {
            vertices: [[x0, y0, z], [x1, y1, z], [x0, y1, z]],
        },
    ]
}

/// A vertical wall quad at fixed `x`, spanning `z=[z0,z1]` from `y0` up
/// to `y1` -- the same shape as [`wall`], turned to face along x.
fn wall_along_z(x: f32, z0: f32, z1: f32, y0: f32, y1: f32) -> Vec<CollisionTriangle> {
    vec![
        CollisionTriangle {
            vertices: [[x, y0, z0], [x, y0, z1], [x, y1, z1]],
        },
        CollisionTriangle {
            vertices: [[x, y0, z0], [x, y1, z1], [x, y1, z0]],
        },
    ]
}

fn nav_quad(x0: f32, x1: f32, z0: f32, z1: f32, y: f32) -> NavClearanceMeshInput {
    NavClearanceMeshInput {
        vertices: vec![[x0, y, z0], [x1, y, z0], [x1, y, z1], [x0, y, z1]],
        polygons: vec![[0, 1, 2], [0, 2, 3]],
        protected_edges: Vec::new(),
    }
}

#[test]
fn empty_collision_never_removes_or_cuts() {
    let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
    let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
    assert_eq!(result.removed_unsupported, 0);
    assert_eq!(result.cut_obstructed, 0);
}

/// Whether the clipped, validated result reports `(x, z)` walkable: the
/// point-level question the sub-triangle re-triangulation exists to answer.
fn walkable_at(result: &NavClearanceResult, x: f32, z: f32) -> bool {
    result
        .polygons
        .iter()
        .zip(&result.walkable)
        .any(|(tri, &walkable)| {
            let (Some(&a), Some(&b), Some(&c)) = (
                result.vertices.get(tri[0] as usize),
                result.vertices.get(tri[1] as usize),
                result.vertices.get(tri[2] as usize),
            ) else {
                return false;
            };
            walkable && barycentric_xz(x, z, a, b, c).is_some()
        })
}

/// Whether `(x0, z0)` and `(x1, z1)` sit in the same walkable connected
/// component -- the routability question, over shared polygon edges.
fn connected(result: &NavClearanceResult, from: (f32, f32), to: (f32, f32)) -> bool {
    let mesh = NavClearanceMeshInput {
        vertices: result.vertices.clone(),
        polygons: result.polygons.clone(),
        protected_edges: Vec::new(),
    };
    let (roots, _) = label_components(&mesh, &result.walkable);
    let locate = |x: f32, z: f32| {
        result
            .polygons
            .iter()
            .enumerate()
            .find(|(index, tri)| {
                let (Some(&a), Some(&b), Some(&c)) = (
                    result.vertices.get(tri[0] as usize),
                    result.vertices.get(tri[1] as usize),
                    result.vertices.get(tri[2] as usize),
                ) else {
                    return false;
                };
                result.walkable[*index] && barycentric_xz(x, z, a, b, c).is_some()
            })
            .map(|(index, _)| roots[index])
    };
    match (locate(from.0, from.1), locate(to.0, to.1)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

#[test]
fn the_void_boundary_is_clipped_out_of_a_straddling_triangle() {
    // F171.2 / the #164 restroom-overhang class. The floor only reaches
    // x = 1.5 while the authored nav quad spans x = 0..4, so every
    // authored triangle straddles floor and void: no whole-triangle
    // verdict can express this, but the clip cuts each along the floor's
    // edge. Supported floor stays walkable, the void does not.
    let mesh = nav_quad(0.0, 4.0, 0.0, 2.0, 0.0);
    let collision = floor(0.0, 1.5, -0.5, 2.5, 0.0);
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert!(result.clipped_polygons > 0, "{result:?}");
    assert!(result.removed_unsupported > 0, "{result:?}");
    assert!(
        walkable_at(&result, 0.5, 1.0),
        "supported floor must stay walkable"
    );
    assert!(
        !walkable_at(&result, 3.0, 1.0),
        "the void must be clipped away"
    );
}

#[test]
fn a_hairline_collision_seam_does_not_read_as_a_void() {
    // Cooked static collision is assembled from independently placed
    // meshes that abut without welding, so hairline seams between floor
    // placements are void by the letter of the geometry. They must not
    // punch holes in the nav mesh.
    let mut collision = floor(-1.0, 1.97, -1.0, 5.0, 0.0);
    collision.extend(floor(2.03, 5.0, -1.0, 5.0, 0.0));
    let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert_eq!(result.removed_unsupported, 0, "{result:?}");
    assert!(walkable_at(&result, 2.0, 2.0), "{result:?}");
}

#[test]
fn a_floor_a_full_step_below_still_supports() {
    let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
    let collision = floor(-1.0, 5.0, -1.0, 5.0, -0.4);
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert_eq!(result.removed_unsupported, 0, "{result:?}");
}

#[test]
fn a_floor_beyond_the_step_below_does_not_support() {
    let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
    let collision = floor(-1.0, 5.0, -1.0, 5.0, -1.0);
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert_eq!(result.removed_unsupported, 2, "{result:?}");
}

/// A big triangle midpoint-subdivided into four: the central triangle
/// (index 3, vertices D,E,F) is the one fully interior triangle.
fn subdivided_triangle() -> NavClearanceMeshInput {
    NavClearanceMeshInput {
        vertices: vec![
            [0.0, 0.0, 0.0], // 0 A
            [4.0, 0.0, 0.0], // 1 B
            [0.0, 0.0, 4.0], // 2 C
            [2.0, 0.0, 0.0], // 3 D
            [2.0, 0.0, 2.0], // 4 E
            [0.0, 0.0, 2.0], // 5 F
        ],
        polygons: vec![[0, 3, 5], [3, 1, 4], [5, 4, 2], [3, 4, 5]],
        protected_edges: Vec::new(),
    }
}

#[test]
fn a_tall_interior_collider_is_clipped_out_with_an_agent_radius_margin() {
    let mesh = subdivided_triangle();
    let mut collision = floor(-1.0, 5.0, -1.0, 5.0, 0.0);
    // A tall wall stub (rises well above the step height) inside the room.
    collision.extend(wall(1.2, 1.5, 1.33, 0.0, 2.0));
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert_eq!(result.removed_unsupported, 0, "{result:?}");
    assert!(result.cut_obstructed > 0, "{result:?}");
    assert!(
        !walkable_at(&result, 1.35, 1.33),
        "the stub's own footprint must be clipped away"
    );
    assert!(
        !walkable_at(&result, 1.35, 1.15),
        "and so must the agent-radius margin around it"
    );
    assert!(
        walkable_at(&result, 0.3, 0.3),
        "floor a clear distance from the stub must survive: {result:?}"
    );
}

/// A flight of stairs steep enough that the riser *two steps up* is still
/// within the agent radius while already rising more than a step height
/// above the tread the agent stands on. Judged from the query point, that
/// classifies the whole flight as wall and strands everything it serves;
/// judged from each riser's own footing (the tread it stands on) the
/// flight stays walkable and both levels stay connected. Vault 101's
/// stairs are this shape: the footing rule took that cell's
/// largest-component share from 90% to 98%.
/// A collider is judged against the walkable surface at *its own*
/// footprint, not at the query point. Here a 0.3 m riser stands on a
/// 0.3 m ledge the agent can step onto, so it is one step above what it
/// rests on -- climbable -- even though its top (0.6 m) is past the step
/// height measured from the query point. This is the shape every
/// staircase has, and measuring from the query point instead classifies
/// flights as walls: it cost Vault 101 8 points of largest-component
/// share (90% -> 98%) by stranding everything its stairs serve.
#[test]
fn a_riser_standing_on_a_reachable_ledge_is_climbable_not_an_obstruction() {
    let mesh = nav_quad(0.0, 4.0, 0.0, 2.0, 0.0);
    let mut collision = floor(-1.0, 5.0, -1.0, 3.0, 0.0);
    // The ledge the riser stands on, one step up and within reach.
    collision.extend(floor(2.0, 5.0, -1.0, 3.0, 0.3));
    // The riser itself: from the ledge top up another step.
    collision.extend(wall_along_z(2.0, -1.0, 3.0, 0.3, 0.6));
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert_eq!(result.cut_obstructed, 0, "{result:?}");
    assert!(
        walkable_at(&result, 1.8, 1.0),
        "floor right up against a climbable riser stays walkable: {result:?}"
    );
}

/// The same riser with nothing to stand on at its footprint: it rises
/// 0.6 m straight off the floor, past the step height, so it obstructs
/// and its agent-radius margin is clipped away.
#[test]
fn the_same_riser_with_no_footing_obstructs() {
    let mesh = nav_quad(0.0, 4.0, 0.0, 2.0, 0.0);
    let mut collision = floor(-1.0, 5.0, -1.0, 3.0, 0.0);
    collision.extend(wall_along_z(2.0, -1.0, 3.0, 0.3, 0.6));
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert!(result.cut_obstructed > 0, "{result:?}");
    assert!(
        !walkable_at(&result, 1.8, 1.0),
        "floor inside the riser's agent-radius margin is clipped: {result:?}"
    );
}

#[test]
fn a_step_overable_riser_does_not_cut_the_stair_tread() {
    // A short riser (0.0..0.3 m, under the 0.5 m step height) on the same
    // interior triangle must NOT cut it -- stairs stay traversable.
    let mesh = subdivided_triangle();
    let mut collision = floor(-1.0, 5.0, -1.0, 5.0, 0.0);
    collision.extend(wall(1.2, 1.5, 1.33, 0.0, 0.3));
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert_eq!(result.cut_obstructed, 0, "{result:?}");
}

#[test]
fn a_perimeter_wall_erodes_only_an_agent_radius_strip() {
    // A room whose nav mesh runs up to its own perimeter wall keeps
    // everything an agent radius clear of it. Wave 10 could only choose
    // between cutting a whole perimeter triangle (eroding whole rooms) and
    // cutting nothing; the clip takes exactly the strip the capsule cannot
    // occupy, and the room stays one connected component.
    let mesh = nav_quad(0.0, 4.0, 0.0, 2.0, 0.0);
    let mut collision = floor(-1.0, 5.0, -1.0, 3.0, 0.0);
    collision.extend(wall(-1.0, 5.0, 0.0, 0.0, 2.0));
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert!(
        !walkable_at(&result, 2.0, 0.2),
        "the strip against the wall must be clipped"
    );
    assert!(
        walkable_at(&result, 2.0, 1.5),
        "the rest of the room must survive: {result:?}"
    );
    assert_eq!(result.component_count, 1, "{result:?}");
}

#[test]
fn posts_flanking_a_triangles_opening_leave_a_wide_enough_passage_connected() {
    // F171.1 / the #148 metro-entrance class: two posts flank an opening
    // *inside* a nav triangle, so the triangle's centroid stays clear and
    // no whole-triangle test can see them. The gap between them is 1.4 m --
    // wider than the 0.7 m agent diameter -- so the clip must leave a
    // passage that still connects the two sides.
    let mesh = nav_quad(0.0, 6.0, 0.0, 6.0, 0.0);
    let mut collision = floor(-1.0, 7.0, -1.0, 7.0, 0.0);
    collision.extend(wall(0.0, 2.3, 3.0, 0.0, 2.0));
    collision.extend(wall(3.7, 6.0, 3.0, 0.0, 2.0));
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert!(result.cut_obstructed > 0, "{result:?}");
    assert!(
        walkable_at(&result, 3.0, 3.0),
        "the gap between the posts must stay walkable: {result:?}"
    );
    assert!(
        connected(&result, (3.0, 1.0), (3.0, 5.0)),
        "the two sides must stay connected through the gap: {result:?}"
    );
}

#[test]
fn posts_flanking_a_triangles_opening_disconnect_a_sub_diameter_passage() {
    // The same geometry with a 0.5 m gap -- narrower than the agent
    // diameter. The clip closes it, and the two sides are honestly
    // unreachable from one another rather than being reconnected by the
    // connectivity guard: a route across is `unreachable` at query time
    // instead of wedging an agent in the frame.
    let mesh = nav_quad(0.0, 6.0, 0.0, 6.0, 0.0);
    let mut collision = floor(-1.0, 7.0, -1.0, 7.0, 0.0);
    collision.extend(wall(0.0, 2.75, 3.0, 0.0, 2.0));
    collision.extend(wall(3.25, 6.0, 3.0, 0.0, 2.0));
    let result = validate_and_clear(&mesh, &collision, NavClearanceParams::default());
    assert!(
        !walkable_at(&result, 3.0, 3.0),
        "a sub-diameter gap must close: {result:?}"
    );
    assert!(
        !connected(&result, (3.0, 1.0), (3.0, 5.0)),
        "the two sides must not stay connected: {result:?}"
    );
}

/// A corridor along X of `half_widths.len()` stations, centred on z=1,
/// with a per-station half-width so it can be pinched in the middle.
fn pinched_corridor(half_widths: &[f32]) -> NavClearanceMeshInput {
    let mut vertices = Vec::new();
    for (i, hw) in half_widths.iter().enumerate() {
        let x = i as f32 * 2.0;
        vertices.push([x, 0.0, 1.0 - hw]); // bottom row, index 2*i
        vertices.push([x, 0.0, 1.0 + hw]); // top row, index 2*i + 1
    }
    let mut polygons = Vec::new();
    for i in 0..half_widths.len() - 1 {
        let ba = (2 * i) as u32;
        let ta = (2 * i + 1) as u32;
        let bb = (2 * (i + 1)) as u32;
        let tb = (2 * (i + 1) + 1) as u32;
        polygons.push([ba, bb, tb]);
        polygons.push([ba, tb, ta]);
    }
    NavClearanceMeshInput {
        vertices,
        polygons,
        protected_edges: Vec::new(),
    }
}

#[test]
fn a_one_metre_doorway_stays_connected_with_an_eroded_passage() {
    // A uniform 1.0 m corridor (half-width 0.5 > radius): the centre line
    // is 0.5 m > 0.35 m from each wall, so every triangle keeps a fitting
    // point -- one connected component, nothing dropped.
    let mesh = pinched_corridor(&[0.5, 0.5, 0.5, 0.5]);
    let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
    assert_eq!(result.dropped_unfit, 0, "{result:?}");
    assert_eq!(result.component_count, 1, "{result:?}");
    assert_eq!(result.largest_component, mesh.polygons.len(), "{result:?}");
}

#[test]
fn a_sub_diameter_pinch_disconnects_the_two_wide_ends() {
    // Wide (half-width 1.0) at both ends, pinched to a 0.5 m gap
    // (half-width 0.25 < radius) in the middle: the neck fits nowhere and
    // drops, splitting the corridor into two components; the wide ends
    // survive.
    let mesh = pinched_corridor(&[1.0, 0.25, 0.25, 1.0]);
    let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
    assert!(result.dropped_unfit > 0, "the neck must drop: {result:?}");
    assert_eq!(
        result.component_count, 2,
        "the pinch must split the corridor: {result:?}"
    );
    assert!(
        result.walkable_count > 0,
        "the wide ends survive: {result:?}"
    );
}

#[test]
fn protected_triangles_are_never_dropped_or_cut() {
    // A narrow (sub-diameter) quad whose z=0 edge is protected: the
    // protected triangle stays walkable even though the agent does not
    // fit, so an authored doorway/seam is never severed.
    let mut mesh = nav_quad(0.0, 4.0, 0.0, 0.3, 0.0);
    mesh.protected_edges = vec![(0, 1)];
    let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
    // Polygon 0 = [0,1,2] owns edge (0,1) -> protected; polygon 1 =
    // [0,2,3] does not own (0,1).
    assert!(result.walkable[0], "protected triangle stays walkable");
    assert_eq!(result.protected_count, 1, "{result:?}");
}

/// F171.5. Landmass validates a mesh as a whole and rejects all of it over
/// a single bad polygon, leaving the cell with no navigation at all --
/// which no connectivity metric can see, because they measure the graph
/// this pass built rather than the graph the runtime will accept. The gate
/// is what closes that hole, so it must fire on a polygon whose vertices
/// are collinear.
#[test]
fn the_geometry_gate_rejects_a_collinear_polygon() {
    let mut mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
    // A third triangle whose three vertices lie on one line.
    mesh.vertices.push([2.0, 0.0, 0.0]);
    mesh.polygons.push([0, 4, 1]);
    let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
    assert_eq!(result.invalid_geometry, 1, "{result:?}");
    assert!(!result.walkable[2], "the collinear polygon must be dropped");
    assert!(
        result.walkable[0] && result.walkable[1],
        "and nothing else: {result:?}"
    );
}

/// The other way landmass rejects a mesh: one polygon wound against the
/// rest. The runtime validates under a single global winding (retrying
/// reversed as a whole), so an inverted polygon can never be accommodated
/// -- it invalidates every other polygon with it.
#[test]
fn the_geometry_gate_rejects_a_polygon_wound_against_its_mesh() {
    let mut mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
    mesh.vertices.push([6.0, 0.0, 0.0]);
    mesh.vertices.push([6.0, 0.0, 4.0]);
    // Wound the opposite way round from the quad's own two triangles.
    mesh.polygons.push([1, 5, 2]);
    mesh.polygons.push([5, 2, 6]);
    let result = validate_and_clear(&mesh, &[], NavClearanceParams::default());
    assert!(result.invalid_geometry > 0, "{result:?}");
    assert!(
        result.walkable[0] && result.walkable[1],
        "the majority winding survives: {result:?}"
    );
}

#[test]
fn the_pass_is_deterministic_across_calls() {
    let mesh = pinched_corridor(&[1.0, 0.25, 0.25, 1.0]);
    let first = validate_and_clear(&mesh, &[], NavClearanceParams::default());
    let second = validate_and_clear(&mesh, &[], NavClearanceParams::default());
    assert_eq!(first, second);
}

#[test]
fn a_zero_radius_pass_is_a_no_op() {
    let mesh = nav_quad(0.0, 4.0, 0.0, 4.0, 0.0);
    let params = NavClearanceParams {
        agent_radius: 0.0,
        ..NavClearanceParams::default()
    };
    let result = validate_and_clear(&mesh, &[], params);
    assert!(result.walkable.iter().all(|&w| w));
}
