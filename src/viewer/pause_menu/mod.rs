//! Fallout-style ESC pause menu: blurred scene snapshot, monofonto stack,
//! and Continue/Quit actions over `GameplayModal::Paused`.

mod plugin;
mod snapshot;
mod ui;

pub(crate) use plugin::PauseMenuPlugin;
