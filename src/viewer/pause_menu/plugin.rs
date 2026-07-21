//! Bevy adapter for the ESC pause menu over `GameplayModal::Paused`.

use bevy::prelude::*;

use crate::app_state::GameplayModal;

use super::snapshot::{self, PauseSnapshot};
use super::ui::{self, PauseMenuUiState};

pub(crate) struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PauseMenuUiState>()
            .init_resource::<PauseSnapshot>()
            .add_systems(Startup, ui::load_pause_menu_assets)
            .add_systems(
                OnEnter(GameplayModal::Paused),
                (snapshot::begin_snapshot_capture, ui::open_pause_menu).chain(),
            )
            .add_systems(
                OnExit(GameplayModal::Paused),
                (ui::close_pause_menu, snapshot::restore_world_camera).chain(),
            )
            .add_systems(
                Update,
                (
                    snapshot::reveal_if_capture_stalled,
                    ui::handle_keyboard,
                    ui::handle_pointer,
                )
                    .chain()
                    .in_set(crate::viewer::plugins::ViewerSet::Ui)
                    .run_if(in_state(GameplayModal::Paused)),
            );
    }
}
