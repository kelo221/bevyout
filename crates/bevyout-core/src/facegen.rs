//! Engine-independent Fallout 3 FaceGen coefficient contracts.
//!
//! The ESM `FGGS`, `FGGA`, and `FGTS` fields are opaque byte arrays at the
//! parser boundary.  This module owns their bounded, little-endian decoding
//! and the deterministic race-default plus actor-trait combination policy.
//! File-backed EGM/EGT morph data remains an application preparation concern.

use std::fmt;

use serde::{Deserialize, Serialize};

pub const GEOMETRY_SYMMETRIC_COEFFICIENTS: usize = 50;
pub const GEOMETRY_ASYMMETRIC_COEFFICIENTS: usize = 30;
pub const TEXTURE_SYMMETRIC_COEFFICIENTS: usize = 50;

pub const GEOMETRY_SYMMETRIC_BYTES: usize = GEOMETRY_SYMMETRIC_COEFFICIENTS * 4;
pub const GEOMETRY_ASYMMETRIC_BYTES: usize = GEOMETRY_ASYMMETRIC_COEFFICIENTS * 4;
pub const TEXTURE_SYMMETRIC_BYTES: usize = TEXTURE_SYMMETRIC_COEFFICIENTS * 4;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FaceGenComponent {
    GeometrySymmetric,
    GeometryAsymmetric,
    TextureSymmetric,
}

impl FaceGenComponent {
    #[must_use]
    pub const fn expected_coefficients(self) -> usize {
        match self {
            Self::GeometrySymmetric => GEOMETRY_SYMMETRIC_COEFFICIENTS,
            Self::GeometryAsymmetric => GEOMETRY_ASYMMETRIC_COEFFICIENTS,
            Self::TextureSymmetric => TEXTURE_SYMMETRIC_COEFFICIENTS,
        }
    }

    #[must_use]
    pub const fn expected_bytes(self) -> usize {
        self.expected_coefficients() * 4
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::GeometrySymmetric => "geometry_symmetric",
            Self::GeometryAsymmetric => "geometry_asymmetric",
            Self::TextureSymmetric => "texture_symmetric",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FaceGenAssetKind {
    GeometryMorph,
    TextureMorph,
    TriMorph,
}

impl FaceGenAssetKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::GeometryMorph => "geometry_morph",
            Self::TextureMorph => "texture_morph",
            Self::TriMorph => "tri_morph",
        }
    }
}

/// Raw bytes carried from an actor or race record without reinterpretation.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FaceGenRaw {
    pub geometry_symmetric: Option<Vec<u8>>,
    pub geometry_asymmetric: Option<Vec<u8>>,
    pub texture_symmetric: Option<Vec<u8>>,
}

impl FaceGenRaw {
    #[must_use]
    pub fn any_authored(&self) -> bool {
        self.geometry_symmetric.is_some()
            || self.geometry_asymmetric.is_some()
            || self.texture_symmetric.is_some()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.any_authored()
    }

    fn component(&self, component: FaceGenComponent) -> Option<&[u8]> {
        match component {
            FaceGenComponent::GeometrySymmetric => self.geometry_symmetric.as_deref(),
            FaceGenComponent::GeometryAsymmetric => self.geometry_asymmetric.as_deref(),
            FaceGenComponent::TextureSymmetric => self.texture_symmetric.as_deref(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FaceGenCoefficients {
    pub geometry_symmetric: Vec<f32>,
    pub geometry_asymmetric: Vec<f32>,
    pub texture_symmetric: Vec<f32>,
}

impl FaceGenCoefficients {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            geometry_symmetric: vec![0.0; GEOMETRY_SYMMETRIC_COEFFICIENTS],
            geometry_asymmetric: vec![0.0; GEOMETRY_ASYMMETRIC_COEFFICIENTS],
            texture_symmetric: vec![0.0; TEXTURE_SYMMETRIC_COEFFICIENTS],
        }
    }

    fn component_mut(&mut self, component: FaceGenComponent) -> &mut Vec<f32> {
        match component {
            FaceGenComponent::GeometrySymmetric => &mut self.geometry_symmetric,
            FaceGenComponent::GeometryAsymmetric => &mut self.geometry_asymmetric,
            FaceGenComponent::TextureSymmetric => &mut self.texture_symmetric,
        }
    }

    #[must_use]
    pub fn component(&self, component: FaceGenComponent) -> &[f32] {
        match component {
            FaceGenComponent::GeometrySymmetric => &self.geometry_symmetric,
            FaceGenComponent::GeometryAsymmetric => &self.geometry_asymmetric,
            FaceGenComponent::TextureSymmetric => &self.texture_symmetric,
        }
    }
}

/// Resolved FaceGen data retained in the actor catalog and assembly
/// descriptor.  Keeping both sources and the combined coefficients makes
/// template/race inheritance inspectable and makes cache identity include the
/// exact source bytes rather than only their floating-point sum.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FaceGenResolved {
    pub actor: FaceGenRaw,
    pub race: FaceGenRaw,
    pub coefficients: FaceGenCoefficients,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FaceGenDiagnostic {
    UnsupportedLength {
        component: FaceGenComponent,
        expected_bytes: usize,
        actual_bytes: usize,
    },
    NonFiniteCoefficient {
        component: FaceGenComponent,
        index: usize,
        value: f32,
    },
    NonFiniteCombinedCoefficient {
        component: FaceGenComponent,
        index: usize,
        value: f32,
    },
    MissingAsset {
        asset: FaceGenAssetKind,
        path: String,
    },
    UnsupportedAsset {
        asset: FaceGenAssetKind,
        reason: String,
    },
    GeometryVertexCountMismatch {
        expected: usize,
        actual: usize,
    },
    GeometryBaseVertexCountMismatch {
        expected: usize,
        actual: usize,
    },
    GeometryUvCountMismatch {
        expected: usize,
        actual: usize,
    },
    TextureDimensionsMismatch {
        expected_width: u32,
        expected_height: u32,
        actual_width: u32,
        actual_height: u32,
    },
}

impl FaceGenDiagnostic {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedLength { .. } => "unsupported_facegen_layout",
            Self::NonFiniteCoefficient { .. } => "nonfinite_facegen_coefficient",
            Self::NonFiniteCombinedCoefficient { .. } => "nonfinite_facegen_combined",
            Self::MissingAsset { .. } => "missing_facegen_asset",
            Self::UnsupportedAsset { .. } => "unsupported_facegen_asset",
            Self::GeometryVertexCountMismatch { .. } => "facegen_vertex_count_mismatch",
            Self::GeometryBaseVertexCountMismatch { .. } => "facegen_base_vertex_count_mismatch",
            Self::GeometryUvCountMismatch { .. } => "facegen_uv_count_mismatch",
            Self::TextureDimensionsMismatch { .. } => "facegen_texture_dimensions_mismatch",
        }
    }
}

impl fmt::Display for FaceGenDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLength {
                component,
                expected_bytes,
                actual_bytes,
            } => write!(
                formatter,
                "{} requires {} bytes, got {}",
                component.label(),
                expected_bytes,
                actual_bytes
            ),
            Self::NonFiniteCoefficient {
                component,
                index,
                value,
            } => write!(
                formatter,
                "{} coefficient {} is non-finite ({value:?})",
                component.label(),
                index
            ),
            Self::NonFiniteCombinedCoefficient {
                component,
                index,
                value,
            } => write!(
                formatter,
                "combined {} coefficient {} is non-finite ({value:?})",
                component.label(),
                index
            ),
            Self::MissingAsset { asset, path } => {
                write!(formatter, "{} asset is missing: {path}", asset.label())
            }
            Self::UnsupportedAsset { asset, reason } => {
                write!(
                    formatter,
                    "{} asset is unsupported: {reason}",
                    asset.label()
                )
            }
            Self::GeometryVertexCountMismatch { expected, actual } => write!(
                formatter,
                "FaceGen geometry has {actual} vertices but the head has {expected}"
            ),
            Self::GeometryBaseVertexCountMismatch { expected, actual } => write!(
                formatter,
                "FaceGen TRI base has {expected} vertices but the head has {actual}"
            ),
            Self::GeometryUvCountMismatch { expected, actual } => write!(
                formatter,
                "FaceGen TRI has {expected} UV entries but the head has {actual}"
            ),
            Self::TextureDimensionsMismatch {
                expected_width,
                expected_height,
                actual_width,
                actual_height,
            } => write!(
                formatter,
                "FaceGen texture is {actual_width}x{actual_height}, expected {expected_width}x{expected_height}"
            ),
        }
    }
}

/// Decode one canonical coefficient array.  Presence is strict: a component
/// must be exactly its documented little-endian finite-f32 byte layout.
pub fn decode_component(
    component: FaceGenComponent,
    bytes: &[u8],
) -> Result<Vec<f32>, FaceGenDiagnostic> {
    let expected_bytes = component.expected_bytes();
    if bytes.len() != expected_bytes {
        return Err(FaceGenDiagnostic::UnsupportedLength {
            component,
            expected_bytes,
            actual_bytes: bytes.len(),
        });
    }
    let mut coefficients = Vec::with_capacity(component.expected_coefficients());
    for index in 0..component.expected_coefficients() {
        let start = index * 4;
        let value = f32::from_le_bytes(bytes[start..start + 4].try_into().unwrap());
        if !value.is_finite() {
            return Err(FaceGenDiagnostic::NonFiniteCoefficient {
                component,
                index,
                value,
            });
        }
        coefficients.push(value);
    }
    Ok(coefficients)
}

fn decoded_or_zero(
    raw: &FaceGenRaw,
    component: FaceGenComponent,
) -> Result<Vec<f32>, FaceGenDiagnostic> {
    raw.component(component).map_or_else(
        || Ok(vec![0.0; component.expected_coefficients()]),
        |bytes| decode_component(component, bytes),
    )
}

/// Resolve one actor's authored values over the selected race's defaults.
/// Components absent from either source contribute zero; a wholly empty pair
/// returns `Ok(None)` so callers can distinguish NotAuthored from malformed
/// authored data.
pub fn resolve_facegen(
    actor: &FaceGenRaw,
    race: Option<&FaceGenRaw>,
) -> Result<Option<FaceGenResolved>, FaceGenDiagnostic> {
    let race = race.cloned().unwrap_or_default();
    if actor.is_empty() && race.is_empty() {
        return Ok(None);
    }

    let mut coefficients = FaceGenCoefficients::zero();
    for component in [
        FaceGenComponent::GeometrySymmetric,
        FaceGenComponent::GeometryAsymmetric,
        FaceGenComponent::TextureSymmetric,
    ] {
        let actor_values = decoded_or_zero(actor, component)?;
        let race_values = decoded_or_zero(&race, component)?;
        let combined = coefficients.component_mut(component);
        for (index, (actor_value, race_value)) in
            actor_values.into_iter().zip(race_values).enumerate()
        {
            let value = actor_value + race_value;
            if !value.is_finite() {
                return Err(FaceGenDiagnostic::NonFiniteCombinedCoefficient {
                    component,
                    index,
                    value,
                });
            }
            combined[index] = value;
        }
    }

    Ok(Some(FaceGenResolved {
        actor: actor.clone(),
        race,
        coefficients,
    }))
}

#[cfg(test)]
#[path = "tests/facegen.rs"]
mod tests;
