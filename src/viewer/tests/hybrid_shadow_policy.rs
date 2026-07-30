use super::*;

#[test]
fn darker_source_wins_when_both_are_available() {
    assert_eq!(hybrid_shadow_visibility(Some(0.8), Some(0.2)), 0.2);
    assert_eq!(hybrid_shadow_visibility(Some(0.35), Some(0.6)), 0.35);
}

#[test]
fn an_unavailable_source_does_not_remove_the_other_visibility() {
    assert_eq!(hybrid_shadow_visibility(Some(0.35), None), 0.35);
    assert_eq!(hybrid_shadow_visibility(None, Some(0.6)), 0.6);
    assert_eq!(hybrid_shadow_visibility(None, None), 1.0);
}

#[test]
fn invalid_visibility_is_treated_as_lit() {
    assert_eq!(hybrid_shadow_visibility(Some(f32::NAN), None), 1.0);
    assert_eq!(hybrid_shadow_visibility(None, Some(f32::INFINITY)), 1.0);
}
