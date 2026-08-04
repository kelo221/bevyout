//! Debug-only navigation commands and projections.

mod actions;
mod capsule;
mod command;
pub(crate) mod player;
mod probes;
mod roster;

pub(crate) use actions::*;
pub(crate) use capsule::spawn_debug_capsule as spawn_test_agent;
pub(crate) use command::*;
pub(crate) use roster::*;
