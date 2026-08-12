use bevy::prelude::*;

fn move_entities(mut query: Query<&mut Transform>) {
    for mut transform in &mut query {
        transform.translation.x += 1.0;
    }
}

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("mesh.glb");
    commands.insert_resource(SceneRoot(handle));
}

fn configure(app: &mut App) {
    app.add_systems(Update, (move_entities, update_ui));
}
