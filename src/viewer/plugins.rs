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
    console, console_ui, day_night, hud, interaction, nav, pause_menu, perception, pipboy,
    pipboy_reader, player, weapon, world, world_items,
};

/// Cross-slice ordering is intentionally narrow: only user input, interaction
/// dispatch, world synchronization, and presentation need a shared frame
/// contract. Systems inside each feature keep their local ordering.
#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum ViewerSet {
    Input,
    Interaction,
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
    pub(crate) resident_cell_limit: usize,
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
            .add(day_night::DayNightPlugin {
                cycle_seconds: self.day_night_cycle_seconds,
            })
            .add(hud::HudPlugin)
            .add(interaction::InteractionPlugin)
            .add(actor::ActorPlugin)
            .add(actor_state::ActorStatePlugin)
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
            .add(world_items::WorldItemsPlugin)
            .add(nav::NavPlugin)
            .add(ai::AiPackagePlugin)
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::AssetPlugin;
    use bevy::state::app::StatesPlugin;

    use super::*;

    #[test]
    fn viewer_group_installs_feature_plugins_and_forwards_configuration() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
            .add_plugins(ViewerPlugins {
                disable_physics: true,
                resident_cell_limit: 7,
                agent_port: None,
                day_night_cycle_seconds: None,
            });

        assert!(app.is_plugin_added::<player::PlayerPlugin>());
        assert!(app.is_plugin_added::<hud::HudPlugin>());
        assert!(app.is_plugin_added::<interaction::InteractionPlugin>());
        assert!(app.is_plugin_added::<actor::ActorPlugin>());
        assert!(app.is_plugin_added::<actor_state::ActorStatePlugin>());
        assert!(app.is_plugin_added::<weapon::WeaponPlugin>());
        assert!(app.is_plugin_added::<actor_animation::ActorAnimationPlugin>());
        assert!(app.is_plugin_added::<pause_menu::PauseMenuPlugin>());
        assert!(app.is_plugin_added::<world::WorldPlugin>());
        assert!(app.is_plugin_added::<nav::NavPlugin>());
        assert!(app.world().resource::<player::PhysicsDisabled>().0);
        assert_eq!(app.world().resource::<world::ResidentCellLimit>().0, 7);
    }
}
