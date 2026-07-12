[bevy](../index.html)::[scene](index.html)

# Function on\_add\_scene\_patch\_instance 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#691-695)

```rust
pub fn on_add_scene_patch_instance(
    add: On<'_, '_, Add, ScenePatchInstance>,
    queued_scenes: ResMut<'_, QueuedScenes>,
    instances: Query<'_, '_, &ScenePatchInstance>,
)
```

An [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer") system that queues newly added [`ScenePatchInstance`](../prelude/struct.ScenePatchInstance.html "struct bevy::prelude::ScenePatchInstance") entities.