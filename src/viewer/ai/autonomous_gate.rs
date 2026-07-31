//! Pure gate for the autonomous package driver (issue #218): decides whether
//! a freshly-seeded actor should be auto-bound to a nav agent and started
//! on its package. Std-only (no Bevy import) so it compiles verbatim into
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
#[path = "tests/autonomous_gate.rs"]
mod tests;
