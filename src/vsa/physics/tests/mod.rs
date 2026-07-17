use super::*;

fn body(layer: u8, flags: u8, phantom: bool) -> PreparedPhysicsBody {
    PreparedPhysicsBody {
        layer,
        filter_flags: flags,
        phantom,
        shapes: vec![PreparedPhysicsShape::Sphere {
            center: [0.0; 3],
            radius: 1.0,
        }],
        ..Default::default()
    }
}

#[test]
fn non_collidable_query_and_character_layers_never_block_player() {
    for layer in [0, 8, 12, 15, 21, 29, 30, 33, 34, 35, 40, 43] {
        assert!(!body_blocks_player(&body(layer, 0, false)), "layer {layer}");
    }
    assert!(!body_blocks_player(&body(1, 0x40, false)));
    assert!(!body_blocks_player(&body(1, 0, true)));
    assert!(body_blocks_player(&body(1, 0, false)));
    assert!(body_blocks_player(&body(44, 0, false)));
}

#[test]
fn exact_sphere_and_capsule_geometry_validate_without_proxy_meshes() {
    let asset = PreparedPhysicsAsset {
        schema_version: PHYSICS_ASSET_SCHEMA_VERSION,
        source: PreparedPhysicsSource::AuthoredHavok,
        bodies: vec![PreparedPhysicsBody {
            shapes: vec![
                PreparedPhysicsShape::Sphere {
                    center: [1.0, 2.0, 3.0],
                    radius: 0.4,
                },
                PreparedPhysicsShape::Capsule {
                    point1: [0.0, -1.0, 0.0],
                    point2: [0.0, 1.0, 0.0],
                    radius: 0.2,
                },
            ],
            ..Default::default()
        }],
        joints: Vec::new(),
    };
    validate_physics_asset(&asset).unwrap();
    assert_eq!(asset.bodies[0].shapes[0].kind(), "sphere");
    assert_eq!(asset.bodies[0].shapes[1].kind(), "capsule");
}

#[test]
fn dynamic_scope_rejects_concave_and_constrained_bodies() {
    let mut dynamic = body(4, 0, false);
    dynamic.motion_type = "MO_SYS_DYNAMIC".into();
    dynamic.mass = 2.0;
    let mut asset = PreparedPhysicsAsset {
        schema_version: PHYSICS_ASSET_SCHEMA_VERSION,
        source: PreparedPhysicsSource::AuthoredHavok,
        bodies: vec![dynamic],
        joints: Vec::new(),
    };
    assert_eq!(
        classify_placement(&PreparedSemantic::Static, &asset),
        PreparedPhysicsClassification::Dynamic
    );
    asset.bodies[0].constrained = true;
    assert_eq!(
        classify_placement(&PreparedSemantic::Static, &asset),
        PreparedPhysicsClassification::Static
    );
    asset.bodies[0].constrained = false;
    asset.bodies[0].shapes = vec![PreparedPhysicsShape::TriangleMesh {
        vertices: vec![[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        indices: vec![0, 1, 2],
    }];
    assert_eq!(
        classify_placement(&PreparedSemantic::Static, &asset),
        PreparedPhysicsClassification::Static
    );
}
