use super::*;

#[test]
fn no_observations_keeps_the_route() {
    assert_eq!(
        decide(RepathObservation::default()),
        RepathDecision::KeepRoute
    );
}

#[test]
fn destination_cell_unloaded_always_fails_even_alongside_other_triggers() {
    let observation = RepathObservation {
        destination_cell_unloaded: true,
        door_became_unblocked: true,
        ..Default::default()
    };
    assert_eq!(decide(observation), RepathDecision::Fail);
}

#[test]
fn a_newly_blocked_door_triggers_repath() {
    let observation = RepathObservation {
        door_became_blocked: true,
        ..Default::default()
    };
    assert_eq!(decide(observation), RepathDecision::Repath);
}

#[test]
fn a_newly_unblocked_door_triggers_repath() {
    let observation = RepathObservation {
        door_became_unblocked: true,
        ..Default::default()
    };
    assert_eq!(decide(observation), RepathDecision::Repath);
}

#[test]
fn a_moved_target_triggers_repath() {
    let observation = RepathObservation {
        target_moved_beyond_tolerance: true,
        ..Default::default()
    };
    assert_eq!(decide(observation), RepathDecision::Repath);
}

#[test]
fn an_off_link_agent_triggers_repath() {
    let observation = RepathObservation {
        agent_off_link: true,
        ..Default::default()
    };
    assert_eq!(decide(observation), RepathDecision::Repath);
}
