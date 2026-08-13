use bevy::prelude::*;
use std::collections::HashMap;

fn hot_blocking(query: Query<&Transform>) {
    std::thread::sleep(std::time::Duration::from_millis(1));
    for _ in &query {}
}

fn hot_allocating(mut commands: Commands) {
    let values = Vec::new();
    commands.spawn_empty();
    drop(values);
}

fn noisy_runtime(query: Query<&Transform>) {
    println!("{}", query.is_empty());
}

fn per_frame_load(asset_server: Res<AssetServer>) {
    let _handle = asset_server.load("mesh.glb");
}

fn panic_system(query: Query<&Transform>) {
    query.iter().next().unwrap();
}

fn configure(app: &mut App) {
    app.add_systems(Update, (hot_blocking, hot_allocating).chain());
}

fn overlap(left: Query<&mut Transform>, right: Query<&mut Transform>) {
    drop((left, right));
}
