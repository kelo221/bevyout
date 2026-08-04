use bevy::prelude::*;
use bevy_boxddd::boxddd;
use bevyout_core::manifest::exterior::{ExteriorCellPackage, GridCoordinate, matching_portals};

use crate::viewer::nav::agent::*;
use crate::viewer::nav::landmass_graph;
use crate::viewer::nav::world::links::animation_link_start_edge;
use crate::viewer::player;
use crate::vsa::PreparedNavGraph;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExteriorPortalSide {
    pub(crate) mesh_form_id: u32,
    pub(crate) triangle_index: u32,
    pub(crate) interval: [[f32; 3]; 2],
    pub(crate) triangle: [[f32; 3]; 3],
    pub(crate) matched_edge: usize,
    pub(crate) residual: f32,
    pub(crate) border_plane_residual: f32,
}

/// Move an exterior seam's animation-link endpoints into the owning cell's
/// walkable side. Fallout's adjacent NAVM tiles deliberately share the same
/// world-space border, so leaving a point exactly on that seam lets landmass's
/// point sampler choose either island (or neither after clearance). The
/// prepared nav boundary already includes the agent-radius erosion; one
/// radius is therefore the smallest stable inset that makes both endpoints
/// unambiguous without changing the authored crossing height.
pub(crate) const EXTERIOR_PORTAL_LINK_INSET_METRES: f32 = AGENT_RADIUS;

/// Landmass receives a point endpoint, while the source side also receives a
/// tiny finite segment (`ANIMATION_LINK_PORTAL_HALF_LENGTH`). Keep the final
/// point far enough inside the selected post-clearance triangle for that
/// segment to remain attached, without moving the crossing a full agent
/// radius away from the shared seam.
pub(crate) const EXTERIOR_PORTAL_MIN_INTERIOR_MARGIN_METRES: f32 = 0.02;

pub(crate) fn interval_midpoint(interval: [[f32; 3]; 2]) -> Vec3 {
    (Vec3::from_array(interval[0]) + Vec3::from_array(interval[1])) * 0.5
}

pub(crate) fn triangle_centroid(triangle: [[f32; 3]; 3]) -> Vec3 {
    (Vec3::from_array(triangle[0]) + Vec3::from_array(triangle[1]) + Vec3::from_array(triangle[2]))
        / 3.0
}

pub(crate) fn signed_triangle_area_xz(a: Vec3, b: Vec3, point: Vec3) -> f32 {
    (b.x - a.x) * (point.z - a.z) - (b.z - a.z) * (point.x - a.x)
}

pub(crate) fn point_in_triangle_xz(point: Vec3, triangle: [[f32; 3]; 3]) -> bool {
    let vertices = triangle.map(Vec3::from_array);
    let areas = [
        signed_triangle_area_xz(vertices[0], vertices[1], point),
        signed_triangle_area_xz(vertices[1], vertices[2], point),
        signed_triangle_area_xz(vertices[2], vertices[0], point),
    ];
    let has_negative = areas.iter().any(|area| *area < -1e-5);
    let has_positive = areas.iter().any(|area| *area > 1e-5);
    !(has_negative && has_positive)
}

pub(crate) fn point_residual(left: [f32; 3], right: [f32; 3]) -> f32 {
    left.into_iter()
        .zip(right)
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, f32::max)
}

pub(crate) fn border_plane_residual(
    interval: [[f32; 3]; 2],
    portal: &bevyout_core::manifest::exterior::ExteriorBorderPortal,
) -> f32 {
    let axis = if portal.edge <= 1 { 0 } else { 2 };
    let expected = (portal.start[axis] + portal.end[axis]) * 0.5;
    (interval[0][axis] - expected).abs() + (interval[1][axis] - expected).abs()
}

pub(crate) fn exterior_portal_point_candidates(side: &ExteriorPortalSide, edge: u8) -> Vec<Vec3> {
    let raw_midpoint = interval_midpoint(side.interval);
    let requested_midpoint = interval_midpoint(inset_exterior_portal_interval(side.interval, edge));
    let centroid = triangle_centroid(side.triangle);
    let mut candidates = Vec::new();

    // Preserve the authored-radius inset when it is actually inside the
    // selected polygon. The post-clearance edge already represents the
    // agent's walkable boundary, so only the invalid cardinal inset falls
    // back to a smaller centroid-directed point.
    if point_in_triangle_xz(requested_midpoint, side.triangle)
        && requested_midpoint.distance(raw_midpoint) >= EXTERIOR_PORTAL_MIN_INTERIOR_MARGIN_METRES
    {
        candidates.push(requested_midpoint);
    }

    let toward_centroid = centroid - raw_midpoint;
    let centroid_distance = toward_centroid.length();
    if centroid_distance.is_finite() && centroid_distance > 0.0 {
        let direction = toward_centroid / centroid_distance;
        // Increasing margins are deterministic fallbacks for shallow or
        // diagonally selected border triangles. Every point lies on the
        // edge-to-centroid segment, so convexity keeps it in the triangle.
        for margin in [
            EXTERIOR_PORTAL_MIN_INTERIOR_MARGIN_METRES,
            0.05,
            0.1,
            0.2,
            0.35,
            0.5,
        ] {
            if margin >= centroid_distance {
                continue;
            }
            let candidate = raw_midpoint + direction * margin;
            if point_in_triangle_xz(candidate, side.triangle)
                && !candidates
                    .iter()
                    .any(|existing: &Vec3| existing.distance_squared(candidate) < 1e-8)
            {
                candidates.push(candidate);
            }
        }
    }
    candidates
}

pub(crate) fn source_segment_inside_triangle(
    from: Vec3,
    to: Vec3,
    triangle: [[f32; 3]; 3],
) -> bool {
    let (start, end) = animation_link_start_edge(from, to);
    point_in_triangle_xz(start, triangle) && point_in_triangle_xz(end, triangle)
}

pub(crate) fn select_exterior_portal_points(
    left: &ExteriorPortalSide,
    left_edge: u8,
    right: &ExteriorPortalSide,
    right_edge: u8,
) -> Option<(Vec3, Vec3)> {
    let left_candidates = exterior_portal_point_candidates(left, left_edge);
    let right_candidates = exterior_portal_point_candidates(right, right_edge);
    left_candidates
        .iter()
        .flat_map(|left_point| {
            right_candidates
                .iter()
                .map(move |right_point| (*left_point, *right_point))
        })
        .find(|(left_point, right_point)| {
            source_segment_inside_triangle(*left_point, *right_point, left.triangle)
                && source_segment_inside_triangle(*right_point, *left_point, right.triangle)
        })
}

pub(crate) fn exterior_portal_inward_direction(edge: u8) -> Vec3 {
    match edge {
        // `navigation_border_portals` uses 0=max X, 1=min X, 2=min Z,
        // 3=max Z.
        0 => Vec3::NEG_X,
        1 => Vec3::X,
        2 => Vec3::Z,
        3 => Vec3::NEG_Z,
        _ => Vec3::ZERO,
    }
}

pub(crate) fn inset_exterior_portal_interval(interval: [[f32; 3]; 2], edge: u8) -> [[f32; 3]; 2] {
    let offset = exterior_portal_inward_direction(edge) * EXTERIOR_PORTAL_LINK_INSET_METRES;
    [
        (Vec3::from_array(interval[0]) + offset).to_array(),
        (Vec3::from_array(interval[1]) + offset).to_array(),
    ]
}

/// Converts producer-generated, world-space border portals into ordinary
/// landmass merge inputs. Matching is restricted to adjacent grid cells and
/// each side is resolved back to its semantic prepared polygon, so links are
/// owned by the same resident graph set as their endpoint islands.
pub(crate) fn exterior_portal_merge_inputs(
    packages: &[(GridCoordinate, ExteriorCellPackage)],
    graph: &PreparedNavGraph,
) -> Vec<landmass_graph::MergeInput> {
    let mut links = Vec::new();
    let mut adjacent_pairs = 0_usize;
    let mut matched_portals = 0_usize;
    let mut unresolved_left = 0_usize;
    let mut unresolved_right = 0_usize;
    let mut unsafe_endpoints = 0_usize;
    for (left_index, (left_grid, left_package)) in packages.iter().enumerate() {
        for (right_grid, right_package) in packages.iter().skip(left_index + 1) {
            let adjacent =
                (left_grid.x - right_grid.x).abs() + (left_grid.y - right_grid.y).abs() == 1;
            if !adjacent {
                continue;
            }
            adjacent_pairs += 1;
            let (Some(left_navigation), Some(right_navigation)) = (
                left_package.navigation.as_ref(),
                right_package.navigation.as_ref(),
            ) else {
                continue;
            };
            let portal_matches = matching_portals(
                *left_grid,
                &left_navigation.border_portals,
                *right_grid,
                &right_navigation.border_portals,
            );
            matched_portals += portal_matches.len();
            for (left_portal_index, right_portal_index) in portal_matches {
                let Some(left_side) = find_exterior_portal_side(
                    graph,
                    &left_navigation.border_portals[left_portal_index],
                    left_package.cell_form_id,
                ) else {
                    unresolved_left += 1;
                    continue;
                };
                let Some(right_side) = find_exterior_portal_side(
                    graph,
                    &right_navigation.border_portals[right_portal_index],
                    right_package.cell_form_id,
                ) else {
                    unresolved_right += 1;
                    continue;
                };
                let left_portal = &left_navigation.border_portals[left_portal_index];
                let right_portal = &right_navigation.border_portals[right_portal_index];
                let Some((left_point, right_point)) = select_exterior_portal_points(
                    &left_side,
                    left_portal.edge,
                    &right_side,
                    right_portal.edge,
                ) else {
                    unsafe_endpoints += 1;
                    warn!(
                        "exterior nav border endpoint rejected cells={:08x}<->{:08x} grids=({},{})->({},{}), meshes={:08x}/{:08x}, triangles={}/{}, matched_edges={}/{}, residuals={:.4}/{:.4}",
                        left_package.cell_form_id,
                        right_package.cell_form_id,
                        left_grid.x,
                        left_grid.y,
                        right_grid.x,
                        right_grid.y,
                        left_side.mesh_form_id,
                        right_side.mesh_form_id,
                        left_side.triangle_index,
                        right_side.triangle_index,
                        left_side.matched_edge,
                        right_side.matched_edge,
                        left_side.residual,
                        right_side.residual,
                    );
                    continue;
                };
                let raw_left = interval_midpoint(left_side.interval);
                let raw_right = interval_midpoint(right_side.interval);
                let requested_left = interval_midpoint(inset_exterior_portal_interval(
                    left_side.interval,
                    left_portal.edge,
                ));
                let requested_right = interval_midpoint(inset_exterior_portal_interval(
                    right_side.interval,
                    right_portal.edge,
                ));
                info!(
                    "exterior-nav-portal cells={:08x}<->{:08x} grids=({},{})->({},{}), edges={}/{}, meshes={:08x}/{:08x}, triangles={}/{}, matched_edges={}/{}, residuals={:.4}/{:.4}, raw_midpoints=({:.3},{:.3},{:.3})/({:.3},{:.3},{:.3}), inset_midpoints=({:.3},{:.3},{:.3})/({:.3},{:.3},{:.3}), link_points=({:.3},{:.3},{:.3})/({:.3},{:.3},{:.3}), inside_selected=true,true",
                    left_package.cell_form_id,
                    right_package.cell_form_id,
                    left_grid.x,
                    left_grid.y,
                    right_grid.x,
                    right_grid.y,
                    left_portal.edge,
                    right_portal.edge,
                    left_side.mesh_form_id,
                    right_side.mesh_form_id,
                    left_side.triangle_index,
                    right_side.triangle_index,
                    left_side.matched_edge,
                    right_side.matched_edge,
                    left_side.residual,
                    right_side.residual,
                    raw_left.x,
                    raw_left.y,
                    raw_left.z,
                    raw_right.x,
                    raw_right.y,
                    raw_right.z,
                    requested_left.x,
                    requested_left.y,
                    requested_left.z,
                    requested_right.x,
                    requested_right.y,
                    requested_right.z,
                    left_point.x,
                    left_point.y,
                    left_point.z,
                    right_point.x,
                    right_point.y,
                    right_point.z,
                );
                links.push(landmass_graph::MergeInput {
                    mesh_a_form_id: left_side.mesh_form_id,
                    triangle_a: left_side.triangle_index,
                    mesh_b_form_id: right_side.mesh_form_id,
                    triangle_b: right_side.triangle_index,
                    // The descriptor consumes the interval midpoint. Keep
                    // each side as a degenerate interval at the verified
                    // interior point so no later stage reconstructs the
                    // invalid cardinally-inset endpoint.
                    interval_a: [left_point.to_array(), left_point.to_array()],
                    interval_b: [right_point.to_array(), right_point.to_array()],
                });
            }
        }
    }
    links.sort_by(|left, right| {
        left.mesh_a_form_id
            .cmp(&right.mesh_a_form_id)
            .then_with(|| left.triangle_a.cmp(&right.triangle_a))
            .then_with(|| left.mesh_b_form_id.cmp(&right.mesh_b_form_id))
            .then_with(|| left.triangle_b.cmp(&right.triangle_b))
            .then_with(|| left.interval_a[0][0].total_cmp(&right.interval_a[0][0]))
            .then_with(|| left.interval_a[0][1].total_cmp(&right.interval_a[0][1]))
            .then_with(|| left.interval_a[0][2].total_cmp(&right.interval_a[0][2]))
    });
    links.dedup();
    if adjacent_pairs > 0 {
        warn!(
            "exterior nav border candidates adjacent_pairs={} matched_portals={} resolved_links={} unresolved_left={} unresolved_right={} unsafe_endpoints={}",
            adjacent_pairs,
            matched_portals,
            links.len(),
            unresolved_left,
            unresolved_right,
            unsafe_endpoints,
        );
    }
    links
}

pub(crate) fn find_exterior_portal_side(
    graph: &PreparedNavGraph,
    portal: &bevyout_core::manifest::exterior::ExteriorBorderPortal,
    cell_form_id: u32,
) -> Option<ExteriorPortalSide> {
    // Clearance can move non-protected boundary vertices inward by roughly
    // one agent radius. The producer portal remains the authored/raw edge;
    // use a bounded matching band here rather than requiring post-clearance
    // vertices to be bit-identical to that source edge.
    let tolerance = portal.tolerance.max(0.75);
    let mut best: Option<ExteriorPortalSide> = None;
    for mesh in &graph.meshes {
        if mesh.cell_form_id != Some(cell_form_id) {
            continue;
        }
        for polygon in &mesh.polygons {
            if !polygon.walkable {
                continue;
            }
            for edge in 0..3 {
                let Some(&a) = mesh.vertices.get(polygon.vertex_indices[edge] as usize) else {
                    continue;
                };
                let Some(&b) = mesh
                    .vertices
                    .get(polygon.vertex_indices[(edge + 1) % 3] as usize)
                else {
                    continue;
                };
                // The source border edge can retain a same-mesh NVTR
                // neighbor even when it is also an exterior seam. Endpoint
                // identity is the stronger contract here; requiring
                // `adjacency == None` would drop valid authored border
                // portals before the resident-cell matcher can own them.
                let interval = if points_close(a, portal.start, tolerance)
                    && points_close(b, portal.end, tolerance)
                {
                    Some([a, b])
                } else if points_close(a, portal.end, tolerance)
                    && points_close(b, portal.start, tolerance)
                {
                    Some([b, a])
                } else {
                    None
                };
                if let Some(interval) = interval {
                    let Some(triangle) = polygon
                        .vertex_indices
                        .into_iter()
                        .map(|index| mesh.vertices.get(index as usize).copied())
                        .collect::<Option<Vec<_>>>()
                        .and_then(|vertices| vertices.try_into().ok())
                    else {
                        continue;
                    };
                    let candidate = ExteriorPortalSide {
                        mesh_form_id: mesh.form_id,
                        triangle_index: polygon.index,
                        interval,
                        triangle,
                        matched_edge: edge,
                        residual: point_residual(interval[0], portal.start)
                            + point_residual(interval[1], portal.end),
                        border_plane_residual: border_plane_residual(interval, portal),
                    };
                    let replace = best.as_ref().is_none_or(|current| {
                        candidate
                            .residual
                            .total_cmp(&current.residual)
                            .then_with(|| {
                                candidate
                                    .border_plane_residual
                                    .total_cmp(&current.border_plane_residual)
                            })
                            .then_with(|| candidate.mesh_form_id.cmp(&current.mesh_form_id))
                            .then_with(|| candidate.triangle_index.cmp(&current.triangle_index))
                            .then_with(|| candidate.matched_edge.cmp(&current.matched_edge))
                            .is_lt()
                    });
                    if replace {
                        best = Some(candidate);
                    }
                }
            }
        }
    }
    best
}

pub(crate) fn points_close(left: [f32; 3], right: [f32; 3], tolerance: f32) -> bool {
    left.into_iter()
        .zip(right)
        .all(|(left, right)| (left - right).abs() <= tolerance)
}

/// Why a merge portal candidate failed runtime collision-visibility
/// validation (issue #154 real-data acceptance correction). Reported once
/// per dropped link via a stable `warn!` line naming both sides' mesh/
/// triangle ids (`ensure_archipelago`).
#[derive(Debug, Clone, Copy)]
pub(crate) enum MergeLinkRejection {
    /// The capsule sweep from the near portal point to the far one did not
    /// reach the far point without first contacting something.
    SweptBlocked,
    /// No walkable ground support was found within step height below the
    /// crossing's midpoint or its far point.
    NoGroundSupport,
}

impl MergeLinkRejection {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            MergeLinkRejection::SweptBlocked => "swept blocked",
            MergeLinkRejection::NoGroundSupport => "no ground support",
        }
    }
}

/// Runtime collision-visibility validation for one merge portal candidate
/// (issue #154 real-data acceptance correction): prepare-side geometric
/// validation (opposing directions, an overlapping interval --
/// `vsa::prepare::nav_graph::validate_portal_candidate`) has no cooked
/// physics to check against, and real FranklinMetro02 data showed it can
/// accept a candidate that is a genuine seam in the abstract navmesh
/// topology but empty air (or blocked by intervening geometry) in the
/// actual level -- one accepted portal with a 1.69 m XZ gap swept a live
/// agent clean off the mesh edge into the void (`y` still falling at
/// -348 m when observed). This runs once per candidate link at
/// archipelago-build time (`ensure_archipelago`, where the cooked BoxDDD
/// collision world is already available), mirroring where issue #154's
/// step-height check already moved to for the identical "no cooked
/// physics prepare-side" reason.
///
/// Two checks, both using the same capsule/filters ordinary agent movement
/// uses (`step_agent_kcc`'s own `mover`/`collision_filter`/
/// `support_filter`, constructed identically by the caller):
/// 1. Ground support (`player::try_step_down` -- the same step-height-
///    bounded downward probe the KCC itself uses when stepping down) must
///    exist within step height below both the crossing's midpoint and
///    `end`. This is what actually catches the void-fall case: a capsule
///    swept purely horizontally never contacts a floor that simply is not
///    there underneath it.
/// 2. A capsule slide from `start` to `end` (`player::move_mover`, the
///    same move-and-slide collision response ordinary agent/player
///    movement runs every tick -- deliberately *not* a single raw
///    `boxddd::World::cast_mover`) must actually arrive within a small
///    tolerance. A raw single sweep starting exactly at `start` routinely
///    reports "blocked immediately" for an otherwise walkable seam: `start`
///    is an un-eroded seam boundary point (`erosion_policy`'s protected-
///    edge rule deliberately never pulls a merge-triangle vertex inward,
///    so both sides keep agreeing on the same seam position), which in
///    real FO3 data commonly sits flush against the near-side wall -- a
///    capsule centred exactly there already touches that wall at the very
///    first query. `move_mover`'s plane-based sliding is what real
///    per-tick movement already relies on to handle a capsule touching a
///    wall without misreporting the whole crossing as impassable; a raw
///    cast has no such contact tolerance.
///
/// `start`/`end` are feet-level points (the same convention every other
/// nav-graph point in this module uses -- see `TRAVEL_ARRIVAL_DISTANCE`'s
/// doc comment); both are raised by `AGENT_HEIGHT / 2` to the capsule-
/// centre height `step_agent_kcc`'s own `origin` convention expects before
/// either check runs.
pub(crate) fn validate_merge_link_collision(
    world: &mut boxddd::World,
    mover: &boxddd::Capsule,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
    start: Vec3,
    end: Vec3,
) -> Result<(), MergeLinkRejection> {
    validate_merge_link_collision_with_support_probe(
        world,
        mover,
        collision_filter,
        support_filter,
        start,
        end,
        true,
    )
}

/// The adjacent-cell variant of [`validate_merge_link_collision`]. Cell
/// colliders can legitimately leave the exact shared seam midpoint without a
/// support hit even when both inset endpoints are supported and the capsule
/// can cross the seam. Do not weaken same-cell merge validation for that
/// streaming-boundary artifact.
pub(crate) fn validate_exterior_merge_link_collision(
    world: &mut boxddd::World,
    mover: &boxddd::Capsule,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
    start: Vec3,
    end: Vec3,
) -> Result<(), MergeLinkRejection> {
    validate_merge_link_collision_with_support_probe(
        world,
        mover,
        collision_filter,
        support_filter,
        start,
        end,
        false,
    )
}

pub(crate) fn validate_merge_link_collision_with_support_probe(
    world: &mut boxddd::World,
    mover: &boxddd::Capsule,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
    start: Vec3,
    end: Vec3,
    probe_midpoint: bool,
) -> Result<(), MergeLinkRejection> {
    let to_capsule_center = Vec3::new(0.0, AGENT_HEIGHT * 0.5, 0.0);
    let start_origin = start + to_capsule_center;
    let end_origin = end + to_capsule_center;
    let mid_origin = start_origin.lerp(end_origin, 0.5);

    if probe_midpoint
        && player::try_step_down(
            world,
            player::to_box_vec3(mid_origin),
            mover,
            boxddd::Vec3::ZERO,
            collision_filter,
            support_filter,
        )
        .is_none()
    {
        return Err(MergeLinkRejection::NoGroundSupport);
    }
    if player::try_step_down(
        world,
        player::to_box_vec3(end_origin),
        mover,
        boxddd::Vec3::ZERO,
        collision_filter,
        support_filter,
    )
    .is_none()
    {
        return Err(MergeLinkRejection::NoGroundSupport);
    }

    let delta = player::to_box_vec3(end_origin - start_origin);
    let (achieved_box, ..) = player::move_mover(
        world,
        player::to_box_vec3(start_origin),
        mover,
        delta,
        collision_filter,
        support_filter,
        true,
        false,
    );
    let achieved = player::from_box_vec3(achieved_box);
    if (achieved - end_origin).length() > MERGE_LINK_SWEEP_TOLERANCE {
        return Err(MergeLinkRejection::SweptBlocked);
    }
    Ok(())
}
