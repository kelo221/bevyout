[bevy](../index.html)::[scene](index.html)

# Function resolve\_scene\_patches 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#603-610)

```rust
pub fn resolve_scene_patches(
    events: MessageReader<'_, '_, AssetEvent<ScenePatch>>,
    list_events: MessageReader<'_, '_, AssetEvent<SceneListPatch>>,
    assets: Res<'_, AssetServer>,
    patches: ResMut<'_, Assets<ScenePatch>>,
    list_patches: ResMut<'_, Assets<SceneListPatch>>,
    waiting: ResMut<'_, WaitingScenes>,
)
```

A [`System`](../prelude/trait.System.html "trait bevy::prelude::System") that resolves [`ScenePatch`](struct.ScenePatch.html "struct bevy::scene::ScenePatch") and [`SceneListPatch`](struct.SceneListPatch.html "struct bevy::scene::SceneListPatch") assets whose dependencies have been fully loaded.