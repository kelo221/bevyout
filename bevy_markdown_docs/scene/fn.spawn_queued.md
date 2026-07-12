[bevy](../index.html)::[scene](index.html)

# Function spawn\_queued 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#704-711)

```rust
pub fn spawn_queued(
    world: &mut World,
    scene_patch_instances: &mut QueryState<&ScenePatchInstance>,
    queued: Local<'_, QueuedScenes>,
    bundle_scratch: Local<'_, BundleScratch>,
    reader: Local<'_, MessageCursor<AssetEvent<ScenePatch>>>,
    list_reader: Local<'_, MessageCursor<AssetEvent<SceneListPatch>>>,
)
```

A system that spawns queued scenes when they are loaded.