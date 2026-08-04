use glam::{Mat3, Mat4, Quat, Vec3};

use nif::fo3::{Scene, SceneMesh};

// These values are in Fallout's NIF units.  The native GLB path uses 70 NIF
// units per metre, so the distance gate is about 86 mm and the clearance is
// about 1.4 mm.  The gate keeps detached hair strands from following a face
// morph while the clearance prevents a triangle edge from remaining coplanar
// with the deformed head.
const MAX_HEAD_HAIR_DISTANCE: f32 = 6.0;
const HAIR_HEAD_CLEARANCE: f32 = 0.10;

#[derive(Debug, Clone)]
pub(crate) struct NativeFaceGenHeadFit {
    samples: Vec<HeadFitSample>,
}

#[derive(Debug, Clone, Copy)]
struct HeadFitSample {
    rest_position: Vec3,
    displacement: Vec3,
}

/// Builds a point-displacement field in the assembled actor's skeleton space.
///
/// Head meshes retain their part-local skin inverse-bind matrices when they
/// are merged into the actor.  Applying the same matrices against the shared
/// skeleton here makes the fit use the exact space that the GLB skin shader
/// uses, rather than guessing from an unskinned head bounding box.
pub(crate) fn build_native_facegen_head_fit(
    actor: &Scene,
    rest_head: &Scene,
    deformed_head: &Scene,
) -> Option<NativeFaceGenHeadFit> {
    if rest_head.nodes.len() != deformed_head.nodes.len() {
        return None;
    }
    let actor_globals = scene_global_transforms(actor)?;
    let rest_positions = skinned_positions(rest_head, actor, &actor_globals)?;
    let deformed_positions = skinned_positions(deformed_head, actor, &actor_globals)?;
    if rest_positions.len() != deformed_positions.len() || rest_positions.is_empty() {
        return None;
    }

    let mut has_displacement = false;
    let samples = rest_positions
        .into_iter()
        .zip(deformed_positions)
        .filter_map(|(rest_position, deformed_position)| {
            let displacement = deformed_position - rest_position;
            if !rest_position.is_finite() || !displacement.is_finite() {
                return None;
            }
            has_displacement |= displacement.length_squared() > f32::EPSILON;
            Some(HeadFitSample {
                rest_position,
                displacement,
            })
        })
        .collect::<Vec<_>>();
    (has_displacement && !samples.is_empty()).then_some(NativeFaceGenHeadFit { samples })
}

/// Moves a hair part outward only where its unchanged rest-pose surface would
/// be reached by the deformed FaceGen head.  The operation is deterministic,
/// leaves hair topology/UVs/materials intact, and returns the moved vertex
/// count for focused tests and diagnostics.
pub(crate) fn fit_native_hair_to_facegen(
    actor: &Scene,
    hair: &mut Scene,
    attachment: &str,
    fit: &NativeFaceGenHeadFit,
) -> usize {
    let Some(actor_globals) = scene_global_transforms(actor) else {
        return 0;
    };
    let Some(attachment_index) = actor.nodes.iter().position(|node| {
        node.mesh.is_none() && scene_node_key(&node.name) == scene_node_key(attachment)
    }) else {
        return 0;
    };
    let Some(hair_globals) = scene_global_transforms(hair) else {
        return 0;
    };
    let attachment_global = actor_globals[attachment_index];
    let max_distance_squared = MAX_HEAD_HAIR_DISTANCE * MAX_HEAD_HAIR_DISTANCE;
    let mut moved_vertices = 0;

    for (node_index, node) in hair.nodes.iter_mut().enumerate() {
        if node.skin.is_some() {
            continue;
        }
        let Some(mesh) = node.mesh.as_mut() else {
            continue;
        };
        let hair_global = attachment_global * hair_globals[node_index];
        if !hair_global.is_finite() {
            continue;
        }
        let inverse_hair_global = hair_global.inverse();
        if !inverse_hair_global.is_finite() {
            continue;
        }

        let mut changed = false;
        for position in &mut mesh.positions {
            let hair_position = hair_global.transform_point3(Vec3::from_array(*position));
            let Some((sample, distance_squared)) = nearest_head_sample(fit, hair_position) else {
                continue;
            };
            if distance_squared > max_distance_squared {
                continue;
            }
            let from_head = hair_position - sample.rest_position;
            let distance = from_head.length();
            if distance <= f32::EPSILON {
                continue;
            }
            let outward = from_head / distance;
            let head_advance = sample.displacement.dot(outward);
            if !head_advance.is_finite() || head_advance <= 0.0 {
                continue;
            }
            let correction = outward * (head_advance + HAIR_HEAD_CLEARANCE);
            let local_correction = inverse_hair_global.transform_vector3(correction);
            if !local_correction.is_finite() {
                continue;
            }
            *position = (Vec3::from_array(*position) + local_correction).to_array();
            changed = true;
            moved_vertices += 1;
        }
        if changed {
            recompute_hair_basis(mesh);
        }
    }
    moved_vertices
}

fn nearest_head_sample(
    fit: &NativeFaceGenHeadFit,
    position: Vec3,
) -> Option<(&HeadFitSample, f32)> {
    fit.samples
        .iter()
        .map(|sample| (sample, (position - sample.rest_position).length_squared()))
        .min_by(|(_, left), (_, right)| left.total_cmp(right))
}

fn skinned_positions(scene: &Scene, actor: &Scene, actor_globals: &[Mat4]) -> Option<Vec<Vec3>> {
    let mut positions = Vec::new();
    for node in &scene.nodes {
        let Some(mesh) = node.mesh.as_ref() else {
            continue;
        };
        let Some(skin_index) = node.skin else {
            continue;
        };
        let skin = scene.skins.get(skin_index)?;
        let matrices = skin_matrices(scene, actor, actor_globals, skin)?;
        if mesh.joints.len() != mesh.positions.len() || mesh.weights.len() != mesh.positions.len() {
            return None;
        }
        for ((position, joints), weights) in
            mesh.positions.iter().zip(&mesh.joints).zip(&mesh.weights)
        {
            let mut transformed = Vec3::ZERO;
            let mut total_weight = 0.0;
            for (joint, weight) in joints.iter().zip(weights) {
                if !weight.is_finite() || *weight <= 0.0 {
                    continue;
                }
                let matrix = matrices.get(usize::from(*joint))?;
                transformed += matrix.transform_point3(Vec3::from_array(*position)) * *weight;
                total_weight += *weight;
            }
            if !total_weight.is_finite() || total_weight <= f32::EPSILON {
                return None;
            }
            positions.push(transformed / total_weight);
        }
    }
    (!positions.is_empty()).then_some(positions)
}

fn skin_matrices(
    part: &Scene,
    actor: &Scene,
    actor_globals: &[Mat4],
    skin: &nif::fo3::SceneSkin,
) -> Option<Vec<Mat4>> {
    skin.joints
        .iter()
        .enumerate()
        .map(|(index, &part_joint)| {
            let part_node = part.nodes.get(part_joint)?;
            let actor_joint = actor.nodes.iter().position(|node| {
                node.mesh.is_none() && scene_node_key(&node.name) == scene_node_key(&part_node.name)
            })?;
            let inverse_bind = Mat4::from_cols_array(skin.inverse_bind_matrices.get(index)?);
            let matrix = actor_globals.get(actor_joint).copied()? * inverse_bind;
            matrix.is_finite().then_some(matrix)
        })
        .collect()
}

fn scene_global_transforms(scene: &Scene) -> Option<Vec<Mat4>> {
    let mut parents = vec![None; scene.nodes.len()];
    for (parent, node) in scene.nodes.iter().enumerate() {
        for &child in &node.children {
            let slot = parents.get_mut(child)?;
            if slot.is_some() {
                return None;
            }
            *slot = Some(parent);
        }
    }
    let mut globals = vec![None; scene.nodes.len()];
    let mut visiting = vec![false; scene.nodes.len()];
    for node in 0..scene.nodes.len() {
        resolve_global_transform(scene, &parents, &mut globals, &mut visiting, node)?;
    }
    globals.into_iter().collect()
}

fn resolve_global_transform(
    scene: &Scene,
    parents: &[Option<usize>],
    globals: &mut [Option<Mat4>],
    visiting: &mut [bool],
    node: usize,
) -> Option<Mat4> {
    if let Some(global) = globals.get(node).copied().flatten() {
        return Some(global);
    }
    if *visiting.get(node)? {
        return None;
    }
    visiting[node] = true;
    let local = scene_transform(&scene.nodes.get(node)?.transform);
    let global = if let Some(parent) = parents.get(node).copied().flatten() {
        resolve_global_transform(scene, parents, globals, visiting, parent)? * local
    } else {
        local
    };
    visiting[node] = false;
    if !global.is_finite() {
        return None;
    }
    globals[node] = Some(global);
    Some(global)
}

fn scene_transform(transform: &nif::fo3::Transform) -> Mat4 {
    let rotation =
        Quat::from_mat3(&Mat3::from_cols_array(&transform.rotation).transpose()).normalize();
    Mat4::from_scale_rotation_translation(
        Vec3::splat(transform.scale),
        rotation,
        Vec3::from_array(transform.translation),
    )
}

fn scene_node_key(name: &str) -> String {
    let mut parts = name
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    let side = (parts.len() >= 3 && parts.first().is_some_and(|part| part.starts_with("bip")))
        .then(|| parts.pop_if(|part| part == "l" || part == "r"))
        .flatten();
    if let Some(side) = side {
        parts.insert(1, side);
    }
    parts.concat()
}

fn recompute_hair_basis(mesh: &mut SceneMesh) {
    let vertex_count = mesh.positions.len();
    if mesh.normals.len() != vertex_count
        || mesh.tangents.len() != vertex_count
        || mesh.tex_coords.len() != vertex_count
    {
        return;
    }
    let mut normals = vec![Vec3::ZERO; vertex_count];
    let mut tangents = vec![Vec3::ZERO; vertex_count];
    let mut bitangents = vec![Vec3::ZERO; vertex_count];
    for triangle in mesh.indices.chunks_exact(3) {
        let [i0, i1, i2] = [
            usize::from(triangle[0]),
            usize::from(triangle[1]),
            usize::from(triangle[2]),
        ];
        if i0 >= vertex_count || i1 >= vertex_count || i2 >= vertex_count {
            return;
        }
        let p0 = Vec3::from_array(mesh.positions[i0]);
        let edge1 = Vec3::from_array(mesh.positions[i1]) - p0;
        let edge2 = Vec3::from_array(mesh.positions[i2]) - p0;
        let face_normal = edge1.cross(edge2);
        for index in [i0, i1, i2] {
            normals[index] += face_normal;
        }
        let uv1 = mesh.tex_coords[i1];
        let uv0 = mesh.tex_coords[i0];
        let uv2 = mesh.tex_coords[i2];
        let duv1 = [uv1[0] - uv0[0], uv1[1] - uv0[1]];
        let duv2 = [uv2[0] - uv0[0], uv2[1] - uv0[1]];
        let determinant = duv1[0] * duv2[1] - duv2[0] * duv1[1];
        if determinant.abs() <= 1.0e-8 || !determinant.is_finite() {
            continue;
        }
        let inverse = 1.0 / determinant;
        let tangent = (edge1 * duv2[1] - edge2 * duv1[1]) * inverse;
        let bitangent = (edge2 * duv1[0] - edge1 * duv2[0]) * inverse;
        for index in [i0, i1, i2] {
            tangents[index] += tangent;
            bitangents[index] += bitangent;
        }
    }
    for index in 0..vertex_count {
        let normal = normals[index].normalize_or_zero();
        let tangent = (tangents[index] - normal * normal.dot(tangents[index])).normalize_or_zero();
        if normal.length_squared() <= f32::EPSILON || tangent.length_squared() <= f32::EPSILON {
            continue;
        }
        let handedness = if normal.cross(tangent).dot(bitangents[index]) < 0.0 {
            -1.0
        } else {
            1.0
        };
        mesh.normals[index] = normal.to_array();
        mesh.tangents[index] = [tangent.x, tangent.y, tangent.z, handedness];
    }
}
