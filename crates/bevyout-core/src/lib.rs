//! Engine-independent contracts and deterministic policy for bevyout.
//!
//! This crate deliberately excludes Bevy, filesystem access, and process
//! execution. The application crate adapts these values at its preparation
//! and runtime boundaries.

pub mod actor;
pub mod actor_animation;
pub mod actor_state;
pub mod barter;
pub mod chems;
pub mod combat;
pub mod content;
pub mod crafting;
pub mod crime;
pub mod detection;
pub mod dialogue;
pub mod disposition;
pub mod effects;
pub mod facegen;
pub mod faction;
pub mod form_id;
pub mod geometry;
pub mod image_space;
pub mod item_transaction;
pub mod items;
pub mod lifecycle;
pub mod lighting;
pub mod local_light_policy;
pub mod manifest;
pub mod minigames;
pub mod pause_menu;
pub mod perception;
pub mod perks;
pub mod radiation;
pub mod repair;
pub mod stats;
pub mod time;
pub mod time_of_day;
pub mod weapon;
