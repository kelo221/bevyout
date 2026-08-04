use bevy::prelude::*;
use bevy_landmass::prelude::*;

/// Spawns one logical off-mesh link as *two* unidirectional
/// `AnimationLink3d`s (start -> end and end -> start) rather than one
/// `bidirectional: true` link: landmass 0.9.1's bidirectional path
/// (`nav_data.rs`'s reverse `OffMeshLink` insert) indexes the *start*
/// island's polygon array with the *end* portal's polygon index when
/// computing `destination_type_index`, which panics ("index out of bounds")
/// the moment the two ends sit on different islands -- exactly the
/// cross-mesh case every link this module spawns is for. Confirmed on real
/// FranklinMetro02 data (end polygon 260 vs start island's 72 polygons);
/// two unidirectional links take the correctly-indexed non-bidirectional
/// path and are semantically identical. Reported upstream as
/// <https://github.com/andriyDev/landmass/issues/192>; collapse back to one
/// `bidirectional: true` link once a fixed release is adopted.
///
/// `cost` (issue #154 feature 3): door links keep passing the previous flat
/// `1.0`; merge links pass their own `MergeLinkDescriptor::distance` (real
/// traversal distance between the two portal-interval midpoints) so
/// landmass's route cost reflects how far a crossing actually moves the
/// agent instead of treating every merge seam as equally cheap.
///
/// Landmass 0.9.2 assumes every portal passed through its boundary clipping
/// path has a non-zero horizontal extent. Point portals are valid animation
/// links, but a point animation link can share a node with a native boundary
/// link; Landmass then normalizes that point while building clip polygons and
/// produces NaNs. Keep the destination as a point for the intended sampling
/// semantics, while giving the source a tiny finite horizontal portal. This
/// is deliberately local to the adapter and can be removed once the upstream
/// Landmass boundary-link filtering fix is available.
pub(crate) const ANIMATION_LINK_PORTAL_HALF_LENGTH: f32 = 0.005;

pub(crate) fn animation_link_start_edge(from: Vec3, to: Vec3) -> (Vec3, Vec3) {
    let horizontal = Vec3::new(to.x - from.x, 0.0, to.z - from.z);
    let direction = horizontal.normalize_or_zero();
    let direction = if direction.length_squared() > 0.0 {
        direction
    } else {
        Vec3::X
    };
    (
        from - direction * ANIMATION_LINK_PORTAL_HALF_LENGTH,
        from + direction * ANIMATION_LINK_PORTAL_HALF_LENGTH,
    )
}

pub(crate) fn animation_link_end_edge(from: Vec3, to: Vec3) -> (Vec3, Vec3) {
    let horizontal = Vec3::new(to.x - from.x, 0.0, to.z - from.z);
    let direction = horizontal.normalize_or_zero();
    let direction = if direction.length_squared() > 0.0 {
        direction
    } else {
        Vec3::X
    };
    (
        to - direction * ANIMATION_LINK_PORTAL_HALF_LENGTH,
        to + direction * ANIMATION_LINK_PORTAL_HALF_LENGTH,
    )
}

/// `kind` (issue #162 feature 1): every door link passes the reserved `0`
/// (never quarantined); a merge link passes its own deterministic
/// `landmass_graph::merge_link_kind`, giving `PermittedAnimationLinks` a
/// per-link identity to exclude for one agent without touching any other
/// link -- both unidirectional links of one logical portal get the *same*
/// `kind`, so a quarantine excludes the whole crossing in either direction.
pub(crate) fn spawn_link_pair(
    world: &mut World,
    archipelago_entity: Entity,
    start: Vec3,
    end: Vec3,
    cost: f32,
    kind: usize,
) -> [Entity; 2] {
    spawn_link_pair_with_destination(world, archipelago_entity, start, end, cost, kind, false)
}

/// Exterior-cell variant of [`spawn_link_pair`]. Both ends are already
/// guaranteed to be inside their selected post-clearance triangles; keeping a
/// finite destination portal makes landmass use its edge sampler for this
/// otherwise geometry-only cross-cell seam. The tiny segment remains local to
/// the endpoint and does not change the authored crossing.
pub(crate) fn spawn_exterior_link_pair(
    world: &mut World,
    archipelago_entity: Entity,
    start: Vec3,
    end: Vec3,
    cost: f32,
    kind: usize,
) -> [Entity; 2] {
    spawn_link_pair_with_destination(world, archipelago_entity, start, end, cost, kind, true)
}

pub(crate) fn spawn_link_pair_with_destination(
    world: &mut World,
    archipelago_entity: Entity,
    start: Vec3,
    end: Vec3,
    cost: f32,
    kind: usize,
    finite_destination: bool,
) -> [Entity; 2] {
    let mut spawn_one = |from: Vec3, to: Vec3| {
        world
            .spawn(AnimationLink3dBundle {
                link: AnimationLink3d {
                    start_edge: animation_link_start_edge(from, to),
                    end_edge: if finite_destination {
                        animation_link_end_edge(from, to)
                    } else {
                        (to, to)
                    },
                    kind,
                    cost,
                    bidirectional: false,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            })
            .id()
    };
    [spawn_one(start, end), spawn_one(end, start)]
}
