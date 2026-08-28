//! Typed composition boundary for the runtime viewer.
//!
//! Feature modules expose small [`Plugin`] values while this group owns the
//! startup order and the few cross-slice same-frame phases that are part of
//! the viewer contract.

use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;

use crate::app_state::AppStatePlugin;

use super::{
    actor, actor_animation, actor_state, agent_bridge, ai, animation, audio, bindings, cinema,
    console, console_ui, day_night, dialogue, effects, game_time, hud, interaction, minigames, nav,
    pause_menu, perception, pipboy, pipboy_reader, player, screen_fx, stats, weapon, world,
    world_items,
};

/// Cross-slice ordering is intentionally narrow: only user input, interaction
/// dispatch, world synchronization, and presentation need a shared frame
/// contract. Systems inside each feature keep their local ordering.
#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum ViewerSet {
    Input,
    Interaction,
    Dialogue,
    WorldSync,
    Ui,
}

struct ViewerSchedulePlugin;

impl Plugin for ViewerSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                ViewerSet::Input,
                ViewerSet::Interaction,
                ViewerSet::Dialogue,
                ViewerSet::WorldSync,
                ViewerSet::Ui,
            )
                .chain(),
        );
    }
}

/// Complete feature-level viewer composition. Configuration remains explicit
/// at the composition root instead of being hidden in global resources.
pub(crate) struct ViewerPlugins {
    pub(crate) disable_physics: bool,
    pub(crate) worldspace_lod: bool,
    pub(crate) resident_cell_limit: usize,
    pub(crate) exterior_resident_cell_limit: usize,
    pub(crate) agent_port: Option<u16>,
    pub(crate) day_night_cycle_seconds: Option<f32>,
}

impl PluginGroup for ViewerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut plugins = PluginGroupBuilder::start::<Self>()
            .add(ViewerSchedulePlugin)
            .add(crate::console::ConsolePlugin);

        if let Some(port) = self.agent_port {
            plugins = plugins.add(agent_bridge::AgentBridgePlugin { port });
        }

        plugins
            .add(AppStatePlugin)
            .add(player::PlayerPlugin {
                disable_physics: self.disable_physics,
            })
            .add(bindings::BindingsPlugin)
            .add(audio::ViewerAudioPlugin)
            .add(game_time::GameTimePlugin)
            .add(day_night::DayNightPlugin {
                cycle_seconds: self.day_night_cycle_seconds,
            })
            .add(hud::HudPlugin)
            .add(interaction::InteractionPlugin)
            .add(minigames::MinigamesPlugin)
            .add(dialogue::DialoguePlugin)
            .add(actor::ActorPlugin)
            .add(actor_state::ActorStatePlugin)
            .add(stats::StatsPlugin)
            .add(effects::EffectsPlugin)
            .add(screen_fx::ScreenFxPlugin)
            .add(weapon::WeaponPlugin)
            .add(perception::PerceptionPlugin)
            .add(actor_animation::ActorAnimationPlugin)
            .add(pipboy::PipBoyPlugin)
            .add(pipboy_reader::PipBoyReaderPlugin)
            .add(animation::PlacementAnimationPlugin)
            .add(console::ViewerConsolePlugin)
            .add(console_ui::ConsoleUiPlugin)
            .add(pause_menu::PauseMenuPlugin)
            .add(cinema::CinemaPlugin)
            .add(world::WorldPlugin {
                resident_cell_limit: self.resident_cell_limit,
            })
            .add(world::exterior::ExteriorWorldPlugin {
                resident_cell_limit: self.exterior_resident_cell_limit,
                worldspace_lod: self.worldspace_lod,
            })
            .add(world_items::WorldItemsPlugin)
            .add(nav::NavPlugin)
            .add(ai::AiPackagePlugin)
    }
}

#[cfg(test)]
#[path = "tests/plugins.rs"]
mod tests;
