use anyhow::{Context, Result, bail};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::Path;

use super::manifest::{PreparedPhysicsClassification, PreparedSemantic};

pub(crate) const PHYSICS_ASSET_SCHEMA_VERSION: u32 = 2;

pub(crate) use bevyout_core::manifest::PreparedPhysicsSource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedPhysicsAsset {
    pub(crate) schema_version: u32,
    pub(crate) source: PreparedPhysicsSource,
    pub(crate) bodies: Vec<PreparedPhysicsBody>,
    /// Articulated Havok relationships. Empty is valid for ordinary props;
    /// actor assemblies use this list when the debug ragdoll is enabled.
    #[serde(default)]
    pub(crate) joints: Vec<PreparedPhysicsJoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedPhysicsJoint {
    pub(crate) kind: String,
    pub(crate) body_a: u32,
    pub(crate) body_b: u32,
    pub(crate) anchor_a: [f32; 3],
    pub(crate) anchor_b: [f32; 3],
    pub(crate) axis_a: [f32; 3],
    pub(crate) axis_b: [f32; 3],
    pub(crate) lower_limit: Option<f32>,
    pub(crate) upper_limit: Option<f32>,
    pub(crate) cone_limit: Option<f32>,
    pub(crate) twist_limit: Option<f32>,
}

impl Default for PreparedPhysicsJoint {
    fn default() -> Self {
        Self {
            kind: "fixed".into(),
            body_a: 0,
            body_b: 0,
            anchor_a: [0.0; 3],
            anchor_b: [0.0; 3],
            axis_a: [0.0, 1.0, 0.0],
            axis_b: [0.0, 1.0, 0.0],
            lower_limit: None,
            upper_limit: None,
            cone_limit: None,
            twist_limit: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PreparedPhysicsBody {
    pub(crate) group_id: u32,
    /// GLB scene node that owns this body's collision objects (anim-v6+);
    /// keyframed bodies use it to follow their animated node at runtime.
    pub(crate) node: Option<String>,
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
            node: None,
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

/// Conservative axis-aligned proxy enclosing every prepared collision shape.
/// Rotated boxes use their circumscribed radius so the result never clips the
/// authored shape. Runtime drops use this when a concave/static sidecar cannot
/// safely become a dynamic body.
pub(crate) fn dynamic_proxy_bounds(asset: &PreparedPhysicsAsset) -> Option<([f32; 3], [f32; 3])> {
    let mut minimum = [f32::INFINITY; 3];
    let mut maximum = [f32::NEG_INFINITY; 3];
    let mut include = |point: [f32; 3]| {
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(point[axis]);
            maximum[axis] = maximum[axis].max(point[axis]);
        }
    };
    for shape in asset.bodies.iter().flat_map(|body| &body.shapes) {
        match shape {
            PreparedPhysicsShape::Box {
                center,
                half_extents,
                ..
            } => {
                let radius = half_extents
                    .iter()
                    .map(|value| value * value)
                    .sum::<f32>()
                    .sqrt();
                include([center[0] - radius, center[1] - radius, center[2] - radius]);
                include([center[0] + radius, center[1] + radius, center[2] + radius]);
            }
            PreparedPhysicsShape::Sphere { center, radius } => {
                include([center[0] - radius, center[1] - radius, center[2] - radius]);
                include([center[0] + radius, center[1] + radius, center[2] + radius]);
            }
            PreparedPhysicsShape::Capsule {
                point1,
                point2,
                radius,
            } => {
                for point in [point1, point2] {
                    include([point[0] - radius, point[1] - radius, point[2] - radius]);
                    include([point[0] + radius, point[1] + radius, point[2] + radius]);
                }
            }
            PreparedPhysicsShape::ConvexHull { points }
            | PreparedPhysicsShape::TriangleMesh {
                vertices: points, ..
            } => points.iter().copied().for_each(&mut include),
        }
    }
    if minimum
        .iter()
        .chain(maximum.iter())
        .any(|value| !value.is_finite())
    {
        return None;
    }
    let center = std::array::from_fn(|axis| (minimum[axis] + maximum[axis]) * 0.5);
    let half_extents =
        std::array::from_fn(|axis| ((maximum[axis] - minimum[axis]) * 0.5).max(0.05));
    Some((center, half_extents))
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
    let body_ids = asset
        .bodies
        .iter()
        .map(|body| body.group_id)
        .collect::<std::collections::HashSet<_>>();
    for joint in &asset.joints {
        if !body_ids.contains(&joint.body_a) || !body_ids.contains(&joint.body_b) {
            bail!(
                "physics sidecar joint references missing bodies {} and {}",
                joint.body_a,
                joint.body_b
            );
        }
        if joint.body_a == joint.body_b {
            bail!("physics sidecar joint connects a body to itself");
        }
        if [joint.anchor_a, joint.anchor_b, joint.axis_a, joint.axis_b]
            .iter()
            .flatten()
            .any(|value| !value.is_finite())
            || [
                joint.lower_limit,
                joint.upper_limit,
                joint.cone_limit,
                joint.twist_limit,
            ]
            .iter()
            .flatten()
            .any(|value| !value.is_finite())
        {
            bail!("physics sidecar joint contains non-finite data");
        }
        if joint
            .lower_limit
            .zip(joint.upper_limit)
            .is_some_and(|(lower, upper)| lower > upper)
        {
            bail!("physics sidecar joint has inverted limits");
        }
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
#[path = "physics/tests/mod.rs"]
mod tests;
