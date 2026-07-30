use super::*;

#[test]
fn clear_path_keeps_the_one_metre_camera_distance() {
    let decision = choose_candidate(|_| false);
    assert_eq!(decision.mode, DropPlacementMode::Camera);
    assert_eq!(decision.distance, Some(1.0));
}

#[test]
fn blocked_candidates_retreat_by_ten_centimetres() {
    let decision = choose_candidate(|distance| distance > 0.75);
    assert_eq!(decision.mode, DropPlacementMode::Retreat);
    assert!((decision.distance.expect("retreat should choose a distance") - 0.7).abs() < 0.001);
}

#[test]
fn an_unresolvable_path_uses_the_player_fallback() {
    let decision = choose_candidate(|_| true);
    assert_eq!(decision.mode, DropPlacementMode::PlayerFallback);
    assert_eq!(decision.distance, None);
}

#[test]
fn query_failure_can_be_represented_as_a_blocked_candidate() {
    let query_failed = true;
    let decision = choose_candidate(|_| query_failed);
    assert_eq!(decision.mode, DropPlacementMode::PlayerFallback);
}
