use super::*;
use crate::vsa::prepare::{PreparedNavMesh, PreparedNavPolygon};

fn graph(width: f32, depth: f32) -> PreparedNavGraph {
    PreparedNavGraph {
        meshes: vec![PreparedNavMesh {
            form_id: 1,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [width, 0.0, 0.0],
                [width, 0.0, depth],
                [0.0, 0.0, depth],
            ],
            polygons: vec![
                PreparedNavPolygon {
                    index: 0,
                    vertex_indices: [0, 1, 2],
                    adjacency: [None, None, Some(1)],
                    ..Default::default()
                },
                PreparedNavPolygon {
                    index: 1,
                    vertex_indices: [0, 2, 3],
                    adjacency: [Some(0), None, None],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[test]
fn small_room_gets_one_eye_height_probe() {
    let layouts = reflection_probe_layouts(&graph(4.0, 4.0));
    assert_eq!(layouts.len(), 1);
    assert!((layouts[0].capture_translation[1] - 1.65).abs() < 1.0e-6);
    assert!(layouts[0].capture_translation[0] > 0.0);
    assert!(layouts[0].capture_translation[2] > 0.0);
}

#[test]
fn output_never_exceeds_cell_cap() {
    let mut graph = graph(100.0, 100.0);
    let mesh = &mut graph.meshes[0];
    mesh.vertices.clear();
    mesh.polygons.clear();
    for index in 0..64_u32 {
        let x = (index % 8) as f32 * 13.0;
        let z = (index / 8) as f32 * 13.0;
        let base = mesh.vertices.len() as u32;
        mesh.vertices
            .extend([[x, 0.0, z], [x + 10.0, 0.0, z], [x, 0.0, z + 10.0]]);
        mesh.polygons.push(PreparedNavPolygon {
            index,
            vertex_indices: [base, base + 1, base + 2],
            ..Default::default()
        });
    }
    assert_eq!(
        reflection_probe_layouts(&graph).len(),
        REFLECTION_PROBE_MAX_COUNT
    );
}
