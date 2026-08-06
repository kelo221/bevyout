use super::super::denoise::{DenoiseError, DenoiseFeature, denoise};

const FEATURE: DenoiseFeature = DenoiseFeature {
    position: [0.0, 0.0, 0.0],
    normal: [0.0, 0.0, 1.0],
    material_id: 1,
    relative_variance: 0.0,
    coverage: 1.0,
    sample_count: 1,
};

fn features(count: usize) -> Vec<DenoiseFeature> {
    vec![FEATURE; count]
}

#[test]
fn same_chart_outlier_is_reduced_by_deterministic_feature_guided_filter() {
    let mut first = vec![[0.0, 0.0, 0.0], [10.0, 10.0, 10.0], [0.0, 0.0, 0.0]];
    let mut second = first.clone();
    let owners = vec![Some(7), Some(7), Some(7)];
    let features = features(3);

    denoise(&mut first, &owners, &features, 3, 1, 1).unwrap();
    denoise(&mut second, &owners, &features, 3, 1, 1).unwrap();

    assert_eq!(first, second);
    assert!(first[1][0] < 10.0);
    assert!(first[1][0] > 0.0);
}

#[test]
fn chart_owner_and_invalid_owner_are_absolute_barriers() {
    let mut chart_barrier = vec![[0.0, 0.0, 0.0], [10.0, 10.0, 10.0], [0.0, 0.0, 0.0]];
    let features = features(3);
    denoise(
        &mut chart_barrier,
        &[Some(1), Some(2), Some(1)],
        &features,
        3,
        1,
        1,
    )
    .unwrap();
    assert_eq!(chart_barrier[1], [10.0, 10.0, 10.0]);

    let mut invalid_barrier = vec![[0.0, 0.0, 0.0], [10.0, 10.0, 10.0], [0.0, 0.0, 0.0]];
    denoise(
        &mut invalid_barrier,
        &[Some(1), None, Some(1)],
        &features,
        3,
        1,
        1,
    )
    .unwrap();
    assert_eq!(invalid_barrier[1], [10.0, 10.0, 10.0]);
}

#[test]
fn position_normal_material_and_variance_features_preserve_an_edge() {
    let mut matching = vec![[0.0; 3], [10.0; 3], [0.0; 3]];
    let mut edge_features = features(3);
    edge_features[1] = DenoiseFeature {
        position: [1.0, 0.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        material_id: 2,
        relative_variance: 4.0,
        coverage: 0.25,
        sample_count: 1,
    };
    let owners = vec![Some(1); 3];
    denoise(&mut matching, &owners, &features(3), 3, 1, 1).unwrap();

    let mut edge_preserved = vec![[0.0; 3], [10.0; 3], [0.0; 3]];
    denoise(&mut edge_preserved, &owners, &edge_features, 3, 1, 1).unwrap();

    assert!(edge_preserved[1][0] > matching[1][0]);
}

#[test]
fn iteration_count_and_input_lengths_are_deterministic_and_validated() {
    let owners = vec![Some(1); 5];
    let features = features(5);
    let original = vec![[0.0; 3], [0.0; 3], [10.0; 3], [0.0; 3], [0.0; 3]];
    let mut no_iterations = original.clone();
    denoise(&mut no_iterations, &owners, &features, 5, 1, 0).unwrap();
    assert_eq!(no_iterations, original);

    let mut one_iteration = original.clone();
    let mut two_iterations = original.clone();
    denoise(&mut one_iteration, &owners, &features, 5, 1, 1).unwrap();
    denoise(&mut two_iterations, &owners, &features, 5, 1, 2).unwrap();
    assert_ne!(one_iteration, original);
    assert_ne!(two_iterations, one_iteration);

    let error = denoise(&mut no_iterations, &owners[..4], &features, 5, 1, 1);
    assert_eq!(error, Err(DenoiseError::BufferLengthMismatch));
}
