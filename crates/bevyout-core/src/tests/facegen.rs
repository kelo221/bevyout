use super::*;

fn bytes_for(count: usize, value: f32) -> Vec<u8> {
    (0..count)
        .flat_map(|_| value.to_le_bytes())
        .collect::<Vec<_>>()
}

#[test]
fn canonical_components_decode_exactly() {
    let geometry = decode_component(
        FaceGenComponent::GeometrySymmetric,
        &bytes_for(GEOMETRY_SYMMETRIC_COEFFICIENTS, 1.25),
    )
    .unwrap();
    assert_eq!(geometry.len(), 50);
    assert_eq!(geometry[0], 1.25);

    let asymmetric = decode_component(
        FaceGenComponent::GeometryAsymmetric,
        &bytes_for(GEOMETRY_ASYMMETRIC_COEFFICIENTS, -2.0),
    )
    .unwrap();
    assert_eq!(asymmetric.len(), 30);

    let texture = decode_component(
        FaceGenComponent::TextureSymmetric,
        &bytes_for(TEXTURE_SYMMETRIC_COEFFICIENTS, 0.5),
    )
    .unwrap();
    assert_eq!(texture.len(), 50);
}

#[test]
fn unsupported_lengths_and_nonfinite_values_are_typed() {
    let error = decode_component(FaceGenComponent::GeometrySymmetric, &[0; 4]).unwrap_err();
    assert_eq!(error.code(), "unsupported_facegen_layout");
    assert!(matches!(
        error,
        FaceGenDiagnostic::UnsupportedLength {
            expected_bytes: 200,
            actual_bytes: 4,
            ..
        }
    ));

    let mut bytes = bytes_for(GEOMETRY_SYMMETRIC_COEFFICIENTS, 0.0);
    bytes[..4].copy_from_slice(&f32::NAN.to_le_bytes());
    let error = decode_component(FaceGenComponent::GeometrySymmetric, &bytes).unwrap_err();
    assert_eq!(error.code(), "nonfinite_facegen_coefficient");
    assert!(matches!(
        error,
        FaceGenDiagnostic::NonFiniteCoefficient { index: 0, .. }
    ));
}

#[test]
fn race_defaults_and_actor_traits_are_added_without_loss() {
    let race = FaceGenRaw {
        geometry_symmetric: Some(bytes_for(GEOMETRY_SYMMETRIC_COEFFICIENTS, 1.0)),
        ..FaceGenRaw::default()
    };
    let actor = FaceGenRaw {
        geometry_symmetric: Some(bytes_for(GEOMETRY_SYMMETRIC_COEFFICIENTS, 2.0)),
        ..FaceGenRaw::default()
    };
    let resolved = resolve_facegen(&actor, Some(&race)).unwrap().unwrap();
    assert_eq!(resolved.coefficients.geometry_symmetric[0], 3.0);
    assert_eq!(resolved.actor, actor);
    assert_eq!(resolved.race, race);
}

#[test]
fn empty_sources_are_not_authored() {
    assert!(
        resolve_facegen(&FaceGenRaw::default(), None)
            .unwrap()
            .is_none()
    );
}
