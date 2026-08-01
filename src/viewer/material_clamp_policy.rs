//! Pure clamp policy for the viewer's `StandardMaterial` metallic /
//! dielectric-specular / roughness runtime clamps (issue #269, PERF wave 1).
//!
//! Consolidates the three independent baseline maps that
//! `apply_metallic_gate` / `apply_dielectric_specular_gate` /
//! `apply_roughness_scale` used to own into one settings value and one
//! baseline store, so a single system can apply every engaged clamp per
//! material in one mutation pass, and idle frames consume
//! `AssetEvent<StandardMaterial>` only.
//!
//! Kept free of Bevy imports so the executable-specification suite in
//! `tests/features.rs` can include this file verbatim (the same pattern as
//! `realtime_shadow_policy.rs` / `ao_policy.rs`). Keys are generic: the
//! runtime adapter in `viewer::controls` uses `AssetId<StandardMaterial>`,
//! the spec drives plain integers.

use std::collections::HashMap;
use std::hash::Hash;

/// Roughness multiplier that leaves materials untouched.
pub(crate) const IDENTITY_ROUGHNESS_SCALE: f32 = 1.0;

/// Combined settings for the three material clamps, owning the revision
/// counter the runtime schedules full re-application passes off.
///
/// `set_*` bumps `revision` only when the value actually changes: a console
/// `setrender` that re-asserts the current value costs nothing, while a
/// real change is applied to the whole asset store exactly once. Keeping
/// three separate `Res<_>` change ticks was not an option -- the point of
/// #269 is a single policy owner, and one mergeable counter also frees
/// console callers from touching Bevy change detection deliberately.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ClampSettings {
    metallic_enabled: bool,
    dielectric_enabled: bool,
    roughness_scale: f32,
    revision: u64,
}

impl Default for ClampSettings {
    fn default() -> Self {
        Self {
            metallic_enabled: true,
            dielectric_enabled: true,
            roughness_scale: IDENTITY_ROUGHNESS_SCALE,
            revision: 0,
        }
    }
}

impl ClampSettings {
    pub(crate) fn revision(&self) -> u64 {
        self.revision
    }

    pub(crate) fn metallic_enabled(&self) -> bool {
        self.metallic_enabled
    }

    pub(crate) fn dielectric_enabled(&self) -> bool {
        self.dielectric_enabled
    }

    pub(crate) fn roughness_scale(&self) -> f32 {
        self.roughness_scale
    }

    pub(crate) fn set_metallic_enabled(&mut self, enabled: bool) {
        if enabled != self.metallic_enabled {
            self.metallic_enabled = enabled;
            self.bump();
        }
    }

    pub(crate) fn set_dielectric_enabled(&mut self, enabled: bool) {
        if enabled != self.dielectric_enabled {
            self.dielectric_enabled = enabled;
            self.bump();
        }
    }

    pub(crate) fn set_roughness_scale(&mut self, scale: f32) {
        if scale != self.roughness_scale {
            self.roughness_scale = scale;
            self.bump();
        }
    }

    fn bump(&mut self) {
        self.revision += 1;
    }

    /// Engaged means the clamp actively rewrites materials right now.
    /// `setrender metallic 0` engages the metallic gate, `setrender
    /// roughness_scale 1` disengages the roughness clamp, matching the
    /// pre-#269 gate semantics.
    pub(crate) fn metallic_engaged(&self) -> bool {
        !self.metallic_enabled
    }

    pub(crate) fn dielectric_engaged(&self) -> bool {
        !self.dielectric_enabled
    }

    pub(crate) fn roughness_engaged(&self) -> bool {
        self.roughness_scale != IDENTITY_ROUGHNESS_SCALE
    }

    pub(crate) fn any_engaged(&self) -> bool {
        self.metallic_engaged() || self.dielectric_engaged() || self.roughness_engaged()
    }
}

/// The three material factors the clamp systems are allowed to rewrite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MaterialFactors {
    pub(crate) metallic: f32,
    pub(crate) reflectance: f32,
    pub(crate) perceptual_roughness: f32,
}

/// Per-material snapshot of clamped factors.
///
/// Each field is captured at that field's own engage time (never earlier),
/// so disengaging restores bit-exact authored values no matter how
/// engagements overlap: engaging metallic and later engaging roughness must
/// reproduce the old three-map behavior where each gate snapshotted its own
/// factor on its own first engaged frame.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ClampBaseline {
    metallic: Option<f32>,
    reflectance: Option<f32>,
    perceptual_roughness: Option<f32>,
}

impl ClampBaseline {
    /// True when no field holds a snapshot; such an entry is never stored.
    pub(crate) fn is_clear(&self) -> bool {
        self.metallic.is_none() && self.reflectance.is_none() && self.perceptual_roughness.is_none()
    }

    // The three field getters complete the entry's observer surface for
    // the pure unit spec; the runtime adapter only needs `is_clear`
    // (they are `#[allow]`-ed here, used from `tests/material_clamp_policy.rs`).
    #[allow(dead_code)]
    pub(crate) fn metallic(&self) -> Option<f32> {
        self.metallic
    }

    #[allow(dead_code)]
    pub(crate) fn reflectance(&self) -> Option<f32> {
        self.reflectance
    }

    #[allow(dead_code)]
    pub(crate) fn perceptual_roughness(&self) -> Option<f32> {
        self.perceptual_roughness
    }
}

/// Decide one material's factors under `settings`.
///
/// `baseline` is the material's entry in the clamp store (or a fresh,
/// clear entry for a material seen for the first time). For every engaged
/// field the pre-clamp value is snapshoted on first contact and the
/// clamped target is computed from the snapshot (never from the live,
/// already-clamped value, so repeated engaged frames and live re-touches
/// converge instead of compounding). For every disengaged field any stored
/// snapshot is consumed and the exact snapshot value becomes the target.
///
/// The returned factors are the material's target state; the runtime
/// writes only differing fields so untouched materials emit no
/// `AssetEvent::Modified`.
pub(crate) fn decide(
    settings: &ClampSettings,
    baseline: &mut ClampBaseline,
    current: MaterialFactors,
) -> MaterialFactors {
    let mut target = current;
    decide_field(
        settings.metallic_engaged(),
        &mut baseline.metallic,
        &mut target.metallic,
        |_| 0.0,
    );
    decide_field(
        settings.dielectric_engaged(),
        &mut baseline.reflectance,
        &mut target.reflectance,
        |_| 0.0,
    );
    decide_field(
        settings.roughness_engaged(),
        &mut baseline.perceptual_roughness,
        &mut target.perceptual_roughness,
        |base| (base * settings.roughness_scale()).clamp(0.0, 1.0),
    );
    target
}

/// One clamped field: capture-then-clamp while engaged, restore-and-forget
/// while disengaged.
fn decide_field(
    engaged: bool,
    baseline: &mut Option<f32>,
    target: &mut f32,
    clamp: impl FnOnce(f32) -> f32,
) {
    if engaged {
        let base = baseline.get_or_insert(*target);
        *target = clamp(*base);
    } else if let Some(restored) = baseline.take() {
        *target = restored;
    }
}

/// The one baseline authority behind `apply_material_clamps`, plus the
/// settings revision the asset store was last fully applied against.
///
/// Invariants:
///
/// - an entry exists only while at least one field of it holds a snapshot
///   (`record` never stores clear entries);
/// - a field snapshot exists only while that field is engaged (full passes
///   consume snapshots of disengaged fields, including for asset ids that
///   vanished without a `Removed` event, via [`Self::prune_disengaged`]).
#[derive(Clone, Debug)]
pub(crate) struct ClampStore<K>
where
    K: Eq + Hash + Copy,
{
    applied_revision: u64,
    baselines: HashMap<K, ClampBaseline>,
}

impl<K> Default for ClampStore<K>
where
    K: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self {
            applied_revision: 0,
            baselines: HashMap::new(),
        }
    }
}

impl<K> ClampStore<K>
where
    K: Eq + Hash + Copy,
{
    /// `settings` moved past the last full application: every material must
    /// be re-decided once. Incremental `Added`/`Modified` processing is
    /// subsumed by that pass and skipped while this holds.
    pub(crate) fn needs_full_pass(&self, settings: &ClampSettings) -> bool {
        settings.revision() != self.applied_revision
    }

    /// Called once the full pass finished deciding every present material.
    pub(crate) fn mark_applied(&mut self, settings: &ClampSettings) {
        self.applied_revision = settings.revision();
    }

    /// `AssetEvent::Removed` hygiene: drop the material's baseline entry so
    /// the store cannot grow stale entries for dead assets.
    pub(crate) fn release(&mut self, id: K) {
        self.baselines.remove(&id);
    }

    /// Take `id`'s entry out for a [`decide`] call; re-store the result
    /// with [`Self::record`].
    pub(crate) fn take(&mut self, id: K) -> ClampBaseline {
        self.baselines.remove(&id).unwrap_or_default()
    }

    /// Store a decided entry back, keeping the store free of clear entries.
    pub(crate) fn record(&mut self, id: K, baseline: ClampBaseline) {
        if !baseline.is_clear() {
            self.baselines.insert(id, baseline);
        }
    }

    /// Discard snapshots of fields `settings` no longer engages. Full
    /// passes already restore-and-forget disengaged fields for present
    /// materials; this covers asset ids that left the store without a
    /// `Removed` event reaching the system (the pre-#269 gates' take-all
    /// disengagement cleared those too).
    pub(crate) fn prune_disengaged(&mut self, settings: &ClampSettings) {
        if !settings.any_engaged() {
            self.baselines.clear();
            return;
        }
        let metallic_engaged = settings.metallic_engaged();
        let dielectric_engaged = settings.dielectric_engaged();
        let roughness_engaged = settings.roughness_engaged();
        self.baselines.retain(|_, baseline| {
            if !metallic_engaged {
                baseline.metallic = None;
            }
            if !dielectric_engaged {
                baseline.reflectance = None;
            }
            if !roughness_engaged {
                baseline.perceptual_roughness = None;
            }
            !baseline.is_clear()
        });
    }

    // Store observers for the executable spec / pure unit spec; the
    // runtime adapter drives `release`/`take`/`record`/`prune_disengaged`
    // and never asks these questions.
    #[allow(dead_code)]
    pub(crate) fn baseline_count(&self) -> usize {
        self.baselines.len()
    }

    #[allow(dead_code)]
    pub(crate) fn baseline(&self, id: K) -> Option<&ClampBaseline> {
        self.baselines.get(&id)
    }
}

#[cfg(test)]
#[path = "tests/material_clamp_policy.rs"]
mod tests;
