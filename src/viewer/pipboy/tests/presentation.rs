use super::*;

#[test]
fn pose_endpoints_are_exact() {
    assert_eq!(pose_transform(0.0).translation, HIDDEN_TRANSLATION);
    assert_eq!(pose_transform(1.0).translation, SHOWN_TRANSLATION);
}

#[test]
fn smoothstep_is_bounded_and_hits_endpoints() {
    assert_eq!(smoothstep(0.0), 0.0);
    assert_eq!(smoothstep(1.0), 1.0);
    assert!((smoothstep(0.5) - 0.5).abs() < f32::EPSILON);
}

#[test]
fn screen_uv_maps_to_render_target_edges() {
    assert_eq!(render_target_point(SCREEN_UV_MIN), Vec2::ZERO);
    assert_eq!(
        render_target_point(SCREEN_UV_MAX),
        Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
    );
    let quarter = SCREEN_UV_MIN + (SCREEN_UV_MAX - SCREEN_UV_MIN) * Vec2::new(0.25, 0.75);
    assert_eq!(render_target_point(quarter), Vec2::new(256.0, 576.0));
}

#[test]
fn phase_progress_is_continuous_in_both_directions() {
    let raising = phase_progress(PresentationPhase::Raising, RAISE_SECONDS * 0.4);
    let lowering = phase_progress(PresentationPhase::Lowering, LOWER_SECONDS * 0.6);
    assert!((raising - 0.4).abs() < f32::EPSILON);
    assert!((lowering - 0.4).abs() < f32::EPSILON);
}
