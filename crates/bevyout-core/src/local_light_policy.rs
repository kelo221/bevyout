//! Deterministic presentation policy for streamed exterior local lights.
//!
//! A prepared package is the authority for a light's cell ownership.  The
//! policy keeps that owner on every candidate and never infers ownership from
//! the light position, so a light on a cell border cannot move between cells.
//! Ranking is presentation-only: it does not perform visibility, occlusion,
//! collision, gameplay, or runtime entity decisions.

use std::cmp::Ordering;

use crate::manifest::exterior::{ExteriorCellPackage, GridCoordinate, PreparedExteriorLight};

/// Current runtime default. Callers may pass a smaller or zero budget for a
/// specific presentation policy test or quality setting.
pub const DEFAULT_EXTERIOR_LOCAL_LIGHT_BUDGET: usize = 64;

/// Stable identity of the streamed cell that owns a prepared local light.
///
/// Grid coordinates alone are not globally unique across worldspaces, and a
/// cell FormID alone does not preserve its streaming-grid boundary. Keeping
/// all three values makes the ownership contract explicit for later runtime
/// teardown without changing the prepared light schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExteriorLocalLightOwner {
    pub worldspace_form_id: u32,
    pub cell_form_id: u32,
    pub grid: GridCoordinate,
}

impl ExteriorLocalLightOwner {
    pub const fn new(worldspace_form_id: u32, cell_form_id: u32, grid: GridCoordinate) -> Self {
        Self {
            worldspace_form_id,
            cell_form_id,
            grid,
        }
    }

    pub fn from_package(package: &ExteriorCellPackage) -> Self {
        Self::new(
            package.worldspace_form_id,
            package.cell_form_id,
            package.grid,
        )
    }

    /// Zero worldspace or cell identities cannot own runtime presentation
    /// state. Negative grid coordinates are valid exterior cells.
    pub const fn is_valid(self) -> bool {
        self.worldspace_form_id != 0 && self.cell_form_id != 0
    }
}

/// A prepared light plus the authoritative streamed-cell owner supplied by
/// its package.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExteriorLocalLightCandidate<'a> {
    pub owner: ExteriorLocalLightOwner,
    pub light: &'a PreparedExteriorLight,
}

impl<'a> ExteriorLocalLightCandidate<'a> {
    pub const fn new(owner: ExteriorLocalLightOwner, light: &'a PreparedExteriorLight) -> Self {
        Self { owner, light }
    }

    /// A candidate is selectable only when its identity and presentation
    /// inputs are usable. Finite HDR color values are allowed; the policy does
    /// not clamp authored color values. A local light needs a strictly
    /// positive finite range to have a meaningful presentation extent.
    pub fn is_valid(self) -> bool {
        self.owner.is_valid()
            && self.light.reference_form_id != 0
            && finite_values(&self.light.position)
            && finite_values(&self.light.color_rgba)
            && self.light.range.is_finite()
            && self.light.range > 0.0
    }
}

/// One valid candidate in deterministic nearest-first order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RankedExteriorLocalLight<'a> {
    pub candidate: ExteriorLocalLightCandidate<'a>,
    /// Squared world-space distance. Squaring avoids a square root while
    /// preserving nearest-first ordering.
    pub distance_squared: f64,
}

/// Build owner-tagged candidates directly from the package boundary.
pub fn candidates_for_package<'a>(
    package: &'a ExteriorCellPackage,
) -> impl Iterator<Item = ExteriorLocalLightCandidate<'a>> + 'a {
    let owner = ExteriorLocalLightOwner::from_package(package);
    package
        .local_lights
        .iter()
        .map(move |light| ExteriorLocalLightCandidate::new(owner, light))
}

/// Rank valid local-light candidates by stable distance, FormID, and owner.
///
/// Non-finite observer coordinates and invalid candidates produce no ranked
/// entries. The owner and prepared light payload are retained in each result;
/// no candidate is reassigned based on its position.
pub fn rank_exterior_local_lights<'a, I>(
    observer_position: [f32; 3],
    candidates: I,
) -> Vec<RankedExteriorLocalLight<'a>>
where
    I: IntoIterator<Item = ExteriorLocalLightCandidate<'a>>,
{
    if !finite_values(&observer_position) {
        return Vec::new();
    }

    let mut ranked = candidates
        .into_iter()
        .filter_map(|candidate| {
            let distance_squared = distance_squared(observer_position, candidate)?;
            Some(RankedExteriorLocalLight {
                candidate,
                distance_squared,
            })
        })
        .collect::<Vec<_>>();
    ranked.sort_by(compare_ranked_lights);
    ranked
}

/// Select at most `max_active` valid candidates from the global streamed set.
/// A zero budget always returns an empty selection, and a budget larger than
/// the valid input keeps every valid candidate.
pub fn select_exterior_local_lights<'a, I>(
    observer_position: [f32; 3],
    max_active: usize,
    candidates: I,
) -> Vec<RankedExteriorLocalLight<'a>>
where
    I: IntoIterator<Item = ExteriorLocalLightCandidate<'a>>,
{
    if max_active == 0 {
        return Vec::new();
    }

    let mut ranked = rank_exterior_local_lights(observer_position, candidates);
    ranked.truncate(max_active);
    ranked
}

/// Select at most `max_active` candidates for one exact streamed-cell owner.
/// A candidate from another cell cannot consume this cell's budget, even when
/// it is closer to the observer.
pub fn select_exterior_local_lights_for_owner<'a, I>(
    owner: ExteriorLocalLightOwner,
    observer_position: [f32; 3],
    max_active: usize,
    candidates: I,
) -> Vec<RankedExteriorLocalLight<'a>>
where
    I: IntoIterator<Item = ExteriorLocalLightCandidate<'a>>,
{
    if !owner.is_valid() {
        return Vec::new();
    }

    select_exterior_local_lights(
        observer_position,
        max_active,
        candidates
            .into_iter()
            .filter(move |candidate| candidate.owner == owner),
    )
}

fn distance_squared(
    observer_position: [f32; 3],
    candidate: ExteriorLocalLightCandidate<'_>,
) -> Option<f64> {
    if !candidate.is_valid() {
        return None;
    }

    let distance_squared = candidate
        .light
        .position
        .into_iter()
        .zip(observer_position)
        .map(|(position, observer)| {
            let delta = f64::from(position) - f64::from(observer);
            delta * delta
        })
        .sum::<f64>();
    distance_squared.is_finite().then_some(distance_squared)
}

fn compare_ranked_lights(
    left: &RankedExteriorLocalLight<'_>,
    right: &RankedExteriorLocalLight<'_>,
) -> Ordering {
    left.distance_squared
        .total_cmp(&right.distance_squared)
        .then_with(|| {
            left.candidate
                .light
                .reference_form_id
                .cmp(&right.candidate.light.reference_form_id)
        })
        .then_with(|| left.candidate.owner.cmp(&right.candidate.owner))
        // Duplicate reference IDs under one owner are malformed but still
        // receive a deterministic order from the prepared presentation data.
        .then_with(|| {
            compare_float_arrays(
                left.candidate.light.position,
                right.candidate.light.position,
            )
        })
        .then_with(|| {
            left.candidate
                .light
                .range
                .total_cmp(&right.candidate.light.range)
        })
        .then_with(|| {
            compare_float_arrays(
                left.candidate.light.color_rgba,
                right.candidate.light.color_rgba,
            )
        })
}

fn compare_float_arrays<const N: usize>(left: [f32; N], right: [f32; N]) -> Ordering {
    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.total_cmp(&right))
        .find(|ordering| *ordering != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

fn finite_values<const N: usize>(values: &[f32; N]) -> bool {
    values.iter().all(|value| value.is_finite())
}

#[cfg(test)]
#[path = "tests/local_light_policy.rs"]
mod tests;
