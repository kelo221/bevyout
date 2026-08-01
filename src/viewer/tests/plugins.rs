use super::*;
use bevy::asset::AssetPlugin;
use bevy::state::app::StatesPlugin;

#[test]
fn viewer_group_installs_feature_plugins_and_forwards_configuration() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
        .add_plugins(ViewerPlugins {
            disable_physics: true,
            worldspace_lod: false,
            resident_cell_limit: 7,
            exterior_resident_cell_limit: 25,
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
