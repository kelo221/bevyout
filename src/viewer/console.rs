//! Viewer-dependent console commands and UI visibility markers.

use std::path::PathBuf;

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::PointLightShadowSamples;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::PrimaryWindow;
use bevy_boxddd::prelude::BoxdddDebugDrawSettings;
use serde_json::{Map, Value, json};

use crate::app_state::GameplayModal;
use crate::console::{
    ConsoleCommand, ConsoleCommandProvider, ConsoleCommandResult, ConsoleEntityHooks, ConsoleError,
    ConsoleInvocation, ConsoleRegistry, resolve_reference,
};
use crate::item_transaction::{HolderId, ItemInstanceId, TransactionRequest};
use crate::vsa::{PreparedItemCatalog, PreparedItemStats, PreparedSemantic};

use super::controls::{
    AmbientScale, AoStrength, EmissionScale, FogStrength, HorizontalFov, IrradianceIntensity,
    LightingScale, LightsDisabled, MAX_HORIZONTAL_FOV_DEGREES, MAX_REFLECTION_PROBE_STRENGTH,
    MAX_ROUGHNESS_SCALE, MIN_HORIZONTAL_FOV_DEGREES, MIN_REFLECTION_PROBE_STRENGTH,
    MIN_ROUGHNESS_SCALE, MaterialClampSettings, ReflectionProbeSettings, UnlitMode,
    VolumetricFogMultiplier, horizontal_to_vertical_fov,
};
use super::inventory::{InventoryStack, StackKey};
#[cfg(test)]
use super::lighting::PreparedPointShadowRuntime;
use super::lighting::{RealtimeShadowSettings, shadow_cache_status};
use super::{
    actor, actor_animation, actor_state, diagnostics, interaction, nav, nav_overlay, player, stats,
};
mod actor_state_commands;
// pub(crate): issue #218's autonomous package driver calls `start_package`
// directly -- one implementation shared with the console `runpackage`.
pub(crate) mod ai_package_commands;
mod cinema_commands;
mod common;
mod dialogue_commands;
mod effect_commands;
mod item_commands;
mod navigation_commands;
mod perception_commands;
mod perk_commands;
mod persistence_commands;
mod player_commands;
mod render_commands;
mod screen_fx_commands;
mod stats_commands;
mod ui_commands;
mod weapon_commands;
mod world_commands;

use common::{no_args, parse_item_form_id, toggle_result};
#[cfg(test)]
use render_commands::*;

#[derive(Component)]
pub(crate) struct GameUi;

#[derive(Component)]
pub(crate) struct DiagnosticUi;

#[derive(Resource, Clone, Copy, Debug)]
struct GameUiState {
    visible: bool,
}

impl Default for GameUiState {
    fn default() -> Self {
        Self { visible: true }
    }
}

#[derive(Resource, Clone, Copy, Debug)]
struct DiagnosticUiState {
    visible: bool,
}

impl Default for DiagnosticUiState {
    fn default() -> Self {
        Self { visible: true }
    }
}

pub(crate) struct ViewerConsolePlugin;

impl Plugin for ViewerConsolePlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    app.init_resource::<GameUiState>()
        .init_resource::<DiagnosticUiState>()
        .init_resource::<RealtimeShadowSettings>()
        .init_resource::<nav_overlay::NavMeshOverlayState>()
        .init_resource::<nav_overlay::NavOverlayExposureLock>()
        // Issue #151: the `tdi`-toggled debug info HUD -- state lives with
        // the HUD it drives in `diagnostics.rs`; spawn/update follow the
        // same Startup+Update split as `player::mod`'s collider/step HUDs.
        .init_resource::<diagnostics::DebugInfoState>()
        .add_systems(Startup, diagnostics::spawn_debug_info_hud)
        .add_systems(
            Update,
            (
                ui_commands::sync_ui_visibility,
                nav_overlay::despawn_stale_nav_overlay,
                diagnostics::update_debug_info_hud,
            )
                .in_set(super::plugins::ViewerSet::Ui),
        );
    {
        let mut hooks = app.world_mut().resource_mut::<ConsoleEntityHooks>();
        hooks.register_transform_mutated(player::console_transform_mutated);
        hooks.register_angle_adapter(player::console_get_angles, player::console_set_angles);
    }
    let mut registry = app.world_mut().resource_mut::<ConsoleRegistry>();
    let providers: [&dyn ConsoleCommandProvider; 17] = [
        &player_commands::PlayerCommandProvider,
        &navigation_commands::NavigationCommandProvider,
        &world_commands::WorldCommandProvider,
        &render_commands::RenderCommandProvider,
        &ui_commands::UiCommandProvider,
        &dialogue_commands::DialogueCommandProvider,
        &item_commands::ItemCommandProvider,
        &effect_commands::EffectCommandProvider,
        &persistence_commands::PersistenceCommandProvider,
        &ai_package_commands::AiPackageCommandProvider,
        &actor_state_commands::ActorStateCommandProvider,
        &cinema_commands::CinemaCommandProvider,
        &perception_commands::PerceptionCommandProvider,
        &weapon_commands::WeaponCommandProvider,
        &screen_fx_commands::ScreenFxCommandProvider,
        &stats_commands::StatsCommandProvider,
        &perk_commands::PerkCommandProvider,
    ];
    for provider in providers {
        registry
            .register_provider(provider)
            .expect("built-in viewer console providers must be unique");
    }
}

#[cfg(test)]
#[path = "console/tests.rs"]
mod tests;
