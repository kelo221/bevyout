//! Explicit application and gameplay states (issue #35).
//!
//! Owns `AppState`, `GameplayModal`, the `RequestStateTransition` message,
//! legal-transition validation, and the narrow `AppStatePlugin` that other
//! slices (currently only `viewer`) add to their `App` and gate their
//! systems against via `in_state`. No rendering or gameplay logic lives
//! here — only the state graph and its transition rules.

mod plugin;

pub(crate) use plugin::{
    AppState, AppStatePlugin, GameplayModal, LoadingTarget, auto_advance_from_boot,
    auto_advance_from_loading,
};

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
