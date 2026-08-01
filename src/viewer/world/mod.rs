//! Runtime cell map, predictive door-graph neighbor preloader (issue #51),
//! and instant door-transition cell swap (issue #52).
//!
//! `policy.rs` is a pure, std-only planner (mirrors `vsa::cell_map`'s
//! dependency-free design so `tests/features.rs` can include it verbatim);
//! `preload.rs` is the Bevy-side glue that reads `<asset_root>/cellmap.ron`
//! into `policy::CellGraph`, tracks `ActiveCell`/`ResidentCells`, and drives
//! background manifest parsing plus hidden per-cell root spawning for
//! planned loads.
//!
//! `swap_policy.rs` is likewise a pure, std-only seam (issues #52/#59): the
//! eligibility decision (instant vs. fallback), the fallback load outcome
//! and lifecycle (cancel/supersede/fail), the overlay-fade curve, and the
//! collider-build stagger queue. `swap.rs` is the Bevy-side glue that turns
//! a door activation into either an instant same-frame cell swap or a
//! loading-screen fallback that reuses `preload.rs`'s background manifest
//! parse, then performs the same activation steps either way.
//! `persist.rs`/`persist_policy.rs` (issues #60/#61) own save-state capture
//! and application across every load/unload path.
//!
//! Cell lifecycle (issue #63) — the states the systems below imply, made
//! explicit: **Absent** (not resident) → **Loading** (background manifest
//! parse, `PendingPreloadParse`) → **Spawning** (`PendingCellSpawns`
//! drains bounded chunks under a hidden root) → **Ready** (resident,
//! hidden, every scene handle loaded) → **Active** (swap: visible,
//! refs registered, colliders built and recorded in the per-cell
//! `ownership_policy::CellColliderLedger`, save state applied) →
//! **Resident-hidden** (swapped away: state captured to `ActiveSaveState`,
//! colliders torn down via the ledger, scenes stay spawned) → **Evicted**
//! (captured, colliders torn down, root despawned; Bevy's asset handle
//! refcounting is the asset barrier — a shared GLB survives as long as any
//! resident cell's `scene_handles` still hold it).
//!
//! `reveal_policy.rs` is a fourth pure, std-only seam (issue #55): bounded
//! reveal-chunk planning, ordered nearest-to-arrival first, for a
//! preloaded cell's placement entities. `reveal.rs` is the Bevy-side glue
//! `swap.rs` calls into during `activate_resident_cell` to flip visibility
//! in bounded chunks across a few frames instead of all at once -- see that
//! module's doc comment for the measured spike this amortizes.

use bevy::prelude::{
    Camera3d, GlobalTransform, IntoScheduleConfigs, Query, Res, ResMut, Resource, Transform,
    Update, With, in_state,
};
use bevyout_core::manifest::exterior::{
    WorldLocation, WorldLocationExterior, WorldLocationInterior,
};

use crate::app_state::AppState;
use crate::viewer::LoadedSceneManifest;
use crate::viewer::player::FpsPlayer;

pub(crate) mod exterior;
mod ownership_policy;
mod persist;
mod persist_policy;
mod policy;
mod preload;
mod reveal;
mod reveal_policy;
mod swap;
mod swap_policy;

pub(crate) use ownership_policy::CellColliderLedger;
pub(crate) use persist::{
    ActiveSaveState, DynamicBodyRestore, PersistRestores, PlaythroughSeed,
    apply_exterior_cell_state, apply_save_state_at_startup, capture_exterior_cell_state,
    write_save_slot,
};
#[cfg(test)]
pub(crate) use preload::ResidentCellLimit;
pub(crate) use preload::{ActiveCell, ResidentCell, ResidentCells, ResidentState};
pub(crate) use swap_policy::{
    COLLIDER_BUILD_BUDGET_PER_FRAME, ColliderBuildPhase, ColliderBuildQueue,
    next_collider_build_phase, partition_collider_indices,
};

/// Canonical player location used by save/reload and the world-location
/// console surface. The runtime updates this from the authoritative player
/// transform (or free camera while FPS mode is disabled).
#[derive(Resource, Debug, Clone, Default, PartialEq)]
pub(crate) struct CurrentWorldLocation(pub(crate) Option<WorldLocation>);

pub(crate) struct WorldPlugin {
    pub(crate) resident_cell_limit: usize,
}

impl bevy::app::Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        install(app, self.resident_cell_limit);
    }
}

fn install(app: &mut bevy::app::App, resident_cell_limit: usize) {
    persist::install(app);
    preload::install(app, resident_cell_limit);
    reveal::install(app);
    swap::install(app);
    app.init_resource::<CurrentWorldLocation>().add_systems(
        Update,
        update_current_world_location
            .after(crate::viewer::plugins::ViewerSet::WorldSync)
            .run_if(in_state(AppState::InGame)),
    );
}

fn update_current_world_location(
    manifest: Res<LoadedSceneManifest>,
    mut location: ResMut<CurrentWorldLocation>,
    player: Query<(&Transform, &FpsPlayer)>,
    cameras: Query<&GlobalTransform, (With<Camera3d>, bevy::prelude::Without<FpsPlayer>)>,
) {
    let Some((position, rotation)) = player
        .iter()
        .next()
        .map(|(transform, _)| (transform.translation, transform.rotation))
        .or_else(|| {
            cameras
                .iter()
                .next()
                .map(GlobalTransform::to_scale_rotation_translation)
                .map(|(_, rotation, position)| (position, rotation))
        })
    else {
        return;
    };
    let position = position.to_array();
    let rotation_xyzw = rotation.to_array();
    location.0 = if let Some(exterior) = manifest.exterior.as_ref() {
        Some(WorldLocation::Exterior(WorldLocationExterior {
            worldspace_form_id: exterior.worldspace_form_id,
            position,
            rotation_xyzw,
        }))
    } else {
        Some(WorldLocation::Interior(WorldLocationInterior {
            cell_form_id: manifest.cell.form_id,
            position,
            rotation_xyzw,
        }))
    };
}
