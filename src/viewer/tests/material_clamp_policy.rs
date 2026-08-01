use super::*;

fn factors(metallic: f32, reflectance: f32, perceptual_roughness: f32) -> MaterialFactors {
    MaterialFactors {
        metallic,
        reflectance,
        perceptual_roughness,
    }
}

fn engaged_material(
    settings: &ClampSettings,
    current: MaterialFactors,
) -> (ClampBaseline, MaterialFactors) {
    let mut baseline = ClampBaseline::default();
    let target = decide(settings, &mut baseline, current);
    assert!(
        !baseline.is_clear(),
        "engaged clamps must snapshot a baseline"
    );
    (baseline, target)
}

#[test]
fn engaging_the_metallic_gate_snapshots_and_zeros() {
    let mut settings = ClampSettings::default();
    settings.set_metallic_enabled(false);
    let (baseline, target) = engaged_material(&settings, factors(0.75, 0.5, 0.25));
    assert_eq!(baseline.metallic(), Some(0.75));
    assert_eq!(target, factors(0.0, 0.5, 0.25));
}

#[test]
fn engaging_the_dielectric_gate_snapshots_and_zeros() {
    let mut settings = ClampSettings::default();
    settings.set_dielectric_enabled(false);
    let (baseline, target) = engaged_material(&settings, factors(0.75, 0.2, 0.25));
    assert_eq!(baseline.reflectance(), Some(0.2));
    assert_eq!(target, factors(0.75, 0.0, 0.25));
}

#[test]
fn disengaging_restores_the_snapshot_bit_exactly_and_forgets_it() {
    let mut settings = ClampSettings::default();
    let authored = factors(0.8, 0.35, 0.6);
    settings.set_metallic_enabled(false);
    let (mut baseline, clamped) = engaged_material(&settings, authored);
    assert_eq!(clamped.metallic, 0.0);

    settings.set_metallic_enabled(true);
    let restored = decide(&settings, &mut baseline, clamped);
    assert_eq!(restored, authored);
    assert!(
        baseline.is_clear(),
        "restore must consume the field snapshot"
    );
}

#[test]
fn redeciding_an_engaged_material_converges_instead_of_compounding() {
    let mut settings = ClampSettings::default();
    settings.set_roughness_scale(0.5);
    let mut baseline = ClampBaseline::default();
    let first = decide(&settings, &mut baseline, factors(0.0, 0.0, 0.75));
    assert_eq!(first.perceptual_roughness, 0.375);
    let second = decide(&settings, &mut baseline, first);
    assert_eq!(
        second.perceptual_roughness, 0.375,
        "targets come from the snapshot, never the clamped live value"
    );
}

#[test]
fn roughness_targets_clamp_to_one_without_losing_the_snapshot() {
    let mut settings = ClampSettings::default();
    settings.set_roughness_scale(2.0);
    let (mut baseline, target) = engaged_material(&settings, factors(0.0, 0.0, 0.75));
    assert_eq!(target.perceptual_roughness, 1.0);
    assert_eq!(baseline.perceptual_roughness(), Some(0.75));

    settings.set_roughness_scale(1.0);
    let restored = decide(&settings, &mut baseline, target);
    assert_eq!(restored.perceptual_roughness, 0.75);
}

#[test]
fn overlapping_engagements_capture_each_field_at_its_own_engage_time() {
    let authored = factors(0.9, 0.4, 0.5);
    let mut settings = ClampSettings::default();
    settings.set_metallic_enabled(false);
    let mut baseline = ClampBaseline::default();
    let clamped = decide(&settings, &mut baseline, authored);
    assert!(baseline.perceptual_roughness().is_none());
    assert_eq!(clamped, factors(0.0, 0.4, 0.5));

    // Roughness changes while only the metallic gate is engaged...
    let touched = factors(0.0, 0.4, 0.25);
    let after = decide(&settings, &mut baseline, touched);
    assert_eq!(after.perceptual_roughness, 0.25);

    // ...then the roughness clamp engages and must snapshot the NEW value,
    // exactly like the old per-gate baseline maps did.
    settings.set_roughness_scale(0.5);
    let rescaled = decide(&settings, &mut baseline, after);
    assert_eq!(rescaled.perceptual_roughness, 0.125);
    assert_eq!(baseline.perceptual_roughness(), Some(0.25));

    settings.set_metallic_enabled(true);
    settings.set_roughness_scale(1.0);
    let restored = decide(&settings, &mut baseline, rescaled);
    assert_eq!(restored, factors(0.9, 0.4, 0.25));
    assert!(baseline.is_clear());
}

#[test]
fn setting_the_same_value_does_not_bump_the_revision() {
    let mut settings = ClampSettings::default();
    assert_eq!(settings.revision(), 0);
    settings.set_metallic_enabled(true);
    settings.set_roughness_scale(1.0);
    assert_eq!(settings.revision(), 0);
    settings.set_roughness_scale(0.5);
    assert_eq!(settings.revision(), 1);
    settings.set_roughness_scale(1.0);
    assert_eq!(settings.revision(), 2);
}

#[test]
fn full_pass_bookkeeping_tracks_the_settings_revision() {
    let mut settings = ClampSettings::default();
    let mut store: ClampStore<u32> = ClampStore::default();
    assert!(!store.needs_full_pass(&settings));
    settings.set_metallic_enabled(false);
    assert!(store.needs_full_pass(&settings));
    store.mark_applied(&settings);
    assert!(!store.needs_full_pass(&settings));
}

#[test]
fn release_drops_the_materials_baseline_entry() {
    let mut settings = ClampSettings::default();
    settings.set_metallic_enabled(false);
    let mut store: ClampStore<u32> = ClampStore::default();
    for id in [1u32, 2] {
        let mut baseline = store.take(id);
        decide(&settings, &mut baseline, factors(0.5 * id as f32, 0.5, 0.5));
        store.record(id, baseline);
    }
    assert_eq!(store.baseline_count(), 2);
    store.release(1);
    assert_eq!(store.baseline_count(), 1);
    assert!(store.baseline(1).is_none());
}

#[test]
fn prune_disengaged_discards_absent_materials_disengaged_snapshots() {
    let mut settings = ClampSettings::default();
    settings.set_metallic_enabled(false);
    settings.set_roughness_scale(0.5);

    let mut store: ClampStore<u32> = ClampStore::default();
    // Material 2 left the store without its Removed event reaching the
    // system: the old take-all disengagement dropped such entries too.
    for id in [1u32, 2] {
        let mut baseline = store.take(id);
        decide(&settings, &mut baseline, factors(0.5, 0.5, 0.5));
        store.record(id, baseline);
    }

    settings.set_metallic_enabled(true);
    store.prune_disengaged(&settings);
    let kept = store.baseline(2).expect("engaged fields stay baselined");
    assert_eq!(kept.metallic(), None);
    assert_eq!(kept.perceptual_roughness(), Some(0.5));

    settings.set_roughness_scale(1.0);
    store.prune_disengaged(&settings);
    assert_eq!(store.baseline_count(), 0);
}
