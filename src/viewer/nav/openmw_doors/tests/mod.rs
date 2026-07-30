use super::*;

fn base() -> DoorAccessObservation {
    DoorAccessObservation {
        lock_level: Some(25),
        trapped: false,
        key_form_id: Some(0x1234),
        holder_has_key: false,
    }
}

#[test]
fn an_untrapped_unlocked_door_is_always_openable() {
    assert!(door_openable(DoorAccessObservation {
        lock_level: None,
        ..base()
    }));
    assert!(door_openable(DoorAccessObservation {
        lock_level: Some(0),
        ..base()
    }));
}

#[test]
fn a_locked_door_is_openable_when_the_actor_holds_its_key() {
    assert!(door_openable(DoorAccessObservation {
        holder_has_key: true,
        ..base()
    }));
}

#[test]
fn a_locked_door_stays_shut_when_the_actor_lacks_the_key() {
    assert!(!door_openable(DoorAccessObservation {
        holder_has_key: false,
        ..base()
    }));
}

#[test]
fn a_locked_door_with_no_assigned_key_never_opens_for_anyone() {
    // OpenMW's `keyId.empty()` case: `if (keyId.empty()) return;` --
    // there is no key to hold, so no actor can force it open this way.
    assert!(!door_openable(DoorAccessObservation {
        key_form_id: None,
        holder_has_key: false,
        ..base()
    }));
}

#[test]
fn a_trapped_door_never_opens_even_with_the_right_key() {
    assert!(!door_openable(DoorAccessObservation {
        trapped: true,
        holder_has_key: true,
        ..base()
    }));
}

#[test]
fn a_trapped_but_unlocked_door_still_never_opens() {
    assert!(!door_openable(DoorAccessObservation {
        trapped: true,
        lock_level: None,
        ..base()
    }));
}
