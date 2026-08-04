use super::*;

#[test]
fn observations_expose_reached_and_blocking_door_without_backend_types() {
    let reached = NavObservation {
        status: NavStatus::Reached,
    };
    assert!(reached.is_reached());
    assert!(!reached.is_failed());
    assert_eq!(reached.blocking_door(), None);

    let blocked = NavObservation {
        status: NavStatus::Failed(NavFailureReason::BlockedDoor(FormId(0x1234))),
    };
    assert!(!blocked.is_reached());
    assert!(blocked.is_failed());
    assert_eq!(blocked.blocking_door(), Some(0x1234));
}
