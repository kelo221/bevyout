use super::*;

#[test]
fn steady_disabled_state_needs_no_writes() {
    assert!(!disabled_shadow_writes_needed(false, false));
}

#[test]
fn a_settings_change_pays_one_cleanup_pass() {
    assert!(disabled_shadow_writes_needed(true, false));
    assert!(disabled_shadow_writes_needed(true, true));
}

#[test]
fn a_lingering_selection_pays_one_cleanup_pass() {
    assert!(disabled_shadow_writes_needed(false, true));
}
