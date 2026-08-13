use bevy::prelude::*;

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene = asset_server.load("scene.glb#Scene0");
    let scratch = Vec::new();
    commands.spawn(SceneRoot(scene));
    drop(scratch);
}
