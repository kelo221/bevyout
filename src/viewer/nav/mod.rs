//! M4 wave 3 (#112): `bevy_landmass` navigation-backend spike. Owns loading
//! the prepared per-cell nav graph (#111's `navgraph.ron`, the same way
//! `nav_overlay.rs` reads it), converting it into a validated
//! `bevy_landmass` navigation mesh via the pure `landmass_graph` module, and
//! running a crude kinematic test agent (`agent/`) driven by the `tna`
//! console command family. This is the runtime navigation slice seam #113
//! grows into the full Fallout adapter (travel doors, AI packages); this
//! wave stays a spike -- one archipelago per active cell (one island per
//! prepared nav mesh within it), built lazily, torn down on cell swap.

use bevy::prelude::*;

pub(crate) mod agent;
pub(crate) mod api;
pub(crate) mod debug;
pub(crate) mod diagnostics;
pub(crate) mod door_link;
pub(crate) mod doors;
pub(crate) mod handoff;
mod input;
pub(crate) mod landmass_graph;
pub(crate) mod ledger_policy;
pub(crate) mod movement_policy;
pub(crate) mod openmw_doors;
mod plugin;
pub(crate) mod repath;
pub(crate) mod traversal;
pub(crate) mod world;

pub(crate) use input::*;
pub(crate) use plugin::NavBackendPlugin;

pub(crate) struct NavPlugin;

impl Plugin for NavPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    agent::install(app);
}
