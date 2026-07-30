//! Deterministic reflection-probe placement over the post-clearance NAVM graph.
//!
//! This module is deliberately pure. The prepare adapter owns collision/material
//! capture and artifact IO; this policy only decides stable capture positions and
//! local cuboid influence regions.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::{PreparedNavGraph, allocate_probe_counts};

pub(crate) const REFLECTION_PROBE_EYE_HEIGHT: f32 = 1.65;
pub(crate) const REFLECTION_PROBE_SPACING: f32 = 12.0;
pub(crate) const REFLECTION_PROBE_MAX_COUNT: usize = 16;
pub(crate) const REFLECTION_PROBE_FALLOFF: f32 = 0.5;
const MIN_HORIZONTAL_HALF_EXTENT: f32 = 2.0;
const MAX_HORIZONTAL_HALF_EXTENT: f32 = 12.0;
const DEFAULT_VERTICAL_HALF_EXTENT: f32 = 2.5;
const MIN_ROOM_REGION_AREA: f32 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ReflectionProbeLayout {
    pub(crate) capture_translation: [f32; 3],
    pub(crate) influence_half_extents: [f32; 3],
    pub(crate) parallax_half_extents: [f32; 3],
    pub(crate) falloff: [f32; 3],
}

#[derive(Clone)]
struct Triangle {
    node: (usize, usize),
    vertices: [[f32; 3]; 3],
    centroid: [f32; 3],
    area: f32,
}

#[derive(Clone)]
struct Region {
    triangles: Vec<Triangle>,
    area: f32,
}

type NavTriangleMap = BTreeMap<(usize, usize), Triangle>;
type NavNeighborMap = BTreeMap<(usize, usize), BTreeSet<(usize, usize)>>;

/// Places at least one probe in each largest room-like NAVM region, then adds
/// farthest-point samples in large regions until the 12 m density target or
/// the per-cell cap is reached.
pub(crate) fn reflection_probe_layouts(graph: &PreparedNavGraph) -> Vec<ReflectionProbeLayout> {
    let (triangles, neighbors) = graph_triangles(graph);
    let mut regions = connected_regions(&triangles, &neighbors);
    regions.retain(|region| region.area >= MIN_ROOM_REGION_AREA);
    regions.sort_by(|a, b| {
        b.area
            .total_cmp(&a.area)
            .then_with(|| region_key(a).cmp(&region_key(b)))
    });

    let allocations = allocate_probe_counts(
        &regions.iter().map(|region| region.area).collect::<Vec<_>>(),
        REFLECTION_PROBE_SPACING,
        REFLECTION_PROBE_MAX_COUNT,
    );
    let mut selected = Vec::<(usize, usize)>::new();
    for (region_index, allocation) in allocations.iter().enumerate() {
        if *allocation > 0 {
            let first = most_central_triangle(&regions[region_index]);
            selected.push((region_index, first));
        }
    }

    loop {
        if selected.len() >= REFLECTION_PROBE_MAX_COUNT {
            break;
        }
        let mut best: Option<(f32, usize, usize)> = None;
        for (region_index, region) in regions.iter().enumerate() {
            let desired = allocations[region_index];
            let current = selected
                .iter()
                .filter(|(candidate_region, _)| *candidate_region == region_index)
                .count();
            if current >= desired {
                continue;
            }
            let existing = selected
                .iter()
                .filter(|(candidate_region, _)| *candidate_region == region_index)
                .map(|(_, triangle)| *triangle)
                .collect::<Vec<_>>();
            if let Some((triangle, distance_squared)) = farthest_triangle(region, &existing) {
                let candidate = (distance_squared, region_index, triangle);
                if best.is_none_or(|current| {
                    candidate.0 > current.0
                        || (candidate.0 == current.0
                            && (candidate.1, candidate.2) < (current.1, current.2))
                }) {
                    best = Some(candidate);
                }
            }
        }
        let Some((_, region_index, triangle)) = best else {
            break;
        };
        selected.push((region_index, triangle));
    }

    selected.sort_unstable();
    let mut layouts = Vec::with_capacity(selected.len());
    for (region_index, triangle_index) in &selected {
        let region = &regions[*region_index];
        let capture = add_eye_height(region.triangles[*triangle_index].centroid);
        let mut minimum = capture;
        let mut maximum = capture;
        for triangle in &region.triangles {
            let nearest = selected
                .iter()
                .filter(|(candidate_region, _)| candidate_region == region_index)
                .min_by(|(_, left), (_, right)| {
                    distance_squared(triangle.centroid, region.triangles[*left].centroid)
                        .total_cmp(&distance_squared(
                            triangle.centroid,
                            region.triangles[*right].centroid,
                        ))
                        .then_with(|| left.cmp(right))
                })
                .map(|(_, candidate)| *candidate);
            if nearest != Some(*triangle_index) {
                continue;
            }
            for vertex in triangle.vertices {
                for axis in [0, 2] {
                    minimum[axis] = minimum[axis].min(vertex[axis]);
                    maximum[axis] = maximum[axis].max(vertex[axis]);
                }
            }
        }
        let half_x = ((capture[0] - minimum[0]).max(maximum[0] - capture[0]) + 2.5)
            .clamp(MIN_HORIZONTAL_HALF_EXTENT, MAX_HORIZONTAL_HALF_EXTENT);
        let half_z = ((capture[2] - minimum[2]).max(maximum[2] - capture[2]) + 2.5)
            .clamp(MIN_HORIZONTAL_HALF_EXTENT, MAX_HORIZONTAL_HALF_EXTENT);
        let half_extents = [half_x, DEFAULT_VERTICAL_HALF_EXTENT, half_z];
        layouts.push(ReflectionProbeLayout {
            capture_translation: capture,
            influence_half_extents: half_extents,
            parallax_half_extents: half_extents,
            falloff: [REFLECTION_PROBE_FALLOFF; 3],
        });
    }
    layouts
}

fn graph_triangles(graph: &PreparedNavGraph) -> (NavTriangleMap, NavNeighborMap) {
    let mut triangles = BTreeMap::new();
    let mut neighbors = BTreeMap::<_, BTreeSet<_>>::new();
    let mut polygon_lookup = Vec::with_capacity(graph.meshes.len());
    let mut door_nodes = BTreeSet::new();

    for (mesh_index, mesh) in graph.meshes.iter().enumerate() {
        let lookup = mesh
            .polygons
            .iter()
            .enumerate()
            .map(|(slot, polygon)| (polygon.index, slot))
            .collect::<BTreeMap<_, _>>();
        for door in &mesh.doors {
            if let Some(slot) = lookup.get(&door.triangle_index) {
                door_nodes.insert((mesh_index, *slot));
            }
        }
        for door in &mesh.derived_doors {
            if let Some(slot) = lookup.get(&door.triangle_index) {
                door_nodes.insert((mesh_index, *slot));
            }
        }
        for (slot, polygon) in mesh.polygons.iter().enumerate() {
            if !polygon.walkable {
                continue;
            }
            let Some(vertices) = polygon_vertices(mesh, polygon.vertex_indices) else {
                continue;
            };
            let area = triangle_area_xz(vertices);
            if area <= 1.0e-4 {
                continue;
            }
            let node = (mesh_index, slot);
            triangles.insert(
                node,
                Triangle {
                    node,
                    vertices,
                    centroid: triangle_centroid(vertices),
                    area,
                },
            );
            neighbors.entry(node).or_default();
        }
        polygon_lookup.push(lookup);
    }

    for (node, triangle) in &triangles {
        let mesh = &graph.meshes[node.0];
        let polygon = &mesh.polygons[node.1];
        if door_nodes.contains(node) {
            continue;
        }
        for neighbor_index in polygon.adjacency.into_iter().flatten() {
            let Some(neighbor_slot) = polygon_lookup[node.0].get(&neighbor_index).copied() else {
                continue;
            };
            let neighbor = (node.0, neighbor_slot);
            if triangles.contains_key(&neighbor) && !door_nodes.contains(&neighbor) {
                neighbors.entry(*node).or_default().insert(neighbor);
                neighbors.entry(neighbor).or_default().insert(*node);
            }
        }
        let _ = triangle;
    }

    for merge in &graph.mesh_merges {
        let Some(mesh_a) = graph
            .meshes
            .iter()
            .position(|mesh| mesh.form_id == merge.mesh_a_form_id)
        else {
            continue;
        };
        let Some(mesh_b) = graph
            .meshes
            .iter()
            .position(|mesh| mesh.form_id == merge.mesh_b_form_id)
        else {
            continue;
        };
        let (Some(slot_a), Some(slot_b)) = (
            polygon_lookup[mesh_a].get(&merge.triangle_a).copied(),
            polygon_lookup[mesh_b].get(&merge.triangle_b).copied(),
        ) else {
            continue;
        };
        let a = (mesh_a, slot_a);
        let b = (mesh_b, slot_b);
        if triangles.contains_key(&a)
            && triangles.contains_key(&b)
            && !door_nodes.contains(&a)
            && !door_nodes.contains(&b)
        {
            neighbors.entry(a).or_default().insert(b);
            neighbors.entry(b).or_default().insert(a);
        }
    }
    (triangles, neighbors)
}

fn connected_regions(
    triangles: &BTreeMap<(usize, usize), Triangle>,
    neighbors: &BTreeMap<(usize, usize), BTreeSet<(usize, usize)>>,
) -> Vec<Region> {
    let mut remaining = triangles.keys().copied().collect::<BTreeSet<_>>();
    let mut regions = Vec::new();
    while let Some(start) = remaining.pop_first() {
        let mut queue = VecDeque::from([start]);
        let mut region = Vec::new();
        while let Some(node) = queue.pop_front() {
            let Some(triangle) = triangles.get(&node) else {
                continue;
            };
            region.push(triangle.clone());
            if let Some(adjacent) = neighbors.get(&node) {
                for neighbor in adjacent {
                    if remaining.remove(neighbor) {
                        queue.push_back(*neighbor);
                    }
                }
            }
        }
        region.sort_by_key(|triangle| triangle.node);
        let area = region.iter().map(|triangle| triangle.area).sum();
        regions.push(Region {
            triangles: region,
            area,
        });
    }
    regions
}

fn region_key(region: &Region) -> (usize, usize) {
    region
        .triangles
        .first()
        .map_or((usize::MAX, usize::MAX), |triangle| triangle.node)
}

fn most_central_triangle(region: &Region) -> usize {
    let weighted = region.triangles.iter().fold([0.0; 3], |mut sum, triangle| {
        for (axis, value) in sum.iter_mut().enumerate() {
            *value += triangle.centroid[axis] * triangle.area;
        }
        sum
    });
    let center = weighted.map(|value| value / region.area.max(f32::EPSILON));
    region
        .triangles
        .iter()
        .enumerate()
        .min_by(|(left_index, left), (right_index, right)| {
            distance_squared(left.centroid, center)
                .total_cmp(&distance_squared(right.centroid, center))
                .then_with(|| left_index.cmp(right_index))
        })
        .map_or(0, |(index, _)| index)
}

fn farthest_triangle(region: &Region, selected: &[usize]) -> Option<(usize, f32)> {
    region
        .triangles
        .iter()
        .enumerate()
        .filter(|(index, _)| !selected.contains(index))
        .map(|(index, triangle)| {
            let nearest = selected
                .iter()
                .map(|selected| {
                    distance_squared(triangle.centroid, region.triangles[*selected].centroid)
                })
                .fold(f32::INFINITY, f32::min);
            (index, nearest)
        })
        .max_by(|left, right| {
            left.1
                .total_cmp(&right.1)
                .then_with(|| right.0.cmp(&left.0))
        })
}

fn polygon_vertices(mesh: &super::PreparedNavMesh, indices: [u32; 3]) -> Option<[[f32; 3]; 3]> {
    Some([
        *mesh.vertices.get(indices[0] as usize)?,
        *mesh.vertices.get(indices[1] as usize)?,
        *mesh.vertices.get(indices[2] as usize)?,
    ])
}

fn triangle_centroid(vertices: [[f32; 3]; 3]) -> [f32; 3] {
    std::array::from_fn(|axis| (vertices[0][axis] + vertices[1][axis] + vertices[2][axis]) / 3.0)
}

fn triangle_area_xz(vertices: [[f32; 3]; 3]) -> f32 {
    let ab = [
        vertices[1][0] - vertices[0][0],
        vertices[1][2] - vertices[0][2],
    ];
    let ac = [
        vertices[2][0] - vertices[0][0],
        vertices[2][2] - vertices[0][2],
    ];
    (ab[0] * ac[1] - ab[1] * ac[0]).abs() * 0.5
}

fn add_eye_height(mut point: [f32; 3]) -> [f32; 3] {
    point[1] += REFLECTION_PROBE_EYE_HEIGHT;
    point
}

fn distance_squared(left: [f32; 3], right: [f32; 3]) -> f32 {
    (left[0] - right[0]).powi(2) + (left[2] - right[2]).powi(2)
}

#[cfg(test)]
#[path = "tests/reflection_probe_policy.rs"]
mod tests;
