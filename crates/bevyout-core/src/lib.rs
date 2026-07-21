//! Engine-independent contracts and deterministic policy for bevyout.
//!
//! This crate deliberately excludes Bevy, filesystem access, and process
//! execution. The application crate adapts these values at its preparation
//! and runtime boundaries.

pub mod actor;
pub mod actor_animation;
pub mod content;
pub mod form_id;
pub mod geometry;
pub mod item_transaction;
pub mod items;
pub mod manifest;
pub mod pause_menu;
