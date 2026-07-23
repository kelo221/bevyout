//! Pure gate for the autonomous package driver (issue #218): decides whether
//! a freshly-seeded actor should be auto-bound to a nav agent and started on
//! its package. Std-only (no Bevy import) so it compiles verbatim into
//! `tests/features.rs` via `#[path]`, mirroring every other pure decision
//! module in this codebase (`nav/repath.rs`, `nav/movement_policy.rs`, ...):
//! the cucumber scenario proving "an alive actor is selected for auto-bind"
//! exercises the exact rule [`crate::viewer::ai::autonomous`]'s Bevy system
//! consults, not a restatement of it.

/// Whether an actor should be selected for the autonomous package driver
/// this tick.
///
/// `is_alive` mirrors `ActorLifeState::Alive` (a corpse is never
/// auto-driven, matching `actor_state::seed_actor_states`'s own "life state
/// is a fact about the actor, not something the driver overrides" stance).
/// `already_nav_bound`/`already_has_controller` are what make the decision
/// idempotent: an actor a console `tna bind`/`runpackage` already touched is
/// left alone, so the autonomous driver only ever picks up an actor nobody
/// has touched yet, and console tools keep working exactly as before,
/// side by side with it.
#[must_use]
pub(crate) fn eligible_for_autonomous_start(
    is_alive: bool,
    already_nav_bound: bool,
    already_has_controller: bool,
) -> bool {
    is_alive && !already_nav_bound && !already_has_controller
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_fresh_alive_untouched_actor_is_eligible() {
        assert!(eligible_for_autonomous_start(true, false, false));
    }

    #[test]
    fn a_dead_actor_is_never_eligible() {
        assert!(!eligible_for_autonomous_start(false, false, false));
        // Even if somehow already bound/controlled, death alone is enough
        // to reject -- this asserts life-state is checked unconditionally,
        // not merely first-checked-first-fails.
        assert!(!eligible_for_autonomous_start(false, true, true));
    }

    #[test]
    fn an_already_nav_bound_actor_is_left_to_the_console() {
        assert!(!eligible_for_autonomous_start(true, true, false));
    }

    #[test]
    fn an_actor_already_running_a_package_is_left_alone() {
        assert!(!eligible_for_autonomous_start(true, false, true));
    }
}
