use anyhow::{Context, Result, bail};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::Path;

use super::manifest::{PreparedPhysicsClassification, PreparedSemantic};

pub(crate) const PHYSICS_ASSET_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum PreparedPhysicsSource {
    AuthoredHavok,
    #[default]
    GeneratedRender,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedPhysicsAsset {
    pub(crate) schema_version: u32,
    pub(crate) source: PreparedPhysicsSource,
    pub(crate) bodies: Vec<PreparedPhysicsBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PreparedPhysicsBody {
    pub(crate) group_id: u32,
    pub(crate) motion_type: String,
    pub(crate) quality_type: String,
    pub(crate) mass: f32,
    pub(crate) center_of_mass: [f32; 3],
    pub(crate) inertia: [[f32; 3]; 3],
    pub(crate) linear_velocity: [f32; 3],
    pub(crate) angular_velocity: [f32; 3],
    pub(crate) gravity_factor: f32,
    pub(crate) linear_damping: f32,
    pub(crate) angular_damping: f32,
    pub(crate) friction: f32,
    pub(crate) restitution: f32,
    pub(crate) max_linear_velocity: f32,
    pub(crate) max_angular_velocity: f32,
    pub(crate) sleep_enabled: bool,
    pub(crate) ccd_enabled: bool,
    pub(crate) layer: u8,
    pub(crate) filter_flags: u8,
    pub(crate) material: Option<u32>,
    pub(crate) material_name: Option<String>,
    pub(crate) phantom: bool,
    pub(crate) constrained: bool,
    pub(crate) shapes: Vec<PreparedPhysicsShape>,
}

impl Default for PreparedPhysicsBody {
    fn default() -> Self {
        Self {
            group_id: 0,
            motion_type: "MO_SYS_FIXED".into(),
            quality_type: "MO_QUAL_FIXED".into(),
            mass: 0.0,
            center_of_mass: [0.0; 3],
            inertia: [[0.0; 3]; 3],
            linear_velocity: [0.0; 3],
            angular_velocity: [0.0; 3],
            gravity_factor: 1.0,
            linear_damping: 0.0,
            angular_damping: 0.0,
            friction: 0.8,
            restitution: 0.0,
            max_linear_velocity: 0.0,
            max_angular_velocity: 0.0,
            sleep_enabled: true,
            ccd_enabled: false,
            layer: 1,
            filter_flags: 0,
            material: None,
            material_name: None,
            phantom: false,
            constrained: false,
            shapes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub(crate) enum PreparedPhysicsShape {
    Box {
        center: [f32; 3],
        half_extents: [f32; 3],
        rotation_xyzw: [f32; 4],
    },
    Sphere {
        center: [f32; 3],
        radius: f32,
    },
    Capsule {
        point1: [f32; 3],
        point2: [f32; 3],
        radius: f32,
    },
    ConvexHull {
        points: Vec<[f32; 3]>,
    },
    TriangleMesh {
        vertices: Vec<[f32; 3]>,
        indices: Vec<u32>,
    },
}

impl PreparedPhysicsShape {
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Self::Box { .. } => "box",
            Self::Sphere { .. } => "sphere",
            Self::Capsule { .. } => "capsule",
            Self::ConvexHull { .. } => "convex_hull",
            Self::TriangleMesh { .. } => "triangle_mesh",
        }
    }

    pub(crate) fn triangle_count(&self) -> usize {
        match self {
            Self::TriangleMesh { indices, .. } => indices.len() / 3,
            _ => 0,
        }
    }

    pub(crate) fn supports_dynamic(&self) -> bool {
        !matches!(self, Self::TriangleMesh { .. })
    }

    fn usable(&self) -> bool {
        match self {
            Self::Box {
                center,
                half_extents,
                rotation_xyzw,
            } => {
                finite3(*center)
                    && finite4(*rotation_xyzw)
                    && half_extents
                        .iter()
                        .all(|value| value.is_finite() && *value > 0.0)
            }
            Self::Sphere { center, radius } => {
                finite3(*center) && radius.is_finite() && *radius > 0.0
            }
            Self::Capsule {
                point1,
                point2,
                radius,
            } => finite3(*point1) && finite3(*point2) && radius.is_finite() && *radius > 0.0,
            Self::ConvexHull { points } => points.len() >= 4 && points.iter().all(|p| finite3(*p)),
            Self::TriangleMesh { vertices, indices } => {
                vertices.len() >= 3
                    && indices.len() >= 3
                    && indices.len() % 3 == 0
                    && vertices.iter().all(|p| finite3(*p))
                    && indices
                        .iter()
                        .all(|index| (*index as usize) < vertices.len())
            }
        }
    }
}

fn finite3(values: [f32; 3]) -> bool {
    values.iter().all(|value| value.is_finite())
}

fn finite4(values: [f32; 4]) -> bool {
    values.iter().all(|value| value.is_finite())
}

pub(crate) fn physics_sidecar_name(glb_name: &str) -> String {
    glb_name.strip_suffix(".glb").map_or_else(
        || format!("{glb_name}.physics.json.gz"),
        |stem| format!("{stem}.physics.json.gz"),
    )
}

pub(crate) fn read_physics_asset(path: &Path) -> Result<PreparedPhysicsAsset> {
    let file = fs::File::open(path)
        .with_context(|| format!("physics sidecar does not exist: {}", path.display()))?;
    let mut decoder = GzDecoder::new(file);
    let mut bytes = Vec::new();
    decoder
        .read_to_end(&mut bytes)
        .with_context(|| format!("invalid gzip physics sidecar: {}", path.display()))?;
    let asset: PreparedPhysicsAsset = serde_json::from_slice(&bytes)
        .with_context(|| format!("invalid physics JSON: {}", path.display()))?;
    validate_physics_asset(&asset)?;
    Ok(asset)
}

pub(crate) fn validate_physics_asset(asset: &PreparedPhysicsAsset) -> Result<()> {
    if asset.schema_version != PHYSICS_ASSET_SCHEMA_VERSION {
        bail!(
            "unsupported physics sidecar schema {} (expected {})",
            asset.schema_version,
            PHYSICS_ASSET_SCHEMA_VERSION
        );
    }
    if asset
        .bodies
        .iter()
        .any(|body| body.shapes.is_empty() || body.shapes.iter().any(|shape| !shape.usable()))
    {
        bail!("physics sidecar contains an unusable body or shape");
    }
    Ok(())
}

pub(crate) fn body_blocks_player(body: &PreparedPhysicsBody) -> bool {
    if body.phantom || body.filter_flags & 0x40 != 0 {
        return false;
    }
    !matches!(
        body.layer,
        0 | 8 | 12 | 15 | 16 | 18 | 21..=25 | 29..=31 | 33..=40 | 43
    )
}

pub(crate) fn classify_placement(
    semantic: &PreparedSemantic,
    asset: &PreparedPhysicsAsset,
) -> PreparedPhysicsClassification {
    let blockers = asset
        .bodies
        .iter()
        .filter(|body| body_blocks_player(body))
        .collect::<Vec<_>>();
    let dynamic_semantic = matches!(
        semantic,
        PreparedSemantic::Pickup(_) | PreparedSemantic::Static | PreparedSemantic::Unsupported
    );
    if dynamic_semantic
        && blockers.len() == 1
        && is_dynamic_motion(&blockers[0].motion_type)
        && blockers[0].mass.is_finite()
        && blockers[0].mass > 0.0
        && !blockers[0].constrained
        && blockers[0]
            .shapes
            .iter()
            .all(PreparedPhysicsShape::supports_dynamic)
    {
        PreparedPhysicsClassification::Dynamic
    } else if blockers
        .iter()
        .any(|body| is_keyframed_motion(&body.motion_type))
    {
        PreparedPhysicsClassification::Kinematic
    } else {
        PreparedPhysicsClassification::Static
    }
}

pub(crate) fn dynamic_rejection_reason(
    semantic: &PreparedSemantic,
    asset: &PreparedPhysicsAsset,
) -> Option<&'static str> {
    if classify_placement(semantic, asset) == PreparedPhysicsClassification::Dynamic {
        return None;
    }
    let dynamic_bodies = asset
        .bodies
        .iter()
        .filter(|body| body_blocks_player(body) && is_dynamic_motion(&body.motion_type))
        .collect::<Vec<_>>();
    if dynamic_bodies.is_empty() {
        return None;
    }
    if !matches!(
        semantic,
        PreparedSemantic::Pickup(_) | PreparedSemantic::Static | PreparedSemantic::Unsupported
    ) {
        return Some("placement semantic is excluded from prop dynamics");
    }
    if dynamic_bodies.len() != 1 {
        return Some("authored collision has multiple dynamic bodies");
    }
    let body = dynamic_bodies[0];
    if body.constrained {
        return Some("authored dynamic body is constrained");
    }
    if !body.mass.is_finite() || body.mass <= 0.0 {
        return Some("authored dynamic body has invalid mass");
    }
    if body.shapes.iter().any(|shape| !shape.supports_dynamic()) {
        return Some("authored dynamic body contains concave triangle collision");
    }
    Some("authored dynamic body is outside the supported single-prop scope")
}

pub(crate) fn is_dynamic_motion(value: &str) -> bool {
    let value = value.to_ascii_uppercase();
    value.contains("DYNAMIC")
        || value.contains("SPHERE_INERTIA")
        || value.contains("BOX_INERTIA")
        || value.contains("THIN_BOX_INERTIA")
}

fn is_keyframed_motion(value: &str) -> bool {
    value.to_ascii_uppercase().contains("KEYFRAMED")
}

#[cfg(test)]
mod tests {
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
}
