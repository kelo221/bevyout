//! Engine-independent contracts and deterministic policy for bevyout.
//!
//! This crate deliberately excludes Bevy, filesystem access, and process
//! execution. The application crate adapts these values at its preparation
//! and runtime boundaries.

pub mod actor;
pub mod actor_animation;
pub mod actor_state;
pub mod combat;
pub mod content;
pub mod dialogue;
pub mod disposition;
pub mod faction;
pub mod form_id;
pub mod geometry;
pub mod item_transaction;
pub mod items;
pub mod manifest;
pub mod pause_menu;
pub mod perception;
pub mod time_of_day;
pub mod weapon;
