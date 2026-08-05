use super::super::rust_scene::TransportMaterial;
use super::super::transport::{
    adaptive::AdaptiveEstimator,
    material::sample_material,
    sampling::{cosine_hemisphere_direction, sample_seed, seed_from_fingerprint},
};
use bevy::math::{Vec2, Vec3, Vec4};

#[test]
fn sample_seed_is_stable_and_partitioned_by_spatial_sample_inputs() {
    let scene_seed = seed_from_fingerprint("scene-fingerprint");
    assert_eq!(sample_seed(scene_seed, 3, 7), sample_seed(scene_seed, 3, 7));
    assert_ne!(sample_seed(scene_seed, 3, 7), sample_seed(scene_seed, 4, 7));
    assert_ne!(sample_seed(scene_seed, 3, 7), sample_seed(scene_seed, 3, 8));
}

#[test]
fn cosine_sampling_is_deterministic_and_stays_in_the_hemisphere() {
    let first = (0..64)
        .map(|sample| cosine_hemisphere_direction(Vec3::Y, sample_seed(17, 12, sample), sample, 64))
        .collect::<Vec<_>>();
    let second = (0..64)
        .map(|sample| cosine_hemisphere_direction(Vec3::Y, sample_seed(17, 12, sample), sample, 64))
        .collect::<Vec<_>>();
    assert_eq!(first, second);
    assert!(first.iter().all(|direction| direction.dot(Vec3::Y) >= 0.0));
}

#[test]
fn material_sampling_applies_linear_diffuse_and_emissive_contracts_once() {
    let material = TransportMaterial {
        base_color_factor: Vec4::new(0.5, 0.5, 0.5, 1.0),
        metallic_factor: 0.0,
        emissive_factor: Vec3::splat(0.5),
        ..Default::default()
    };
    let sample = sample_material(&material, Vec2::ZERO, Vec4::ONE);
    assert!((sample.alpha - 1.0).abs() < f32::EPSILON);
    assert!(sample.base_color.x > 0.2 && sample.base_color.x < 0.22);
    assert!(sample.emissive.x > 0.002 && sample.emissive.x < 0.003);
}

#[test]
fn adaptive_estimator_uses_a_stable_centered_variance_estimate() {
    let mut estimator = AdaptiveEstimator::new(2, 16, 0.0);
    estimator.add([1_000_000.0, 1_000_000.0, 1_000_000.0]);
    estimator.add([1_000_001.0, 1_000_001.0, 1_000_001.0]);
    estimator.add([999_999.0, 999_999.0, 999_999.0]);

    assert_eq!(estimator.sample_count(), 3);
    assert_eq!(estimator.mean(), [1_000_000.0; 3]);
    for variance in estimator.variance() {
        assert!((variance - 1.0).abs() < 1.0e-12);
    }
    for second_moment in estimator.second_moment() {
        assert!((second_moment - 1_000_000_000_000.666_6).abs() < 0.25);
    }
}

#[test]
fn adaptive_policy_honors_minimum_samples_before_converging() {
    let mut estimator = AdaptiveEstimator::new(4, 16, 0.0);

    for sample_count in 1..=3 {
        estimator.add([2.0, 2.0, 2.0]);
        assert_eq!(estimator.sample_count(), sample_count);
        assert!(!estimator.should_stop());
    }

    estimator.add([2.0, 2.0, 2.0]);
    assert!(estimator.should_stop());
}

#[test]
fn adaptive_policy_continues_for_noisy_samples_and_stops_at_maximum() {
    let mut estimator = AdaptiveEstimator::new(2, 3, 1.0e-12);
    estimator.add([0.0, 0.0, 0.0]);
    estimator.add([10.0, 10.0, 10.0]);

    assert!(!estimator.should_stop());
    assert_eq!(estimator.relative_variance(), 1.0);

    estimator.add([0.0, 0.0, 0.0]);
    assert!(estimator.should_stop());
}
